#[doc = "Reader of register MRIS"]
pub type R = crate::R<u32, super::MRIS>;
#[doc = "Reader of field `RIS`"]
pub type RIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLKRIS`"]
pub type CLKRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMARXRIS`"]
pub type DMARXRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMATXRIS`"]
pub type DMATXRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `NACKRIS`"]
pub type NACKRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `STARTRIS`"]
pub type STARTRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `STOPRIS`"]
pub type STOPRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `ARBLOSTRIS`"]
pub type ARBLOSTRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXRIS`"]
pub type TXRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXRIS`"]
pub type RXRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFERIS`"]
pub type TXFERIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFFRIS`"]
pub type RXFFRIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Master Raw Interrupt Status"]
    #[inline(always)]
    pub fn ris(&self) -> RIS_R {
        RIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock Timeout Raw Interrupt Status"]
    #[inline(always)]
    pub fn clkris(&self) -> CLKRIS_R {
        CLKRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn dmarxris(&self) -> DMARXRIS_R {
        DMARXRIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn dmatxris(&self) -> DMATXRIS_R {
        DMATXRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Address/Data NACK Raw Interrupt Status"]
    #[inline(always)]
    pub fn nackris(&self) -> NACKRIS_R {
        NACKRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - START Detection Raw Interrupt Status"]
    #[inline(always)]
    pub fn startris(&self) -> STARTRIS_R {
        STARTRIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - STOP Detection Raw Interrupt Status"]
    #[inline(always)]
    pub fn stopris(&self) -> STOPRIS_R {
        STOPRIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Arbitration Lost Raw Interrupt Status"]
    #[inline(always)]
    pub fn arblostris(&self) -> ARBLOSTRIS_R {
        ARBLOSTRIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit Request Raw Interrupt Status"]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive FIFO Request Raw Interrupt Status"]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit FIFO Empty Raw Interrupt Status"]
    #[inline(always)]
    pub fn txferis(&self) -> TXFERIS_R {
        TXFERIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Receive FIFO Full Raw Interrupt Status"]
    #[inline(always)]
    pub fn rxffris(&self) -> RXFFRIS_R {
        RXFFRIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
