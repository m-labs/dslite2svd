#[doc = "Reader of register EEPROT"]
pub type R = crate::R<u32, super::EEPROT>;
#[doc = "Writer for register EEPROT"]
pub type W = crate::W<u32, super::EEPROT>;
#[doc = "Register EEPROT `reset()`'s with value 0"]
impl crate::ResetValue for super::EEPROT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Protection Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROT_A {
    #[doc = "0: This setting is the default. If there is no password, the block is not protected and is readable and writable"]
    RWNPW,
    #[doc = "1: If there is a password, the block is readable or writable only when unlocked"]
    RWPW,
    #[doc = "2: If there is no password, the block is readable, not writable"]
    RONPW,
}
impl From<PROT_A> for u8 {
    #[inline(always)]
    fn from(variant: PROT_A) -> Self {
        match variant {
            PROT_A::RWNPW => 0,
            PROT_A::RWPW => 1,
            PROT_A::RONPW => 2,
        }
    }
}
#[doc = "Reader of field `PROT`"]
pub type PROT_R = crate::R<u8, PROT_A>;
impl PROT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PROT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PROT_A::RWNPW),
            1 => Val(PROT_A::RWPW),
            2 => Val(PROT_A::RONPW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RWNPW`"]
    #[inline(always)]
    pub fn is_rwnpw(&self) -> bool {
        *self == PROT_A::RWNPW
    }
    #[doc = "Checks if the value of the field is `RWPW`"]
    #[inline(always)]
    pub fn is_rwpw(&self) -> bool {
        *self == PROT_A::RWPW
    }
    #[doc = "Checks if the value of the field is `RONPW`"]
    #[inline(always)]
    pub fn is_ronpw(&self) -> bool {
        *self == PROT_A::RONPW
    }
}
#[doc = "Write proxy for field `PROT`"]
pub struct PROT_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "This setting is the default. If there is no password, the block is not protected and is readable and writable"]
    #[inline(always)]
    pub fn rwnpw(self) -> &'a mut W {
        self.variant(PROT_A::RWNPW)
    }
    #[doc = "If there is a password, the block is readable or writable only when unlocked"]
    #[inline(always)]
    pub fn rwpw(self) -> &'a mut W {
        self.variant(PROT_A::RWPW)
    }
    #[doc = "If there is no password, the block is readable, not writable"]
    #[inline(always)]
    pub fn ronpw(self) -> &'a mut W {
        self.variant(PROT_A::RONPW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `ACC`"]
pub type ACC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACC`"]
pub struct ACC_W<'a> {
    w: &'a mut W,
}
impl<'a> ACC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Protection Control"]
    #[inline(always)]
    pub fn prot(&self) -> PROT_R {
        PROT_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Access Control"]
    #[inline(always)]
    pub fn acc(&self) -> ACC_R {
        ACC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Protection Control"]
    #[inline(always)]
    pub fn prot(&mut self) -> PROT_W {
        PROT_W { w: self }
    }
    #[doc = "Bit 3 - Access Control"]
    #[inline(always)]
    pub fn acc(&mut self) -> ACC_W {
        ACC_W { w: self }
    }
}
