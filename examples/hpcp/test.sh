#!/bin/bash

python _essentia.py | tee out.1
python _muslib.py | tee out.2
diff out.1 out.2 > /dev/null
if [[ $? -ne 0 ]];
then
    echo "different"
else
    echo "same"
fi

read -p "paused before cleaning"
rm out.1 out.2
