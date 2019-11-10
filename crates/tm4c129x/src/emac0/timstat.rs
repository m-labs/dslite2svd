#[doc = "Reader of register TIMSTAT"]
pub type R = crate::R<u32, super::TIMSTAT>;
#[doc = "Reader of field `TSSOVF`"]
pub type TSSOVF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSTARGT`"]
pub type TSTARGT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Timestamp Seconds Overflow"]
    #[inline(always)]
    pub fn tssovf(&self) -> TSSOVF_R {
        TSSOVF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timestamp Target Time Reached"]
    #[inline(always)]
    pub fn tstargt(&self) -> TSTARGT_R {
        TSTARGT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
