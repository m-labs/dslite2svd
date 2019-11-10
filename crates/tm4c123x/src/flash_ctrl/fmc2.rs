#[doc = "Reader of register FMC2"]
pub type R = crate::R<u32, super::FMC2>;
#[doc = "Writer for register FMC2"]
pub type W = crate::W<u32, super::FMC2>;
#[doc = "Register FMC2 `reset()`'s with value 0"]
impl crate::ResetValue for super::FMC2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WRBUF`"]
pub type WRBUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRBUF`"]
pub struct WRBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> WRBUF_W<'a> {
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
#[doc = "Reader of field `WRKEY`"]
pub type WRKEY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WRKEY`"]
pub struct WRKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> WRKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 17)) | (((value as u32) & 0x7fff) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Buffered Flash Memory Write"]
    #[inline(always)]
    pub fn wrbuf(&self) -> WRBUF_R {
        WRBUF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 17:31 - FLASH write key"]
    #[inline(always)]
    pub fn wrkey(&self) -> WRKEY_R {
        WRKEY_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Buffered Flash Memory Write"]
    #[inline(always)]
    pub fn wrbuf(&mut self) -> WRBUF_W {
        WRBUF_W { w: self }
    }
    #[doc = "Bits 17:31 - FLASH write key"]
    #[inline(always)]
    pub fn wrkey(&mut self) -> WRKEY_W {
        WRKEY_W { w: self }
    }
}
