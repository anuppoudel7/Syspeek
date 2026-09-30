pkgname=syspeek
pkgver=0.1.0
pkgrel=1
pkgdesc="A fast and powerful Linux system information and diagnostics tool"
arch=('x86_64')
url="https://github.com/anuppoudel7/Syspeek"

depends=('glibc')
makedepends=('cargo')

source=("$pkgname-$pkgver.tar.gz::https://github.com/anuppoudel7/Syspeek/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('3fa046b145baa7d5e3dac16defa8ec4dbc4a001b6000c87974e99cd17518d737')

build() {
    cd "Syspeek-$pkgver"

    cargo build --release --locked
}

package() {
    cd "Syspeek-$pkgver"

    install -Dm755 "target/release/syspeek" \
        "$pkgdir/usr/bin/syspeek"
}
