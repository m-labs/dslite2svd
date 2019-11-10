#[doc = "Reader of register MMIS"]
pub type R = crate::R<u32, super::MMIS>;
#[doc = "Reader of field `MIS`"]
pub type MIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLKMIS`"]
pub type CLKMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMARXMIS`"]
pub type DMARXMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMATXMIS`"]
pub type DMATXMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `NACKMIS`"]
pub type NACKMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `STARTMIS`"]
pub type STARTMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `STOPMIS`"]
pub type STOPMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `ARBLOSTMIS`"]
pub type ARBLOSTMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXMIS`"]
pub type TXMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXMIS`"]
pub type RXMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFEMIS`"]
pub type TXFEMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFFMIS`"]
pub type RXFFMIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Masked Interrupt Status"]
    #[inline(always)]
    pub fn mis(&self) -> MIS_R {
        MIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock Timeout Masked Interrupt Status"]
    #[inline(always)]
    pub fn clkmis(&self) -> CLKMIS_R {
        CLKMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive DMA Interrupt Status"]
    #[inline(always)]
    pub fn dmarxmis(&self) -> DMARXMIS_R {
        DMARXMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit DMA Interrupt Status"]
    #[inline(always)]
    pub fn dmatxmis(&self) -> DMATXMIS_R {
        DMATXMIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Address/Data NACK Interrupt Mask"]
    #[inline(always)]
    pub fn nackmis(&self) -> NACKMIS_R {
        NACKMIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - START Detection Interrupt Mask"]
    #[inline(always)]
    pub fn startmis(&self) -> STARTMIS_R {
        STARTMIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - STOP Detection Interrupt Mask"]
    #[inline(always)]
    pub fn stopmis(&self) -> STOPMIS_R {
        STOPMIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Arbitration Lost Interrupt Mask"]
    #[inline(always)]
    pub fn arblostmis(&self) -> ARBLOSTMIS_R {
        ARBLOSTMIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit Request Interrupt Mask"]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn txfemis(&self) -> TXFEMIS_R {
        TXFEMIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn rxffmis(&self) -> RXFFMIS_R {
        RXFFMIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
