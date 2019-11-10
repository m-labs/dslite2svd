#[doc = "Reader of register HB8TIME4"]
pub type R = crate::R<u32, super::HB8TIME4>;
#[doc = "Writer for register HB8TIME4"]
pub type W = crate::W<u32, super::HB8TIME4>;
#[doc = "Register HB8TIME4 `reset()`'s with value 0"]
impl crate::ResetValue for super::HB8TIME4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDWSM`"]
pub type RDWSM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDWSM`"]
pub struct RDWSM_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWSM_W<'a> {
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
#[doc = "Reader of field `WRWSM`"]
pub type WRWSM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRWSM`"]
pub struct WRWSM_W<'a> {
    w: &'a mut W,
}
impl<'a> WRWSM_W<'a> {
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
#[doc = "Reader of field `CAPWIDTH`"]
pub type CAPWIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAPWIDTH`"]
pub struct CAPWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPWIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `IRDYDLY`"]
pub type IRDYDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IRDYDLY`"]
pub struct IRDYDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDYDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CS3n Read Wait State Minus One"]
    #[inline(always)]
    pub fn rdwsm(&self) -> RDWSM_R {
        RDWSM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CS3n Write Wait State Minus One"]
    #[inline(always)]
    pub fn wrwsm(&self) -> WRWSM_R {
        WRWSM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - CS3n Inter-transfer Capture Width"]
    #[inline(always)]
    pub fn capwidth(&self) -> CAPWIDTH_R {
        CAPWIDTH_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - CS3n Input Ready Delay"]
    #[inline(always)]
    pub fn irdydly(&self) -> IRDYDLY_R {
        IRDYDLY_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CS3n Read Wait State Minus One"]
    #[inline(always)]
    pub fn rdwsm(&mut self) -> RDWSM_W {
        RDWSM_W { w: self }
    }
    #[doc = "Bit 4 - CS3n Write Wait State Minus One"]
    #[inline(always)]
    pub fn wrwsm(&mut self) -> WRWSM_W {
        WRWSM_W { w: self }
    }
    #[doc = "Bits 12:13 - CS3n Inter-transfer Capture Width"]
    #[inline(always)]
    pub fn capwidth(&mut self) -> CAPWIDTH_W {
        CAPWIDTH_W { w: self }
    }
    #[doc = "Bits 24:25 - CS3n Input Ready Delay"]
    #[inline(always)]
    pub fn irdydly(&mut self) -> IRDYDLY_W {
        IRDYDLY_W { w: self }
    }
}
