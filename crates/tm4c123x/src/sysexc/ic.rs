#[doc = "Writer for register IC"]
pub type W = crate::W<u32, super::IC>;
#[doc = "Register IC `reset()`'s with value 0"]
impl crate::ResetValue for super::IC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `FPIDCIC`"]
pub struct FPIDCIC_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIDCIC_W<'a> {
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
#[doc = "Write proxy for field `FPDZCIC`"]
pub struct FPDZCIC_W<'a> {
    w: &'a mut W,
}
impl<'a> FPDZCIC_W<'a> {
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
#[doc = "Write proxy for field `FPIOCIC`"]
pub struct FPIOCIC_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIOCIC_W<'a> {
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
#[doc = "Write proxy for field `FPUFCIC`"]
pub struct FPUFCIC_W<'a> {
    w: &'a mut W,
}
impl<'a> FPUFCIC_W<'a> {
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
#[doc = "Write proxy for field `FPOFCIC`"]
pub struct FPOFCIC_W<'a> {
    w: &'a mut W,
}
impl<'a> FPOFCIC_W<'a> {
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
#[doc = "Write proxy for field `FPIXCIC`"]
pub struct FPIXCIC_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIXCIC_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Interrupt Clear"]
    #[inline(always)]
    pub fn fpidcic(&mut self) -> FPIDCIC_W {
        FPIDCIC_W { w: self }
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Interrupt Clear"]
    #[inline(always)]
    pub fn fpdzcic(&mut self) -> FPDZCIC_W {
        FPDZCIC_W { w: self }
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Interrupt Clear"]
    #[inline(always)]
    pub fn fpiocic(&mut self) -> FPIOCIC_W {
        FPIOCIC_W { w: self }
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Interrupt Clear"]
    #[inline(always)]
    pub fn fpufcic(&mut self) -> FPUFCIC_W {
        FPUFCIC_W { w: self }
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Interrupt Clear"]
    #[inline(always)]
    pub fn fpofcic(&mut self) -> FPOFCIC_W {
        FPOFCIC_W { w: self }
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Interrupt Clear"]
    #[inline(always)]
    pub fn fpixcic(&mut self) -> FPIXCIC_W {
        FPIXCIC_W { w: self }
    }
}
