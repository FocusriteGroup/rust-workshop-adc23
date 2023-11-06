#pragma once

#ifdef __cplusplus
extern "C" {
#endif

enum Waveform : int {
    WAVEFORM_SINE,
    WAVEFORM_SAWTOOTH,
    WAVEFORM_SQUARE,
};

struct DspContext;

struct DspContext* dsp_create();
void dsp_destroy(struct DspContext* dsp);

void dsp_prepare(struct DspContext* dsp, int sample_rate);
void dsp_render(struct DspContext* dsp, float* output, int num_samples);

void dsp_trigger_note(struct DspContext* dsp, float frequency, float cutoff);
void dsp_set_waveform(struct DspContext* dsp, enum Waveform waveform);
void dsp_set_attack(struct DspContext* dsp, float attack);
void dsp_set_release(struct DspContext* dsp, float release);

#ifdef __cplusplus
}
#endif
