#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PC {
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
#[doc = "Possible values of the field `MCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCRR {
    #[doc = "Eighth conversion rate. After a conversion completes, the logic pauses for 112 TADC periods before starting the next conversion"]
    _1_8,
    #[doc = "Quarter conversion rate. After a conversion completes, the logic pauses for 48 TADC periods before starting the next conversion"]
    _1_4,
    #[doc = "Half conversion rate. After a conversion completes, the logic pauses for 16 TADC periods before starting the next conversion"]
    _1_2,
    #[doc = "Full conversion rate (FCONV) as defined by TADC and NSH"]
    FULL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MCRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MCRR::_1_8 => 1,
            MCRR::_1_4 => 3,
            MCRR::_1_2 => 5,
            MCRR::FULL => 7,
            MCRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MCRR {
        match value {
            1 => MCRR::_1_8,
            3 => MCRR::_1_4,
            5 => MCRR::_1_2,
            7 => MCRR::FULL,
            i => MCRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1_8`"]
    #[inline]
    pub fn is_1_8(&self) -> bool {
        *self == MCRR::_1_8
    }
    #[doc = "Checks if the value of the field is `_1_4`"]
    #[inline]
    pub fn is_1_4(&self) -> bool {
        *self == MCRR::_1_4
    }
    #[doc = "Checks if the value of the field is `_1_2`"]
    #[inline]
    pub fn is_1_2(&self) -> bool {
        *self == MCRR::_1_2
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline]
    pub fn is_full(&self) -> bool {
        *self == MCRR::FULL
    }
}
#[doc = "Values that can be written to the field `MCR`"]
pub enum MCRW {
    #[doc = "Eighth conversion rate. After a conversion completes, the logic pauses for 112 TADC periods before starting the next conversion"]
    _1_8,
    #[doc = "Quarter conversion rate. After a conversion completes, the logic pauses for 48 TADC periods before starting the next conversion"]
    _1_4,
    #[doc = "Half conversion rate. After a conversion completes, the logic pauses for 16 TADC periods before starting the next conversion"]
    _1_2,
    #[doc = "Full conversion rate (FCONV) as defined by TADC and NSH"]
    FULL,
}
impl MCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MCRW::_1_8 => 1,
            MCRW::_1_4 => 3,
            MCRW::_1_2 => 5,
            MCRW::FULL => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCRW<'a> {
    w: &'a mut W,
}
impl<'a> _MCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Eighth conversion rate. After a conversion completes, the logic pauses for 112 TADC periods before starting the next conversion"]
    #[inline]
    pub fn _1_8(self) -> &'a mut W {
        self.variant(MCRW::_1_8)
    }
    #[doc = "Quarter conversion rate. After a conversion completes, the logic pauses for 48 TADC periods before starting the next conversion"]
    #[inline]
    pub fn _1_4(self) -> &'a mut W {
        self.variant(MCRW::_1_4)
    }
    #[doc = "Half conversion rate. After a conversion completes, the logic pauses for 16 TADC periods before starting the next conversion"]
    #[inline]
    pub fn _1_2(self) -> &'a mut W {
        self.variant(MCRW::_1_2)
    }
    #[doc = "Full conversion rate (FCONV) as defined by TADC and NSH"]
    #[inline]
    pub fn full(self) -> &'a mut W {
        self.variant(MCRW::FULL)
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
    #[doc = "Bits 0:3 - Conversion Rate"]
    #[inline]
    pub fn mcr(&self) -> MCRR {
        MCRR::_from({
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
    #[doc = "Bits 0:3 - Conversion Rate"]
    #[inline]
    pub fn mcr(&mut self) -> _MCRW {
        _MCRW { w: self }
    }
}
