# Caveats
- includes bingen-generated bindings for amd64 in a file to save the
  dependency, other architectures need to use `--features make_bindings`.

# Features
- `make_bindings`: Use `bindgen` for the bindings instead of using the
  pregenerated ones from the included file.
- `update_bindings`: Update the pregenerated bindings, this is a development
  aid.
- `build_server`: Build the clockkit server too, requires `make`.

# Clockkit Version bundled
8ddc8f80c79dba9cebe7f67bc299164e78ae7b0f

# License
MIT