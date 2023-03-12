#!/usr/bin/bash

if [ ! -d "build/" ];
then
	mkdir "build/"
fi

read -p "Build .deb package (y/n): " build_deb
read -p "Build Windows executable (y/n): " build_win

# Build .deb package
if [ $build_deb == "y" ] || [ $build_deb == "Y" ];
then
	read -p "Version: " version
	read -p "Revision: " revision

	echo "Building .deb package..."

	deb_root="void_${version}-${revision}_amd64"
	
	cargo build -r

	mkdir $deb_root
	mkdir "${deb_root}/DEBIAN/"
	echo """Package: void
Version: ${version}
Architecture: amd64
Maintainer: Vie <vie.devr@gmail.com>
Description: Code editor written in Rust just for fun.
""" > "${deb_root}/DEBIAN/control"

	mkdir -p "${deb_root}/usr/bin/"
	cp -r "target/release/void" "${deb_root}/usr/bin/"

	mkdir -p "${deb_root}/usr/share/void/"
	cp -r "grammars/" "grammars.json" "${deb_root}/usr/share/void/"

	dpkg-deb --build --root-owner-group $deb_root
	mv "${deb_root}.deb" "build/"

	echo "Cleaning up..."

	# Cleanup
	rm -rf $deb_root
	rm -rf "target/release/"

	echo "Done!"
fi
# End building .deb package

# Build Windows executable
if [ $build_win == "y" ] || [ $build_win == "Y" ];
then
	echo "Building Windows executable..."

	cargo build -r --target "x86_64-pc-windows-gnu"

	mkdir -p "build/Void/data"
	cp -r "target/x86_64-pc-windows-gnu/release/void.exe" "build/Void/"
	cp -r "grammars/" "grammars.json" "build/Void/data/"

	echo "Cleaning up..."

	# Cleanup
	rm -rf "target/x86_64-pc-windows-gnu/"

	echo "Done!"
fi
# End building Windows executable
