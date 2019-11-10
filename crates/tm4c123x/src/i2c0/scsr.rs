#[doc = "Reader of register SCSR"]
pub type R = crate::R<u32, super::SCSR>;
#[doc = "Reader of field `DA`"]
pub type DA_R = crate::R<bool, bool>;
#[doc = "Reader of field `RREQ`"]
pub type RREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `TREQ`"]
pub type TREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `FBR`"]
pub type FBR_R = crate::R<bool, bool>;
#[doc = "Reader of field `OAR2SEL`"]
pub type OAR2SEL_R = crate::R<bool, bool>;
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
    #[doc = "Bit 3 - OAR2 Address Matched"]
    #[inline(always)]
    pub fn oar2sel(&self) -> OAR2SEL_R {
        OAR2SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
