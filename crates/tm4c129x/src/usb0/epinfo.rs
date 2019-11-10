#[doc = "Reader of register EPINFO"]
pub type R = crate::R<u8, super::EPINFO>;
#[doc = "Reader of field `TXEP`"]
pub type TXEP_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXEP`"]
pub type RXEP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - TX Endpoints"]
    #[inline(always)]
    pub fn txep(&self) -> TXEP_R {
        TXEP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RX Endpoints"]
    #[inline(always)]
    pub fn rxep(&self) -> RXEP_R {
        RXEP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
