use clap::{Args, Parser};

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
    /// Number of samples to render
    #[arg(long, short, default_value_t = 128)]
    buffer_size: usize,

    /// Frequency of the oscillator
    #[arg(long, short, default_value_t = 220.0)]
    frequency: f32,

    /// Cutoff frequency of the filter
    #[arg(long, short, default_value_t = 12_000.0)]
    cutoff: f32,
}

fn main() {
    match Commands::parse() {
        Commands::Play(dsp_args) => {
            println!("{dsp_args:#?}");
        }
        Commands::Plot(dsp_args) => {
            println!("{dsp_args:#?}");
        }
        Commands::Snapshot {
            file_name,
            dsp_args,
        } => {
            println!("{dsp_args:#?}");
            println!("Saving snapshot to: {file_name}");
        }
    }
}
