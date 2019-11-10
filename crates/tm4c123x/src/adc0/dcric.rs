#[doc = "Writer for register DCRIC"]
pub type W = crate::W<u32, super::DCRIC>;
#[doc = "Register DCRIC `reset()`'s with value 0"]
impl crate::ResetValue for super::DCRIC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
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
#[doc = "Write proxy for field `DCTRIG0`"]
pub struct DCTRIG0_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG0_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG1`"]
pub struct DCTRIG1_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG1_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG2`"]
pub struct DCTRIG2_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG2_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG3`"]
pub struct DCTRIG3_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG3_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG4`"]
pub struct DCTRIG4_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Write proxy for field `DCTRIG5`"]
pub struct DCTRIG5_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Write proxy for field `DCTRIG6`"]
pub struct DCTRIG6_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG6_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG7`"]
pub struct DCTRIG7_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Digital Comparator Interrupt 0"]
    #[inline(always)]
    pub fn dcint0(&mut self) -> DCINT0_W {
        DCINT0_W { w: self }
    }
    #[doc = "Bit 1 - Digital Comparator Interrupt 1"]
    #[inline(always)]
    pub fn dcint1(&mut self) -> DCINT1_W {
        DCINT1_W { w: self }
    }
    #[doc = "Bit 2 - Digital Comparator Interrupt 2"]
    #[inline(always)]
    pub fn dcint2(&mut self) -> DCINT2_W {
        DCINT2_W { w: self }
    }
    #[doc = "Bit 3 - Digital Comparator Interrupt 3"]
    #[inline(always)]
    pub fn dcint3(&mut self) -> DCINT3_W {
        DCINT3_W { w: self }
    }
    #[doc = "Bit 4 - Digital Comparator Interrupt 4"]
    #[inline(always)]
    pub fn dcint4(&mut self) -> DCINT4_W {
        DCINT4_W { w: self }
    }
    #[doc = "Bit 5 - Digital Comparator Interrupt 5"]
    #[inline(always)]
    pub fn dcint5(&mut self) -> DCINT5_W {
        DCINT5_W { w: self }
    }
    #[doc = "Bit 6 - Digital Comparator Interrupt 6"]
    #[inline(always)]
    pub fn dcint6(&mut self) -> DCINT6_W {
        DCINT6_W { w: self }
    }
    #[doc = "Bit 7 - Digital Comparator Interrupt 7"]
    #[inline(always)]
    pub fn dcint7(&mut self) -> DCINT7_W {
        DCINT7_W { w: self }
    }
    #[doc = "Bit 16 - Digital Comparator Trigger 0"]
    #[inline(always)]
    pub fn dctrig0(&mut self) -> DCTRIG0_W {
        DCTRIG0_W { w: self }
    }
    #[doc = "Bit 17 - Digital Comparator Trigger 1"]
    #[inline(always)]
    pub fn dctrig1(&mut self) -> DCTRIG1_W {
        DCTRIG1_W { w: self }
    }
    #[doc = "Bit 18 - Digital Comparator Trigger 2"]
    #[inline(always)]
    pub fn dctrig2(&mut self) -> DCTRIG2_W {
        DCTRIG2_W { w: self }
    }
    #[doc = "Bit 19 - Digital Comparator Trigger 3"]
    #[inline(always)]
    pub fn dctrig3(&mut self) -> DCTRIG3_W {
        DCTRIG3_W { w: self }
    }
    #[doc = "Bit 20 - Digital Comparator Trigger 4"]
    #[inline(always)]
    pub fn dctrig4(&mut self) -> DCTRIG4_W {
        DCTRIG4_W { w: self }
    }
    #[doc = "Bit 21 - Digital Comparator Trigger 5"]
    #[inline(always)]
    pub fn dctrig5(&mut self) -> DCTRIG5_W {
        DCTRIG5_W { w: self }
    }
    #[doc = "Bit 22 - Digital Comparator Trigger 6"]
    #[inline(always)]
    pub fn dctrig6(&mut self) -> DCTRIG6_W {
        DCTRIG6_W { w: self }
    }
    #[doc = "Bit 23 - Digital Comparator Trigger 7"]
    #[inline(always)]
    pub fn dctrig7(&mut self) -> DCTRIG7_W {
        DCTRIG7_W { w: self }
    }
}
