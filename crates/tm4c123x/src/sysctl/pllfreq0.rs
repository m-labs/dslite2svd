#[doc = "Reader of register PLLFREQ0"]
pub type R = crate::R<u32, super::PLLFREQ0>;
#[doc = "Writer for register PLLFREQ0"]
pub type W = crate::W<u32, super::PLLFREQ0>;
#[doc = "Register PLLFREQ0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PLLFREQ0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MINT`"]
pub type MINT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MINT`"]
pub struct MINT_W<'a> {
    w: &'a mut W,
}
impl<'a> MINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `MFRAC`"]
pub type MFRAC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MFRAC`"]
pub struct MFRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> MFRAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | (((value as u32) & 0x03ff) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - PLL M Integer Value"]
    #[inline(always)]
    pub fn mint(&self) -> MINT_R {
        MINT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - PLL M Fractional Value"]
    #[inline(always)]
    pub fn mfrac(&self) -> MFRAC_R {
        MFRAC_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - PLL M Integer Value"]
    #[inline(always)]
    pub fn mint(&mut self) -> MINT_W {
        MINT_W { w: self }
    }
    #[doc = "Bits 10:19 - PLL M Fractional Value"]
    #[inline(always)]
    pub fn mfrac(&mut self) -> MFRAC_W {
        MFRAC_W { w: self }
    }
}
