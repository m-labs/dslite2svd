#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RSCLKCFG {
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
pub struct PSYSDIVR {
    bits: u16,
}
impl PSYSDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OSYSDIVR {
    bits: u16,
}
impl OSYSDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `OSCSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSRCR {
    #[doc = "PIOSC is oscillator source"]
    PIOSC,
    #[doc = "LFIOSC is oscillator source"]
    LFIOSC,
    #[doc = "MOSC is oscillator source"]
    MOSC,
    #[doc = "Hibernation Module RTC Oscillator (RTCOSC)"]
    RTC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OSCSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OSCSRCR::PIOSC => 0,
            OSCSRCR::LFIOSC => 2,
            OSCSRCR::MOSC => 3,
            OSCSRCR::RTC => 4,
            OSCSRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OSCSRCR {
        match value {
            0 => OSCSRCR::PIOSC,
            2 => OSCSRCR::LFIOSC,
            3 => OSCSRCR::MOSC,
            4 => OSCSRCR::RTC,
            i => OSCSRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIOSC`"]
    #[inline]
    pub fn is_piosc(&self) -> bool {
        *self == OSCSRCR::PIOSC
    }
    #[doc = "Checks if the value of the field is `LFIOSC`"]
    #[inline]
    pub fn is_lfiosc(&self) -> bool {
        *self == OSCSRCR::LFIOSC
    }
    #[doc = "Checks if the value of the field is `MOSC`"]
    #[inline]
    pub fn is_mosc(&self) -> bool {
        *self == OSCSRCR::MOSC
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline]
    pub fn is_rtc(&self) -> bool {
        *self == OSCSRCR::RTC
    }
}
#[doc = "Possible values of the field `PLLSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSRCR {
    #[doc = "PIOSC is PLL input clock source"]
    PIOSC,
    #[doc = "MOSC is the PLL input clock source"]
    MOSC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PLLSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLLSRCR::PIOSC => 0,
            PLLSRCR::MOSC => 3,
            PLLSRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLLSRCR {
        match value {
            0 => PLLSRCR::PIOSC,
            3 => PLLSRCR::MOSC,
            i => PLLSRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIOSC`"]
    #[inline]
    pub fn is_piosc(&self) -> bool {
        *self == PLLSRCR::PIOSC
    }
    #[doc = "Checks if the value of the field is `MOSC`"]
    #[inline]
    pub fn is_mosc(&self) -> bool {
        *self == PLLSRCR::MOSC
    }
}
#[doc = r" Value of the field"]
pub struct USEPLLR {
    bits: bool,
}
impl USEPLLR {
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
pub struct ACGR {
    bits: bool,
}
impl ACGR {
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
pub struct NEWFREQR {
    bits: bool,
}
impl NEWFREQR {
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
pub struct MEMTIMUR {
    bits: bool,
}
impl MEMTIMUR {
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
pub struct _PSYSDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _PSYSDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OSYSDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _OSYSDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSCSRC`"]
pub enum OSCSRCW {
    #[doc = "PIOSC is oscillator source"]
    PIOSC,
    #[doc = "LFIOSC is oscillator source"]
    LFIOSC,
    #[doc = "MOSC is oscillator source"]
    MOSC,
    #[doc = "Hibernation Module RTC Oscillator (RTCOSC)"]
    RTC,
}
impl OSCSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OSCSRCW::PIOSC => 0,
            OSCSRCW::LFIOSC => 2,
            OSCSRCW::MOSC => 3,
            OSCSRCW::RTC => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCSRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PIOSC is oscillator source"]
    #[inline]
    pub fn piosc(self) -> &'a mut W {
        self.variant(OSCSRCW::PIOSC)
    }
    #[doc = "LFIOSC is oscillator source"]
    #[inline]
    pub fn lfiosc(self) -> &'a mut W {
        self.variant(OSCSRCW::LFIOSC)
    }
    #[doc = "MOSC is oscillator source"]
    #[inline]
    pub fn mosc(self) -> &'a mut W {
        self.variant(OSCSRCW::MOSC)
    }
    #[doc = "Hibernation Module RTC Oscillator (RTCOSC)"]
    #[inline]
    pub fn rtc(self) -> &'a mut W {
        self.variant(OSCSRCW::RTC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLSRC`"]
pub enum PLLSRCW {
    #[doc = "PIOSC is PLL input clock source"]
    PIOSC,
    #[doc = "MOSC is the PLL input clock source"]
    MOSC,
}
impl PLLSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLLSRCW::PIOSC => 0,
            PLLSRCW::MOSC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLSRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PIOSC is PLL input clock source"]
    #[inline]
    pub fn piosc(self) -> &'a mut W {
        self.variant(PLLSRCW::PIOSC)
    }
    #[doc = "MOSC is the PLL input clock source"]
    #[inline]
    pub fn mosc(self) -> &'a mut W {
        self.variant(PLLSRCW::MOSC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USEPLLW<'a> {
    w: &'a mut W,
}
impl<'a> _USEPLLW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACGW<'a> {
    w: &'a mut W,
}
impl<'a> _ACGW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NEWFREQW<'a> {
    w: &'a mut W,
}
impl<'a> _NEWFREQW<'a> {
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
pub struct _MEMTIMUW<'a> {
    w: &'a mut W,
}
impl<'a> _MEMTIMUW<'a> {
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
    #[doc = "Bits 0:9 - PLL System Clock Divisor"]
    #[inline]
    pub fn psysdiv(&self) -> PSYSDIVR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PSYSDIVR { bits }
    }
    #[doc = "Bits 10:19 - Oscillator System Clock Divisor"]
    #[inline]
    pub fn osysdiv(&self) -> OSYSDIVR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        OSYSDIVR { bits }
    }
    #[doc = "Bits 20:23 - Oscillator Source"]
    #[inline]
    pub fn oscsrc(&self) -> OSCSRCR {
        OSCSRCR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - PLL Source"]
    #[inline]
    pub fn pllsrc(&self) -> PLLSRCR {
        PLLSRCR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Use PLL"]
    #[inline]
    pub fn usepll(&self) -> USEPLLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USEPLLR { bits }
    }
    #[doc = "Bit 29 - Auto Clock Gating"]
    #[inline]
    pub fn acg(&self) -> ACGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACGR { bits }
    }
    #[doc = "Bit 30 - New PLLFREQ Accept"]
    #[inline]
    pub fn newfreq(&self) -> NEWFREQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NEWFREQR { bits }
    }
    #[doc = "Bit 31 - Memory Timing Register Update"]
    #[inline]
    pub fn memtimu(&self) -> MEMTIMUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MEMTIMUR { bits }
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
    #[doc = "Bits 0:9 - PLL System Clock Divisor"]
    #[inline]
    pub fn psysdiv(&mut self) -> _PSYSDIVW {
        _PSYSDIVW { w: self }
    }
    #[doc = "Bits 10:19 - Oscillator System Clock Divisor"]
    #[inline]
    pub fn osysdiv(&mut self) -> _OSYSDIVW {
        _OSYSDIVW { w: self }
    }
    #[doc = "Bits 20:23 - Oscillator Source"]
    #[inline]
    pub fn oscsrc(&mut self) -> _OSCSRCW {
        _OSCSRCW { w: self }
    }
    #[doc = "Bits 24:27 - PLL Source"]
    #[inline]
    pub fn pllsrc(&mut self) -> _PLLSRCW {
        _PLLSRCW { w: self }
    }
    #[doc = "Bit 28 - Use PLL"]
    #[inline]
    pub fn usepll(&mut self) -> _USEPLLW {
        _USEPLLW { w: self }
    }
    #[doc = "Bit 29 - Auto Clock Gating"]
    #[inline]
    pub fn acg(&mut self) -> _ACGW {
        _ACGW { w: self }
    }
    #[doc = "Bit 30 - New PLLFREQ Accept"]
    #[inline]
    pub fn newfreq(&mut self) -> _NEWFREQW {
        _NEWFREQW { w: self }
    }
    #[doc = "Bit 31 - Memory Timing Register Update"]
    #[inline]
    pub fn memtimu(&mut self) -> _MEMTIMUW {
        _MEMTIMUW { w: self }
    }
}
