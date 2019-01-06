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
    $ curl -O -J -L https://github.com/wdv4758h/cargo-arch/releases/download/v0.1.1/cargo-arch-v0.1.1-x86_64-unknown-linux-gnu.tar.gz

    # by wget
    $ wget https://github.com/wdv4758h/cargo-arch/releases/download/v0.1.1/cargo-arch-v0.1.1-x86_64-unknown-linux-gnu.tar.gz



Usage
========================================

.. code-block:: sh

    $ cargo arch --help
    cargo-arch 0.1.1
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
        -b, --build <build>    whether build the source [default: true]  [possible values: true, false]


.. code-block:: sh

    $ cargo arch
    ==> Making package: cargo-arch 0.1.1-1 (Sun 06 Jan 2019 09:01:09 PM CST)
    ==> Checking runtime dependencies...
    ==> Checking buildtime dependencies...
    ==> Retrieving sources...
    ==> Extracting sources...
    ==> Starting pkgver()...
    ==> Updated version: cargo-arch 0.1.1.r0.gbef965b-1
    ==> Starting build()...
       Compiling proc-macro2 v0.4.24
       Compiling unicode-xid v0.1.0
       Compiling serde v1.0.84
       Compiling unicode-width v0.1.5
       Compiling libc v0.2.15
       Compiling vec_map v0.8.1
       Compiling yaml-rust v0.3.5
       Compiling strsim v0.7.0
       Compiling ansi_term v0.11.0
       Compiling bitflags v1.0.4
       Compiling textwrap v0.10.0
       Compiling atty v0.2.11
       Compiling clap v2.32.0
       Compiling quote v0.6.10
       Compiling syn v0.15.24
       Compiling toml v0.4.10
       Compiling serde_derive v1.0.84
       Compiling cargo-arch v0.1.1 (/home/user/cargo-arch)
        Finished release [optimized] target(s) in 1m 48s
    ==> Entering fakeroot environment...
    ==> Starting package()...
      Installing cargo-arch v0.1.1 (/home/user/cargo-arch)
        Finished release [optimized] target(s) in 0.07s
      Installing /home/user/cargo-arch/pkg/cargo-arch/bin/cargo-arch
    warning: be sure to add `/home/user/cargo-arch/pkg/cargo-arch/bin` to your PATH to be able to run the installed binaries
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
    ==> Finished making: cargo-arch 0.1.1.r0.gbef965b-1 (Sun 06 Jan 2019 09:02:59 PM CST)



Information About Binary
========================================

Commands Dependency
------------------------------

* `makepkg <https://wiki.archlinux.org/index.php/makepkg>`_



Changelog
========================================

Not Implemented Yet (Plan)
------------------------------


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
