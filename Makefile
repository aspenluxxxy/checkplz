ROOT_DIR              := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
APPS_DIR              := ${ROOT_DIR}/programs
BUILD_DIR             := ${ROOT_DIR}/build
CONFIG_DIR            := ${ROOT_DIR}/config
INITRAMFS_DIR         := ${BUILD_DIR}/initramfs
ISO_DIR               := ${BUILD_DIR}/iso
LIBS_DIR              := ${ROOT_DIR}/libraries
SCRIPTS_DIR           := ${ROOT_DIR}/scripts
SYSROOT_DIR           := ${BUILD_DIR}/sysroot

CHECKNIT_DIR          := ${ROOT_DIR}/checknit
MUSL_DIR              := ${LIBS_DIR}/musl

LIBIMOBILEDEVICE_DIR  := ${LIBS_DIR}/libimobiledevice
LIBNCURSES_DIR        := ${LIBS_DIR}/ncurses
LIBPLIST_DIR          := ${LIBS_DIR}/libplist
LIBREADLINE_DIR       := ${LIBS_DIR}/readline
LIBUSBMUXD_DIR        := ${LIBS_DIR}/libusbmuxd
LIBUSB_DIR            := ${LIBS_DIR}/libusb
LINUX_DIR             := ${LIBS_DIR}/linux
OPENSSL_DIR           := ${LIBS_DIR}/openssl

BUSYBOX_DIR           := ${APPS_DIR}/busybox
DROPBEAR_DIR          := ${APPS_DIR}/dropbear
IRECOVERY_DIR         := ${APPS_DIR}/irecovery
USBMUXD_DIR           := ${APPS_DIR}/usbmuxd

MUSL_CFLAGS           := -Oz -s -ffunction-sections -fdata-sections
MUSL_LDFLAGS          := -Oz -s -fuse-ld=lld -Wl,--gc-sections

LTO_CFLAGS            := -I"${SYSROOT_DIR}/include" -Oz -s -flto -ffunction-sections -fdata-sections -D__STDC_NO_ATOMICS__
LTO_LDFLAGS           := -L"${SYSROOT_DIR}/lib" -Wl,--gc-sections

CHECKRA1N_VERSION     := 4bf2f7e1dd201eda7d6220350db666f507d6f70e07845b772926083a8a96cd2b

export RANLIB         := llvm-ranlib
export AR             := llvm-ar
export CC             := ccache ${SCRIPTS_DIR}/musl-clang
export PATH           := ${SCRIPTS_DIR}:$(PATH)

all: initramfs iso


linux:
	cp -f "${CONFIG_DIR}/linux.config" "${LINUX_DIR}/.config"
	LLVM=1 LLVM_IAS=1 $(MAKE) -C "${LINUX_DIR}"
	cp -f "${LINUX_DIR}/arch/x86/boot/bzImage" "${BUILD_DIR}/bzImage"
	LLVM=1 LLVM_IAS=1 $(MAKE) -C "${LINUX_DIR}" headers_install INSTALL_HDR_PATH="${SYSROOT_DIR}"

clean_linux:
	$(MAKE) -C "${LINUX_DIR}" clean

musl: export CC = ccache clang
musl: export CFLAGS = ${MUSL_CFLAGS}
musl: export LDFLAGS = ${MUSL_LDFLAGS}
musl:
	cd "${MUSL_DIR}" && ./configure \
		--disable-shared \
		--prefix="${SYSROOT_DIR}" \
		--syslibdir="${SYSROOT_DIR}/lib"
	$(MAKE) -C "${MUSL_DIR}"
	$(MAKE) -C "${MUSL_DIR}" install

clean_musl:
	$(MAKE) -C "${MUSL_DIR}" clean

libplist: export CFLAGS = ${LTO_CFLAGS}
libplist: export LDFLAGS = ${LTO_LDFLAGS}
libplist: musl
	cd "${LIBPLIST_DIR}" && ./autogen.sh \
		--prefix="${SYSROOT_DIR}" \
		--disable-shared \
		--without-cython
	$(MAKE) -C "${LIBPLIST_DIR}"
	$(MAKE) -C "${LIBPLIST_DIR}" install

clean_libplist:
	$(MAKE) -C "${LIBPLIST_DIR}" clean

