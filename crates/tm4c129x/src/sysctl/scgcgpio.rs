#[doc = "Reader of register SCGCGPIO"]
pub type R = crate::R<u32, super::SCGCGPIO>;
#[doc = "Writer for register SCGCGPIO"]
pub type W = crate::W<u32, super::SCGCGPIO>;
#[doc = "Register SCGCGPIO `reset()`'s with value 0"]
impl crate::ResetValue for super::SCGCGPIO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `S0`"]
pub type S0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S0`"]
pub struct S0_W<'a> {
    w: &'a mut W,
}
impl<'a> S0_W<'a> {
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
#[doc = "Reader of field `S1`"]
pub type S1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S1`"]
pub struct S1_W<'a> {
    w: &'a mut W,
}
impl<'a> S1_W<'a> {
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
#[doc = "Reader of field `S2`"]
pub type S2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S2`"]
pub struct S2_W<'a> {
    w: &'a mut W,
}
impl<'a> S2_W<'a> {
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
#[doc = "Reader of field `S3`"]
pub type S3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S3`"]
pub struct S3_W<'a> {
    w: &'a mut W,
}
impl<'a> S3_W<'a> {
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
#[doc = "Reader of field `S4`"]
pub type S4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S4`"]
pub struct S4_W<'a> {
    w: &'a mut W,
}
impl<'a> S4_W<'a> {
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
#[doc = "Reader of field `S5`"]
pub type S5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S5`"]
pub struct S5_W<'a> {
    w: &'a mut W,
}
impl<'a> S5_W<'a> {
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
#[doc = "Reader of field `S6`"]
pub type S6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S6`"]
pub struct S6_W<'a> {
    w: &'a mut W,
}
impl<'a> S6_W<'a> {
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
#[doc = "Reader of field `S7`"]
pub type S7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S7`"]
pub struct S7_W<'a> {
    w: &'a mut W,
}
impl<'a> S7_W<'a> {
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
#[doc = "Reader of field `S8`"]
pub type S8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S8`"]
pub struct S8_W<'a> {
    w: &'a mut W,
}
impl<'a> S8_W<'a> {
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
#[doc = "Reader of field `S9`"]
pub type S9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S9`"]
pub struct S9_W<'a> {
    w: &'a mut W,
}
impl<'a> S9_W<'a> {
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
#[doc = "Reader of field `S10`"]
pub type S10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S10`"]
pub struct S10_W<'a> {
    w: &'a mut W,
}
impl<'a> S10_W<'a> {
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
#[doc = "Reader of field `S11`"]
pub type S11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S11`"]
pub struct S11_W<'a> {
    w: &'a mut W,
}
impl<'a> S11_W<'a> {
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
#[doc = "Reader of field `S12`"]
pub type S12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S12`"]
pub struct S12_W<'a> {
    w: &'a mut W,
}
impl<'a> S12_W<'a> {
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
#[doc = "Reader of field `S13`"]
pub type S13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S13`"]
pub struct S13_W<'a> {
    w: &'a mut W,
}
impl<'a> S13_W<'a> {
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
#[doc = "Reader of field `S14`"]
pub type S14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S14`"]
pub struct S14_W<'a> {
    w: &'a mut W,
}
impl<'a> S14_W<'a> {
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
    #[doc = "Bit 0 - GPIO Port A Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s0(&self) -> S0_R {
        S0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIO Port B Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s1(&self) -> S1_R {
        S1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO Port C Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s2(&self) -> S2_R {
        S2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO Port D Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s3(&self) -> S3_R {
        S3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIO Port E Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s4(&self) -> S4_R {
        S4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIO Port F Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s5(&self) -> S5_R {
        S5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIO Port G Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s6(&self) -> S6_R {
        S6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO Port H Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s7(&self) -> S7_R {
        S7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPIO Port J Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s8(&self) -> S8_R {
        S8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPIO Port K Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s9(&self) -> S9_R {
        S9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPIO Port L Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s10(&self) -> S10_R {
        S10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPIO Port M Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s11(&self) -> S11_R {
        S11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - GPIO Port N Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s12(&self) -> S12_R {
        S12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - GPIO Port P Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s13(&self) -> S13_R {
        S13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - GPIO Port Q Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s14(&self) -> S14_R {
        S14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO Port A Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s0(&mut self) -> S0_W {
        S0_W { w: self }
    }
    #[doc = "Bit 1 - GPIO Port B Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s1(&mut self) -> S1_W {
        S1_W { w: self }
    }
    #[doc = "Bit 2 - GPIO Port C Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s2(&mut self) -> S2_W {
        S2_W { w: self }
    }
    #[doc = "Bit 3 - GPIO Port D Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s3(&mut self) -> S3_W {
        S3_W { w: self }
    }
    #[doc = "Bit 4 - GPIO Port E Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s4(&mut self) -> S4_W {
        S4_W { w: self }
    }
    #[doc = "Bit 5 - GPIO Port F Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s5(&mut self) -> S5_W {
        S5_W { w: self }
    }
    #[doc = "Bit 6 - GPIO Port G Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s6(&mut self) -> S6_W {
        S6_W { w: self }
    }
    #[doc = "Bit 7 - GPIO Port H Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s7(&mut self) -> S7_W {
        S7_W { w: self }
    }
    #[doc = "Bit 8 - GPIO Port J Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s8(&mut self) -> S8_W {
        S8_W { w: self }
    }
    #[doc = "Bit 9 - GPIO Port K Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s9(&mut self) -> S9_W {
        S9_W { w: self }
    }
    #[doc = "Bit 10 - GPIO Port L Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s10(&mut self) -> S10_W {
        S10_W { w: self }
    }
    #[doc = "Bit 11 - GPIO Port M Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s11(&mut self) -> S11_W {
        S11_W { w: self }
    }
    #[doc = "Bit 12 - GPIO Port N Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s12(&mut self) -> S12_W {
        S12_W { w: self }
    }
    #[doc = "Bit 13 - GPIO Port P Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s13(&mut self) -> S13_W {
        S13_W { w: self }
    }
    #[doc = "Bit 14 - GPIO Port Q Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s14(&mut self) -> S14_W {
        S14_W { w: self }
    }
}
