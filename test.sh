#!/bin/env sh
for dir in day*; do
        pushd $dir &&
        ./test &&
        popd
done
