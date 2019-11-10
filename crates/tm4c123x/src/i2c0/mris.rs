#[doc = "Reader of register MRIS"]
pub type R = crate::R<u32, super::MRIS>;
#[doc = "Reader of field `RIS`"]
pub type RIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLKRIS`"]
pub type CLKRIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Master Raw Interrupt Status"]
    #[inline(always)]
    pub fn ris(&self) -> RIS_R {
        RIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock Timeout Raw Interrupt Status"]
    #[inline(always)]
    pub fn clkris(&self) -> CLKRIS_R {
        CLKRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
