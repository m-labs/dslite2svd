#[doc = "Reader of register SCSR"]
pub type R = crate::R<u32, super::SCSR>;
#[doc = "Reader of field `DA`"]
pub type DA_R = crate::R<bool, bool>;
#[doc = "Reader of field `RREQ`"]
pub type RREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFIFO`"]
pub type TXFIFO_R = crate::R<bool, bool>;
#[doc = "Reader of field `TREQ`"]
pub type TREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `FBR`"]
pub type FBR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFIFO`"]
pub type RXFIFO_R = crate::R<bool, bool>;
#[doc = "Reader of field `OAR2SEL`"]
pub type OAR2SEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `QCMDST`"]
pub type QCMDST_R = crate::R<bool, bool>;
#[doc = "Reader of field `QCMDRW`"]
pub type QCMDRW_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTDMATX`"]
pub type ACTDMATX_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTDMARX`"]
pub type ACTDMARX_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Device Active"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 0 - Receive Request"]
    #[inline(always)]
    pub fn rreq(&self) -> RREQ_R {
        RREQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Enable"]
    #[inline(always)]
    pub fn txfifo(&self) -> TXFIFO_R {
        TXFIFO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Request"]
    #[inline(always)]
    pub fn treq(&self) -> TREQ_R {
        TREQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - First Byte Received"]
    #[inline(always)]
    pub fn fbr(&self) -> FBR_R {
        FBR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX FIFO Enable"]
    #[inline(always)]
    pub fn rxfifo(&self) -> RXFIFO_R {
        RXFIFO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OAR2 Address Matched"]
    #[inline(always)]
    pub fn oar2sel(&self) -> OAR2SEL_R {
        OAR2SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Quick Command Status"]
    #[inline(always)]
    pub fn qcmdst(&self) -> QCMDST_R {
        QCMDST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Quick Command Read / Write"]
    #[inline(always)]
    pub fn qcmdrw(&self) -> QCMDRW_R {
        QCMDRW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DMA TX Active Status"]
    #[inline(always)]
    pub fn actdmatx(&self) -> ACTDMATX_R {
        ACTDMATX_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DMA RX Active Status"]
    #[inline(always)]
    pub fn actdmarx(&self) -> ACTDMARX_R {
        ACTDMARX_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
