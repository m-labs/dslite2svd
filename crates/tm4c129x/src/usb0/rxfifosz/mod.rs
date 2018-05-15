#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::RXFIFOSZ {
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
#[doc = "Possible values of the field `SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZER {
    #[doc = "8"]
    _8,
    #[doc = "16"]
    _16,
    #[doc = "32"]
    _32,
    #[doc = "64"]
    _64,
    #[doc = "128"]
    _128,
    #[doc = "256"]
    _256,
    #[doc = "512"]
    _512,
    #[doc = "1024"]
    _1024,
    #[doc = "2048"]
    _2048,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SIZER::_8 => 0,
            SIZER::_16 => 1,
            SIZER::_32 => 2,
            SIZER::_64 => 3,
            SIZER::_128 => 4,
            SIZER::_256 => 5,
            SIZER::_512 => 6,
            SIZER::_1024 => 7,
            SIZER::_2048 => 8,
            SIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SIZER {
        match value {
            0 => SIZER::_8,
            1 => SIZER::_16,
            2 => SIZER::_32,
            3 => SIZER::_64,
            4 => SIZER::_128,
            5 => SIZER::_256,
            6 => SIZER::_512,
            7 => SIZER::_1024,
            8 => SIZER::_2048,
            i => SIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == SIZER::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == SIZER::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == SIZER::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == SIZER::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == SIZER::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == SIZER::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline]
    pub fn is_512(&self) -> bool {
        *self == SIZER::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline]
    pub fn is_1024(&self) -> bool {
        *self == SIZER::_1024
    }
    #[doc = "Checks if the value of the field is `_2048`"]
    #[inline]
    pub fn is_2048(&self) -> bool {
        *self == SIZER::_2048
    }
}
#[doc = r" Value of the field"]
pub struct DPBR {
    bits: bool,
}
impl DPBR {
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
#[doc = "Values that can be written to the field `SIZE`"]
pub enum SIZEW {
    #[doc = "8"]
    _8,
    #[doc = "16"]
    _16,
    #[doc = "32"]
    _32,
    #[doc = "64"]
    _64,
    #[doc = "128"]
    _128,
    #[doc = "256"]
    _256,
    #[doc = "512"]
    _512,
    #[doc = "1024"]
    _1024,
    #[doc = "2048"]
    _2048,
}
impl SIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SIZEW::_8 => 0,
            SIZEW::_16 => 1,
            SIZEW::_32 => 2,
            SIZEW::_64 => 3,
            SIZEW::_128 => 4,
            SIZEW::_256 => 5,
            SIZEW::_512 => 6,
            SIZEW::_1024 => 7,
            SIZEW::_2048 => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _SIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(SIZEW::_8)
    }
    #[doc = "16"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(SIZEW::_16)
    }
    #[doc = "32"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(SIZEW::_32)
    }
    #[doc = "64"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(SIZEW::_64)
    }
    #[doc = "128"]
    #[inline]
    pub fn _128(self) -> &'a mut W {
        self.variant(SIZEW::_128)
    }
    #[doc = "256"]
    #[inline]
    pub fn _256(self) -> &'a mut W {
        self.variant(SIZEW::_256)
    }
    #[doc = "512"]
    #[inline]
    pub fn _512(self) -> &'a mut W {
        self.variant(SIZEW::_512)
    }
    #[doc = "1024"]
    #[inline]
    pub fn _1024(self) -> &'a mut W {
        self.variant(SIZEW::_1024)
    }
    #[doc = "2048"]
    #[inline]
    pub fn _2048(self) -> &'a mut W {
        self.variant(SIZEW::_2048)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DPBW<'a> {
    w: &'a mut W,
}
impl<'a> _DPBW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:3 - Max Packet Size"]
    #[inline]
    pub fn size(&self) -> SIZER {
        SIZER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 4 - Double Packet Buffer Support"]
    #[inline]
    pub fn dpb(&self) -> DPBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        DPBR { bits }
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Max Packet Size"]
    #[inline]
    pub fn size(&mut self) -> _SIZEW {
        _SIZEW { w: self }
    }
    #[doc = "Bit 4 - Double Packet Buffer Support"]
    #[inline]
    pub fn dpb(&mut self) -> _DPBW {
        _DPBW { w: self }
    }
}
