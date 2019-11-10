# async-file-benchmark

NOTICE: This probably does not benchmark what anyone thinks it does. The I/O
pattern and the benchmark itself is fairly contrived and semi-purposefully
anti-optimized. The only thing I would really trust with this benchmark is to
compare different commits of each project against themselves to test changes
in schedulers or threadpool handling.

## Setup

You'll need a `file.dat`. For example, 256KiB of random data:

```
dd if=/dev/urandom of=file.dat count=256 bs=1024
```

You will probably need to raise your open files limit to run this:

```
ulimit -n unlimited
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
