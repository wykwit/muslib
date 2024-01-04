#!/bin/bash

python _essentia.py
# TODO: add python implementation here
# python _muslib.py
cargo run --example loader sine.wav > sine.out.2
python ../scripts/plot.py sine.out.1 &
python ../scripts/plot.py sine.out.2 &
python ../scripts/diff.py sine.out.1 sine.out.2

read -p "paused before cleaning"
rm sine.out.1 sine.out.2
