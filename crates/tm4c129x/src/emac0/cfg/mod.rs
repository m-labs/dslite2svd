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
#[doc = "Possible values of the field `PRELEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRELENR {
    #[doc = "7 bytes of preamble"]
    _7,
    #[doc = "5 bytes of preamble"]
    _5,
    #[doc = "3 bytes of preamble"]
    _3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRELENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRELENR::_7 => 0,
            PRELENR::_5 => 1,
            PRELENR::_3 => 2,
            PRELENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRELENR {
        match value {
            0 => PRELENR::_7,
            1 => PRELENR::_5,
            2 => PRELENR::_3,
            i => PRELENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == PRELENR::_7
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == PRELENR::_5
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == PRELENR::_3
    }
}
#[doc = r" Value of the field"]
pub struct RER {
    bits: bool,
}
impl RER {
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
pub struct TER {
    bits: bool,
}
impl TER {
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
pub struct DCR {
    bits: bool,
}
impl DCR {
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
#[doc = "Possible values of the field `BL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLR {
    #[doc = "k = min (n,10)"]
    _1024,
    #[doc = "k = min (n,8)"]
    _256,
    #[doc = "k = min (n,4)"]
    _8,
    #[doc = "k = min (n,1)"]
    _2,
}
impl BLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BLR::_1024 => 0,
            BLR::_256 => 1,
            BLR::_8 => 2,
            BLR::_2 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BLR {
        match value {
            0 => BLR::_1024,
            1 => BLR::_256,
            2 => BLR::_8,
            3 => BLR::_2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline]
    pub fn is_1024(&self) -> bool {
        *self == BLR::_1024
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == BLR::_256
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == BLR::_8
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == BLR::_2
    }
}
#[doc = r" Value of the field"]
pub struct ACSR {
    bits: bool,
}
impl ACSR {
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
pub struct DRR {
    bits: bool,
}
impl DRR {
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
pub struct IPCR {
    bits: bool,
}
impl IPCR {
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
pub struct DUPMR {
    bits: bool,
}
impl DUPMR {
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
pub struct LOOPBMR {
    bits: bool,
}
impl LOOPBMR {
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
pub struct DROR {
    bits: bool,
}
impl DROR {
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
pub struct FESR {
    bits: bool,
}
impl FESR {
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
pub struct PSR {
    bits: bool,
}
impl PSR {
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
pub struct DISCRSR {
    bits: bool,
}
impl DISCRSR {
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
#[doc = "Possible values of the field `IFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFGR {
    #[doc = "96 bit times"]
    _96,
    #[doc = "88 bit times"]
    _88,
    #[doc = "80 bit times"]
    _80,
    #[doc = "72 bit times"]
    _72,
    #[doc = "64 bit times"]
    _64,
    #[doc = "56 bit times"]
    _56,
    #[doc = "48 bit times"]
    _48,
    #[doc = "40 bit times"]
    _40,
}
impl IFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IFGR::_96 => 0,
            IFGR::_88 => 1,
            IFGR::_80 => 2,
            IFGR::_72 => 3,
            IFGR::_64 => 4,
            IFGR::_56 => 5,
            IFGR::_48 => 6,
            IFGR::_40 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IFGR {
        match value {
            0 => IFGR::_96,
            1 => IFGR::_88,
            2 => IFGR::_80,
            3 => IFGR::_72,
            4 => IFGR::_64,
            5 => IFGR::_56,
            6 => IFGR::_48,
            7 => IFGR::_40,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_96`"]
    #[inline]
    pub fn is_96(&self) -> bool {
        *self == IFGR::_96
    }
    #[doc = "Checks if the value of the field is `_88`"]
    #[inline]
    pub fn is_88(&self) -> bool {
        *self == IFGR::_88
    }
    #[doc = "Checks if the value of the field is `_80`"]
    #[inline]
    pub fn is_80(&self) -> bool {
        *self == IFGR::_80
    }
    #[doc = "Checks if the value of the field is `_72`"]
    #[inline]
    pub fn is_72(&self) -> bool {
        *self == IFGR::_72
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == IFGR::_64
    }
    #[doc = "Checks if the value of the field is `_56`"]
    #[inline]
    pub fn is_56(&self) -> bool {
        *self == IFGR::_56
    }
    #[doc = "Checks if the value of the field is `_48`"]
    #[inline]
    pub fn is_48(&self) -> bool {
        *self == IFGR::_48
    }
    #[doc = "Checks if the value of the field is `_40`"]
    #[inline]
    pub fn is_40(&self) -> bool {
        *self == IFGR::_40
    }
}
#[doc = r" Value of the field"]
pub struct JFENR {
    bits: bool,
}
impl JFENR {
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
pub struct JDR {
    bits: bool,
}
impl JDR {
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
pub struct WDDISR {
    bits: bool,
}
impl WDDISR {
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
pub struct CSTR {
    bits: bool,
}
impl CSTR {
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
pub struct TWOKPENR {
    bits: bool,
}
impl TWOKPENR {
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
#[doc = "Values that can be written to the field `PRELEN`"]
pub enum PRELENW {
    #[doc = "7 bytes of preamble"]
    _7,
    #[doc = "5 bytes of preamble"]
    _5,
    #[doc = "3 bytes of preamble"]
    _3,
}
impl PRELENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRELENW::_7 => 0,
            PRELENW::_5 => 1,
            PRELENW::_3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRELENW<'a> {
    w: &'a mut W,
}
impl<'a> _PRELENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRELENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "7 bytes of preamble"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(PRELENW::_7)
    }
    #[doc = "5 bytes of preamble"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(PRELENW::_5)
    }
    #[doc = "3 bytes of preamble"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(PRELENW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _REW<'a> {
    w: &'a mut W,
}
impl<'a> _REW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCW<'a> {
    w: &'a mut W,
}
impl<'a> _DCW<'a> {
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
#[doc = "Values that can be written to the field `BL`"]
pub enum BLW {
    #[doc = "k = min (n,10)"]
    _1024,
    #[doc = "k = min (n,8)"]
    _256,
    #[doc = "k = min (n,4)"]
    _8,
    #[doc = "k = min (n,1)"]
    _2,
}
impl BLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BLW::_1024 => 0,
            BLW::_256 => 1,
            BLW::_8 => 2,
            BLW::_2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLW<'a> {
    w: &'a mut W,
}
impl<'a> _BLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "k = min (n,10)"]
    #[inline]
    pub fn _1024(self) -> &'a mut W {
        self.variant(BLW::_1024)
    }
    #[doc = "k = min (n,8)"]
    #[inline]
    pub fn _256(self) -> &'a mut W {
        self.variant(BLW::_256)
    }
    #[doc = "k = min (n,4)"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(BLW::_8)
    }
    #[doc = "k = min (n,1)"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(BLW::_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACSW<'a> {
    w: &'a mut W,
}
impl<'a> _ACSW<'a> {
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
pub struct _DRW<'a> {
    w: &'a mut W,
}
impl<'a> _DRW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IPCW<'a> {
    w: &'a mut W,
}
impl<'a> _IPCW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DUPMW<'a> {
    w: &'a mut W,
}
impl<'a> _DUPMW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOOPBMW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPBMW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DROW<'a> {
    w: &'a mut W,
}
impl<'a> _DROW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FESW<'a> {
    w: &'a mut W,
}
impl<'a> _FESW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PSW<'a> {
    w: &'a mut W,
}
impl<'a> _PSW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DISCRSW<'a> {
    w: &'a mut W,
}
impl<'a> _DISCRSW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IFG`"]
pub enum IFGW {
    #[doc = "96 bit times"]
    _96,
    #[doc = "88 bit times"]
    _88,
    #[doc = "80 bit times"]
    _80,
    #[doc = "72 bit times"]
    _72,
    #[doc = "64 bit times"]
    _64,
    #[doc = "56 bit times"]
    _56,
    #[doc = "48 bit times"]
    _48,
    #[doc = "40 bit times"]
    _40,
}
impl IFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IFGW::_96 => 0,
            IFGW::_88 => 1,
            IFGW::_80 => 2,
            IFGW::_72 => 3,
            IFGW::_64 => 4,
            IFGW::_56 => 5,
            IFGW::_48 => 6,
            IFGW::_40 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IFGW<'a> {
    w: &'a mut W,
}
impl<'a> _IFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "96 bit times"]
    #[inline]
    pub fn _96(self) -> &'a mut W {
        self.variant(IFGW::_96)
    }
    #[doc = "88 bit times"]
    #[inline]
    pub fn _88(self) -> &'a mut W {
        self.variant(IFGW::_88)
    }
    #[doc = "80 bit times"]
    #[inline]
    pub fn _80(self) -> &'a mut W {
        self.variant(IFGW::_80)
    }
    #[doc = "72 bit times"]
    #[inline]
    pub fn _72(self) -> &'a mut W {
        self.variant(IFGW::_72)
    }
    #[doc = "64 bit times"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(IFGW::_64)
    }
    #[doc = "56 bit times"]
    #[inline]
    pub fn _56(self) -> &'a mut W {
        self.variant(IFGW::_56)
    }
    #[doc = "48 bit times"]
    #[inline]
    pub fn _48(self) -> &'a mut W {
        self.variant(IFGW::_48)
    }
    #[doc = "40 bit times"]
    #[inline]
    pub fn _40(self) -> &'a mut W {
        self.variant(IFGW::_40)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _JFENW<'a> {
    w: &'a mut W,
}
impl<'a> _JFENW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _JDW<'a> {
    w: &'a mut W,
}
impl<'a> _JDW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WDDISW<'a> {
    w: &'a mut W,
}
impl<'a> _WDDISW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CSTW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TWOKPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TWOKPENW<'a> {
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
        const OFFSET: u8 = 27;
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
    #[doc = "Bits 0:1 - Preamble Length for Transmit Frames"]
    #[inline]
    pub fn prelen(&self) -> PRELENR {
        PRELENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline]
    pub fn re(&self) -> RER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RER { bits }
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline]
    pub fn te(&self) -> TER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TER { bits }
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline]
    pub fn dc(&self) -> DCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCR { bits }
    }
    #[doc = "Bits 5:6 - Back-Off Limit"]
    #[inline]
    pub fn bl(&self) -> BLR {
        BLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping"]
    #[inline]
    pub fn acs(&self) -> ACSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACSR { bits }
    }
    #[doc = "Bit 9 - Disable Retry"]
    #[inline]
    pub fn dr(&self) -> DRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DRR { bits }
    }
    #[doc = "Bit 10 - Checksum Offload"]
    #[inline]
    pub fn ipc(&self) -> IPCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IPCR { bits }
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline]
    pub fn dupm(&self) -> DUPMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DUPMR { bits }
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline]
    pub fn loopbm(&self) -> LOOPBMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOOPBMR { bits }
    }
    #[doc = "Bit 13 - Disable Receive Own"]
    #[inline]
    pub fn dro(&self) -> DROR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DROR { bits }
    }
    #[doc = "Bit 14 - Speed"]
    #[inline]
    pub fn fes(&self) -> FESR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FESR { bits }
    }
    #[doc = "Bit 15 - Port Select"]
    #[inline]
    pub fn ps(&self) -> PSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PSR { bits }
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission"]
    #[inline]
    pub fn discrs(&self) -> DISCRSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISCRSR { bits }
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap (IFG)"]
    #[inline]
    pub fn ifg(&self) -> IFGR {
        IFGR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Jumbo Frame Enable"]
    #[inline]
    pub fn jfen(&self) -> JFENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        JFENR { bits }
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline]
    pub fn jd(&self) -> JDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        JDR { bits }
    }
    #[doc = "Bit 23 - Watchdog Disable"]
    #[inline]
    pub fn wddis(&self) -> WDDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WDDISR { bits }
    }
    #[doc = "Bit 25 - CRC Stripping for Type Frames"]
    #[inline]
    pub fn cst(&self) -> CSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSTR { bits }
    }
    #[doc = "Bit 27 - IEEE 802"]
    #[inline]
    pub fn twokpen(&self) -> TWOKPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TWOKPENR { bits }
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
    #[doc = "Bits 0:1 - Preamble Length for Transmit Frames"]
    #[inline]
    pub fn prelen(&mut self) -> _PRELENW {
        _PRELENW { w: self }
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline]
    pub fn re(&mut self) -> _REW {
        _REW { w: self }
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline]
    pub fn te(&mut self) -> _TEW {
        _TEW { w: self }
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline]
    pub fn dc(&mut self) -> _DCW {
        _DCW { w: self }
    }
    #[doc = "Bits 5:6 - Back-Off Limit"]
    #[inline]
    pub fn bl(&mut self) -> _BLW {
        _BLW { w: self }
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping"]
    #[inline]
    pub fn acs(&mut self) -> _ACSW {
        _ACSW { w: self }
    }
    #[doc = "Bit 9 - Disable Retry"]
    #[inline]
    pub fn dr(&mut self) -> _DRW {
        _DRW { w: self }
    }
    #[doc = "Bit 10 - Checksum Offload"]
    #[inline]
    pub fn ipc(&mut self) -> _IPCW {
        _IPCW { w: self }
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline]
    pub fn dupm(&mut self) -> _DUPMW {
        _DUPMW { w: self }
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline]
    pub fn loopbm(&mut self) -> _LOOPBMW {
        _LOOPBMW { w: self }
    }
    #[doc = "Bit 13 - Disable Receive Own"]
    #[inline]
    pub fn dro(&mut self) -> _DROW {
        _DROW { w: self }
    }
    #[doc = "Bit 14 - Speed"]
    #[inline]
    pub fn fes(&mut self) -> _FESW {
        _FESW { w: self }
    }
    #[doc = "Bit 15 - Port Select"]
    #[inline]
    pub fn ps(&mut self) -> _PSW {
        _PSW { w: self }
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission"]
    #[inline]
    pub fn discrs(&mut self) -> _DISCRSW {
        _DISCRSW { w: self }
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap (IFG)"]
    #[inline]
    pub fn ifg(&mut self) -> _IFGW {
        _IFGW { w: self }
    }
    #[doc = "Bit 20 - Jumbo Frame Enable"]
    #[inline]
    pub fn jfen(&mut self) -> _JFENW {
        _JFENW { w: self }
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline]
    pub fn jd(&mut self) -> _JDW {
        _JDW { w: self }
    }
    #[doc = "Bit 23 - Watchdog Disable"]
    #[inline]
    pub fn wddis(&mut self) -> _WDDISW {
        _WDDISW { w: self }
    }
    #[doc = "Bit 25 - CRC Stripping for Type Frames"]
    #[inline]
    pub fn cst(&mut self) -> _CSTW {
        _CSTW { w: self }
    }
    #[doc = "Bit 27 - IEEE 802"]
    #[inline]
    pub fn twokpen(&mut self) -> _TWOKPENW {
        _TWOKPENW { w: self }
    }
}
