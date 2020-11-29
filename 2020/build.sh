#!/bin/bash

export CXXFLAGS="-flto"
export LDFLAGS="-flto"

mkdir build-native
cd build-native && \
cmake .. -DCMAKE_BUILD_TYPE=Release && \
make
cd ..

mkdir build-web
cd build-web && \
emcmake cmake .. -DCMAKE_BUILD_TYPE=Release && \
emmake make
cd ..