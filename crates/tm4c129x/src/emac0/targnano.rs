#[doc = "Reader of register TARGNANO"]
pub type R = crate::R<u32, super::TARGNANO>;
#[doc = "Writer for register TARGNANO"]
pub type W = crate::W<u32, super::TARGNANO>;
#[doc = "Register TARGNANO `reset()`'s with value 0"]
impl crate::ResetValue for super::TARGNANO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TTSLO`"]
pub type TTSLO_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TTSLO`"]
pub struct TTSLO_W<'a> {
    w: &'a mut W,
}
impl<'a> TTSLO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | ((value as u32) & 0x7fff_ffff);
        self.w
    }
}
#[doc = "Reader of field `TRGTBUSY`"]
pub type TRGTBUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRGTBUSY`"]
pub struct TRGTBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGTBUSY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:30 - Target Timestamp Low Register"]
    #[inline(always)]
    pub fn ttslo(&self) -> TTSLO_R {
        TTSLO_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Target Time Register Busy"]
    #[inline(always)]
    pub fn trgtbusy(&self) -> TRGTBUSY_R {
        TRGTBUSY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Target Timestamp Low Register"]
    #[inline(always)]
    pub fn ttslo(&mut self) -> TTSLO_W {
        TTSLO_W { w: self }
    }
    #[doc = "Bit 31 - Target Time Register Busy"]
    #[inline(always)]
    pub fn trgtbusy(&mut self) -> TRGTBUSY_W {
        TRGTBUSY_W { w: self }
    }
}
