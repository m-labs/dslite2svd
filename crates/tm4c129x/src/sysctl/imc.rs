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
#[doc = "Reader of field `BORIM`"]
pub type BORIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BORIM`"]
pub struct BORIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BORIM_W<'a> {
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
impl R {
    #[doc = "Bit 1 - Brown-Out Reset Interrupt Mask"]
    #[inline(always)]
    pub fn borim(&self) -> BORIM_R {
        BORIM_R::new(((self.bits >> 1) & 0x01) != 0)
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
    #[doc = "Bit 8 - MOSC Power Up Interrupt Mask"]
    #[inline(always)]
    pub fn moscpupim(&self) -> MOSCPUPIM_R {
        MOSCPUPIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Brown-Out Reset Interrupt Mask"]
    #[inline(always)]
    pub fn borim(&mut self) -> BORIM_W {
        BORIM_W { w: self }
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
    #[doc = "Bit 8 - MOSC Power Up Interrupt Mask"]
    #[inline(always)]
    pub fn moscpupim(&mut self) -> MOSCPUPIM_W {
        MOSCPUPIM_W { w: self }
    }
}
