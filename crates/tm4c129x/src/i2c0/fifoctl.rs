#[doc = "Reader of register FIFOCTL"]
pub type R = crate::R<u32, super::FIFOCTL>;
#[doc = "Writer for register FIFOCTL"]
pub type W = crate::W<u32, super::FIFOCTL>;
#[doc = "Register FIFOCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFOCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXTRIG`"]
pub type TXTRIG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXTRIG`"]
pub struct TXTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTRIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `DMATXENA`"]
pub type DMATXENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMATXENA`"]
pub struct DMATXENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATXENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `TXFLUSH`"]
pub type TXFLUSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFLUSH`"]
pub struct TXFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFLUSH_W<'a> {
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
#[doc = "Reader of field `TXASGNMT`"]
pub type TXASGNMT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXASGNMT`"]
pub struct TXASGNMT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXASGNMT_W<'a> {
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
#[doc = "Reader of field `RXTRIG`"]
pub type RXTRIG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXTRIG`"]
pub struct RXTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTRIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `DMARXENA`"]
pub type DMARXENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMARXENA`"]
pub struct DMARXENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARXENA_W<'a> {
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
#[doc = "Reader of field `RXFLUSH`"]
pub type RXFLUSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFLUSH`"]
pub struct RXFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFLUSH_W<'a> {
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
#[doc = "Reader of field `RXASGNMT`"]
pub type RXASGNMT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXASGNMT`"]
pub struct RXASGNMT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXASGNMT_W<'a> {
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
    #[doc = "Bits 0:2 - TX FIFO Trigger"]
    #[inline(always)]
    pub fn txtrig(&self) -> TXTRIG_R {
        TXTRIG_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 13 - DMA TX Channel Enable"]
    #[inline(always)]
    pub fn dmatxena(&self) -> DMATXENA_R {
        DMATXENA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TX FIFO Flush"]
    #[inline(always)]
    pub fn txflush(&self) -> TXFLUSH_R {
        TXFLUSH_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TX Control Assignment"]
    #[inline(always)]
    pub fn txasgnmt(&self) -> TXASGNMT_R {
        TXASGNMT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - RX FIFO Trigger"]
    #[inline(always)]
    pub fn rxtrig(&self) -> RXTRIG_R {
        RXTRIG_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 29 - DMA RX Channel Enable"]
    #[inline(always)]
    pub fn dmarxena(&self) -> DMARXENA_R {
        DMARXENA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - RX FIFO Flush"]
    #[inline(always)]
    pub fn rxflush(&self) -> RXFLUSH_R {
        RXFLUSH_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - RX Control Assignment"]
    #[inline(always)]
    pub fn rxasgnmt(&self) -> RXASGNMT_R {
        RXASGNMT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TX FIFO Trigger"]
    #[inline(always)]
    pub fn txtrig(&mut self) -> TXTRIG_W {
        TXTRIG_W { w: self }
    }
    #[doc = "Bit 13 - DMA TX Channel Enable"]
    #[inline(always)]
    pub fn dmatxena(&mut self) -> DMATXENA_W {
        DMATXENA_W { w: self }
    }
    #[doc = "Bit 14 - TX FIFO Flush"]
    #[inline(always)]
    pub fn txflush(&mut self) -> TXFLUSH_W {
        TXFLUSH_W { w: self }
    }
    #[doc = "Bit 15 - TX Control Assignment"]
    #[inline(always)]
    pub fn txasgnmt(&mut self) -> TXASGNMT_W {
        TXASGNMT_W { w: self }
    }
    #[doc = "Bits 16:18 - RX FIFO Trigger"]
    #[inline(always)]
    pub fn rxtrig(&mut self) -> RXTRIG_W {
        RXTRIG_W { w: self }
    }
    #[doc = "Bit 29 - DMA RX Channel Enable"]
    #[inline(always)]
    pub fn dmarxena(&mut self) -> DMARXENA_W {
        DMARXENA_W { w: self }
    }
    #[doc = "Bit 30 - RX FIFO Flush"]
    #[inline(always)]
    pub fn rxflush(&mut self) -> RXFLUSH_W {
        RXFLUSH_W { w: self }
    }
    #[doc = "Bit 31 - RX Control Assignment"]
    #[inline(always)]
    pub fn rxasgnmt(&mut self) -> RXASGNMT_W {
        RXASGNMT_W { w: self }
    }
}
