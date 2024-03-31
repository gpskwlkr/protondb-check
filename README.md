<p align="center">
  <img alt="ProtonDB logo" src="./readme_items/protondb.svg" height="150" width="150" />
</p>

<h1 align="center">
protondb-check
</h1>

<div align="center">
  <a href="https://github.com/gpskwlkr/protondb-check/actions/workflows/rust.yml" style="text-decoration: none;">
    <img alt="build status" src="https://img.shields.io/github/actions/workflow/status/gpskwlkr/protondb-check/rust.yml?branch=main&style=for-the-badge">
  </a>
  <a href="LICENSE" style="text-decoration: none;">
    <img alt="license" src="https://img.shields.io/badge/license-MIT-blue?style=for-the-badge">
  </a>
    <a href="https://crates.io/crates/protondb-check" style="text-decoration: none;">
    <img alt="" src="https://img.shields.io/crates/v/protondb-check?style=for-the-badge">
  </a>
  
  <a href="https://crates.io/crates/protondb-check" style="text-decoration: none;">
    <img alt="" src="https://img.shields.io/crates/d/protondb-check?style=for-the-badge">
  </a>

  <a href="https://coveralls.io/github/gpskwlkr/protondb-check?branch=main" style="text-decoration: none;">
    <img alt="" src="https://img.shields.io/coverallsCoverage/github/gpskwlkr/protondb-check.svg?branch=main&style=for-the-badge">
  </a>
</div>
<br>

> <code>protondb-check</code> is currently in active development stage, there might be bugs or other problems.

## Table Of Contents

- [About](#about)
- [Available commands](#available-commands)
- [Install](#install)
- [To do](#to-do)
- [Known issues](#known-issues)

# About

`protondb-check` relies on data provided by Steam on products you own and later on lets you choose the game you want to check utilizing ProtonDB API.

# Available commands

> While `-p` or `-a` are both listed as not required, at least one should be provided.

| Command        | Description                                     | Args                | Required | Example                               |
| -------------- | ----------------------------------------------- | ------------------- | -------- | ------------------------------------- |
| protondb-check | Choose one game from all owned                  | `-p` `--profile-id` | No       | `protondb-check -p 76561198354374976` |
| protondb-check | Check particular app even if not owned in Steam | `-a` `--app-id`     | No       | `protondb-check -a 1145360`           |

# Install

## Windows / Linux

You can install `protondb-check` via

`cargo install protondb-check`

or using prebuilt binaries on the [Releases](https://github.com/gpskwlkr/protondb-check/releases) page.

## MacOS

Only `cargo install protondb-check` available for MacOS.

## Compiling from source

### Dependencies

- cargo
- rustc
- openssl
- pkg-config / pkgconf

`git clone https://github.com/gpskwlkr/protondb-check && cd protondb-check`

`cargo build --release`

`cd target/release && ./protondb-check`

# To do

- [ ] Tests coverage
- [x] Refactor code to be more safe
- [x] Refactor code to work with `clap` for better args
- [x] Receive single app-id through args
- [ ] Receive single game name through args
- [ ] Better way to handle filtering out DLCs
- [ ] More to come...

# Known issues

- [ ] DLC filtering is kinda bad, right now, if you have 0 hours on record in any steam product, it won't be included in `protondb-check` list, since Steam doesn't count hours on DLC's, soundtracks, etc.

If anything else occurs, feel free to submit to [issues](https://github.com/gpskwlkr/protondb-check/issues) page, otherwise, you can join [Discord](https://discord.gg/dR25EbTW) server.
