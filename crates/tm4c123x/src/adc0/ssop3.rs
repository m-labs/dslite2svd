#[doc = "Reader of register SSOP3"]
pub type R = crate::R<u32, super::SSOP3>;
#[doc = "Writer for register SSOP3"]
pub type W = crate::W<u32, super::SSOP3>;
#[doc = "Register SSOP3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSOP3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `S0DCOP`"]
pub type S0DCOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S0DCOP`"]
pub struct S0DCOP_W<'a> {
    w: &'a mut W,
}
impl<'a> S0DCOP_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Sample 0 Digital Comparator Operation"]
    #[inline(always)]
    pub fn s0dcop(&self) -> S0DCOP_R {
        S0DCOP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sample 0 Digital Comparator Operation"]
    #[inline(always)]
    pub fn s0dcop(&mut self) -> S0DCOP_W {
        S0DCOP_W { w: self }
    }
}
