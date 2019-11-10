#[doc = "Reader of register CCONF"]
pub type R = crate::R<u8, super::CCONF>;
#[doc = "Writer for register CCONF"]
pub type W = crate::W<u8, super::CCONF>;
#[doc = "Register CCONF `reset()`'s with value 0"]
impl crate::ResetValue for super::CCONF {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXEDMA`"]
pub type RXEDMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXEDMA`"]
pub struct RXEDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEDMA_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `TXEDMA`"]
pub type TXEDMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXEDMA`"]
pub struct TXEDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEDMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TX Early DMA Enable"]
    #[inline(always)]
    pub fn rxedma(&self) -> RXEDMA_R {
        RXEDMA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TX Early DMA Enable"]
    #[inline(always)]
    pub fn txedma(&self) -> TXEDMA_R {
        TXEDMA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX Early DMA Enable"]
    #[inline(always)]
    pub fn rxedma(&mut self) -> RXEDMA_W {
        RXEDMA_W { w: self }
    }
    #[doc = "Bit 1 - TX Early DMA Enable"]
    #[inline(always)]
    pub fn txedma(&mut self) -> TXEDMA_W {
        TXEDMA_W { w: self }
    }
}
