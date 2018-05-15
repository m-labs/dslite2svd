#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EEPROM Size Information"]
    pub eesize: EESIZE,
    #[doc = "0x04 - EEPROM Current Block"]
    pub eeblock: EEBLOCK,
    #[doc = "0x08 - EEPROM Current Offset"]
    pub eeoffset: EEOFFSET,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - EEPROM Read-Write"]
    pub eerdwr: EERDWR,
    #[doc = "0x14 - EEPROM Read-Write with Increment"]
    pub eerdwrinc: EERDWRINC,
    #[doc = "0x18 - EEPROM Done Status"]
    pub eedone: EEDONE,
    #[doc = "0x1c - EEPROM Support Control and Status"]
    pub eesupp: EESUPP,
    #[doc = "0x20 - EEPROM Unlock"]
    pub eeunlock: EEUNLOCK,
    _reserved1: [u8; 12usize],
    #[doc = "0x30 - EEPROM Protection"]
    pub eeprot: EEPROT,
    #[doc = "0x34 - EEPROM Password"]
    pub eepass0: EEPASS0,
    #[doc = "0x38 - EEPROM Password"]
    pub eepass1: EEPASS1,
    #[doc = "0x3c - EEPROM Password"]
    pub eepass2: EEPASS2,
    #[doc = "0x40 - EEPROM Interrupt"]
    pub eeint: EEINT,
    _reserved2: [u8; 12usize],
    #[doc = "0x50 - EEPROM Block Hide 0"]
    pub eehide0: EEHIDE0,
    #[doc = "0x54 - EEPROM Block Hide 1"]
    pub eehide1: EEHIDE1,
    #[doc = "0x58 - EEPROM Block Hide 2"]
    pub eehide2: EEHIDE2,
    _reserved3: [u8; 36usize],
    #[doc = "0x80 - EEPROM Debug Mass Erase"]
    pub eedbgme: EEDBGME,
    _reserved4: [u8; 3900usize],
    #[doc = "0xfc0 - EEPROM Peripheral Properties"]
    pub pp: PP,
}
#[doc = "EEPROM Size Information"]
pub struct EESIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Size Information"]
pub mod eesize;
#[doc = "EEPROM Current Block"]
pub struct EEBLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Current Block"]
pub mod eeblock;
#[doc = "EEPROM Current Offset"]
pub struct EEOFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Current Offset"]
pub mod eeoffset;
#[doc = "EEPROM Read-Write"]
pub struct EERDWR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Read-Write"]
pub mod eerdwr;
#[doc = "EEPROM Read-Write with Increment"]
pub struct EERDWRINC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Read-Write with Increment"]
pub mod eerdwrinc;
#[doc = "EEPROM Done Status"]
pub struct EEDONE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Done Status"]
pub mod eedone;
#[doc = "EEPROM Support Control and Status"]
pub struct EESUPP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Support Control and Status"]
pub mod eesupp;
#[doc = "EEPROM Unlock"]
pub struct EEUNLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Unlock"]
pub mod eeunlock;
#[doc = "EEPROM Protection"]
pub struct EEPROT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Protection"]
pub mod eeprot;
#[doc = "EEPROM Password"]
pub struct EEPASS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Password"]
pub mod eepass0;
#[doc = "EEPROM Password"]
pub struct EEPASS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Password"]
pub mod eepass1;
#[doc = "EEPROM Password"]
pub struct EEPASS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Password"]
pub mod eepass2;
#[doc = "EEPROM Interrupt"]
pub struct EEINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Interrupt"]
pub mod eeint;
#[doc = "EEPROM Block Hide 0"]
pub struct EEHIDE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Block Hide 0"]
pub mod eehide0;
#[doc = "EEPROM Block Hide 1"]
pub struct EEHIDE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Block Hide 1"]
pub mod eehide1;
#[doc = "EEPROM Block Hide 2"]
pub struct EEHIDE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Block Hide 2"]
pub mod eehide2;
#[doc = "EEPROM Debug Mass Erase"]
pub struct EEDBGME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Debug Mass Erase"]
pub mod eedbgme;
#[doc = "EEPROM Peripheral Properties"]
pub struct PP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Peripheral Properties"]
pub mod pp;
