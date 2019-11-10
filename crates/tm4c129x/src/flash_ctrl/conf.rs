#[doc = "Reader of register CONF"]
pub type R = crate::R<u32, super::CONF>;
#[doc = "Writer for register CONF"]
pub type W = crate::W<u32, super::CONF>;
#[doc = "Register CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FPFOFF`"]
pub type FPFOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPFOFF`"]
pub struct FPFOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FPFOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `FPFON`"]
pub type FPFON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPFON`"]
pub struct FPFON_W<'a> {
    w: &'a mut W,
}
impl<'a> FPFON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `CLRTV`"]
pub type CLRTV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRTV`"]
pub struct CLRTV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRTV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `SPFE`"]
pub type SPFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPFE`"]
pub struct SPFE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPFE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `FMME`"]
pub type FMME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMME`"]
pub struct FMME_W<'a> {
    w: &'a mut W,
}
impl<'a> FMME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Force Prefetch Off"]
    #[inline(always)]
    pub fn fpfoff(&self) -> FPFOFF_R {
        FPFOFF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Force Prefetch On"]
    #[inline(always)]
    pub fn fpfon(&self) -> FPFON_R {
        FPFON_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Clear Valid Tags"]
    #[inline(always)]
    pub fn clrtv(&self) -> CLRTV_R {
        CLRTV_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Single Prefetch Mode Enable"]
    #[inline(always)]
    pub fn spfe(&self) -> SPFE_R {
        SPFE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Flash Mirror Mode Enable"]
    #[inline(always)]
    pub fn fmme(&self) -> FMME_R {
        FMME_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Force Prefetch Off"]
    #[inline(always)]
    pub fn fpfoff(&mut self) -> FPFOFF_W {
        FPFOFF_W { w: self }
    }
    #[doc = "Bit 17 - Force Prefetch On"]
    #[inline(always)]
    pub fn fpfon(&mut self) -> FPFON_W {
        FPFON_W { w: self }
    }
    #[doc = "Bit 20 - Clear Valid Tags"]
    #[inline(always)]
    pub fn clrtv(&mut self) -> CLRTV_W {
        CLRTV_W { w: self }
    }
    #[doc = "Bit 29 - Single Prefetch Mode Enable"]
    #[inline(always)]
    pub fn spfe(&mut self) -> SPFE_W {
        SPFE_W { w: self }
    }
    #[doc = "Bit 30 - Flash Mirror Mode Enable"]
    #[inline(always)]
    pub fn fmme(&mut self) -> FMME_W {
        FMME_W { w: self }
    }
}
