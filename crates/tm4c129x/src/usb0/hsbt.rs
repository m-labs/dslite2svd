#[doc = "Reader of register HSBT"]
pub type R = crate::R<u16, super::HSBT>;
#[doc = "Writer for register HSBT"]
pub type W = crate::W<u16, super::HSBT>;
#[doc = "Register HSBT `reset()`'s with value 0"]
impl crate::ResetValue for super::HSBT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HSBT`"]
pub type HSBT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSBT`"]
pub struct HSBT_W<'a> {
    w: &'a mut W,
}
impl<'a> HSBT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - High Speed Timeout Adder"]
    #[inline(always)]
    pub fn hsbt(&self) -> HSBT_R {
        HSBT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - High Speed Timeout Adder"]
    #[inline(always)]
    pub fn hsbt(&mut self) -> HSBT_W {
        HSBT_W { w: self }
    }
}
