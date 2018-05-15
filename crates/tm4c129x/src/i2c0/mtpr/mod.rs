#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MTPR {
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
pub struct TPRR {
    bits: u8,
}
impl TPRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HSR {
    bits: bool,
}
impl HSR {
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
#[doc = "Possible values of the field `PULSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PULSELR {
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
impl PULSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PULSELR::BYPASS => 0,
            PULSELR::_1 => 1,
            PULSELR::_2 => 2,
            PULSELR::_3 => 3,
            PULSELR::_4 => 4,
            PULSELR::_8 => 5,
            PULSELR::_16 => 6,
            PULSELR::_31 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PULSELR {
        match value {
            0 => PULSELR::BYPASS,
            1 => PULSELR::_1,
            2 => PULSELR::_2,
            3 => PULSELR::_3,
            4 => PULSELR::_4,
            5 => PULSELR::_8,
            6 => PULSELR::_16,
            7 => PULSELR::_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline]
    pub fn is_bypass(&self) -> bool {
        *self == PULSELR::BYPASS
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PULSELR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == PULSELR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == PULSELR::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == PULSELR::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == PULSELR::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == PULSELR::_16
    }
    #[doc = "Checks if the value of the field is `_31`"]
    #[inline]
    pub fn is_31(&self) -> bool {
        *self == PULSELR::_31
    }
}
#[doc = r" Proxy"]
pub struct _TPRW<'a> {
    w: &'a mut W,
}
impl<'a> _TPRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HSW<'a> {
    w: &'a mut W,
}
impl<'a> _HSW<'a> {
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
#[doc = "Values that can be written to the field `PULSEL`"]
pub enum PULSELW {
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
impl PULSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PULSELW::BYPASS => 0,
            PULSELW::_1 => 1,
            PULSELW::_2 => 2,
            PULSELW::_3 => 3,
            PULSELW::_4 => 4,
            PULSELW::_8 => 5,
            PULSELW::_16 => 6,
            PULSELW::_31 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PULSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PULSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PULSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Bypass"]
    #[inline]
    pub fn bypass(self) -> &'a mut W {
        self.variant(PULSELW::BYPASS)
    }
    #[doc = "1 clock"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PULSELW::_1)
    }
    #[doc = "2 clocks"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(PULSELW::_2)
    }
    #[doc = "3 clocks"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(PULSELW::_3)
    }
    #[doc = "4 clocks"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(PULSELW::_4)
    }
    #[doc = "8 clocks"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(PULSELW::_8)
    }
    #[doc = "16 clocks"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(PULSELW::_16)
    }
    #[doc = "31 clocks"]
    #[inline]
    pub fn _31(self) -> &'a mut W {
        self.variant(PULSELW::_31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:6 - Timer Period"]
    #[inline]
    pub fn tpr(&self) -> TPRR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TPRR { bits }
    }
    #[doc = "Bit 7 - High-Speed Enable"]
    #[inline]
    pub fn hs(&self) -> HSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSR { bits }
    }
    #[doc = "Bits 16:18 - Glitch Suppression Pulse Width"]
    #[inline]
    pub fn pulsel(&self) -> PULSELR {
        PULSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:6 - Timer Period"]
    #[inline]
    pub fn tpr(&mut self) -> _TPRW {
        _TPRW { w: self }
    }
    #[doc = "Bit 7 - High-Speed Enable"]
    #[inline]
    pub fn hs(&mut self) -> _HSW {
        _HSW { w: self }
    }
    #[doc = "Bits 16:18 - Glitch Suppression Pulse Width"]
    #[inline]
    pub fn pulsel(&mut self) -> _PULSELW {
        _PULSELW { w: self }
    }
}
