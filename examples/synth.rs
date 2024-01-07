use muslib::algs::synth;
use muslib::mixer::Writer;

fn main() {
    let mut g = synth::Wavetable {
        generator: synth::Generator::new(440.0, None, Some(synth::Waveform::Sin)),
        envelope: Some(synth::Envelope {
            a: 0.1,
            h: 0.02,
            d: 0.2,
            s: 0.6,
            r: 0.1,
        }),
        samples: None,
    };

    // we start with A_4 at 440Hz
    let mut out = g.time(0.8).u16();

    g.generator.freq(880.0); // A_5
    let mut t = g.time(0.7).u16();
    out.append(&mut t);

    g.generator.freq(660.0); // E_5
    let mut t = g.time(1.0).u16();
    out.append(&mut t);

    g.generator.freq(0.0); // silence
    let mut t = g.time(1.0).u16();
    out.append(&mut t);

    let mut w = Writer::new();
    w.file("test.wav".into())
        .sample_rate(44100)
        .write(&out)
        .unwrap();
}
