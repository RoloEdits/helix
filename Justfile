set shell := ["nu", "-c"]

default: build

path := "C:/Program Files/helix"

build:
    RUSTFLAGS='-C target-cpu=native' cargo build --profile opt --bin hx

install:
    mkdir {{ path }}
    cp target/opt/hx.exe {{ path }}
    cp runtime -r {{ path }}

clean:
    rm -r target/
