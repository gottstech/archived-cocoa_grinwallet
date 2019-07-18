# cocoa_grinwallet
IOS Grin Wallet Pod

## Build
### Set up the environment

- Install Xcode build tools:

`xcode-select --install`

- Install Rust:

`curl https://sh.rustup.rs -sSf | sh`

- Add ios architectures to rustup:

`rustup target add aarch64-apple-ios x86_64-apple-ios armv7s-apple-ios`

- Install `cargo-lipo`, a cargo sub-command for creating iOS libs:

`cargo install cargo-lipo`

### Build the libs

```
git clone --recursive https://github.com/gottstech/cocoa_grinwallet.git
cd cocoa_grinwallet/rust
cargo lipo --release --targets aarch64-apple-ios,x86_64-apple-ios,armv7s-apple-ios
./copy_libs.sh
```

The generated libs are in `Library/` folder.

## License

Apache License v2.0.

## Credits

The code was using the [Ironbelly](https://github.com/cyclefortytwo/ironbelly) as the reference.

The related code taken with thanks and respect, with license details in all derived source files.

Both Ironbelly and this project, are using same open source licence: Apache Licence v2.0.


