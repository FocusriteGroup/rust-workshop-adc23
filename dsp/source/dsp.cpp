#include "dsp.h"

#include "filter.h"
#include "oscillator.h"

struct DspContext {
    Oscillator osc;
    LowPassGate lpg;
};

DspContext* dsp_create() {
    return new DspContext;
}

void dsp_destroy(DspContext* dsp) {
    delete dsp;
}

void dsp_prepare(DspContext* dsp, int sample_rate) {
    dsp->osc.set_sample_rate(sample_rate);
}

void dsp_render(DspContext* dsp, float* output, int num_samples) {
    dsp->osc.render(output, num_samples);
    dsp->lpg.render(output, num_samples);
}

void dsp_trigger_note(DspContext* dsp, float frequency, float cutoff) {
    dsp->osc.set_frequency(frequency);
    dsp->lpg.trigger(cutoff);
}

void dsp_set_waveform(DspContext* dsp, Waveform waveform) {
    dsp->osc.set_waveform(waveform);
}

void dsp_set_attack(struct DspContext* dsp, float attack) {
    dsp->lpg.set_attack(attack);
}

void dsp_set_release(struct DspContext* dsp, float release) {
    dsp->lpg.set_release(release);
}
