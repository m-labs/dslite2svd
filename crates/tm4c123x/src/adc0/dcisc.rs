#[doc = "Reader of register DCISC"]
pub type R = crate::R<u32, super::DCISC>;
#[doc = "Writer for register DCISC"]
pub type W = crate::W<u32, super::DCISC>;
#[doc = "Register DCISC `reset()`'s with value 0"]
impl crate::ResetValue for super::DCISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCINT0`"]
pub type DCINT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCINT0`"]
pub struct DCINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> DCINT0_W<'a> {
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
#[doc = "Reader of field `DCINT1`"]
pub type DCINT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCINT1`"]
pub struct DCINT1_W<'a> {
    w: &'a mut W,
}
impl<'a> DCINT1_W<'a> {
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
#[doc = "Reader of field `DCINT2`"]
pub type DCINT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCINT2`"]
pub struct DCINT2_W<'a> {
    w: &'a mut W,
}
impl<'a> DCINT2_W<'a> {
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
#[doc = "Reader of field `DCINT3`"]
pub type DCINT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCINT3`"]
pub struct DCINT3_W<'a> {
    w: &'a mut W,
}
impl<'a> DCINT3_W<'a> {
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
#[doc = "Reader of field `DCINT4`"]
pub type DCINT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCINT4`"]
pub struct DCINT4_W<'a> {
    w: &'a mut W,
}
impl<'a> DCINT4_W<'a> {
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
#[doc = "Reader of field `DCINT5`"]
pub type DCINT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCINT5`"]
pub struct DCINT5_W<'a> {
    w: &'a mut W,
}
impl<'a> DCINT5_W<'a> {
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
#[doc = "Reader of field `DCINT6`"]
pub type DCINT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCINT6`"]
pub struct DCINT6_W<'a> {
    w: &'a mut W,
}
impl<'a> DCINT6_W<'a> {
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
#[doc = "Reader of field `DCINT7`"]
pub type DCINT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCINT7`"]
pub struct DCINT7_W<'a> {
    w: &'a mut W,
}
impl<'a> DCINT7_W<'a> {
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
    #[doc = "Bit 0 - Digital Comparator 0 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dcint0(&self) -> DCINT0_R {
        DCINT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Digital Comparator 1 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dcint1(&self) -> DCINT1_R {
        DCINT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Digital Comparator 2 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dcint2(&self) -> DCINT2_R {
        DCINT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Digital Comparator 3 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dcint3(&self) -> DCINT3_R {
        DCINT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Digital Comparator 4 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dcint4(&self) -> DCINT4_R {
        DCINT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Digital Comparator 5 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dcint5(&self) -> DCINT5_R {
        DCINT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Digital Comparator 6 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dcint6(&self) -> DCINT6_R {
        DCINT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Digital Comparator 7 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dcint7(&self) -> DCINT7_R {
        DCINT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Digital Comparator 0 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dcint0(&mut self) -> DCINT0_W {
        DCINT0_W { w: self }
    }
    #[doc = "Bit 1 - Digital Comparator 1 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dcint1(&mut self) -> DCINT1_W {
        DCINT1_W { w: self }
    }
    #[doc = "Bit 2 - Digital Comparator 2 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dcint2(&mut self) -> DCINT2_W {
        DCINT2_W { w: self }
    }
    #[doc = "Bit 3 - Digital Comparator 3 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dcint3(&mut self) -> DCINT3_W {
        DCINT3_W { w: self }
    }
    #[doc = "Bit 4 - Digital Comparator 4 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dcint4(&mut self) -> DCINT4_W {
        DCINT4_W { w: self }
    }
    #[doc = "Bit 5 - Digital Comparator 5 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dcint5(&mut self) -> DCINT5_W {
        DCINT5_W { w: self }
    }
    #[doc = "Bit 6 - Digital Comparator 6 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dcint6(&mut self) -> DCINT6_W {
        DCINT6_W { w: self }
    }
    #[doc = "Bit 7 - Digital Comparator 7 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dcint7(&mut self) -> DCINT7_W {
        DCINT7_W { w: self }
    }
}
