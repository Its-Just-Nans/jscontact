# jscontact [![crates.io version](https://img.shields.io/crates/v/jscontact)](https://crates.io/crates/jscontact) ![crates.io downloads](https://img.shields.io/crates/d/jscontact)

- <https://docs.rs/jscontact> - documentation
- <https://github.com/Its-Just-Nans/jscontact> - repository
- <https://www.rfc-editor.org/rfc/rfc9553> - RFC 9553

## Tests

```sh
rm -rf tests/rfc9553
python tests/get_figures.py
cargo build
cargo test -- --test-threads=1
cargo test --no-default-features -- --test-threads=1
# the --test-threads=1 is used to have a deterministic (ordered) output
```

## License

Licensed under the MIT license [LICENSE](LICENSE) except for the `tests` directory.

The `tests` directory contains the following files:

- the RFC 9553 figures
- some imported tests files from other repositories
