default: build

build:
    CC="clang" CXX="clang++" CXXFLAGS="-march=native -flto -fuse-ld=lld" CFLAGS="-march=native -flto -fuse-ld=lld" cargo build --profile opt --bin hx

install:
    cp -f target/opt/hx ~/.local/bin/hx
    cp -fr runtime/queries ~/.config/helix/runtime/queries
    cp -fr runtime/themes ~/.config/helix/runtime/themes
    hx --grammar fetch
    CXX="clang++" CXXFLAGS="-march=native -fuse-ld=lld" hx --grammar build
