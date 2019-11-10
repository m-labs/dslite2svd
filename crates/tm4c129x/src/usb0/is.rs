#[doc = "Reader of register IS"]
pub type R = crate::R<u8, super::IS>;
#[doc = "Reader of field `SUSPEND`"]
pub type SUSPEND_R = crate::R<bool, bool>;
#[doc = "Reader of field `RESUME`"]
pub type RESUME_R = crate::R<bool, bool>;
#[doc = "Reader of field `BABBLE`"]
pub type BABBLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RESET`"]
pub type RESET_R = crate::R<bool, bool>;
#[doc = "Reader of field `SOF`"]
pub type SOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CONN`"]
pub type CONN_R = crate::R<bool, bool>;
#[doc = "Reader of field `DISCON`"]
pub type DISCON_R = crate::R<bool, bool>;
#[doc = "Reader of field `SESREQ`"]
pub type SESREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `VBUSERR`"]
pub type VBUSERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SUSPEND Signaling Detected"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RESUME Signaling Detected"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Babble Detected"]
    #[inline(always)]
    pub fn babble(&self) -> BABBLE_R {
        BABBLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RESET Signaling Detected"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Session Connect"]
    #[inline(always)]
    pub fn conn(&self) -> CONN_R {
        CONN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Session Disconnect (OTG only)"]
    #[inline(always)]
    pub fn discon(&self) -> DISCON_R {
        DISCON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SESSION REQUEST (OTG only)"]
    #[inline(always)]
    pub fn sesreq(&self) -> SESREQ_R {
        SESREQ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - VBUS Error (OTG only)"]
    #[inline(always)]
    pub fn vbuserr(&self) -> VBUSERR_R {
        VBUSERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
