set shell := ["nu", "-c"]

default: build

build:
    CC="clang" CXX="clang++" AR="llvm-lib" CXXFLAGS="-march=native -O3 -fuse-ld=lld" CFLAGS="-march=native -O3 -fuse-ld=lld" cargo build --profile opt --bin hx
