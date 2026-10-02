#!/usr/bin/env bash

set -euo pipefail

PACKAGE="syspeek"
VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)"
ARCH="amd64"
PACKAGE_DIR="debian-package"
OUTPUT="${PACKAGE}_${VERSION}-1_${ARCH}.deb"

if [[ -z "$VERSION" ]]; then
    echo "Error: could not determine version from Cargo.toml"
    exit 1
fi

rm -rf "$PACKAGE_DIR"
mkdir -p "$PACKAGE_DIR/DEBIAN"
mkdir -p "$PACKAGE_DIR/usr/bin"

cargo build --release --locked

cp "target/release/$PACKAGE" "$PACKAGE_DIR/usr/bin/$PACKAGE"
chmod 755 "$PACKAGE_DIR/usr/bin/$PACKAGE"

cat > "$PACKAGE_DIR/DEBIAN/control" <<EOF
Package: syspeek
Version: ${VERSION}-1
Section: utils
Priority: optional
Architecture: ${ARCH}
Maintainer: Anup Paudel
Description: A fast and powerful Linux system information and diagnostics tool
 Syspeek is a Linux command-line utility for displaying system,
 hardware, network, Docker, development environment, and diagnostic information.
EOF

dpkg-deb --build --root-owner-group "$PACKAGE_DIR" "$OUTPUT"

echo "Built: $OUTPUT"