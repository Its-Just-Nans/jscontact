# jscontact

- <https://docs.rs/jscontact> - documentation
- <https://github.com/Its-Just-Nans/jscontact> - repository
- <https://www.ietf.org/rfc/rfc9553.txt> - RFC 9553

## Tests

```sh
rm -rf tests/rfc9553
python tests/get_figures.py
cargo build
cargo test -- --test-threads=1
# the --test-threads=1 is used to have a deterministic (ordered) output
```
