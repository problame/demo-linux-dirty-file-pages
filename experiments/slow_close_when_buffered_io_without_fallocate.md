Interesting behaviors on Debian bullseye `5.10.0-30-arm64`

# Without fallocate, `close()` or `abort()` waits for writeback and it's slow

```
cs@devvm-mbp:[~/learn/demo-linux-dirty-file-pages]: ./target/release/demo-linux-dirty-file-pages  ~/tmp/blob 10Gi
   0.000405339s  INFO demo_linux_dirty_file_pages: parsed args args=Args { path: "/home/cs/tmp/blob", size: Byte(10737418240), io_size_and_align: Byte(4096), direct: false, fallocate: false, dump_kstack_after_ctrlc: false, abort_on_ctrlc: false }
   0.734050106s  INFO demo_linux_dirty_file_pages: Initialized
   0.734396569s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
   0.734426486s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1357824 = [("Pss_Anon", 278528), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
   0.734431361s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 68 MiB
   0.734521862s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 1319374848), ("file_dirty", 70963200), ("file_writeback", 0), ("file_mapped", 77451264)]
   1.734861554s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
   1.734915222s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1374208 = [("Pss_Anon", 294912), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
   1.734920305s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 735 MiB
   1.735001098s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2118623232), ("file_dirty", 769916928), ("file_writeback", 135168), ("file_mapped", 77451264)]
   2.735308036s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
   2.735331245s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1374208 = [("Pss_Anon", 294912), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
   2.735335287s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 896 MiB
   2.735388079s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2342461440), ("file_dirty", 939417600), ("file_writeback", 0), ("file_mapped", 77451264)]
   3.736466732s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
   3.736483816s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1374208 = [("Pss_Anon", 294912), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
   3.736486733s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 908 MiB
   3.736537775s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2438025216), ("file_dirty", 953339904), ("file_writeback", 0), ("file_mapped", 77451264)]
^C   4.079275058s  INFO demo_linux_dirty_file_pages: Ctrl-C received
   4.079290433s  INFO demo_linux_dirty_file_pages: setting stop flag
   4.079298808s  INFO demo_linux_dirty_file_pages: writer stopped, closing file descriptor
   4.736940291s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
   4.736989917s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1382400 = [("Pss_Anon", 303104), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
   4.736994042s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 864 MiB
   4.737044001s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2470465536), ("file_dirty", 906706944), ("file_writeback", 135168), ("file_mapped", 77451264)]
   5.738172023s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
   5.738187814s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1386496 = [("Pss_Anon", 307200), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
   5.738190440s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 833 MiB
   5.738233857s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2470871040), ("file_dirty", 873320448), ("file_writeback", 0), ("file_mapped", 77451264)]
   6.738670283s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
   6.738730200s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1386496 = [("Pss_Anon", 307200), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
   6.738738450s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 790 MiB
   6.738798659s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2471006208), ("file_dirty", 827092992), ("file_writeback", 0), ("file_mapped", 77451264)]
   7.739197748s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
   7.739270082s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1386496 = [("Pss_Anon", 307200), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
   7.739275207s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 712 MiB
   7.739325499s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2471006208), ("file_dirty", 745451520), ("file_writeback", 0), ("file_mapped", 77586432)]
   8.739651000s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
   8.739668792s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1390592 = [("Pss_Anon", 311296), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
   8.739671709s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 655 MiB
   8.739717043s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2471006208), ("file_dirty", 685436928), ("file_writeback", 135168), ("file_mapped", 77586432)]
   9.740060915s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
   9.740077707s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1390592 = [("Pss_Anon", 311296), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
   9.740080582s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 627 MiB
   9.740123457s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2471006208), ("file_dirty", 656510976), ("file_writeback", 0), ("file_mapped", 77586432)]
  10.741382797s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
  10.741404630s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1390592 = [("Pss_Anon", 311296), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
  10.741407547s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 600 MiB
  10.741460423s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2471006208), ("file_dirty", 628666368), ("file_writeback", 135168), ("file_mapped", 77586432)]
  11.741745871s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
  11.741763538s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1394688 = [("Pss_Anon", 315392), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
  11.741766871s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 563 MiB
  11.741871581s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2471006208), ("file_dirty", 590954496), ("file_writeback", 0), ("file_mapped", 77586432)]
  12.743828339s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
  12.743848589s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1394688 = [("Pss_Anon", 315392), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
  12.743851964s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 484 MiB
  12.743908632s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2471006208), ("file_dirty", 507961344), ("file_writeback", 0), ("file_mapped", 77586432)]
  13.744159614s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
  13.744174864s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1394688 = [("Pss_Anon", 315392), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
  13.744178156s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 405 MiB
  13.744258240s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2471006208), ("file_dirty", 424562688), ("file_writeback", 0), ("file_mapped", 77586432)]
  14.746015114s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
  14.746036072s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1394688 = [("Pss_Anon", 315392), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
  14.746039156s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 369 MiB
  14.746082656s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2471006208), ("file_dirty", 387796992), ("file_writeback", 270336), ("file_mapped", 77586432)]
  15.747184476s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
  15.747200268s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1394688 = [("Pss_Anon", 315392), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
  15.747203226s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 342 MiB
  15.747248935s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2471006208), ("file_dirty", 359817216), ("file_writeback", 270336), ("file_mapped", 77586432)]
  16.748601838s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
  16.748624047s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1394688 = [("Pss_Anon", 315392), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
  16.748627255s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 313 MiB
  16.748710465s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2471006208), ("file_dirty", 327782400), ("file_writeback", 135168), ("file_mapped", 77586432)]
  17.749531982s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
  17.749566399s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1394688 = [("Pss_Anon", 315392), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
  17.749569941s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 237 MiB
  17.749673109s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2471006208), ("file_dirty", 248573952), ("file_writeback", 0), ("file_mapped", 77586432)]
  18.750201994s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
  18.750389788s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1394688 = [("Pss_Anon", 315392), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
  18.750395663s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 163 MiB
  18.750454789s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2471006208), ("file_dirty", 170852352), ("file_writeback", 135168), ("file_mapped", 77586432)]
  19.750770251s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
  19.750779918s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1394688 = [("Pss_Anon", 315392), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
  19.750782918s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 127 MiB
  19.750843918s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2471006208), ("file_dirty", 133275648), ("file_writeback", 135168), ("file_mapped", 77586432)]
  20.751236461s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
  20.751260837s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1398784 = [("Pss_Anon", 319488), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
  20.751263670s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 102 MiB
  20.751311754s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2471006208), ("file_dirty", 107458560), ("file_writeback", 0), ("file_mapped", 77586432)]
  21.751778086s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
  21.751816212s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1398784 = [("Pss_Anon", 319488), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
  21.751821962s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 100 MiB
  21.751964380s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2472898560), ("file_dirty", 105566208), ("file_writeback", 0), ("file_mapped", 77586432)]
  22.752366000s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(756), rssanon: Some(140), rssfile: Some(616), rssshmem: Some(0), vmsize: Some(138744)
  22.752377416s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1398784 = [("Pss_Anon", 319488), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
  22.752380166s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 37 MiB
  22.752427584s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2473844736), ("file_dirty", 38658048), ("file_writeback", 0), ("file_mapped", 77586432)]
  23.254528605s  INFO demo_linux_dirty_file_pages: file descriptor closed, exiting
```

