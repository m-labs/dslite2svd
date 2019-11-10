#[doc = "Reader of register RFIFOCNT"]
pub type R = crate::R<u32, super::RFIFOCNT>;
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - FIFO Count"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x0f) as u8)
    }
}
