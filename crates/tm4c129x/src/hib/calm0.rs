#[doc = "Reader of register CALM0"]
pub type R = crate::R<u32, super::CALM0>;
#[doc = "Writer for register CALM0"]
pub type W = crate::W<u32, super::CALM0>;
#[doc = "Register CALM0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CALM0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEC`"]
pub type SEC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEC`"]
pub struct SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `MIN`"]
pub type MIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MIN`"]
pub struct MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `HR`"]
pub type HR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HR`"]
pub struct HR_W<'a> {
    w: &'a mut W,
}
impl<'a> HR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `AMPM`"]
pub type AMPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMPM`"]
pub struct AMPM_W<'a> {
    w: &'a mut W,
}
impl<'a> AMPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Seconds"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Minutes"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Hours"]
    #[inline(always)]
    pub fn hr(&self) -> HR_R {
        HR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - AM/PM Designation"]
    #[inline(always)]
    pub fn ampm(&self) -> AMPM_R {
        AMPM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Seconds"]
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W {
        SEC_W { w: self }
    }
    #[doc = "Bits 8:13 - Minutes"]
    #[inline(always)]
    pub fn min(&mut self) -> MIN_W {
        MIN_W { w: self }
    }
    #[doc = "Bits 16:20 - Hours"]
    #[inline(always)]
    pub fn hr(&mut self) -> HR_W {
        HR_W { w: self }
    }
    #[doc = "Bit 22 - AM/PM Designation"]
    #[inline(always)]
    pub fn ampm(&mut self) -> AMPM_W {
        AMPM_W { w: self }
    }
}
