# iobenchy
Benchmark file and network latency and bandwidth.

## Status
This README.md is used as a design document as I plan out features.

- [x] Sketch out goals in README.md
- [x] Disk IO
  - [x] Write specified number of random bytes to a file
  - [x] Time how long it takes to write the data
  - [ ] Break down writes into various sizes
- [ ] Network IO
  - [ ] Do anything
- [ ] General
  - [ ] Add verbosity levels
    - [ ] Verbose (default) - Show messages about what is happening as the benchmark runs
    - [ ] Quiet - Just the results
    - [ ] Debug - (maybe?) More info about what is happening
  - [ ] Option to write results table or JSON to a file
  - [ ] Exit codes (e.g. 0 for success, 1 for error)
  - [ ] Display results in a table (see "Storage IO" below for desired format)
  - [ ] Option to print results as JSON instead of a table
  - [ ] Parse units for the number of bytes (KB, MB, MiB, etc.)

"Screenshot" of Current Status
```
iobenchy file --bytes 1000000000
Benchmarking disk IO for './'
Generating 1000000000 bytes of random data.
Generated 1000000000 bytes of random data in 1.884408232s.
Writing 1000000000 bytes of random data to ./.iobenchy.tmp1.
Wrote 1000000000 bytes of random data to ./.iobenchy.tmp1 in 617.501031ms.
```

## About
The iobenchy command is designed to get a rough estimate of file and network
performance using a single binary.

## Usage

### Storage IO
~~~
iobenchy file --path=/mnt/somevolume --bytes=4GB

Write Size    IOPS    Bandwidth  Queue Depth
     4 kiB   1,000       4 MB/s  1
     1 MiB     100     100 MB/s  1
     4 MiB      50     200 MB/s  1
~~~

### Network IO

System A:
~~~
iobenchy network --server --port=8080

Waiting for client...
~~~

System B:
~~~
iobenchy network --client --port=8080 --bytes=4GB

Packet Size   Latency    Bandwidth  Parallel
      4 kiB      1 ms       4 MB/s  1
      1 MiB    100 ms      10 MB/s  1
      4 MiB    400 ms      10 MB/s  1
~~~
