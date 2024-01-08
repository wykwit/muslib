import math

import muslib


def u16_to_f64(data: list[int]) -> list[float]:
    d = 1 << 15
    return [(x - d) / d for x in data]


# the absolute value of each element in a vector of complex numbers
def magnitude(data: list[tuple[float, float]]) -> list[float]:
    # Spectrum algorithm in Essentia calls FFT and then Magnitude, which is this one line
    return [math.sqrt(x[0] ** 2 + x[1] ** 2) for x in data]


# extract peaks from a spectrum
def peaks(
    data: list[float],  # spectrum
    max_frequency=5000.0,
    sample_rate=44100.0,
    threshold=0.0,
) -> tuple[list[float], list[float]]:
    # SpectralPeaks algorithm in Essentia calls PeakDetection and renames outputs

    # this equation comes from the PeakDetection algorithm, there is a longer explanation there
    def interpolate(left, mid, right, c_bin):
        delta_x = 0.5 * ((left - right) / (left - 2 * mid + right))
        r_bin = c_bin + delta_x
        r_val = mid - 0.25 * (left - right) * delta_x
        return r_bin, r_val

    size = len(data)
    scale = sample_rate / size
    peaks = []  # (pos, val) vec

    i = 0
    if i + 1 < size and data[i] > data[i + 1] and data[i] > threshold:
        # don't ask me why, that's what they do
        peaks.append((i * scale, data[i]))

    while True:
        # finding the peak
        while i + 1 < size - 1 and data[i] >= data[i + 1]:
            i += 1
        while i + 1 < size - 1 and data[i] < data[i + 1]:
            i += 1

        # walking through the plateau
        j = i
        while j + 1 < size - 1 and data[j] == data[j + 1]:
            j += 1

        # checking direction at the end of plateau
        if j + 1 < size - 1 and data[j + 1] < data[j] and data[j] > threshold:
            # the direction is down
            if j != i:
                # we found plateau peak between i and j
                b = (i + j) * 0.5
                v = data[i]
            else:
                # we found a nice peak
                b, v = interpolate(data[j - 1], data[j], data[j + 1], j)

            p = b * scale
            if p > max_frequency:
                # we've gone too far already
                break
            else:
                peaks.append((p, v))

        # continue
        i = j

        # at the end of data
        if i + 1 >= size - 1:
            # tbh that bit to me looks kinda crazy too
            if (
                i == size - 2
                and data[i - 1] < data[i]
                and data[i + 1] < data[i]
                and data[i] > threshold
            ):
                b, v = interpolate(data[i - 1], data[i], data[i + 1], j)
                p = b * scale
                peaks.append((p, v))
            # end loop
            break

    positions = [x[0] for x in peaks]
    amplitudes = [x[1] for x in peaks]

    return (positions, amplitudes)  # aka (frequencies, magnitudes)


# algorithms
loader = muslib.MonoLoader()
fft = muslib.FFT()
hpcp = muslib.HPCP()

# applying computations over data
size = 2048
data = "sample.wav"
data, sample_rate = loader.compute(data)
data = u16_to_f64(data[:size])
data = fft.compute(data)
data = magnitude(data)
frequencies, magnitudes = peaks(data, sample_rate=sample_rate, threshold=0.5)
data = hpcp.compute(frequencies, magnitudes)
data = [round(x, 5) for x in data]

# display results
print(data)
