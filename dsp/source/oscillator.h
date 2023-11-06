#pragma once

#include "dsp.h"
#include "maths.h"

struct Oscillator {
    void set_waveform(Waveform waveform) {
        _waveform = waveform;
    }

    void set_sample_rate(int sample_rate) {
        _sample_rate = sample_rate;
    }

    void set_frequency(float frequency) {
        _frequency = frequency;
    }

    void render(float* output, int num_samples) {
        const auto render = get_sample_renderer(_waveform);
        const auto increment = _frequency / static_cast<float> (_sample_rate);
        for (auto i = 0; i < num_samples; ++i) {
            output[i] = render(_phase, increment);
            _phase += increment;
            if (_phase > 1.) _phase -= 1.;
        }
    }

  private:
    using RenderFunction = float (*)(float, float);

    static RenderFunction get_sample_renderer(Waveform waveform) {
        switch (waveform) {
            case WAVEFORM_SAWTOOTH: return signals::saw;
            case WAVEFORM_SQUARE: return signals::sqr;
            case WAVEFORM_SINE:
            default: return signals::sin;
        }
    }

    float _phase = 0.;
    int _sample_rate = 48'000;
    Waveform _waveform = WAVEFORM_SQUARE;
    float _frequency = 440.;
};
