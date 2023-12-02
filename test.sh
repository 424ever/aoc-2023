#!/bin/env sh
for dir in day*; do
        cd $dir &&
        ./test &&
        cd ..
done
