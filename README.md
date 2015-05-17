#logoboy

ROM utility for ripping and replacing the default logo scroll that displays before Nintendo Game Boy games

##Installation

```bash
make && sudo make install
```

Note: This utility depends on libpng

##Usage

```bash
Usage: ./logoboy [-g,--get] <rom.gb> <outfile.png>
                 [-s,--set] <rom.gb> <infile.png>
                 [-h,--help]
```

Note: New logos should:

* be an 8-bit PNG
* have dimensions of 48px x 8px, and
* include pixels that are only black (#000000) or white (#ffffff)

##Explanation

The opening logo scroll for every Game Boy game is actually stored on each ROM and used as a sort of check to make sure the game has booted correctly. The logo is 0x30 bytes at offset 0x104 on each ROM. Because the logo is checked upon boot up, modifying the logo on a ROM will cause the game to freeze right after the modified logo scrolls in.

![GitBoy or GameHub?](example.gif)

##Disclaimer

Please read Nintendo's [rules](http://www.nintendo.com/corp/legal.jsp) regarding ROMs and emulation. I do not provide ROMs in this project and all intellectual property belongs to Nintendo with all rights reserved.