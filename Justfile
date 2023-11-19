export CC := "clang"
export CXX := "clang++"

cflags := "-march=native -O3 -flto -fuse-ld=lld -fomit-frame-pointer -funroll-loops -mllvm=-enable-dfa-jump-thread -fno-plt -Wl,--icf=all"
export CFLAGS := cflags
export CXXFLAGS := cflags

export RUSTFLAGS := "-Ctarget-cpu=native -Clinker=clang -Clink-arg=-fuse-ld=lld -Clink-args=-Wl,--icf=all"

default: build

build:
    cargo build --release --bin hx

install: build
    mkdir -p ~/.local/bin
    cp -f target/release/hx ~/.local/bin/hx
    just grammar

grammar:
    mkdir -p ~/.config/helix/runtime/queries ~/.config/helix/runtime/themes
    cp -fr runtime/queries/* ~/.config/helix/runtime/queries/
    cp -fr runtime/themes/* ~/.config/helix/runtime/themes/
    ~/.local/bin/hx --grammar fetch
    ~/.local/bin/hx --grammar build
