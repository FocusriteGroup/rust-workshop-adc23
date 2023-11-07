mod dsp;
mod state;

use clap::{Args, Parser};
use dsp::Waveform;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Parser)]
enum Commands {
    /// Play the DSP from the audio output
    Play {
        #[arg(long, conflicts_with = "DspArguments")]
        dsp_file: Option<PathBuf>,

        #[command(flatten)]
        dsp_args: DspArguments,
    },

    /// Plot the DSP output to a graph
    Plot {
        #[arg(long, conflicts_with = "DspArguments")]
        dsp_file: Option<PathBuf>,

        #[command(flatten)]
        dsp_args: DspArguments,
    },

    /// Save a snapshot
    Snapshot {
        /// The file name of the snapshot
        file_name: String,

        #[arg(long, conflicts_with = "DspArguments")]
        dsp_file: Option<PathBuf>,

        #[command(flatten)]
        dsp_args: DspArguments,
    },
}

#[derive(Args, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DspArguments {
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
        Commands::Play { dsp_file, dsp_args } => {
            let args = dsp_file.map_or(dsp_args, state::load);
            let buffer = render_dsp(&args);
            println!("{buffer:#?}");
        }
        Commands::Plot { dsp_file, dsp_args } => {
            let args = dsp_file.map_or(dsp_args, state::load);
            let buffer = render_dsp(&args);
            println!("{buffer:#?}");
        }
        Commands::Snapshot {
            file_name,
            dsp_file,
            dsp_args,
        } => {
            let args = dsp_file.map_or(dsp_args, state::load);
            let buffer = render_dsp(&args);
            println!("{buffer:#?}");
            println!("Saving snapshot to: {file_name}");
        }
    }
}
