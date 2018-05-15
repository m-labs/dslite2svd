#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ALTCLKCFG {
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
#[doc = "Possible values of the field `ALTCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALTCLKR {
    #[doc = "PIOSC"]
    PIOSC,
    #[doc = "Hibernation Module Real-time clock output (RTCOSC)"]
    RTCOSC,
    #[doc = "Low-frequency internal oscillator (LFIOSC)"]
    LFIOSC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ALTCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ALTCLKR::PIOSC => 0,
            ALTCLKR::RTCOSC => 3,
            ALTCLKR::LFIOSC => 4,
            ALTCLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ALTCLKR {
        match value {
            0 => ALTCLKR::PIOSC,
            3 => ALTCLKR::RTCOSC,
            4 => ALTCLKR::LFIOSC,
            i => ALTCLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIOSC`"]
    #[inline]
    pub fn is_piosc(&self) -> bool {
        *self == ALTCLKR::PIOSC
    }
    #[doc = "Checks if the value of the field is `RTCOSC`"]
    #[inline]
    pub fn is_rtcosc(&self) -> bool {
        *self == ALTCLKR::RTCOSC
    }
    #[doc = "Checks if the value of the field is `LFIOSC`"]
    #[inline]
    pub fn is_lfiosc(&self) -> bool {
        *self == ALTCLKR::LFIOSC
    }
}
#[doc = "Values that can be written to the field `ALTCLK`"]
pub enum ALTCLKW {
    #[doc = "PIOSC"]
    PIOSC,
    #[doc = "Hibernation Module Real-time clock output (RTCOSC)"]
    RTCOSC,
    #[doc = "Low-frequency internal oscillator (LFIOSC)"]
    LFIOSC,
}
impl ALTCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ALTCLKW::PIOSC => 0,
            ALTCLKW::RTCOSC => 3,
            ALTCLKW::LFIOSC => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALTCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _ALTCLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALTCLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PIOSC"]
    #[inline]
    pub fn piosc(self) -> &'a mut W {
        self.variant(ALTCLKW::PIOSC)
    }
    #[doc = "Hibernation Module Real-time clock output (RTCOSC)"]
    #[inline]
    pub fn rtcosc(self) -> &'a mut W {
        self.variant(ALTCLKW::RTCOSC)
    }
    #[doc = "Low-frequency internal oscillator (LFIOSC)"]
    #[inline]
    pub fn lfiosc(self) -> &'a mut W {
        self.variant(ALTCLKW::LFIOSC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:3 - Alternate Clock Source"]
    #[inline]
    pub fn altclk(&self) -> ALTCLKR {
        ALTCLKR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:3 - Alternate Clock Source"]
    #[inline]
    pub fn altclk(&mut self) -> _ALTCLKW {
        _ALTCLKW { w: self }
    }
}
