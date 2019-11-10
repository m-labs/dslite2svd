#[doc = "Reader of register _2_INTEN"]
pub type R = crate::R<u32, super::_2_INTEN>;
#[doc = "Writer for register _2_INTEN"]
pub type W = crate::W<u32, super::_2_INTEN>;
#[doc = "Register _2_INTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::_2_INTEN {
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
#[doc = "Reader of field `TRCNTZERO`"]
pub type TRCNTZERO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRCNTZERO`"]
pub struct TRCNTZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> TRCNTZERO_W<'a> {
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
#[doc = "Reader of field `TRCNTLOAD`"]
pub type TRCNTLOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRCNTLOAD`"]
pub struct TRCNTLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRCNTLOAD_W<'a> {
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
#[doc = "Reader of field `TRCMPAU`"]
pub type TRCMPAU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRCMPAU`"]
pub struct TRCMPAU_W<'a> {
    w: &'a mut W,
}
impl<'a> TRCMPAU_W<'a> {
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
#[doc = "Reader of field `TRCMPAD`"]
pub type TRCMPAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRCMPAD`"]
pub struct TRCMPAD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRCMPAD_W<'a> {
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
#[doc = "Reader of field `TRCMPBU`"]
pub type TRCMPBU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRCMPBU`"]
pub struct TRCMPBU_W<'a> {
    w: &'a mut W,
}
impl<'a> TRCMPBU_W<'a> {
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
#[doc = "Reader of field `TRCMPBD`"]
pub type TRCMPBD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRCMPBD`"]
pub struct TRCMPBD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRCMPBD_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Interrupt for Counter=0"]
    #[inline(always)]
    pub fn intcntzero(&self) -> INTCNTZERO_R {
        INTCNTZERO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt for Counter=PWMnLOAD"]
    #[inline(always)]
    pub fn intcntload(&self) -> INTCNTLOAD_R {
        INTCNTLOAD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt for Counter=PWMnCMPA Up"]
    #[inline(always)]
    pub fn intcmpau(&self) -> INTCMPAU_R {
        INTCMPAU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt for Counter=PWMnCMPA Down"]
    #[inline(always)]
    pub fn intcmpad(&self) -> INTCMPAD_R {
        INTCMPAD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt for Counter=PWMnCMPB Up"]
    #[inline(always)]
    pub fn intcmpbu(&self) -> INTCMPBU_R {
        INTCMPBU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt for Counter=PWMnCMPB Down"]
    #[inline(always)]
    pub fn intcmpbd(&self) -> INTCMPBD_R {
        INTCMPBD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Trigger for Counter=0"]
    #[inline(always)]
    pub fn trcntzero(&self) -> TRCNTZERO_R {
        TRCNTZERO_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Trigger for Counter=PWMnLOAD"]
    #[inline(always)]
    pub fn trcntload(&self) -> TRCNTLOAD_R {
        TRCNTLOAD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Trigger for Counter=PWMnCMPA Up"]
    #[inline(always)]
    pub fn trcmpau(&self) -> TRCMPAU_R {
        TRCMPAU_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Trigger for Counter=PWMnCMPA Down"]
    #[inline(always)]
    pub fn trcmpad(&self) -> TRCMPAD_R {
        TRCMPAD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Trigger for Counter=PWMnCMPB Up"]
    #[inline(always)]
    pub fn trcmpbu(&self) -> TRCMPBU_R {
        TRCMPBU_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Trigger for Counter=PWMnCMPB Down"]
    #[inline(always)]
    pub fn trcmpbd(&self) -> TRCMPBD_R {
        TRCMPBD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt for Counter=0"]
    #[inline(always)]
    pub fn intcntzero(&mut self) -> INTCNTZERO_W {
        INTCNTZERO_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt for Counter=PWMnLOAD"]
    #[inline(always)]
    pub fn intcntload(&mut self) -> INTCNTLOAD_W {
        INTCNTLOAD_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt for Counter=PWMnCMPA Up"]
    #[inline(always)]
    pub fn intcmpau(&mut self) -> INTCMPAU_W {
        INTCMPAU_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt for Counter=PWMnCMPA Down"]
    #[inline(always)]
    pub fn intcmpad(&mut self) -> INTCMPAD_W {
        INTCMPAD_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt for Counter=PWMnCMPB Up"]
    #[inline(always)]
    pub fn intcmpbu(&mut self) -> INTCMPBU_W {
        INTCMPBU_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt for Counter=PWMnCMPB Down"]
    #[inline(always)]
    pub fn intcmpbd(&mut self) -> INTCMPBD_W {
        INTCMPBD_W { w: self }
    }
    #[doc = "Bit 8 - Trigger for Counter=0"]
    #[inline(always)]
    pub fn trcntzero(&mut self) -> TRCNTZERO_W {
        TRCNTZERO_W { w: self }
    }
    #[doc = "Bit 9 - Trigger for Counter=PWMnLOAD"]
    #[inline(always)]
    pub fn trcntload(&mut self) -> TRCNTLOAD_W {
        TRCNTLOAD_W { w: self }
    }
    #[doc = "Bit 10 - Trigger for Counter=PWMnCMPA Up"]
    #[inline(always)]
    pub fn trcmpau(&mut self) -> TRCMPAU_W {
        TRCMPAU_W { w: self }
    }
    #[doc = "Bit 11 - Trigger for Counter=PWMnCMPA Down"]
    #[inline(always)]
    pub fn trcmpad(&mut self) -> TRCMPAD_W {
        TRCMPAD_W { w: self }
    }
    #[doc = "Bit 12 - Trigger for Counter=PWMnCMPB Up"]
    #[inline(always)]
    pub fn trcmpbu(&mut self) -> TRCMPBU_W {
        TRCMPBU_W { w: self }
    }
    #[doc = "Bit 13 - Trigger for Counter=PWMnCMPB Down"]
    #[inline(always)]
    pub fn trcmpbd(&mut self) -> TRCMPBD_W {
        TRCMPBD_W { w: self }
    }
}
