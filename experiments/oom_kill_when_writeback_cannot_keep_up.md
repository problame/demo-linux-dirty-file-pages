In this experiment we're going to demonstrate how a process can get OOM killed
if there is enough reclaimable memory but writeback can't keep up.

Kernel: Debian bullseye `5.10.0-30-arm64`

# Setup

Create test cgroup
```
sudo mkdir /sys/fs/cgroup/test
sudo chown -R "$(id -u)":"$(id -g)" /sys/fs/cgroup/test
# 1 GiB memory.max
echo "$((1000 * 1024 * 1024))" > /sys/fs/cgroup/test/memory.max
```

```
bash
echo $$ | sudo tee /sys/fs/cgroup/test/cgroup.procs
./target/release/demo-linux-dirty-file-pages  ~/tmp/blob 10Gi
# with default sysctl  vm.dirty_ratio = 20, this should result in steady state nr_dirty=200MiB, file_dirty=202346496
```

Leave above command running and create a new terminal

```
mkdir ~/tmp/tmpfs
sudo mount -t tmpfs none ~/tmp/tmpfs
sudo chown "$(id -u)":"$(id -g)" ~/tmp/tmpfs
bash
echo $$ | sudo tee /sys/fs/cgroup/test/cgroup.procs
dd if=/dev/urandom of=~/tmp/tmpfs/allocation bs=1M count=500
# this should make nr_dirty drop to 100MiB because we the blob we wrote to tmpfs consumes 500MiB anon/shmem memory
#
# if we create a larger allocation, we'll see oom kills
# => go back to 0 allocation
rm ~/tmp/tmpfs/allocation*
# => do a bunch of small allocations that exhaust the cgroup memory piecemeal
for i in $(seq 1000); do dd if=/dev/urandom of=~/tmp/tmpfs/allocation.$i bs=1M count=1; sleep 0.01;  done
# usually the kernel oom killer will kill not just the last dd but also the demo-linux-dirty-file-pages process
```


