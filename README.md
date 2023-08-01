
*Note: This project is still under construction!*

# Kneeboard Notes

Provides a way to create notes for A5 aviation kneeboards. The format of the generated notes is primarily relevent to PPL(A) in the UK.

The notes are generated as a PDF document for printing, and can be saved as [YAML](https://en.wikipedia.org/wiki/YAML), [JSON](https://en.wikipedia.org/wiki/JSON) is also supported to a lesser extent.

The online version is available here: [Kneeboard Notes](https://kneeboard.github.io)

# Disclaimer

Do not use the notes generated by this software - they are only for illustrative purposes.

Any reliance you place on this software and/or the generated notes is strictly at your own risk!

## Build and run

### Command line tool
```cargo run --bin kneeboard```

### Web UI
Install WebAssembly target

```rustup target add wasm32-unknown-unknown```

Install trunk

```cargo install --locked trunk```

```
cd web
trunk serve
```

Open ```127.0.0.1:8080``` in a browser

## Development

Before submitting a pull request, please:

Run ```cargo clippy``` and resolve all issues.

Run ```cargo test``` and resolve all issues.

Finally run ```cargo fmt```

## License

Licensed under:

Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE.md) or http://www.apache.org/licenses/LICENSE-2.0)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.
