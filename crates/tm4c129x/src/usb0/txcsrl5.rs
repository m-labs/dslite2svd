#[doc = "Reader of register TXCSRL5"]
pub type R = crate::R<u8, super::TXCSRL5>;
#[doc = "Writer for register TXCSRL5"]
pub type W = crate::W<u8, super::TXCSRL5>;
#[doc = "Register TXCSRL5 `reset()`'s with value 0"]
impl crate::ResetValue for super::TXCSRL5 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXRDY`"]
pub type TXRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXRDY`"]
pub struct TXRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRDY_W<'a> {
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
#[doc = "Reader of field `FIFONE`"]
pub type FIFONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFONE`"]
pub struct FIFONE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFONE_W<'a> {
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
#[doc = "Reader of field `ERROR`"]
pub type ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERROR`"]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `UNDRN`"]
pub type UNDRN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNDRN`"]
pub struct UNDRN_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDRN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `FLUSH`"]
pub type FLUSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLUSH`"]
pub struct FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUSH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SETUP`"]
pub type SETUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETUP`"]
pub struct SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `STALL`"]
pub type STALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STALL`"]
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `STALLED`"]
pub type STALLED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STALLED`"]
pub struct STALLED_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CLRDT`"]
pub type CLRDT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRDT`"]
pub struct CLRDT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRDT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `NAKTO`"]
pub type NAKTO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NAKTO`"]
pub struct NAKTO_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Packet Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FIFO Not Empty"]
    #[inline(always)]
    pub fn fifone(&self) -> FIFONE_R {
        FIFONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Error"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Underrun"]
    #[inline(always)]
    pub fn undrn(&self) -> UNDRN_R {
        UNDRN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Flush FIFO"]
    #[inline(always)]
    pub fn flush(&self) -> FLUSH_R {
        FLUSH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Setup Packet"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Send STALL"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint Stalled"]
    #[inline(always)]
    pub fn stalled(&self) -> STALLED_R {
        STALLED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Clear Data Toggle"]
    #[inline(always)]
    pub fn clrdt(&self) -> CLRDT_R {
        CLRDT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - NAK Timeout"]
    #[inline(always)]
    pub fn nakto(&self) -> NAKTO_R {
        NAKTO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Packet Ready"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TXRDY_W {
        TXRDY_W { w: self }
    }
    #[doc = "Bit 1 - FIFO Not Empty"]
    #[inline(always)]
    pub fn fifone(&mut self) -> FIFONE_W {
        FIFONE_W { w: self }
    }
    #[doc = "Bit 2 - Error"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
    #[doc = "Bit 2 - Underrun"]
    #[inline(always)]
    pub fn undrn(&mut self) -> UNDRN_W {
        UNDRN_W { w: self }
    }
    #[doc = "Bit 3 - Flush FIFO"]
    #[inline(always)]
    pub fn flush(&mut self) -> FLUSH_W {
        FLUSH_W { w: self }
    }
    #[doc = "Bit 4 - Setup Packet"]
    #[inline(always)]
    pub fn setup(&mut self) -> SETUP_W {
        SETUP_W { w: self }
    }
    #[doc = "Bit 4 - Send STALL"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    #[doc = "Bit 5 - Endpoint Stalled"]
    #[inline(always)]
    pub fn stalled(&mut self) -> STALLED_W {
        STALLED_W { w: self }
    }
    #[doc = "Bit 6 - Clear Data Toggle"]
    #[inline(always)]
    pub fn clrdt(&mut self) -> CLRDT_W {
        CLRDT_W { w: self }
    }
    #[doc = "Bit 7 - NAK Timeout"]
    #[inline(always)]
    pub fn nakto(&mut self) -> NAKTO_W {
        NAKTO_W { w: self }
    }
}
