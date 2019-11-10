#[doc = "Reader of register FMC"]
pub type R = crate::R<u32, super::FMC>;
#[doc = "Writer for register FMC"]
pub type W = crate::W<u32, super::FMC>;
#[doc = "Register FMC `reset()`'s with value 0"]
impl crate::ResetValue for super::FMC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WRITE`"]
pub type WRITE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRITE`"]
pub struct WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_W<'a> {
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
#[doc = "Reader of field `ERASE`"]
pub type ERASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERASE`"]
pub struct ERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_W<'a> {
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
#[doc = "Reader of field `MERASE`"]
pub type MERASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MERASE`"]
pub struct MERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> MERASE_W<'a> {
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
#[doc = "Reader of field `COMT`"]
pub type COMT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMT`"]
pub struct COMT_W<'a> {
    w: &'a mut W,
}
impl<'a> COMT_W<'a> {
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
    #[doc = "Bit 0 - Write a Word into Flash Memory"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Erase a Page of Flash Memory"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mass Erase Flash Memory"]
    #[inline(always)]
    pub fn merase(&self) -> MERASE_R {
        MERASE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Commit Register Value"]
    #[inline(always)]
    pub fn comt(&self) -> COMT_R {
        COMT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 17:31 - FLASH write key"]
    #[inline(always)]
    pub fn wrkey(&self) -> WRKEY_R {
        WRKEY_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Write a Word into Flash Memory"]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W {
        WRITE_W { w: self }
    }
    #[doc = "Bit 1 - Erase a Page of Flash Memory"]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W {
        ERASE_W { w: self }
    }
    #[doc = "Bit 2 - Mass Erase Flash Memory"]
    #[inline(always)]
    pub fn merase(&mut self) -> MERASE_W {
        MERASE_W { w: self }
    }
    #[doc = "Bit 3 - Commit Register Value"]
    #[inline(always)]
    pub fn comt(&mut self) -> COMT_W {
        COMT_W { w: self }
    }
    #[doc = "Bits 17:31 - FLASH write key"]
    #[inline(always)]
    pub fn wrkey(&mut self) -> WRKEY_W {
        WRKEY_W { w: self }
    }
}
