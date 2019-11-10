#[doc = "Reader of register DCGCGPIO"]
pub type R = crate::R<u32, super::DCGCGPIO>;
#[doc = "Writer for register DCGCGPIO"]
pub type W = crate::W<u32, super::DCGCGPIO>;
#[doc = "Register DCGCGPIO `reset()`'s with value 0"]
impl crate::ResetValue for super::DCGCGPIO {
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
#[doc = "Reader of field `D6`"]
pub type D6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D6`"]
pub struct D6_W<'a> {
    w: &'a mut W,
}
impl<'a> D6_W<'a> {
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
#[doc = "Reader of field `D7`"]
pub type D7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D7`"]
pub struct D7_W<'a> {
    w: &'a mut W,
}
impl<'a> D7_W<'a> {
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
#[doc = "Reader of field `D8`"]
pub type D8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D8`"]
pub struct D8_W<'a> {
    w: &'a mut W,
}
impl<'a> D8_W<'a> {
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
#[doc = "Reader of field `D9`"]
pub type D9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D9`"]
pub struct D9_W<'a> {
    w: &'a mut W,
}
impl<'a> D9_W<'a> {
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
#[doc = "Reader of field `D10`"]
pub type D10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D10`"]
pub struct D10_W<'a> {
    w: &'a mut W,
}
impl<'a> D10_W<'a> {
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
#[doc = "Reader of field `D11`"]
pub type D11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D11`"]
pub struct D11_W<'a> {
    w: &'a mut W,
}
impl<'a> D11_W<'a> {
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
#[doc = "Reader of field `D12`"]
pub type D12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D12`"]
pub struct D12_W<'a> {
    w: &'a mut W,
}
impl<'a> D12_W<'a> {
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
#[doc = "Reader of field `D13`"]
pub type D13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D13`"]
pub struct D13_W<'a> {
    w: &'a mut W,
}
impl<'a> D13_W<'a> {
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
#[doc = "Reader of field `D14`"]
pub type D14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D14`"]
pub struct D14_W<'a> {
    w: &'a mut W,
}
impl<'a> D14_W<'a> {
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
    #[doc = "Bit 0 - GPIO Port A Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d0(&self) -> D0_R {
        D0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIO Port B Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d1(&self) -> D1_R {
        D1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO Port C Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d2(&self) -> D2_R {
        D2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO Port D Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d3(&self) -> D3_R {
        D3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIO Port E Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d4(&self) -> D4_R {
        D4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIO Port F Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d5(&self) -> D5_R {
        D5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIO Port G Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d6(&self) -> D6_R {
        D6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO Port H Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d7(&self) -> D7_R {
        D7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPIO Port J Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d8(&self) -> D8_R {
        D8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPIO Port K Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d9(&self) -> D9_R {
        D9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPIO Port L Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d10(&self) -> D10_R {
        D10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPIO Port M Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d11(&self) -> D11_R {
        D11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - GPIO Port N Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d12(&self) -> D12_R {
        D12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - GPIO Port P Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d13(&self) -> D13_R {
        D13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - GPIO Port Q Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d14(&self) -> D14_R {
        D14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO Port A Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d0(&mut self) -> D0_W {
        D0_W { w: self }
    }
    #[doc = "Bit 1 - GPIO Port B Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d1(&mut self) -> D1_W {
        D1_W { w: self }
    }
    #[doc = "Bit 2 - GPIO Port C Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d2(&mut self) -> D2_W {
        D2_W { w: self }
    }
    #[doc = "Bit 3 - GPIO Port D Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d3(&mut self) -> D3_W {
        D3_W { w: self }
    }
    #[doc = "Bit 4 - GPIO Port E Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d4(&mut self) -> D4_W {
        D4_W { w: self }
    }
    #[doc = "Bit 5 - GPIO Port F Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d5(&mut self) -> D5_W {
        D5_W { w: self }
    }
    #[doc = "Bit 6 - GPIO Port G Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d6(&mut self) -> D6_W {
        D6_W { w: self }
    }
    #[doc = "Bit 7 - GPIO Port H Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d7(&mut self) -> D7_W {
        D7_W { w: self }
    }
    #[doc = "Bit 8 - GPIO Port J Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d8(&mut self) -> D8_W {
        D8_W { w: self }
    }
    #[doc = "Bit 9 - GPIO Port K Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d9(&mut self) -> D9_W {
        D9_W { w: self }
    }
    #[doc = "Bit 10 - GPIO Port L Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d10(&mut self) -> D10_W {
        D10_W { w: self }
    }
    #[doc = "Bit 11 - GPIO Port M Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d11(&mut self) -> D11_W {
        D11_W { w: self }
    }
    #[doc = "Bit 12 - GPIO Port N Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d12(&mut self) -> D12_W {
        D12_W { w: self }
    }
    #[doc = "Bit 13 - GPIO Port P Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d13(&mut self) -> D13_W {
        D13_W { w: self }
    }
    #[doc = "Bit 14 - GPIO Port Q Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn d14(&mut self) -> D14_W {
        D14_W { w: self }
    }
}
