#!/bin/env sh
for dir in day*; do
        cd $dir &&
        ./clean &&
        cd ..
done
