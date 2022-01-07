# `spi-flash`

This crate provides a generic [`embedded-hal`]-based driver for different
families of SPI Flash and EEPROM chips.

Right now, only 25-series Flash chips are supported. Feel free to send PRs to
support other families though!

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

## Usage

Add an entry to your `Cargo.toml`:

```toml
[dependencies]
spi-flash = { git = "https://github.com/technocreatives/spi-flash" }
```
