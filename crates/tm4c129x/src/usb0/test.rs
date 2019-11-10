#[doc = "Reader of register TEST"]
pub type R = crate::R<u8, super::TEST>;
#[doc = "Writer for register TEST"]
pub type W = crate::W<u8, super::TEST>;
#[doc = "Register TEST `reset()`'s with value 0"]
impl crate::ResetValue for super::TEST {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TESTSE0NAK`"]
pub type TESTSE0NAK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TESTSE0NAK`"]
pub struct TESTSE0NAK_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTSE0NAK_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `TESTJ`"]
pub type TESTJ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TESTJ`"]
pub struct TESTJ_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTJ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TESTK`"]
pub type TESTK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TESTK`"]
pub struct TESTK_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TESTPKT`"]
pub type TESTPKT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TESTPKT`"]
pub struct TESTPKT_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTPKT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `FORCEHS`"]
pub type FORCEHS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCEHS`"]
pub struct FORCEHS_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEHS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `FORCEFS`"]
pub type FORCEFS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCEFS`"]
pub struct FORCEFS_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEFS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `FIFOACC`"]
pub type FIFOACC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFOACC`"]
pub struct FIFOACC_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOACC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `FORCEH`"]
pub type FORCEH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCEH`"]
pub struct FORCEH_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Test_SE0_NAK Test Mode Enable"]
    #[inline(always)]
    pub fn testse0nak(&self) -> TESTSE0NAK_R {
        TESTSE0NAK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Test_J Mode Enable"]
    #[inline(always)]
    pub fn testj(&self) -> TESTJ_R {
        TESTJ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Test_K Mode Enable"]
    #[inline(always)]
    pub fn testk(&self) -> TESTK_R {
        TESTK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Test Packet Mode Enable"]
    #[inline(always)]
    pub fn testpkt(&self) -> TESTPKT_R {
        TESTPKT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Force High-Speed Mode"]
    #[inline(always)]
    pub fn forcehs(&self) -> FORCEHS_R {
        FORCEHS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Force Full-Speed Mode"]
    #[inline(always)]
    pub fn forcefs(&self) -> FORCEFS_R {
        FORCEFS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FIFO Access"]
    #[inline(always)]
    pub fn fifoacc(&self) -> FIFOACC_R {
        FIFOACC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Force Host Mode"]
    #[inline(always)]
    pub fn forceh(&self) -> FORCEH_R {
        FORCEH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Test_SE0_NAK Test Mode Enable"]
    #[inline(always)]
    pub fn testse0nak(&mut self) -> TESTSE0NAK_W {
        TESTSE0NAK_W { w: self }
    }
    #[doc = "Bit 1 - Test_J Mode Enable"]
    #[inline(always)]
    pub fn testj(&mut self) -> TESTJ_W {
        TESTJ_W { w: self }
    }
    #[doc = "Bit 2 - Test_K Mode Enable"]
    #[inline(always)]
    pub fn testk(&mut self) -> TESTK_W {
        TESTK_W { w: self }
    }
    #[doc = "Bit 3 - Test Packet Mode Enable"]
    #[inline(always)]
    pub fn testpkt(&mut self) -> TESTPKT_W {
        TESTPKT_W { w: self }
    }
    #[doc = "Bit 4 - Force High-Speed Mode"]
    #[inline(always)]
    pub fn forcehs(&mut self) -> FORCEHS_W {
        FORCEHS_W { w: self }
    }
    #[doc = "Bit 5 - Force Full-Speed Mode"]
    #[inline(always)]
    pub fn forcefs(&mut self) -> FORCEFS_W {
        FORCEFS_W { w: self }
    }
    #[doc = "Bit 6 - FIFO Access"]
    #[inline(always)]
    pub fn fifoacc(&mut self) -> FIFOACC_W {
        FIFOACC_W { w: self }
    }
    #[doc = "Bit 7 - Force Host Mode"]
    #[inline(always)]
    pub fn forceh(&mut self) -> FORCEH_W {
        FORCEH_W { w: self }
    }
}
