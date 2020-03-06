#!/bin/bash

git clone 'https://github.com/tpoechtrager/osxcross'
cd 'osxcross'
wget -nc 'https://s3.dockerproject.org/darwin/v2/MacOSX10.10.sdk.tar.xz'
mv 'MacOSX10.10.sdk.tar.xz' 'tarballs/'
UNATTENDED='yes' OSX_VERSION_MIN='10.7' ./build.sh
