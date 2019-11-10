#[doc = "Reader of register TPLOG7"]
pub type R = crate::R<u32, super::TPLOG7>;
#[doc = "Reader of field `TRIG0`"]
pub type TRIG0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRIG1`"]
pub type TRIG1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRIG2`"]
pub type TRIG2_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRIG3`"]
pub type TRIG3_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSC`"]
pub type XOSC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Status of TMPR\\[0\\] Trigger"]
    #[inline(always)]
    pub fn trig0(&self) -> TRIG0_R {
        TRIG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status of TMPR\\[1\\] Trigger"]
    #[inline(always)]
    pub fn trig1(&self) -> TRIG1_R {
        TRIG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status of TMPR\\[2\\] Trigger"]
    #[inline(always)]
    pub fn trig2(&self) -> TRIG2_R {
        TRIG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status of TMPR\\[3\\] Trigger"]
    #[inline(always)]
    pub fn trig3(&self) -> TRIG3_R {
        TRIG3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Status of external 32"]
    #[inline(always)]
    pub fn xosc(&self) -> XOSC_R {
        XOSC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
