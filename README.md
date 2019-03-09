dslite2svd
==========

TI does not publish SVD files for many of their newer CPUs, including the TM4C series.
However, they do publish some sort of debug-related files in [Energia][] that are basically
a not-invented-here variant of SVD. This repository contains a converter, _dslite2svd_.

[Energia]: http://energia.nu/

Usage
-----

1. Install Ruby >= 2.0. Run `bundle install`.
2. Install [svd2rust]. Run `cargo install svd2rust`.
3. Install [form]. Run `cargo install form`.
4. Install TM4C support package in Energia.
5. Copy the `targetdb` directory from `energia-*/hardware/tools/DSLite/common`
   to the root of this repository.
6. Modify `Makefile` to include your target if it's not already there. Run `make`.

The up-to-date SVD files will be placed in the `svd` directory. For convenience,
they are already provided in this repository.

[form]: https://github.com/djmcgill/form
[svd2rust]: https://github.com/rust-embedded/svd2rust

Limitations
-----------

The output of _dslite2svd_ is checked against the SVD schema using xmllint, if it is installed
(xmllint is a part of libxml). It should also be robust against unrecognized input,
but no strong guarantees are made.

Some features are not implemented:
  * The CPU section of the SVD file is not populated.

License (of dslite2svd)
-----------------------

[0-clause BSD license](LICENSE-0BSD.txt).

License (of generated SVD files)
--------------------------------

Register definitions are collections of facts and not original works and therefore are
not generally covered by copyright. Energia or its TM4C module are also not covered by EULA.
