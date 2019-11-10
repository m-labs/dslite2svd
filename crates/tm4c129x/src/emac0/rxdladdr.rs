#[doc = "Reader of register RXDLADDR"]
pub type R = crate::R<u32, super::RXDLADDR>;
#[doc = "Writer for register RXDLADDR"]
pub type W = crate::W<u32, super::RXDLADDR>;
#[doc = "Register RXDLADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::RXDLADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STRXLIST`"]
pub type STRXLIST_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STRXLIST`"]
pub struct STRXLIST_W<'a> {
    w: &'a mut W,
}
impl<'a> STRXLIST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Start of Receive List"]
    #[inline(always)]
    pub fn strxlist(&self) -> STRXLIST_R {
        STRXLIST_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start of Receive List"]
    #[inline(always)]
    pub fn strxlist(&mut self) -> STRXLIST_W {
        STRXLIST_W { w: self }
    }
}
