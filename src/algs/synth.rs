use std::f64::consts::PI;

/// waveforms supported by the tone generator
pub enum Waveform {
    /// sinusoidal wave
    Sin,
    /// square wave
    Square,
    /// sawtooth wave
    Sawtooth
}

/// tone generator with a given frequency and sample rate
pub struct Generator {
    freq: f64,
    sample_rate: f64,
    waveform: Waveform,
}

impl Generator {
    /// create a new tone generator 
    pub fn new(freq: f64, sample_rate: Option<f64>, w: Option<Waveform>) -> Self {
        Generator {
            freq,
            sample_rate: sample_rate.unwrap_or(44100.0),
            waveform: w.unwrap_or(Waveform::Sin),
        }
    }
    
    /// change the tone frequency for this tone generator
    pub fn freq(&mut self, f: f64) -> &Self {
        self.freq = f;
        self
    }

    /// change the sample rate for this tone generator
    pub fn sample_rate(&mut self, sample_rate: f64) -> &Self {
        self.sample_rate = sample_rate;
        self
    }
    
    /// change the tone waveform type for this tone generator
    pub fn w(&mut self, w: Waveform) -> &Self {
        self.waveform = w;
        self
    }
    
    /// amplitude value of the sinusoidal wave tone for a sample x
    fn sin(&self, x: f64) -> f64 {
        let x: f64 = PI * 2.0 * x * self.freq / self.sample_rate;
        x.sin()
    }

    /// amplitude value of the square wave tone for a sample x
    fn sqr(&self, x: f64) -> f64 {
        if (2.0 * x * self.freq / self.sample_rate) % 2.0 < 1.0 {
            1.0
        } else {
            -1.0
        }
    }

    /// amplitude of the sawtooth wave tone for a sample x
    fn saw(&self, x: f64) -> f64 {
        let x: f64 = x * self.freq / self.sample_rate;
        2.0 * (x - x.floor()) - 1.0
    }

    /// amplitude value from range <-1; 1> of the tone for a sample x
    pub fn amplitude(&self, x: usize) -> f64 {
        let x = x as f64;
        match self.waveform {
            Waveform::Sin => self.sin(x),
            Waveform::Square => self.sqr(x),
            Waveform::Sawtooth => self.saw(x),
        }
    }

    /// sample number for time given in seconds
    pub fn time(&self, t: f64) -> usize {
        (t * self.sample_rate).ceil() as usize
    }
}

/// linear envelope used for wavetable generation
pub struct Envelope {
    /// attack - time duration in seconds
    pub a: f64,
    /// hold - time duration in seconds
    pub h: f64,
    /// decay - time duration in seconds
    pub d: f64,
    /// sustain - amplitude level maintained until the key is released
    pub s: f64,
    /// release - time duration in seconds
    pub r: f64,
}

impl Envelope {
    /// create a new ADSR envelope
    pub fn adsr(a: f64, d: f64, s: f64, r: f64) -> Self {
        Envelope { a, h: 0.0, d, s, r }
    }

    /// find a multiplier that should be applied to the tone at point x
    /// 
    /// - for a known duration pass the amount of samples
    /// - for unknown duration pass 0 and the release will not be applied
    pub fn multiplier(&self, g: &Generator, x: usize, duration: usize) -> f64 {
        // convert time in seconds to samples and find which function slope to apply
        let a = g.time(self.a);
        if x < a {
            return (x as f64) / (a as f64);
        }

        let h = a + g.time(self.h);
        if x < h {
            return 1.0;
        }

        let d = g.time(self.d);
        if x < h + d {
            let x = x - h;
            return 1.0 - (x as f64) * (1.0 - self.s) / (d as f64);
        }

        let r = g.time(self.r);
        if duration > 0 && x > duration - r {
            let x = x - (duration - r);
            return self.s - (x as f64) * (self.s) / (r as f64);
        }

        return self.s;
    }
}

/// wavetable generator
pub struct Wavetable {
    /// base tone generator
    pub generator: Generator,
    /// envelope applied to the base tone
    pub envelope: Option<Envelope>,
    /// number of samples to be generated
    pub samples: Option<usize>,
}

