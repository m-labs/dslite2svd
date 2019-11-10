#[doc = "Reader of register MIS"]
pub type R = crate::R<u32, super::MIS>;
#[doc = "Reader of field `RORMIS`"]
pub type RORMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTMIS`"]
pub type RTMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXMIS`"]
pub type RXMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXMIS`"]
pub type TXMIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SSI Receive Overrun Masked Interrupt Status"]
    #[inline(always)]
    pub fn rormis(&self) -> RORMIS_R {
        RORMIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Masked Interrupt Status"]
    #[inline(always)]
    pub fn rtmis(&self) -> RTMIS_R {
        RTMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SSI Receive FIFO Masked Interrupt Status"]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Masked Interrupt Status"]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
