#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCR2 {
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
#[doc = "Possible values of the field `GFPW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GFPWR {
    #[doc = "Bypass"]
    BYPASS,
    #[doc = "1 clock"]
    _1,
    #[doc = "2 clocks"]
    _2,
    #[doc = "3 clocks"]
    _3,
    #[doc = "4 clocks"]
    _4,
    #[doc = "8 clocks"]
    _8,
    #[doc = "16 clocks"]
    _16,
    #[doc = "31 clocks"]
    _31,
}
impl GFPWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GFPWR::BYPASS => 0,
            GFPWR::_1 => 1,
            GFPWR::_2 => 2,
            GFPWR::_3 => 3,
            GFPWR::_4 => 4,
            GFPWR::_8 => 5,
            GFPWR::_16 => 6,
            GFPWR::_31 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GFPWR {
        match value {
            0 => GFPWR::BYPASS,
            1 => GFPWR::_1,
            2 => GFPWR::_2,
            3 => GFPWR::_3,
            4 => GFPWR::_4,
            5 => GFPWR::_8,
            6 => GFPWR::_16,
            7 => GFPWR::_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline]
    pub fn is_bypass(&self) -> bool {
        *self == GFPWR::BYPASS
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == GFPWR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == GFPWR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == GFPWR::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == GFPWR::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == GFPWR::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == GFPWR::_16
    }
    #[doc = "Checks if the value of the field is `_31`"]
    #[inline]
    pub fn is_31(&self) -> bool {
        *self == GFPWR::_31
    }
}
#[doc = "Values that can be written to the field `GFPW`"]
pub enum GFPWW {
    #[doc = "Bypass"]
    BYPASS,
    #[doc = "1 clock"]
    _1,
    #[doc = "2 clocks"]
    _2,
    #[doc = "3 clocks"]
    _3,
    #[doc = "4 clocks"]
    _4,
    #[doc = "8 clocks"]
    _8,
    #[doc = "16 clocks"]
    _16,
    #[doc = "31 clocks"]
    _31,
}
impl GFPWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GFPWW::BYPASS => 0,
            GFPWW::_1 => 1,
            GFPWW::_2 => 2,
            GFPWW::_3 => 3,
            GFPWW::_4 => 4,
            GFPWW::_8 => 5,
            GFPWW::_16 => 6,
            GFPWW::_31 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GFPWW<'a> {
    w: &'a mut W,
}
impl<'a> _GFPWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GFPWW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Bypass"]
    #[inline]
    pub fn bypass(self) -> &'a mut W {
        self.variant(GFPWW::BYPASS)
    }
    #[doc = "1 clock"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GFPWW::_1)
    }
    #[doc = "2 clocks"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(GFPWW::_2)
    }
    #[doc = "3 clocks"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(GFPWW::_3)
    }
    #[doc = "4 clocks"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(GFPWW::_4)
    }
    #[doc = "8 clocks"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(GFPWW::_8)
    }
    #[doc = "16 clocks"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(GFPWW::_16)
    }
    #[doc = "31 clocks"]
    #[inline]
    pub fn _31(self) -> &'a mut W {
        self.variant(GFPWW::_31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 4:6 - I2C Glitch Filter Pulse Width"]
    #[inline]
    pub fn gfpw(&self) -> GFPWR {
        GFPWR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
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
    #[doc = "Bits 4:6 - I2C Glitch Filter Pulse Width"]
    #[inline]
    pub fn gfpw(&mut self) -> _GFPWW {
        _GFPWW { w: self }
    }
}
