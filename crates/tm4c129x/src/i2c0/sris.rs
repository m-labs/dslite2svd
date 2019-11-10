#[doc = "Reader of register SRIS"]
pub type R = crate::R<u32, super::SRIS>;
#[doc = "Reader of field `DATARIS`"]
pub type DATARIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `STARTRIS`"]
pub type STARTRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `STOPRIS`"]
pub type STOPRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMARXRIS`"]
pub type DMARXRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMATXRIS`"]
pub type DMATXRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXRIS`"]
pub type TXRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXRIS`"]
pub type RXRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFERIS`"]
pub type TXFERIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFFRIS`"]
pub type RXFFRIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Data Raw Interrupt Status"]
    #[inline(always)]
    pub fn dataris(&self) -> DATARIS_R {
        DATARIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start Condition Raw Interrupt Status"]
    #[inline(always)]
    pub fn startris(&self) -> STARTRIS_R {
        STARTRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stop Condition Raw Interrupt Status"]
    #[inline(always)]
    pub fn stopris(&self) -> STOPRIS_R {
        STOPRIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn dmarxris(&self) -> DMARXRIS_R {
        DMARXRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn dmatxris(&self) -> DMATXRIS_R {
        DMATXRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Request Raw Interrupt Status"]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO Request Raw Interrupt Status"]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Empty Raw Interrupt Status"]
    #[inline(always)]
    pub fn txferis(&self) -> TXFERIS_R {
        TXFERIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive FIFO Full Raw Interrupt Status"]
    #[inline(always)]
    pub fn rxffris(&self) -> RXFFRIS_R {
        RXFFRIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
