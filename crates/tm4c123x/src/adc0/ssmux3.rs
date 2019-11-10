#[doc = "Reader of register SSMUX3"]
pub type R = crate::R<u32, super::SSMUX3>;
#[doc = "Writer for register SSMUX3"]
pub type W = crate::W<u32, super::SSMUX3>;
#[doc = "Register SSMUX3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSMUX3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MUX0`"]
pub type MUX0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MUX0`"]
pub struct MUX0_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - 1st Sample Input Select"]
    #[inline(always)]
    pub fn mux0(&self) -> MUX0_R {
        MUX0_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1st Sample Input Select"]
    #[inline(always)]
    pub fn mux0(&mut self) -> MUX0_W {
        MUX0_W { w: self }
    }
}
