import essentia.standard

# algorithms
loader = essentia.standard.MonoLoader(filename="sample.wav")
fft = essentia.standard.FFT()
magnitude = essentia.standard.Magnitude()
peaks = essentia.standard.SpectralPeaks()
hpcp = essentia.standard.HPCP()

# applying compute on data
size = 2048
data = loader.compute()
data = data[:size]
data = fft.compute(data)
data = magnitude.compute(data)
frequencies, magnitudes = peaks.compute(data)
data = hpcp.compute(frequencies, magnitudes)
data = [round(x, 5) for x in data]

# display results
print(data)
