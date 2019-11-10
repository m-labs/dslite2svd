#[doc = "Reader of register SOAR2"]
pub type R = crate::R<u32, super::SOAR2>;
#[doc = "Writer for register SOAR2"]
pub type W = crate::W<u32, super::SOAR2>;
#[doc = "Register SOAR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SOAR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OAR2`"]
pub type OAR2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OAR2`"]
pub struct OAR2_W<'a> {
    w: &'a mut W,
}
impl<'a> OAR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `OAR2EN`"]
pub type OAR2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OAR2EN`"]
pub struct OAR2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OAR2EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - I2C Slave Own Address 2"]
    #[inline(always)]
    pub fn oar2(&self) -> OAR2_R {
        OAR2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - I2C Slave Own Address 2 Enable"]
    #[inline(always)]
    pub fn oar2en(&self) -> OAR2EN_R {
        OAR2EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - I2C Slave Own Address 2"]
    #[inline(always)]
    pub fn oar2(&mut self) -> OAR2_W {
        OAR2_W { w: self }
    }
    #[doc = "Bit 7 - I2C Slave Own Address 2 Enable"]
    #[inline(always)]
    pub fn oar2en(&mut self) -> OAR2EN_W {
        OAR2EN_W { w: self }
    }
}
