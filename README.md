**Important Note**:  
- This repo has been archived and replaced by the new [cocoa_grinwallet](https://github.com/gottstech/cocoa_grinwallet) repo.
- The main reason is this repo has commited the three binary libraries which makes the repo bigger and bigger and hard to clone.
- In the new [cocoa_grinwallet](https://github.com/gottstech/cocoa_grinwallet) repo, the binary libraries will be taken as the "release" assets, and no more commits on binaries.

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
git clone --recursive --depth 1 https://github.com/gottstech/cocoa_grinwallet.git
cd cocoa_grinwallet/rust
export OPENSSL_DIR="/usr/local/opt/openssl"
cargo lipo --release --targets aarch64-apple-ios,x86_64-apple-ios,armv7s-apple-ios
./copy_libs.sh
```

Note:
- The generated libs are in `Library/` folder.
- The `--depth 1` parameter of `git clone` is strongly proposed, to avoid downloading the big git history, since the three library files have about 100MB in git for each version with new libraries.
- If don't have openssl installed, please run:
  - For Mac: `brew install openssl`
  - For Linux: `sudo apt install libssl-dev`

## License

Apache License v2.0.

## Credits

The code was using the [Ironbelly](https://github.com/cyclefortytwo/ironbelly) as the reference.

The related code taken with thanks and respect, with license details in all derived source files.

Both Ironbelly and this project, are using same open source licence: Apache Licence v2.0.


