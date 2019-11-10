#[doc = "Reader of register IM"]
pub type R = crate::R<u32, super::IM>;
#[doc = "Writer for register IM"]
pub type W = crate::W<u32, super::IM>;
#[doc = "Register IM `reset()`'s with value 0"]
impl crate::ResetValue for super::IM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MASK0`"]
pub type MASK0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK0`"]
pub struct MASK0_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK0_W<'a> {
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
#[doc = "Reader of field `MASK1`"]
pub type MASK1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK1`"]
pub struct MASK1_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK1_W<'a> {
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
#[doc = "Reader of field `MASK2`"]
pub type MASK2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK2`"]
pub struct MASK2_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK2_W<'a> {
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
#[doc = "Reader of field `MASK3`"]
pub type MASK3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK3`"]
pub struct MASK3_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK3_W<'a> {
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
#[doc = "Reader of field `DCONSS0`"]
pub type DCONSS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCONSS0`"]
pub struct DCONSS0_W<'a> {
    w: &'a mut W,
}
impl<'a> DCONSS0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `DCONSS1`"]
pub type DCONSS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCONSS1`"]
pub struct DCONSS1_W<'a> {
    w: &'a mut W,
}
impl<'a> DCONSS1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `DCONSS2`"]
pub type DCONSS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCONSS2`"]
pub struct DCONSS2_W<'a> {
    w: &'a mut W,
}
impl<'a> DCONSS2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `DCONSS3`"]
pub type DCONSS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCONSS3`"]
pub struct DCONSS3_W<'a> {
    w: &'a mut W,
}
impl<'a> DCONSS3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SS0 Interrupt Mask"]
    #[inline(always)]
    pub fn mask0(&self) -> MASK0_R {
        MASK0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SS1 Interrupt Mask"]
    #[inline(always)]
    pub fn mask1(&self) -> MASK1_R {
        MASK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SS2 Interrupt Mask"]
    #[inline(always)]
    pub fn mask2(&self) -> MASK2_R {
        MASK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SS3 Interrupt Mask"]
    #[inline(always)]
    pub fn mask3(&self) -> MASK3_R {
        MASK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Digital Comparator Interrupt on SS0"]
    #[inline(always)]
    pub fn dconss0(&self) -> DCONSS0_R {
        DCONSS0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Digital Comparator Interrupt on SS1"]
    #[inline(always)]
    pub fn dconss1(&self) -> DCONSS1_R {
        DCONSS1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Digital Comparator Interrupt on SS2"]
    #[inline(always)]
    pub fn dconss2(&self) -> DCONSS2_R {
        DCONSS2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Digital Comparator Interrupt on SS3"]
    #[inline(always)]
    pub fn dconss3(&self) -> DCONSS3_R {
        DCONSS3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS0 Interrupt Mask"]
    #[inline(always)]
    pub fn mask0(&mut self) -> MASK0_W {
        MASK0_W { w: self }
    }
    #[doc = "Bit 1 - SS1 Interrupt Mask"]
    #[inline(always)]
    pub fn mask1(&mut self) -> MASK1_W {
        MASK1_W { w: self }
    }
    #[doc = "Bit 2 - SS2 Interrupt Mask"]
    #[inline(always)]
    pub fn mask2(&mut self) -> MASK2_W {
        MASK2_W { w: self }
    }
    #[doc = "Bit 3 - SS3 Interrupt Mask"]
    #[inline(always)]
    pub fn mask3(&mut self) -> MASK3_W {
        MASK3_W { w: self }
    }
    #[doc = "Bit 16 - Digital Comparator Interrupt on SS0"]
    #[inline(always)]
    pub fn dconss0(&mut self) -> DCONSS0_W {
        DCONSS0_W { w: self }
    }
    #[doc = "Bit 17 - Digital Comparator Interrupt on SS1"]
    #[inline(always)]
    pub fn dconss1(&mut self) -> DCONSS1_W {
        DCONSS1_W { w: self }
    }
    #[doc = "Bit 18 - Digital Comparator Interrupt on SS2"]
    #[inline(always)]
    pub fn dconss2(&mut self) -> DCONSS2_W {
        DCONSS2_W { w: self }
    }
    #[doc = "Bit 19 - Digital Comparator Interrupt on SS3"]
    #[inline(always)]
    pub fn dconss3(&mut self) -> DCONSS3_W {
        DCONSS3_W { w: self }
    }
}
