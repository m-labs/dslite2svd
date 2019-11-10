#[doc = "Reader of register SMIS"]
pub type R = crate::R<u32, super::SMIS>;
#[doc = "Reader of field `DATAMIS`"]
pub type DATAMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `STARTMIS`"]
pub type STARTMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `STOPMIS`"]
pub type STOPMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMARXMIS`"]
pub type DMARXMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMATXMIS`"]
pub type DMATXMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXMIS`"]
pub type TXMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXMIS`"]
pub type RXMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFEMIS`"]
pub type TXFEMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFFMIS`"]
pub type RXFFMIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Data Masked Interrupt Status"]
    #[inline(always)]
    pub fn datamis(&self) -> DATAMIS_R {
        DATAMIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start Condition Masked Interrupt Status"]
    #[inline(always)]
    pub fn startmis(&self) -> STARTMIS_R {
        STARTMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stop Condition Masked Interrupt Status"]
    #[inline(always)]
    pub fn stopmis(&self) -> STOPMIS_R {
        STOPMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn dmarxmis(&self) -> DMARXMIS_R {
        DMARXMIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn dmatxmis(&self) -> DMATXMIS_R {
        DMATXMIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn txfemis(&self) -> TXFEMIS_R {
        TXFEMIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn rxffmis(&self) -> RXFFMIS_R {
        RXFFMIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
