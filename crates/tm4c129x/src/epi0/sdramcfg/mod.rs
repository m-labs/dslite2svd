#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SDRAMCFG {
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
#[doc = "Possible values of the field `SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZER {
    #[doc = "64 megabits (8MB)"]
    _8MB,
    #[doc = "128 megabits (16MB)"]
    _16MB,
    #[doc = "256 megabits (32MB)"]
    _32MB,
    #[doc = "512 megabits (64MB)"]
    _64MB,
}
impl SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SIZER::_8MB => 0,
            SIZER::_16MB => 1,
            SIZER::_32MB => 2,
            SIZER::_64MB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SIZER {
        match value {
            0 => SIZER::_8MB,
            1 => SIZER::_16MB,
            2 => SIZER::_32MB,
            3 => SIZER::_64MB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8MB`"]
    #[inline]
    pub fn is_8mb(&self) -> bool {
        *self == SIZER::_8MB
    }
    #[doc = "Checks if the value of the field is `_16MB`"]
    #[inline]
    pub fn is_16mb(&self) -> bool {
        *self == SIZER::_16MB
    }
    #[doc = "Checks if the value of the field is `_32MB`"]
    #[inline]
    pub fn is_32mb(&self) -> bool {
        *self == SIZER::_32MB
    }
    #[doc = "Checks if the value of the field is `_64MB`"]
    #[inline]
    pub fn is_64mb(&self) -> bool {
        *self == SIZER::_64MB
    }
}
#[doc = r" Value of the field"]
pub struct SLEEPR {
    bits: bool,
}
impl SLEEPR {
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
pub struct RFSHR {
    bits: u16,
}
impl RFSHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `FREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQR {
    #[doc = "0 - 15 MHz"]
    NONE,
    #[doc = "15 - 30 MHz"]
    _15MHZ,
    #[doc = "30 - 50 MHz"]
    _30MHZ,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FREQR::NONE => 0,
            FREQR::_15MHZ => 1,
            FREQR::_30MHZ => 2,
            FREQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FREQR {
        match value {
            0 => FREQR::NONE,
            1 => FREQR::_15MHZ,
            2 => FREQR::_30MHZ,
            i => FREQR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == FREQR::NONE
    }
    #[doc = "Checks if the value of the field is `_15MHZ`"]
    #[inline]
    pub fn is_15mhz(&self) -> bool {
        *self == FREQR::_15MHZ
    }
    #[doc = "Checks if the value of the field is `_30MHZ`"]
    #[inline]
    pub fn is_30mhz(&self) -> bool {
        *self == FREQR::_30MHZ
    }
}
#[doc = "Values that can be written to the field `SIZE`"]
pub enum SIZEW {
    #[doc = "64 megabits (8MB)"]
    _8MB,
    #[doc = "128 megabits (16MB)"]
    _16MB,
    #[doc = "256 megabits (32MB)"]
    _32MB,
    #[doc = "512 megabits (64MB)"]
    _64MB,
}
impl SIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SIZEW::_8MB => 0,
            SIZEW::_16MB => 1,
            SIZEW::_32MB => 2,
            SIZEW::_64MB => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _SIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "64 megabits (8MB)"]
    #[inline]
    pub fn _8mb(self) -> &'a mut W {
        self.variant(SIZEW::_8MB)
    }
    #[doc = "128 megabits (16MB)"]
    #[inline]
    pub fn _16mb(self) -> &'a mut W {
        self.variant(SIZEW::_16MB)
    }
    #[doc = "256 megabits (32MB)"]
    #[inline]
    pub fn _32mb(self) -> &'a mut W {
        self.variant(SIZEW::_32MB)
    }
    #[doc = "512 megabits (64MB)"]
    #[inline]
    pub fn _64mb(self) -> &'a mut W {
        self.variant(SIZEW::_64MB)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SLEEPW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RFSHW<'a> {
    w: &'a mut W,
}
impl<'a> _RFSHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 2047;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FREQ`"]
pub enum FREQW {
    #[doc = "0 - 15 MHz"]
    NONE,
    #[doc = "15 - 30 MHz"]
    _15MHZ,
    #[doc = "30 - 50 MHz"]
    _30MHZ,
}
impl FREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FREQW::NONE => 0,
            FREQW::_15MHZ => 1,
            FREQW::_30MHZ => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FREQW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FREQW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "0 - 15 MHz"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(FREQW::NONE)
    }
    #[doc = "15 - 30 MHz"]
    #[inline]
    pub fn _15mhz(self) -> &'a mut W {
        self.variant(FREQW::_15MHZ)
    }
    #[doc = "30 - 50 MHz"]
    #[inline]
    pub fn _30mhz(self) -> &'a mut W {
        self.variant(FREQW::_30MHZ)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - Size of SDRAM"]
    #[inline]
    pub fn size(&self) -> SIZER {
        SIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 9 - Sleep Mode"]
    #[inline]
    pub fn sleep(&self) -> SLEEPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SLEEPR { bits }
    }
    #[doc = "Bits 16:26 - Refresh Counter"]
    #[inline]
    pub fn rfsh(&self) -> RFSHR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RFSHR { bits }
    }
    #[doc = "Bits 30:31 - EPI Frequency Range"]
    #[inline]
    pub fn freq(&self) -> FREQR {
        FREQR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:1 - Size of SDRAM"]
    #[inline]
    pub fn size(&mut self) -> _SIZEW {
        _SIZEW { w: self }
    }
    #[doc = "Bit 9 - Sleep Mode"]
    #[inline]
    pub fn sleep(&mut self) -> _SLEEPW {
        _SLEEPW { w: self }
    }
    #[doc = "Bits 16:26 - Refresh Counter"]
    #[inline]
    pub fn rfsh(&mut self) -> _RFSHW {
        _RFSHW { w: self }
    }
    #[doc = "Bits 30:31 - EPI Frequency Range"]
    #[inline]
    pub fn freq(&mut self) -> _FREQW {
        _FREQW { w: self }
    }
}
