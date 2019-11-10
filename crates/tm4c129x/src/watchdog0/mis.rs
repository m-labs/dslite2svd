#[doc = "Reader of register MIS"]
pub type R = crate::R<u32, super::MIS>;
#[doc = "Reader of field `WDTMIS`"]
pub type WDTMIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Watchdog Masked Interrupt Status"]
    #[inline(always)]
    pub fn wdtmis(&self) -> WDTMIS_R {
        WDTMIS_R::new((self.bits & 0x01) != 0)
    }
}
