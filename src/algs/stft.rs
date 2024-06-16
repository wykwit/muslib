use pyo3::{pyclass, pymethods};
use symphonia::core::dsp::complex::Complex;
use symphonia::core::dsp::fft::Fft;

use super::Algorithm;

#[pyclass(get_all)]
pub struct FFT {
    /// Input: list[float] -- audio input frame, max len 65535
    #[pyo3(set)]
    pub frame: Vec<f64>,
    /// Output: list[tuple[float, float]] -- fft data
    pub fft_data: Vec<(f32, f32)>,
}

#[pymethods]
impl FFT {
    #[new]
    fn pynew() -> Self {
        FFT {
            frame: Vec::new(),
            fft_data: Vec::new(),
        }
    }

    /// Compute the Algorithm
    ///
    /// Inputs:
    ///   - frame: list[float]
    ///
    /// Outputs:
    ///   - fft_data: list[tuple[float, float]]
    ///
    /// See data descriptors for more details.
    #[pyo3(name = "compute", signature = (frame = None))]
    fn pycompute(&mut self, frame: Option<Vec<f64>>) -> Vec<(f32, f32)> {
        if let Some(arg) = frame {
            self.frame = arg
        }

        self.compute();

        self.fft_data.clone()
    }
}

impl Algorithm for FFT {
    fn new() -> Self {
        Self::pynew()
    }

    fn compute(&mut self) {
        // contruct a buffer of complex numbers
        let mut buf: [Complex; Fft::MAX_SIZE] = [Complex { re: 0.0, im: 0.0 }; Fft::MAX_SIZE];
        let n = std::cmp::min(self.frame.len(), Fft::MAX_SIZE);
        let buf = &mut buf[0..n];
        for i in 0..n {
            buf[i].re = self.frame[i] as f32;
        }

        let fft = Fft::new(n);
        fft.fft_inplace(buf);

        // convert and store the buffer as output
        self.fft_data = Vec::with_capacity(n);
        for x in buf.iter().take(n) {
            self.fft_data.push((x.re, x.im))
        }
    }
}

#[pyclass(get_all)]
pub struct IFFT {
    /// Input: list[tuple[float, float]] -- fft data, max len 65535
    #[pyo3(set)]
    pub fft_data: Vec<(f64, f64)>,
    /// Output: list[float] -- the IFFT of the input frame
    pub frame: Vec<f32>,
}

#[pymethods]
impl IFFT {
    #[new]
    fn pynew() -> Self {
        IFFT {
            fft_data: Vec::new(),
            frame: Vec::new(),
        }
    }

    /// Compute the Algorithm
    ///
    /// Inputs:
    ///   - fft_data: list[tuple[float, float]]
    ///
    /// Outputs:
    ///   - frame: list[float]
    ///
    /// See data descriptors for more details.
    #[pyo3(name = "compute", signature = (fft_data = None))]
    fn pycompute(&mut self, fft_data: Option<Vec<(f64, f64)>>) -> Vec<f32> {
        if let Some(arg) = fft_data {
            self.fft_data = arg
        }

        self.compute();

        self.frame.clone()
    }
}

impl Algorithm for IFFT {
    fn new() -> Self {
        Self::pynew()
    }

    fn compute(&mut self) {
        // contruct a buffer of complex numbers
        let mut buf: [Complex; Fft::MAX_SIZE] = [Complex { re: 0.0, im: 0.0 }; Fft::MAX_SIZE];
        let n = std::cmp::min(self.fft_data.len(), Fft::MAX_SIZE);
        let buf = &mut buf[0..n];
        for i in 0..n {
            buf[i].re = self.fft_data[i].0 as f32;
            buf[i].im = self.fft_data[i].1 as f32;
        }

        let fft = Fft::new(n);
        fft.ifft_inplace(buf);

        // convert and store the buffer as output
        self.frame = Vec::with_capacity(n);
        for x in buf.iter().take(n) {
            self.frame.push(x.re)
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn fft() {
        // TODO
    }

    #[test]
    fn ifft() {
        // TODO
    }
}
