# Aperture
> Free and open source Rust bindings intended to expose a subset of Steamworks.

## What is this?

These are safe, idiomatic APIs for Steamworks built on top of a well contained FFI layer.

This API exists to bridge the gap between Steam (which is dominantly in C++ and C) and **your** games written in Rust.

> [!NOTE]
> This project does not aim to replace `[steamworks-rs](https://github.com/Noxime/steamworks-rs)`; it is an independant implementation with a different scope and philosophy.

## What this is not

This is not planned to be a complete re-implementation of Steamworks in Rust. The primary objective of this repository is to implement a minimal set of functions that the majority of games require.

Complex and heavy systems like `ISteamEconomy` or `ISteamGameServer` are not planned to be implemented in the near term due to complexity.

## Usage

> [!IMPORTANT]
> Coming soon, here be dragons...

## License

This project is licensed solely under the [MIT License](./LICENSE). Steamworks itself remains the proprietary property of Valve Software and **is not included in this project.**

> [!IMPORTANT]
> We cannot redistribute the Steamworks SDK. You **need** your own copy from Valve directly.

## Contributing!

Contributions are always welcome and will never be turned away. If you can make this project better, faster, or more in-line with Steamworks, fork the repository and lend a hand!

A short guide on our code style and tips on contributing can be found [right here!](./.github/CONTRIBUTING.md)
