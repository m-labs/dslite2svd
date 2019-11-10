#[doc = "Reader of register RXCOUNT2"]
pub type R = crate::R<u16, super::RXCOUNT2>;
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:12 - Receive Packet Count"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x1fff) as u16)
    }
}
