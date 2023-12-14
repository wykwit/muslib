//! Rust library for music synthesis and processing, inspired by [Essentia](https://essentia.upf.edu/).
//!
//! It provides a few simple algorithms and utilities:
//!
//!   - tonal analysis with harmonic pitch class profile -- **HPCP**
//!   - inverse fast Fourier transform -- **IFFT**
//!   - short-time Fourier transform -- **STFT**
//!   - simple **mixer** to create mono tracks
//!   - **synth**esizer for simple waveforms
//!
//! This should be sufficient to allow for flexible synthesis, processing and analysis of audio.
//! 
//! ```
//! println!("TODO: sample rust code")
//! ```

#![warn(missing_docs)]

/// algorithms implementation
pub mod algs;
/// provides a command line interface
mod cli;
/// utilities to load samples and cut them up into frames
pub mod frames;
/// simple mixer to create mono tracks
pub mod mixer;
/// synthesizer for simple waveforms
pub mod synth;

// re-exports
pub use crate::cli::start;
