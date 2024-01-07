use std::fs::File;
use std::path::PathBuf;

use hound::{WavSpec, WavWriter};

use symphonia::core::audio::{AudioBuffer, Signal};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::conv::ConvertibleSample;
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

/// Loader provides a facility for audio input.
///
/// That means you can
/// load files by their filenames,
/// mix down tracks from that file down to mono,
/// and finally inspect separate audio frames.
///
/// Essentia has a few layers of classes to provide this functionality:
///  AudioLoader -> MonoLoader -> EasyLoader.
/// In muslib this has been unified with a single Loader.
/// Essentia is using ffmpeg for the AudioLoader, we however are using Symphonia.
pub struct Loader<T> {
    file_path: PathBuf,
    gain: Option<f64>,
    channel: Option<usize>,
    track: Option<usize>,
    sample_rate: Option<u32>,
    data: Vec<T>,
}

impl<T: ConvertibleSample> Loader<T> {
    /// creates a new Loader instance with empty or default values
    pub fn new() -> Self {
        Loader {
            file_path: PathBuf::from(""),
            gain: None,        // will default to neutral
            channel: None,     // None will mix down all the channels to mono
            track: None,       // defaults to the first track
            sample_rate: None, // will be discovered on .load()
            data: Vec::new(),
        }
    }

    /// set path to a file that this Loader will read
    pub fn file(&mut self, file_path: PathBuf) -> &mut Self {
        self.file_path = file_path;
        self
    }

    /// set gain value that will be used while mixing all the channels
    pub fn gain(&mut self, gain: f64) -> &mut Self {
        self.gain = Some(gain);
        self
    }

    /// pick a single channel to read from
    pub fn channel(&mut self, channel: usize) -> &mut Self {
        self.channel = Some(channel);
        self
    }

    /// read all channels and mix them down to mono
    pub fn mono(&mut self) -> &mut Self {
        self.channel = None;
        self
    }

    /// execute the Loader to load and mix the data
    pub fn load(&mut self) -> Result<&Self, Error> {
        let file = File::open(self.file_path.to_owned())
            .expect("Could not open file path for this Loader.");

        let mut hint = Hint::new();
        if let Some(ext) = self.file_path.extension() {
            if let Some(ext_s) = ext.to_str() {
                hint.with_extension(ext_s);
            }
        }

        let mss_opts = MediaSourceStreamOptions::default();
        let mss = MediaSourceStream::new(Box::new(file), mss_opts);

        let format_opts = FormatOptions::default();
        let metadata_opts = MetadataOptions::default();
        let probe = symphonia::default::get_probe()
            .format(&hint, mss, &format_opts, &metadata_opts)
            .expect("Could not recognize and load this file format.");

        let mut format = probe.format;
        let track = self
            .track
            .and_then(|t| format.tracks().get(t))
            .or_else(|| {
                format
                    .tracks()
                    .iter()
                    .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            })
            .expect("Could not find any supported audio tracks.");
        let track_id = track.id;

        let decode_opts = DecoderOptions::default();
        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &decode_opts)
            .expect("The codec is unsupported.");

        // decode
        loop {
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                // finished reading the file
                Err(Error::ResetRequired) => {
                    return Ok(self);
                }
                Err(Error::DecodeError(_)) | Err(Error::IoError(_)) => {
                    return Ok(self);
                }
                // real errors
                Err(err) => {
                    return Err(err);
                }
            };

            if packet.track_id() != track_id {
                continue;
            }

            while !format.metadata().is_latest() {
                format.metadata().pop();
            }

            match decoder.decode(&packet) {
                Ok(decoded) => {
                    let spec = *decoded.spec();
                    if let Some(rate) = self.sample_rate {
                        if rate != spec.rate {
                            // sample rate mismatch error between reads, stream changed
                            return Err(Error::ResetRequired);
                        }
                    } else {
                        self.sample_rate = Some(spec.rate);
                    }

                    let mut data = AudioBuffer::<T>::new(decoded.capacity() as u64, spec);
                    decoded.convert(&mut data);

                    let len = decoded.frames(); // n of samples in each channel
                    let channels = data.spec().channels.count();

                    let mut buf = Vec::<T>::with_capacity(len);
                    let channel = data.chan(self.channel.unwrap_or(0));

                    for i in 0..len {
                        buf.push(self.apply_gain(channel[i]));
                    }

                    if self.channel.is_none() {
                        // mixing down to mono
                        for ch in 1..channels {
                            let channel = data.chan(ch);
                            for i in 0..len {
                                buf[i] = buf[i] + self.apply_gain(channel[i]);
                            }
                        }
                    }

                    self.data.append(&mut buf);
                }
                Err(Error::ResetRequired) => {
                    // stream changed, so we finished reading the file
                    return Ok(self);
                }
                Err(Error::DecodeError(_)) | Err(Error::IoError(_)) => {
                    // the packet can be discarded
                    continue;
                }
                Err(err) => return Err(err),
            }
        }
    }

    fn apply_gain(&self, x: T) -> T {
        // TODO
        x
    }

    /// read the loaded pcm data as a vector
    pub fn data(&self) -> Vec<T> {
        self.data.clone()
    }

    /// read the loaded sample rate
    pub fn sample_rate(&self) -> Option<u32> {
        self.sample_rate
    }
}

/// Writer provides a facility for audio output.
///
/// Essentia is using ffmpeg for the AudioWriter,
/// we however are using hound and for now only support output to simple 16-bit WAV files.
pub struct Writer {
    file_path: PathBuf,
    spec: WavSpec,
}

impl Writer {
    /// creates a new Writer instance with empty or default values
    pub fn new() -> Self {
        Writer {
            file_path: PathBuf::from(""),
            spec: WavSpec {
                channels: 1,
                sample_rate: 44100,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            },
        }
    }

    /// set path to a file that this Writer will use
    pub fn file(&mut self, file_path: PathBuf) -> &mut Self {
        self.file_path = file_path;
        self
    }

    /// set sample rate that will be used with this Writer
    pub fn sample_rate(&mut self, sample_rate: u32) -> &mut Self {
        self.spec.sample_rate = sample_rate;
        self
    }

    /// execute the Writer to store data in a file
    pub fn write(&self, data: &Vec<u16>) -> Result<(), ()> {
        let mut writer = WavWriter::create(self.file_path.to_owned(), self.spec)
            .expect("Failed to create a file for the Writer.");

        for t in data.iter() {
            let t = (*t ^ 0x8000) as i16; // hack for the sign conversion
            writer
                .write_sample(t)
                .expect("Failed to write an output sample.");
        }

        return Ok(());
    }
}
