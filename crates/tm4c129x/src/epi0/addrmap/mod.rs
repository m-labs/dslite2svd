#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ADDRMAP {
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
#[doc = "Possible values of the field `ERADR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERADRR {
    #[doc = "Not mapped"]
    NONE,
    #[doc = "At 0x6000.0000"]
    _6000,
    #[doc = "At 0x8000.0000"]
    _8000,
    #[doc = "Only to be used with Host Bus quad chip select. In quad chip select mode, CS0n maps to 0x6000.0000 and CS1n maps to 0x8000.0000"]
    HBQS,
}
impl ERADRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ERADRR::NONE => 0,
            ERADRR::_6000 => 1,
            ERADRR::_8000 => 2,
            ERADRR::HBQS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ERADRR {
        match value {
            0 => ERADRR::NONE,
            1 => ERADRR::_6000,
            2 => ERADRR::_8000,
            3 => ERADRR::HBQS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ERADRR::NONE
    }
    #[doc = "Checks if the value of the field is `_6000`"]
    #[inline]
    pub fn is_6000(&self) -> bool {
        *self == ERADRR::_6000
    }
    #[doc = "Checks if the value of the field is `_8000`"]
    #[inline]
    pub fn is_8000(&self) -> bool {
        *self == ERADRR::_8000
    }
    #[doc = "Checks if the value of the field is `HBQS`"]
    #[inline]
    pub fn is_hbqs(&self) -> bool {
        *self == ERADRR::HBQS
    }
}
#[doc = "Possible values of the field `ERSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERSZR {
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    _256B,
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    _64KB,
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    _16MB,
    #[doc = "256 MB; lower address range: 0x000.0000 to 0xFFF.FFFF"]
    _256MB,
}
impl ERSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ERSZR::_256B => 0,
            ERSZR::_64KB => 1,
            ERSZR::_16MB => 2,
            ERSZR::_256MB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ERSZR {
        match value {
            0 => ERSZR::_256B,
            1 => ERSZR::_64KB,
            2 => ERSZR::_16MB,
            3 => ERSZR::_256MB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_256B`"]
    #[inline]
    pub fn is_256b(&self) -> bool {
        *self == ERSZR::_256B
    }
    #[doc = "Checks if the value of the field is `_64KB`"]
    #[inline]
    pub fn is_64kb(&self) -> bool {
        *self == ERSZR::_64KB
    }
    #[doc = "Checks if the value of the field is `_16MB`"]
    #[inline]
    pub fn is_16mb(&self) -> bool {
        *self == ERSZR::_16MB
    }
    #[doc = "Checks if the value of the field is `_256MB`"]
    #[inline]
    pub fn is_256mb(&self) -> bool {
        *self == ERSZR::_256MB
    }
}
#[doc = "Possible values of the field `EPADR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPADRR {
    #[doc = "Not mapped"]
    NONE,
    #[doc = "At 0xA000.0000"]
    A000,
    #[doc = "At 0xC000.0000"]
    C000,
    #[doc = "Only to be used with Host Bus quad chip select. In quad chip select mode, CS2n maps to 0xA000.0000 and CS3n maps to 0xC000.0000"]
    HBQS,
}
impl EPADRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EPADRR::NONE => 0,
            EPADRR::A000 => 1,
            EPADRR::C000 => 2,
            EPADRR::HBQS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EPADRR {
        match value {
            0 => EPADRR::NONE,
            1 => EPADRR::A000,
            2 => EPADRR::C000,
            3 => EPADRR::HBQS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == EPADRR::NONE
    }
    #[doc = "Checks if the value of the field is `A000`"]
    #[inline]
    pub fn is_a000(&self) -> bool {
        *self == EPADRR::A000
    }
    #[doc = "Checks if the value of the field is `C000`"]
    #[inline]
    pub fn is_c000(&self) -> bool {
        *self == EPADRR::C000
    }
    #[doc = "Checks if the value of the field is `HBQS`"]
    #[inline]
    pub fn is_hbqs(&self) -> bool {
        *self == EPADRR::HBQS
    }
}
#[doc = "Possible values of the field `EPSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPSZR {
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    _256B,
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    _64KB,
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    _16MB,
    #[doc = "256 MB; lower address range: 0x000.0000 to 0xFFF.FFFF"]
    _256MB,
}
impl EPSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EPSZR::_256B => 0,
            EPSZR::_64KB => 1,
            EPSZR::_16MB => 2,
            EPSZR::_256MB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EPSZR {
        match value {
            0 => EPSZR::_256B,
            1 => EPSZR::_64KB,
            2 => EPSZR::_16MB,
            3 => EPSZR::_256MB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_256B`"]
    #[inline]
    pub fn is_256b(&self) -> bool {
        *self == EPSZR::_256B
    }
    #[doc = "Checks if the value of the field is `_64KB`"]
    #[inline]
    pub fn is_64kb(&self) -> bool {
        *self == EPSZR::_64KB
    }
    #[doc = "Checks if the value of the field is `_16MB`"]
    #[inline]
    pub fn is_16mb(&self) -> bool {
        *self == EPSZR::_16MB
    }
    #[doc = "Checks if the value of the field is `_256MB`"]
    #[inline]
    pub fn is_256mb(&self) -> bool {
        *self == EPSZR::_256MB
    }
}
#[doc = "Possible values of the field `ECADR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECADRR {
    #[doc = "Not mapped"]
    NONE,
    #[doc = "At 0x1000.0000"]
    _1000,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ECADRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ECADRR::NONE => 0,
            ECADRR::_1000 => 1,
            ECADRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ECADRR {
        match value {
            0 => ECADRR::NONE,
            1 => ECADRR::_1000,
            i => ECADRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ECADRR::NONE
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == ECADRR::_1000
    }
}
#[doc = "Possible values of the field `ECSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECSZR {
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    _256B,
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    _64KB,
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    _16MB,
    #[doc = "256MB; lower address range: 0x000.0000 to 0x0FFF.FFFF"]
    _256MB,
}
impl ECSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ECSZR::_256B => 0,
            ECSZR::_64KB => 1,
            ECSZR::_16MB => 2,
            ECSZR::_256MB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ECSZR {
        match value {
            0 => ECSZR::_256B,
            1 => ECSZR::_64KB,
            2 => ECSZR::_16MB,
            3 => ECSZR::_256MB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_256B`"]
    #[inline]
    pub fn is_256b(&self) -> bool {
        *self == ECSZR::_256B
    }
    #[doc = "Checks if the value of the field is `_64KB`"]
    #[inline]
    pub fn is_64kb(&self) -> bool {
        *self == ECSZR::_64KB
    }
    #[doc = "Checks if the value of the field is `_16MB`"]
    #[inline]
    pub fn is_16mb(&self) -> bool {
        *self == ECSZR::_16MB
    }
    #[doc = "Checks if the value of the field is `_256MB`"]
    #[inline]
    pub fn is_256mb(&self) -> bool {
        *self == ECSZR::_256MB
    }
}
#[doc = "Values that can be written to the field `ERADR`"]
pub enum ERADRW {
    #[doc = "Not mapped"]
    NONE,
    #[doc = "At 0x6000.0000"]
    _6000,
    #[doc = "At 0x8000.0000"]
    _8000,
    #[doc = "Only to be used with Host Bus quad chip select. In quad chip select mode, CS0n maps to 0x6000.0000 and CS1n maps to 0x8000.0000"]
    HBQS,
}
impl ERADRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ERADRW::NONE => 0,
            ERADRW::_6000 => 1,
            ERADRW::_8000 => 2,
            ERADRW::HBQS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERADRW<'a> {
    w: &'a mut W,
}
impl<'a> _ERADRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERADRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Not mapped"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(ERADRW::NONE)
    }
    #[doc = "At 0x6000.0000"]
    #[inline]
    pub fn _6000(self) -> &'a mut W {
        self.variant(ERADRW::_6000)
    }
    #[doc = "At 0x8000.0000"]
    #[inline]
    pub fn _8000(self) -> &'a mut W {
        self.variant(ERADRW::_8000)
    }
    #[doc = "Only to be used with Host Bus quad chip select. In quad chip select mode, CS0n maps to 0x6000.0000 and CS1n maps to 0x8000.0000"]
    #[inline]
    pub fn hbqs(self) -> &'a mut W {
        self.variant(ERADRW::HBQS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERSZ`"]
pub enum ERSZW {
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    _256B,
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    _64KB,
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    _16MB,
    #[doc = "256 MB; lower address range: 0x000.0000 to 0xFFF.FFFF"]
    _256MB,
}
impl ERSZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ERSZW::_256B => 0,
            ERSZW::_64KB => 1,
            ERSZW::_16MB => 2,
            ERSZW::_256MB => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERSZW<'a> {
    w: &'a mut W,
}
impl<'a> _ERSZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERSZW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    #[inline]
    pub fn _256b(self) -> &'a mut W {
        self.variant(ERSZW::_256B)
    }
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    #[inline]
    pub fn _64kb(self) -> &'a mut W {
        self.variant(ERSZW::_64KB)
    }
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    #[inline]
    pub fn _16mb(self) -> &'a mut W {
        self.variant(ERSZW::_16MB)
    }
    #[doc = "256 MB; lower address range: 0x000.0000 to 0xFFF.FFFF"]
    #[inline]
    pub fn _256mb(self) -> &'a mut W {
        self.variant(ERSZW::_256MB)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EPADR`"]
pub enum EPADRW {
    #[doc = "Not mapped"]
    NONE,
    #[doc = "At 0xA000.0000"]
    A000,
    #[doc = "At 0xC000.0000"]
    C000,
    #[doc = "Only to be used with Host Bus quad chip select. In quad chip select mode, CS2n maps to 0xA000.0000 and CS3n maps to 0xC000.0000"]
    HBQS,
}
impl EPADRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPADRW::NONE => 0,
            EPADRW::A000 => 1,
            EPADRW::C000 => 2,
            EPADRW::HBQS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPADRW<'a> {
    w: &'a mut W,
}
impl<'a> _EPADRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPADRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Not mapped"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(EPADRW::NONE)
    }
    #[doc = "At 0xA000.0000"]
    #[inline]
    pub fn a000(self) -> &'a mut W {
        self.variant(EPADRW::A000)
    }
    #[doc = "At 0xC000.0000"]
    #[inline]
    pub fn c000(self) -> &'a mut W {
        self.variant(EPADRW::C000)
    }
    #[doc = "Only to be used with Host Bus quad chip select. In quad chip select mode, CS2n maps to 0xA000.0000 and CS3n maps to 0xC000.0000"]
    #[inline]
    pub fn hbqs(self) -> &'a mut W {
        self.variant(EPADRW::HBQS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EPSZ`"]
pub enum EPSZW {
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    _256B,
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    _64KB,
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    _16MB,
    #[doc = "256 MB; lower address range: 0x000.0000 to 0xFFF.FFFF"]
    _256MB,
}
impl EPSZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPSZW::_256B => 0,
            EPSZW::_64KB => 1,
            EPSZW::_16MB => 2,
            EPSZW::_256MB => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPSZW<'a> {
    w: &'a mut W,
}
impl<'a> _EPSZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPSZW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    #[inline]
    pub fn _256b(self) -> &'a mut W {
        self.variant(EPSZW::_256B)
    }
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    #[inline]
    pub fn _64kb(self) -> &'a mut W {
        self.variant(EPSZW::_64KB)
    }
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    #[inline]
    pub fn _16mb(self) -> &'a mut W {
        self.variant(EPSZW::_16MB)
    }
    #[doc = "256 MB; lower address range: 0x000.0000 to 0xFFF.FFFF"]
    #[inline]
    pub fn _256mb(self) -> &'a mut W {
        self.variant(EPSZW::_256MB)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ECADR`"]
pub enum ECADRW {
    #[doc = "Not mapped"]
    NONE,
    #[doc = "At 0x1000.0000"]
    _1000,
}
impl ECADRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ECADRW::NONE => 0,
            ECADRW::_1000 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECADRW<'a> {
    w: &'a mut W,
}
impl<'a> _ECADRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECADRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Not mapped"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(ECADRW::NONE)
    }
    #[doc = "At 0x1000.0000"]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(ECADRW::_1000)
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
#[doc = "Values that can be written to the field `ECSZ`"]
pub enum ECSZW {
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    _256B,
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    _64KB,
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    _16MB,
    #[doc = "256MB; lower address range: 0x000.0000 to 0x0FFF.FFFF"]
    _256MB,
}
impl ECSZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ECSZW::_256B => 0,
            ECSZW::_64KB => 1,
            ECSZW::_16MB => 2,
            ECSZW::_256MB => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECSZW<'a> {
    w: &'a mut W,
}
impl<'a> _ECSZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECSZW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    #[inline]
    pub fn _256b(self) -> &'a mut W {
        self.variant(ECSZW::_256B)
    }
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    #[inline]
    pub fn _64kb(self) -> &'a mut W {
        self.variant(ECSZW::_64KB)
    }
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    #[inline]
    pub fn _16mb(self) -> &'a mut W {
        self.variant(ECSZW::_16MB)
    }
    #[doc = "256MB; lower address range: 0x000.0000 to 0x0FFF.FFFF"]
    #[inline]
    pub fn _256mb(self) -> &'a mut W {
        self.variant(ECSZW::_256MB)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
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
    #[doc = "Bits 0:1 - External RAM Address"]
    #[inline]
    pub fn eradr(&self) -> ERADRR {
        ERADRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - External RAM Size"]
    #[inline]
    pub fn ersz(&self) -> ERSZR {
        ERSZR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - External Peripheral Address"]
    #[inline]
    pub fn epadr(&self) -> EPADRR {
        EPADRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - External Peripheral Size"]
    #[inline]
    pub fn epsz(&self) -> EPSZR {
        EPSZR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - External Code Address"]
    #[inline]
    pub fn ecadr(&self) -> ECADRR {
        ECADRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - External Code Size"]
    #[inline]
    pub fn ecsz(&self) -> ECSZR {
        ECSZR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
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
    #[doc = "Bits 0:1 - External RAM Address"]
    #[inline]
    pub fn eradr(&mut self) -> _ERADRW {
        _ERADRW { w: self }
    }
    #[doc = "Bits 2:3 - External RAM Size"]
    #[inline]
    pub fn ersz(&mut self) -> _ERSZW {
        _ERSZW { w: self }
    }
    #[doc = "Bits 4:5 - External Peripheral Address"]
    #[inline]
    pub fn epadr(&mut self) -> _EPADRW {
        _EPADRW { w: self }
    }
    #[doc = "Bits 6:7 - External Peripheral Size"]
    #[inline]
    pub fn epsz(&mut self) -> _EPSZW {
        _EPSZW { w: self }
    }
    #[doc = "Bits 8:9 - External Code Address"]
    #[inline]
    pub fn ecadr(&mut self) -> _ECADRW {
        _ECADRW { w: self }
    }
    #[doc = "Bits 10:11 - External Code Size"]
    #[inline]
    pub fn ecsz(&mut self) -> _ECSZW {
        _ECSZW { w: self }
    }
}
