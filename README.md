# Asterisk

Asterisk is command manager for command line.

![movie](https://user-images.githubusercontent.com/10832834/158797649-6045a65c-5ffe-465c-baaf-06db52558e2c.gif)

## Features

- add/remove/edit new functions in the shell
- add/remove/edit new functions as alias to existing commands
- add/remove/edit new functions within namespace
- conditional functions
- pre/post hooks in commands

## Installation

### From Source

```bash
$ git clone https://github.com/mika-f/asterisk.git
$ cd /path/to/asterisk
$ cargo build --release
```

### From Package Manager

```bash
$ cargo install asterisk
```

## Basic Usage

initialize or reload asterisk:

```bash
# bash
$ eval $(ast init bash)

# zsh
$ eval $(ast init zsh)

# fish
$ ast init fish | source

# PowerShell
$ ast init pwsh | Invoke-Expression
```

add a new function to asterisk:

```bash
# interactive
$ ast add

# command line
$ ast add \
  --command "rg" \
  --name search \
  --description "search expression with ripgrep
```

remove a function from asterisk:

```bash
$ ast remove search
```

execute command with asterisk:

```bash
# pass-through
$ ast exec search

# direct
$ search
```


## Advanced Usage

### Command Alias

if you want to add a subcommands to existing commands:

```bash
# add `clear` sub-command to git
$ ast add --wrap git --name clear --command ...

# and reload asterisk
$ exec -l $SHELL

# execute with sub-command
$ git clear
```


### Namespace

if you want to add a subcommands into a new/existing namespace:

```bash
# add `jpy` sub-command into `ether` namespace
$ ast add --wrap ether --name jpy --command ...

# and reload asterisk
$ exec -l $SHELL

# execute with sub-command
$ ether jpy
```

### Command-Line Named Arguments

if you want to provide some arguments to functions:

```bash
# add `currency` arg into sub-command of `coin` namespace
$ ast add --wrap coin --name pair \
  --command 'curl -H "X-CMC_PRO_API_KEY: XXX" -s "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?slug=${currency}&convert=${fiat}" | jq "[.data][][].quote.${fiat}.price"'

# reload asterisk
$ exec -l $SHELL

# execute with args
$ coin pair --fiat=JPY --currency=ethereum
```

## License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your opinion.
