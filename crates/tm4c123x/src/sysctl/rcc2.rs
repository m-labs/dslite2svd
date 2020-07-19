#[doc = "Reader of register RCC2"]
pub type R = crate::R<u32, super::RCC2>;
#[doc = "Writer for register RCC2"]
pub type W = crate::W<u32, super::RCC2>;
#[doc = "Register RCC2 `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Oscillator Source 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSCSRC2_A {
    #[doc = "0: MOSC"]
    MO = 0,
    #[doc = "1: PIOSC"]
    IO = 1,
    #[doc = "2: PIOSC/4"]
    IO4 = 2,
    #[doc = "3: LFIOSC"]
    _30 = 3,
    #[doc = "7: 32.768 kHz"]
    _32 = 7,
}
impl From<OSCSRC2_A> for u8 {
    #[inline(always)]
    fn from(variant: OSCSRC2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OSCSRC2`"]
pub type OSCSRC2_R = crate::R<u8, OSCSRC2_A>;
impl OSCSRC2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OSCSRC2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OSCSRC2_A::MO),
            1 => Val(OSCSRC2_A::IO),
            2 => Val(OSCSRC2_A::IO4),
            3 => Val(OSCSRC2_A::_30),
            7 => Val(OSCSRC2_A::_32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MO`"]
    #[inline(always)]
    pub fn is_mo(&self) -> bool {
        *self == OSCSRC2_A::MO
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == OSCSRC2_A::IO
    }
    #[doc = "Checks if the value of the field is `IO4`"]
    #[inline(always)]
    pub fn is_io4(&self) -> bool {
        *self == OSCSRC2_A::IO4
    }
    #[doc = "Checks if the value of the field is `_30`"]
    #[inline(always)]
    pub fn is_30(&self) -> bool {
        *self == OSCSRC2_A::_30
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == OSCSRC2_A::_32
    }
}
#[doc = "Write proxy for field `OSCSRC2`"]
pub struct OSCSRC2_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCSRC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCSRC2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn mo(self) -> &'a mut W {
        self.variant(OSCSRC2_A::MO)
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(OSCSRC2_A::IO)
    }
    #[doc = "PIOSC/4"]
    #[inline(always)]
    pub fn io4(self) -> &'a mut W {
        self.variant(OSCSRC2_A::IO4)
    }
    #[doc = "LFIOSC"]
    #[inline(always)]
    pub fn _30(self) -> &'a mut W {
        self.variant(OSCSRC2_A::_30)
    }
    #[doc = "32.768 kHz"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(OSCSRC2_A::_32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `BYPASS2`"]
pub type BYPASS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPASS2`"]
pub struct BYPASS2_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS2_W<'a> {
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
#[doc = "Reader of field `PWRDN2`"]
pub type PWRDN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRDN2`"]
pub struct PWRDN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRDN2_W<'a> {
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
#[doc = "Reader of field `USBPWRDN`"]
pub type USBPWRDN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBPWRDN`"]
pub struct USBPWRDN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPWRDN_W<'a> {
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
#[doc = "Reader of field `SYSDIV2LSB`"]
pub type SYSDIV2LSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSDIV2LSB`"]
pub struct SYSDIV2LSB_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSDIV2LSB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `SYSDIV2`"]
pub type SYSDIV2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSDIV2`"]
pub struct SYSDIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSDIV2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 23)) | (((value as u32) & 0x3f) << 23);
        self.w
    }
}
#[doc = "Reader of field `DIV400`"]
pub type DIV400_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIV400`"]
pub struct DIV400_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV400_W<'a> {
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
#[doc = "Reader of field `USERCC2`"]
pub type USERCC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USERCC2`"]
pub struct USERCC2_W<'a> {
    w: &'a mut W,
}
impl<'a> USERCC2_W<'a> {
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
    #[doc = "Bits 4:6 - Oscillator Source 2"]
    #[inline(always)]
    pub fn oscsrc2(&self) -> OSCSRC2_R {
        OSCSRC2_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 11 - PLL Bypass 2"]
    #[inline(always)]
    pub fn bypass2(&self) -> BYPASS2_R {
        BYPASS2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Power-Down PLL 2"]
    #[inline(always)]
    pub fn pwrdn2(&self) -> PWRDN2_R {
        PWRDN2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Power-Down USB PLL"]
    #[inline(always)]
    pub fn usbpwrdn(&self) -> USBPWRDN_R {
        USBPWRDN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Additional LSB for SYSDIV2"]
    #[inline(always)]
    pub fn sysdiv2lsb(&self) -> SYSDIV2LSB_R {
        SYSDIV2LSB_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 23:28 - System Clock Divisor 2"]
    #[inline(always)]
    pub fn sysdiv2(&self) -> SYSDIV2_R {
        SYSDIV2_R::new(((self.bits >> 23) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Divide PLL as 400 MHz vs. 200 MHz"]
    #[inline(always)]
    pub fn div400(&self) -> DIV400_R {
        DIV400_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Use RCC2"]
    #[inline(always)]
    pub fn usercc2(&self) -> USERCC2_R {
        USERCC2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - Oscillator Source 2"]
    #[inline(always)]
    pub fn oscsrc2(&mut self) -> OSCSRC2_W {
        OSCSRC2_W { w: self }
    }
    #[doc = "Bit 11 - PLL Bypass 2"]
    #[inline(always)]
    pub fn bypass2(&mut self) -> BYPASS2_W {
        BYPASS2_W { w: self }
    }
    #[doc = "Bit 13 - Power-Down PLL 2"]
    #[inline(always)]
    pub fn pwrdn2(&mut self) -> PWRDN2_W {
        PWRDN2_W { w: self }
    }
    #[doc = "Bit 14 - Power-Down USB PLL"]
    #[inline(always)]
    pub fn usbpwrdn(&mut self) -> USBPWRDN_W {
        USBPWRDN_W { w: self }
    }
    #[doc = "Bit 22 - Additional LSB for SYSDIV2"]
    #[inline(always)]
    pub fn sysdiv2lsb(&mut self) -> SYSDIV2LSB_W {
        SYSDIV2LSB_W { w: self }
    }
    #[doc = "Bits 23:28 - System Clock Divisor 2"]
    #[inline(always)]
    pub fn sysdiv2(&mut self) -> SYSDIV2_W {
        SYSDIV2_W { w: self }
    }
    #[doc = "Bit 30 - Divide PLL as 400 MHz vs. 200 MHz"]
    #[inline(always)]
    pub fn div400(&mut self) -> DIV400_W {
        DIV400_W { w: self }
    }
    #[doc = "Bit 31 - Use RCC2"]
    #[inline(always)]
    pub fn usercc2(&mut self) -> USERCC2_W {
        USERCC2_W { w: self }
    }
}
