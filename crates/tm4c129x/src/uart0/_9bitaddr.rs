#[doc = "Reader of register _9BITADDR"]
pub type R = crate::R<u32, super::_9BITADDR>;
#[doc = "Writer for register _9BITADDR"]
pub type W = crate::W<u32, super::_9BITADDR>;
#[doc = "Register _9BITADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::_9BITADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `_9BITEN`"]
pub type _9BITEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `_9BITEN`"]
pub struct _9BITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> _9BITEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Self Address for 9-Bit Mode"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - Enable 9-Bit Mode"]
    #[inline(always)]
    pub fn _9biten(&self) -> _9BITEN_R {
        _9BITEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Self Address for 9-Bit Mode"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Bit 15 - Enable 9-Bit Mode"]
    #[inline(always)]
    pub fn _9biten(&mut self) -> _9BITEN_W {
        _9BITEN_W { w: self }
    }
}
