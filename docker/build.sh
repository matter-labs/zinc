#!/bin/bash -Cerx

export VERSION_ZINC="${1}"

export TARGET_MACOS='x86_64-apple-darwin'

mkdir --verbose "zinc-release-${VERSION_ZINC}"



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

# Building cross-compiling tools
git clone \
    'https://github.com/tpoechtrager/osxcross' \
    'osxcross/'
wget --verbose \
    --output-document 'osxcross/tarballs/MacOSX10.10.sdk.tar.xz' \
    'https://s3.dockerproject.org/darwin/v2/MacOSX10.10.sdk.tar.xz'
UNATTENDED='yes' OSX_VERSION_MIN='10.7' bash 'osxcross/build.sh'
export PATH="${PATH}:/zinc-dev/osxcross/target/bin/"
export CC='o64-clang'
export CXX='o64-clang++'

# Downloading OpenSSL
wget --verbose \
  --output-document 'openssl-1.1.1i.tar.gz' \
  'https://homebrew.bintray.com/bottles/openssl%401.1-1.1.1i.catalina.bottle.tar.gz'
tar --verbose --extract --file 'openssl-1.1.1i.tar.gz'
export OPENSSL_DIR='/zinc-dev/openssl@1.1/1.1.1i/'

# Configuring
cat <<EOT >> '.cargo/config'
[target.x86_64-apple-darwin]
linker = "x86_64-apple-darwin14-clang"
ar = "x86_64-apple-darwin14-ar"
EOT
cat <<EOT >> './zandbox/Cargo.toml'
[dependencies.openssl]
version = "0.10"
features = [ "vendored" ]
EOT

# Building
cargo build --verbose --release --target "${TARGET_MACOS}"

# Cleanup
unset CC
unset CXX
unset OPENSSL_DIR

# Bundling
mkdir --verbose "zinc-${VERSION_ZINC}-macos"
mv --verbose --force \
    "target/${TARGET_MACOS}/release/zargo" \
    "target/${TARGET_MACOS}/release/znc" \
    "target/${TARGET_MACOS}/release/zvm" \
    "zinc-${VERSION_ZINC}-macos"
zip --verbose -r \
    "zinc-release-${VERSION_ZINC}/zinc-${VERSION_ZINC}-macos.zip" \
    "zinc-${VERSION_ZINC}-macos"
