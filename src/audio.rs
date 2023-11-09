use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

/// # Playing Audio Output
///
/// We want to output audio to the system output device.
///
/// We're using [cpal](https://docs.rs/cpal/latest/cpal/) to access the audio
/// device.
///
/// Implement the closure to write the contents of `audio_buffer` to the output
///
/// Hints:
/// - `output` is a different size and channel layout to `audio_buffer`
/// - `audio_buffer` is borrowed. Is it safe to move into the audio thread?

pub fn play(audio_buffer: &[f32], sample_rate: usize) {
    let duration = audio_buffer.len() as f64 / sample_rate as f64;
    let duration = std::time::Duration::from_secs_f64(duration);

    let device = cpal::default_host()
        .default_output_device()
        .expect("Error finding default output device");

    println!(
        "Output device: {}",
        device.name().expect("Error getting output device name")
    );

    let config = device
        .default_output_config()
        .expect("Error getting output device config");

    println!("Output config: {:?}", config);

    let (cpal::SampleFormat::F32 | cpal::SampleFormat::F64) = config.sample_format() else {
        panic!("Unsupported sample format");
    };

    let audio_buffer = audio_buffer.to_vec();
    let mut audio_buffer = audio_buffer.into_iter();
    let channel_count = config.config().channels as usize;

    let stream = device
        .build_output_stream(
            &config.config(),
            move |output: &mut [f32], _| {
                for frame in output.chunks_mut(channel_count) {
                    let value = audio_buffer.next().unwrap_or_default();

                    for sample in frame.iter_mut() {
                        *sample = value;
                    }
                }
            },
            |err| panic!("Error occurred on stream: {err}"),
            None,
        )
        .expect("Error creating output stream");

    stream.play().expect("Error playing output stream");

    std::thread::sleep(duration);
}