openssl: export CFLAGS = ${LTO_CFLAGS}
openssl: export LDFLAGS = ${LTO_LDFLAGS}
openssl: linux musl
	cd "${OPENSSL_DIR}" && ./Configure \
			--prefix="${SYSROOT_DIR}" \
			--release \
			-static \
			no-async \
			no-buildtest-c++ \
			no-comp \
			no-dtls \
			no-dtls1 \
			no-ec2m \
			no-idea \
			no-makedepend \
			no-mdc2 \
			no-psk \
			no-rc5 \
			no-seed \
			no-shared \
			no-sm2 \
			no-sm4 \
			no-srp \
			no-ssl2 \
			no-ssl3 \
			no-stdio \
			no-tests \
			no-unit-test \
			no-weak-ssl-ciphers \
			no-zlib \
			${LTO_CFLAGS} \
			${LTO_LDFLAGS} \
			linux-x86_64-clang
	$(MAKE) -C "${OPENSSL_DIR}" build_libs
	$(MAKE) -C "${OPENSSL_DIR}" install_sw install_ssldirs

clean_openssl:
	$(MAKE) -C "${OPENSSL_DIR}" clean

libusbmuxd: export CFLAGS = ${LTO_CFLAGS}
libusbmuxd: export LDFLAGS = ${LTO_LDFLAGS}
libusbmuxd: libplist
	cd "${LIBUSBMUXD_DIR}" && ./autogen.sh \
		--prefix="${SYSROOT_DIR}" \
		--disable-shared
	$(MAKE) -C "${LIBUSBMUXD_DIR}"
	$(MAKE) -C "${LIBUSBMUXD_DIR}" install

clean_libusbmuxd:
	$(MAKE) -C "${LIBUSBMUXD_DIR}" clean

libimobiledevice: export CFLAGS = ${LTO_CFLAGS}
libimobiledevice: export LDFLAGS = ${LTO_LDFLAGS}
libimobiledevice: libusbmuxd openssl
	cd "${LIBIMOBILEDEVICE_DIR}" && ./autogen.sh \
		--prefix="${SYSROOT_DIR}" \
		--with-sysroot="${SYSROOT_DIR}" \
		--without-cython \
		--enable-shared=no \
		--enable-static=yes
	$(MAKE) -C "${LIBIMOBILEDEVICE_DIR}"
	$(MAKE) -C "${LIBIMOBILEDEVICE_DIR}" install

clean_libimobiledevice:
	$(MAKE) -C "${LIBUSBMUXD_DIR}" clean

libusb: export CFLAGS = ${LTO_CFLAGS} -Werror=implicit-function-declaration
libusb: export LDFLAGS = ${LTO_LDFLAGS}
libusb: linux musl
	cd "${LIBUSB_DIR}" && ./autogen.sh \
		--prefix="${SYSROOT_DIR}" \
		--enable-shared=no \
		--enable-static=yes \
		--enable-examples-build=no \
		--enable-tests-build=no \
		--enable-udev=no
	$(MAKE) -C "${LIBUSB_DIR}"
	$(MAKE) -C "${LIBUSB_DIR}" install

clean_libusb:
	$(MAKE) -C "${LIBUSB_DIR}" clean

usbmuxd: export CFLAGS = ${LTO_CFLAGS}
usbmuxd: export LDFLAGS = ${LTO_LDFLAGS}
usbmuxd: libusb libusbmuxd libimobiledevice
	# I fucking hate automake
	$(CC) \
		-I"${SYSROOT_DIR}/include/libusb-1.0" \
		${LTO_CFLAGS} \
		${LTO_LDFLAGS} \
		-o "${SYSROOT_DIR}/bin/usbmuxd" \
		-D__STDC_NO_ATOMICS__ -DHAVE_CLOCK_GETTIME -DPACKAGE_NAME="\"usbmuxd\"" \
		-DPACKAGE_TARNAME="\"usbmuxd\"" -DPACKAGE_VERSION="\"1.1.1\"" \
		-DPACKAGE_STRING="\"usbmuxd 1.1.1\"" -DPACKAGE_BUGREPORT="\"https://github.com/libimobiledevice/usbmuxd/issues\"" \
		-DPACKAGE_URL="\"https://libimobiledevice.org\"" -DPACKAGE="\"usbmuxd\"" -DVERSION="\"1.1.1\"" -DHAVE_PPOLL \
		-DHAVE_LIBIMOBILEDEVICE -DHAVE_ENUM_IDEVICE_CONNECTION_TYPE \
		-static -lplist-2.0 -lusb-1.0 -limobiledevice-1.0 -lcrypto -lssl -lusbmuxd-2.0 \
		"${USBMUXD_DIR}/src/client.c" \
		"${USBMUXD_DIR}/src/conf.c" \
		"${USBMUXD_DIR}/src/device.c" \
		"${USBMUXD_DIR}/src/log.c" \
		"${USBMUXD_DIR}/src/main.c" \
		"${USBMUXD_DIR}/src/preflight.c" \
		"${USBMUXD_DIR}/src/usb.c" \
		"${USBMUXD_DIR}/src/utils.c"

