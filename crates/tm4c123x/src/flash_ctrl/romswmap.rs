#[doc = "Reader of register ROMSWMAP"]
pub type R = crate::R<u32, super::ROMSWMAP>;
#[doc = "Reader of field `SAFERTOS`"]
pub type SAFERTOS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SafeRTOS Present"]
    #[inline(always)]
    pub fn safertos(&self) -> SAFERTOS_R {
        SAFERTOS_R::new((self.bits & 0x01) != 0)
    }
}
