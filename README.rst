========================================================================
cargo-arch - generate Arch Linux packages from information in Cargo.toml
========================================================================



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



Changelog
========================================

Not Implemented Yet (Plan)
------------------------------


v0.1.0 (2016-07-27)
------------------------------

* support building Arch Linux's package



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
* `clap-rs <https://github.com/kbknapp/clap-rs>`_ for arguments parsing
* `Rust Team <https://www.rust-lang.org/team.html>`_
* and every project I've used



License
========================================

cargo-arch is licensed under the Apache-2.0 License - see the ``LICENSE`` file for details
