#[doc = "Reader of register GPCS"]
pub type R = crate::R<u32, super::GPCS>;
#[doc = "Writer for register GPCS"]
pub type W = crate::W<u32, super::GPCS>;
#[doc = "Register GPCS `reset()`'s with value 0"]
impl crate::ResetValue for super::GPCS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEVMOD`"]
pub type DEVMOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEVMOD`"]
pub struct DEVMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVMOD_W<'a> {
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
#[doc = "Reader of field `DEVMODOTG`"]
pub type DEVMODOTG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEVMODOTG`"]
pub struct DEVMODOTG_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVMODOTG_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Device Mode"]
    #[inline(always)]
    pub fn devmod(&self) -> DEVMOD_R {
        DEVMOD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Device Mode"]
    #[inline(always)]
    pub fn devmodotg(&self) -> DEVMODOTG_R {
        DEVMODOTG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device Mode"]
    #[inline(always)]
    pub fn devmod(&mut self) -> DEVMOD_W {
        DEVMOD_W { w: self }
    }
    #[doc = "Bit 1 - Enable Device Mode"]
    #[inline(always)]
    pub fn devmodotg(&mut self) -> DEVMODOTG_W {
        DEVMODOTG_W { w: self }
    }
}
