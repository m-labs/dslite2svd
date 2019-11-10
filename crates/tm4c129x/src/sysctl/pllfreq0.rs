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
    pub fn bits(self, value: u16) -> &'a mut W {
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
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | (((value as u32) & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Reader of field `PLLPWR`"]
pub type PLLPWR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLPWR`"]
pub struct PLLPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLPWR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
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
    #[doc = "Bit 23 - PLL Power"]
    #[inline(always)]
    pub fn pllpwr(&self) -> PLLPWR_R {
        PLLPWR_R::new(((self.bits >> 23) & 0x01) != 0)
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
    #[doc = "Bit 23 - PLL Power"]
    #[inline(always)]
    pub fn pllpwr(&mut self) -> PLLPWR_W {
        PLLPWR_W { w: self }
    }
}
