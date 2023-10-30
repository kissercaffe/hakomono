
# Practice creating a custom container engine. 

This page contains the tutorial content.
https://litchipi.github.io/series/container_in_rust

The license follows the link below.
https://github.com/litchipi/crabcan


# How to use
```
mkdir -p ./mountdir
cargo build
cp /bin/bash ./mountdir
cp /bin/ls ./mountdir
sudo ./target/debug/hakomono --debug -u 0 -m ./mountdir/ -c "/bash" -a /lib64:/lib64 -a /lib:/lib
```

If your OS is using cgroup v1, you need to configure it to use v2.