use crate::DspArguments;

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
    todo!("Implement the functionality for saving the audio_buffer to a .wav file using hound")
}
