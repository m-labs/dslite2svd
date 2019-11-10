#[doc = "Reader of register RXIE"]
pub type R = crate::R<u16, super::RXIE>;
#[doc = "Writer for register RXIE"]
pub type W = crate::W<u16, super::RXIE>;
#[doc = "Register RXIE `reset()`'s with value 0"]
impl crate::ResetValue for super::RXIE {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EP1`"]
pub type EP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP1`"]
pub struct EP1_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `EP2`"]
pub type EP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP2`"]
pub struct EP2_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `EP3`"]
pub type EP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP3`"]
pub struct EP3_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `EP4`"]
pub type EP4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP4`"]
pub struct EP4_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `EP5`"]
pub type EP5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP5`"]
pub struct EP5_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `EP6`"]
pub type EP6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP6`"]
pub struct EP6_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `EP7`"]
pub type EP7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP7`"]
pub struct EP7_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - RX Endpoint 1 Interrupt Enable"]
    #[inline(always)]
    pub fn ep1(&self) -> EP1_R {
        EP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX Endpoint 2 Interrupt Enable"]
    #[inline(always)]
    pub fn ep2(&self) -> EP2_R {
        EP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RX Endpoint 3 Interrupt Enable"]
    #[inline(always)]
    pub fn ep3(&self) -> EP3_R {
        EP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RX Endpoint 4 Interrupt Enable"]
    #[inline(always)]
    pub fn ep4(&self) -> EP4_R {
        EP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RX Endpoint 5 Interrupt Enable"]
    #[inline(always)]
    pub fn ep5(&self) -> EP5_R {
        EP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RX Endpoint 6 Interrupt Enable"]
    #[inline(always)]
    pub fn ep6(&self) -> EP6_R {
        EP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RX Endpoint 7 Interrupt Enable"]
    #[inline(always)]
    pub fn ep7(&self) -> EP7_R {
        EP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RX Endpoint 1 Interrupt Enable"]
    #[inline(always)]
    pub fn ep1(&mut self) -> EP1_W {
        EP1_W { w: self }
    }
    #[doc = "Bit 2 - RX Endpoint 2 Interrupt Enable"]
    #[inline(always)]
    pub fn ep2(&mut self) -> EP2_W {
        EP2_W { w: self }
    }
    #[doc = "Bit 3 - RX Endpoint 3 Interrupt Enable"]
    #[inline(always)]
    pub fn ep3(&mut self) -> EP3_W {
        EP3_W { w: self }
    }
    #[doc = "Bit 4 - RX Endpoint 4 Interrupt Enable"]
    #[inline(always)]
    pub fn ep4(&mut self) -> EP4_W {
        EP4_W { w: self }
    }
    #[doc = "Bit 5 - RX Endpoint 5 Interrupt Enable"]
    #[inline(always)]
    pub fn ep5(&mut self) -> EP5_W {
        EP5_W { w: self }
    }
    #[doc = "Bit 6 - RX Endpoint 6 Interrupt Enable"]
    #[inline(always)]
    pub fn ep6(&mut self) -> EP6_W {
        EP6_W { w: self }
    }
    #[doc = "Bit 7 - RX Endpoint 7 Interrupt Enable"]
    #[inline(always)]
    pub fn ep7(&mut self) -> EP7_W {
        EP7_W { w: self }
    }
}
