#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EPI Configuration"]
    pub cfg: CFG,
    #[doc = "0x04 - EPI Main Baud Rate"]
    pub baud: BAUD,
    #[doc = "0x08 - EPI Main Baud Rate"]
    pub baud2: BAUD2,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - EPI Host-Bus 16 Configuration"]
    pub hb16cfg: HB16CFG,
    #[doc = "0x14 - EPI Host-Bus 8 Configuration 2"]
    pub hb8cfg2: HB8CFG2,
    _reserved1: [u8; 4usize],
    #[doc = "0x1c - EPI Address Map"]
    pub addrmap: ADDRMAP,
    #[doc = "0x20 - EPI Read Size 0"]
    pub rsize0: RSIZE0,
    #[doc = "0x24 - EPI Read Address 0"]
    pub raddr0: RADDR0,
    #[doc = "0x28 - EPI Non-Blocking Read Data 0"]
    pub rpstd0: RPSTD0,
    _reserved2: [u8; 4usize],
    #[doc = "0x30 - EPI Read Size 1"]
    pub rsize1: RSIZE1,
    #[doc = "0x34 - EPI Read Address 1"]
    pub raddr1: RADDR1,
    #[doc = "0x38 - EPI Non-Blocking Read Data 1"]
    pub rpstd1: RPSTD1,
    _reserved3: [u8; 36usize],
    #[doc = "0x60 - EPI Status"]
    pub stat: STAT,
    _reserved4: [u8; 8usize],
    #[doc = "0x6c - EPI Read FIFO Count"]
    pub rfifocnt: RFIFOCNT,
    #[doc = "0x70 - EPI Read FIFO"]
    pub readfifo0: READFIFO0,
    #[doc = "0x74 - EPI Read FIFO Alias 1"]
    pub readfifo1: READFIFO1,
    #[doc = "0x78 - EPI Read FIFO Alias 2"]
    pub readfifo2: READFIFO2,
    #[doc = "0x7c - EPI Read FIFO Alias 3"]
    pub readfifo3: READFIFO3,
    #[doc = "0x80 - EPI Read FIFO Alias 4"]
    pub readfifo4: READFIFO4,
    #[doc = "0x84 - EPI Read FIFO Alias 5"]
    pub readfifo5: READFIFO5,
    #[doc = "0x88 - EPI Read FIFO Alias 6"]
    pub readfifo6: READFIFO6,
    #[doc = "0x8c - EPI Read FIFO Alias 7"]
    pub readfifo7: READFIFO7,
    _reserved5: [u8; 368usize],
    #[doc = "0x200 - EPI FIFO Level Selects"]
    pub fifolvl: FIFOLVL,
    #[doc = "0x204 - EPI Write FIFO Count"]
    pub wfifocnt: WFIFOCNT,
    #[doc = "0x208 - EPI DMA Transmit Count"]
    pub dmatxcnt: DMATXCNT,
    _reserved6: [u8; 4usize],
    #[doc = "0x210 - EPI Interrupt Mask"]
    pub im: IM,
    #[doc = "0x214 - EPI Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x218 - EPI Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x21c - EPI Error and Interrupt Status and Clear"]
    pub eisc: EISC,
    _reserved7: [u8; 232usize],
    #[doc = "0x308 - EPI Host-Bus 8 Configuration 3"]
    pub hb8cfg3: HB8CFG3,
    #[doc = "0x30c - EPI Host-Bus 16 Configuration 4"]
    pub hb16cfg4: HB16CFG4,
    #[doc = "0x310 - EPI Host-Bus 8 Timing Extension"]
    pub hb8time: HB8TIME,
    #[doc = "0x314 - EPI Host-Bus 8 Timing Extension"]
    pub hb8time2: HB8TIME2,
    #[doc = "0x318 - EPI Host-Bus 16 Timing Extension"]
    pub hb16time3: HB16TIME3,
    #[doc = "0x31c - EPI Host-Bus 8 Timing Extension"]
    pub hb8time4: HB8TIME4,
    _reserved8: [u8; 64usize],
    #[doc = "0x360 - EPI Host-Bus PSRAM"]
    pub hbpsram: HBPSRAM,
}
#[doc = "EPI Configuration"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Configuration"]
pub mod cfg;
#[doc = "EPI Main Baud Rate"]
pub struct BAUD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Main Baud Rate"]
pub mod baud;
#[doc = "EPI Main Baud Rate"]
pub struct BAUD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Main Baud Rate"]
pub mod baud2;
#[doc = "EPI Host-Bus 16 Configuration"]
pub struct HB16CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Host-Bus 16 Configuration"]
pub mod hb16cfg;
#[doc = "EPI General-Purpose Configuration"]
pub struct GPCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI General-Purpose Configuration"]
pub mod gpcfg;
#[doc = "EPI SDRAM Configuration"]
pub struct SDRAMCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI SDRAM Configuration"]
pub mod sdramcfg;
#[doc = "EPI Host-Bus 8 Configuration"]
pub struct HB8CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Host-Bus 8 Configuration"]
pub mod hb8cfg;
#[doc = "EPI Host-Bus 8 Configuration 2"]
pub struct HB8CFG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Host-Bus 8 Configuration 2"]
pub mod hb8cfg2;
#[doc = "EPI Host-Bus 16 Configuration 2"]
pub struct HB16CFG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Host-Bus 16 Configuration 2"]
pub mod hb16cfg2;
#[doc = "EPI Address Map"]
pub struct ADDRMAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Address Map"]
pub mod addrmap;
#[doc = "EPI Read Size 0"]
pub struct RSIZE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Read Size 0"]
pub mod rsize0;
#[doc = "EPI Read Address 0"]
pub struct RADDR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Read Address 0"]
pub mod raddr0;
#[doc = "EPI Non-Blocking Read Data 0"]
pub struct RPSTD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Non-Blocking Read Data 0"]
pub mod rpstd0;
#[doc = "EPI Read Size 1"]
pub struct RSIZE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Read Size 1"]
pub mod rsize1;
#[doc = "EPI Read Address 1"]
pub struct RADDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Read Address 1"]
pub mod raddr1;
#[doc = "EPI Non-Blocking Read Data 1"]
pub struct RPSTD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Non-Blocking Read Data 1"]
pub mod rpstd1;
#[doc = "EPI Status"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Status"]
pub mod stat;
#[doc = "EPI Read FIFO Count"]
pub struct RFIFOCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Read FIFO Count"]
pub mod rfifocnt;
#[doc = "EPI Read FIFO"]
pub struct READFIFO0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Read FIFO"]
pub mod readfifo0;
#[doc = "EPI Read FIFO Alias 1"]
pub struct READFIFO1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Read FIFO Alias 1"]
pub mod readfifo1;
#[doc = "EPI Read FIFO Alias 2"]
pub struct READFIFO2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Read FIFO Alias 2"]
pub mod readfifo2;
#[doc = "EPI Read FIFO Alias 3"]
pub struct READFIFO3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Read FIFO Alias 3"]
pub mod readfifo3;
#[doc = "EPI Read FIFO Alias 4"]
pub struct READFIFO4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Read FIFO Alias 4"]
pub mod readfifo4;
#[doc = "EPI Read FIFO Alias 5"]
pub struct READFIFO5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Read FIFO Alias 5"]
pub mod readfifo5;
#[doc = "EPI Read FIFO Alias 6"]
pub struct READFIFO6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Read FIFO Alias 6"]
pub mod readfifo6;
#[doc = "EPI Read FIFO Alias 7"]
pub struct READFIFO7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Read FIFO Alias 7"]
pub mod readfifo7;
#[doc = "EPI FIFO Level Selects"]
pub struct FIFOLVL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI FIFO Level Selects"]
pub mod fifolvl;
#[doc = "EPI Write FIFO Count"]
pub struct WFIFOCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Write FIFO Count"]
pub mod wfifocnt;
#[doc = "EPI DMA Transmit Count"]
pub struct DMATXCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI DMA Transmit Count"]
pub mod dmatxcnt;
#[doc = "EPI Interrupt Mask"]
pub struct IM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Interrupt Mask"]
pub mod im;
#[doc = "EPI Raw Interrupt Status"]
pub struct RIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Raw Interrupt Status"]
pub mod ris;
#[doc = "EPI Masked Interrupt Status"]
pub struct MIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Masked Interrupt Status"]
pub mod mis;
#[doc = "EPI Error and Interrupt Status and Clear"]
pub struct EISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Error and Interrupt Status and Clear"]
pub mod eisc;
#[doc = "EPI Host-Bus 8 Configuration 3"]
pub struct HB8CFG3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Host-Bus 8 Configuration 3"]
pub mod hb8cfg3;
#[doc = "EPI Host-Bus 16 Configuration 3"]
pub struct HB16CFG3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Host-Bus 16 Configuration 3"]
pub mod hb16cfg3;
#[doc = "EPI Host-Bus 16 Configuration 4"]
pub struct HB16CFG4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Host-Bus 16 Configuration 4"]
pub mod hb16cfg4;
#[doc = "EPI Host-Bus 8 Configuration 4"]
pub struct HB8CFG4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Host-Bus 8 Configuration 4"]
pub mod hb8cfg4;
#[doc = "EPI Host-Bus 8 Timing Extension"]
pub struct HB8TIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Host-Bus 8 Timing Extension"]
pub mod hb8time;
#[doc = "EPI Host-Bus 16 Timing Extension"]
pub struct HB16TIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Host-Bus 16 Timing Extension"]
pub mod hb16time;
#[doc = "EPI Host-Bus 8 Timing Extension"]
pub struct HB8TIME2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Host-Bus 8 Timing Extension"]
pub mod hb8time2;
#[doc = "EPI Host-Bus 16 Timing Extension"]
pub struct HB16TIME2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Host-Bus 16 Timing Extension"]
pub mod hb16time2;
#[doc = "EPI Host-Bus 16 Timing Extension"]
pub struct HB16TIME3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Host-Bus 16 Timing Extension"]
pub mod hb16time3;
#[doc = "EPI Host-Bus 8 Timing Extension"]
pub struct HB8TIME3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Host-Bus 8 Timing Extension"]
pub mod hb8time3;
#[doc = "EPI Host-Bus 8 Timing Extension"]
pub struct HB8TIME4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Host-Bus 8 Timing Extension"]
pub mod hb8time4;
#[doc = "EPI Host-Bus 16 Timing Extension"]
pub struct HB16TIME4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Host-Bus 16 Timing Extension"]
pub mod hb16time4;
#[doc = "EPI Host-Bus PSRAM"]
pub struct HBPSRAM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Host-Bus PSRAM"]
pub mod hbpsram;
