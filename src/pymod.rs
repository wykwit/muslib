use pyo3::prelude::*;

use crate::algs::*;

#[pymodule]
/// Rust library for music synthesis and processing, inspired by Essentia.
fn muslib(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<hpcp::HPCP>()?;
    m.add_class::<io::MonoLoader>()?;
    m.add_class::<io::MonoWriter>()?;
    m.add_class::<stft::FFT>()?;
    m.add_class::<stft::IFFT>()?;
    m.add_class::<synth::Synthesizer>()?;
    Ok(())
}
