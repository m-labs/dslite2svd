#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::_3_CTL {
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
pub struct ENABLER {
    bits: bool,
}
impl ENABLER {
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
pub struct MODER {
    bits: bool,
}
impl MODER {
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
pub struct DEBUGR {
    bits: bool,
}
impl DEBUGR {
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
pub struct LOADUPDR {
    bits: bool,
}
impl LOADUPDR {
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
pub struct CMPAUPDR {
    bits: bool,
}
impl CMPAUPDR {
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
pub struct CMPBUPDR {
    bits: bool,
}
impl CMPBUPDR {
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
#[doc = "Possible values of the field `GENAUPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENAUPDR {
    #[doc = "Immediate"]
    I,
    #[doc = "Locally Synchronized"]
    LS,
    #[doc = "Globally Synchronized"]
    GS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GENAUPDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GENAUPDR::I => 0,
            GENAUPDR::LS => 2,
            GENAUPDR::GS => 3,
            GENAUPDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GENAUPDR {
        match value {
            0 => GENAUPDR::I,
            2 => GENAUPDR::LS,
            3 => GENAUPDR::GS,
            i => GENAUPDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline]
    pub fn is_i(&self) -> bool {
        *self == GENAUPDR::I
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline]
    pub fn is_ls(&self) -> bool {
        *self == GENAUPDR::LS
    }
    #[doc = "Checks if the value of the field is `GS`"]
    #[inline]
    pub fn is_gs(&self) -> bool {
        *self == GENAUPDR::GS
    }
}
#[doc = "Possible values of the field `GENBUPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENBUPDR {
    #[doc = "Immediate"]
    I,
    #[doc = "Locally Synchronized"]
    LS,
    #[doc = "Globally Synchronized"]
    GS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GENBUPDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GENBUPDR::I => 0,
            GENBUPDR::LS => 2,
            GENBUPDR::GS => 3,
            GENBUPDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GENBUPDR {
        match value {
            0 => GENBUPDR::I,
            2 => GENBUPDR::LS,
            3 => GENBUPDR::GS,
            i => GENBUPDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline]
    pub fn is_i(&self) -> bool {
        *self == GENBUPDR::I
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline]
    pub fn is_ls(&self) -> bool {
        *self == GENBUPDR::LS
    }
    #[doc = "Checks if the value of the field is `GS`"]
    #[inline]
    pub fn is_gs(&self) -> bool {
        *self == GENBUPDR::GS
    }
}
#[doc = "Possible values of the field `DBCTLUPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBCTLUPDR {
    #[doc = "Immediate"]
    I,
    #[doc = "Locally Synchronized"]
    LS,
    #[doc = "Globally Synchronized"]
    GS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DBCTLUPDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DBCTLUPDR::I => 0,
            DBCTLUPDR::LS => 2,
            DBCTLUPDR::GS => 3,
            DBCTLUPDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DBCTLUPDR {
        match value {
            0 => DBCTLUPDR::I,
            2 => DBCTLUPDR::LS,
            3 => DBCTLUPDR::GS,
            i => DBCTLUPDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline]
    pub fn is_i(&self) -> bool {
        *self == DBCTLUPDR::I
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline]
    pub fn is_ls(&self) -> bool {
        *self == DBCTLUPDR::LS
    }
    #[doc = "Checks if the value of the field is `GS`"]
    #[inline]
    pub fn is_gs(&self) -> bool {
        *self == DBCTLUPDR::GS
    }
}
#[doc = "Possible values of the field `DBRISEUPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBRISEUPDR {
    #[doc = "Immediate"]
    I,
    #[doc = "Locally Synchronized"]
    LS,
    #[doc = "Globally Synchronized"]
    GS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DBRISEUPDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DBRISEUPDR::I => 0,
            DBRISEUPDR::LS => 2,
            DBRISEUPDR::GS => 3,
            DBRISEUPDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DBRISEUPDR {
        match value {
            0 => DBRISEUPDR::I,
            2 => DBRISEUPDR::LS,
            3 => DBRISEUPDR::GS,
            i => DBRISEUPDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline]
    pub fn is_i(&self) -> bool {
        *self == DBRISEUPDR::I
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline]
    pub fn is_ls(&self) -> bool {
        *self == DBRISEUPDR::LS
    }
    #[doc = "Checks if the value of the field is `GS`"]
    #[inline]
    pub fn is_gs(&self) -> bool {
        *self == DBRISEUPDR::GS
    }
}
#[doc = "Possible values of the field `DBFALLUPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBFALLUPDR {
    #[doc = "Immediate"]
    I,
    #[doc = "Locally Synchronized"]
    LS,
    #[doc = "Globally Synchronized"]
    GS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DBFALLUPDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DBFALLUPDR::I => 0,
            DBFALLUPDR::LS => 2,
            DBFALLUPDR::GS => 3,
            DBFALLUPDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DBFALLUPDR {
        match value {
            0 => DBFALLUPDR::I,
            2 => DBFALLUPDR::LS,
            3 => DBFALLUPDR::GS,
            i => DBFALLUPDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline]
    pub fn is_i(&self) -> bool {
        *self == DBFALLUPDR::I
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline]
    pub fn is_ls(&self) -> bool {
        *self == DBFALLUPDR::LS
    }
    #[doc = "Checks if the value of the field is `GS`"]
    #[inline]
    pub fn is_gs(&self) -> bool {
        *self == DBFALLUPDR::GS
    }
}
#[doc = r" Value of the field"]
pub struct FLTSRCR {
    bits: bool,
}
impl FLTSRCR {
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
pub struct MINFLTPERR {
    bits: bool,
}
impl MINFLTPERR {
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
pub struct LATCHR {
    bits: bool,
}
impl LATCHR {
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
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
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
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DEBUGW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBUGW<'a> {
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
pub struct _LOADUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _LOADUPDW<'a> {
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
pub struct _CMPAUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPAUPDW<'a> {
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
pub struct _CMPBUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPBUPDW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GENAUPD`"]
pub enum GENAUPDW {
    #[doc = "Immediate"]
    I,
    #[doc = "Locally Synchronized"]
    LS,
    #[doc = "Globally Synchronized"]
    GS,
}
impl GENAUPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GENAUPDW::I => 0,
            GENAUPDW::LS => 2,
            GENAUPDW::GS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GENAUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _GENAUPDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GENAUPDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline]
    pub fn i(self) -> &'a mut W {
        self.variant(GENAUPDW::I)
    }
    #[doc = "Locally Synchronized"]
    #[inline]
    pub fn ls(self) -> &'a mut W {
        self.variant(GENAUPDW::LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline]
    pub fn gs(self) -> &'a mut W {
        self.variant(GENAUPDW::GS)
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
#[doc = "Values that can be written to the field `GENBUPD`"]
pub enum GENBUPDW {
    #[doc = "Immediate"]
    I,
    #[doc = "Locally Synchronized"]
    LS,
    #[doc = "Globally Synchronized"]
    GS,
}
impl GENBUPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GENBUPDW::I => 0,
            GENBUPDW::LS => 2,
            GENBUPDW::GS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GENBUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _GENBUPDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GENBUPDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline]
    pub fn i(self) -> &'a mut W {
        self.variant(GENBUPDW::I)
    }
    #[doc = "Locally Synchronized"]
    #[inline]
    pub fn ls(self) -> &'a mut W {
        self.variant(GENBUPDW::LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline]
    pub fn gs(self) -> &'a mut W {
        self.variant(GENBUPDW::GS)
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
#[doc = "Values that can be written to the field `DBCTLUPD`"]
pub enum DBCTLUPDW {
    #[doc = "Immediate"]
    I,
    #[doc = "Locally Synchronized"]
    LS,
    #[doc = "Globally Synchronized"]
    GS,
}
impl DBCTLUPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DBCTLUPDW::I => 0,
            DBCTLUPDW::LS => 2,
            DBCTLUPDW::GS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBCTLUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _DBCTLUPDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBCTLUPDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline]
    pub fn i(self) -> &'a mut W {
        self.variant(DBCTLUPDW::I)
    }
    #[doc = "Locally Synchronized"]
    #[inline]
    pub fn ls(self) -> &'a mut W {
        self.variant(DBCTLUPDW::LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline]
    pub fn gs(self) -> &'a mut W {
        self.variant(DBCTLUPDW::GS)
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
#[doc = "Values that can be written to the field `DBRISEUPD`"]
pub enum DBRISEUPDW {
    #[doc = "Immediate"]
    I,
    #[doc = "Locally Synchronized"]
    LS,
    #[doc = "Globally Synchronized"]
    GS,
}
impl DBRISEUPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DBRISEUPDW::I => 0,
            DBRISEUPDW::LS => 2,
            DBRISEUPDW::GS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBRISEUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _DBRISEUPDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBRISEUPDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline]
    pub fn i(self) -> &'a mut W {
        self.variant(DBRISEUPDW::I)
    }
    #[doc = "Locally Synchronized"]
    #[inline]
    pub fn ls(self) -> &'a mut W {
        self.variant(DBRISEUPDW::LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline]
    pub fn gs(self) -> &'a mut W {
        self.variant(DBRISEUPDW::GS)
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
#[doc = "Values that can be written to the field `DBFALLUPD`"]
pub enum DBFALLUPDW {
    #[doc = "Immediate"]
    I,
    #[doc = "Locally Synchronized"]
    LS,
    #[doc = "Globally Synchronized"]
    GS,
}
impl DBFALLUPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DBFALLUPDW::I => 0,
            DBFALLUPDW::LS => 2,
            DBFALLUPDW::GS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBFALLUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _DBFALLUPDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBFALLUPDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline]
    pub fn i(self) -> &'a mut W {
        self.variant(DBFALLUPDW::I)
    }
    #[doc = "Locally Synchronized"]
    #[inline]
    pub fn ls(self) -> &'a mut W {
        self.variant(DBFALLUPDW::LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline]
    pub fn gs(self) -> &'a mut W {
        self.variant(DBFALLUPDW::GS)
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
#[doc = r" Proxy"]
pub struct _FLTSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FLTSRCW<'a> {
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
#[doc = r" Proxy"]
pub struct _MINFLTPERW<'a> {
    w: &'a mut W,
}
impl<'a> _MINFLTPERW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LATCHW<'a> {
    w: &'a mut W,
}
impl<'a> _LATCHW<'a> {
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
        const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - PWM Block Enable"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLER { bits }
    }
    #[doc = "Bit 1 - Counter Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MODER { bits }
    }
    #[doc = "Bit 2 - Debug Mode"]
    #[inline]
    pub fn debug(&self) -> DEBUGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEBUGR { bits }
    }
    #[doc = "Bit 3 - Load Register Update Mode"]
    #[inline]
    pub fn loadupd(&self) -> LOADUPDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOADUPDR { bits }
    }
    #[doc = "Bit 4 - Comparator A Update Mode"]
    #[inline]
    pub fn cmpaupd(&self) -> CMPAUPDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CMPAUPDR { bits }
    }
    #[doc = "Bit 5 - Comparator B Update Mode"]
    #[inline]
    pub fn cmpbupd(&self) -> CMPBUPDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CMPBUPDR { bits }
    }
    #[doc = "Bits 6:7 - PWMnGENA Update Mode"]
    #[inline]
    pub fn genaupd(&self) -> GENAUPDR {
        GENAUPDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - PWMnGENB Update Mode"]
    #[inline]
    pub fn genbupd(&self) -> GENBUPDR {
        GENBUPDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - PWMnDBCTL Update Mode"]
    #[inline]
    pub fn dbctlupd(&self) -> DBCTLUPDR {
        DBCTLUPDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - PWMnDBRISE Update Mode"]
    #[inline]
    pub fn dbriseupd(&self) -> DBRISEUPDR {
        DBRISEUPDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - PWMnDBFALL Update Mode"]
    #[inline]
    pub fn dbfallupd(&self) -> DBFALLUPDR {
        DBFALLUPDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Fault Condition Source"]
    #[inline]
    pub fn fltsrc(&self) -> FLTSRCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FLTSRCR { bits }
    }
    #[doc = "Bit 17 - Minimum Fault Period"]
    #[inline]
    pub fn minfltper(&self) -> MINFLTPERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MINFLTPERR { bits }
    }
    #[doc = "Bit 18 - Latch Fault Input"]
    #[inline]
    pub fn latch(&self) -> LATCHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LATCHR { bits }
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
    #[doc = "Bit 0 - PWM Block Enable"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 1 - Counter Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 2 - Debug Mode"]
    #[inline]
    pub fn debug(&mut self) -> _DEBUGW {
        _DEBUGW { w: self }
    }
    #[doc = "Bit 3 - Load Register Update Mode"]
    #[inline]
    pub fn loadupd(&mut self) -> _LOADUPDW {
        _LOADUPDW { w: self }
    }
    #[doc = "Bit 4 - Comparator A Update Mode"]
    #[inline]
    pub fn cmpaupd(&mut self) -> _CMPAUPDW {
        _CMPAUPDW { w: self }
    }
    #[doc = "Bit 5 - Comparator B Update Mode"]
    #[inline]
    pub fn cmpbupd(&mut self) -> _CMPBUPDW {
        _CMPBUPDW { w: self }
    }
    #[doc = "Bits 6:7 - PWMnGENA Update Mode"]
    #[inline]
    pub fn genaupd(&mut self) -> _GENAUPDW {
        _GENAUPDW { w: self }
    }
    #[doc = "Bits 8:9 - PWMnGENB Update Mode"]
    #[inline]
    pub fn genbupd(&mut self) -> _GENBUPDW {
        _GENBUPDW { w: self }
    }
    #[doc = "Bits 10:11 - PWMnDBCTL Update Mode"]
    #[inline]
    pub fn dbctlupd(&mut self) -> _DBCTLUPDW {
        _DBCTLUPDW { w: self }
    }
    #[doc = "Bits 12:13 - PWMnDBRISE Update Mode"]
    #[inline]
    pub fn dbriseupd(&mut self) -> _DBRISEUPDW {
        _DBRISEUPDW { w: self }
    }
    #[doc = "Bits 14:15 - PWMnDBFALL Update Mode"]
    #[inline]
    pub fn dbfallupd(&mut self) -> _DBFALLUPDW {
        _DBFALLUPDW { w: self }
    }
    #[doc = "Bit 16 - Fault Condition Source"]
    #[inline]
    pub fn fltsrc(&mut self) -> _FLTSRCW {
        _FLTSRCW { w: self }
    }
    #[doc = "Bit 17 - Minimum Fault Period"]
    #[inline]
    pub fn minfltper(&mut self) -> _MINFLTPERW {
        _MINFLTPERW { w: self }
    }
    #[doc = "Bit 18 - Latch Fault Input"]
    #[inline]
    pub fn latch(&mut self) -> _LATCHW {
        _LATCHW { w: self }
    }
}
