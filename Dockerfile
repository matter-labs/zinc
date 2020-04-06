FROM rust:1.42 as zinc-builder
COPY . zinc-dev/

WORKDIR /zinc-dev/
RUN apt update -y

RUN apt install -y musl musl-dev musl-tools
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --verbose --release --target x86_64-unknown-linux-musl
RUN tar -C target/x86_64-unknown-linux-musl/release/ -czvf zinc-0.1.5-linux.tar.gz zargo zvm znc schnorr

RUN apt install -y dos2unix zip clang cmake gcc g++ zlib1g-dev libmpc-dev libmpfr-dev libgmp-dev
RUN rustup target add x86_64-apple-darwin
RUN dos2unix osxcross_setup.sh
RUN /bin/bash osxcross_setup.sh
ENV PATH /zinc-dev/osxcross/target/bin:$PATH
ENV CC o64-clang
ENV CXX o64-clang++
RUN cargo build --verbose --release --target x86_64-apple-darwin
RUN zip --junk-paths zinc-0.1.5-macos.zip target/x86_64-apple-darwin/release/zargo target/x86_64-apple-darwin/release/zvm target/x86_64-apple-darwin/release/znc target/x86_64-apple-darwin/release/schnorr
