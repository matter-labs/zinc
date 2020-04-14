FROM rust:1.42 as zinc-builder
COPY . zinc-dev/

WORKDIR /zinc-dev/

RUN apt-get update --yes

# Auxiliary tools
RUN apt-get install --yes \
    'apt-utils' \
    'dialog' \
    'dos2unix' \
    'zip'

# Fixing the Windows '\r\n' line endings
RUN dos2unix 'docker/build.sh'

# Main build script, requires the Zinc and Rust versions
RUN /bin/bash 'docker/build.sh' '0.1.5' '1.42.0'
