#!/bin/bash -Cerx

export VERSION_ZINC="${1}"
export VERSION_RUST="${2}"

export TARGET_WINDOWS='x86_64-pc-windows-gnu'
export TARGET_LINUX='x86_64-unknown-linux-musl'
export TARGET_MACOS='x86_64-apple-darwin'

mkdir --verbose "zinc-release-${VERSION_ZINC}"



#######################################################################################################################
### (DO NOT USE YET)                                    Windows                                                     ###
#######################################################################################################################

## Preparation
#apt-get install --yes 'gcc-mingw-w64-x86-64'
#rustup target add "${TARGET_WINDOWS}"
#mkdir --verbose \
#    --parents "/usr/local/rustup/toolchains/${VERSION_RUST}-x86_64-unknown-linux-gnu/lib/rustlib/${TARGET_WINDOWS}/lib/"
#cp --verbose --force \
#    '/usr/x86_64-w64-mingw32/lib/crt2.o' \
#    '/usr/x86_64-w64-mingw32/lib/dllcrt2.o' \
#    "/usr/local/rustup/toolchains/${VERSION_RUST}-x86_64-unknown-linux-gnu/lib/rustlib/${TARGET_WINDOWS}/lib/"
#
## Building
#cargo build --verbose --release --target "${TARGET_WINDOWS}"
#
## Archiving
#mkdir --verbose "zinc-${VERSION_ZINC}-windows"
#mv --verbose --force \
#    "target/${TARGET_WINDOWS}/release/zargo.exe" \
#    "target/${TARGET_WINDOWS}/release/zvm.exe" \
#    "target/${TARGET_WINDOWS}/release/znc.exe" \
#    "target/${TARGET_WINDOWS}/release/schnorr.exe" \
#    "zinc-${VERSION_ZINC}-windows"
#zip --verbose -r \
#    "zinc-release-${VERSION_ZINC}/zinc-${VERSION_ZINC}-windows.zip" \
#    "zinc-${VERSION_ZINC}-windows"



#######################################################################################################################
###                                                      Linux                                                      ###
#######################################################################################################################

# Preparation
apt-get install --yes \
    'musl' \
    'musl-dev' \
    'musl-tools'
rustup target add "${TARGET_LINUX}"

# Building
cargo build --verbose --release --target "${TARGET_LINUX}"

# Archiving
mkdir --verbose "zinc-${VERSION_ZINC}-linux"
mv --verbose --force \
    "target/${TARGET_LINUX}/release/zargo" \
    "target/${TARGET_LINUX}/release/zvm" \
    "target/${TARGET_LINUX}/release/znc" \
    "target/${TARGET_LINUX}/release/schnorr" \
    "zinc-${VERSION_ZINC}-linux"
tar --verbose \
    --create --gzip --file "zinc-release-${VERSION_ZINC}/zinc-${VERSION_ZINC}-linux.tar.gz" \
    "zinc-${VERSION_ZINC}-linux"



#######################################################################################################################
###                                                      MacOS                                                      ###
#######################################################################################################################

# Preparation
apt-get install --yes \
    'clang' \
    'cmake' \
    'gcc' \
    'g++' \
    'zlib1g-dev' \
    'libmpc-dev' \
    'libmpfr-dev' \
    'libgmp-dev'
rustup target add "${TARGET_MACOS}"
git clone \
    'https://github.com/tpoechtrager/osxcross' \
    'osxcross/'
wget --verbose \
    --output-document 'osxcross/tarballs/MacOSX10.10.sdk.tar.xz' \
    'https://s3.dockerproject.org/darwin/v2/MacOSX10.10.sdk.tar.xz'
UNATTENDED='yes' OSX_VERSION_MIN='10.7' bash 'osxcross/build.sh'
export PATH="/zinc-dev/osxcross/target/bin:${PATH}"
export CC='o64-clang'
export CXX='o64-clang++'

# Building
cargo build --verbose --release --target "${TARGET_MACOS}"

# Archiving
mkdir --verbose "zinc-${VERSION_ZINC}-macos"
mv --verbose --force \
    "target/${TARGET_MACOS}/release/zargo" \
    "target/${TARGET_MACOS}/release/zvm" \
    "target/${TARGET_MACOS}/release/znc" \
    "target/${TARGET_MACOS}/release/schnorr" \
    "zinc-${VERSION_ZINC}-macos"
zip --verbose -r \
    "zinc-release-${VERSION_ZINC}/zinc-${VERSION_ZINC}-macos.zip" \
    "zinc-${VERSION_ZINC}-macos"
