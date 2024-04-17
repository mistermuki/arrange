/*
    Parts of this file utilize code from and/or inspired by https://github.com/YosysHQ/icestorm.

    Copyright (C) 2012 - 2022  Claire Xenia Wolf <claire@yosyshq.com>

    Permission to use, copy, modify, and/or distribute this software for any
    purpose with or without fee is hereby granted, provided that the above
    copyright notice and this permission notice appear in all copies.

    THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
    WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
    MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
    ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
    WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
    ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
    OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
*/
pub enum FlashCommand {
    ///  Write Enable
    WE = 0x06,
    ///  Volatile SR Write Enable
    SRWE = 0x50,
    ///  Write Disable
    WD = 0x04,
    ///  Release Power-Down, returns Device ID
    RPD = 0xAB,
    ///   Read Manufacturer/Device ID
    MFGID = 0x90,
    ///  Read JEDEC ID
    JEDECID = 0x9F,
    ///  Read Unique ID
    UID = 0x4B,
    ///  Read Data
    RD = 0x03,
    ///  Fast Read
    FR = 0x0B,
    ///  Page Program
    PP = 0x02,
    ///  Sector Erase 4kb
    SE = 0x20,
    ///  Block Erase 32kb
    BE32 = 0x52,
    ///  Block Erase 64kb
    BE64 = 0xD8,
    ///  Chip Erase
    CE = 0xC7,
    ///  Read Status Register 1
    RSR1 = 0x05,
    ///  Write Status Register 1
    WSR1 = 0x01,
    ///  Read Status Register 2
    RSR2 = 0x35,
    ///  Write Status Register 2
    WSR2 = 0x31,
    ///  Read Status Register 3
    RSR3 = 0x15,
    ///  Write Status Register 3
    WSR3 = 0x11,
    ///  Read SFDP Register
    RSFDP = 0x5A,
    ///  Erase Security Register
    ESR = 0x44,
    ///  Program Security Register
    PSR = 0x42,
    ///  Read Security Register
    RSR = 0x48,
    ///  Global Block Lock
    GBL = 0x7E,
    ///  Global Block Unlock
    GBU = 0x98,
    ///  Read Block Lock
    RBL = 0x3D,
    ///  Read Sector Protection Registers (adesto)
    RPR = 0x3C,
    ///  Individual Block Lock
    IBL = 0x36,
    ///  Individual Block Unlock
    IBU = 0x39,
    ///  Erase / Program Suspend
    EPS = 0x75,
    ///  Erase / Program Resume
    EPR = 0x7A,
    ///  Power-down
    PD = 0xB9,
    ///  Enter QPI mode
    QPI = 0x38,
    ///  Enable Reset
    ERESET = 0x66,
    ///  Reset Device
    RESET = 0x99,
}
