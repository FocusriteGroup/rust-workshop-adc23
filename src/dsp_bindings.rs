/* automatically generated by rust-bindgen 0.69.1 */

pub const Waveform_WAVEFORM_SINE: Waveform = 0;
pub const Waveform_WAVEFORM_SAWTOOTH: Waveform = 1;
pub const Waveform_WAVEFORM_SQUARE: Waveform = 2;
pub type Waveform = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DspContext {
    _unused: [u8; 0],
}
extern "C" {
    pub fn dsp_create() -> *mut DspContext;
}
extern "C" {
    pub fn dsp_destroy(dsp: *mut DspContext);
}
extern "C" {
    pub fn dsp_prepare(dsp: *mut DspContext, sample_rate: ::std::os::raw::c_int);
}
extern "C" {
    pub fn dsp_render(dsp: *mut DspContext, output: *mut f32, num_samples: ::std::os::raw::c_int);
}
extern "C" {
    pub fn dsp_trigger_note(dsp: *mut DspContext, frequency: f32, cutoff: f32);
}
extern "C" {
    pub fn dsp_set_waveform(dsp: *mut DspContext, waveform: Waveform);
}
extern "C" {
    pub fn dsp_set_attack(dsp: *mut DspContext, attack: f32);
}
extern "C" {
    pub fn dsp_set_release(dsp: *mut DspContext, release: f32);
}
