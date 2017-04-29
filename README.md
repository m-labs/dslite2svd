ti2svd
======

TI does not publish SVD files for many of their newer CPUs, including the TM4C series.
However, they do publish some sort of debug-related files in [Energia][] that are basically
a not-invented-here variant of SVD. This repository contains a converter.

[Energia]: http://energia.nu/

Usage
-----

1. Install Ruby >= 2.0. Run `bundle install`.
2. Install TM4C support package in Energia.
3. Copy the `targetdb` directory from `energia-*/hardware/tools/DSLite` to the root
   of this repository.
4. Modify `Makefile` to include your target if it's not already there. Run `make`.

The up-to-date SVD files will be placed in the `svd` directory. For convenience,
they are already provided in this repository.

Limitations
-----------

The output of ti2svd is checked against the SVD schema using xmllint, if it is installed
(xmllint is a part of libxml). It should also be robust against unrecognized input,
but no strong guarantees are made.

Some features are not implemented:
  * Reset values are ignored and not converted. The TI XML descriptions I looked at
    do not have them anyway.
  * The CPU section of the SVD file is not populated.

License (of ti2svd)
-------------------

[0-clause BSD license](LICENSE-0BSD.txt).

License (of generated SVD files)
--------------------------------

Register definitions are collections of facts and not original works and therefore are
not generally covered by copyright. Energia or its TM4C module are also not covered by EULA.
