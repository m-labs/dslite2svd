#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMAOPMODE {
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
#[doc = r" Value of the field"]
pub struct SRR {
    bits: bool,
}
impl SRR {
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
pub struct OSFR {
    bits: bool,
}
impl OSFR {
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
#[doc = "Possible values of the field `RTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCR {
    #[doc = "64 bytes"]
    _64,
    #[doc = "32 bytes"]
    _32,
    #[doc = "96 bytes"]
    _96,
    #[doc = "128 bytes"]
    _128,
}
impl RTCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RTCR::_64 => 0,
            RTCR::_32 => 1,
            RTCR::_96 => 2,
            RTCR::_128 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RTCR {
        match value {
            0 => RTCR::_64,
            1 => RTCR::_32,
            2 => RTCR::_96,
            3 => RTCR::_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == RTCR::_64
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == RTCR::_32
    }
    #[doc = "Checks if the value of the field is `_96`"]
    #[inline]
    pub fn is_96(&self) -> bool {
        *self == RTCR::_96
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == RTCR::_128
    }
}
#[doc = r" Value of the field"]
pub struct DGFR {
    bits: bool,
}
impl DGFR {
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
pub struct FUFR {
    bits: bool,
}
impl FUFR {
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
pub struct FEFR {
    bits: bool,
}
impl FEFR {
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
pub struct STR {
    bits: bool,
}
impl STR {
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
#[doc = "Possible values of the field `TTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TTCR {
    #[doc = "64 bytes"]
    _64,
    #[doc = "128 bytes"]
    _128,
    #[doc = "192 bytes"]
    _192,
    #[doc = "256 bytes"]
    _256,
    #[doc = "40 bytes"]
    _40,
    #[doc = "32 bytes"]
    _32,
    #[doc = "24 bytes"]
    _24,
    #[doc = "16 bytes"]
    _16,
}
impl TTCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TTCR::_64 => 0,
            TTCR::_128 => 1,
            TTCR::_192 => 2,
            TTCR::_256 => 3,
            TTCR::_40 => 4,
            TTCR::_32 => 5,
            TTCR::_24 => 6,
            TTCR::_16 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TTCR {
        match value {
            0 => TTCR::_64,
            1 => TTCR::_128,
            2 => TTCR::_192,
            3 => TTCR::_256,
            4 => TTCR::_40,
            5 => TTCR::_32,
            6 => TTCR::_24,
            7 => TTCR::_16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == TTCR::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == TTCR::_128
    }
    #[doc = "Checks if the value of the field is `_192`"]
    #[inline]
    pub fn is_192(&self) -> bool {
        *self == TTCR::_192
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == TTCR::_256
    }
    #[doc = "Checks if the value of the field is `_40`"]
    #[inline]
    pub fn is_40(&self) -> bool {
        *self == TTCR::_40
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == TTCR::_32
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline]
    pub fn is_24(&self) -> bool {
        *self == TTCR::_24
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == TTCR::_16
    }
}
#[doc = r" Value of the field"]
pub struct FTFR {
    bits: bool,
}
impl FTFR {
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
pub struct TSFR {
    bits: bool,
}
impl TSFR {
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
pub struct DFFR {
    bits: bool,
}
impl DFFR {
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
pub struct RSFR {
    bits: bool,
}
impl RSFR {
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
pub struct DTR {
    bits: bool,
}
impl DTR {
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
#[doc = r" Proxy"]
pub struct _SRW<'a> {
    w: &'a mut W,
}
impl<'a> _SRW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OSFW<'a> {
    w: &'a mut W,
}
impl<'a> _OSFW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC`"]
pub enum RTCW {
    #[doc = "64 bytes"]
    _64,
    #[doc = "32 bytes"]
    _32,
    #[doc = "96 bytes"]
    _96,
    #[doc = "128 bytes"]
    _128,
}
impl RTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RTCW::_64 => 0,
            RTCW::_32 => 1,
            RTCW::_96 => 2,
            RTCW::_128 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "64 bytes"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(RTCW::_64)
    }
    #[doc = "32 bytes"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(RTCW::_32)
    }
    #[doc = "96 bytes"]
    #[inline]
    pub fn _96(self) -> &'a mut W {
        self.variant(RTCW::_96)
    }
    #[doc = "128 bytes"]
    #[inline]
    pub fn _128(self) -> &'a mut W {
        self.variant(RTCW::_128)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DGFW<'a> {
    w: &'a mut W,
}
impl<'a> _DGFW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FUFW<'a> {
    w: &'a mut W,
}
impl<'a> _FUFW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FEFW<'a> {
    w: &'a mut W,
}
impl<'a> _FEFW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STW<'a> {
    w: &'a mut W,
}
impl<'a> _STW<'a> {
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
#[doc = "Values that can be written to the field `TTC`"]
pub enum TTCW {
    #[doc = "64 bytes"]
    _64,
    #[doc = "128 bytes"]
    _128,
    #[doc = "192 bytes"]
    _192,
    #[doc = "256 bytes"]
    _256,
    #[doc = "40 bytes"]
    _40,
    #[doc = "32 bytes"]
    _32,
    #[doc = "24 bytes"]
    _24,
    #[doc = "16 bytes"]
    _16,
}
impl TTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TTCW::_64 => 0,
            TTCW::_128 => 1,
            TTCW::_192 => 2,
            TTCW::_256 => 3,
            TTCW::_40 => 4,
            TTCW::_32 => 5,
            TTCW::_24 => 6,
            TTCW::_16 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TTCW<'a> {
    w: &'a mut W,
}
impl<'a> _TTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TTCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "64 bytes"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(TTCW::_64)
    }
    #[doc = "128 bytes"]
    #[inline]
    pub fn _128(self) -> &'a mut W {
        self.variant(TTCW::_128)
    }
    #[doc = "192 bytes"]
    #[inline]
    pub fn _192(self) -> &'a mut W {
        self.variant(TTCW::_192)
    }
    #[doc = "256 bytes"]
    #[inline]
    pub fn _256(self) -> &'a mut W {
        self.variant(TTCW::_256)
    }
    #[doc = "40 bytes"]
    #[inline]
    pub fn _40(self) -> &'a mut W {
        self.variant(TTCW::_40)
    }
    #[doc = "32 bytes"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(TTCW::_32)
    }
    #[doc = "24 bytes"]
    #[inline]
    pub fn _24(self) -> &'a mut W {
        self.variant(TTCW::_24)
    }
    #[doc = "16 bytes"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(TTCW::_16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FTFW<'a> {
    w: &'a mut W,
}
impl<'a> _FTFW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSFW<'a> {
    w: &'a mut W,
}
impl<'a> _TSFW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DFFW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RSFW<'a> {
    w: &'a mut W,
}
impl<'a> _RSFW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTW<'a> {
    w: &'a mut W,
}
impl<'a> _DTW<'a> {
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
        const OFFSET: u8 = 26;
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
    #[doc = "Bit 1 - Start or Stop Receive"]
    #[inline]
    pub fn sr(&self) -> SRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SRR { bits }
    }
    #[doc = "Bit 2 - Operate on Second Frame"]
    #[inline]
    pub fn osf(&self) -> OSFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OSFR { bits }
    }
    #[doc = "Bits 3:4 - Receive Threshold Control"]
    #[inline]
    pub fn rtc(&self) -> RTCR {
        RTCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Drop Giant Frame Enable"]
    #[inline]
    pub fn dgf(&self) -> DGFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DGFR { bits }
    }
    #[doc = "Bit 6 - Forward Undersized Good Frames"]
    #[inline]
    pub fn fuf(&self) -> FUFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FUFR { bits }
    }
    #[doc = "Bit 7 - Forward Error Frames"]
    #[inline]
    pub fn fef(&self) -> FEFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FEFR { bits }
    }
    #[doc = "Bit 13 - Start or Stop Transmission Command"]
    #[inline]
    pub fn st(&self) -> STR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STR { bits }
    }
    #[doc = "Bits 14:16 - Transmit Threshold Control"]
    #[inline]
    pub fn ttc(&self) -> TTCR {
        TTCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Flush Transmit FIFO"]
    #[inline]
    pub fn ftf(&self) -> FTFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FTFR { bits }
    }
    #[doc = "Bit 21 - Transmit Store and Forward"]
    #[inline]
    pub fn tsf(&self) -> TSFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSFR { bits }
    }
    #[doc = "Bit 24 - Disable Flushing of Received Frames"]
    #[inline]
    pub fn dff(&self) -> DFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DFFR { bits }
    }
    #[doc = "Bit 25 - Receive Store and Forward"]
    #[inline]
    pub fn rsf(&self) -> RSFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSFR { bits }
    }
    #[doc = "Bit 26 - Disable Dropping of TCP/IP Checksum Error Frames"]
    #[inline]
    pub fn dt(&self) -> DTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTR { bits }
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
    #[doc = "Bit 1 - Start or Stop Receive"]
    #[inline]
    pub fn sr(&mut self) -> _SRW {
        _SRW { w: self }
    }
    #[doc = "Bit 2 - Operate on Second Frame"]
    #[inline]
    pub fn osf(&mut self) -> _OSFW {
        _OSFW { w: self }
    }
    #[doc = "Bits 3:4 - Receive Threshold Control"]
    #[inline]
    pub fn rtc(&mut self) -> _RTCW {
        _RTCW { w: self }
    }
    #[doc = "Bit 5 - Drop Giant Frame Enable"]
    #[inline]
    pub fn dgf(&mut self) -> _DGFW {
        _DGFW { w: self }
    }
    #[doc = "Bit 6 - Forward Undersized Good Frames"]
    #[inline]
    pub fn fuf(&mut self) -> _FUFW {
        _FUFW { w: self }
    }
    #[doc = "Bit 7 - Forward Error Frames"]
    #[inline]
    pub fn fef(&mut self) -> _FEFW {
        _FEFW { w: self }
    }
    #[doc = "Bit 13 - Start or Stop Transmission Command"]
    #[inline]
    pub fn st(&mut self) -> _STW {
        _STW { w: self }
    }
    #[doc = "Bits 14:16 - Transmit Threshold Control"]
    #[inline]
    pub fn ttc(&mut self) -> _TTCW {
        _TTCW { w: self }
    }
    #[doc = "Bit 20 - Flush Transmit FIFO"]
    #[inline]
    pub fn ftf(&mut self) -> _FTFW {
        _FTFW { w: self }
    }
    #[doc = "Bit 21 - Transmit Store and Forward"]
    #[inline]
    pub fn tsf(&mut self) -> _TSFW {
        _TSFW { w: self }
    }
    #[doc = "Bit 24 - Disable Flushing of Received Frames"]
    #[inline]
    pub fn dff(&mut self) -> _DFFW {
        _DFFW { w: self }
    }
    #[doc = "Bit 25 - Receive Store and Forward"]
    #[inline]
    pub fn rsf(&mut self) -> _RSFW {
        _RSFW { w: self }
    }
    #[doc = "Bit 26 - Disable Dropping of TCP/IP Checksum Error Frames"]
    #[inline]
    pub fn dt(&mut self) -> _DTW {
        _DTW { w: self }
    }
}
