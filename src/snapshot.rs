use crate::DspArguments;
use hound::{SampleFormat, WavSpec, WavWriter};

/// # Snapshots 💾
///
/// We want to be able to save our audio buffer to a .wav file so that we can play it back or
/// analyse it later.
///
/// We can use the [`hound` 🐕](https://docs.rs/hound/3.5.1/hound/) library for this. Check out the
/// example in the documentation that shows how to use `hound::WavWriter`.
///
/// Hints:
/// - The audio buffer is a single channel of 32-bit float samples.
/// - The sample rate is available from the `DspArguments` parameter.

pub(crate) fn save(file_name: &str, dsp_args: &DspArguments, audio_buffer: &[f32]) {
    let spec = WavSpec {
        channels: 1,
        sample_rate: dsp_args.sample_rate as u32,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };

    let file_name = format!("{file_name}.wav");
    let mut writer = WavWriter::create(file_name, spec).expect("failed to create a wav writer");

    for sample in audio_buffer {
        writer
            .write_sample(*sample)
            .expect("failed to write a sample to the file");
    }

    writer.finalize().expect("failed to update the wav header");
}
