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
#[doc = "Reader of field `FPIDCIM`"]
pub type FPIDCIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIDCIM`"]
pub struct FPIDCIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIDCIM_W<'a> {
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
#[doc = "Reader of field `FPDZCIM`"]
pub type FPDZCIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPDZCIM`"]
pub struct FPDZCIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FPDZCIM_W<'a> {
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
#[doc = "Reader of field `FPIOCIM`"]
pub type FPIOCIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIOCIM`"]
pub struct FPIOCIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIOCIM_W<'a> {
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
#[doc = "Reader of field `FPUFCIM`"]
pub type FPUFCIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPUFCIM`"]
pub struct FPUFCIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FPUFCIM_W<'a> {
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
#[doc = "Reader of field `FPOFCIM`"]
pub type FPOFCIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPOFCIM`"]
pub struct FPOFCIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FPOFCIM_W<'a> {
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
#[doc = "Reader of field `FPIXCIM`"]
pub type FPIXCIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIXCIM`"]
pub struct FPIXCIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIXCIM_W<'a> {
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
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Interrupt Mask"]
    #[inline(always)]
    pub fn fpidcim(&self) -> FPIDCIM_R {
        FPIDCIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Interrupt Mask"]
    #[inline(always)]
    pub fn fpdzcim(&self) -> FPDZCIM_R {
        FPDZCIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Interrupt Mask"]
    #[inline(always)]
    pub fn fpiocim(&self) -> FPIOCIM_R {
        FPIOCIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Interrupt Mask"]
    #[inline(always)]
    pub fn fpufcim(&self) -> FPUFCIM_R {
        FPUFCIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Interrupt Mask"]
    #[inline(always)]
    pub fn fpofcim(&self) -> FPOFCIM_R {
        FPOFCIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Interrupt Mask"]
    #[inline(always)]
    pub fn fpixcim(&self) -> FPIXCIM_R {
        FPIXCIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Interrupt Mask"]
    #[inline(always)]
    pub fn fpidcim(&mut self) -> FPIDCIM_W {
        FPIDCIM_W { w: self }
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Interrupt Mask"]
    #[inline(always)]
    pub fn fpdzcim(&mut self) -> FPDZCIM_W {
        FPDZCIM_W { w: self }
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Interrupt Mask"]
    #[inline(always)]
    pub fn fpiocim(&mut self) -> FPIOCIM_W {
        FPIOCIM_W { w: self }
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Interrupt Mask"]
    #[inline(always)]
    pub fn fpufcim(&mut self) -> FPUFCIM_W {
        FPUFCIM_W { w: self }
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Interrupt Mask"]
    #[inline(always)]
    pub fn fpofcim(&mut self) -> FPOFCIM_W {
        FPOFCIM_W { w: self }
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Interrupt Mask"]
    #[inline(always)]
    pub fn fpixcim(&mut self) -> FPIXCIM_W {
        FPIXCIM_W { w: self }
    }
}
