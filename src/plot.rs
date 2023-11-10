use crate::fft::Fft;
use plotly::{
    layout::{GridPattern, Layout, LayoutGrid},
    Plot, Scatter,
};

pub fn generate(audio_buffer: &[f32], sample_rate: usize) {
    let mut plot = Plot::new();

    let time_trace = Scatter::new((0..audio_buffer.len()).collect(), audio_buffer.to_vec())
        .name("time")
        .x_axis("x1")
        .y_axis("y1");
    plot.add_trace(time_trace);

    let fft_result = Fft::process(audio_buffer, sample_rate as u32);
    let freq_trace = Scatter::new(fft_result.frequencies, fft_result.magnitudes)
        .name("freqz")
        .x_axis("x2")
        .y_axis("y2");
    plot.add_trace(freq_trace);

    let layout = Layout::new().grid(
        LayoutGrid::new()
            .rows(2)
            .columns(1)
            .pattern(GridPattern::Independent),
    );
    plot.set_layout(layout);

    plot.show();
}
