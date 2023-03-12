#!/usr/bin/bash

cargo build -r

read -p "Version: " version
read -p "Revision: " revision

root="void_${version}-${revision}_amd64"

mkdir $root
mkdir "${root}/DEBIAN/"
echo """
Package: void
Version: ${version}
Architecture: amd64
Maintainer: Vie <vie.devr@gmail.com>
Description: Code editor written in Rust just for fun.
""" > "${root}/DEBIAN/control"

mkdir -p "${root}/usr/bin/"
cp -r "target/release/void" "${root}/usr/bin/"

mkdir -p "${root}/usr/share/void/"
cp -r "grammars" "grammars.json" "${root}/usr/share/void/"

dpkg-deb --build --root-owner-group $root
