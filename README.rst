========================================================================
cargo-arch - generate Arch Linux packages from information in Cargo.toml
========================================================================

cargo-arch will generate ``PKGBUILD`` from information in Cargo.toml.
You can add extra information in ``[package.metadata.arch]`` sections,
options can be found by ``man PKGBUILD``.

`Documentation <https://wdv4758h.github.io/cargo-arch/cargo_arch/>`_


.. contents:: Table of Contents



Installation
========================================

From `crate.io <https://crates.io/>`_

.. code-block:: sh

    $ cargo install cargo-arch


From GitHub

.. code-block:: sh

    $ cargo install --git https://github.com/wdv4758h/cargo-arch


Download Prebuilt Binary

.. code-block:: sh

    # by curl
    $ curl -O -J -L https://github.com/wdv4758h/cargo-arch/releases/download/v0.1.0/cargo-arch-v0.1.0-x86_64-unknown-linux-gnu.tar.gz

    # by wget
    $ wget https://github.com/wdv4758h/cargo-arch/releases/download/v0.1.0/cargo-arch-v0.1.0-x86_64-unknown-linux-gnu.tar.gz



Usage
========================================

.. code-block:: sh

    $ cargo arch --help
    cargo-arch 0.1.0
    Chiu-Hsiang Hsu <wdv4758h@gmail.com>
    Rust Arch Linux package packer

    USAGE:
        cargo arch [FLAGS] [OPTIONS]

    FLAGS:
        -f, --force        Overwrite existing package
        -h, --help         Prints help information
        -i, --install      Install package after successful build
            --mksrcinfo    Run mksrcinfo
        -s, --syncdeps     Install missing dependencies with pacman
        -V, --version      Prints version information

    OPTIONS:
        -b, --build <build>    whether build the source [default: true]


.. code-block:: sh

    $ cargo arch
    ==> Making package: cargo-arch 0.1.0-1 (Wed Jul 27 10:38:13 CST 2016)
    ==> Checking runtime dependencies...
    ==> Checking buildtime dependencies...
    ==> Retrieving sources...
    ==> Extracting sources...
    ==> Starting pkgver()...
    ==> Removing existing $pkgdir/ directory...
    ==> Starting build()...
    ==> Entering fakeroot environment...
    ==> Starting package()...
      Installing /home/user/zone/cargo-arch/pkg/cargo-arch/bin/cargo-arch
    warning: be sure to add `/home/user/zone/cargo-arch/pkg/cargo-arch/bin` to your PATH to be able to run the installed binaries
    ==> Tidying install...
      -> Removing libtool files...
      -> Purging unwanted files...
      -> Removing static library files...
      -> Stripping unneeded symbols from binaries and libraries...
      -> Compressing man and info pages...
    ==> Checking for packaging issue...
    ==> Creating package "cargo-arch"...
      -> Generating .PKGINFO file...
      -> Generating .BUILDINFO file...
      -> Generating .MTREE file...
      -> Compressing package...
    ==> Leaving fakeroot environment.
    ==> Finished making: cargo-arch 0.1.0-1 (Wed Jul 27 10:38:15 CST 2016)



Information About Binary
========================================

Size
------------------------------

x86_64, Linux (build on Arch Linux)

+------------+---------+------------+--------------+-----------+
| Filename   | Version | Stripped ? | Size (Bytes) | Size (MB) |
+------------+---------+------------+--------------+-----------+
| cargo-arch | v0.1.0  | No         | 1767456      | 1.7M      |
+------------+---------+------------+--------------+-----------+
| cargo-arch | v0.1.0  | Yes        | 1382280      | 1.4M      |
+------------+---------+------------+--------------+-----------+


x86_64, Linux, musl (build on Arch Linux)

+------------+---------+------------+--------------+-----------+
| Filename   | Version | Stripped ? | Size (Bytes) | Size (MB) |
+------------+---------+------------+--------------+-----------+
| cargo-arch | v0.1.0  | No         | 2139368      | 2.1M      |
+------------+---------+------------+--------------+-----------+
| cargo-arch | v0.1.0  | Yes        | 1482536      | 1.5M      |
+------------+---------+------------+--------------+-----------+


Shared Library Dependency
------------------------------

x86_64, Linux (build on Arch Linux)

.. code-block:: sh

    $ ldd ./target/release/cargo-arch
            linux-vdso.so.1 (0x00007ffec8387000)
            libdl.so.2 => /usr/lib/libdl.so.2 (0x00007f5f798c7000)
            libpthread.so.0 => /usr/lib/libpthread.so.0 (0x00007f5f796aa000)
            libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x00007f5f79494000)
            libc.so.6 => /usr/lib/libc.so.6 (0x00007f5f790f3000)
            /lib64/ld-linux-x86-64.so.2 (0x00007f5f79acb000)



x86_64, Linux, musl (build on Arch Linux)

.. code-block:: sh

    $ ldd ./target/x86_64-unknown-linux-musl/release/cargo-arch
            not a dynamic executable


Commands Dependency
------------------------------

* `makepkg <https://wiki.archlinux.org/index.php/makepkg>`_



Changelog
========================================

Not Implemented Yet (Plan)
------------------------------


v0.1.0 (2016-07-27)
------------------------------

* support building Arch Linux packages



Notice
========================================

I've only tested on my x86_64 Linux.
Other platforms are built by CI.
If they don't work properly, please tell me.



Developement
========================================

Making Release
------------------------------

1. update version in ``src/arguments.yml``
2. update version in ``Cargo.toml``
3. update version in ``Cargo.lock``
4. add git tag



Special Thanks
========================================

* `cargo-deb <https://github.com/mmstick/cargo-deb>`_ for generates Debian packages
* `rust-everywhere <https://github.com/japaric/rust-everywhere/>`_ for CI integration
* `clap-rs <https://github.com/kbknapp/clap-rs>`_ for arguments parsing
* `Rust Team <https://www.rust-lang.org/team.html>`_
* and every project I've used



License
========================================

cargo-arch is licensed under the Apache-2.0 License - see the ``LICENSE`` file for details
