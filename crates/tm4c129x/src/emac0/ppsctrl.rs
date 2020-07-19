#[doc = "Reader of register PPSCTRL"]
pub type R = crate::R<u32, super::PPSCTRL>;
#[doc = "Writer for register PPSCTRL"]
pub type W = crate::W<u32, super::PPSCTRL>;
#[doc = "Register PPSCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PPSCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PPSCTRL`"]
pub type PPSCTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PPSCTRL`"]
pub struct PPSCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> PPSCTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PPSEN0`"]
pub type PPSEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PPSEN0`"]
pub struct PPSEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PPSEN0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Target Time Register Mode for PPS0 Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGMODS0_A {
    #[doc = "0: Indicates that the Target Time registers are programmed only for generating the interrupt event"]
    INTONLY = 0,
    #[doc = "2: Indicates that the Target Time registers are programmed for generating the interrupt event and starting or stopping the generation of the EN0PPS output signal"]
    INTPPS0 = 2,
    #[doc = "3: Indicates that the Target Time registers are programmed only for starting or stopping the generation of the EN0PPS output signal. No interrupt is asserted"]
    PPS0ONLY = 3,
}
impl From<TRGMODS0_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGMODS0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRGMODS0`"]
pub type TRGMODS0_R = crate::R<u8, TRGMODS0_A>;
impl TRGMODS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRGMODS0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRGMODS0_A::INTONLY),
            2 => Val(TRGMODS0_A::INTPPS0),
            3 => Val(TRGMODS0_A::PPS0ONLY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INTONLY`"]
    #[inline(always)]
    pub fn is_intonly(&self) -> bool {
        *self == TRGMODS0_A::INTONLY
    }
    #[doc = "Checks if the value of the field is `INTPPS0`"]
    #[inline(always)]
    pub fn is_intpps0(&self) -> bool {
        *self == TRGMODS0_A::INTPPS0
    }
    #[doc = "Checks if the value of the field is `PPS0ONLY`"]
    #[inline(always)]
    pub fn is_pps0only(&self) -> bool {
        *self == TRGMODS0_A::PPS0ONLY
    }
}
#[doc = "Write proxy for field `TRGMODS0`"]
pub struct TRGMODS0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGMODS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGMODS0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Indicates that the Target Time registers are programmed only for generating the interrupt event"]
    #[inline(always)]
    pub fn intonly(self) -> &'a mut W {
        self.variant(TRGMODS0_A::INTONLY)
    }
    #[doc = "Indicates that the Target Time registers are programmed for generating the interrupt event and starting or stopping the generation of the EN0PPS output signal"]
    #[inline(always)]
    pub fn intpps0(self) -> &'a mut W {
        self.variant(TRGMODS0_A::INTPPS0)
    }
    #[doc = "Indicates that the Target Time registers are programmed only for starting or stopping the generation of the EN0PPS output signal. No interrupt is asserted"]
    #[inline(always)]
    pub fn pps0only(self) -> &'a mut W {
        self.variant(TRGMODS0_A::PPS0ONLY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - EN0PPS Output Frequency Control (PPSCTRL) or Command Control (PPSCMD)"]
    #[inline(always)]
    pub fn ppsctrl(&self) -> PPSCTRL_R {
        PPSCTRL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable"]
    #[inline(always)]
    pub fn ppsen0(&self) -> PPSEN0_R {
        PPSEN0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Target Time Register Mode for PPS0 Output"]
    #[inline(always)]
    pub fn trgmods0(&self) -> TRGMODS0_R {
        TRGMODS0_R::new(((self.bits >> 5) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EN0PPS Output Frequency Control (PPSCTRL) or Command Control (PPSCMD)"]
    #[inline(always)]
    pub fn ppsctrl(&mut self) -> PPSCTRL_W {
        PPSCTRL_W { w: self }
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable"]
    #[inline(always)]
    pub fn ppsen0(&mut self) -> PPSEN0_W {
        PPSEN0_W { w: self }
    }
    #[doc = "Bits 5:6 - Target Time Register Mode for PPS0 Output"]
    #[inline(always)]
    pub fn trgmods0(&mut self) -> TRGMODS0_W {
        TRGMODS0_W { w: self }
    }
}
