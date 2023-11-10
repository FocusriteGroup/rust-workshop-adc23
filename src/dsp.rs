use clap::ValueEnum;

mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/", env!("DSP_BINDINGS")));
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum Waveform {
    Sine = ffi::Waveform_WAVEFORM_SINE as isize,
    Sawtooth = ffi::Waveform_WAVEFORM_SAWTOOTH as isize,
    Square = ffi::Waveform_WAVEFORM_SQUARE as isize,
}

pub struct Dsp {
    context: *mut ffi::DspContext,
}

impl Default for Dsp {
    fn default() -> Self {
        Self {
            context: unsafe { ffi::dsp_create() },
        }
    }
}

impl Drop for Dsp {
    fn drop(&mut self) {
        unsafe { ffi::dsp_destroy(self.context) };
    }
}

// Since `Dsp` holds a raw pointer, this wrapper needs
// to handle the memory and thread safety checks that
// the borrow checker will use. These methods _could_
// be `&self` or `&mut self`, since they have `unsafe`
// access to a raw pointer, it is up to us to define
// the safety guarantees.
//
// In our case, the C++ DSP code does not have any guards
// against race conditions. So these methods are `&mut self`
// which will tell the borrow checker that we cannot
// have shared access.
impl Dsp {
    pub fn prepare(&mut self, sample_rate: u32) {
        unsafe { ffi::dsp_prepare(self.context, sample_rate as i32) };
    }

    pub fn render(&mut self, output: &mut [f32]) {
        unsafe { ffi::dsp_render(self.context, output.as_mut_ptr(), output.len() as i32) };
    }

    pub fn trigger(&mut self, frequency: f32, cutoff: f32) {
        unsafe { ffi::dsp_trigger_note(self.context, frequency, cutoff) };
    }

    pub fn set_attack(&mut self, attack: f32) {
        unsafe { ffi::dsp_set_attack(self.context, attack) };
    }

    pub fn set_release(&mut self, release: f32) {
        unsafe { ffi::dsp_set_release(self.context, release) };
    }

    pub fn set_waveform(&mut self, waveform: Waveform) {
        unsafe { ffi::dsp_set_waveform(self.context, waveform as i32) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn db_to_linear(level_db: f32) -> f32 {
        10.0_f32.powf(level_db / 20.0)
    }

    #[test]
    fn low_output_before_trigger() {
        let threshold = db_to_linear(-35.0);

        const SAMPLE_RATE: u32 = 48_000;
        const BUFFER_SIZE: usize = 1_024;
        let mut output = [0.0; BUFFER_SIZE];

        let mut dsp = Dsp::default();

        dsp.prepare(SAMPLE_RATE);
        dsp.render(&mut output);

        assert!(output.iter().all(|value| value.abs() < threshold));
    }

    #[test]
    fn high_output_after_trigger() {
        let threshold = db_to_linear(-12.0);

        let mut dsp = Dsp::default();

        const SAMPLE_RATE: u32 = 48_000;
        const BUFFER_SIZE: usize = 1_024;
        let mut output = [0.0; BUFFER_SIZE];

        dsp.prepare(SAMPLE_RATE);
        dsp.trigger(440.0, (SAMPLE_RATE / 4) as f32);
        dsp.set_waveform(Waveform::Sawtooth);
        dsp.render(&mut output);

        let max_value = output
            .iter()
            .fold(0.0, |max, &x| if x.abs() > max { x.abs() } else { max });

        assert!(max_value > threshold);
    }
}
