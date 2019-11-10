#[doc = "Reader of register SRGPIO"]
pub type R = crate::R<u32, super::SRGPIO>;
#[doc = "Writer for register SRGPIO"]
pub type W = crate::W<u32, super::SRGPIO>;
#[doc = "Register SRGPIO `reset()`'s with value 0"]
impl crate::ResetValue for super::SRGPIO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `R0`"]
pub type R0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R0`"]
pub struct R0_W<'a> {
    w: &'a mut W,
}
impl<'a> R0_W<'a> {
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
#[doc = "Reader of field `R1`"]
pub type R1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R1`"]
pub struct R1_W<'a> {
    w: &'a mut W,
}
impl<'a> R1_W<'a> {
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
#[doc = "Reader of field `R2`"]
pub type R2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R2`"]
pub struct R2_W<'a> {
    w: &'a mut W,
}
impl<'a> R2_W<'a> {
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
#[doc = "Reader of field `R3`"]
pub type R3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R3`"]
pub struct R3_W<'a> {
    w: &'a mut W,
}
impl<'a> R3_W<'a> {
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
#[doc = "Reader of field `R4`"]
pub type R4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R4`"]
pub struct R4_W<'a> {
    w: &'a mut W,
}
impl<'a> R4_W<'a> {
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
#[doc = "Reader of field `R5`"]
pub type R5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R5`"]
pub struct R5_W<'a> {
    w: &'a mut W,
}
impl<'a> R5_W<'a> {
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
#[doc = "Reader of field `R6`"]
pub type R6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R6`"]
pub struct R6_W<'a> {
    w: &'a mut W,
}
impl<'a> R6_W<'a> {
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
#[doc = "Reader of field `R7`"]
pub type R7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R7`"]
pub struct R7_W<'a> {
    w: &'a mut W,
}
impl<'a> R7_W<'a> {
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
#[doc = "Reader of field `R8`"]
pub type R8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R8`"]
pub struct R8_W<'a> {
    w: &'a mut W,
}
impl<'a> R8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `R9`"]
pub type R9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R9`"]
pub struct R9_W<'a> {
    w: &'a mut W,
}
impl<'a> R9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `R10`"]
pub type R10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R10`"]
pub struct R10_W<'a> {
    w: &'a mut W,
}
impl<'a> R10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `R11`"]
pub type R11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R11`"]
pub struct R11_W<'a> {
    w: &'a mut W,
}
impl<'a> R11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `R12`"]
pub type R12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R12`"]
pub struct R12_W<'a> {
    w: &'a mut W,
}
impl<'a> R12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `R13`"]
pub type R13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R13`"]
pub struct R13_W<'a> {
    w: &'a mut W,
}
impl<'a> R13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `R14`"]
pub type R14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R14`"]
pub struct R14_W<'a> {
    w: &'a mut W,
}
impl<'a> R14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - GPIO Port A Software Reset"]
    #[inline(always)]
    pub fn r0(&self) -> R0_R {
        R0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIO Port B Software Reset"]
    #[inline(always)]
    pub fn r1(&self) -> R1_R {
        R1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO Port C Software Reset"]
    #[inline(always)]
    pub fn r2(&self) -> R2_R {
        R2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO Port D Software Reset"]
    #[inline(always)]
    pub fn r3(&self) -> R3_R {
        R3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIO Port E Software Reset"]
    #[inline(always)]
    pub fn r4(&self) -> R4_R {
        R4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIO Port F Software Reset"]
    #[inline(always)]
    pub fn r5(&self) -> R5_R {
        R5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIO Port G Software Reset"]
    #[inline(always)]
    pub fn r6(&self) -> R6_R {
        R6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO Port H Software Reset"]
    #[inline(always)]
    pub fn r7(&self) -> R7_R {
        R7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPIO Port J Software Reset"]
    #[inline(always)]
    pub fn r8(&self) -> R8_R {
        R8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPIO Port K Software Reset"]
    #[inline(always)]
    pub fn r9(&self) -> R9_R {
        R9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPIO Port L Software Reset"]
    #[inline(always)]
    pub fn r10(&self) -> R10_R {
        R10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPIO Port M Software Reset"]
    #[inline(always)]
    pub fn r11(&self) -> R11_R {
        R11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - GPIO Port N Software Reset"]
    #[inline(always)]
    pub fn r12(&self) -> R12_R {
        R12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - GPIO Port P Software Reset"]
    #[inline(always)]
    pub fn r13(&self) -> R13_R {
        R13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - GPIO Port Q Software Reset"]
    #[inline(always)]
    pub fn r14(&self) -> R14_R {
        R14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO Port A Software Reset"]
    #[inline(always)]
    pub fn r0(&mut self) -> R0_W {
        R0_W { w: self }
    }
    #[doc = "Bit 1 - GPIO Port B Software Reset"]
    #[inline(always)]
    pub fn r1(&mut self) -> R1_W {
        R1_W { w: self }
    }
    #[doc = "Bit 2 - GPIO Port C Software Reset"]
    #[inline(always)]
    pub fn r2(&mut self) -> R2_W {
        R2_W { w: self }
    }
    #[doc = "Bit 3 - GPIO Port D Software Reset"]
    #[inline(always)]
    pub fn r3(&mut self) -> R3_W {
        R3_W { w: self }
    }
    #[doc = "Bit 4 - GPIO Port E Software Reset"]
    #[inline(always)]
    pub fn r4(&mut self) -> R4_W {
        R4_W { w: self }
    }
    #[doc = "Bit 5 - GPIO Port F Software Reset"]
    #[inline(always)]
    pub fn r5(&mut self) -> R5_W {
        R5_W { w: self }
    }
    #[doc = "Bit 6 - GPIO Port G Software Reset"]
    #[inline(always)]
    pub fn r6(&mut self) -> R6_W {
        R6_W { w: self }
    }
    #[doc = "Bit 7 - GPIO Port H Software Reset"]
    #[inline(always)]
    pub fn r7(&mut self) -> R7_W {
        R7_W { w: self }
    }
    #[doc = "Bit 8 - GPIO Port J Software Reset"]
    #[inline(always)]
    pub fn r8(&mut self) -> R8_W {
        R8_W { w: self }
    }
    #[doc = "Bit 9 - GPIO Port K Software Reset"]
    #[inline(always)]
    pub fn r9(&mut self) -> R9_W {
        R9_W { w: self }
    }
    #[doc = "Bit 10 - GPIO Port L Software Reset"]
    #[inline(always)]
    pub fn r10(&mut self) -> R10_W {
        R10_W { w: self }
    }
    #[doc = "Bit 11 - GPIO Port M Software Reset"]
    #[inline(always)]
    pub fn r11(&mut self) -> R11_W {
        R11_W { w: self }
    }
    #[doc = "Bit 12 - GPIO Port N Software Reset"]
    #[inline(always)]
    pub fn r12(&mut self) -> R12_W {
        R12_W { w: self }
    }
    #[doc = "Bit 13 - GPIO Port P Software Reset"]
    #[inline(always)]
    pub fn r13(&mut self) -> R13_W {
        R13_W { w: self }
    }
    #[doc = "Bit 14 - GPIO Port Q Software Reset"]
    #[inline(always)]
    pub fn r14(&mut self) -> R14_W {
        R14_W { w: self }
    }
}
