#[doc = "Reader of register IMC"]
pub type R = crate::R<u32, super::IMC>;
#[doc = "Writer for register IMC"]
pub type W = crate::W<u32, super::IMC>;
#[doc = "Register IMC `reset()`'s with value 0"]
impl crate::ResetValue for super::IMC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BOR1IM`"]
pub type BOR1IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOR1IM`"]
pub struct BOR1IM_W<'a> {
    w: &'a mut W,
}
impl<'a> BOR1IM_W<'a> {
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
#[doc = "Reader of field `MOFIM`"]
pub type MOFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOFIM`"]
pub struct MOFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MOFIM_W<'a> {
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
#[doc = "Reader of field `PLLLIM`"]
pub type PLLLIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLLIM`"]
pub struct PLLLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLLIM_W<'a> {
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
#[doc = "Reader of field `USBPLLLIM`"]
pub type USBPLLLIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBPLLLIM`"]
pub struct USBPLLLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPLLLIM_W<'a> {
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
#[doc = "Reader of field `MOSCPUPIM`"]
pub type MOSCPUPIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOSCPUPIM`"]
pub struct MOSCPUPIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCPUPIM_W<'a> {
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
#[doc = "Reader of field `VDDAIM`"]
pub type VDDAIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDDAIM`"]
pub struct VDDAIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDAIM_W<'a> {
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
#[doc = "Reader of field `BOR0IM`"]
pub type BOR0IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOR0IM`"]
pub struct BOR0IM_W<'a> {
    w: &'a mut W,
}
impl<'a> BOR0IM_W<'a> {
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
impl R {
    #[doc = "Bit 1 - VDD under BOR1 Interrupt Mask"]
    #[inline(always)]
    pub fn bor1im(&self) -> BOR1IM_R {
        BOR1IM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Main Oscillator Failure Interrupt Mask"]
    #[inline(always)]
    pub fn mofim(&self) -> MOFIM_R {
        MOFIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL Lock Interrupt Mask"]
    #[inline(always)]
    pub fn plllim(&self) -> PLLLIM_R {
        PLLLIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB PLL Lock Interrupt Mask"]
    #[inline(always)]
    pub fn usbplllim(&self) -> USBPLLLIM_R {
        USBPLLLIM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MOSC Power Up Interrupt Mask"]
    #[inline(always)]
    pub fn moscpupim(&self) -> MOSCPUPIM_R {
        MOSCPUPIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - VDDA Power OK Interrupt Mask"]
    #[inline(always)]
    pub fn vddaim(&self) -> VDDAIM_R {
        VDDAIM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - VDD under BOR0 Interrupt Mask"]
    #[inline(always)]
    pub fn bor0im(&self) -> BOR0IM_R {
        BOR0IM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - VDD under BOR1 Interrupt Mask"]
    #[inline(always)]
    pub fn bor1im(&mut self) -> BOR1IM_W {
        BOR1IM_W { w: self }
    }
    #[doc = "Bit 3 - Main Oscillator Failure Interrupt Mask"]
    #[inline(always)]
    pub fn mofim(&mut self) -> MOFIM_W {
        MOFIM_W { w: self }
    }
    #[doc = "Bit 6 - PLL Lock Interrupt Mask"]
    #[inline(always)]
    pub fn plllim(&mut self) -> PLLLIM_W {
        PLLLIM_W { w: self }
    }
    #[doc = "Bit 7 - USB PLL Lock Interrupt Mask"]
    #[inline(always)]
    pub fn usbplllim(&mut self) -> USBPLLLIM_W {
        USBPLLLIM_W { w: self }
    }
    #[doc = "Bit 8 - MOSC Power Up Interrupt Mask"]
    #[inline(always)]
    pub fn moscpupim(&mut self) -> MOSCPUPIM_W {
        MOSCPUPIM_W { w: self }
    }
    #[doc = "Bit 10 - VDDA Power OK Interrupt Mask"]
    #[inline(always)]
    pub fn vddaim(&mut self) -> VDDAIM_W {
        VDDAIM_W { w: self }
    }
    #[doc = "Bit 11 - VDD under BOR0 Interrupt Mask"]
    #[inline(always)]
    pub fn bor0im(&mut self) -> BOR0IM_W {
        BOR0IM_W { w: self }
    }
}
