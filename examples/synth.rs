use muslib::algs::{synth, Algorithm, Variable};
use muslib::mixer::Writer;

fn main() {
    let mut s = synth::new();

    // params
    s.envelope.update(Some(vec![0.1, 0.02, 0.2, 0.6, 0.1]));

    // input
    s.freq.update(Some(vec![440.0, 880.0, 660.0, 0.0])); // A_4, A_5, E_5, silence
    s.durations.update(Some(vec![0.8, 0.7, 1.0, 0.5]));
    s.compute();

    // output
    let out = s.pcm_data.value().as_ref().unwrap();

    // save
    let mut w = Writer::new();
    w.file("test.wav".into())
        .sample_rate(44100)
        .write(&out)
        .unwrap();
}
