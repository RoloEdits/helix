# set shell := ["nu", "-c"]

default: build

build:
    CC="clang" CXX="clang++" CXXFLAGS="-march=native" CFLAGS="-march=native" cargo build --profile opt --bin hx

install:
    cp -f target/opt/hx ~/.local/bin/hx
    cp -fr runtime/queries ~/.config/helix/runtime/queries
    cp -fr runtime/themes ~/.config/helix/runtime/themes
    hx --grammar fetch
    CXXFLAGS="-march=native" hx --grammar build
