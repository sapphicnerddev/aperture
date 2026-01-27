# Contributing to Aperture

Thanks for your interest in contributing to Aperture! 🎉  
Contributions of all kinds are welcome — code, documentation, testing, and design discussions.

## Scope & Philosophy

Aperture aims to provide **safe, idiomatic Rust APIs** built on top of a **minimal, well-contained unsafe FFI layer** for Steamworks.

When contributing, please keep in mind:
- The `aperture-sys` crate should remain **low-level, minimal, and boring**
- All unsafe code must be **justified, documented, and contained**
- Higher-level abstractions belong in the safe Rust layer, not `aperture-sys`
- Unsafe code should stay out of the main `aperture` crate at all costs
- Functions should be properly documented

## Getting Started

1. Fork the repository
2. Create a new branch for your changes
3. Make your changes with clear, focused commits
4. Open a pull request describing *what* you changed and *why*

For larger changes, opening an issue first is encouraged.

## Code Guidelines

- Follow standard Rust formatting (`cargo fmt` before committing code)
- Avoid introducing unnecessary dependencies
- Prefer explicitness over cleverness, especially in unsafe code
- Public APIs should be documented

## Steamworks Disclaimer

This project **does not** redistribute the Steamworks SDK. \
You must use your own copy obtained directly from Valve.

Please do not include Steamworks headers, binaries, or other proprietary files in pull requests.

## Questions & Discussion

If something is unclear or you want feedback before implementing a change, feel free to open an issue — discussion is always welcome and encouraged.

Thanks for helping make Aperture better! ✨