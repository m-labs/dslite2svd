#[doc = "Reader of register _2_RIS"]
pub type R = crate::R<u32, super::_2_RIS>;
#[doc = "Reader of field `INTCNTZERO`"]
pub type INTCNTZERO_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTCNTLOAD`"]
pub type INTCNTLOAD_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTCMPAU`"]
pub type INTCMPAU_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTCMPAD`"]
pub type INTCMPAD_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTCMPBU`"]
pub type INTCMPBU_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTCMPBD`"]
pub type INTCMPBD_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Counter=0 Interrupt Status"]
    #[inline(always)]
    pub fn intcntzero(&self) -> INTCNTZERO_R {
        INTCNTZERO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counter=Load Interrupt Status"]
    #[inline(always)]
    pub fn intcntload(&self) -> INTCNTLOAD_R {
        INTCNTLOAD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comparator A Up Interrupt Status"]
    #[inline(always)]
    pub fn intcmpau(&self) -> INTCMPAU_R {
        INTCMPAU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Comparator A Down Interrupt Status"]
    #[inline(always)]
    pub fn intcmpad(&self) -> INTCMPAD_R {
        INTCMPAD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comparator B Up Interrupt Status"]
    #[inline(always)]
    pub fn intcmpbu(&self) -> INTCMPBU_R {
        INTCMPBU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Comparator B Down Interrupt Status"]
    #[inline(always)]
    pub fn intcmpbd(&self) -> INTCMPBD_R {
        INTCMPBD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
