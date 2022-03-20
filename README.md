# Caveats
- includes bingen-generated bindings for amd64 in a file to save the
  dependency, other architectures need to use `--features make_bindings`.
- currently this crate always builds the bundled sources and does not check for
  an installation of `clockkit`.

# Features
- `make_bindings`: Use `bindgen` for the bindings instead of using the
  pregenerated ones from the included file.
- `update_bindings`: Update the pregenerated bindings, this is a development
  aid.
- `build_server`: Build the clockkit server too, requires `make`.

# Clockkit
For further details see the [clockkit repository](https://github.com/camilleg/clockkit).

## Version bundled
a7856021da846988d022879c95ec745caa5ae9e8

# License
MIT
