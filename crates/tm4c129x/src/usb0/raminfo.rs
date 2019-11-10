#[doc = "Reader of register RAMINFO"]
pub type R = crate::R<u8, super::RAMINFO>;
#[doc = "Reader of field `RAMBITS`"]
pub type RAMBITS_R = crate::R<u8, u8>;
#[doc = "Reader of field `DMACHAN`"]
pub type DMACHAN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - RAM Address Bus Width"]
    #[inline(always)]
    pub fn rambits(&self) -> RAMBITS_R {
        RAMBITS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DMA Channels"]
    #[inline(always)]
    pub fn dmachan(&self) -> DMACHAN_R {
        DMACHAN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
