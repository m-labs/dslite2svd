#[doc = "Reader of register CHASGN"]
pub type R = crate::R<u32, super::CHASGN>;
#[doc = "Writer for register CHASGN"]
pub type W = crate::W<u32, super::CHASGN>;
#[doc = "Register CHASGN `reset()`'s with value 0"]
impl crate::ResetValue for super::CHASGN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel \\[n\\]
Assignment Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CHASGN_A {
    #[doc = "0: Use the primary channel assignment"]
    PRIMARY = 0,
    #[doc = "1: Use the secondary channel assignment"]
    SECONDARY = 1,
}
impl From<CHASGN_A> for u32 {
    #[inline(always)]
    fn from(variant: CHASGN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CHASGN`"]
pub type CHASGN_R = crate::R<u32, CHASGN_A>;
impl CHASGN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, CHASGN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CHASGN_A::PRIMARY),
            1 => Val(CHASGN_A::SECONDARY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRIMARY`"]
    #[inline(always)]
    pub fn is_primary(&self) -> bool {
        *self == CHASGN_A::PRIMARY
    }
    #[doc = "Checks if the value of the field is `SECONDARY`"]
    #[inline(always)]
    pub fn is_secondary(&self) -> bool {
        *self == CHASGN_A::SECONDARY
    }
}
#[doc = "Write proxy for field `CHASGN`"]
pub struct CHASGN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHASGN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHASGN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Use the primary channel assignment"]
    #[inline(always)]
    pub fn primary(self) -> &'a mut W {
        self.variant(CHASGN_A::PRIMARY)
    }
    #[doc = "Use the secondary channel assignment"]
    #[inline(always)]
    pub fn secondary(self) -> &'a mut W {
        self.variant(CHASGN_A::SECONDARY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Assignment Select"]
    #[inline(always)]
    pub fn chasgn(&self) -> CHASGN_R {
        CHASGN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Assignment Select"]
    #[inline(always)]
    pub fn chasgn(&mut self) -> CHASGN_W {
        CHASGN_W { w: self }
    }
}
