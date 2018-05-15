#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ENUPD {
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
#[doc = "Possible values of the field `ENUPD0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENUPD0R {
    #[doc = "Immediate"]
    IMM,
    #[doc = "Locally Synchronized"]
    LSYNC,
    #[doc = "Globally Synchronized"]
    GSYNC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ENUPD0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENUPD0R::IMM => 0,
            ENUPD0R::LSYNC => 2,
            ENUPD0R::GSYNC => 3,
            ENUPD0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENUPD0R {
        match value {
            0 => ENUPD0R::IMM,
            2 => ENUPD0R::LSYNC,
            3 => ENUPD0R::GSYNC,
            i => ENUPD0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMM`"]
    #[inline]
    pub fn is_imm(&self) -> bool {
        *self == ENUPD0R::IMM
    }
    #[doc = "Checks if the value of the field is `LSYNC`"]
    #[inline]
    pub fn is_lsync(&self) -> bool {
        *self == ENUPD0R::LSYNC
    }
    #[doc = "Checks if the value of the field is `GSYNC`"]
    #[inline]
    pub fn is_gsync(&self) -> bool {
        *self == ENUPD0R::GSYNC
    }
}
#[doc = "Possible values of the field `ENUPD1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENUPD1R {
    #[doc = "Immediate"]
    IMM,
    #[doc = "Locally Synchronized"]
    LSYNC,
    #[doc = "Globally Synchronized"]
    GSYNC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ENUPD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENUPD1R::IMM => 0,
            ENUPD1R::LSYNC => 2,
            ENUPD1R::GSYNC => 3,
            ENUPD1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENUPD1R {
        match value {
            0 => ENUPD1R::IMM,
            2 => ENUPD1R::LSYNC,
            3 => ENUPD1R::GSYNC,
            i => ENUPD1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMM`"]
    #[inline]
    pub fn is_imm(&self) -> bool {
        *self == ENUPD1R::IMM
    }
    #[doc = "Checks if the value of the field is `LSYNC`"]
    #[inline]
    pub fn is_lsync(&self) -> bool {
        *self == ENUPD1R::LSYNC
    }
    #[doc = "Checks if the value of the field is `GSYNC`"]
    #[inline]
    pub fn is_gsync(&self) -> bool {
        *self == ENUPD1R::GSYNC
    }
}
#[doc = "Possible values of the field `ENUPD2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENUPD2R {
    #[doc = "Immediate"]
    IMM,
    #[doc = "Locally Synchronized"]
    LSYNC,
    #[doc = "Globally Synchronized"]
    GSYNC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ENUPD2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENUPD2R::IMM => 0,
            ENUPD2R::LSYNC => 2,
            ENUPD2R::GSYNC => 3,
            ENUPD2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENUPD2R {
        match value {
            0 => ENUPD2R::IMM,
            2 => ENUPD2R::LSYNC,
            3 => ENUPD2R::GSYNC,
            i => ENUPD2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMM`"]
    #[inline]
    pub fn is_imm(&self) -> bool {
        *self == ENUPD2R::IMM
    }
    #[doc = "Checks if the value of the field is `LSYNC`"]
    #[inline]
    pub fn is_lsync(&self) -> bool {
        *self == ENUPD2R::LSYNC
    }
    #[doc = "Checks if the value of the field is `GSYNC`"]
    #[inline]
    pub fn is_gsync(&self) -> bool {
        *self == ENUPD2R::GSYNC
    }
}
#[doc = "Possible values of the field `ENUPD3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENUPD3R {
    #[doc = "Immediate"]
    IMM,
    #[doc = "Locally Synchronized"]
    LSYNC,
    #[doc = "Globally Synchronized"]
    GSYNC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ENUPD3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENUPD3R::IMM => 0,
            ENUPD3R::LSYNC => 2,
            ENUPD3R::GSYNC => 3,
            ENUPD3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENUPD3R {
        match value {
            0 => ENUPD3R::IMM,
            2 => ENUPD3R::LSYNC,
            3 => ENUPD3R::GSYNC,
            i => ENUPD3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMM`"]
    #[inline]
    pub fn is_imm(&self) -> bool {
        *self == ENUPD3R::IMM
    }
    #[doc = "Checks if the value of the field is `LSYNC`"]
    #[inline]
    pub fn is_lsync(&self) -> bool {
        *self == ENUPD3R::LSYNC
    }
    #[doc = "Checks if the value of the field is `GSYNC`"]
    #[inline]
    pub fn is_gsync(&self) -> bool {
        *self == ENUPD3R::GSYNC
    }
}
#[doc = "Possible values of the field `ENUPD4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENUPD4R {
    #[doc = "Immediate"]
    IMM,
    #[doc = "Locally Synchronized"]
    LSYNC,
    #[doc = "Globally Synchronized"]
    GSYNC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ENUPD4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENUPD4R::IMM => 0,
            ENUPD4R::LSYNC => 2,
            ENUPD4R::GSYNC => 3,
            ENUPD4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENUPD4R {
        match value {
            0 => ENUPD4R::IMM,
            2 => ENUPD4R::LSYNC,
            3 => ENUPD4R::GSYNC,
            i => ENUPD4R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMM`"]
    #[inline]
    pub fn is_imm(&self) -> bool {
        *self == ENUPD4R::IMM
    }
    #[doc = "Checks if the value of the field is `LSYNC`"]
    #[inline]
    pub fn is_lsync(&self) -> bool {
        *self == ENUPD4R::LSYNC
    }
    #[doc = "Checks if the value of the field is `GSYNC`"]
    #[inline]
    pub fn is_gsync(&self) -> bool {
        *self == ENUPD4R::GSYNC
    }
}
#[doc = "Possible values of the field `ENUPD5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENUPD5R {
    #[doc = "Immediate"]
    IMM,
    #[doc = "Locally Synchronized"]
    LSYNC,
    #[doc = "Globally Synchronized"]
    GSYNC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ENUPD5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENUPD5R::IMM => 0,
            ENUPD5R::LSYNC => 2,
            ENUPD5R::GSYNC => 3,
            ENUPD5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENUPD5R {
        match value {
            0 => ENUPD5R::IMM,
            2 => ENUPD5R::LSYNC,
            3 => ENUPD5R::GSYNC,
            i => ENUPD5R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMM`"]
    #[inline]
    pub fn is_imm(&self) -> bool {
        *self == ENUPD5R::IMM
    }
    #[doc = "Checks if the value of the field is `LSYNC`"]
    #[inline]
    pub fn is_lsync(&self) -> bool {
        *self == ENUPD5R::LSYNC
    }
    #[doc = "Checks if the value of the field is `GSYNC`"]
    #[inline]
    pub fn is_gsync(&self) -> bool {
        *self == ENUPD5R::GSYNC
    }
}
#[doc = "Possible values of the field `ENUPD6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENUPD6R {
    #[doc = "Immediate"]
    IMM,
    #[doc = "Locally Synchronized"]
    LSYNC,
    #[doc = "Globally Synchronized"]
    GSYNC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ENUPD6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENUPD6R::IMM => 0,
            ENUPD6R::LSYNC => 2,
            ENUPD6R::GSYNC => 3,
            ENUPD6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENUPD6R {
        match value {
            0 => ENUPD6R::IMM,
            2 => ENUPD6R::LSYNC,
            3 => ENUPD6R::GSYNC,
            i => ENUPD6R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMM`"]
    #[inline]
    pub fn is_imm(&self) -> bool {
        *self == ENUPD6R::IMM
    }
    #[doc = "Checks if the value of the field is `LSYNC`"]
    #[inline]
    pub fn is_lsync(&self) -> bool {
        *self == ENUPD6R::LSYNC
    }
    #[doc = "Checks if the value of the field is `GSYNC`"]
    #[inline]
    pub fn is_gsync(&self) -> bool {
        *self == ENUPD6R::GSYNC
    }
}
#[doc = "Possible values of the field `ENUPD7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENUPD7R {
    #[doc = "Immediate"]
    IMM,
    #[doc = "Locally Synchronized"]
    LSYNC,
    #[doc = "Globally Synchronized"]
    GSYNC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ENUPD7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENUPD7R::IMM => 0,
            ENUPD7R::LSYNC => 2,
            ENUPD7R::GSYNC => 3,
            ENUPD7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENUPD7R {
        match value {
            0 => ENUPD7R::IMM,
            2 => ENUPD7R::LSYNC,
            3 => ENUPD7R::GSYNC,
            i => ENUPD7R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMM`"]
    #[inline]
    pub fn is_imm(&self) -> bool {
        *self == ENUPD7R::IMM
    }
    #[doc = "Checks if the value of the field is `LSYNC`"]
    #[inline]
    pub fn is_lsync(&self) -> bool {
        *self == ENUPD7R::LSYNC
    }
    #[doc = "Checks if the value of the field is `GSYNC`"]
    #[inline]
    pub fn is_gsync(&self) -> bool {
        *self == ENUPD7R::GSYNC
    }
}
#[doc = "Values that can be written to the field `ENUPD0`"]
pub enum ENUPD0W {
    #[doc = "Immediate"]
    IMM,
    #[doc = "Locally Synchronized"]
    LSYNC,
    #[doc = "Globally Synchronized"]
    GSYNC,
}
impl ENUPD0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENUPD0W::IMM => 0,
            ENUPD0W::LSYNC => 2,
            ENUPD0W::GSYNC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENUPD0W<'a> {
    w: &'a mut W,
}
impl<'a> _ENUPD0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENUPD0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline]
    pub fn imm(self) -> &'a mut W {
        self.variant(ENUPD0W::IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline]
    pub fn lsync(self) -> &'a mut W {
        self.variant(ENUPD0W::LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline]
    pub fn gsync(self) -> &'a mut W {
        self.variant(ENUPD0W::GSYNC)
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
#[doc = "Values that can be written to the field `ENUPD1`"]
pub enum ENUPD1W {
    #[doc = "Immediate"]
    IMM,
    #[doc = "Locally Synchronized"]
    LSYNC,
    #[doc = "Globally Synchronized"]
    GSYNC,
}
impl ENUPD1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENUPD1W::IMM => 0,
            ENUPD1W::LSYNC => 2,
            ENUPD1W::GSYNC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENUPD1W<'a> {
    w: &'a mut W,
}
impl<'a> _ENUPD1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENUPD1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline]
    pub fn imm(self) -> &'a mut W {
        self.variant(ENUPD1W::IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline]
    pub fn lsync(self) -> &'a mut W {
        self.variant(ENUPD1W::LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline]
    pub fn gsync(self) -> &'a mut W {
        self.variant(ENUPD1W::GSYNC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENUPD2`"]
pub enum ENUPD2W {
    #[doc = "Immediate"]
    IMM,
    #[doc = "Locally Synchronized"]
    LSYNC,
    #[doc = "Globally Synchronized"]
    GSYNC,
}
impl ENUPD2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENUPD2W::IMM => 0,
            ENUPD2W::LSYNC => 2,
            ENUPD2W::GSYNC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENUPD2W<'a> {
    w: &'a mut W,
}
impl<'a> _ENUPD2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENUPD2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline]
    pub fn imm(self) -> &'a mut W {
        self.variant(ENUPD2W::IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline]
    pub fn lsync(self) -> &'a mut W {
        self.variant(ENUPD2W::LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline]
    pub fn gsync(self) -> &'a mut W {
        self.variant(ENUPD2W::GSYNC)
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
#[doc = "Values that can be written to the field `ENUPD3`"]
pub enum ENUPD3W {
    #[doc = "Immediate"]
    IMM,
    #[doc = "Locally Synchronized"]
    LSYNC,
    #[doc = "Globally Synchronized"]
    GSYNC,
}
impl ENUPD3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENUPD3W::IMM => 0,
            ENUPD3W::LSYNC => 2,
            ENUPD3W::GSYNC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENUPD3W<'a> {
    w: &'a mut W,
}
impl<'a> _ENUPD3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENUPD3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline]
    pub fn imm(self) -> &'a mut W {
        self.variant(ENUPD3W::IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline]
    pub fn lsync(self) -> &'a mut W {
        self.variant(ENUPD3W::LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline]
    pub fn gsync(self) -> &'a mut W {
        self.variant(ENUPD3W::GSYNC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENUPD4`"]
pub enum ENUPD4W {
    #[doc = "Immediate"]
    IMM,
    #[doc = "Locally Synchronized"]
    LSYNC,
    #[doc = "Globally Synchronized"]
    GSYNC,
}
impl ENUPD4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENUPD4W::IMM => 0,
            ENUPD4W::LSYNC => 2,
            ENUPD4W::GSYNC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENUPD4W<'a> {
    w: &'a mut W,
}
impl<'a> _ENUPD4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENUPD4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline]
    pub fn imm(self) -> &'a mut W {
        self.variant(ENUPD4W::IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline]
    pub fn lsync(self) -> &'a mut W {
        self.variant(ENUPD4W::LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline]
    pub fn gsync(self) -> &'a mut W {
        self.variant(ENUPD4W::GSYNC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENUPD5`"]
pub enum ENUPD5W {
    #[doc = "Immediate"]
    IMM,
    #[doc = "Locally Synchronized"]
    LSYNC,
    #[doc = "Globally Synchronized"]
    GSYNC,
}
impl ENUPD5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENUPD5W::IMM => 0,
            ENUPD5W::LSYNC => 2,
            ENUPD5W::GSYNC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENUPD5W<'a> {
    w: &'a mut W,
}
impl<'a> _ENUPD5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENUPD5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline]
    pub fn imm(self) -> &'a mut W {
        self.variant(ENUPD5W::IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline]
    pub fn lsync(self) -> &'a mut W {
        self.variant(ENUPD5W::LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline]
    pub fn gsync(self) -> &'a mut W {
        self.variant(ENUPD5W::GSYNC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENUPD6`"]
pub enum ENUPD6W {
    #[doc = "Immediate"]
    IMM,
    #[doc = "Locally Synchronized"]
    LSYNC,
    #[doc = "Globally Synchronized"]
    GSYNC,
}
impl ENUPD6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENUPD6W::IMM => 0,
            ENUPD6W::LSYNC => 2,
            ENUPD6W::GSYNC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENUPD6W<'a> {
    w: &'a mut W,
}
impl<'a> _ENUPD6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENUPD6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline]
    pub fn imm(self) -> &'a mut W {
        self.variant(ENUPD6W::IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline]
    pub fn lsync(self) -> &'a mut W {
        self.variant(ENUPD6W::LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline]
    pub fn gsync(self) -> &'a mut W {
        self.variant(ENUPD6W::GSYNC)
    }
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
#[doc = "Values that can be written to the field `ENUPD7`"]
pub enum ENUPD7W {
    #[doc = "Immediate"]
    IMM,
    #[doc = "Locally Synchronized"]
    LSYNC,
    #[doc = "Globally Synchronized"]
    GSYNC,
}
impl ENUPD7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENUPD7W::IMM => 0,
            ENUPD7W::LSYNC => 2,
            ENUPD7W::GSYNC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENUPD7W<'a> {
    w: &'a mut W,
}
impl<'a> _ENUPD7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENUPD7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline]
    pub fn imm(self) -> &'a mut W {
        self.variant(ENUPD7W::IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline]
    pub fn lsync(self) -> &'a mut W {
        self.variant(ENUPD7W::LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline]
    pub fn gsync(self) -> &'a mut W {
        self.variant(ENUPD7W::GSYNC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:1 - MnPWM0 Enable Update Mode"]
    #[inline]
    pub fn enupd0(&self) -> ENUPD0R {
        ENUPD0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - MnPWM1 Enable Update Mode"]
    #[inline]
    pub fn enupd1(&self) -> ENUPD1R {
        ENUPD1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - MnPWM2 Enable Update Mode"]
    #[inline]
    pub fn enupd2(&self) -> ENUPD2R {
        ENUPD2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - MnPWM3 Enable Update Mode"]
    #[inline]
    pub fn enupd3(&self) -> ENUPD3R {
        ENUPD3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - MnPWM4 Enable Update Mode"]
    #[inline]
    pub fn enupd4(&self) -> ENUPD4R {
        ENUPD4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - MnPWM5 Enable Update Mode"]
    #[inline]
    pub fn enupd5(&self) -> ENUPD5R {
        ENUPD5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - MnPWM6 Enable Update Mode"]
    #[inline]
    pub fn enupd6(&self) -> ENUPD6R {
        ENUPD6R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - MnPWM7 Enable Update Mode"]
    #[inline]
    pub fn enupd7(&self) -> ENUPD7R {
        ENUPD7R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:1 - MnPWM0 Enable Update Mode"]
    #[inline]
    pub fn enupd0(&mut self) -> _ENUPD0W {
        _ENUPD0W { w: self }
    }
    #[doc = "Bits 2:3 - MnPWM1 Enable Update Mode"]
    #[inline]
    pub fn enupd1(&mut self) -> _ENUPD1W {
        _ENUPD1W { w: self }
    }
    #[doc = "Bits 4:5 - MnPWM2 Enable Update Mode"]
    #[inline]
    pub fn enupd2(&mut self) -> _ENUPD2W {
        _ENUPD2W { w: self }
    }
    #[doc = "Bits 6:7 - MnPWM3 Enable Update Mode"]
    #[inline]
    pub fn enupd3(&mut self) -> _ENUPD3W {
        _ENUPD3W { w: self }
    }
    #[doc = "Bits 8:9 - MnPWM4 Enable Update Mode"]
    #[inline]
    pub fn enupd4(&mut self) -> _ENUPD4W {
        _ENUPD4W { w: self }
    }
    #[doc = "Bits 10:11 - MnPWM5 Enable Update Mode"]
    #[inline]
    pub fn enupd5(&mut self) -> _ENUPD5W {
        _ENUPD5W { w: self }
    }
    #[doc = "Bits 12:13 - MnPWM6 Enable Update Mode"]
    #[inline]
    pub fn enupd6(&mut self) -> _ENUPD6W {
        _ENUPD6W { w: self }
    }
    #[doc = "Bits 14:15 - MnPWM7 Enable Update Mode"]
    #[inline]
    pub fn enupd7(&mut self) -> _ENUPD7W {
        _ENUPD7W { w: self }
    }
}
