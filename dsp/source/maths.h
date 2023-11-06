#pragma once

#include <array>
#include <functional>

namespace dsp {
inline float interpolate(float index, const float* lut) {
    const auto x0 = static_cast<unsigned int>(index);
    const auto x1 = x0 + 1;
    const auto w1 = index - static_cast<float> (x0);
    const auto w0 = 1.0f - w1;
    return lut[x0] * w0 + lut[x1] * w1;
}

inline float poly_blep(float t, float dt) {
    if (t < dt) {
        t /= dt;
        return t + t - t * t - 1.0f;
    }

    if (t > 1.0f - dt) {
        t = (t - 1.0f) / dt;
        return t * t + t + t + 1.0f;
    }

    return 0.0f;
}
}  // namespace dsp

namespace lut {
template<auto func, std::size_t LENGTH>
constexpr auto generate() {
    std::array<float, LENGTH> table;
    for (auto i = 0u; i < table.size(); ++i)
        table[i] = static_cast<float> (func(static_cast<float>(i) / static_cast<float> (table.size())));
    return table;
}

template<std::size_t LENGTH>
inline auto lookup(float phase, const std::array<float, LENGTH>& table) {
    const auto index = phase * static_cast<float>(table.size() - 1);
    return dsp::interpolate(index, table.data());
}

[[maybe_unused]] inline auto tan(float phase) {
    static auto const table =
        generate<[](float x) { return std::tan(0.5 * M_PI * x); }, 4096>();
    return lookup(phase, table);
}

inline auto sin(float phase) {
    static const auto table =
        generate<[](float x) { return std::sin(2.0 * M_PI * x); }, 4096>();
    return lookup(phase, table);
}
}  // namespace lut

namespace signals {
inline float sin(float phase, float) {
    return lut::sin(phase);
}

inline float saw(float phase, float dt) {
    return (2.0f * phase - 1.0f) - dsp::poly_blep(phase, dt);
}

inline float sqr(float phase, float dt) {
    const auto phase_shifted = (phase < 0.5f) ? phase + 0.5f : phase - 0.5f;
    const auto value =
        (phase < 0.5f ? 1.0f : -1.0f) + dsp::poly_blep(phase, dt);
    return value - dsp::poly_blep(phase_shifted, dt);
}
}  // namespace signals
