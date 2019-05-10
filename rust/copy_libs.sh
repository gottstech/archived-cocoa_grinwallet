#!/bin/sh

ls -l target/aarch64-apple-ios/release/libgrinwallet.a
ls -l target/armv7s-apple-ios/release/libgrinwallet.a
ls -l target/x86_64-apple-ios/release/libgrinwallet.a

cp target/aarch64-apple-ios/release/libgrinwallet.a ../cocoa_grinwallet/Library/libgrinwallet_aarch64-apple-ios.a
cp target/armv7s-apple-ios/release/libgrinwallet.a ../cocoa_grinwallet/Library/libgrinwallet_armv7s-apple-ios.a
cp target/x86_64-apple-ios/release/libgrinwallet.a ../cocoa_grinwallet/Library/libgrinwallet_x86_64-apple-ios.a

ls -l ../cocoa_grinwallet/Library

