#[doc = "Reader of register IO"]
pub type R = crate::R<u32, super::IO>;
#[doc = "Writer for register IO"]
pub type W = crate::W<u32, super::IO>;
#[doc = "Register IO `reset()`'s with value 0"]
impl crate::ResetValue for super::IO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WUUNLK`"]
pub type WUUNLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUUNLK`"]
pub struct WUUNLK_W<'a> {
    w: &'a mut W,
}
impl<'a> WUUNLK_W<'a> {
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
#[doc = "Reader of field `WURSTEN`"]
pub type WURSTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WURSTEN`"]
pub struct WURSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WURSTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `IOWRC`"]
pub type IOWRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOWRC`"]
pub struct IOWRC_W<'a> {
    w: &'a mut W,
}
impl<'a> IOWRC_W<'a> {
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
    #[doc = "Bit 0 - I/O Wake Pad Configuration Enable"]
    #[inline(always)]
    pub fn wuunlk(&self) -> WUUNLK_R {
        WUUNLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reset Wake Source Enable"]
    #[inline(always)]
    pub fn wursten(&self) -> WURSTEN_R {
        WURSTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 31 - I/O Write Complete"]
    #[inline(always)]
    pub fn iowrc(&self) -> IOWRC_R {
        IOWRC_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O Wake Pad Configuration Enable"]
    #[inline(always)]
    pub fn wuunlk(&mut self) -> WUUNLK_W {
        WUUNLK_W { w: self }
    }
    #[doc = "Bit 4 - Reset Wake Source Enable"]
    #[inline(always)]
    pub fn wursten(&mut self) -> WURSTEN_W {
        WURSTEN_W { w: self }
    }
    #[doc = "Bit 31 - I/O Write Complete"]
    #[inline(always)]
    pub fn iowrc(&mut self) -> IOWRC_W {
        IOWRC_W { w: self }
    }
}
