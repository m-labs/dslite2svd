#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Reader of field `RIRIS`"]
pub type RIRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTSRIS`"]
pub type CTSRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCDRIS`"]
pub type DCDRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DSRRIS`"]
pub type DSRRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXRIS`"]
pub type RXRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXRIS`"]
pub type TXRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTRIS`"]
pub type RTRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `FERIS`"]
pub type FERIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PERIS`"]
pub type PERIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `BERIS`"]
pub type BERIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `OERIS`"]
pub type OERIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOTRIS`"]
pub type EOTRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `_9BITRIS`"]
pub type _9BITRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMARXRIS`"]
pub type DMARXRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMATXRIS`"]
pub type DMATXRIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - UART Ring Indicator Modem Raw Interrupt Status"]
    #[inline(always)]
    pub fn riris(&self) -> RIRIS_R {
        RIRIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UART Clear to Send Modem Raw Interrupt Status"]
    #[inline(always)]
    pub fn ctsris(&self) -> CTSRIS_R {
        CTSRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - UART Data Carrier Detect Modem Raw Interrupt Status"]
    #[inline(always)]
    pub fn dcdris(&self) -> DCDRIS_R {
        DCDRIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - UART Data Set Ready Modem Raw Interrupt Status"]
    #[inline(always)]
    pub fn dsrris(&self) -> DSRRIS_R {
        DSRRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UART Receive Raw Interrupt Status"]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UART Transmit Raw Interrupt Status"]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UART Receive Time-Out Raw Interrupt Status"]
    #[inline(always)]
    pub fn rtris(&self) -> RTRIS_R {
        RTRIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UART Framing Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn feris(&self) -> FERIS_R {
        FERIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UART Parity Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn peris(&self) -> PERIS_R {
        PERIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UART Break Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn beris(&self) -> BERIS_R {
        BERIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UART Overrun Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn oeris(&self) -> OERIS_R {
        OERIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - End of Transmission Raw Interrupt Status"]
    #[inline(always)]
    pub fn eotris(&self) -> EOTRIS_R {
        EOTRIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 9-Bit Mode Raw Interrupt Status"]
    #[inline(always)]
    pub fn _9bitris(&self) -> _9BITRIS_R {
        _9BITRIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Receive DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn dmarxris(&self) -> DMARXRIS_R {
        DMARXRIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Transmit DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn dmatxris(&self) -> DMATXRIS_R {
        DMATXRIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
