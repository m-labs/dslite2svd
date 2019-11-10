#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Reader of field `WDTRIS`"]
pub type WDTRIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Watchdog Raw Interrupt Status"]
    #[inline(always)]
    pub fn wdtris(&self) -> WDTRIS_R {
        WDTRIS_R::new((self.bits & 0x01) != 0)
    }
}
