#pragma once

#include <algorithm>
#include <cmath>
#include <iostream>

#include "maths.h"

struct LowPassGate {
    void set_attack(float attack) {
        _attack = compute_coeff(attack);
    }

    void set_release(float release) {
        _release = compute_coeff(release);
    }

    void trigger(float cutoff) {
        _target_freq = cutoff;
        _is_in_attack_phase = true;
    }

    void render(float* buffer, int num_samples) {
        for (auto i = 0; i < num_samples; ++i) {
            if (i % SUB_SAMPLING_RATE == 0) advance();
            process(buffer[i]);
        }
    }

  private:
    [[nodiscard]] inline float compute_coeff(float value) const {
        const auto coeff =
            value * static_cast<float>(_sample_rate) / SUB_SAMPLING_RATE;
        return std::exp(-1.0f / coeff);
    }

    inline void advance() {
        _current_freq =
            _is_in_attack_phase
                ? (1.0f - _attack) * _target_freq + _attack * _current_freq
                : (1.0f - _release) * MIN_FREQ + _release * _current_freq;

        constexpr auto MAX_FREQ_FACTOR = 0.98f / 2.f;
        _current_freq =
            std::clamp(_current_freq, MIN_FREQ,
                       static_cast<float>(_sample_rate) * MAX_FREQ_FACTOR);

        constexpr auto MAX_FREQ_RAMPING_FACTOR = 0.8f;
        if (_current_freq >= _target_freq * MAX_FREQ_RAMPING_FACTOR)
            _is_in_attack_phase = false;
    }

    inline void process(float& sample) {
        const auto f = 2.f * lut::sin(_current_freq /
                                      (4.f * static_cast<float>(_sample_rate)));
        _lpf = _lpf + f * _bpf;
        _hpf = sample - _lpf - _q * _bpf;
        _bpf = f * _hpf + _bpf;
        sample = _lpf;
    }

    static constexpr auto SUB_SAMPLING_RATE = 8u;
    static constexpr auto MIN_FREQ = 10.f;

    bool _is_in_attack_phase = false;
    int _sample_rate = 48'000;
    float _current_freq = 0.f;
    float _target_freq = 440.f;
    float _lpf = 0.0f;
    float _bpf = 0.0f;
    float _hpf = 0.0f;
    float _q = 0.707f;
    float _attack = compute_coeff(0.001f);
    float _release = compute_coeff(0.1f);
};
