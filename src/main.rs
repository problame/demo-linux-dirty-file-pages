use core::panic;
use std::{
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
            info!("Signalling stop");
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
    let mut read_buf = make_buffer();
    while !stop.load(std::sync::atomic::Ordering::Relaxed) {
        let offset = rand::thread_rng()
            .gen_range(0..args.size.as_u64() / (args.io_size_and_align.as_u64()))
            * (args.io_size_and_align.as_u64());
        if rand::thread_rng().gen_bool(1.0) {
            rand::thread_rng().fill_bytes(&mut write_buf);
            if let Err(e) = file.write_all_at(&write_buf, offset) {
                panic!("write failed: {offset:x} {e}");
            }
        } else {
            file.read_exact_at(&mut read_buf, offset).unwrap();
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
        "process: vmrss: {:?}, rssanon: {:?}, rssfile: {:?}, rssshmem: {:?}, vmsize: {:?}, smaps_rollup: {:?}",
        vmrss, rssanon, rssfile, rssshmem, vmsize, smaps_rollup,
    );
    info!(
        "system : nr_dirty: {} MiB",
        (u64::try_from(vmstat["nr_dirty"]).unwrap() * page_sz) >> 20
    );
}
