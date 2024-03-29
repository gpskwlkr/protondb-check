<p align="center">
  <img alt="ProtonDB logo" src="./readme_items/protondb.svg" height="150" width="150" />
</p>

<h1 align="center">
ProtonDB Checker.
</h1>

> <code>protondb-check</code> is a simple CLI tool which lets you check compatibility of your Steam games with Proton.

> <code>protondb-check</code> is currently in active development stage, there might be bugs or other problems.

## Table Of Contents

- [About](#-about)
- [Available commands](#-available-commands)
- [Install](#-install)
- [To do](#-to-do)

## About

`protondb-check` relies on data provided by Steam on products you own and later on lets you choose the game you want to check utilizing ProtonDB API.

## Available commands

| Command        | Args               | Required | Example                            |
| -------------- | ------------------ | -------- | ---------------------------------- |
| protondb-check | `steam_profile_id` | Yes      | `protondb-check 76561198354374976` |

## Install

You can install `protondb-check` via

`cargo install protondb-check`

## To do

- [ ] Tests coverage
- [ ] Refactor code to be more safe
- [ ] Refactor code to work with `clap` for better args
- [ ] Receive single app-id through args
- [ ] Receive single game name through args
- [ ] Better way to handle filtering out DLCs
- [ ] More to come...
