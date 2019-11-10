#[doc = "Reader of register DCGCACMP"]
pub type R = crate::R<u32, super::DCGCACMP>;
#[doc = "Writer for register DCGCACMP"]
pub type W = crate::W<u32, super::DCGCACMP>;
#[doc = "Register DCGCACMP `reset()`'s with value 0"]
impl crate::ResetValue for super::DCGCACMP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D0`"]
pub type D0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D0`"]
pub struct D0_W<'a> {
    w: &'a mut W,
}
impl<'a> D0_W<'a> {
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
    #[doc = "Bit 0 - Analog Comparator Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d0(&self) -> D0_R {
        D0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d0(&mut self) -> D0_W {
        D0_W { w: self }
    }
}
