#!/bin/bash

# Configure version of Zinc to install from releases https://github.com/matter-labs/zinc/releases
VERSION="0.2.0"

# Configure path to install the Zinc binaries (i.e. typically on Linux use "/usr/local/bin")
ZINC_BINARIES_PATH=/tmp/zinc-${VERSION}-linux/bin

# Configure path to hack on Zinc examples
ZINC_EXAMPLES_PATH=~/code/src

# Download and unpack Zinc binaries to configured path
mkdir -p ${ZINC_BINARIES_PATH} && \
curl -LO https://github.com/matter-labs/zinc/releases/download/${VERSION}/zinc-${VERSION}-linux.tar.gz && \
tar -xvf ./zinc-${VERSION}-linux.tar.gz -C ${ZINC_BINARIES_PATH} && \
rm ./zinc-${VERSION}-linux.tar.gz && \
echo "PATH='$ZINC_BINARIES_PATH:$PATH';" >> ~/.bash_profile && \
. ~/.bash_profile && \

# Download and unpack Zinc examples to
mkdir -p ${ZINC_EXAMPLES_PATH}/examples && \
curl -LO https://github.com/matter-labs/zinc/releases/download/${VERSION}/examples.zip
unzip ./examples.zip -d ${ZINC_EXAMPLES_PATH}/examples && \
rm ./examples.zip && \
cd ${ZINC_BINARIES_PATH}
