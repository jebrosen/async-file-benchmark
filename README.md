# async-file-benchmark

## Preface

This benchmark should *not* be used as-is as a performance comparision between
`async-std` and `tokio`. It was designed to highlight certain pathological
cases and does not reflect a realistic or recommended usage pattern.

There are several issues with using this as a direct comparative benchmark:

* The number of tasks and buffer size are unrealistic, and adjusting them can
  affect results dramatically.
* It uses default configurations of both libraries. These default
  configurations might not actually be suitable for this workload, and even
  under default *operating system configurations* this benchmark will not run.
* It uses arbitrary versions of the libraries, e.g. master branches at the time
  I last ran and subsequently committed changes. Using the latest master or
  latest releases is often more useful (and please indicate exact version
  numbers or commit hashes when discussing or comparing results for
  reproducibility).
    * At the time of this writing `tokio` is undergoing several major
      refactorings, and those results are likely to change significantly
      in the future.

The benchmark is most useful for tracking changes in executors, e.g. "did
change X really improve Y, as measured by this benchmark", and as a stress
test.

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
