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
    /// TODO: Add "buffer_size" as CLI argument here

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
        Commands::Snapshot { .. } => {
            todo!("Print `file_name` and `dsp_args`");
        }
        // "_" is the "catch all" case, like `default:` in a C/C++ switch statement,
        _ => todo!("Handle the plot command"),
    }
}