impl Wavetable {
    /// set the number of samples to be generated based on time duration in seconds
    pub fn time(&mut self, t: f64) -> &Self {
        self.samples = Some(self.generator.time(t));
        self
    }

    /// generate a wavetable of u16 type samples 
    pub fn u16(&self) -> Vec<u16> {
        let n = self.samples
            .expect("Lenght for the Wavetable synth output is not set. Call .time() first.");

        let mut output: Vec<u16> = Vec::with_capacity(n);

        let m = (u16::MAX / 2) as f64;
        let g = &self.generator;
        for i in 0..n {
            let f = match &self.envelope {
                Some(e) => e.multiplier(g, i, n),
                None => 1.0,
            };
            let v = m + f * g.amplitude(i) * m;
            output.push(v.round() as u16);
        }
        output
    }
}

// TODO: Algorithm trait implementation

#[cfg(test)]
mod tests {
    use super::{Envelope, Generator, Waveform, Wavetable};

    #[test]
    fn generator() {
        let input = [22, 33, 55, 77];
        let result = [
            // sin
            [
                0.9816950853641806,
                0.8785620487276837,
                -0.30155494042174996,
                -0.9934299572800557,
            ],
            // sqr
            [1.0, 1.0, -1.0, -1.0],
            // saw
            [
                -0.5609977324263038,
                -0.3414965986394558,
                0.0975056689342404,
                0.5365079365079366,
            ],
        ];

        let mut g = Generator::new(440.0, Some(44100.0), None);
        for i in 0..input.len() {
            g.w(Waveform::Sin);
            assert_eq!(g.amplitude(input[i]), result[0][i], "test {}", i);
            g.w(Waveform::Square);
            assert_eq!(g.amplitude(input[i]), result[1][i], "test {}", i);
            g.w(Waveform::Sawtooth);
            assert_eq!(g.amplitude(input[i]), result[2][i], "test {}", i);
        }
    }
    
    #[test]
    fn envelope() {
        let input = [
        //  ([a,   h,   d,   s,   r  ], [x, duration])
            ([0.0, 0.0, 0.0, 0.0, 0.0], [0_usize, 0_usize]),
            ([0.0, 0.0, 0.0, 0.8, 0.0], [10, 0]),       // s
            ([1.0, 0.0, 0.0, 0.0, 0.0], [500, 0]),      // a
            ([1.0, 5.0, 0.0, 0.0, 0.0], [1500, 0]),     // h
            ([1.0, 0.0, 0.0, 0.0, 0.0], [1500, 0]),     // h
            ([1.0, 0.0, 1.0, 0.0, 0.0], [1500, 0]),     // d
            ([1.0, 0.0, 1.0, 0.8, 0.0], [2500, 0]),     // s
            ([1.0, 0.0, 1.0, 0.0, 0.0], [2500, 0]),     // s
            ([1.0, 1.0, 1.0, 0.5, 0.0], [2500, 0]),     // d
            ([1.0, 1.0, 1.0, 0.5, 1.0], [3500, 4000]),  // r
            ([1.0, 1.0, 1.0, 1.0, 1.0], [3500, 4000]),  // r
            ([1.0, 1.0, 1.0, 0.8, 1.0], [3500, 0]),     // r
        ];

        let result = [
            // multiplier: f64
            0.0,    // sanity
            0.8,    // s
            0.5,    // a
            1.0,    // h
            0.0,    // h
            0.5,    // d
            0.8,    // s
            0.0,    // s
            0.75,   // d
            0.25,   // r
            0.5,    // r
            0.8,    // r
        ];
    
        let g = Generator::new(440.0, Some(1000.0), None);
        for i in 0..input.len() {
            let e = Envelope {
                a: input[i].0[0],
                h: input[i].0[1],
                d: input[i].0[2],
                s: input[i].0[3],
                r: input[i].0[4],
            };
            let x = input[i].1[0];
            let duration = input[i].1[1];
            assert_eq!(e.multiplier(&g, x, duration), result[i], "test {}", i);
        }
    }

