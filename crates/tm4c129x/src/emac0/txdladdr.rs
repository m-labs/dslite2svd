#[doc = "Reader of register TXDLADDR"]
pub type R = crate::R<u32, super::TXDLADDR>;
#[doc = "Writer for register TXDLADDR"]
pub type W = crate::W<u32, super::TXDLADDR>;
#[doc = "Register TXDLADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::TXDLADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXDLADDR`"]
pub type TXDLADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TXDLADDR`"]
pub struct TXDLADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDLADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Start of Transmit List Base Address"]
    #[inline(always)]
    pub fn txdladdr(&self) -> TXDLADDR_R {
        TXDLADDR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start of Transmit List Base Address"]
    #[inline(always)]
    pub fn txdladdr(&mut self) -> TXDLADDR_W {
        TXDLADDR_W { w: self }
    }
}
