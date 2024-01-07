/// harmonic pitch class profile
pub mod hpcp;
/// inverse fast Fourier transform
pub mod ifft;
/// short-time Fourier transform
pub mod stft;
/// synthesizer for simple waveforms
pub mod synth;

/// type alias for Variable data info
pub type DataInfo = str;

/// abstraction for all exported Algorithms
pub trait Algorithm {
    /// name of the Algorithm
    fn name(&self) -> &str;
    /// description of the Algorithm
    fn desc(&self) -> &str;

    // Algorithms have Inputs and Outputs
    /// vector with names of Inputs
    fn input_names(&self) -> Vec<&str>;
    /// informations for Input with a given name
    fn input_info(&self, key: &str) -> &DataInfo;
    /// vector with names of Outputs
    fn output_names(&self) -> Vec<&str>;
    /// informations for Input with a given name
    fn output_info(&self, key: &str) -> &DataInfo;

    // Configurables have Params
    /// vector with names of Params
    fn param_names(&self) -> Vec<&str>;
    /// informations for Param with a given name
    fn param_info(&self, key: &str) -> &DataInfo;

    // Algorithms you can always .compute()
    /// execute the Algorithm on given Inputs to produce some Outputs
    fn compute(&mut self);
}

/// every Input, Output, and Param used by an Algorithm
pub trait Variable {
    /// type representing the actual value of a Variable
    type Data;

    /// name of the Variable
    fn name(&self) -> &str;
    /// description of the Variable
    fn desc(&self) -> &str;
    /// the actual value of the Variable
    fn value(&self) -> &Option<Self::Data>;
    /// set value of the Variable to be x
    fn update(&mut self, x: Option<Self::Data>);

    /// information on data for this Variable
    ///
    /// This should contain:
    /// - type information
    /// - range of valid values
    fn data_info(&self) -> &str;
}

/// Var is a generic implementation of Variable
pub struct Var<T> {
    name: String,
    desc: String,
    value: Option<T>,
    data_info: String,
}

impl<T> Variable for Var<T> {
    type Data = T;

    fn name(&self) -> &str {
        &self.name
    }

    fn desc(&self) -> &str {
        &self.desc
    }

    fn value(&self) -> &Option<Self::Data> {
        &self.value
    }

    fn update(&mut self, x: Option<Self::Data>) {
        self.value = x
    }

    fn data_info(&self) -> &DataInfo {
        &self.data_info
    }
}
