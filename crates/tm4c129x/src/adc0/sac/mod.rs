#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SAC {
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
#[doc = "Possible values of the field `AVG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVGR {
    #[doc = "No hardware oversampling"]
    OFF,
    #[doc = "2x hardware oversampling"]
    _2X,
    #[doc = "4x hardware oversampling"]
    _4X,
    #[doc = "8x hardware oversampling"]
    _8X,
    #[doc = "16x hardware oversampling"]
    _16X,
    #[doc = "32x hardware oversampling"]
    _32X,
    #[doc = "64x hardware oversampling"]
    _64X,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AVGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AVGR::OFF => 0,
            AVGR::_2X => 1,
            AVGR::_4X => 2,
            AVGR::_8X => 3,
            AVGR::_16X => 4,
            AVGR::_32X => 5,
            AVGR::_64X => 6,
            AVGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AVGR {
        match value {
            0 => AVGR::OFF,
            1 => AVGR::_2X,
            2 => AVGR::_4X,
            3 => AVGR::_8X,
            4 => AVGR::_16X,
            5 => AVGR::_32X,
            6 => AVGR::_64X,
            i => AVGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == AVGR::OFF
    }
    #[doc = "Checks if the value of the field is `_2X`"]
    #[inline]
    pub fn is_2x(&self) -> bool {
        *self == AVGR::_2X
    }
    #[doc = "Checks if the value of the field is `_4X`"]
    #[inline]
    pub fn is_4x(&self) -> bool {
        *self == AVGR::_4X
    }
    #[doc = "Checks if the value of the field is `_8X`"]
    #[inline]
    pub fn is_8x(&self) -> bool {
        *self == AVGR::_8X
    }
    #[doc = "Checks if the value of the field is `_16X`"]
    #[inline]
    pub fn is_16x(&self) -> bool {
        *self == AVGR::_16X
    }
    #[doc = "Checks if the value of the field is `_32X`"]
    #[inline]
    pub fn is_32x(&self) -> bool {
        *self == AVGR::_32X
    }
    #[doc = "Checks if the value of the field is `_64X`"]
    #[inline]
    pub fn is_64x(&self) -> bool {
        *self == AVGR::_64X
    }
}
#[doc = "Values that can be written to the field `AVG`"]
pub enum AVGW {
    #[doc = "No hardware oversampling"]
    OFF,
    #[doc = "2x hardware oversampling"]
    _2X,
    #[doc = "4x hardware oversampling"]
    _4X,
    #[doc = "8x hardware oversampling"]
    _8X,
    #[doc = "16x hardware oversampling"]
    _16X,
    #[doc = "32x hardware oversampling"]
    _32X,
    #[doc = "64x hardware oversampling"]
    _64X,
}
impl AVGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AVGW::OFF => 0,
            AVGW::_2X => 1,
            AVGW::_4X => 2,
            AVGW::_8X => 3,
            AVGW::_16X => 4,
            AVGW::_32X => 5,
            AVGW::_64X => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AVGW<'a> {
    w: &'a mut W,
}
impl<'a> _AVGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AVGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No hardware oversampling"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(AVGW::OFF)
    }
    #[doc = "2x hardware oversampling"]
    #[inline]
    pub fn _2x(self) -> &'a mut W {
        self.variant(AVGW::_2X)
    }
    #[doc = "4x hardware oversampling"]
    #[inline]
    pub fn _4x(self) -> &'a mut W {
        self.variant(AVGW::_4X)
    }
    #[doc = "8x hardware oversampling"]
    #[inline]
    pub fn _8x(self) -> &'a mut W {
        self.variant(AVGW::_8X)
    }
    #[doc = "16x hardware oversampling"]
    #[inline]
    pub fn _16x(self) -> &'a mut W {
        self.variant(AVGW::_16X)
    }
    #[doc = "32x hardware oversampling"]
    #[inline]
    pub fn _32x(self) -> &'a mut W {
        self.variant(AVGW::_32X)
    }
    #[doc = "64x hardware oversampling"]
    #[inline]
    pub fn _64x(self) -> &'a mut W {
        self.variant(AVGW::_64X)
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
    #[doc = "Bits 0:2 - Hardware Averaging Control"]
    #[inline]
    pub fn avg(&self) -> AVGR {
        AVGR::_from({
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
    #[doc = "Bits 0:2 - Hardware Averaging Control"]
    #[inline]
    pub fn avg(&mut self) -> _AVGW {
        _AVGW { w: self }
    }
}
