#[doc = "Reader of register PTBOCTL"]
pub type R = crate::R<u32, super::PTBOCTL>;
#[doc = "Writer for register PTBOCTL"]
pub type W = crate::W<u32, super::PTBOCTL>;
#[doc = "Register PTBOCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::PTBOCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "VDD (VDDS) under BOR Event Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VDD_UBOR_A {
    #[doc = "0: No Action"]
    NONE = 0,
    #[doc = "1: System control interrupt"]
    SYSINT = 1,
    #[doc = "2: NMI"]
    NMI = 2,
    #[doc = "3: Reset"]
    RST = 3,
}
impl From<VDD_UBOR_A> for u8 {
    #[inline(always)]
    fn from(variant: VDD_UBOR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VDD_UBOR`"]
pub type VDD_UBOR_R = crate::R<u8, VDD_UBOR_A>;
impl VDD_UBOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDD_UBOR_A {
        match self.bits {
            0 => VDD_UBOR_A::NONE,
            1 => VDD_UBOR_A::SYSINT,
            2 => VDD_UBOR_A::NMI,
            3 => VDD_UBOR_A::RST,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == VDD_UBOR_A::NONE
    }
    #[doc = "Checks if the value of the field is `SYSINT`"]
    #[inline(always)]
    pub fn is_sysint(&self) -> bool {
        *self == VDD_UBOR_A::SYSINT
    }
    #[doc = "Checks if the value of the field is `NMI`"]
    #[inline(always)]
    pub fn is_nmi(&self) -> bool {
        *self == VDD_UBOR_A::NMI
    }
    #[doc = "Checks if the value of the field is `RST`"]
    #[inline(always)]
    pub fn is_rst(&self) -> bool {
        *self == VDD_UBOR_A::RST
    }
}
#[doc = "Write proxy for field `VDD_UBOR`"]
pub struct VDD_UBOR_W<'a> {
    w: &'a mut W,
}
impl<'a> VDD_UBOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDD_UBOR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(VDD_UBOR_A::NONE)
    }
    #[doc = "System control interrupt"]
    #[inline(always)]
    pub fn sysint(self) -> &'a mut W {
        self.variant(VDD_UBOR_A::SYSINT)
    }
    #[doc = "NMI"]
    #[inline(always)]
    pub fn nmi(self) -> &'a mut W {
        self.variant(VDD_UBOR_A::NMI)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn rst(self) -> &'a mut W {
        self.variant(VDD_UBOR_A::RST)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "VDDA under BOR Event Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VDDA_UBOR_A {
    #[doc = "0: No Action"]
    NONE = 0,
    #[doc = "1: System control interrupt"]
    SYSINT = 1,
    #[doc = "2: NMI"]
    NMI = 2,
    #[doc = "3: Reset"]
    RST = 3,
}
impl From<VDDA_UBOR_A> for u8 {
    #[inline(always)]
    fn from(variant: VDDA_UBOR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VDDA_UBOR`"]
pub type VDDA_UBOR_R = crate::R<u8, VDDA_UBOR_A>;
impl VDDA_UBOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDA_UBOR_A {
        match self.bits {
            0 => VDDA_UBOR_A::NONE,
            1 => VDDA_UBOR_A::SYSINT,
            2 => VDDA_UBOR_A::NMI,
            3 => VDDA_UBOR_A::RST,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == VDDA_UBOR_A::NONE
    }
    #[doc = "Checks if the value of the field is `SYSINT`"]
    #[inline(always)]
    pub fn is_sysint(&self) -> bool {
        *self == VDDA_UBOR_A::SYSINT
    }
    #[doc = "Checks if the value of the field is `NMI`"]
    #[inline(always)]
    pub fn is_nmi(&self) -> bool {
        *self == VDDA_UBOR_A::NMI
    }
    #[doc = "Checks if the value of the field is `RST`"]
    #[inline(always)]
    pub fn is_rst(&self) -> bool {
        *self == VDDA_UBOR_A::RST
    }
}
#[doc = "Write proxy for field `VDDA_UBOR`"]
pub struct VDDA_UBOR_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDA_UBOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDA_UBOR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(VDDA_UBOR_A::NONE)
    }
    #[doc = "System control interrupt"]
    #[inline(always)]
    pub fn sysint(self) -> &'a mut W {
        self.variant(VDDA_UBOR_A::SYSINT)
    }
    #[doc = "NMI"]
    #[inline(always)]
    pub fn nmi(self) -> &'a mut W {
        self.variant(VDDA_UBOR_A::NMI)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn rst(self) -> &'a mut W {
        self.variant(VDDA_UBOR_A::RST)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - VDD (VDDS) under BOR Event Action"]
    #[inline(always)]
    pub fn vdd_ubor(&self) -> VDD_UBOR_R {
        VDD_UBOR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - VDDA under BOR Event Action"]
    #[inline(always)]
    pub fn vdda_ubor(&self) -> VDDA_UBOR_R {
        VDDA_UBOR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - VDD (VDDS) under BOR Event Action"]
    #[inline(always)]
    pub fn vdd_ubor(&mut self) -> VDD_UBOR_W {
        VDD_UBOR_W { w: self }
    }
    #[doc = "Bits 8:9 - VDDA under BOR Event Action"]
    #[inline(always)]
    pub fn vdda_ubor(&mut self) -> VDDA_UBOR_W {
        VDDA_UBOR_W { w: self }
    }
}
