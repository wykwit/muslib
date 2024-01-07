import muslib

loader = muslib.MonoLoader()
audio, sample_rate = loader(file="sine.wav")
outstr = " ".join(map(lambda x: str(round(x * 100)), audio))

with open("sine.out.3", "w") as f:
    f.write(outstr)
