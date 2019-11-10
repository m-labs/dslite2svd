#[doc = "Reader of register _1_FLTSTAT1"]
pub type R = crate::R<u32, super::_1_FLTSTAT1>;
#[doc = "Writer for register _1_FLTSTAT1"]
pub type W = crate::W<u32, super::_1_FLTSTAT1>;
#[doc = "Register _1_FLTSTAT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::_1_FLTSTAT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCMP0`"]
pub type DCMP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCMP0`"]
pub struct DCMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMP0_W<'a> {
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
#[doc = "Reader of field `DCMP1`"]
pub type DCMP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCMP1`"]
pub struct DCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMP1_W<'a> {
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
#[doc = "Reader of field `DCMP2`"]
pub type DCMP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCMP2`"]
pub struct DCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMP2_W<'a> {
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
#[doc = "Reader of field `DCMP3`"]
pub type DCMP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCMP3`"]
pub struct DCMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMP3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DCMP4`"]
pub type DCMP4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCMP4`"]
pub struct DCMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMP4_W<'a> {
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
#[doc = "Reader of field `DCMP5`"]
pub type DCMP5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCMP5`"]
pub struct DCMP5_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMP5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `DCMP6`"]
pub type DCMP6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCMP6`"]
pub struct DCMP6_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMP6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DCMP7`"]
pub type DCMP7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCMP7`"]
pub struct DCMP7_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMP7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Digital Comparator 0 Trigger"]
    #[inline(always)]
    pub fn dcmp0(&self) -> DCMP0_R {
        DCMP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Digital Comparator 1 Trigger"]
    #[inline(always)]
    pub fn dcmp1(&self) -> DCMP1_R {
        DCMP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Digital Comparator 2 Trigger"]
    #[inline(always)]
    pub fn dcmp2(&self) -> DCMP2_R {
        DCMP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Digital Comparator 3 Trigger"]
    #[inline(always)]
    pub fn dcmp3(&self) -> DCMP3_R {
        DCMP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Digital Comparator 4 Trigger"]
    #[inline(always)]
    pub fn dcmp4(&self) -> DCMP4_R {
        DCMP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Digital Comparator 5 Trigger"]
    #[inline(always)]
    pub fn dcmp5(&self) -> DCMP5_R {
        DCMP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Digital Comparator 6 Trigger"]
    #[inline(always)]
    pub fn dcmp6(&self) -> DCMP6_R {
        DCMP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Digital Comparator 7 Trigger"]
    #[inline(always)]
    pub fn dcmp7(&self) -> DCMP7_R {
        DCMP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Digital Comparator 0 Trigger"]
    #[inline(always)]
    pub fn dcmp0(&mut self) -> DCMP0_W {
        DCMP0_W { w: self }
    }
    #[doc = "Bit 1 - Digital Comparator 1 Trigger"]
    #[inline(always)]
    pub fn dcmp1(&mut self) -> DCMP1_W {
        DCMP1_W { w: self }
    }
    #[doc = "Bit 2 - Digital Comparator 2 Trigger"]
    #[inline(always)]
    pub fn dcmp2(&mut self) -> DCMP2_W {
        DCMP2_W { w: self }
    }
    #[doc = "Bit 3 - Digital Comparator 3 Trigger"]
    #[inline(always)]
    pub fn dcmp3(&mut self) -> DCMP3_W {
        DCMP3_W { w: self }
    }
    #[doc = "Bit 4 - Digital Comparator 4 Trigger"]
    #[inline(always)]
    pub fn dcmp4(&mut self) -> DCMP4_W {
        DCMP4_W { w: self }
    }
    #[doc = "Bit 5 - Digital Comparator 5 Trigger"]
    #[inline(always)]
    pub fn dcmp5(&mut self) -> DCMP5_W {
        DCMP5_W { w: self }
    }
    #[doc = "Bit 6 - Digital Comparator 6 Trigger"]
    #[inline(always)]
    pub fn dcmp6(&mut self) -> DCMP6_W {
        DCMP6_W { w: self }
    }
    #[doc = "Bit 7 - Digital Comparator 7 Trigger"]
    #[inline(always)]
    pub fn dcmp7(&mut self) -> DCMP7_W {
        DCMP7_W { w: self }
    }
}