readline: export CFLAGS = ${LTO_CFLAGS}
readline: export LDFLAGS = ${LTO_LDFLAGS}
readline: linux musl
	cd "${LIBREADLINE_DIR}" && ./configure \
		--prefix="${SYSROOT_DIR}" \
		--enable-shared=no \
		--enable-static=yes \
		--disable-install-examples
	$(MAKE) -C "${LIBREADLINE_DIR}"
	$(MAKE) -C "${LIBREADLINE_DIR}" install

clean_readline:
	$(MAKE) -C "${LIBREADLINE_DIR}" clean

ncurses: export CFLAGS = ${LTO_CFLAGS}
ncurses: export LDFLAGS = ${LTO_LDFLAGS}
ncurses: linux musl
	cd "${LIBNCURSES_DIR}" && ./configure \
		--prefix="${SYSROOT_DIR}" \
		--disable-big-core \
		--with-normal \
		--without-ada \
		--without-cxx \
		--without-cxx-binding \
		--without-debug \
		--without-manpages \
		--without-progs \
		--without-tack \
		--without-tests
	$(MAKE) -C "${LIBNCURSES_DIR}"
	$(MAKE) -C "${LIBNCURSES_DIR}" install

clean_ncurses:
	$(MAKE) -C "${LIBNCURSES_DIR}" clean

irecovery: export CFLAGS = ${LTO_CFLAGS}
irecovery: export LDFLAGS = ${LTO_LDFLAGS} -lncurses
irecovery: libusb readline ncurses
	cd "${IRECOVERY_DIR}" && ./autogen.sh \
		--prefix="${SYSROOT_DIR}" \
		--enable-shared=no \
		--enable-static=yes \
		--disable-install-examples \
		--without-udev
	$(MAKE) -C "${IRECOVERY_DIR}"
	$(MAKE) -C "${IRECOVERY_DIR}" install

clean_irecovery:
	$(MAKE) -C "${IRECOVERY_DIR}" clean

busybox: export CFLAGS = ${LTO_CFLAGS}
busybox: export LDFLAGS = ${LTO_LDFLAGS}
busybox: linux musl
	cp -f "${CONFIG_DIR}/busybox.config" "${BUSYBOX_DIR}/.config"
	$(MAKE) -C "${BUSYBOX_DIR}"
	cp -f "${BUSYBOX_DIR}/busybox" "${SYSROOT_DIR}/bin/busybox"

clean_busybox:
	$(MAKE) -C "${BUSYBOX_DIR}" clean

dropbear: export CFLAGS = ${LTO_CFLAGS} -Werror=implicit-function-declaration
dropbear: export LDFLAGS = ${LTO_LDFLAGS}
dropbear: linux musl
	cd "${DROPBEAR_DIR}" && autoconf && autoheader
	cd "${DROPBEAR_DIR}" && ./configure \
		--disable-harden \
		--disable-syslog \
		--disable-zlib \
		--enable-static \
		--prefix="${SYSROOT_DIR}"
	$(MAKE) -C "${DROPBEAR_DIR}" MULTI=1 PROGRAMS="dbclient scp"
	cp -f "${DROPBEAR_DIR}/dropbearmulti" "${SYSROOT_DIR}/bin/dropbearmulti"
	ln -sf "/bin/dropbearmulti" "${SYSROOT_DIR}/bin/scp"
	ln -sf "/bin/dropbearmulti" "${SYSROOT_DIR}/bin/dbclient"

clean_dropbear:
	$(MAKE) -C "${DROPBEAR_DIR}" clean

clean_sysroot:
	rm -rf "${SYSROOT_DIR}/bin" "${SYSROOT_DIR}/include" "${SYSROOT_DIR}/lib" "${SYSROOT_DIR}/share" "${BUILD_DIR}/bzImage" "${BUILD_DIR}/init.xz"

initramfs_pre: linux musl busybox libusbmuxd usbmuxd dropbear irecovery
	curl -sS --tlsv1.2 "https://assets.checkra.in/downloads/linux/cli/x86_64/${CHECKRA1N_VERSION}/checkra1n" -o "${SYSROOT_DIR}/bin/checkra1n"
	chmod +x "${SYSROOT_DIR}/bin/checkra1n"

