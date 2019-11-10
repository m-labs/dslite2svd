#[doc = "Reader of register FR"]
pub type R = crate::R<u32, super::FR>;
#[doc = "Reader of field `CTS`"]
pub type CTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DSR`"]
pub type DSR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCD`"]
pub type DCD_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFE`"]
pub type RXFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFF`"]
pub type TXFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFF`"]
pub type RXFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFE`"]
pub type TXFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RI`"]
pub type RI_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Clear To Send"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data Set Ready"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Data Carrier Detect"]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - UART Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UART Receive FIFO Empty"]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UART Transmit FIFO Full"]
    #[inline(always)]
    pub fn txff(&self) -> TXFF_R {
        TXFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UART Receive FIFO Full"]
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UART Transmit FIFO Empty"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Ring Indicator"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
