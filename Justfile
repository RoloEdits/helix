# set shell := ["nu", "-c"]

default: build

build:
    CC="clang" CXX="clang++" CXXFLAGS="-march=native" CFLAGS="-march=native" cargo build --profile opt --bin hx
