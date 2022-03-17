# Asterisk

Asterisk is command manager for command line.

![movie](https://user-images.githubusercontent.com/10832834/158797649-6045a65c-5ffe-465c-baaf-06db52558e2c.gif)

## Features

- add/remove/edit new functions in the shell
- add/remove/edit new functions as alias to existing commands
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

## Usage

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

if you want to add a subcommands to existing commands:

```bash
# add `clear` sub-command to git
$ ast add --wrap git --name clear --command ...

# and reload asterisk
$ exec -l $SHELL

# execute with sub-command
$ git clear
```

## License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your opinion.
