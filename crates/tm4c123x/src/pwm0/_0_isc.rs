#[doc = "Reader of register _0_ISC"]
pub type R = crate::R<u32, super::_0_ISC>;
#[doc = "Writer for register _0_ISC"]
pub type W = crate::W<u32, super::_0_ISC>;
#[doc = "Register _0_ISC `reset()`'s with value 0"]
impl crate::ResetValue for super::_0_ISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTCNTZERO`"]
pub type INTCNTZERO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTCNTZERO`"]
pub struct INTCNTZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCNTZERO_W<'a> {
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
#[doc = "Reader of field `INTCNTLOAD`"]
pub type INTCNTLOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTCNTLOAD`"]
pub struct INTCNTLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCNTLOAD_W<'a> {
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
#[doc = "Reader of field `INTCMPAU`"]
pub type INTCMPAU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTCMPAU`"]
pub struct INTCMPAU_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCMPAU_W<'a> {
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
#[doc = "Reader of field `INTCMPAD`"]
pub type INTCMPAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTCMPAD`"]
pub struct INTCMPAD_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCMPAD_W<'a> {
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
#[doc = "Reader of field `INTCMPBU`"]
pub type INTCMPBU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTCMPBU`"]
pub struct INTCMPBU_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCMPBU_W<'a> {
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
#[doc = "Reader of field `INTCMPBD`"]
pub type INTCMPBD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTCMPBD`"]
pub struct INTCMPBD_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCMPBD_W<'a> {
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
    #[doc = "Bit 0 - Counter=0 Interrupt"]
    #[inline(always)]
    pub fn intcntzero(&self) -> INTCNTZERO_R {
        INTCNTZERO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counter=Load Interrupt"]
    #[inline(always)]
    pub fn intcntload(&self) -> INTCNTLOAD_R {
        INTCNTLOAD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comparator A Up Interrupt"]
    #[inline(always)]
    pub fn intcmpau(&self) -> INTCMPAU_R {
        INTCMPAU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Comparator A Down Interrupt"]
    #[inline(always)]
    pub fn intcmpad(&self) -> INTCMPAD_R {
        INTCMPAD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comparator B Up Interrupt"]
    #[inline(always)]
    pub fn intcmpbu(&self) -> INTCMPBU_R {
        INTCMPBU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Comparator B Down Interrupt"]
    #[inline(always)]
    pub fn intcmpbd(&self) -> INTCMPBD_R {
        INTCMPBD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter=0 Interrupt"]
    #[inline(always)]
    pub fn intcntzero(&mut self) -> INTCNTZERO_W {
        INTCNTZERO_W { w: self }
    }
    #[doc = "Bit 1 - Counter=Load Interrupt"]
    #[inline(always)]
    pub fn intcntload(&mut self) -> INTCNTLOAD_W {
        INTCNTLOAD_W { w: self }
    }
    #[doc = "Bit 2 - Comparator A Up Interrupt"]
    #[inline(always)]
    pub fn intcmpau(&mut self) -> INTCMPAU_W {
        INTCMPAU_W { w: self }
    }
    #[doc = "Bit 3 - Comparator A Down Interrupt"]
    #[inline(always)]
    pub fn intcmpad(&mut self) -> INTCMPAD_W {
        INTCMPAD_W { w: self }
    }
    #[doc = "Bit 4 - Comparator B Up Interrupt"]
    #[inline(always)]
    pub fn intcmpbu(&mut self) -> INTCMPBU_W {
        INTCMPBU_W { w: self }
    }
    #[doc = "Bit 5 - Comparator B Down Interrupt"]
    #[inline(always)]
    pub fn intcmpbd(&mut self) -> INTCMPBD_W {
        INTCMPBD_W { w: self }
    }
}
