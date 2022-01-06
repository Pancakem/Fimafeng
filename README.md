# Welcome to Fimafeng 👋
![Version](https://img.shields.io/badge/version-0.1.0-blue.svg?cacheSeconds=2592000)
[![Documentation](https://img.shields.io/badge/documentation-yes-brightgreen.svg)](https://github.com/Pancakem/Fimafeng)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/Pancakem/Fimafeng/blob/main/LICENSE)

> Fimafeng is a very simple web server implementation of a subset of HTTP/1.1 written from scratch. Its few features are:

- HTTPS
- Virtual hosting

It is easy to spin up an instance on your machine; see the [usage](#usage) section.

## Install

To start, clone this [repo](https://github.com/Pancakem/Fimafeng)

```sh
git clone https://github.com/Pancakem/Fimafeng
cd Fimafeng
```

Fimafeng is written in [Rust](https://rust-lang.org). It is recommended to [install Rust](https://www.rust-lang.org/tools/install) with [rustup](https://rust-lang.github.io/rustup/index.html), a tool that installs and manages different versions of the Rust toolchain. It also installs [cargo](https://doc.rust-lang.org/cargo/index.html) by default, which this project uses.

## Usage
To start `Fimafeng`, all that is required is the binary and config file. One only needs to pass in the config file and voila!

```sh
fimafeng path/to/config.yaml
```

## Run tests

Fimafeng only tests its parser. The tests can be run by:

```sh
cargo test
```

## Configuration
Configuring Fimafeng is done with config files written in YAML. Example config files are provided in /resources:


## Author

👤 **Marvin Ouma**

* Website: https://32bitsaviour.netlify.app/
* Github: [@pancakem](https://github.com/pancakem)

## Show your support

Give a ⭐️ if this project helped you!


## 📝 License

Copyright © 2022 [Marvin Ouma](https://github.com/pancakem).

This project is [MIT](https://github.com/Pancakem/Fimafeng/blob/main/LICENSE) licensed.