Use the `--dump-kstack-after-ctrlc` flag to see what we're waiting for

```
sudo ./target/release/demo-linux-dirty-file-pages  ~/tmp/blob 10Gi --dump-kstack-after-ctrlc
```


```
[<0>] __switch_to+0xc0/0x120
[<0>] add_transaction_credits+0x1f4/0x470 [jbd2]
[<0>] start_this_handle+0x100/0x630 [jbd2]
[<0>] jbd2__journal_start+0x118/0x244 [jbd2]
[<0>] __ext4_journal_start_sb+0x144/0x160 [ext4]
[<0>] ext4_writepages+0x2ec/0xe40 [ext4]
[<0>] do_writepages+0x5c/0xfc
[<0>] __filemap_fdatawrite_range+0x100/0x174
[<0>] filemap_flush+0x24/0x30
[<0>] ext4_alloc_da_blocks+0x34/0xa0 [ext4]
[<0>] ext4_release_file+0x80/0xf0 [ext4]
[<0>] __fput+0x88/0x264
[<0>] ____fput+0x18/0x24
[<0>] task_work_run+0xc4/0x17c
[<0>] do_notify_resume+0x254/0x930
[<0>] work_pending+0xc/0x618
```


# With fallocate, `close()`ing is fast

```
cs@devvm-mbp:[~/learn/demo-linux-dirty-file-pages]: ./target/release/demo-linux-dirty-file-pages  ~/tmp/blob 10Gi --fallocate
   0.000237336s  INFO demo_linux_dirty_file_pages: parsed args args=Args { path: "/home/cs/tmp/blob", size: Byte(10737418240), io_size_and_align: Byte(4096), direct: false, fallocate: true, dump_kstack_after_ctrlc: false, abort_on_ctrlc: false }
   0.008201271s  INFO demo_linux_dirty_file_pages: Initialized
   0.008665360s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(768), rssanon: Some(140), rssfile: Some(628), rssshmem: Some(0), vmsize: Some(138744)
   0.008687694s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1354752 = [("Pss_Anon", 274432), ("Pss_File", 1080320), ("Pss_Shmem", 0)]
   0.008693402s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 0 MiB
   0.008812529s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 1324105728), ("file_dirty", 946176), ("file_writeback", 0), ("file_mapped", 77586432)]
   1.009122254s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(768), rssanon: Some(140), rssfile: Some(628), rssshmem: Some(0), vmsize: Some(138744)
   1.009197547s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1374208 = [("Pss_Anon", 294912), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
   1.009202547s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 832 MiB
   1.009244089s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2257305600), ("file_dirty", 872103936), ("file_writeback", 135168), ("file_mapped", 77586432)]
   2.011459378s  INFO demo_linux_dirty_file_pages: /proc/self/status (inaccurate) vmrss: Some(768), rssanon: Some(140), rssfile: Some(628), rssshmem: Some(0), vmsize: Some(138744)
   2.011476170s  INFO demo_linux_dirty_file_pages: /proc/self/smaps_rollup (accurate): 1374208 = [("Pss_Anon", 294912), ("Pss_File", 1079296), ("Pss_Shmem", 0)]
   2.011479003s  INFO demo_linux_dirty_file_pages: system : nr_dirty: 858 MiB
   2.011525379s  INFO demo_linux_dirty_file_pages: my cgroup "/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope" memory.max="max" memory.high="max" memory.stat=[("file", 2354085888), ("file_dirty", 900624384), ("file_writeback", 0), ("file_mapped", 77586432)]
^C   2.683575120s  INFO demo_linux_dirty_file_pages: Ctrl-C received
   2.683595828s  INFO demo_linux_dirty_file_pages: setting stop flag
   2.683599328s  INFO demo_linux_dirty_file_pages: writer stopped, closing file descriptor
   2.683605245s  INFO demo_linux_dirty_file_pages: file descriptor closed, exiting
```
