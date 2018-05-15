#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Master Control"]
    pub ctl: CTL,
    #[doc = "0x04 - PWM Time Base Sync"]
    pub sync: SYNC,
    #[doc = "0x08 - PWM Output Enable"]
    pub enable: ENABLE,
    #[doc = "0x0c - PWM Output Inversion"]
    pub invert: INVERT,
    #[doc = "0x10 - PWM Output Fault"]
    pub fault: FAULT,
    #[doc = "0x14 - PWM Interrupt Enable"]
    pub inten: INTEN,
    #[doc = "0x18 - PWM Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x1c - PWM Interrupt Status and Clear"]
    pub isc: ISC,
    #[doc = "0x20 - PWM Status"]
    pub status: STATUS,
    #[doc = "0x24 - PWM Fault Condition Value"]
    pub faultval: FAULTVAL,
    #[doc = "0x28 - PWM Enable Update"]
    pub enupd: ENUPD,
    _reserved0: [u8; 20usize],
    #[doc = "0x40 - PWM0 Control"]
    pub _0_ctl: _0_CTL,
    #[doc = "0x44 - PWM0 Interrupt and Trigger Enable"]
    pub _0_inten: _0_INTEN,
    #[doc = "0x48 - PWM0 Raw Interrupt Status"]
    pub _0_ris: _0_RIS,
    #[doc = "0x4c - PWM0 Interrupt Status and Clear"]
    pub _0_isc: _0_ISC,
    #[doc = "0x50 - PWM0 Load"]
    pub _0_load: _0_LOAD,
    #[doc = "0x54 - PWM0 Counter"]
    pub _0_count: _0_COUNT,
    #[doc = "0x58 - PWM0 Compare A"]
    pub _0_cmpa: _0_CMPA,
    #[doc = "0x5c - PWM0 Compare B"]
    pub _0_cmpb: _0_CMPB,
    #[doc = "0x60 - PWM0 Generator A Control"]
    pub _0_gena: _0_GENA,
    #[doc = "0x64 - PWM0 Generator B Control"]
    pub _0_genb: _0_GENB,
    #[doc = "0x68 - PWM0 Dead-Band Control"]
    pub _0_dbctl: _0_DBCTL,
    #[doc = "0x6c - PWM0 Dead-Band Rising-Edge Delay"]
    pub _0_dbrise: _0_DBRISE,
    #[doc = "0x70 - PWM0 Dead-Band Falling-Edge-Delay"]
    pub _0_dbfall: _0_DBFALL,
    #[doc = "0x74 - PWM0 Fault Source 0"]
    pub _0_fltsrc0: _0_FLTSRC0,
    #[doc = "0x78 - PWM0 Fault Source 1"]
    pub _0_fltsrc1: _0_FLTSRC1,
    #[doc = "0x7c - PWM0 Minimum Fault Period"]
    pub _0_minfltper: _0_MINFLTPER,
    #[doc = "0x80 - PWM1 Control"]
    pub _1_ctl: _1_CTL,
    #[doc = "0x84 - PWM1 Interrupt and Trigger Enable"]
    pub _1_inten: _1_INTEN,
    #[doc = "0x88 - PWM1 Raw Interrupt Status"]
    pub _1_ris: _1_RIS,
    #[doc = "0x8c - PWM1 Interrupt Status and Clear"]
    pub _1_isc: _1_ISC,
    #[doc = "0x90 - PWM1 Load"]
    pub _1_load: _1_LOAD,
    #[doc = "0x94 - PWM1 Counter"]
    pub _1_count: _1_COUNT,
    #[doc = "0x98 - PWM1 Compare A"]
    pub _1_cmpa: _1_CMPA,
    #[doc = "0x9c - PWM1 Compare B"]
    pub _1_cmpb: _1_CMPB,
    #[doc = "0xa0 - PWM1 Generator A Control"]
    pub _1_gena: _1_GENA,
    #[doc = "0xa4 - PWM1 Generator B Control"]
    pub _1_genb: _1_GENB,
    #[doc = "0xa8 - PWM1 Dead-Band Control"]
    pub _1_dbctl: _1_DBCTL,
    #[doc = "0xac - PWM1 Dead-Band Rising-Edge Delay"]
    pub _1_dbrise: _1_DBRISE,
    #[doc = "0xb0 - PWM1 Dead-Band Falling-Edge-Delay"]
    pub _1_dbfall: _1_DBFALL,
    #[doc = "0xb4 - PWM1 Fault Source 0"]
    pub _1_fltsrc0: _1_FLTSRC0,
    #[doc = "0xb8 - PWM1 Fault Source 1"]
    pub _1_fltsrc1: _1_FLTSRC1,
    #[doc = "0xbc - PWM1 Minimum Fault Period"]
    pub _1_minfltper: _1_MINFLTPER,
    #[doc = "0xc0 - PWM2 Control"]
    pub _2_ctl: _2_CTL,
    #[doc = "0xc4 - PWM2 Interrupt and Trigger Enable"]
    pub _2_inten: _2_INTEN,
    #[doc = "0xc8 - PWM2 Raw Interrupt Status"]
    pub _2_ris: _2_RIS,
    #[doc = "0xcc - PWM2 Interrupt Status and Clear"]
    pub _2_isc: _2_ISC,
    #[doc = "0xd0 - PWM2 Load"]
    pub _2_load: _2_LOAD,
    #[doc = "0xd4 - PWM2 Counter"]
    pub _2_count: _2_COUNT,
    #[doc = "0xd8 - PWM2 Compare A"]
    pub _2_cmpa: _2_CMPA,
    #[doc = "0xdc - PWM2 Compare B"]
    pub _2_cmpb: _2_CMPB,
    #[doc = "0xe0 - PWM2 Generator A Control"]
    pub _2_gena: _2_GENA,
    #[doc = "0xe4 - PWM2 Generator B Control"]
    pub _2_genb: _2_GENB,
    #[doc = "0xe8 - PWM2 Dead-Band Control"]
    pub _2_dbctl: _2_DBCTL,
    #[doc = "0xec - PWM2 Dead-Band Rising-Edge Delay"]
    pub _2_dbrise: _2_DBRISE,
    #[doc = "0xf0 - PWM2 Dead-Band Falling-Edge-Delay"]
    pub _2_dbfall: _2_DBFALL,
    #[doc = "0xf4 - PWM2 Fault Source 0"]
    pub _2_fltsrc0: _2_FLTSRC0,
    #[doc = "0xf8 - PWM2 Fault Source 1"]
    pub _2_fltsrc1: _2_FLTSRC1,
    #[doc = "0xfc - PWM2 Minimum Fault Period"]
    pub _2_minfltper: _2_MINFLTPER,
    #[doc = "0x100 - PWM3 Control"]
    pub _3_ctl: _3_CTL,
    #[doc = "0x104 - PWM3 Interrupt and Trigger Enable"]
    pub _3_inten: _3_INTEN,
    #[doc = "0x108 - PWM3 Raw Interrupt Status"]
    pub _3_ris: _3_RIS,
    #[doc = "0x10c - PWM3 Interrupt Status and Clear"]
    pub _3_isc: _3_ISC,
    #[doc = "0x110 - PWM3 Load"]
    pub _3_load: _3_LOAD,
    #[doc = "0x114 - PWM3 Counter"]
    pub _3_count: _3_COUNT,
    #[doc = "0x118 - PWM3 Compare A"]
    pub _3_cmpa: _3_CMPA,
    #[doc = "0x11c - PWM3 Compare B"]
    pub _3_cmpb: _3_CMPB,
    #[doc = "0x120 - PWM3 Generator A Control"]
    pub _3_gena: _3_GENA,
    #[doc = "0x124 - PWM3 Generator B Control"]
    pub _3_genb: _3_GENB,
    #[doc = "0x128 - PWM3 Dead-Band Control"]
    pub _3_dbctl: _3_DBCTL,
    #[doc = "0x12c - PWM3 Dead-Band Rising-Edge Delay"]
    pub _3_dbrise: _3_DBRISE,
    #[doc = "0x130 - PWM3 Dead-Band Falling-Edge-Delay"]
    pub _3_dbfall: _3_DBFALL,
    #[doc = "0x134 - PWM3 Fault Source 0"]
    pub _3_fltsrc0: _3_FLTSRC0,
    #[doc = "0x138 - PWM3 Fault Source 1"]
    pub _3_fltsrc1: _3_FLTSRC1,
    #[doc = "0x13c - PWM3 Minimum Fault Period"]
    pub _3_minfltper: _3_MINFLTPER,
    _reserved1: [u8; 1728usize],
    #[doc = "0x800 - PWM0 Fault Pin Logic Sense"]
    pub _0_fltsen: _0_FLTSEN,
    #[doc = "0x804 - PWM0 Fault Status 0"]
    pub _0_fltstat0: _0_FLTSTAT0,
    #[doc = "0x808 - PWM0 Fault Status 1"]
    pub _0_fltstat1: _0_FLTSTAT1,
    _reserved2: [u8; 116usize],
    #[doc = "0x880 - PWM1 Fault Pin Logic Sense"]
    pub _1_fltsen: _1_FLTSEN,
    #[doc = "0x884 - PWM1 Fault Status 0"]
    pub _1_fltstat0: _1_FLTSTAT0,
    #[doc = "0x888 - PWM1 Fault Status 1"]
    pub _1_fltstat1: _1_FLTSTAT1,
    _reserved3: [u8; 116usize],
    #[doc = "0x900 - PWM2 Fault Pin Logic Sense"]
    pub _2_fltsen: _2_FLTSEN,
    #[doc = "0x904 - PWM2 Fault Status 0"]
    pub _2_fltstat0: _2_FLTSTAT0,
    #[doc = "0x908 - PWM2 Fault Status 1"]
    pub _2_fltstat1: _2_FLTSTAT1,
    _reserved4: [u8; 116usize],
    #[doc = "0x980 - PWM3 Fault Pin Logic Sense"]
    pub _3_fltsen: _3_FLTSEN,
    #[doc = "0x984 - PWM3 Fault Status 0"]
    pub _3_fltstat0: _3_FLTSTAT0,
    #[doc = "0x988 - PWM3 Fault Status 1"]
    pub _3_fltstat1: _3_FLTSTAT1,
    _reserved5: [u8; 1588usize],
    #[doc = "0xfc0 - PWM Peripheral Properties"]
    pub pp: PP,
    _reserved6: [u8; 4usize],
    #[doc = "0xfc8 - PWM Clock Configuration"]
    pub cc: CC,
}
#[doc = "PWM Master Control"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Master Control"]
pub mod ctl;
#[doc = "PWM Time Base Sync"]
pub struct SYNC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Time Base Sync"]
pub mod sync;
#[doc = "PWM Output Enable"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Output Enable"]
pub mod enable;
#[doc = "PWM Output Inversion"]
pub struct INVERT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Output Inversion"]
pub mod invert;
#[doc = "PWM Output Fault"]
pub struct FAULT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Output Fault"]
pub mod fault;
#[doc = "PWM Interrupt Enable"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Interrupt Enable"]
pub mod inten;
#[doc = "PWM Raw Interrupt Status"]
pub struct RIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Raw Interrupt Status"]
pub mod ris;
#[doc = "PWM Interrupt Status and Clear"]
pub struct ISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Interrupt Status and Clear"]
pub mod isc;
#[doc = "PWM Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Status"]
pub mod status;
#[doc = "PWM Fault Condition Value"]
pub struct FAULTVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Fault Condition Value"]
pub mod faultval;
#[doc = "PWM Enable Update"]
pub struct ENUPD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Enable Update"]
pub mod enupd;
#[doc = "PWM0 Control"]
pub struct _0_CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM0 Control"]
pub mod _0_ctl;
#[doc = "PWM0 Interrupt and Trigger Enable"]
pub struct _0_INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM0 Interrupt and Trigger Enable"]
pub mod _0_inten;
#[doc = "PWM0 Raw Interrupt Status"]
pub struct _0_RIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM0 Raw Interrupt Status"]
pub mod _0_ris;
#[doc = "PWM0 Interrupt Status and Clear"]
pub struct _0_ISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM0 Interrupt Status and Clear"]
pub mod _0_isc;
#[doc = "PWM0 Load"]
pub struct _0_LOAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM0 Load"]
pub mod _0_load;
#[doc = "PWM0 Counter"]
pub struct _0_COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM0 Counter"]
pub mod _0_count;
#[doc = "PWM0 Compare A"]
pub struct _0_CMPA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM0 Compare A"]
pub mod _0_cmpa;
#[doc = "PWM0 Compare B"]
pub struct _0_CMPB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM0 Compare B"]
pub mod _0_cmpb;
#[doc = "PWM0 Generator A Control"]
pub struct _0_GENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM0 Generator A Control"]
pub mod _0_gena;
#[doc = "PWM0 Generator B Control"]
pub struct _0_GENB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM0 Generator B Control"]
pub mod _0_genb;
#[doc = "PWM0 Dead-Band Control"]
pub struct _0_DBCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM0 Dead-Band Control"]
pub mod _0_dbctl;
#[doc = "PWM0 Dead-Band Rising-Edge Delay"]
pub struct _0_DBRISE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM0 Dead-Band Rising-Edge Delay"]
pub mod _0_dbrise;
#[doc = "PWM0 Dead-Band Falling-Edge-Delay"]
pub struct _0_DBFALL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM0 Dead-Band Falling-Edge-Delay"]
pub mod _0_dbfall;
#[doc = "PWM0 Fault Source 0"]
pub struct _0_FLTSRC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM0 Fault Source 0"]
pub mod _0_fltsrc0;
#[doc = "PWM0 Fault Source 1"]
pub struct _0_FLTSRC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM0 Fault Source 1"]
pub mod _0_fltsrc1;
#[doc = "PWM0 Minimum Fault Period"]
pub struct _0_MINFLTPER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM0 Minimum Fault Period"]
pub mod _0_minfltper;
#[doc = "PWM1 Control"]
pub struct _1_CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM1 Control"]
pub mod _1_ctl;
#[doc = "PWM1 Interrupt and Trigger Enable"]
pub struct _1_INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM1 Interrupt and Trigger Enable"]
pub mod _1_inten;
#[doc = "PWM1 Raw Interrupt Status"]
pub struct _1_RIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM1 Raw Interrupt Status"]
pub mod _1_ris;
#[doc = "PWM1 Interrupt Status and Clear"]
pub struct _1_ISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM1 Interrupt Status and Clear"]
pub mod _1_isc;
#[doc = "PWM1 Load"]
pub struct _1_LOAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM1 Load"]
pub mod _1_load;
#[doc = "PWM1 Counter"]
pub struct _1_COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM1 Counter"]
pub mod _1_count;
#[doc = "PWM1 Compare A"]
pub struct _1_CMPA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM1 Compare A"]
pub mod _1_cmpa;
#[doc = "PWM1 Compare B"]
pub struct _1_CMPB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM1 Compare B"]
pub mod _1_cmpb;
#[doc = "PWM1 Generator A Control"]
pub struct _1_GENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM1 Generator A Control"]
pub mod _1_gena;
#[doc = "PWM1 Generator B Control"]
pub struct _1_GENB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM1 Generator B Control"]
pub mod _1_genb;
#[doc = "PWM1 Dead-Band Control"]
pub struct _1_DBCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM1 Dead-Band Control"]
pub mod _1_dbctl;
#[doc = "PWM1 Dead-Band Rising-Edge Delay"]
pub struct _1_DBRISE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM1 Dead-Band Rising-Edge Delay"]
pub mod _1_dbrise;
#[doc = "PWM1 Dead-Band Falling-Edge-Delay"]
pub struct _1_DBFALL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM1 Dead-Band Falling-Edge-Delay"]
pub mod _1_dbfall;
#[doc = "PWM1 Fault Source 0"]
pub struct _1_FLTSRC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM1 Fault Source 0"]
pub mod _1_fltsrc0;
#[doc = "PWM1 Fault Source 1"]
pub struct _1_FLTSRC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM1 Fault Source 1"]
pub mod _1_fltsrc1;
#[doc = "PWM1 Minimum Fault Period"]
pub struct _1_MINFLTPER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM1 Minimum Fault Period"]
pub mod _1_minfltper;
#[doc = "PWM2 Control"]
pub struct _2_CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM2 Control"]
pub mod _2_ctl;
#[doc = "PWM2 Interrupt and Trigger Enable"]
pub struct _2_INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM2 Interrupt and Trigger Enable"]
pub mod _2_inten;
#[doc = "PWM2 Raw Interrupt Status"]
pub struct _2_RIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM2 Raw Interrupt Status"]
pub mod _2_ris;
#[doc = "PWM2 Interrupt Status and Clear"]
pub struct _2_ISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM2 Interrupt Status and Clear"]
pub mod _2_isc;
#[doc = "PWM2 Load"]
pub struct _2_LOAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM2 Load"]
pub mod _2_load;
#[doc = "PWM2 Counter"]
pub struct _2_COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM2 Counter"]
pub mod _2_count;
#[doc = "PWM2 Compare A"]
pub struct _2_CMPA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM2 Compare A"]
pub mod _2_cmpa;
#[doc = "PWM2 Compare B"]
pub struct _2_CMPB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM2 Compare B"]
pub mod _2_cmpb;
#[doc = "PWM2 Generator A Control"]
pub struct _2_GENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM2 Generator A Control"]
pub mod _2_gena;
#[doc = "PWM2 Generator B Control"]
pub struct _2_GENB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM2 Generator B Control"]
pub mod _2_genb;
#[doc = "PWM2 Dead-Band Control"]
pub struct _2_DBCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM2 Dead-Band Control"]
pub mod _2_dbctl;
#[doc = "PWM2 Dead-Band Rising-Edge Delay"]
pub struct _2_DBRISE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM2 Dead-Band Rising-Edge Delay"]
pub mod _2_dbrise;
#[doc = "PWM2 Dead-Band Falling-Edge-Delay"]
pub struct _2_DBFALL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM2 Dead-Band Falling-Edge-Delay"]
pub mod _2_dbfall;
#[doc = "PWM2 Fault Source 0"]
pub struct _2_FLTSRC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM2 Fault Source 0"]
pub mod _2_fltsrc0;
#[doc = "PWM2 Fault Source 1"]
pub struct _2_FLTSRC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM2 Fault Source 1"]
pub mod _2_fltsrc1;
#[doc = "PWM2 Minimum Fault Period"]
pub struct _2_MINFLTPER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM2 Minimum Fault Period"]
pub mod _2_minfltper;
#[doc = "PWM3 Control"]
pub struct _3_CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM3 Control"]
pub mod _3_ctl;
#[doc = "PWM3 Interrupt and Trigger Enable"]
pub struct _3_INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM3 Interrupt and Trigger Enable"]
pub mod _3_inten;
#[doc = "PWM3 Raw Interrupt Status"]
pub struct _3_RIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM3 Raw Interrupt Status"]
pub mod _3_ris;
#[doc = "PWM3 Interrupt Status and Clear"]
pub struct _3_ISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM3 Interrupt Status and Clear"]
pub mod _3_isc;
#[doc = "PWM3 Load"]
pub struct _3_LOAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM3 Load"]
pub mod _3_load;
#[doc = "PWM3 Counter"]
pub struct _3_COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM3 Counter"]
pub mod _3_count;
#[doc = "PWM3 Compare A"]
pub struct _3_CMPA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM3 Compare A"]
pub mod _3_cmpa;
#[doc = "PWM3 Compare B"]
pub struct _3_CMPB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM3 Compare B"]
pub mod _3_cmpb;
#[doc = "PWM3 Generator A Control"]
pub struct _3_GENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM3 Generator A Control"]
pub mod _3_gena;
#[doc = "PWM3 Generator B Control"]
pub struct _3_GENB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM3 Generator B Control"]
pub mod _3_genb;
#[doc = "PWM3 Dead-Band Control"]
pub struct _3_DBCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM3 Dead-Band Control"]
pub mod _3_dbctl;
#[doc = "PWM3 Dead-Band Rising-Edge Delay"]
pub struct _3_DBRISE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM3 Dead-Band Rising-Edge Delay"]
pub mod _3_dbrise;
#[doc = "PWM3 Dead-Band Falling-Edge-Delay"]
pub struct _3_DBFALL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM3 Dead-Band Falling-Edge-Delay"]
pub mod _3_dbfall;
#[doc = "PWM3 Fault Source 0"]
pub struct _3_FLTSRC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM3 Fault Source 0"]
pub mod _3_fltsrc0;
#[doc = "PWM3 Fault Source 1"]
pub struct _3_FLTSRC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM3 Fault Source 1"]
pub mod _3_fltsrc1;
#[doc = "PWM3 Minimum Fault Period"]
pub struct _3_MINFLTPER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM3 Minimum Fault Period"]
pub mod _3_minfltper;
#[doc = "PWM0 Fault Pin Logic Sense"]
pub struct _0_FLTSEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM0 Fault Pin Logic Sense"]
pub mod _0_fltsen;
#[doc = "PWM0 Fault Status 0"]
pub struct _0_FLTSTAT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM0 Fault Status 0"]
pub mod _0_fltstat0;
#[doc = "PWM0 Fault Status 1"]
pub struct _0_FLTSTAT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM0 Fault Status 1"]
pub mod _0_fltstat1;
#[doc = "PWM1 Fault Pin Logic Sense"]
pub struct _1_FLTSEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM1 Fault Pin Logic Sense"]
pub mod _1_fltsen;
#[doc = "PWM1 Fault Status 0"]
pub struct _1_FLTSTAT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM1 Fault Status 0"]
pub mod _1_fltstat0;
#[doc = "PWM1 Fault Status 1"]
pub struct _1_FLTSTAT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM1 Fault Status 1"]
pub mod _1_fltstat1;
#[doc = "PWM2 Fault Pin Logic Sense"]
pub struct _2_FLTSEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM2 Fault Pin Logic Sense"]
pub mod _2_fltsen;
#[doc = "PWM2 Fault Status 0"]
pub struct _2_FLTSTAT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM2 Fault Status 0"]
pub mod _2_fltstat0;
#[doc = "PWM2 Fault Status 1"]
pub struct _2_FLTSTAT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM2 Fault Status 1"]
pub mod _2_fltstat1;
#[doc = "PWM3 Fault Pin Logic Sense"]
pub struct _3_FLTSEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM3 Fault Pin Logic Sense"]
pub mod _3_fltsen;
#[doc = "PWM3 Fault Status 0"]
pub struct _3_FLTSTAT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM3 Fault Status 0"]
pub mod _3_fltstat0;
#[doc = "PWM3 Fault Status 1"]
pub struct _3_FLTSTAT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM3 Fault Status 1"]
pub mod _3_fltstat1;
#[doc = "PWM Peripheral Properties"]
pub struct PP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Peripheral Properties"]
pub mod pp;
#[doc = "PWM Clock Configuration"]
pub struct CC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Clock Configuration"]
pub mod cc;
