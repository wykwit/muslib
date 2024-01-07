# muslib

Rust library for music synthesis and processing, inspired by [Essentia](https://essentia.upf.edu/).

It provides a few simple algorithms and utilities:

  - (WIP) tonal analysis with harmonic pitch class profile -- **HPCP**
  - (WIP) inverse fast Fourier transform -- **IFFT**
  - (WIP) short-time Fourier transform -- **STFT**
  - simple **mixer** to create mono tracks
  - **synth**esizer for simple waveforms

## Installation

You can download pre-built binaries from [the releases page](https://gitlab.com/wykwit/muslib/-/releases).
Otherwise see [build](#build) instructions for more details.

The Rust crate should be available through [crates.io](https://crates.io/).
You can add it with a simple:
```
$ cargo add muslib
```

There should also be a Python package available on [pypi.org](https://pypi.org).
You can install it with a simple:
```
$ pip install muslib
```

## Usage

This project provides a Rust library that can be used in other Rust projects
or from Python code with our [PyO3](https://pyo3.rs/) bindings.

See [docs](#docs) for a more detailed documentation
and [examples](#examples) to learn how you could use this project in your own code.

## Development

### build

You can use a simple `cargo` command to compile a release binary.
The output will be in the `target/release` directory.

```
$ cargo build --release
```

To compile a Python wheel use `maturin`.

```
$ maturin build
```

All project dependencies are listed in the `Cargo.toml` file.

### docs

You can generate docs with a simple `cargo` command.
The output will be in the `target/doc` directory.

```
$ cargo doc
```

The documentation should also be available through [docs.rs](https://docs.rs/).

### tests

```
$ cargo test
```

TODO: create build CI and link it here

### examples

You can run Rust examples from the `examples` directory with:
```
$ cargo run --example [name] [args]
```

You will also find Python examples there, including those comparing the usage of muslib and Essentia for simple tasks.

Some test scripts and helpers that were often run by hand during development are also there.

## alternatives

Other interesting Rust projects for audio DSP:

- [dasp](https://github.com/RustAudio/dasp)
- [fundsp](https://github.com/SamiPerttu/fundsp)

## license

In the same spirit as Essentia, this project is licensed under [AGPL-3.0](https://www.gnu.org/licenses/agpl-3.0.en.html).

```
Copyright (C) 2023 wykwit

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```
