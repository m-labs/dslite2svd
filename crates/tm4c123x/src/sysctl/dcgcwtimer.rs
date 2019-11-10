#[doc = "Reader of register DCGCWTIMER"]
pub type R = crate::R<u32, super::DCGCWTIMER>;
#[doc = "Writer for register DCGCWTIMER"]
pub type W = crate::W<u32, super::DCGCWTIMER>;
#[doc = "Register DCGCWTIMER `reset()`'s with value 0"]
impl crate::ResetValue for super::DCGCWTIMER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D0`"]
pub type D0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D0`"]
pub struct D0_W<'a> {
    w: &'a mut W,
}
impl<'a> D0_W<'a> {
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
#[doc = "Reader of field `D1`"]
pub type D1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D1`"]
pub struct D1_W<'a> {
    w: &'a mut W,
}
impl<'a> D1_W<'a> {
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
#[doc = "Reader of field `D2`"]
pub type D2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D2`"]
pub struct D2_W<'a> {
    w: &'a mut W,
}
impl<'a> D2_W<'a> {
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
#[doc = "Reader of field `D3`"]
pub type D3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D3`"]
pub struct D3_W<'a> {
    w: &'a mut W,
}
impl<'a> D3_W<'a> {
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
#[doc = "Reader of field `D4`"]
pub type D4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D4`"]
pub struct D4_W<'a> {
    w: &'a mut W,
}
impl<'a> D4_W<'a> {
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
#[doc = "Reader of field `D5`"]
pub type D5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D5`"]
pub struct D5_W<'a> {
    w: &'a mut W,
}
impl<'a> D5_W<'a> {
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
impl R {
    #[doc = "Bit 0 - 32/64-Bit Wide General-Purpose Timer 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d0(&self) -> D0_R {
        D0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 32/64-Bit Wide General-Purpose Timer 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d1(&self) -> D1_R {
        D1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 32/64-Bit Wide General-Purpose Timer 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d2(&self) -> D2_R {
        D2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 32/64-Bit Wide General-Purpose Timer 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d3(&self) -> D3_R {
        D3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 32/64-Bit Wide General-Purpose Timer 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d4(&self) -> D4_R {
        D4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 32/64-Bit Wide General-Purpose Timer 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d5(&self) -> D5_R {
        D5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 32/64-Bit Wide General-Purpose Timer 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d0(&mut self) -> D0_W {
        D0_W { w: self }
    }
    #[doc = "Bit 1 - 32/64-Bit Wide General-Purpose Timer 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d1(&mut self) -> D1_W {
        D1_W { w: self }
    }
    #[doc = "Bit 2 - 32/64-Bit Wide General-Purpose Timer 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d2(&mut self) -> D2_W {
        D2_W { w: self }
    }
    #[doc = "Bit 3 - 32/64-Bit Wide General-Purpose Timer 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d3(&mut self) -> D3_W {
        D3_W { w: self }
    }
    #[doc = "Bit 4 - 32/64-Bit Wide General-Purpose Timer 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d4(&mut self) -> D4_W {
        D4_W { w: self }
    }
    #[doc = "Bit 5 - 32/64-Bit Wide General-Purpose Timer 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d5(&mut self) -> D5_W {
        D5_W { w: self }
    }
}
