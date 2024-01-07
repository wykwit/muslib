import essentia.standard

loader = essentia.standard.MonoLoader(filename="sine.wav")
audio = loader()
outstr = " ".join(map(lambda x: str(round(x * 100)), audio))

with open("sine.out.1", "w") as f:
    f.write(outstr)
