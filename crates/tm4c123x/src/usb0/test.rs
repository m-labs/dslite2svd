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
