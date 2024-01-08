import essentia.standard

size = 2048

# algorithms
loader = essentia.standard.MonoLoader(filename="sample.wav")
fft = essentia.standard.FFT()
ifft = essentia.standard.IFFT()
writer = essentia.standard.MonoWriter(filename="out.1.wav")

# applying compute on data
data = loader.compute()
data = data[:size]
data = fft.compute(data)
data = ifft.compute(data)
writer.compute(data)
