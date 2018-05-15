#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CC {
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
#[doc = "Possible values of the field `PWMDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMDIVR {
    #[doc = "/2"]
    _2,
    #[doc = "/4"]
    _4,
    #[doc = "/8"]
    _8,
    #[doc = "/16"]
    _16,
    #[doc = "/32"]
    _32,
    #[doc = "/64"]
    _64,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWMDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWMDIVR::_2 => 0,
            PWMDIVR::_4 => 1,
            PWMDIVR::_8 => 2,
            PWMDIVR::_16 => 3,
            PWMDIVR::_32 => 4,
            PWMDIVR::_64 => 5,
            PWMDIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWMDIVR {
        match value {
            0 => PWMDIVR::_2,
            1 => PWMDIVR::_4,
            2 => PWMDIVR::_8,
            3 => PWMDIVR::_16,
            4 => PWMDIVR::_32,
            5 => PWMDIVR::_64,
            i => PWMDIVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == PWMDIVR::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == PWMDIVR::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == PWMDIVR::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == PWMDIVR::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == PWMDIVR::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == PWMDIVR::_64
    }
}
#[doc = r" Value of the field"]
pub struct USEPWMR {
    bits: bool,
}
impl USEPWMR {
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
#[doc = "Values that can be written to the field `PWMDIV`"]
pub enum PWMDIVW {
    #[doc = "/2"]
    _2,
    #[doc = "/4"]
    _4,
    #[doc = "/8"]
    _8,
    #[doc = "/16"]
    _16,
    #[doc = "/32"]
    _32,
    #[doc = "/64"]
    _64,
}
impl PWMDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWMDIVW::_2 => 0,
            PWMDIVW::_4 => 1,
            PWMDIVW::_8 => 2,
            PWMDIVW::_16 => 3,
            PWMDIVW::_32 => 4,
            PWMDIVW::_64 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMDIVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "/2"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(PWMDIVW::_2)
    }
    #[doc = "/4"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(PWMDIVW::_4)
    }
    #[doc = "/8"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(PWMDIVW::_8)
    }
    #[doc = "/16"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(PWMDIVW::_16)
    }
    #[doc = "/32"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(PWMDIVW::_32)
    }
    #[doc = "/64"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(PWMDIVW::_64)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USEPWMW<'a> {
    w: &'a mut W,
}
impl<'a> _USEPWMW<'a> {
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
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:2 - PWM Clock Divider"]
    #[inline]
    pub fn pwmdiv(&self) -> PWMDIVR {
        PWMDIVR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Use PWM Clock Divisor"]
    #[inline]
    pub fn usepwm(&self) -> USEPWMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USEPWMR { bits }
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
    #[doc = "Bits 0:2 - PWM Clock Divider"]
    #[inline]
    pub fn pwmdiv(&mut self) -> _PWMDIVW {
        _PWMDIVW { w: self }
    }
    #[doc = "Bit 8 - Use PWM Clock Divisor"]
    #[inline]
    pub fn usepwm(&mut self) -> _USEPWMW {
        _USEPWMW { w: self }
    }
}
