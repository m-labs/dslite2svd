#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RCC2 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `OSCSRC2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSRC2R {
    #[doc = "MOSC"]
    MO,
    #[doc = "PIOSC"]
    IO,
    #[doc = "PIOSC/4"]
    IO4,
    #[doc = "LFIOSC"]
    _30,
    #[doc = "32.768 kHz"]
    _32,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OSCSRC2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OSCSRC2R::MO => 0,
            OSCSRC2R::IO => 1,
            OSCSRC2R::IO4 => 2,
            OSCSRC2R::_30 => 3,
            OSCSRC2R::_32 => 7,
            OSCSRC2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OSCSRC2R {
        match value {
            0 => OSCSRC2R::MO,
            1 => OSCSRC2R::IO,
            2 => OSCSRC2R::IO4,
            3 => OSCSRC2R::_30,
            7 => OSCSRC2R::_32,
            i => OSCSRC2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MO`"]
    #[inline]
    pub fn is_mo(&self) -> bool {
        *self == OSCSRC2R::MO
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline]
    pub fn is_io(&self) -> bool {
        *self == OSCSRC2R::IO
    }
    #[doc = "Checks if the value of the field is `IO4`"]
    #[inline]
    pub fn is_io4(&self) -> bool {
        *self == OSCSRC2R::IO4
    }
    #[doc = "Checks if the value of the field is `_30`"]
    #[inline]
    pub fn is_30(&self) -> bool {
        *self == OSCSRC2R::_30
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == OSCSRC2R::_32
    }
}
#[doc = r" Value of the field"]
pub struct BYPASS2R {
    bits: bool,
}
impl BYPASS2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PWRDN2R {
    bits: bool,
}
impl PWRDN2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct USBPWRDNR {
    bits: bool,
}
impl USBPWRDNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SYSDIV2LSBR {
    bits: bool,
}
impl SYSDIV2LSBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SYSDIV2R {
    bits: u8,
}
impl SYSDIV2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIV400R {
    bits: bool,
}
impl DIV400R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct USERCC2R {
    bits: bool,
}
impl USERCC2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Values that can be written to the field `OSCSRC2`"]
pub enum OSCSRC2W {
    #[doc = "MOSC"]
    MO,
    #[doc = "PIOSC"]
    IO,
    #[doc = "PIOSC/4"]
    IO4,
    #[doc = "LFIOSC"]
    _30,
    #[doc = "32.768 kHz"]
    _32,
}
impl OSCSRC2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OSCSRC2W::MO => 0,
            OSCSRC2W::IO => 1,
            OSCSRC2W::IO4 => 2,
            OSCSRC2W::_30 => 3,
            OSCSRC2W::_32 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCSRC2W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCSRC2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCSRC2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "MOSC"]
    #[inline]
    pub fn mo(self) -> &'a mut W {
        self.variant(OSCSRC2W::MO)
    }
    #[doc = "PIOSC"]
    #[inline]
    pub fn io(self) -> &'a mut W {
        self.variant(OSCSRC2W::IO)
    }
    #[doc = "PIOSC/4"]
    #[inline]
    pub fn io4(self) -> &'a mut W {
        self.variant(OSCSRC2W::IO4)
    }
    #[doc = "LFIOSC"]
    #[inline]
    pub fn _30(self) -> &'a mut W {
        self.variant(OSCSRC2W::_30)
    }
    #[doc = "32.768 kHz"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(OSCSRC2W::_32)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BYPASS2W<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASS2W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PWRDN2W<'a> {
    w: &'a mut W,
}
impl<'a> _PWRDN2W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USBPWRDNW<'a> {
    w: &'a mut W,
}
impl<'a> _USBPWRDNW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYSDIV2LSBW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSDIV2LSBW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYSDIV2W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSDIV2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIV400W<'a> {
    w: &'a mut W,
}
impl<'a> _DIV400W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USERCC2W<'a> {
    w: &'a mut W,
}
impl<'a> _USERCC2W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 4:6 - Oscillator Source 2"]
    #[inline]
    pub fn oscsrc2(&self) -> OSCSRC2R {
        OSCSRC2R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - PLL Bypass 2"]
    #[inline]
    pub fn bypass2(&self) -> BYPASS2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BYPASS2R { bits }
    }
    #[doc = "Bit 13 - Power-Down PLL 2"]
    #[inline]
    pub fn pwrdn2(&self) -> PWRDN2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PWRDN2R { bits }
    }
    #[doc = "Bit 14 - Power-Down USB PLL"]
    #[inline]
    pub fn usbpwrdn(&self) -> USBPWRDNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USBPWRDNR { bits }
    }
    #[doc = "Bit 22 - Additional LSB for SYSDIV2"]
    #[inline]
    pub fn sysdiv2lsb(&self) -> SYSDIV2LSBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSDIV2LSBR { bits }
    }
    #[doc = "Bits 23:28 - System Clock Divisor 2"]
    #[inline]
    pub fn sysdiv2(&self) -> SYSDIV2R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYSDIV2R { bits }
    }
    #[doc = "Bit 30 - Divide PLL as 400 MHz vs. 200 MHz"]
    #[inline]
    pub fn div400(&self) -> DIV400R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIV400R { bits }
    }
    #[doc = "Bit 31 - Use RCC2"]
    #[inline]
    pub fn usercc2(&self) -> USERCC2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USERCC2R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 4:6 - Oscillator Source 2"]
    #[inline]
    pub fn oscsrc2(&mut self) -> _OSCSRC2W {
        _OSCSRC2W { w: self }
    }
    #[doc = "Bit 11 - PLL Bypass 2"]
    #[inline]
    pub fn bypass2(&mut self) -> _BYPASS2W {
        _BYPASS2W { w: self }
    }
    #[doc = "Bit 13 - Power-Down PLL 2"]
    #[inline]
    pub fn pwrdn2(&mut self) -> _PWRDN2W {
        _PWRDN2W { w: self }
    }
    #[doc = "Bit 14 - Power-Down USB PLL"]
    #[inline]
    pub fn usbpwrdn(&mut self) -> _USBPWRDNW {
        _USBPWRDNW { w: self }
    }
    #[doc = "Bit 22 - Additional LSB for SYSDIV2"]
    #[inline]
    pub fn sysdiv2lsb(&mut self) -> _SYSDIV2LSBW {
        _SYSDIV2LSBW { w: self }
    }
    #[doc = "Bits 23:28 - System Clock Divisor 2"]
    #[inline]
    pub fn sysdiv2(&mut self) -> _SYSDIV2W {
        _SYSDIV2W { w: self }
    }
    #[doc = "Bit 30 - Divide PLL as 400 MHz vs. 200 MHz"]
    #[inline]
    pub fn div400(&mut self) -> _DIV400W {
        _DIV400W { w: self }
    }
    #[doc = "Bit 31 - Use RCC2"]
    #[inline]
    pub fn usercc2(&mut self) -> _USERCC2W {
        _USERCC2W { w: self }
    }
}
