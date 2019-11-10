#[doc = "Reader of register PWRTC"]
pub type R = crate::R<u32, super::PWRTC>;
#[doc = "Writer for register PWRTC"]
pub type W = crate::W<u32, super::PWRTC>;
#[doc = "Register PWRTC `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRTC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VDD_UBOR`"]
pub type VDD_UBOR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDD_UBOR`"]
pub struct VDD_UBOR_W<'a> {
    w: &'a mut W,
}
impl<'a> VDD_UBOR_W<'a> {
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
#[doc = "Reader of field `VDDA_UBOR`"]
pub type VDDA_UBOR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDDA_UBOR`"]
pub struct VDDA_UBOR_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDA_UBOR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - VDD Under BOR Status"]
    #[inline(always)]
    pub fn vdd_ubor(&self) -> VDD_UBOR_R {
        VDD_UBOR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - VDDA Under BOR Status"]
    #[inline(always)]
    pub fn vdda_ubor(&self) -> VDDA_UBOR_R {
        VDDA_UBOR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VDD Under BOR Status"]
    #[inline(always)]
    pub fn vdd_ubor(&mut self) -> VDD_UBOR_W {
        VDD_UBOR_W { w: self }
    }
    #[doc = "Bit 4 - VDDA Under BOR Status"]
    #[inline(always)]
    pub fn vdda_ubor(&mut self) -> VDDA_UBOR_W {
        VDDA_UBOR_W { w: self }
    }
}
