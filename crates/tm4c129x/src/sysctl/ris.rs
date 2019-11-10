#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Reader of field `BORRIS`"]
pub type BORRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MOFRIS`"]
pub type MOFRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLLLRIS`"]
pub type PLLLRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MOSCPUPRIS`"]
pub type MOSCPUPRIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - Brown-Out Reset Raw Interrupt Status"]
    #[inline(always)]
    pub fn borris(&self) -> BORRIS_R {
        BORRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Main Oscillator Failure Raw Interrupt Status"]
    #[inline(always)]
    pub fn mofris(&self) -> MOFRIS_R {
        MOFRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL Lock Raw Interrupt Status"]
    #[inline(always)]
    pub fn plllris(&self) -> PLLLRIS_R {
        PLLLRIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MOSC Power Up Raw Interrupt Status"]
    #[inline(always)]
    pub fn moscpupris(&self) -> MOSCPUPRIS_R {
        MOSCPUPRIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
