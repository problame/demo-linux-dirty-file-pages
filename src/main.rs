use core::panic;
use std::{
    collections::HashMap,
    io::Read,
    mem::MaybeUninit,
    num::NonZeroUsize,
    os::{
        fd::AsRawFd,
        unix::fs::{FileExt, OpenOptionsExt},
    },
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use clap::Parser;
use rand::{Rng, RngCore};
use tracing::{info, warn};

#[derive(clap::Parser, Debug)]
struct Args {
    path: PathBuf,
    size: byte_unit::Byte,
    #[clap(long, default_value = "4Ki")]
    io_size_and_align: byte_unit::Byte,
    #[clap(long)]
    direct: bool,
    #[clap(long)]
    fallocate: bool,
    #[clap(long)]
    dump_kstack_after_ctrlc: bool,
    #[clap(long)]
    abort_on_ctrlc: bool,
}

fn main() {
    // default RUST_LOG env var to "info"
    std::env::set_var(
        "RUST_LOG",
        std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
    );
    tracing_subscriber::fmt::fmt()
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();

    let args = Args::parse();
    info!(?args, "parsed args");

    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .read(true)
        .custom_flags(if args.direct { libc::O_DIRECT } else { 0 })
        .open(&args.path)
        .unwrap();

    if args.direct {
        let mut mystatx: MaybeUninit<libc::statx> = MaybeUninit::uninit();
        unsafe {
            let err = libc::statx(
                file.as_raw_fd(),
                c"".as_ptr(),
                libc::AT_EMPTY_PATH,
                libc::STATX_DIOALIGN,
                mystatx.as_mut_ptr(),
            );
            assert_eq!(err, 0);
            let mystatx = mystatx.assume_init();
            if mystatx.stx_mask & libc::STATX_DIOALIGN == 0 {
                warn!("DIOALIGN not supported, cannot validate io_size");
            } else {
                info!(%mystatx.stx_dio_mem_align, %mystatx.stx_dio_offset_align, "statx");
                let io_size_and_align = args.io_size_and_align.as_u64();
                assert_eq!(io_size_and_align % (mystatx.stx_dio_mem_align as u64), 0);
                assert_eq!(io_size_and_align % (mystatx.stx_dio_offset_align as u64), 0);
            }
        };
    };
    let make_buffer = || {
        let mapping = unsafe {
            nix::sys::mman::mmap_anonymous(
                None,
                NonZeroUsize::new(args.io_size_and_align.as_u64() as usize).unwrap(),
                nix::sys::mman::ProtFlags::PROT_READ | nix::sys::mman::ProtFlags::PROT_WRITE,
                nix::sys::mman::MapFlags::MAP_PRIVATE,
            )
            .unwrap()
        };
        assert_eq!(
            (mapping.as_ptr() as usize) % args.io_size_and_align.as_u64() as usize,
            0
        );
        unsafe {
            std::slice::from_raw_parts_mut(
                mapping.as_ptr() as *mut u8,
                args.io_size_and_align.as_u64() as usize,
            )
        }
    };

    file.set_len(args.size.as_u64()).unwrap();

    if args.fallocate {
        nix::fcntl::fallocate(
            file.as_raw_fd(),
            nix::fcntl::FallocateFlags::FALLOC_FL_ZERO_RANGE,
            0,
            args.size.as_u64() as i64,
        )
        .unwrap();
    }

    info!("Initialized");

    std::thread::spawn(|| {
        let myself = procfs::process::Process::myself().unwrap();
        loop {
            dump_memory_metrics(&myself);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    let stop = Arc::new(AtomicBool::new(false));

    ctrlc::set_handler({
        let stop = Arc::clone(&stop);
        move || {
            info!("Ctrl-C received");
            if args.abort_on_ctrlc {
                info!("aborting process");
                std::process::abort();
            }
            info!("setting stop flag");
            stop.store(true, std::sync::atomic::Ordering::Relaxed);
            let myself = procfs::process::Process::myself().unwrap();
            if !args.dump_kstack_after_ctrlc {
                return;
            }
            if myself.uid().unwrap() != 0 {
                info!("Not root, skipping stack dump");
                return;
            }
            std::thread::spawn(move || {
                let mut buf = String::new();
                loop {
                    let mut res = myself.open_relative("stack").unwrap();
                    buf.clear();
                    res.read_to_string(&mut buf).unwrap();
                    info!("{}", buf);
                    std::thread::sleep(Duration::from_secs(1));
                }
            });
        }
    })
    .unwrap();

    // indefinitly write blobs of size and alignment args.io_size_and_align into the file
    let mut write_buf = make_buffer();
    while !stop.load(std::sync::atomic::Ordering::Relaxed) {
        let offset = rand::thread_rng()
            .gen_range(0..args.size.as_u64() / (args.io_size_and_align.as_u64()))
            * (args.io_size_and_align.as_u64());
        // change what we write every now and then so it's guaranteed we're dirtying stuff
        if rand::thread_rng().gen_bool(0.01) {
            rand::thread_rng().fill_bytes(&mut write_buf);
        }
        if let Err(e) = file.write_all_at(&write_buf, offset) {
            panic!("write failed: {offset:x} {e}");
        }
    }
    info!("writer stopped, closing file descriptor");
    drop(file);
    info!("file descriptor closed, exiting");
}

fn dump_memory_metrics(myself: &procfs::process::Process) {
    let page_sz = procfs::current_system_info().page_size();
    let vmstat = procfs::vmstat().unwrap();
    let smaps_rollup = {
        // https://www.kernel.org/doc/Documentation/ABI/testing/procfs-smaps_rollup
        let rollup = myself.smaps_rollup().unwrap();
        let cum: Vec<_> = rollup.memory_map_rollup.into_iter().collect();
        assert_eq!(cum.len(), 1);
        let rollup: procfs::process::MemoryMap = cum.into_iter().next().unwrap();
        let keys = ["Pss_Anon", "Pss_File", "Pss_Shmem"];
        let out: Vec<_> = keys
            .into_iter()
            .map(|key| (key, rollup.extension.map[key]))
            .collect();
        out
    };
    let procfs::process::Status {
        vmrss,
        rssanon,
        rssfile,
        rssshmem,
        vmsize,
        ..
    } = myself.status().unwrap();
    info!(
        "/proc/self/status (inaccurate) vmrss: {:?}, rssanon: {:?}, rssfile: {:?}, rssshmem: {:?}, vmsize: {:?}",
        vmrss, rssanon, rssfile, rssshmem, vmsize,
    );
    info!(
        "/proc/self/smaps_rollup (accurate): {} = {smaps_rollup:?}",
        smaps_rollup.iter().map(|(_, v)| v).sum::<u64>(),
    );
    info!(
        "system : nr_dirty: {} MiB",
        (u64::try_from(vmstat["nr_dirty"]).unwrap() * page_sz) >> 20
    );
    let cgroups = myself.cgroups().unwrap();
    assert_eq!(
        cgroups.0.len(),
        1,
        "only cgroupv2 supported, should have just one cgroup"
    );
    let cgroup = cgroups.0.into_iter().next().unwrap();
    assert_eq!(cgroup.hierarchy, 0, "only cgroupv2 is supported");
    assert!(cgroup.pathname.starts_with("/"));
    let root = PathBuf::from(format!("/sys/fs/cgroup{}", cgroup.pathname));
    let memory_stats = std::fs::read_to_string(root.join("memory.stat")).unwrap();
    let print_vars = ["file", "file_dirty", "file_writeback", "file_mapped"];
    let memory_stats = extract_cgroup_memory_stat_vars(&memory_stats, print_vars);
    let memory_max = std::fs::read_to_string(root.join("memory.max")).unwrap().trim().to_owned();
    let memory_high = std::fs::read_to_string(root.join("memory.high")).unwrap().trim().to_owned();
    info!("my cgroup {root:?} memory.max={memory_max:?} memory.high={memory_high:?} memory.stat={memory_stats:?}");
}

fn extract_cgroup_memory_stat_vars<'a>(
    memory_stats: &'a String,
    print_vars: [&'static str; 4],
) -> Vec<(&'a str, u64)> {
    let vars: HashMap<_, _> = memory_stats
        .lines()
        .map(|line| {
            let mut comps = line.split_ascii_whitespace();
            assert_eq!(comps.clone().count(), 2);
            (
                comps.next().unwrap(),
                comps.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect();
    let out = print_vars
        .into_iter()
        .map(|key| (key, vars[key]))
        .collect::<Vec<_>>();
    out
}