init: initramfs_pre
	cargo build --release --target x86_64-unknown-linux-musl --manifest-path "${CHECKNIT_DIR}/Cargo.toml"
	llvm-strip --strip-all "${CHECKNIT_DIR}/target/x86_64-unknown-linux-musl/release/checknit"
	sstrip -z "${CHECKNIT_DIR}/target/x86_64-unknown-linux-musl/release/checknit"

clean_init:
	cargo clean --manifest-path "${CHECKNIT_DIR}/Cargo.toml"

initramfs: initramfs_pre init
	mkdir -p \
		"${INITRAMFS_DIR}/bin" \
		"${INITRAMFS_DIR}/dev" \
		"${INITRAMFS_DIR}/etc" \
		"${INITRAMFS_DIR}/etc/mdev" \
		"${INITRAMFS_DIR}/etc/network" \
		"${INITRAMFS_DIR}/etc/terminfo/l" \
		"${INITRAMFS_DIR}/proc" \
		"${INITRAMFS_DIR}/sbin" \
		"${INITRAMFS_DIR}/sys/dev" \
		"${INITRAMFS_DIR}/tmp" \
		"${INITRAMFS_DIR}/var/lib" \
		"${INITRAMFS_DIR}/var/run"
	cp -f  "${CHECKNIT_DIR}/target/x86_64-unknown-linux-musl/release/checknit" "${INITRAMFS_DIR}/init"
	cp -f  "${CONFIG_DIR}/linux.terminfo" "${INITRAMFS_DIR}/etc/terminfo/l/linux"
	cp -rf "${CONFIG_DIR}/odysseyra1n"    "${INITRAMFS_DIR}/etc"
	cp -rf "${ROOT_DIR}/licenses"/*.txt   "${INITRAMFS_DIR}/"
	cp -rf -t "${INITRAMFS_DIR}/bin" \
		"${SYSROOT_DIR}/bin/busybox" \
		"${SYSROOT_DIR}/bin/checkra1n" \
		"${SYSROOT_DIR}/bin/dropbearmulti" \
		"${SYSROOT_DIR}/bin/iproxy" \
		"${SYSROOT_DIR}/bin/irecovery" \
		"${SYSROOT_DIR}/bin/usbmuxd"
	printf "auto lo\niface lo inet loopback" > "${INITRAMFS_DIR}/etc/network/interfaces"
	ln -sf "/bin/busybox"       "${INITRAMFS_DIR}/sbin/mdev"
	ln -sf "/bin/dropbearmulti" "${INITRAMFS_DIR}/bin/dbclient"
	ln -sf "/bin/dropbearmulti" "${INITRAMFS_DIR}/bin/scp"
	cd "${INITRAMFS_DIR}" && \
		fd -E ".keep" -I | \
		cpio -ov --format=newc --owner=root:root | \
		xz --x86 --check=crc32 -vz9eT0 > ../init.xz

clean_initramfs:
	rm -rf "${INITRAMFS_DIR}"/*

iso: export GRUB_MODS = linux all_video configfile echo part_gpt part_msdos
iso: initramfs
	cp -f "${BUILD_DIR}/bzImage" "${ISO_DIR}/boot/bzImage"
	cp -f "${BUILD_DIR}/init.xz" "${ISO_DIR}/boot/init.xz"
	grub-mkrescue -o "checkplz.iso" "${ISO_DIR}" \
		--compress=xz \
		--fonts= \
		--install-modules="${GRUB_MODS}" \
		--modules="${GRUB_MODS}" \
		--locales= \
		--themes=

clean_iso:
	rm -f "${ROOT_DIR}/checkplz.iso"
	rm -f "${ISO_DIR}/boot/bzImage" "${ISO_DIR}/boot/init.xz"

clean: clean_init clean_linux clean_musl clean_libplist clean_libusbmuxd clean_openssl clean_busybox clean_dropbear clean_sysroot clean_initramfs clean_iso

qemu: initramfs
	qemu-system-x86_64 -m 1G -kernel "${BUILD_DIR}/bzImage" -initrd "${BUILD_DIR}/init.xz"

_prepare_for_release:
	rm -f checkplz.iso.b3sum checkplz.iso.sig
	b3sum checkplz.iso > checkplz.iso.b3sum
	gpg --output checkplz.iso.sig --detach-sig --armor checkplz.iso
