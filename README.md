# run-length-encode-rs
Entry for [BBC Academy summer '22 coding challenge](https://broxy.tools.bbc.co.uk/broxy-kata/).

### Building
1. Install [rust](https://www.rust-lang.org/tools/install).
2. Build the executable with `cargo build --release`. By default it will be located in `target/release/rle`.

### Usage
```
rle mono <inputfile>
    monochrome image encoding
```
```
rle grey <inputfile>
    greyscale (hex shade) image encoding
```
```
rle grey n <inputfile>
    greyscale (hex shade) image encoding with a compression factor (0 to 15)
```

Input files must use [the bespoke kata file format](https://broxy.tools.bbc.co.uk/broxy-kata/).

### Future work
- Mono/hex decoders
- Benchmarking
- Divide & Conquer encoding: can it be done faster if we parallelise encoding sections of the file?
