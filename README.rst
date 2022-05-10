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



Usage
========================================

.. code-block:: sh

    $ cargo arch --help
    cargo-arch 0.1.5
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
    ==> Making package: cargo-arch 0.1.5-1 (Tue 10 May 2022 03:46:55 PM CST)
    ==> Checking runtime dependencies...
    ==> Checking buildtime dependencies...
    ==> Retrieving sources...
    ==> Extracting sources...
    ==> Starting pkgver()...
    ==> Updated version: cargo-arch 0.1.5.r0.g21098bb-1
    ==> Removing existing $pkgdir/ directory...
    ==> Starting build()...
    ==> Entering fakeroot environment...
    ==> Starting package()...
      Installing cargo-arch v0.1.5 (/home/user/cargo-arch)
        Updating crates.io index
       Compiling proc-macro2 v1.0.38
       Compiling unicode-xid v0.2.3
       Compiling syn v1.0.93
       Compiling version_check v0.9.4
       Compiling autocfg v1.1.0
       Compiling libc v0.2.125
       Compiling serde_derive v1.0.137
       Compiling serde v1.0.137
       Compiling anyhow v1.0.57
       Compiling hashbrown v0.11.2
       Compiling os_str_bytes v6.0.0
       Compiling heck v0.4.0
       Compiling textwrap v0.15.0
       Compiling termcolor v1.1.3
       Compiling strsim v0.10.0
       Compiling lazy_static v1.4.0
       Compiling bitflags v1.3.2
       Compiling proc-macro-error-attr v1.0.4
       Compiling proc-macro-error v1.0.4
       Compiling indexmap v1.8.1
       Compiling clap_lex v0.2.0
       Compiling quote v1.0.18
       Compiling atty v0.2.14
       Compiling clap_derive v3.1.7
       Compiling clap v3.1.17
       Compiling toml v0.5.9
       Compiling cargo-arch v0.1.5 (/home/user/cargo-arch)
        Finished release [optimized] target(s) in 27.85s
      Installing /home/user/cargo-arch/pkg/cargo-arch/usr/bin/cargo-arch
       Installed package `cargo-arch v0.1.5 (/home/user/cargo-arch)` (executable `cargo-arch`)
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
    ==> Finished making: cargo-arch 0.1.5.r0.g21098bb-1 (Tue 10 May 2022 03:47:24 PM CST)




Customization
========================================

You can put your ``PKGBUILD`` fields setting in ``Cargo.toml`` under ``[package.metadata.arch]`` section:

.. code-block:: toml

    [package.metadata.arch]
    arch = ["x86_64"]
    makedepends = ["cargo", "my-build-dep1", "my-build-dep2"]
    depends = ["my-run-dep1", "my-run-dep2"]
    provides = ["cargo-arch", "my-command1", "my-command2"]



Information About Binary
========================================

Commands Dependency
------------------------------

* `makepkg <https://wiki.archlinux.org/index.php/makepkg>`_



Changelog
========================================

Not Implemented Yet (Plan)
------------------------------


v0.1.5 (2022-05-10)
------------------------------

* a lot of dependencies update, drop YAML CLI setup
* update Rust edition from 2018 to 2021
* fix #16 by moving the filling of serde's defaults from CargoMetadata to CargoArch (thanks @zraktvor)
* Leave build() empty in order to not build the crate twice (thanks @gkaklas)
* Use --no-track flag to avoid installing .crates.toml and .crates2.json (thanks @gkaklas)
* ensure that the defaults of the arch section are populated (thanks @cardoe)


v0.1.4 (2019-12-07)
------------------------------

* better error report with `anyhow` (thanks @cardoe)
* default arch to x86_64 (thanks @cardoe)
* fix bash array syntax (thanks @cardoe)
* share artifact between build and package stage (thanks @cardoe)



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

* `anyhow <https://github.com/dtolnay/anyhow>`_ for better error message
* `cargo-deb <https://github.com/mmstick/cargo-deb>`_ for generates Debian packages (as a reference)
* `trust <https://github.com/japaric/trust/>`_ for CI integration
* `rust-everywhere <https://github.com/japaric/rust-everywhere/>`_ for CI integration (old)
* `clap-rs <https://github.com/kbknapp/clap-rs>`_ for arguments parsing
* `serde <https://github.com/serde-rs/serde>`_ for nice deserialization API
* `toml-rs <https://github.com/alexcrichton/toml-rs>`_ for parsing TOML config and integrate with Serde
* `Rust Team <https://www.rust-lang.org/team.html>`_
* and every project I've used



License
========================================

cargo-arch is licensed under the Apache-2.0 License - see the ``LICENSE`` file for details
