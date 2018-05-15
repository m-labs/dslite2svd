#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HB16TIME3 {
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
pub struct RDWSMR {
    bits: bool,
}
impl RDWSMR {
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
pub struct WRWSMR {
    bits: bool,
}
impl WRWSMR {
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
pub struct CAPWIDTHR {
    bits: u8,
}
impl CAPWIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PSRAMSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSRAMSZR {
    #[doc = "No row size limitation"]
    _0,
    #[doc = "128 B"]
    _128B,
    #[doc = "256 B"]
    _256B,
    #[doc = "512 B"]
    _512B,
    #[doc = "1024 B"]
    _1KB,
    #[doc = "2048 B"]
    _2KB,
    #[doc = "4096 B"]
    _4KB,
    #[doc = "8192 B"]
    _8KB,
}
impl PSRAMSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSRAMSZR::_0 => 0,
            PSRAMSZR::_128B => 1,
            PSRAMSZR::_256B => 2,
            PSRAMSZR::_512B => 3,
            PSRAMSZR::_1KB => 4,
            PSRAMSZR::_2KB => 5,
            PSRAMSZR::_4KB => 6,
            PSRAMSZR::_8KB => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSRAMSZR {
        match value {
            0 => PSRAMSZR::_0,
            1 => PSRAMSZR::_128B,
            2 => PSRAMSZR::_256B,
            3 => PSRAMSZR::_512B,
            4 => PSRAMSZR::_1KB,
            5 => PSRAMSZR::_2KB,
            6 => PSRAMSZR::_4KB,
            7 => PSRAMSZR::_8KB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PSRAMSZR::_0
    }
    #[doc = "Checks if the value of the field is `_128B`"]
    #[inline]
    pub fn is_128b(&self) -> bool {
        *self == PSRAMSZR::_128B
    }
    #[doc = "Checks if the value of the field is `_256B`"]
    #[inline]
    pub fn is_256b(&self) -> bool {
        *self == PSRAMSZR::_256B
    }
    #[doc = "Checks if the value of the field is `_512B`"]
    #[inline]
    pub fn is_512b(&self) -> bool {
        *self == PSRAMSZR::_512B
    }
    #[doc = "Checks if the value of the field is `_1KB`"]
    #[inline]
    pub fn is_1kb(&self) -> bool {
        *self == PSRAMSZR::_1KB
    }
    #[doc = "Checks if the value of the field is `_2KB`"]
    #[inline]
    pub fn is_2kb(&self) -> bool {
        *self == PSRAMSZR::_2KB
    }
    #[doc = "Checks if the value of the field is `_4KB`"]
    #[inline]
    pub fn is_4kb(&self) -> bool {
        *self == PSRAMSZR::_4KB
    }
    #[doc = "Checks if the value of the field is `_8KB`"]
    #[inline]
    pub fn is_8kb(&self) -> bool {
        *self == PSRAMSZR::_8KB
    }
}
#[doc = r" Value of the field"]
pub struct IRDYDLYR {
    bits: u8,
}
impl IRDYDLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RDWSMW<'a> {
    w: &'a mut W,
}
impl<'a> _RDWSMW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WRWSMW<'a> {
    w: &'a mut W,
}
impl<'a> _WRWSMW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CAPWIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPWIDTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSRAMSZ`"]
pub enum PSRAMSZW {
    #[doc = "No row size limitation"]
    _0,
    #[doc = "128 B"]
    _128B,
    #[doc = "256 B"]
    _256B,
    #[doc = "512 B"]
    _512B,
    #[doc = "1024 B"]
    _1KB,
    #[doc = "2048 B"]
    _2KB,
    #[doc = "4096 B"]
    _4KB,
    #[doc = "8192 B"]
    _8KB,
}
impl PSRAMSZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSRAMSZW::_0 => 0,
            PSRAMSZW::_128B => 1,
            PSRAMSZW::_256B => 2,
            PSRAMSZW::_512B => 3,
            PSRAMSZW::_1KB => 4,
            PSRAMSZW::_2KB => 5,
            PSRAMSZW::_4KB => 6,
            PSRAMSZW::_8KB => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSRAMSZW<'a> {
    w: &'a mut W,
}
impl<'a> _PSRAMSZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSRAMSZW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No row size limitation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSRAMSZW::_0)
    }
    #[doc = "128 B"]
    #[inline]
    pub fn _128b(self) -> &'a mut W {
        self.variant(PSRAMSZW::_128B)
    }
    #[doc = "256 B"]
    #[inline]
    pub fn _256b(self) -> &'a mut W {
        self.variant(PSRAMSZW::_256B)
    }
    #[doc = "512 B"]
    #[inline]
    pub fn _512b(self) -> &'a mut W {
        self.variant(PSRAMSZW::_512B)
    }
    #[doc = "1024 B"]
    #[inline]
    pub fn _1kb(self) -> &'a mut W {
        self.variant(PSRAMSZW::_1KB)
    }
    #[doc = "2048 B"]
    #[inline]
    pub fn _2kb(self) -> &'a mut W {
        self.variant(PSRAMSZW::_2KB)
    }
    #[doc = "4096 B"]
    #[inline]
    pub fn _4kb(self) -> &'a mut W {
        self.variant(PSRAMSZW::_4KB)
    }
    #[doc = "8192 B"]
    #[inline]
    pub fn _8kb(self) -> &'a mut W {
        self.variant(PSRAMSZW::_8KB)
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
#[doc = r" Proxy"]
pub struct _IRDYDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _IRDYDLYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - CS2n Read Wait State Minus One"]
    #[inline]
    pub fn rdwsm(&self) -> RDWSMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RDWSMR { bits }
    }
    #[doc = "Bit 4 - CS2n Write Wait State Minus One"]
    #[inline]
    pub fn wrwsm(&self) -> WRWSMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRWSMR { bits }
    }
    #[doc = "Bits 12:13 - CS2n Inter-transfer Capture Width"]
    #[inline]
    pub fn capwidth(&self) -> CAPWIDTHR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CAPWIDTHR { bits }
    }
    #[doc = "Bits 16:18 - PSRAM Row Size"]
    #[inline]
    pub fn psramsz(&self) -> PSRAMSZR {
        PSRAMSZR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - CS2n Input Ready Delay"]
    #[inline]
    pub fn irdydly(&self) -> IRDYDLYR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IRDYDLYR { bits }
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
    #[doc = "Bit 0 - CS2n Read Wait State Minus One"]
    #[inline]
    pub fn rdwsm(&mut self) -> _RDWSMW {
        _RDWSMW { w: self }
    }
    #[doc = "Bit 4 - CS2n Write Wait State Minus One"]
    #[inline]
    pub fn wrwsm(&mut self) -> _WRWSMW {
        _WRWSMW { w: self }
    }
    #[doc = "Bits 12:13 - CS2n Inter-transfer Capture Width"]
    #[inline]
    pub fn capwidth(&mut self) -> _CAPWIDTHW {
        _CAPWIDTHW { w: self }
    }
    #[doc = "Bits 16:18 - PSRAM Row Size"]
    #[inline]
    pub fn psramsz(&mut self) -> _PSRAMSZW {
        _PSRAMSZW { w: self }
    }
    #[doc = "Bits 24:25 - CS2n Input Ready Delay"]
    #[inline]
    pub fn irdydly(&mut self) -> _IRDYDLYW {
        _IRDYDLYW { w: self }
    }
}
