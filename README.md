# checkplz

checkplz is a small (sub-30MB) Linux ISO that can run checkra1n and odysseyra1n.

## How to burn to a flash drive

**ALL DATA ON THE FLASH DRIVE WILL BE LOST!!**

### Windows

1. Download and open [Rufus](https://rufus.ie/)
2. Select your flash drive under "Device".
3. Ensure "Boot or ISO image" is selected under "Boot selection", then click SELECT and find your checkplz.iso and select it
4. Click "START"

### macOS

1. Open up a terminal
2. Check your disks with `diskutil list`, and find the `/dev/diskN` which corresponds to your flash drive, replacing `N` with the number.
3. Unmount your flash drive with `/dev/diskN`
4. `sudo dd if=/dev/diskN of=checkplz.iso bs=8m`

### Linux

1. Open up a terminal
2. Check your disks with `lsblk`, and find the `/dev/sdX` which corresponds to your flash drive, replacing `X` with the identifier.
3. If neccessary, unmount any partitions with `sudo umount /dev/sdXN`
4. `sudo dd if=/dev/sdX of=checkplz.iso status=progress bs=8m`

### Cross-Platform — Etcher

[balenaEtcher](https://www.balena.io/etcher/) is cross-platform and works, but **I do not endorse it**, as an _Electron app_ to flash USB sticks is just stupid and bloated in my opinion.

### Cross-Platform — Ventoy

If you use [Ventoy](https://www.ventoy.net/en/index.html) to put multiple ISOs on one flash drive, checkplz will work with this too, just ensure that you enable "RAMdisk" before booting checkplz.iso from the Ventoy boot screen!

## Building

This is held together with duct tape and a Makefile. No guarantees are made about whether this will work or not, and it is not my fault if this accidentally deletes everything while building or something. It *should* work, but again, not my fault if it doesn't.

```bash
git clone https://github.com/aspenluxxxy/checkplz
cd checkplz
git submodule update --init --recursive
# You can also do `git clone --recursive https://github.com/aspenluxxxy/checkplz`
patch -p1 -dlibraries/libusbmuxd < patches/libusbmuxd-no-ipv6.patch
patch -p1 -dprograms/busybox < patches/busybox-use-musl-clang.patch
patch -p1 -dprograms/usbmuxd < patches/usbmuxd-dont-conflict-with-libplist.patch
make -j6 iso
```

This will create `checkplz.iso`. You can then flash this to a USB stick, using [Rufus](https://rufus.ie/) or `dd`.

## Licenses

 * [musl](https://musl.libc.org/) and [dropbear](https://matt.ucc.asn.au/dropbear/dropbear.html) are both licensed under the [MIT License](https://tldrlegal.com/license/mit-license)
 * [libusbmuxd](https://github.com/libimobiledevice/libusbmuxd), [libplist](https://github.com/libimobiledevice/libplist), [libusb](https://github.com/libusb/libusb), and [libimobiledevice](https://github.com/libimobiledevice/libimobiledevice) are licensed under the [GNU Lesser General Public License v2.1](https://tldrlegal.com/license/gnu-lesser-general-public-license-v2.1-(lgpl-2.1))
 * [linux](https://www.kernel.org/) and [busybox](https://busybox.net/) are both licensed under the [GNU General Public License v2.0](https://tldrlegal.com/license/gnu-general-public-license-v2)
 * [usbmuxd](https://github.com/libimobiledevice/usbmuxd) and [checknit](./checknit) are both licensed under the [GNU General Public License v3](https://tldrlegal.com/license/gnu-general-public-license-v3-(gpl-3))
 * [openssl](https://www.openssl.org/) is licensed under the [Apache License 2.0](https://tldrlegal.com/license/apache-license-2.0-(apache-2.0))
 * [checkra1n](https://checkra.in/) was made by the checkra1n team/Kim Jong Cracks.
 * The [odysseyra1n bootstrap](https://github.com/coolstar/Odyssey-bootstrap) was made by [coolstar](https://github.com/coolstar)

In addition, any scripts or configuration files contained in this repository that are not already under a different license or copyrighted by someone else are licensed under the [GNU General Public License v3](https://tldrlegal.com/license/gnu-general-public-license-v3-(gpl-3)).

All full license texts are available in the [`licenses`](./licenses) folder of this repository.
