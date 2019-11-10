#[doc = "Reader of register VDCIM"]
pub type R = crate::R<u32, super::VDCIM>;
#[doc = "Writer for register VDCIM"]
pub type W = crate::W<u32, super::VDCIM>;
#[doc = "Register VDCIM `reset()`'s with value 0"]
impl crate::ResetValue for super::VDCIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VD`"]
pub type VD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VD`"]
pub struct VD_W<'a> {
    w: &'a mut W,
}
impl<'a> VD_W<'a> {
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
    #[doc = "Bit 0 - VBUS Droop Interrupt Mask"]
    #[inline(always)]
    pub fn vd(&self) -> VD_R {
        VD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUS Droop Interrupt Mask"]
    #[inline(always)]
    pub fn vd(&mut self) -> VD_W {
        VD_W { w: self }
    }
}
