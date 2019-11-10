#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Reader of field `ACTIVE`"]
pub type ACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `NBRBUSY`"]
pub type NBRBUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `WBUSY`"]
pub type WBUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `INITSEQ`"]
pub type INITSEQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `XFEMPTY`"]
pub type XFEMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `XFFULL`"]
pub type XFFULL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Register Active"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Non-Blocking Read Busy"]
    #[inline(always)]
    pub fn nbrbusy(&self) -> NBRBUSY_R {
        NBRBUSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write Busy"]
    #[inline(always)]
    pub fn wbusy(&self) -> WBUSY_R {
        WBUSY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Initialization Sequence"]
    #[inline(always)]
    pub fn initseq(&self) -> INITSEQ_R {
        INITSEQ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External FIFO Empty"]
    #[inline(always)]
    pub fn xfempty(&self) -> XFEMPTY_R {
        XFEMPTY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - External FIFO Full"]
    #[inline(always)]
    pub fn xffull(&self) -> XFFULL_R {
        XFFULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