    #[test]
    fn wavetable() {
        let result: Vec<u16> = vec![32767, 32836, 33028, 33296, 33572, 33778, 33844, 33715, 33370, 32825, 32134, 31386, 30692, 30169, 29922, 30030, 30524, 31384, 32536, 33853, 35174, 36324, 37131, 37456, 37214, 36387, 35034, 33287, 31341, 29429, 27797, 26670, 26227, 26565, 27691, 29513, 31843, 34420, 36937, 39078, 40558, 41159, 40764, 39373, 37108, 34209, 31002, 27867, 25193, 23325, 22527, 22940, 24562, 27242, 30695, 34529, 38292, 41523, 43811, 44844, 44453, 42638, 39571, 35581, 31124, 26724, 22914, 20174, 18869, 19197, 21169, 24594, 29100, 34174, 39220, 43628, 46850, 48466, 48239, 46148, 42397, 37395, 31713, 26016, 20991, 17257, 15294, 15379, 17551, 21596, 27071, 33352, 39703, 45362, 49636, 51983, 52079, 49866, 45560, 39635, 32767, 25760, 19452, 14611, 11846, 11528, 13744, 18276, 24625, 32066, 39728, 46700, 52133, 55351, 55929, 53751, 49029, 42283, 34284, 25968, 18322, 12272, 8567, 7689, 9789, 14666, 21780, 30319, 39286, 47616, 54306, 58530, 59746, 57764, 52772, 45319, 36258, 26647, 17624, 10274, 5499, 3905, 5728, 10800, 18560, 28122, 38370, 48092, 56121, 61477, 63486, 61863, 56752, 48717, 38677, 27801, 17376, 8649, 2682, 221, 1604, 6957, 15319, 25686, 36822, 47410, 56205, 62181, 64652, 63348, 58448, 50552, 40610, 29809, 19426, 10683, 4601, 1878, 2815, 7275, 14711, 24225, 34683, 44845, 53515, 59682, 62634, 62047, 58012, 51030, 41943, 31837, 21911, 13332, 7104, 3943, 4201, 7824, 14361, 23020, 32767, 42444, 50912, 57181, 60528, 60579, 57353, 51254, 43020, 33639, 24224, 15885, 9597, 6085, 5744, 8589, 14262, 22072, 31082, 40223, 48416, 54702, 58355, 58966, 56486, 51230, 43841, 35205, 26350, 18321, 12057, 8283, 7423, 9555, 14405, 21378, 29636, 38195, 46045, 52266, 56139, 57228, 55426, 50971, 44409, 36531, 28277, 20622, 14463, 10512, 9216, 10704, 14778, 20935, 28431, 36371, 43817, 49895, 53903, 55386, 54192, 50486, 44728, 37614, 29995, 22772, 16793, 12750, 11102, 12019, 15370, 20738, 27470, 34762, 41748, 47610, 51668, 53461, 52801, 49790, 44804, 38452, 31494, 24756, 19028, 14976, 13059, 13480, 16167, 20780, 26755, 33375, 39854, 45430, 49457, 51477, 51274, 48896, 44646, 39045, 32767, 26558, 21149, 17167, 15064, 15068, 17154, 21052, 26283, 32217, 38146, 43373, 47292, 49455, 49631, 47821, 44263, 39396, 33809, 28168, 23137, 19217, 16898, 16456, 17943, 21182, 25791, 31225, 36841, 41976, 46022, 48500, 49118, 47803, 44710, 40205, 34820, 29193, 23988, 19821, 17185, 16392, 17534, 20478, 24874, 30204, 35837, 41107, 45391, 48182, 49151, 48182, 45391, 41107, 35837, 30204, 24874, 20478, 17534, 16392, 17185, 19821, 23988, 29193, 34820, 40205, 44710, 47803, 49118, 48500, 46022, 41976, 36841, 31225, 25791, 21182, 17943, 16456, 16898, 19217, 23137, 28196, 33796, 39274, 43982, 47365, 49021, 48756, 46600, 42809, 37830, 32252, 26736, 21932, 18410, 16585, 16674, 18665, 22324, 27217, 32767, 38317, 43210, 46869, 48860, 48949, 47124, 43602, 38798, 33282, 27704, 22725, 18934, 16778, 16513, 18169, 21552, 26260, 31738, 37338, 42397, 46317, 48636, 49078, 47591, 44352, 39743, 34309, 28693, 23558, 19512, 17034, 16416, 17731, 20824, 25329, 30714, 36341, 41546, 45713, 48349, 49142, 48000, 45056, 40660, 35330, 29697, 24427, 20143, 17352, 16384, 17352, 20143, 24427, 29697, 35330, 40660, 45056, 48000, 49142, 48349, 45713, 41546, 36341, 30714, 25329, 20824, 17731, 16416, 17034, 19512, 23558, 28693, 34309, 39743, 44352, 47591, 49078, 48636, 46317, 42397, 37338, 31738, 26260, 21552, 18169, 16513, 16778, 18934, 22725, 27704, 33282, 38798, 43602, 47124, 48949, 48860, 46869, 43210, 38317, 32767, 27217, 22324, 18665, 16674, 16585, 18410, 21932, 26736, 32252, 37830, 42809, 46600, 48756, 49021, 47365, 43982, 39274, 33796, 28196, 23137, 19217, 16898, 16456, 17943, 21182, 25791, 31225, 36841, 41976, 46022, 48500, 49118, 47803, 44710, 40205, 34820, 29193, 23988, 19821, 17185, 16392, 17534, 20478, 24874, 30204, 35837, 41107, 45391, 48182, 49151, 48182, 45391, 41107, 35837, 30204, 24874, 20478, 17534, 16392, 17185, 19821, 23988, 29193, 34820, 40205, 44710, 47803, 49118, 48500, 46022, 41976, 36841, 31225, 25791, 21182, 17943, 16456, 16898, 19217, 23137, 28196, 33796, 39274, 43982, 47365, 49021, 48756, 46600, 42809, 37830, 32252, 26736, 21932, 18410, 16585, 16674, 18665, 22324, 27217, 32767, 38317, 43210, 46869, 48860, 48949, 47124, 43602, 38798, 33282, 27704, 22725, 18934, 16778, 16513, 18169, 21552, 26260, 31738, 37338, 42397, 46317, 48636, 49078, 47591, 44352, 39743, 34309, 28693, 23558, 19512, 17034, 16416, 17731, 20824, 25329, 30714, 36341, 41546, 45713, 48349, 49040, 47810, 44826, 40462, 35250, 29812, 24792, 20774, 18219, 17407, 18412, 21090, 25105, 29966, 35090, 39871, 43751, 46286, 47198, 46401, 44013, 40339, 35827, 31022, 26491, 22765, 20268, 19277, 19886, 21998, 25342, 29507, 33991, 38260, 41818, 44256, 45306, 44867, 43015, 39989, 36167, 32008, 28009, 24636, 22275, 21186, 21475, 23084, 25801, 29286, 33118, 36838, 40013, 42278, 43386, 43228, 41845, 39425, 36270, 32767, 29333, 26371, 24218, 23111, 23159, 24332, 26469, 29299, 32474, 35615, 38353, 40375, 41461, 41504, 40522, 38655, 36142, 33294, 30453, 27952, 26076, 25031, 24917, 25725, 27337, 29541, 32064, 34600, 36853, 38566, 39552, 39716, 39063, 37694, 35789, 33588, 31360, 29365, 27832, 26924, 26729, 27245, 28389, 30005, 31886, 33803, 35530, 36870, 37681, 37887, 37488, 36554, 35217, 33650, 32046, 30596, 29464, 28768, 28571, 28872, 29612, 30682, 31941, 33229, 34394, 35305, 35868, 36037, 35815, 35252, 34436, 33480, 32507, 31633, 30957, 30543, 30422, 30585, 30989, 31563, 32224, 32883, 33458, 33889, 34136, 34189, 34066, 33804, 33457, 33083, 32738, 32465, 32293, 32229, 32261, 32365, 32503, 32636, 32732];

        let t = Wavetable {
            generator: Generator::new(440.0, Some(8000.0), None),
            envelope: Some(Envelope::adsr(0.02, 0.02, 0.5, 0.02)),
            samples: Some(800),
        };

        assert_eq!(t.u16(), result);
    }
}
