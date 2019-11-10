#[doc = "Reader of register LPMIM"]
pub type R = crate::R<u8, super::LPMIM>;
#[doc = "Reader of field `STALL`"]
pub type STALL_R = crate::R<bool, bool>;
#[doc = "Reader of field `NY`"]
pub type NY_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACK`"]
pub type ACK_R = crate::R<bool, bool>;
#[doc = "Reader of field `NC`"]
pub type NC_R = crate::R<bool, bool>;
#[doc = "Reader of field `RES`"]
pub type RES_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - LPM STALL Interrupt Mask"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LPM NY Interrupt Mask"]
    #[inline(always)]
    pub fn ny(&self) -> NY_R {
        NY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LPM ACK Interrupt Mask"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LPM NC Interrupt Mask"]
    #[inline(always)]
    pub fn nc(&self) -> NC_R {
        NC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LPM Resume Interrupt Mask"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LPM Error Interrupt Mask"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
