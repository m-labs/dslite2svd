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
#[doc = "Reader of field `RIMIM`"]
pub type RIMIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RIMIM`"]
pub struct RIMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RIMIM_W<'a> {
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
#[doc = "Reader of field `CTSMIM`"]
pub type CTSMIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTSMIM`"]
pub struct CTSMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSMIM_W<'a> {
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
#[doc = "Reader of field `DCDMIM`"]
pub type DCDMIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDMIM`"]
pub struct DCDMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDMIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `DSRMIM`"]
pub type DSRMIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSRMIM`"]
pub struct DSRMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRMIM_W<'a> {
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
#[doc = "Reader of field `RXIM`"]
pub type RXIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIM`"]
pub struct RXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIM_W<'a> {
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
#[doc = "Reader of field `TXIM`"]
pub type TXIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXIM`"]
pub struct TXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RTIM`"]
pub type RTIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTIM`"]
pub struct RTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `FEIM`"]
pub type FEIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEIM`"]
pub struct FEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIM_W<'a> {
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
#[doc = "Reader of field `PEIM`"]
pub type PEIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEIM`"]
pub struct PEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PEIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `BEIM`"]
pub type BEIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEIM`"]
pub struct BEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIM_W<'a> {
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
#[doc = "Reader of field `OEIM`"]
pub type OEIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OEIM`"]
pub struct OEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> OEIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `EOTIM`"]
pub type EOTIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOTIM`"]
pub struct EOTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> EOTIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `_9BITIM`"]
pub type _9BITIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `_9BITIM`"]
pub struct _9BITIM_W<'a> {
    w: &'a mut W,
}
impl<'a> _9BITIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `DMARXIM`"]
pub type DMARXIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMARXIM`"]
pub struct DMARXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARXIM_W<'a> {
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
#[doc = "Reader of field `DMATXIM`"]
pub type DMATXIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMATXIM`"]
pub struct DMATXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATXIM_W<'a> {
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
impl R {
    #[doc = "Bit 0 - UART Ring Indicator Modem Interrupt Mask"]
    #[inline(always)]
    pub fn rimim(&self) -> RIMIM_R {
        RIMIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UART Clear to Send Modem Interrupt Mask"]
    #[inline(always)]
    pub fn ctsmim(&self) -> CTSMIM_R {
        CTSMIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - UART Data Carrier Detect Modem Interrupt Mask"]
    #[inline(always)]
    pub fn dcdmim(&self) -> DCDMIM_R {
        DCDMIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - UART Data Set Ready Modem Interrupt Mask"]
    #[inline(always)]
    pub fn dsrmim(&self) -> DSRMIM_R {
        DSRMIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UART Receive Interrupt Mask"]
    #[inline(always)]
    pub fn rxim(&self) -> RXIM_R {
        RXIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UART Transmit Interrupt Mask"]
    #[inline(always)]
    pub fn txim(&self) -> TXIM_R {
        TXIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UART Receive Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UART Framing Error Interrupt Mask"]
    #[inline(always)]
    pub fn feim(&self) -> FEIM_R {
        FEIM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UART Parity Error Interrupt Mask"]
    #[inline(always)]
    pub fn peim(&self) -> PEIM_R {
        PEIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UART Break Error Interrupt Mask"]
    #[inline(always)]
    pub fn beim(&self) -> BEIM_R {
        BEIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UART Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn oeim(&self) -> OEIM_R {
        OEIM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - End of Transmission Interrupt Mask"]
    #[inline(always)]
    pub fn eotim(&self) -> EOTIM_R {
        EOTIM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 9-Bit Mode Interrupt Mask"]
    #[inline(always)]
    pub fn _9bitim(&self) -> _9BITIM_R {
        _9BITIM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Receive DMA Interrupt Mask"]
    #[inline(always)]
    pub fn dmarxim(&self) -> DMARXIM_R {
        DMARXIM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Transmit DMA Interrupt Mask"]
    #[inline(always)]
    pub fn dmatxim(&self) -> DMATXIM_R {
        DMATXIM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Ring Indicator Modem Interrupt Mask"]
    #[inline(always)]
    pub fn rimim(&mut self) -> RIMIM_W {
        RIMIM_W { w: self }
    }
    #[doc = "Bit 1 - UART Clear to Send Modem Interrupt Mask"]
    #[inline(always)]
    pub fn ctsmim(&mut self) -> CTSMIM_W {
        CTSMIM_W { w: self }
    }
    #[doc = "Bit 2 - UART Data Carrier Detect Modem Interrupt Mask"]
    #[inline(always)]
    pub fn dcdmim(&mut self) -> DCDMIM_W {
        DCDMIM_W { w: self }
    }
    #[doc = "Bit 3 - UART Data Set Ready Modem Interrupt Mask"]
    #[inline(always)]
    pub fn dsrmim(&mut self) -> DSRMIM_W {
        DSRMIM_W { w: self }
    }
    #[doc = "Bit 4 - UART Receive Interrupt Mask"]
    #[inline(always)]
    pub fn rxim(&mut self) -> RXIM_W {
        RXIM_W { w: self }
    }
    #[doc = "Bit 5 - UART Transmit Interrupt Mask"]
    #[inline(always)]
    pub fn txim(&mut self) -> TXIM_W {
        TXIM_W { w: self }
    }
    #[doc = "Bit 6 - UART Receive Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn rtim(&mut self) -> RTIM_W {
        RTIM_W { w: self }
    }
    #[doc = "Bit 7 - UART Framing Error Interrupt Mask"]
    #[inline(always)]
    pub fn feim(&mut self) -> FEIM_W {
        FEIM_W { w: self }
    }
    #[doc = "Bit 8 - UART Parity Error Interrupt Mask"]
    #[inline(always)]
    pub fn peim(&mut self) -> PEIM_W {
        PEIM_W { w: self }
    }
    #[doc = "Bit 9 - UART Break Error Interrupt Mask"]
    #[inline(always)]
    pub fn beim(&mut self) -> BEIM_W {
        BEIM_W { w: self }
    }
    #[doc = "Bit 10 - UART Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn oeim(&mut self) -> OEIM_W {
        OEIM_W { w: self }
    }
    #[doc = "Bit 11 - End of Transmission Interrupt Mask"]
    #[inline(always)]
    pub fn eotim(&mut self) -> EOTIM_W {
        EOTIM_W { w: self }
    }
    #[doc = "Bit 12 - 9-Bit Mode Interrupt Mask"]
    #[inline(always)]
    pub fn _9bitim(&mut self) -> _9BITIM_W {
        _9BITIM_W { w: self }
    }
    #[doc = "Bit 16 - Receive DMA Interrupt Mask"]
    #[inline(always)]
    pub fn dmarxim(&mut self) -> DMARXIM_W {
        DMARXIM_W { w: self }
    }
    #[doc = "Bit 17 - Transmit DMA Interrupt Mask"]
    #[inline(always)]
    pub fn dmatxim(&mut self) -> DMATXIM_W {
        DMATXIM_W { w: self }
    }
}
