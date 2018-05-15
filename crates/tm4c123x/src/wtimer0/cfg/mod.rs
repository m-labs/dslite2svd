#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
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
#[doc = "Possible values of the field `CFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGR {
    #[doc = "For a 16/32-bit timer, this value selects the 32-bit timer configuration"]
    _32_BIT_TIMER,
    #[doc = "For a 16/32-bit timer, this value selects the 32-bit real-time clock (RTC) counter configuration"]
    _32_BIT_RTC,
    #[doc = "For a 16/32-bit timer, this value selects the 16-bit timer configuration"]
    _16_BIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFGR::_32_BIT_TIMER => 0,
            CFGR::_32_BIT_RTC => 1,
            CFGR::_16_BIT => 4,
            CFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFGR {
        match value {
            0 => CFGR::_32_BIT_TIMER,
            1 => CFGR::_32_BIT_RTC,
            4 => CFGR::_16_BIT,
            i => CFGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_32_BIT_TIMER`"]
    #[inline]
    pub fn is_32_bit_timer(&self) -> bool {
        *self == CFGR::_32_BIT_TIMER
    }
    #[doc = "Checks if the value of the field is `_32_BIT_RTC`"]
    #[inline]
    pub fn is_32_bit_rtc(&self) -> bool {
        *self == CFGR::_32_BIT_RTC
    }
    #[doc = "Checks if the value of the field is `_16_BIT`"]
    #[inline]
    pub fn is_16_bit(&self) -> bool {
        *self == CFGR::_16_BIT
    }
}
#[doc = "Values that can be written to the field `CFG`"]
pub enum CFGW {
    #[doc = "For a 16/32-bit timer, this value selects the 32-bit timer configuration"]
    _32_BIT_TIMER,
    #[doc = "For a 16/32-bit timer, this value selects the 32-bit real-time clock (RTC) counter configuration"]
    _32_BIT_RTC,
    #[doc = "For a 16/32-bit timer, this value selects the 16-bit timer configuration"]
    _16_BIT,
}
impl CFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFGW::_32_BIT_TIMER => 0,
            CFGW::_32_BIT_RTC => 1,
            CFGW::_16_BIT => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _CFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "For a 16/32-bit timer, this value selects the 32-bit timer configuration"]
    #[inline]
    pub fn _32_bit_timer(self) -> &'a mut W {
        self.variant(CFGW::_32_BIT_TIMER)
    }
    #[doc = "For a 16/32-bit timer, this value selects the 32-bit real-time clock (RTC) counter configuration"]
    #[inline]
    pub fn _32_bit_rtc(self) -> &'a mut W {
        self.variant(CFGW::_32_BIT_RTC)
    }
    #[doc = "For a 16/32-bit timer, this value selects the 16-bit timer configuration"]
    #[inline]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(CFGW::_16_BIT)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - GPTM Configuration"]
    #[inline]
    pub fn cfg(&self) -> CFGR {
        CFGR::_from({
            const MASK: u8 = 7;
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
    #[doc = "Bits 0:2 - GPTM Configuration"]
    #[inline]
    pub fn cfg(&mut self) -> _CFGW {
        _CFGW { w: self }
    }
}
