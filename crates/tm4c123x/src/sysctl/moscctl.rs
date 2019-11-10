#[doc = "Reader of register MOSCCTL"]
pub type R = crate::R<u32, super::MOSCCTL>;
#[doc = "Writer for register MOSCCTL"]
pub type W = crate::W<u32, super::MOSCCTL>;
#[doc = "Register MOSCCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::MOSCCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CVAL`"]
pub type CVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CVAL`"]
pub struct CVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CVAL_W<'a> {
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
#[doc = "Reader of field `MOSCIM`"]
pub type MOSCIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOSCIM`"]
pub struct MOSCIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `NOXTAL`"]
pub type NOXTAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOXTAL`"]
pub struct NOXTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> NOXTAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock Validation for MOSC"]
    #[inline(always)]
    pub fn cval(&self) -> CVAL_R {
        CVAL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MOSC Failure Action"]
    #[inline(always)]
    pub fn moscim(&self) -> MOSCIM_R {
        MOSCIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - No Crystal Connected"]
    #[inline(always)]
    pub fn noxtal(&self) -> NOXTAL_R {
        NOXTAL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Validation for MOSC"]
    #[inline(always)]
    pub fn cval(&mut self) -> CVAL_W {
        CVAL_W { w: self }
    }
    #[doc = "Bit 1 - MOSC Failure Action"]
    #[inline(always)]
    pub fn moscim(&mut self) -> MOSCIM_W {
        MOSCIM_W { w: self }
    }
    #[doc = "Bit 2 - No Crystal Connected"]
    #[inline(always)]
    pub fn noxtal(&mut self) -> NOXTAL_W {
        NOXTAL_W { w: self }
    }
}
