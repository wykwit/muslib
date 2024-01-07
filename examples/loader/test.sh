#!/bin/bash

python _essentia.py
cargo run --example loader sine.wav > sine.out.2
python _muslib.py
python ../scripts/plot.py sine.out.1 &
python ../scripts/plot.py sine.out.2 &
python ../scripts/plot.py sine.out.3 &
python ../scripts/diff.py sine.out.1 sine.out.2

read -p "paused before cleaning"
rm sine.out.1 sine.out.2 sine.out.3
