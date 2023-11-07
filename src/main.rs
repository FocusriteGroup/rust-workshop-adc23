mod audio;
mod dsp;

use clap::{Args, Parser};
use dsp::Waveform;

use crate::audio::play;

#[derive(Parser)]
enum Commands {
    /// Play the DSP from the audio output
    Play(DspArguments),

    /// Plot the DSP output to a graph
    Plot(DspArguments),

    /// Save a snapshot
    Snapshot {
        /// The file name of the snapshot
        file_name: String,

        #[command(flatten)]
        dsp_args: DspArguments,
    },
}

#[derive(Args, Debug)]
struct DspArguments {
    /// Sample rate to render with
    #[arg(long, short, default_value_t = 48_000)]
    sample_rate: usize,

    /// Number of samples to render
    #[arg(long, short, default_value_t = 128)]
    buffer_size: usize,

    /// Waveform of the oscillator
    #[arg(long, short, value_enum, default_value_t = Waveform::Sawtooth)]
    waveform: Waveform,

    /// Frequency of the oscillator
    #[arg(long, short, default_value_t = 220.0)]
    frequency: f32,

    /// Cutoff frequency of the filter
    #[arg(long, short, default_value_t = 12_000.0)]
    cutoff: f32,

    /// Attack time of the filter in seconds
    #[arg(long, short, default_value_t = 0.01)]
    attack: f32,

    /// Release time of the filter in seconds
    #[arg(long, short, default_value_t = 0.1)]
    release: f32,
}

fn render_dsp(args: &DspArguments) -> Vec<f32> {
    let mut buffer = vec![0.; args.buffer_size];

    let mut dsp = dsp::Dsp::default();
    dsp.prepare(args.sample_rate as u32);
    dsp.set_attack(args.attack);
    dsp.set_release(args.release);
    dsp.set_waveform(args.waveform);
    dsp.trigger(args.frequency, args.cutoff);
    dsp.render(&mut buffer);

    buffer
}

fn main() {
    match Commands::parse() {
        Commands::Play(dsp_args) => {
            let buffer = render_dsp(&dsp_args);
            play(&buffer, dsp_args.sample_rate);
        }
        Commands::Plot(dsp_args) => {
            let buffer = render_dsp(&dsp_args);
            println!("{buffer:#?}");
        }
        Commands::Snapshot {
            file_name,
            dsp_args,
        } => {
            let buffer = render_dsp(&dsp_args);
            println!("{buffer:#?}");
            println!("Saving snapshot to: {file_name}");
        }
    }
}
