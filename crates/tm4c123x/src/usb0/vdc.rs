#[doc = "Reader of register VDC"]
pub type R = crate::R<u32, super::VDC>;
#[doc = "Writer for register VDC"]
pub type W = crate::W<u32, super::VDC>;
#[doc = "Register VDC `reset()`'s with value 0"]
impl crate::ResetValue for super::VDC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VBDEN`"]
pub type VBDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBDEN`"]
pub struct VBDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBDEN_W<'a> {
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
    #[doc = "Bit 0 - VBUS Droop Enable"]
    #[inline(always)]
    pub fn vbden(&self) -> VBDEN_R {
        VBDEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUS Droop Enable"]
    #[inline(always)]
    pub fn vbden(&mut self) -> VBDEN_W {
        VBDEN_W { w: self }
    }
}
