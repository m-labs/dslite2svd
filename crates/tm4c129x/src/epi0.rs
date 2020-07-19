#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EPI Configuration"]
    pub cfg: CFG,
    #[doc = "0x04 - EPI Main Baud Rate"]
    pub baud: BAUD,
    #[doc = "0x08 - EPI Main Baud Rate"]
    pub baud2: BAUD2,
    _reserved3: [u8; 4usize],
    _reserved_3_gpcfg: [u8; 4usize],
    _reserved_4_hb8cfg2: [u8; 4usize],
    _reserved5: [u8; 4usize],
    #[doc = "0x1c - EPI Address Map"]
    pub addrmap: ADDRMAP,
    #[doc = "0x20 - EPI Read Size 0"]
    pub rsize0: RSIZE0,
    #[doc = "0x24 - EPI Read Address 0"]
    pub raddr0: RADDR0,
    #[doc = "0x28 - EPI Non-Blocking Read Data 0"]
    pub rpstd0: RPSTD0,
    _reserved9: [u8; 4usize],
    #[doc = "0x30 - EPI Read Size 1"]
    pub rsize1: RSIZE1,
    #[doc = "0x34 - EPI Read Address 1"]
    pub raddr1: RADDR1,
    #[doc = "0x38 - EPI Non-Blocking Read Data 1"]
    pub rpstd1: RPSTD1,
    _reserved12: [u8; 36usize],
    #[doc = "0x60 - EPI Status"]
    pub stat: STAT,
    _reserved13: [u8; 8usize],
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
    _reserved22: [u8; 368usize],
    #[doc = "0x200 - EPI FIFO Level Selects"]
    pub fifolvl: FIFOLVL,
    #[doc = "0x204 - EPI Write FIFO Count"]
    pub wfifocnt: WFIFOCNT,
    #[doc = "0x208 - EPI DMA Transmit Count"]
    pub dmatxcnt: DMATXCNT,
    _reserved25: [u8; 4usize],
    #[doc = "0x210 - EPI Interrupt Mask"]
    pub im: IM,
    #[doc = "0x214 - EPI Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x218 - EPI Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x21c - EPI Error and Interrupt Status and Clear"]
    pub eisc: EISC,
    _reserved29: [u8; 232usize],
    _reserved_29_hb8cfg3: [u8; 4usize],
    _reserved_30_hb8cfg4: [u8; 4usize],
    _reserved_31_hb8time: [u8; 4usize],
    _reserved_32_hb8time2: [u8; 4usize],
    _reserved_33_hb8time3: [u8; 4usize],
    _reserved_34_hb8time4: [u8; 4usize],
    _reserved35: [u8; 64usize],
    #[doc = "0x360 - EPI Host-Bus PSRAM"]
    pub hbpsram: HBPSRAM,
}
impl RegisterBlock {
    #[doc = "0x10 - EPI Host-Bus 8 Configuration"]
    #[inline(always)]
    pub fn hb8cfg(&self) -> &HB8CFG {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const HB8CFG) }
    }
    #[doc = "0x10 - EPI Host-Bus 8 Configuration"]
    #[inline(always)]
    pub fn hb8cfg_mut(&self) -> &mut HB8CFG {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut HB8CFG) }
    }
    #[doc = "0x10 - EPI SDRAM Configuration"]
    #[inline(always)]
    pub fn sdramcfg(&self) -> &SDRAMCFG {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const SDRAMCFG) }
    }
    #[doc = "0x10 - EPI SDRAM Configuration"]
    #[inline(always)]
    pub fn sdramcfg_mut(&self) -> &mut SDRAMCFG {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut SDRAMCFG) }
    }
    #[doc = "0x10 - EPI General-Purpose Configuration"]
    #[inline(always)]
    pub fn gpcfg(&self) -> &GPCFG {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const GPCFG) }
    }
    #[doc = "0x10 - EPI General-Purpose Configuration"]
    #[inline(always)]
    pub fn gpcfg_mut(&self) -> &mut GPCFG {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut GPCFG) }
    }
    #[doc = "0x10 - EPI Host-Bus 16 Configuration"]
    #[inline(always)]
    pub fn hb16cfg(&self) -> &HB16CFG {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const HB16CFG) }
    }
    #[doc = "0x10 - EPI Host-Bus 16 Configuration"]
    #[inline(always)]
    pub fn hb16cfg_mut(&self) -> &mut HB16CFG {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut HB16CFG) }
    }
    #[doc = "0x14 - EPI Host-Bus 16 Configuration 2"]
    #[inline(always)]
    pub fn hb16cfg2(&self) -> &HB16CFG2 {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const HB16CFG2) }
    }
    #[doc = "0x14 - EPI Host-Bus 16 Configuration 2"]
    #[inline(always)]
    pub fn hb16cfg2_mut(&self) -> &mut HB16CFG2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut HB16CFG2) }
    }
    #[doc = "0x14 - EPI Host-Bus 8 Configuration 2"]
    #[inline(always)]
    pub fn hb8cfg2(&self) -> &HB8CFG2 {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const HB8CFG2) }
    }
    #[doc = "0x14 - EPI Host-Bus 8 Configuration 2"]
    #[inline(always)]
    pub fn hb8cfg2_mut(&self) -> &mut HB8CFG2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut HB8CFG2) }
    }
    #[doc = "0x308 - EPI Host-Bus 16 Configuration 3"]
    #[inline(always)]
    pub fn hb16cfg3(&self) -> &HB16CFG3 {
        unsafe { &*(((self as *const Self) as *const u8).add(776usize) as *const HB16CFG3) }
    }
    #[doc = "0x308 - EPI Host-Bus 16 Configuration 3"]
    #[inline(always)]
    pub fn hb16cfg3_mut(&self) -> &mut HB16CFG3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(776usize) as *mut HB16CFG3) }
    }
    #[doc = "0x308 - EPI Host-Bus 8 Configuration 3"]
    #[inline(always)]
    pub fn hb8cfg3(&self) -> &HB8CFG3 {
        unsafe { &*(((self as *const Self) as *const u8).add(776usize) as *const HB8CFG3) }
    }
    #[doc = "0x308 - EPI Host-Bus 8 Configuration 3"]
    #[inline(always)]
    pub fn hb8cfg3_mut(&self) -> &mut HB8CFG3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(776usize) as *mut HB8CFG3) }
    }
    #[doc = "0x30c - EPI Host-Bus 8 Configuration 4"]
    #[inline(always)]
    pub fn hb8cfg4(&self) -> &HB8CFG4 {
        unsafe { &*(((self as *const Self) as *const u8).add(780usize) as *const HB8CFG4) }
    }
    #[doc = "0x30c - EPI Host-Bus 8 Configuration 4"]
    #[inline(always)]
    pub fn hb8cfg4_mut(&self) -> &mut HB8CFG4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(780usize) as *mut HB8CFG4) }
    }
    #[doc = "0x30c - EPI Host-Bus 16 Configuration 4"]
    #[inline(always)]
    pub fn hb16cfg4(&self) -> &HB16CFG4 {
        unsafe { &*(((self as *const Self) as *const u8).add(780usize) as *const HB16CFG4) }
    }
    #[doc = "0x30c - EPI Host-Bus 16 Configuration 4"]
    #[inline(always)]
    pub fn hb16cfg4_mut(&self) -> &mut HB16CFG4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(780usize) as *mut HB16CFG4) }
    }
    #[doc = "0x310 - EPI Host-Bus 16 Timing Extension"]
    #[inline(always)]
    pub fn hb16time(&self) -> &HB16TIME {
        unsafe { &*(((self as *const Self) as *const u8).add(784usize) as *const HB16TIME) }
    }
    #[doc = "0x310 - EPI Host-Bus 16 Timing Extension"]
    #[inline(always)]
    pub fn hb16time_mut(&self) -> &mut HB16TIME {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(784usize) as *mut HB16TIME) }
    }
    #[doc = "0x310 - EPI Host-Bus 8 Timing Extension"]
    #[inline(always)]
    pub fn hb8time(&self) -> &HB8TIME {
        unsafe { &*(((self as *const Self) as *const u8).add(784usize) as *const HB8TIME) }
    }
    #[doc = "0x310 - EPI Host-Bus 8 Timing Extension"]
    #[inline(always)]
    pub fn hb8time_mut(&self) -> &mut HB8TIME {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(784usize) as *mut HB8TIME) }
    }
    #[doc = "0x314 - EPI Host-Bus 16 Timing Extension"]
    #[inline(always)]
    pub fn hb16time2(&self) -> &HB16TIME2 {
        unsafe { &*(((self as *const Self) as *const u8).add(788usize) as *const HB16TIME2) }
    }
    #[doc = "0x314 - EPI Host-Bus 16 Timing Extension"]
    #[inline(always)]
    pub fn hb16time2_mut(&self) -> &mut HB16TIME2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(788usize) as *mut HB16TIME2) }
    }
    #[doc = "0x314 - EPI Host-Bus 8 Timing Extension"]
    #[inline(always)]
    pub fn hb8time2(&self) -> &HB8TIME2 {
        unsafe { &*(((self as *const Self) as *const u8).add(788usize) as *const HB8TIME2) }
    }
    #[doc = "0x314 - EPI Host-Bus 8 Timing Extension"]
    #[inline(always)]
    pub fn hb8time2_mut(&self) -> &mut HB8TIME2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(788usize) as *mut HB8TIME2) }
    }
    #[doc = "0x318 - EPI Host-Bus 8 Timing Extension"]
    #[inline(always)]
    pub fn hb8time3(&self) -> &HB8TIME3 {
        unsafe { &*(((self as *const Self) as *const u8).add(792usize) as *const HB8TIME3) }
    }
    #[doc = "0x318 - EPI Host-Bus 8 Timing Extension"]
    #[inline(always)]
    pub fn hb8time3_mut(&self) -> &mut HB8TIME3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(792usize) as *mut HB8TIME3) }
    }
    #[doc = "0x318 - EPI Host-Bus 16 Timing Extension"]
    #[inline(always)]
    pub fn hb16time3(&self) -> &HB16TIME3 {
        unsafe { &*(((self as *const Self) as *const u8).add(792usize) as *const HB16TIME3) }
    }
    #[doc = "0x318 - EPI Host-Bus 16 Timing Extension"]
    #[inline(always)]
    pub fn hb16time3_mut(&self) -> &mut HB16TIME3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(792usize) as *mut HB16TIME3) }
    }
    #[doc = "0x31c - EPI Host-Bus 16 Timing Extension"]
    #[inline(always)]
    pub fn hb16time4(&self) -> &HB16TIME4 {
        unsafe { &*(((self as *const Self) as *const u8).add(796usize) as *const HB16TIME4) }
    }
    #[doc = "0x31c - EPI Host-Bus 16 Timing Extension"]
    #[inline(always)]
    pub fn hb16time4_mut(&self) -> &mut HB16TIME4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(796usize) as *mut HB16TIME4) }
    }
    #[doc = "0x31c - EPI Host-Bus 8 Timing Extension"]
    #[inline(always)]
    pub fn hb8time4(&self) -> &HB8TIME4 {
        unsafe { &*(((self as *const Self) as *const u8).add(796usize) as *const HB8TIME4) }
    }
    #[doc = "0x31c - EPI Host-Bus 8 Timing Extension"]
    #[inline(always)]
    pub fn hb8time4_mut(&self) -> &mut HB8TIME4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(796usize) as *mut HB8TIME4) }
    }
}
#[doc = "EPI Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "EPI Configuration"]
pub mod cfg;
#[doc = "EPI Main Baud Rate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud](baud) module"]
pub type BAUD = crate::Reg<u32, _BAUD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAUD;
#[doc = "`read()` method returns [baud::R](baud::R) reader structure"]
impl crate::Readable for BAUD {}
#[doc = "`write(|w| ..)` method takes [baud::W](baud::W) writer structure"]
impl crate::Writable for BAUD {}
#[doc = "EPI Main Baud Rate"]
pub mod baud;
#[doc = "EPI Main Baud Rate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud2](baud2) module"]
pub type BAUD2 = crate::Reg<u32, _BAUD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAUD2;
#[doc = "`read()` method returns [baud2::R](baud2::R) reader structure"]
impl crate::Readable for BAUD2 {}
#[doc = "`write(|w| ..)` method takes [baud2::W](baud2::W) writer structure"]
impl crate::Writable for BAUD2 {}
#[doc = "EPI Main Baud Rate"]
pub mod baud2;
#[doc = "EPI Host-Bus 16 Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hb16cfg](hb16cfg) module"]
pub type HB16CFG = crate::Reg<u32, _HB16CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HB16CFG;
#[doc = "`read()` method returns [hb16cfg::R](hb16cfg::R) reader structure"]
impl crate::Readable for HB16CFG {}
#[doc = "`write(|w| ..)` method takes [hb16cfg::W](hb16cfg::W) writer structure"]
impl crate::Writable for HB16CFG {}
#[doc = "EPI Host-Bus 16 Configuration"]
pub mod hb16cfg;
#[doc = "EPI General-Purpose Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpcfg](gpcfg) module"]
pub type GPCFG = crate::Reg<u32, _GPCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPCFG;
#[doc = "`read()` method returns [gpcfg::R](gpcfg::R) reader structure"]
impl crate::Readable for GPCFG {}
#[doc = "`write(|w| ..)` method takes [gpcfg::W](gpcfg::W) writer structure"]
impl crate::Writable for GPCFG {}
#[doc = "EPI General-Purpose Configuration"]
pub mod gpcfg;
#[doc = "EPI SDRAM Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramcfg](sdramcfg) module"]
pub type SDRAMCFG = crate::Reg<u32, _SDRAMCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRAMCFG;
#[doc = "`read()` method returns [sdramcfg::R](sdramcfg::R) reader structure"]
impl crate::Readable for SDRAMCFG {}
#[doc = "`write(|w| ..)` method takes [sdramcfg::W](sdramcfg::W) writer structure"]
impl crate::Writable for SDRAMCFG {}
#[doc = "EPI SDRAM Configuration"]
pub mod sdramcfg;
#[doc = "EPI Host-Bus 8 Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hb8cfg](hb8cfg) module"]
pub type HB8CFG = crate::Reg<u32, _HB8CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HB8CFG;
#[doc = "`read()` method returns [hb8cfg::R](hb8cfg::R) reader structure"]
impl crate::Readable for HB8CFG {}
#[doc = "`write(|w| ..)` method takes [hb8cfg::W](hb8cfg::W) writer structure"]
impl crate::Writable for HB8CFG {}
#[doc = "EPI Host-Bus 8 Configuration"]
pub mod hb8cfg;
#[doc = "EPI Host-Bus 8 Configuration 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hb8cfg2](hb8cfg2) module"]
pub type HB8CFG2 = crate::Reg<u32, _HB8CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HB8CFG2;
#[doc = "`read()` method returns [hb8cfg2::R](hb8cfg2::R) reader structure"]
impl crate::Readable for HB8CFG2 {}
#[doc = "`write(|w| ..)` method takes [hb8cfg2::W](hb8cfg2::W) writer structure"]
impl crate::Writable for HB8CFG2 {}
#[doc = "EPI Host-Bus 8 Configuration 2"]
pub mod hb8cfg2;
#[doc = "EPI Host-Bus 16 Configuration 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hb16cfg2](hb16cfg2) module"]
pub type HB16CFG2 = crate::Reg<u32, _HB16CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HB16CFG2;
#[doc = "`read()` method returns [hb16cfg2::R](hb16cfg2::R) reader structure"]
impl crate::Readable for HB16CFG2 {}
#[doc = "`write(|w| ..)` method takes [hb16cfg2::W](hb16cfg2::W) writer structure"]
impl crate::Writable for HB16CFG2 {}
#[doc = "EPI Host-Bus 16 Configuration 2"]
pub mod hb16cfg2;
#[doc = "EPI Address Map\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addrmap](addrmap) module"]
pub type ADDRMAP = crate::Reg<u32, _ADDRMAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDRMAP;
#[doc = "`read()` method returns [addrmap::R](addrmap::R) reader structure"]
impl crate::Readable for ADDRMAP {}
#[doc = "`write(|w| ..)` method takes [addrmap::W](addrmap::W) writer structure"]
impl crate::Writable for ADDRMAP {}
#[doc = "EPI Address Map"]
pub mod addrmap;
#[doc = "EPI Read Size 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsize0](rsize0) module"]
pub type RSIZE0 = crate::Reg<u32, _RSIZE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSIZE0;
#[doc = "`read()` method returns [rsize0::R](rsize0::R) reader structure"]
impl crate::Readable for RSIZE0 {}
#[doc = "`write(|w| ..)` method takes [rsize0::W](rsize0::W) writer structure"]
impl crate::Writable for RSIZE0 {}
#[doc = "EPI Read Size 0"]
pub mod rsize0;
#[doc = "EPI Read Address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [raddr0](raddr0) module"]
pub type RADDR0 = crate::Reg<u32, _RADDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RADDR0;
#[doc = "`read()` method returns [raddr0::R](raddr0::R) reader structure"]
impl crate::Readable for RADDR0 {}
#[doc = "`write(|w| ..)` method takes [raddr0::W](raddr0::W) writer structure"]
impl crate::Writable for RADDR0 {}
#[doc = "EPI Read Address 0"]
pub mod raddr0;
#[doc = "EPI Non-Blocking Read Data 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpstd0](rpstd0) module"]
pub type RPSTD0 = crate::Reg<u32, _RPSTD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPSTD0;
#[doc = "`read()` method returns [rpstd0::R](rpstd0::R) reader structure"]
impl crate::Readable for RPSTD0 {}
#[doc = "`write(|w| ..)` method takes [rpstd0::W](rpstd0::W) writer structure"]
impl crate::Writable for RPSTD0 {}
#[doc = "EPI Non-Blocking Read Data 0"]
pub mod rpstd0;
#[doc = "EPI Read Size 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsize1](rsize1) module"]
pub type RSIZE1 = crate::Reg<u32, _RSIZE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSIZE1;
#[doc = "`read()` method returns [rsize1::R](rsize1::R) reader structure"]
impl crate::Readable for RSIZE1 {}
#[doc = "`write(|w| ..)` method takes [rsize1::W](rsize1::W) writer structure"]
impl crate::Writable for RSIZE1 {}
#[doc = "EPI Read Size 1"]
pub mod rsize1;
#[doc = "EPI Read Address 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [raddr1](raddr1) module"]
pub type RADDR1 = crate::Reg<u32, _RADDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RADDR1;
#[doc = "`read()` method returns [raddr1::R](raddr1::R) reader structure"]
impl crate::Readable for RADDR1 {}
#[doc = "`write(|w| ..)` method takes [raddr1::W](raddr1::W) writer structure"]
impl crate::Writable for RADDR1 {}
#[doc = "EPI Read Address 1"]
pub mod raddr1;
#[doc = "EPI Non-Blocking Read Data 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpstd1](rpstd1) module"]
pub type RPSTD1 = crate::Reg<u32, _RPSTD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPSTD1;
#[doc = "`read()` method returns [rpstd1::R](rpstd1::R) reader structure"]
impl crate::Readable for RPSTD1 {}
#[doc = "`write(|w| ..)` method takes [rpstd1::W](rpstd1::W) writer structure"]
impl crate::Writable for RPSTD1 {}
#[doc = "EPI Non-Blocking Read Data 1"]
pub mod rpstd1;
#[doc = "EPI Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "EPI Status"]
pub mod stat;
#[doc = "EPI Read FIFO Count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifocnt](rfifocnt) module"]
pub type RFIFOCNT = crate::Reg<u32, _RFIFOCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFIFOCNT;
#[doc = "`read()` method returns [rfifocnt::R](rfifocnt::R) reader structure"]
impl crate::Readable for RFIFOCNT {}
#[doc = "EPI Read FIFO Count"]
pub mod rfifocnt;
#[doc = "EPI Read FIFO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readfifo0](readfifo0) module"]
pub type READFIFO0 = crate::Reg<u32, _READFIFO0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _READFIFO0;
#[doc = "`read()` method returns [readfifo0::R](readfifo0::R) reader structure"]
impl crate::Readable for READFIFO0 {}
#[doc = "EPI Read FIFO"]
pub mod readfifo0;
#[doc = "EPI Read FIFO Alias 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readfifo1](readfifo1) module"]
pub type READFIFO1 = crate::Reg<u32, _READFIFO1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _READFIFO1;
#[doc = "`read()` method returns [readfifo1::R](readfifo1::R) reader structure"]
impl crate::Readable for READFIFO1 {}
#[doc = "EPI Read FIFO Alias 1"]
pub mod readfifo1;
#[doc = "EPI Read FIFO Alias 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readfifo2](readfifo2) module"]
pub type READFIFO2 = crate::Reg<u32, _READFIFO2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _READFIFO2;
#[doc = "`read()` method returns [readfifo2::R](readfifo2::R) reader structure"]
impl crate::Readable for READFIFO2 {}
#[doc = "EPI Read FIFO Alias 2"]
pub mod readfifo2;
#[doc = "EPI Read FIFO Alias 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readfifo3](readfifo3) module"]
pub type READFIFO3 = crate::Reg<u32, _READFIFO3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _READFIFO3;
#[doc = "`read()` method returns [readfifo3::R](readfifo3::R) reader structure"]
impl crate::Readable for READFIFO3 {}
#[doc = "EPI Read FIFO Alias 3"]
pub mod readfifo3;
#[doc = "EPI Read FIFO Alias 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readfifo4](readfifo4) module"]
pub type READFIFO4 = crate::Reg<u32, _READFIFO4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _READFIFO4;
#[doc = "`read()` method returns [readfifo4::R](readfifo4::R) reader structure"]
impl crate::Readable for READFIFO4 {}
#[doc = "EPI Read FIFO Alias 4"]
pub mod readfifo4;
#[doc = "EPI Read FIFO Alias 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readfifo5](readfifo5) module"]
pub type READFIFO5 = crate::Reg<u32, _READFIFO5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _READFIFO5;
#[doc = "`read()` method returns [readfifo5::R](readfifo5::R) reader structure"]
impl crate::Readable for READFIFO5 {}
#[doc = "EPI Read FIFO Alias 5"]
pub mod readfifo5;
#[doc = "EPI Read FIFO Alias 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readfifo6](readfifo6) module"]
pub type READFIFO6 = crate::Reg<u32, _READFIFO6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _READFIFO6;
#[doc = "`read()` method returns [readfifo6::R](readfifo6::R) reader structure"]
impl crate::Readable for READFIFO6 {}
#[doc = "EPI Read FIFO Alias 6"]
pub mod readfifo6;
#[doc = "EPI Read FIFO Alias 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readfifo7](readfifo7) module"]
pub type READFIFO7 = crate::Reg<u32, _READFIFO7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _READFIFO7;
#[doc = "`read()` method returns [readfifo7::R](readfifo7::R) reader structure"]
impl crate::Readable for READFIFO7 {}
#[doc = "EPI Read FIFO Alias 7"]
pub mod readfifo7;
#[doc = "EPI FIFO Level Selects\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifolvl](fifolvl) module"]
pub type FIFOLVL = crate::Reg<u32, _FIFOLVL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOLVL;
#[doc = "`read()` method returns [fifolvl::R](fifolvl::R) reader structure"]
impl crate::Readable for FIFOLVL {}
#[doc = "`write(|w| ..)` method takes [fifolvl::W](fifolvl::W) writer structure"]
impl crate::Writable for FIFOLVL {}
#[doc = "EPI FIFO Level Selects"]
pub mod fifolvl;
#[doc = "EPI Write FIFO Count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wfifocnt](wfifocnt) module"]
pub type WFIFOCNT = crate::Reg<u32, _WFIFOCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WFIFOCNT;
#[doc = "`read()` method returns [wfifocnt::R](wfifocnt::R) reader structure"]
impl crate::Readable for WFIFOCNT {}
#[doc = "EPI Write FIFO Count"]
pub mod wfifocnt;
#[doc = "EPI DMA Transmit Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmatxcnt](dmatxcnt) module"]
pub type DMATXCNT = crate::Reg<u32, _DMATXCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMATXCNT;
#[doc = "`read()` method returns [dmatxcnt::R](dmatxcnt::R) reader structure"]
impl crate::Readable for DMATXCNT {}
#[doc = "`write(|w| ..)` method takes [dmatxcnt::W](dmatxcnt::W) writer structure"]
impl crate::Writable for DMATXCNT {}
#[doc = "EPI DMA Transmit Count"]
pub mod dmatxcnt;
#[doc = "EPI Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [im](im) module"]
pub type IM = crate::Reg<u32, _IM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IM;
#[doc = "`read()` method returns [im::R](im::R) reader structure"]
impl crate::Readable for IM {}
#[doc = "`write(|w| ..)` method takes [im::W](im::W) writer structure"]
impl crate::Writable for IM {}
#[doc = "EPI Interrupt Mask"]
pub mod im;
#[doc = "EPI Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "EPI Raw Interrupt Status"]
pub mod ris;
#[doc = "EPI Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](mis) module"]
pub type MIS = crate::Reg<u32, _MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIS;
#[doc = "`read()` method returns [mis::R](mis::R) reader structure"]
impl crate::Readable for MIS {}
#[doc = "EPI Masked Interrupt Status"]
pub mod mis;
#[doc = "EPI Error and Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eisc](eisc) module"]
pub type EISC = crate::Reg<u32, _EISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EISC;
#[doc = "`read()` method returns [eisc::R](eisc::R) reader structure"]
impl crate::Readable for EISC {}
#[doc = "`write(|w| ..)` method takes [eisc::W](eisc::W) writer structure"]
impl crate::Writable for EISC {}
#[doc = "EPI Error and Interrupt Status and Clear"]
pub mod eisc;
#[doc = "EPI Host-Bus 8 Configuration 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hb8cfg3](hb8cfg3) module"]
pub type HB8CFG3 = crate::Reg<u32, _HB8CFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HB8CFG3;
#[doc = "`read()` method returns [hb8cfg3::R](hb8cfg3::R) reader structure"]
impl crate::Readable for HB8CFG3 {}
#[doc = "`write(|w| ..)` method takes [hb8cfg3::W](hb8cfg3::W) writer structure"]
impl crate::Writable for HB8CFG3 {}
#[doc = "EPI Host-Bus 8 Configuration 3"]
pub mod hb8cfg3;
#[doc = "EPI Host-Bus 16 Configuration 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hb16cfg3](hb16cfg3) module"]
pub type HB16CFG3 = crate::Reg<u32, _HB16CFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HB16CFG3;
#[doc = "`read()` method returns [hb16cfg3::R](hb16cfg3::R) reader structure"]
impl crate::Readable for HB16CFG3 {}
#[doc = "`write(|w| ..)` method takes [hb16cfg3::W](hb16cfg3::W) writer structure"]
impl crate::Writable for HB16CFG3 {}
#[doc = "EPI Host-Bus 16 Configuration 3"]
pub mod hb16cfg3;
#[doc = "EPI Host-Bus 16 Configuration 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hb16cfg4](hb16cfg4) module"]
pub type HB16CFG4 = crate::Reg<u32, _HB16CFG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HB16CFG4;
#[doc = "`read()` method returns [hb16cfg4::R](hb16cfg4::R) reader structure"]
impl crate::Readable for HB16CFG4 {}
#[doc = "`write(|w| ..)` method takes [hb16cfg4::W](hb16cfg4::W) writer structure"]
impl crate::Writable for HB16CFG4 {}
#[doc = "EPI Host-Bus 16 Configuration 4"]
pub mod hb16cfg4;
#[doc = "EPI Host-Bus 8 Configuration 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hb8cfg4](hb8cfg4) module"]
pub type HB8CFG4 = crate::Reg<u32, _HB8CFG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HB8CFG4;
#[doc = "`read()` method returns [hb8cfg4::R](hb8cfg4::R) reader structure"]
impl crate::Readable for HB8CFG4 {}
#[doc = "`write(|w| ..)` method takes [hb8cfg4::W](hb8cfg4::W) writer structure"]
impl crate::Writable for HB8CFG4 {}
#[doc = "EPI Host-Bus 8 Configuration 4"]
pub mod hb8cfg4;
#[doc = "EPI Host-Bus 8 Timing Extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hb8time](hb8time) module"]
pub type HB8TIME = crate::Reg<u32, _HB8TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HB8TIME;
#[doc = "`read()` method returns [hb8time::R](hb8time::R) reader structure"]
impl crate::Readable for HB8TIME {}
#[doc = "`write(|w| ..)` method takes [hb8time::W](hb8time::W) writer structure"]
impl crate::Writable for HB8TIME {}
#[doc = "EPI Host-Bus 8 Timing Extension"]
pub mod hb8time;
#[doc = "EPI Host-Bus 16 Timing Extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hb16time](hb16time) module"]
pub type HB16TIME = crate::Reg<u32, _HB16TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HB16TIME;
#[doc = "`read()` method returns [hb16time::R](hb16time::R) reader structure"]
impl crate::Readable for HB16TIME {}
#[doc = "`write(|w| ..)` method takes [hb16time::W](hb16time::W) writer structure"]
impl crate::Writable for HB16TIME {}
#[doc = "EPI Host-Bus 16 Timing Extension"]
pub mod hb16time;
#[doc = "EPI Host-Bus 8 Timing Extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hb8time2](hb8time2) module"]
pub type HB8TIME2 = crate::Reg<u32, _HB8TIME2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HB8TIME2;
#[doc = "`read()` method returns [hb8time2::R](hb8time2::R) reader structure"]
impl crate::Readable for HB8TIME2 {}
#[doc = "`write(|w| ..)` method takes [hb8time2::W](hb8time2::W) writer structure"]
impl crate::Writable for HB8TIME2 {}
#[doc = "EPI Host-Bus 8 Timing Extension"]
pub mod hb8time2;
#[doc = "EPI Host-Bus 16 Timing Extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hb16time2](hb16time2) module"]
pub type HB16TIME2 = crate::Reg<u32, _HB16TIME2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HB16TIME2;
#[doc = "`read()` method returns [hb16time2::R](hb16time2::R) reader structure"]
impl crate::Readable for HB16TIME2 {}
#[doc = "`write(|w| ..)` method takes [hb16time2::W](hb16time2::W) writer structure"]
impl crate::Writable for HB16TIME2 {}
#[doc = "EPI Host-Bus 16 Timing Extension"]
pub mod hb16time2;
#[doc = "EPI Host-Bus 16 Timing Extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hb16time3](hb16time3) module"]
pub type HB16TIME3 = crate::Reg<u32, _HB16TIME3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HB16TIME3;
#[doc = "`read()` method returns [hb16time3::R](hb16time3::R) reader structure"]
impl crate::Readable for HB16TIME3 {}
#[doc = "`write(|w| ..)` method takes [hb16time3::W](hb16time3::W) writer structure"]
impl crate::Writable for HB16TIME3 {}
#[doc = "EPI Host-Bus 16 Timing Extension"]
pub mod hb16time3;
#[doc = "EPI Host-Bus 8 Timing Extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hb8time3](hb8time3) module"]
pub type HB8TIME3 = crate::Reg<u32, _HB8TIME3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HB8TIME3;
#[doc = "`read()` method returns [hb8time3::R](hb8time3::R) reader structure"]
impl crate::Readable for HB8TIME3 {}
#[doc = "`write(|w| ..)` method takes [hb8time3::W](hb8time3::W) writer structure"]
impl crate::Writable for HB8TIME3 {}
#[doc = "EPI Host-Bus 8 Timing Extension"]
pub mod hb8time3;
#[doc = "EPI Host-Bus 8 Timing Extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hb8time4](hb8time4) module"]
pub type HB8TIME4 = crate::Reg<u32, _HB8TIME4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HB8TIME4;
#[doc = "`read()` method returns [hb8time4::R](hb8time4::R) reader structure"]
impl crate::Readable for HB8TIME4 {}
#[doc = "`write(|w| ..)` method takes [hb8time4::W](hb8time4::W) writer structure"]
impl crate::Writable for HB8TIME4 {}
#[doc = "EPI Host-Bus 8 Timing Extension"]
pub mod hb8time4;
#[doc = "EPI Host-Bus 16 Timing Extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hb16time4](hb16time4) module"]
pub type HB16TIME4 = crate::Reg<u32, _HB16TIME4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HB16TIME4;
#[doc = "`read()` method returns [hb16time4::R](hb16time4::R) reader structure"]
impl crate::Readable for HB16TIME4 {}
#[doc = "`write(|w| ..)` method takes [hb16time4::W](hb16time4::W) writer structure"]
impl crate::Writable for HB16TIME4 {}
#[doc = "EPI Host-Bus 16 Timing Extension"]
pub mod hb16time4;
#[doc = "EPI Host-Bus PSRAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbpsram](hbpsram) module"]
pub type HBPSRAM = crate::Reg<u32, _HBPSRAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HBPSRAM;
#[doc = "`read()` method returns [hbpsram::R](hbpsram::R) reader structure"]
impl crate::Readable for HBPSRAM {}
#[doc = "`write(|w| ..)` method takes [hbpsram::W](hbpsram::W) writer structure"]
impl crate::Writable for HBPSRAM {}
#[doc = "EPI Host-Bus PSRAM"]
pub mod hbpsram;
