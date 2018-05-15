#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LDODPCTL {
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
#[doc = "Possible values of the field `VLDO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLDOR {
    #[doc = "0.90 V"]
    _0_90V,
    #[doc = "0.95 V"]
    _0_95V,
    #[doc = "1.00 V"]
    _1_00V,
    #[doc = "1.05 V"]
    _1_05V,
    #[doc = "1.10 V"]
    _1_10V,
    #[doc = "1.15 V"]
    _1_15V,
    #[doc = "1.20 V"]
    _1_20V,
    #[doc = "1.25 V"]
    _1_25V,
    #[doc = "1.30 V"]
    _1_30V,
    #[doc = "1.35 V"]
    _1_35V,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl VLDOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VLDOR::_0_90V => 18,
            VLDOR::_0_95V => 19,
            VLDOR::_1_00V => 20,
            VLDOR::_1_05V => 21,
            VLDOR::_1_10V => 22,
            VLDOR::_1_15V => 23,
            VLDOR::_1_20V => 24,
            VLDOR::_1_25V => 25,
            VLDOR::_1_30V => 26,
            VLDOR::_1_35V => 27,
            VLDOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VLDOR {
        match value {
            18 => VLDOR::_0_90V,
            19 => VLDOR::_0_95V,
            20 => VLDOR::_1_00V,
            21 => VLDOR::_1_05V,
            22 => VLDOR::_1_10V,
            23 => VLDOR::_1_15V,
            24 => VLDOR::_1_20V,
            25 => VLDOR::_1_25V,
            26 => VLDOR::_1_30V,
            27 => VLDOR::_1_35V,
            i => VLDOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0_90V`"]
    #[inline]
    pub fn is_0_90v(&self) -> bool {
        *self == VLDOR::_0_90V
    }
    #[doc = "Checks if the value of the field is `_0_95V`"]
    #[inline]
    pub fn is_0_95v(&self) -> bool {
        *self == VLDOR::_0_95V
    }
    #[doc = "Checks if the value of the field is `_1_00V`"]
    #[inline]
    pub fn is_1_00v(&self) -> bool {
        *self == VLDOR::_1_00V
    }
    #[doc = "Checks if the value of the field is `_1_05V`"]
    #[inline]
    pub fn is_1_05v(&self) -> bool {
        *self == VLDOR::_1_05V
    }
    #[doc = "Checks if the value of the field is `_1_10V`"]
    #[inline]
    pub fn is_1_10v(&self) -> bool {
        *self == VLDOR::_1_10V
    }
    #[doc = "Checks if the value of the field is `_1_15V`"]
    #[inline]
    pub fn is_1_15v(&self) -> bool {
        *self == VLDOR::_1_15V
    }
    #[doc = "Checks if the value of the field is `_1_20V`"]
    #[inline]
    pub fn is_1_20v(&self) -> bool {
        *self == VLDOR::_1_20V
    }
    #[doc = "Checks if the value of the field is `_1_25V`"]
    #[inline]
    pub fn is_1_25v(&self) -> bool {
        *self == VLDOR::_1_25V
    }
    #[doc = "Checks if the value of the field is `_1_30V`"]
    #[inline]
    pub fn is_1_30v(&self) -> bool {
        *self == VLDOR::_1_30V
    }
    #[doc = "Checks if the value of the field is `_1_35V`"]
    #[inline]
    pub fn is_1_35v(&self) -> bool {
        *self == VLDOR::_1_35V
    }
}
#[doc = r" Value of the field"]
pub struct VADJENR {
    bits: bool,
}
impl VADJENR {
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
#[doc = "Values that can be written to the field `VLDO`"]
pub enum VLDOW {
    #[doc = "0.90 V"]
    _0_90V,
    #[doc = "0.95 V"]
    _0_95V,
    #[doc = "1.00 V"]
    _1_00V,
    #[doc = "1.05 V"]
    _1_05V,
    #[doc = "1.10 V"]
    _1_10V,
    #[doc = "1.15 V"]
    _1_15V,
    #[doc = "1.20 V"]
    _1_20V,
    #[doc = "1.25 V"]
    _1_25V,
    #[doc = "1.30 V"]
    _1_30V,
    #[doc = "1.35 V"]
    _1_35V,
}
impl VLDOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VLDOW::_0_90V => 18,
            VLDOW::_0_95V => 19,
            VLDOW::_1_00V => 20,
            VLDOW::_1_05V => 21,
            VLDOW::_1_10V => 22,
            VLDOW::_1_15V => 23,
            VLDOW::_1_20V => 24,
            VLDOW::_1_25V => 25,
            VLDOW::_1_30V => 26,
            VLDOW::_1_35V => 27,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VLDOW<'a> {
    w: &'a mut W,
}
impl<'a> _VLDOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VLDOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "0.90 V"]
    #[inline]
    pub fn _0_90v(self) -> &'a mut W {
        self.variant(VLDOW::_0_90V)
    }
    #[doc = "0.95 V"]
    #[inline]
    pub fn _0_95v(self) -> &'a mut W {
        self.variant(VLDOW::_0_95V)
    }
    #[doc = "1.00 V"]
    #[inline]
    pub fn _1_00v(self) -> &'a mut W {
        self.variant(VLDOW::_1_00V)
    }
    #[doc = "1.05 V"]
    #[inline]
    pub fn _1_05v(self) -> &'a mut W {
        self.variant(VLDOW::_1_05V)
    }
    #[doc = "1.10 V"]
    #[inline]
    pub fn _1_10v(self) -> &'a mut W {
        self.variant(VLDOW::_1_10V)
    }
    #[doc = "1.15 V"]
    #[inline]
    pub fn _1_15v(self) -> &'a mut W {
        self.variant(VLDOW::_1_15V)
    }
    #[doc = "1.20 V"]
    #[inline]
    pub fn _1_20v(self) -> &'a mut W {
        self.variant(VLDOW::_1_20V)
    }
    #[doc = "1.25 V"]
    #[inline]
    pub fn _1_25v(self) -> &'a mut W {
        self.variant(VLDOW::_1_25V)
    }
    #[doc = "1.30 V"]
    #[inline]
    pub fn _1_30v(self) -> &'a mut W {
        self.variant(VLDOW::_1_30V)
    }
    #[doc = "1.35 V"]
    #[inline]
    pub fn _1_35v(self) -> &'a mut W {
        self.variant(VLDOW::_1_35V)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VADJENW<'a> {
    w: &'a mut W,
}
impl<'a> _VADJENW<'a> {
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
    #[doc = "Bits 0:7 - LDO Output Voltage"]
    #[inline]
    pub fn vldo(&self) -> VLDOR {
        VLDOR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - Voltage Adjust Enable"]
    #[inline]
    pub fn vadjen(&self) -> VADJENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VADJENR { bits }
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
    #[doc = "Bits 0:7 - LDO Output Voltage"]
    #[inline]
    pub fn vldo(&mut self) -> _VLDOW {
        _VLDOW { w: self }
    }
    #[doc = "Bit 31 - Voltage Adjust Enable"]
    #[inline]
    pub fn vadjen(&mut self) -> _VADJENW {
        _VADJENW { w: self }
    }
}
