#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTL {
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
pub struct RTCENR {
    bits: bool,
}
impl RTCENR {
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
pub struct HIBREQR {
    bits: bool,
}
impl HIBREQR {
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
pub struct RTCWENR {
    bits: bool,
}
impl RTCWENR {
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
pub struct PINWENR {
    bits: bool,
}
impl PINWENR {
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
pub struct CLK32ENR {
    bits: bool,
}
impl CLK32ENR {
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
pub struct VABORTR {
    bits: bool,
}
impl VABORTR {
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
pub struct VDD3ONR {
    bits: bool,
}
impl VDD3ONR {
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
pub struct BATWKENR {
    bits: bool,
}
impl BATWKENR {
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
pub struct BATCHKR {
    bits: bool,
}
impl BATCHKR {
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
#[doc = "Possible values of the field `VBATSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATSELR {
    #[doc = "1.9 Volts"]
    _1_9V,
    #[doc = "2.1 Volts (default)"]
    _2_1V,
    #[doc = "2.3 Volts"]
    _2_3V,
    #[doc = "2.5 Volts"]
    _2_5V,
}
impl VBATSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VBATSELR::_1_9V => 0,
            VBATSELR::_2_1V => 1,
            VBATSELR::_2_3V => 2,
            VBATSELR::_2_5V => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VBATSELR {
        match value {
            0 => VBATSELR::_1_9V,
            1 => VBATSELR::_2_1V,
            2 => VBATSELR::_2_3V,
            3 => VBATSELR::_2_5V,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1_9V`"]
    #[inline]
    pub fn is_1_9v(&self) -> bool {
        *self == VBATSELR::_1_9V
    }
    #[doc = "Checks if the value of the field is `_2_1V`"]
    #[inline]
    pub fn is_2_1v(&self) -> bool {
        *self == VBATSELR::_2_1V
    }
    #[doc = "Checks if the value of the field is `_2_3V`"]
    #[inline]
    pub fn is_2_3v(&self) -> bool {
        *self == VBATSELR::_2_3V
    }
    #[doc = "Checks if the value of the field is `_2_5V`"]
    #[inline]
    pub fn is_2_5v(&self) -> bool {
        *self == VBATSELR::_2_5V
    }
}
#[doc = r" Value of the field"]
pub struct OSCBYPR {
    bits: bool,
}
impl OSCBYPR {
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
pub struct OSCDRVR {
    bits: bool,
}
impl OSCDRVR {
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
pub struct WRCR {
    bits: bool,
}
impl WRCR {
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
pub struct _RTCENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCENW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HIBREQW<'a> {
    w: &'a mut W,
}
impl<'a> _HIBREQW<'a> {
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
pub struct _RTCWENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCWENW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINWENW<'a> {
    w: &'a mut W,
}
impl<'a> _PINWENW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLK32ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK32ENW<'a> {
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
pub struct _VABORTW<'a> {
    w: &'a mut W,
}
impl<'a> _VABORTW<'a> {
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
pub struct _VDD3ONW<'a> {
    w: &'a mut W,
}
impl<'a> _VDD3ONW<'a> {
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
#[doc = r" Proxy"]
pub struct _BATWKENW<'a> {
    w: &'a mut W,
}
impl<'a> _BATWKENW<'a> {
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
pub struct _BATCHKW<'a> {
    w: &'a mut W,
}
impl<'a> _BATCHKW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VBATSEL`"]
pub enum VBATSELW {
    #[doc = "1.9 Volts"]
    _1_9V,
    #[doc = "2.1 Volts (default)"]
    _2_1V,
    #[doc = "2.3 Volts"]
    _2_3V,
    #[doc = "2.5 Volts"]
    _2_5V,
}
impl VBATSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VBATSELW::_1_9V => 0,
            VBATSELW::_2_1V => 1,
            VBATSELW::_2_3V => 2,
            VBATSELW::_2_5V => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VBATSELW<'a> {
    w: &'a mut W,
}
impl<'a> _VBATSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VBATSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1.9 Volts"]
    #[inline]
    pub fn _1_9v(self) -> &'a mut W {
        self.variant(VBATSELW::_1_9V)
    }
    #[doc = "2.1 Volts (default)"]
    #[inline]
    pub fn _2_1v(self) -> &'a mut W {
        self.variant(VBATSELW::_2_1V)
    }
    #[doc = "2.3 Volts"]
    #[inline]
    pub fn _2_3v(self) -> &'a mut W {
        self.variant(VBATSELW::_2_3V)
    }
    #[doc = "2.5 Volts"]
    #[inline]
    pub fn _2_5v(self) -> &'a mut W {
        self.variant(VBATSELW::_2_5V)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OSCBYPW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCBYPW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OSCDRVW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCDRVW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WRCW<'a> {
    w: &'a mut W,
}
impl<'a> _WRCW<'a> {
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
    #[doc = "Bit 0 - RTC Timer Enable"]
    #[inline]
    pub fn rtcen(&self) -> RTCENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RTCENR { bits }
    }
    #[doc = "Bit 1 - Hibernation Request"]
    #[inline]
    pub fn hibreq(&self) -> HIBREQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HIBREQR { bits }
    }
    #[doc = "Bit 3 - RTC Wake-up Enable"]
    #[inline]
    pub fn rtcwen(&self) -> RTCWENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RTCWENR { bits }
    }
    #[doc = "Bit 4 - External Wake Pin Enable"]
    #[inline]
    pub fn pinwen(&self) -> PINWENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINWENR { bits }
    }
    #[doc = "Bit 6 - Clocking Enable"]
    #[inline]
    pub fn clk32en(&self) -> CLK32ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLK32ENR { bits }
    }
    #[doc = "Bit 7 - Power Cut Abort Enable"]
    #[inline]
    pub fn vabort(&self) -> VABORTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VABORTR { bits }
    }
    #[doc = "Bit 8 - VDD Powered"]
    #[inline]
    pub fn vdd3on(&self) -> VDD3ONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VDD3ONR { bits }
    }
    #[doc = "Bit 9 - Wake on Low Battery"]
    #[inline]
    pub fn batwken(&self) -> BATWKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BATWKENR { bits }
    }
    #[doc = "Bit 10 - Check Battery Status"]
    #[inline]
    pub fn batchk(&self) -> BATCHKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BATCHKR { bits }
    }
    #[doc = "Bits 13:14 - Select for Low-Battery Comparator"]
    #[inline]
    pub fn vbatsel(&self) -> VBATSELR {
        VBATSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Oscillator Bypass"]
    #[inline]
    pub fn oscbyp(&self) -> OSCBYPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OSCBYPR { bits }
    }
    #[doc = "Bit 17 - Oscillator Drive Capability"]
    #[inline]
    pub fn oscdrv(&self) -> OSCDRVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OSCDRVR { bits }
    }
    #[doc = "Bit 31 - Write Complete/Capable"]
    #[inline]
    pub fn wrc(&self) -> WRCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRCR { bits }
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
    #[doc = "Bit 0 - RTC Timer Enable"]
    #[inline]
    pub fn rtcen(&mut self) -> _RTCENW {
        _RTCENW { w: self }
    }
    #[doc = "Bit 1 - Hibernation Request"]
    #[inline]
    pub fn hibreq(&mut self) -> _HIBREQW {
        _HIBREQW { w: self }
    }
    #[doc = "Bit 3 - RTC Wake-up Enable"]
    #[inline]
    pub fn rtcwen(&mut self) -> _RTCWENW {
        _RTCWENW { w: self }
    }
    #[doc = "Bit 4 - External Wake Pin Enable"]
    #[inline]
    pub fn pinwen(&mut self) -> _PINWENW {
        _PINWENW { w: self }
    }
    #[doc = "Bit 6 - Clocking Enable"]
    #[inline]
    pub fn clk32en(&mut self) -> _CLK32ENW {
        _CLK32ENW { w: self }
    }
    #[doc = "Bit 7 - Power Cut Abort Enable"]
    #[inline]
    pub fn vabort(&mut self) -> _VABORTW {
        _VABORTW { w: self }
    }
    #[doc = "Bit 8 - VDD Powered"]
    #[inline]
    pub fn vdd3on(&mut self) -> _VDD3ONW {
        _VDD3ONW { w: self }
    }
    #[doc = "Bit 9 - Wake on Low Battery"]
    #[inline]
    pub fn batwken(&mut self) -> _BATWKENW {
        _BATWKENW { w: self }
    }
    #[doc = "Bit 10 - Check Battery Status"]
    #[inline]
    pub fn batchk(&mut self) -> _BATCHKW {
        _BATCHKW { w: self }
    }
    #[doc = "Bits 13:14 - Select for Low-Battery Comparator"]
    #[inline]
    pub fn vbatsel(&mut self) -> _VBATSELW {
        _VBATSELW { w: self }
    }
    #[doc = "Bit 16 - Oscillator Bypass"]
    #[inline]
    pub fn oscbyp(&mut self) -> _OSCBYPW {
        _OSCBYPW { w: self }
    }
    #[doc = "Bit 17 - Oscillator Drive Capability"]
    #[inline]
    pub fn oscdrv(&mut self) -> _OSCDRVW {
        _OSCDRVW { w: self }
    }
    #[doc = "Bit 31 - Write Complete/Capable"]
    #[inline]
    pub fn wrc(&mut self) -> _WRCW {
        _WRCW { w: self }
    }
}
