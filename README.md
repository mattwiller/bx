# bx 

[![Build status](https://img.shields.io/github/workflow/status/mattwiller/bx/Rust/master)](https://github.com/mattwiller/bx/actions?query=workflow%3ARust)
[![License: Apache 2.0](https://img.shields.io/crates/l/bx)](https://www.apache.org/licenses/LICENSE-2.0.txt)
[![crates.io](https://img.shields.io/crates/v/bx)](https://crates.io/crates/bx)

A smaller, faster Box CLI

<!-- START tocify -->
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [Changelog](#changelog)
- [License](#license)
<!-- END tocify -->

## Installation

```bash
cargo install bx
```

## Usage

```bash
bx file <FILE_ID> --download-to . --token <BOX_DEV_TOKEN>
bx user -t <BOX_DEV_TOKEN>
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## Changelog

See [CHANGELOG.md](./CHANGELOG.md)

## License

[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0.txt)