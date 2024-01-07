/// harmonic pitch class profile
pub mod hpcp;
/// inverse fast Fourier transform
pub mod ifft;
/// input and output with wav files
pub mod io;
/// short-time Fourier transform
pub mod stft;
/// synthesizer for simple waveforms
pub mod synth;

/// abstraction for all exported Algorithms
pub trait Algorithm {
    /// create a new instance of the Algorithm with default parameters
    fn new() -> Self;
    /// compute the Algorithm for given Inputs to produce some Outputs
    fn compute(&mut self);
}
