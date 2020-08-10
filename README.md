# logoboy

ROM utility for ripping/replacing the Nintendo Game Boy logo scroll

## Build

```bash
cargo build
```

## Install

```bash
cargo install --path .
```

## Usage

```
logoboy 2.0.0
Kevin Selwyn <kevinselwyn@gmail.com>
ROM utility for ripping/replacing the Nintendo Game Boy logo scroll

USAGE:
    logoboy [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    get     Get logo
    set     Set logo
    help    Prints this message or the help of the given subcommand(s)
```

Get:

```
logoboy-get
Get logo

USAGE:
    logoboy get --output <output> --rom <rom>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -r, --rom <rom>          Input ROM
    -o, --output <output>    Output PNG
```

Set:

```
logoboy-set
Set logo

USAGE:
    logoboy set --output <output> --png <png> --rom <rom>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -r, --rom <rom>          Input ROM
    -p, --png <png>          Input PNG
    -o, --output <output>    Output ROM
```

## Explanation

The opening logo scroll for every Game Boy game is actually stored on each ROM and used as a sort of check to make sure the game has booted correctly. The logo is 0x30 bytes at offset 0x104 on each ROM. Because the logo is checked upon boot up, modifying the logo on a ROM will cause the game to freeze right after the modified logo scrolls in.

![GitBoy or GameHub?](example.gif)

## Disclaimer

Please read Nintendo's [rules](http://www.nintendo.com/corp/legal.jsp) regarding ROMs and emulation. I do not provide ROMs in this project and all intellectual property belongs to Nintendo with all rights reserved.
