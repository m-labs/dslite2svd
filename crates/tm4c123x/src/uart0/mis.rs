#[doc = "Reader of register MIS"]
pub type R = crate::R<u32, super::MIS>;
#[doc = "Reader of field `CTSMIS`"]
pub type CTSMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXMIS`"]
pub type RXMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXMIS`"]
pub type TXMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTMIS`"]
pub type RTMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `FEMIS`"]
pub type FEMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEMIS`"]
pub type PEMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `BEMIS`"]
pub type BEMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `OEMIS`"]
pub type OEMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `_9BITMIS`"]
pub type _9BITMIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - UART Clear to Send Modem Masked Interrupt Status"]
    #[inline(always)]
    pub fn ctsmis(&self) -> CTSMIS_R {
        CTSMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UART Receive Masked Interrupt Status"]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UART Transmit Masked Interrupt Status"]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UART Receive Time-Out Masked Interrupt Status"]
    #[inline(always)]
    pub fn rtmis(&self) -> RTMIS_R {
        RTMIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UART Framing Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn femis(&self) -> FEMIS_R {
        FEMIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UART Parity Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn pemis(&self) -> PEMIS_R {
        PEMIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UART Break Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn bemis(&self) -> BEMIS_R {
        BEMIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UART Overrun Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn oemis(&self) -> OEMIS_R {
        OEMIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 9-Bit Mode Masked Interrupt Status"]
    #[inline(always)]
    pub fn _9bitmis(&self) -> _9BITMIS_R {
        _9BITMIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
