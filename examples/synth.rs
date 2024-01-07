use muslib::algs::*;

fn main() {
    let mut s = synth::Synthesizer::new();
    s.envelope = vec![0.1, 0.02, 0.2, 0.6, 0.1];
    s.freq = vec![440.0, 880.0, 660.0, 0.0]; // A_4, A_5, E_5, silence
    s.durations = vec![0.8, 0.7, 1.0, 0.5];
    s.compute();

    let mut w = io::MonoWriter::new();
    w.file = "test.wav".into();
    w.pcm_data = s.pcm_data.unwrap();
    w.compute()
}
