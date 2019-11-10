#[doc = "Reader of register DIVSCLK"]
pub type R = crate::R<u32, super::DIVSCLK>;
#[doc = "Writer for register DIVSCLK"]
pub type W = crate::W<u32, super::DIVSCLK>;
#[doc = "Register DIVSCLK `reset()`'s with value 0"]
impl crate::ResetValue for super::DIVSCLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIV`"]
pub type DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV`"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_A {
    #[doc = "0: System Clock"]
    SYSCLK,
    #[doc = "1: PIOSC"]
    PIOSC,
    #[doc = "2: MOSC"]
    MOSC,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        match variant {
            SRC_A::SYSCLK => 0,
            SRC_A::PIOSC => 1,
            SRC_A::MOSC => 2,
        }
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<u8, SRC_A>;
impl SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRC_A::SYSCLK),
            1 => Val(SRC_A::PIOSC),
            2 => Val(SRC_A::MOSC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == SRC_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `PIOSC`"]
    #[inline(always)]
    pub fn is_piosc(&self) -> bool {
        *self == SRC_A::PIOSC
    }
    #[doc = "Checks if the value of the field is `MOSC`"]
    #[inline(always)]
    pub fn is_mosc(&self) -> bool {
        *self == SRC_A::MOSC
    }
}
#[doc = "Write proxy for field `SRC`"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "System Clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(SRC_A::SYSCLK)
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn piosc(self) -> &'a mut W {
        self.variant(SRC_A::PIOSC)
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn mosc(self) -> &'a mut W {
        self.variant(SRC_A::MOSC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Divisor Value"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Clock Source"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 31 - DIVSCLK Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Divisor Value"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Bits 16:17 - Clock Source"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
    #[doc = "Bit 31 - DIVSCLK Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
