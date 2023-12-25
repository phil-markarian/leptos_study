## Errors

If you get something like the following
``` error from asset pipeline```
you will need to do the following ```cargo install --locked wasm-bindgen-cli```

`--locked` here means that it will adjust the version according to your Cargo.lock file contents.

