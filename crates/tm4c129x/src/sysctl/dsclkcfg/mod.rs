#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DSCLKCFG {
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
pub struct DSSYSDIVR {
    bits: u16,
}
impl DSSYSDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `DSOSCSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSOSCSRCR {
    #[doc = "PIOSC"]
    PIOSC,
    #[doc = "LFIOSC"]
    LFIOSC,
    #[doc = "MOSC"]
    MOSC,
    #[doc = "Hibernation Module RTCOSC"]
    RTC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DSOSCSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DSOSCSRCR::PIOSC => 0,
            DSOSCSRCR::LFIOSC => 2,
            DSOSCSRCR::MOSC => 3,
            DSOSCSRCR::RTC => 4,
            DSOSCSRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DSOSCSRCR {
        match value {
            0 => DSOSCSRCR::PIOSC,
            2 => DSOSCSRCR::LFIOSC,
            3 => DSOSCSRCR::MOSC,
            4 => DSOSCSRCR::RTC,
            i => DSOSCSRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIOSC`"]
    #[inline]
    pub fn is_piosc(&self) -> bool {
        *self == DSOSCSRCR::PIOSC
    }
    #[doc = "Checks if the value of the field is `LFIOSC`"]
    #[inline]
    pub fn is_lfiosc(&self) -> bool {
        *self == DSOSCSRCR::LFIOSC
    }
    #[doc = "Checks if the value of the field is `MOSC`"]
    #[inline]
    pub fn is_mosc(&self) -> bool {
        *self == DSOSCSRCR::MOSC
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline]
    pub fn is_rtc(&self) -> bool {
        *self == DSOSCSRCR::RTC
    }
}
#[doc = r" Value of the field"]
pub struct MOSCDPDR {
    bits: bool,
}
impl MOSCDPDR {
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
pub struct PIOSCPDR {
    bits: bool,
}
impl PIOSCPDR {
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
pub struct _DSSYSDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _DSSYSDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DSOSCSRC`"]
pub enum DSOSCSRCW {
    #[doc = "PIOSC"]
    PIOSC,
    #[doc = "LFIOSC"]
    LFIOSC,
    #[doc = "MOSC"]
    MOSC,
    #[doc = "Hibernation Module RTCOSC"]
    RTC,
}
impl DSOSCSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DSOSCSRCW::PIOSC => 0,
            DSOSCSRCW::LFIOSC => 2,
            DSOSCSRCW::MOSC => 3,
            DSOSCSRCW::RTC => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSOSCSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _DSOSCSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSOSCSRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PIOSC"]
    #[inline]
    pub fn piosc(self) -> &'a mut W {
        self.variant(DSOSCSRCW::PIOSC)
    }
    #[doc = "LFIOSC"]
    #[inline]
    pub fn lfiosc(self) -> &'a mut W {
        self.variant(DSOSCSRCW::LFIOSC)
    }
    #[doc = "MOSC"]
    #[inline]
    pub fn mosc(self) -> &'a mut W {
        self.variant(DSOSCSRCW::MOSC)
    }
    #[doc = "Hibernation Module RTCOSC"]
    #[inline]
    pub fn rtc(self) -> &'a mut W {
        self.variant(DSOSCSRCW::RTC)
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
#[doc = r" Proxy"]
pub struct _MOSCDPDW<'a> {
    w: &'a mut W,
}
impl<'a> _MOSCDPDW<'a> {
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
pub struct _PIOSCPDW<'a> {
    w: &'a mut W,
}
impl<'a> _PIOSCPDW<'a> {
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
    #[doc = "Bits 0:9 - Deep Sleep Clock Divisor"]
    #[inline]
    pub fn dssysdiv(&self) -> DSSYSDIVR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DSSYSDIVR { bits }
    }
    #[doc = "Bits 20:23 - Deep Sleep Oscillator Source"]
    #[inline]
    pub fn dsoscsrc(&self) -> DSOSCSRCR {
        DSOSCSRCR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 30 - MOSC Disable Power Down"]
    #[inline]
    pub fn moscdpd(&self) -> MOSCDPDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MOSCDPDR { bits }
    }
    #[doc = "Bit 31 - PIOSC Power Down"]
    #[inline]
    pub fn pioscpd(&self) -> PIOSCPDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIOSCPDR { bits }
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
    #[doc = "Bits 0:9 - Deep Sleep Clock Divisor"]
    #[inline]
    pub fn dssysdiv(&mut self) -> _DSSYSDIVW {
        _DSSYSDIVW { w: self }
    }
    #[doc = "Bits 20:23 - Deep Sleep Oscillator Source"]
    #[inline]
    pub fn dsoscsrc(&mut self) -> _DSOSCSRCW {
        _DSOSCSRCW { w: self }
    }
    #[doc = "Bit 30 - MOSC Disable Power Down"]
    #[inline]
    pub fn moscdpd(&mut self) -> _MOSCDPDW {
        _MOSCDPDW { w: self }
    }
    #[doc = "Bit 31 - PIOSC Power Down"]
    #[inline]
    pub fn pioscpd(&mut self) -> _PIOSCPDW {
        _PIOSCPDW { w: self }
    }
}
