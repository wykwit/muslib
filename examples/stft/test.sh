#!/bin/bash

python _essentia.py
python _muslib.py
python ../scripts/plot_wav.py out.1.wav &
python ../scripts/plot_wav.py out.2.wav &

read -p "paused before cleaning"
rm out.1.wav out.2.wav
