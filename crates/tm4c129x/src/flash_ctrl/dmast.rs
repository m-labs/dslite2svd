#[doc = "Reader of register DMAST"]
pub type R = crate::R<u32, super::DMAST>;
#[doc = "Writer for register DMAST"]
pub type W = crate::W<u32, super::DMAST>;
#[doc = "Register DMAST `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 11)) | (((value as u32) & 0x0003_ffff) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:28 - Contains the starting address of the flash region accessible by uDMA if the FLASHPP register DFA bit is set"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 11) & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 11:28 - Contains the starting address of the flash region accessible by uDMA if the FLASHPP register DFA bit is set"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
}
