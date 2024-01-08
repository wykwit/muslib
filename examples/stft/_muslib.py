import muslib

size = 2048

def u16_to_f64(data):
    d = 1 << 15
    return [(x - d) / d for x in data]

def f32_to_u16(data):
    d = 1 << 15
    return [d + int(x * d) for x in data]

# algorithms
loader = muslib.MonoLoader()
fft = muslib.FFT()
ifft = muslib.IFFT()
writer = muslib.MonoWriter()

# applying compute on data
data, _ = loader.compute("sample.wav")
data = u16_to_f64(data[:size])
data = fft.compute(data)
data = ifft.compute(data)
data = f32_to_u16(data)
writer.compute("out.2.wav", data)
