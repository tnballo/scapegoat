#! /bin/bash

cargo readme --no-indent-headings > README.md

# Image before crate name/version header
echo -e "<br><p align=\"center\"><img src=\"https://raw.githubusercontent.com/tnballo/scapegoat/master/img/scapegoat.svg\" width=\"333\" alt=\"scapegoat\"></p><br>\n$(cat README.md)" > README.md

# Make sure we're still truly no_std
cargo build --target="thumbv7m-none-eabi"
