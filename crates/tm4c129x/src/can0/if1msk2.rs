#[doc = "Reader of register IF1MSK2"]
pub type R = crate::R<u32, super::IF1MSK2>;
#[doc = "Writer for register IF1MSK2"]
pub type W = crate::W<u32, super::IF1MSK2>;
#[doc = "Register IF1MSK2 `reset()`'s with value 0"]
impl crate::ResetValue for super::IF1MSK2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDMSK`"]
pub type IDMSK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IDMSK`"]
pub struct IDMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
#[doc = "Reader of field `MDIR`"]
pub type MDIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MDIR`"]
pub struct MDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `MXTD`"]
pub type MXTD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MXTD`"]
pub struct MXTD_W<'a> {
    w: &'a mut W,
}
impl<'a> MXTD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - Identifier Mask"]
    #[inline(always)]
    pub fn idmsk(&self) -> IDMSK_R {
        IDMSK_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 14 - Mask Message Direction"]
    #[inline(always)]
    pub fn mdir(&self) -> MDIR_R {
        MDIR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Mask Extended Identifier"]
    #[inline(always)]
    pub fn mxtd(&self) -> MXTD_R {
        MXTD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - Identifier Mask"]
    #[inline(always)]
    pub fn idmsk(&mut self) -> IDMSK_W {
        IDMSK_W { w: self }
    }
    #[doc = "Bit 14 - Mask Message Direction"]
    #[inline(always)]
    pub fn mdir(&mut self) -> MDIR_W {
        MDIR_W { w: self }
    }
    #[doc = "Bit 15 - Mask Extended Identifier"]
    #[inline(always)]
    pub fn mxtd(&mut self) -> MXTD_W {
        MXTD_W { w: self }
    }
}
