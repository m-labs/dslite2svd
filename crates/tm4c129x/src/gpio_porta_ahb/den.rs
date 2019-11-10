#[doc = "Reader of register DEN"]
pub type R = crate::R<u32, super::DEN>;
#[doc = "Writer for register DEN"]
pub type W = crate::W<u32, super::DEN>;
#[doc = "Register DEN `reset()`'s with value 0"]
impl crate::ResetValue for super::DEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEN`"]
pub type DEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEN`"]
pub struct DEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn den(&mut self) -> DEN_W {
        DEN_W { w: self }
    }
}
