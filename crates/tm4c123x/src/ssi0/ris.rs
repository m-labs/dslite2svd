#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Reader of field `RORRIS`"]
pub type RORRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTRIS`"]
pub type RTRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXRIS`"]
pub type RXRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXRIS`"]
pub type TXRIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SSI Receive Overrun Raw Interrupt Status"]
    #[inline(always)]
    pub fn rorris(&self) -> RORRIS_R {
        RORRIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Raw Interrupt Status"]
    #[inline(always)]
    pub fn rtris(&self) -> RTRIS_R {
        RTRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SSI Receive FIFO Raw Interrupt Status"]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Raw Interrupt Status"]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
