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

#![warn(missing_docs)]

/// algorithms implementation
pub mod algs;
/// simple mixer to load and create mono tracks
pub mod mixer;

// muslib python module
mod pymod;
