use plotly::{Plot, Scatter};

// The `_` prefix on `_sample_rate` tells the compiler
// that we are ignored the unused variable, similar to
// a void-cast in C/C++
pub fn generate(audio_buffer: &[f32], _sample_rate: usize) {
    let mut plot = Plot::new();

    let time_trace = Scatter::new((0..audio_buffer.len()).collect(), audio_buffer.to_vec())
        .name("time")
        .x_axis("x1")
        .y_axis("y1");
    plot.add_trace(time_trace);

    todo!("1. Generate the FFT using `Fft::process`");
    todo!("2. Create the FFT trace and add it to the plot");
    todo!("3. Create a layout with 2 rows and 1 column");
    // hint: have a look at the Plotly recipes here
    // https://igiagkiozis.github.io/plotly/content/recipes/subplots/subplots.html

    plot.show();
}

// Note to WSL users:
//
// you might encounter and issue with
//
//      plot.show();
//
// in that case, try running this:
//
//      plot.write_html("dsp.html");
//
// then open "dsp.html" file in your browser
