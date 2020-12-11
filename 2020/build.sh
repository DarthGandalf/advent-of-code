#!/bin/bash

export CXXFLAGS="-flto"
export LDFLAGS="-flto"

mkdir build-native
cd build-native && \
CXX=clang++ CXXFLAGS="${CXXFLAGS} -stdlib=libc++" cmake .. -DCMAKE_BUILD_TYPE=Release && \
make
cd ..

mkdir build-web
cd build-web && \
emcmake cmake .. -DCMAKE_BUILD_TYPE=Release -DCMAKE_EXPORT_COMPILE_COMMANDS=yes && \
emmake make
cd ..

npm ci
npx webpack
