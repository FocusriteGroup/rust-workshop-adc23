use rustfft::num_complex::Complex;

pub struct Fft {
    pub frequencies: Vec<f32>,
    pub magnitudes: Vec<f32>,
}

impl Fft {
    pub fn process(audio_buffer: &[f32], sample_rate: u32) -> Self {
        let mut fft = audio_buffer
            .iter()
            .map(|&x| Complex::new(x, 0.0))
            .collect::<Vec<_>>();

        rustfft::FftPlanner::new()
            .plan_fft_forward(fft.len())
            .process(&mut fft);

        let frequencies = (0..fft.len() / 2)
            .map(|i| i as f32 * sample_rate as f32 / fft.len() as f32)
            .collect::<Vec<_>>();

        let magnitudes = fft
            .iter()
            .take(fft.len() / 2)
            .map(|c| c.norm() / fft.len() as f32)
            .collect::<Vec<_>>();

        Self {
            frequencies,
            magnitudes,
        }
    }
}
