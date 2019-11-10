#[doc = "Reader of register IM"]
pub type R = crate::R<u32, super::IM>;
#[doc = "Writer for register IM"]
pub type W = crate::W<u32, super::IM>;
#[doc = "Register IM `reset()`'s with value 0"]
impl crate::ResetValue for super::IM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PMT`"]
pub type PMT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMT`"]
pub struct PMT_W<'a> {
    w: &'a mut W,
}
impl<'a> PMT_W<'a> {
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
#[doc = "Reader of field `TSI`"]
pub type TSI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSI`"]
pub struct TSI_W<'a> {
    w: &'a mut W,
}
impl<'a> TSI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - PMT Interrupt Mask"]
    #[inline(always)]
    pub fn pmt(&self) -> PMT_R {
        PMT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Mask"]
    #[inline(always)]
    pub fn tsi(&self) -> TSI_R {
        TSI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PMT Interrupt Mask"]
    #[inline(always)]
    pub fn pmt(&mut self) -> PMT_W {
        PMT_W { w: self }
    }
    #[doc = "Bit 9 - Timestamp Interrupt Mask"]
    #[inline(always)]
    pub fn tsi(&mut self) -> TSI_W {
        TSI_W { w: self }
    }
}
