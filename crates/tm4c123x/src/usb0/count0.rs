#[doc = "Reader of register COUNT0"]
pub type R = crate::R<u8, super::COUNT0>;
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - FIFO Count"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x7f) as u8)
    }
}
