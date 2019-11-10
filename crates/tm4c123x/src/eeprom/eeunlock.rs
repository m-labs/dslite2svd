#[doc = "Reader of register EEUNLOCK"]
pub type R = crate::R<u32, super::EEUNLOCK>;
#[doc = "Writer for register EEUNLOCK"]
pub type W = crate::W<u32, super::EEUNLOCK>;
#[doc = "Register EEUNLOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::EEUNLOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UNLOCK`"]
pub type UNLOCK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `UNLOCK`"]
pub struct UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> UNLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - EEPROM Unlock"]
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - EEPROM Unlock"]
    #[inline(always)]
    pub fn unlock(&mut self) -> UNLOCK_W {
        UNLOCK_W { w: self }
    }
}
