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
    $ curl -O -J -L https://github.com/wdv4758h/cargo-arch/releases/download/v0.1.3/cargo-arch-v0.1.3-x86_64-unknown-linux-gnu.tar.gz

    # by wget
    $ wget https://github.com/wdv4758h/cargo-arch/releases/download/v0.1.3/cargo-arch-v0.1.3-x86_64-unknown-linux-gnu.tar.gz



Usage
========================================

.. code-block:: sh

    $ cargo arch --help
    cargo-arch 0.1.3
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
        -b, --build <build>                    whether build the source [default: true]  [possible values:
                                               true, false]
        -p, --manifest-path <manifest-path>    Cargo.toml directory path


.. code-block:: sh

    $ cargo arch
    ==> Making package: cargo-arch 0.1.3-1 (Fri 13 Sep 2019 08:01:50 PM CST)
    ==> Checking runtime dependencies...
    ==> Checking buildtime dependencies...
    ==> Retrieving sources...
    ==> Extracting sources...
    ==> Starting pkgver()...
    ==> Updated version: cargo-arch 0.1.3.r0.g6b86760-1
    ==> Removing existing $pkgdir/ directory...
    ==> Starting build()...
       Compiling proc-macro2 v1.0.3
       Compiling libc v0.2.62
       Compiling unicode-xid v0.2.0
       Compiling serde v1.0.100
       Compiling bitflags v1.1.0
       Compiling syn v1.0.5
       Compiling unicode-width v0.1.6
       Compiling strsim v0.8.0
       Compiling ansi_term v0.11.0
       Compiling yaml-rust v0.3.5
       Compiling vec_map v0.8.1
       Compiling textwrap v0.11.0
       Compiling atty v0.2.13
       Compiling quote v1.0.2
       Compiling clap v2.33.0
       Compiling toml v0.5.3
       Compiling serde_derive v1.0.100
       Compiling cargo-arch v0.1.3 (/home/user/cargo-arch)
        Finished release [optimized] target(s) in 1m 29s
    ==> Entering fakeroot environment...
    ==> Starting package()...
      Installing cargo-arch v0.1.3 (/home/user/cargo-arch)
        Updating crates.io index
        Finished release [optimized] target(s) in 4.07s
      Installing /home/user/cargo-arch/pkg/cargo-arch/usr/bin/cargo-arch
       Installed package `cargo-arch v0.1.3 (/home/user/cargo-arch)` (executable `cargo-arch`)
    warning: be sure to add `/home/user/cargo-arch/pkg/cargo-arch/usr/bin` to your PATH to be able to run the installed binaries
    ==> Tidying install...
      -> Removing libtool files...
      -> Purging unwanted files...
      -> Removing static library files...
      -> Stripping unneeded symbols from binaries and libraries...
      -> Compressing man and info pages...
    ==> Checking for packaging issues...
    ==> Creating package "cargo-arch"...
      -> Generating .PKGINFO file...
      -> Generating .BUILDINFO file...
      -> Generating .MTREE file...
      -> Compressing package...
    ==> Leaving fakeroot environment.
    ==> Finished making: cargo-arch 0.1.3.r0.g6b86760-1 (Fri 13 Sep 2019 08:03:25 PM CST)



Information About Binary
========================================

Commands Dependency
------------------------------

* `makepkg <https://wiki.archlinux.org/index.php/makepkg>`_



Changelog
========================================

Not Implemented Yet (Plan)
------------------------------


v0.1.3 (2019-09-13)
------------------------------

* Add ``--manifest-path`` support (thanks @ZettaScript)
* Update dependencies version


v0.1.2 (2019-01-06)
------------------------------

* Use "/usr" instead of "/" as install base


v0.1.1 (2019-01-06)
------------------------------

* Change to use serde instead of rustc-serialize
* Update PKGBUILD template to remove .crates.toml
* Update toml to use serde API
* Update clap version


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
* `trust <https://github.com/japaric/trust/>`_ for CI integration
* `clap-rs <https://github.com/kbknapp/clap-rs>`_ for arguments parsing
* `serde <https://github.com/serde-rs/serde>`_ for nice deserialization API
* `toml-rs <https://github.com/alexcrichton/toml-rs>`_ for parsing TOML config and integrate with Serde
* `Rust Team <https://www.rust-lang.org/team.html>`_
* and every project I've used



License
========================================

cargo-arch is licensed under the Apache-2.0 License - see the ``LICENSE`` file for details
