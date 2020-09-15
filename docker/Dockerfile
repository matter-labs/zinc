FROM rust:1.46 as zinc-builder
COPY . matter-labs/

WORKDIR /matter-labs/

RUN apt-get update --yes

# Auxiliary tools
RUN apt-get install --yes \
    'apt-utils' \
    'dialog' \
    'dos2unix' \
    'zip'

# Fixing the Windows '\r\n' line endings
RUN dos2unix './zinc-dev/docker/build.sh'

# Main build script, expects the Zinc version
RUN /bin/bash './zinc-dev/docker/build.sh' '0.2.0'
