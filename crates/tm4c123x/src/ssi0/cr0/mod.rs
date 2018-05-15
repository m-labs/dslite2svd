#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR0 {
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
#[doc = "Possible values of the field `DSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSSR {
    #[doc = "4-bit data"]
    _4,
    #[doc = "5-bit data"]
    _5,
    #[doc = "6-bit data"]
    _6,
    #[doc = "7-bit data"]
    _7,
    #[doc = "8-bit data"]
    _8,
    #[doc = "9-bit data"]
    _9,
    #[doc = "10-bit data"]
    _10,
    #[doc = "11-bit data"]
    _11,
    #[doc = "12-bit data"]
    _12,
    #[doc = "13-bit data"]
    _13,
    #[doc = "14-bit data"]
    _14,
    #[doc = "15-bit data"]
    _15,
    #[doc = "16-bit data"]
    _16,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DSSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DSSR::_4 => 3,
            DSSR::_5 => 4,
            DSSR::_6 => 5,
            DSSR::_7 => 6,
            DSSR::_8 => 7,
            DSSR::_9 => 8,
            DSSR::_10 => 9,
            DSSR::_11 => 10,
            DSSR::_12 => 11,
            DSSR::_13 => 12,
            DSSR::_14 => 13,
            DSSR::_15 => 14,
            DSSR::_16 => 15,
            DSSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DSSR {
        match value {
            3 => DSSR::_4,
            4 => DSSR::_5,
            5 => DSSR::_6,
            6 => DSSR::_7,
            7 => DSSR::_8,
            8 => DSSR::_9,
            9 => DSSR::_10,
            10 => DSSR::_11,
            11 => DSSR::_12,
            12 => DSSR::_13,
            13 => DSSR::_14,
            14 => DSSR::_15,
            15 => DSSR::_16,
            i => DSSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == DSSR::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == DSSR::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == DSSR::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == DSSR::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == DSSR::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline]
    pub fn is_9(&self) -> bool {
        *self == DSSR::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == DSSR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == DSSR::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline]
    pub fn is_12(&self) -> bool {
        *self == DSSR::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline]
    pub fn is_13(&self) -> bool {
        *self == DSSR::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline]
    pub fn is_14(&self) -> bool {
        *self == DSSR::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline]
    pub fn is_15(&self) -> bool {
        *self == DSSR::_15
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == DSSR::_16
    }
}
#[doc = "Possible values of the field `FRF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRFR {
    #[doc = "Freescale SPI Frame Format"]
    MOTO,
    #[doc = "Synchronous Serial Frame Format"]
    TI,
    #[doc = "MICROWIRE Frame Format"]
    NMW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FRFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FRFR::MOTO => 0,
            FRFR::TI => 1,
            FRFR::NMW => 2,
            FRFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FRFR {
        match value {
            0 => FRFR::MOTO,
            1 => FRFR::TI,
            2 => FRFR::NMW,
            i => FRFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MOTO`"]
    #[inline]
    pub fn is_moto(&self) -> bool {
        *self == FRFR::MOTO
    }
    #[doc = "Checks if the value of the field is `TI`"]
    #[inline]
    pub fn is_ti(&self) -> bool {
        *self == FRFR::TI
    }
    #[doc = "Checks if the value of the field is `NMW`"]
    #[inline]
    pub fn is_nmw(&self) -> bool {
        *self == FRFR::NMW
    }
}
#[doc = r" Value of the field"]
pub struct SPOR {
    bits: bool,
}
impl SPOR {
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
pub struct SPHR {
    bits: bool,
}
impl SPHR {
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
pub struct SCRR {
    bits: u8,
}
impl SCRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `DSS`"]
pub enum DSSW {
    #[doc = "4-bit data"]
    _4,
    #[doc = "5-bit data"]
    _5,
    #[doc = "6-bit data"]
    _6,
    #[doc = "7-bit data"]
    _7,
    #[doc = "8-bit data"]
    _8,
    #[doc = "9-bit data"]
    _9,
    #[doc = "10-bit data"]
    _10,
    #[doc = "11-bit data"]
    _11,
    #[doc = "12-bit data"]
    _12,
    #[doc = "13-bit data"]
    _13,
    #[doc = "14-bit data"]
    _14,
    #[doc = "15-bit data"]
    _15,
    #[doc = "16-bit data"]
    _16,
}
impl DSSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DSSW::_4 => 3,
            DSSW::_5 => 4,
            DSSW::_6 => 5,
            DSSW::_7 => 6,
            DSSW::_8 => 7,
            DSSW::_9 => 8,
            DSSW::_10 => 9,
            DSSW::_11 => 10,
            DSSW::_12 => 11,
            DSSW::_13 => 12,
            DSSW::_14 => 13,
            DSSW::_15 => 14,
            DSSW::_16 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSSW<'a> {
    w: &'a mut W,
}
impl<'a> _DSSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "4-bit data"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(DSSW::_4)
    }
    #[doc = "5-bit data"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(DSSW::_5)
    }
    #[doc = "6-bit data"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(DSSW::_6)
    }
    #[doc = "7-bit data"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(DSSW::_7)
    }
    #[doc = "8-bit data"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(DSSW::_8)
    }
    #[doc = "9-bit data"]
    #[inline]
    pub fn _9(self) -> &'a mut W {
        self.variant(DSSW::_9)
    }
    #[doc = "10-bit data"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(DSSW::_10)
    }
    #[doc = "11-bit data"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(DSSW::_11)
    }
    #[doc = "12-bit data"]
    #[inline]
    pub fn _12(self) -> &'a mut W {
        self.variant(DSSW::_12)
    }
    #[doc = "13-bit data"]
    #[inline]
    pub fn _13(self) -> &'a mut W {
        self.variant(DSSW::_13)
    }
    #[doc = "14-bit data"]
    #[inline]
    pub fn _14(self) -> &'a mut W {
        self.variant(DSSW::_14)
    }
    #[doc = "15-bit data"]
    #[inline]
    pub fn _15(self) -> &'a mut W {
        self.variant(DSSW::_15)
    }
    #[doc = "16-bit data"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(DSSW::_16)
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
#[doc = "Values that can be written to the field `FRF`"]
pub enum FRFW {
    #[doc = "Freescale SPI Frame Format"]
    MOTO,
    #[doc = "Synchronous Serial Frame Format"]
    TI,
    #[doc = "MICROWIRE Frame Format"]
    NMW,
}
impl FRFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FRFW::MOTO => 0,
            FRFW::TI => 1,
            FRFW::NMW => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRFW<'a> {
    w: &'a mut W,
}
impl<'a> _FRFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Freescale SPI Frame Format"]
    #[inline]
    pub fn moto(self) -> &'a mut W {
        self.variant(FRFW::MOTO)
    }
    #[doc = "Synchronous Serial Frame Format"]
    #[inline]
    pub fn ti(self) -> &'a mut W {
        self.variant(FRFW::TI)
    }
    #[doc = "MICROWIRE Frame Format"]
    #[inline]
    pub fn nmw(self) -> &'a mut W {
        self.variant(FRFW::NMW)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPOW<'a> {
    w: &'a mut W,
}
impl<'a> _SPOW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPHW<'a> {
    w: &'a mut W,
}
impl<'a> _SPHW<'a> {
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
#[doc = r" Proxy"]
pub struct _SCRW<'a> {
    w: &'a mut W,
}
impl<'a> _SCRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:3 - SSI Data Size Select"]
    #[inline]
    pub fn dss(&self) -> DSSR {
        DSSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - SSI Frame Format Select"]
    #[inline]
    pub fn frf(&self) -> FRFR {
        FRFR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - SSI Serial Clock Polarity"]
    #[inline]
    pub fn spo(&self) -> SPOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SPOR { bits }
    }
    #[doc = "Bit 7 - SSI Serial Clock Phase"]
    #[inline]
    pub fn sph(&self) -> SPHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SPHR { bits }
    }
    #[doc = "Bits 8:15 - SSI Serial Clock Rate"]
    #[inline]
    pub fn scr(&self) -> SCRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SCRR { bits }
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
    #[doc = "Bits 0:3 - SSI Data Size Select"]
    #[inline]
    pub fn dss(&mut self) -> _DSSW {
        _DSSW { w: self }
    }
    #[doc = "Bits 4:5 - SSI Frame Format Select"]
    #[inline]
    pub fn frf(&mut self) -> _FRFW {
        _FRFW { w: self }
    }
    #[doc = "Bit 6 - SSI Serial Clock Polarity"]
    #[inline]
    pub fn spo(&mut self) -> _SPOW {
        _SPOW { w: self }
    }
    #[doc = "Bit 7 - SSI Serial Clock Phase"]
    #[inline]
    pub fn sph(&mut self) -> _SPHW {
        _SPHW { w: self }
    }
    #[doc = "Bits 8:15 - SSI Serial Clock Rate"]
    #[inline]
    pub fn scr(&mut self) -> _SCRW {
        _SCRW { w: self }
    }
}
