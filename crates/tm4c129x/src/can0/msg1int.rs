#[doc = "Reader of register MSG1INT"]
pub type R = crate::R<u32, super::MSG1INT>;
#[doc = "Reader of field `INTPND`"]
pub type INTPND_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Interrupt Pending Bits"]
    #[inline(always)]
    pub fn intpnd(&self) -> INTPND_R {
        INTPND_R::new((self.bits & 0xffff) as u16)
    }
}
