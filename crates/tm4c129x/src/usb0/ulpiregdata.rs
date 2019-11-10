#[doc = "Reader of register ULPIREGDATA"]
pub type R = crate::R<u8, super::ULPIREGDATA>;
#[doc = "Writer for register ULPIREGDATA"]
pub type W = crate::W<u8, super::ULPIREGDATA>;
#[doc = "Register ULPIREGDATA `reset()`'s with value 0"]
impl crate::ResetValue for super::ULPIREGDATA {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REGDATA`"]
pub type REGDATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REGDATA`"]
pub struct REGDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> REGDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Register Data"]
    #[inline(always)]
    pub fn regdata(&self) -> REGDATA_R {
        REGDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Register Data"]
    #[inline(always)]
    pub fn regdata(&mut self) -> REGDATA_W {
        REGDATA_W { w: self }
    }
}
