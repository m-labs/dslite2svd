#[doc = "Reader of register ADDRMAP"]
pub type R = crate::R<u32, super::ADDRMAP>;
#[doc = "Writer for register ADDRMAP"]
pub type W = crate::W<u32, super::ADDRMAP>;
#[doc = "Register ADDRMAP `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDRMAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "External RAM Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERADR_A {
    #[doc = "0: Not mapped"]
    NONE,
    #[doc = "1: At 0x6000.0000"]
    _6000,
    #[doc = "2: At 0x8000.0000"]
    _8000,
    #[doc = "3: Only to be used with Host Bus quad chip select. In quad chip select mode, CS0n maps to 0x6000.0000 and CS1n maps to 0x8000.0000"]
    HBQS,
}
impl From<ERADR_A> for u8 {
    #[inline(always)]
    fn from(variant: ERADR_A) -> Self {
        match variant {
            ERADR_A::NONE => 0,
            ERADR_A::_6000 => 1,
            ERADR_A::_8000 => 2,
            ERADR_A::HBQS => 3,
        }
    }
}
#[doc = "Reader of field `ERADR`"]
pub type ERADR_R = crate::R<u8, ERADR_A>;
impl ERADR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERADR_A {
        match self.bits {
            0 => ERADR_A::NONE,
            1 => ERADR_A::_6000,
            2 => ERADR_A::_8000,
            3 => ERADR_A::HBQS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ERADR_A::NONE
    }
    #[doc = "Checks if the value of the field is `_6000`"]
    #[inline(always)]
    pub fn is_6000(&self) -> bool {
        *self == ERADR_A::_6000
    }
    #[doc = "Checks if the value of the field is `_8000`"]
    #[inline(always)]
    pub fn is_8000(&self) -> bool {
        *self == ERADR_A::_8000
    }
    #[doc = "Checks if the value of the field is `HBQS`"]
    #[inline(always)]
    pub fn is_hbqs(&self) -> bool {
        *self == ERADR_A::HBQS
    }
}
#[doc = "Write proxy for field `ERADR`"]
pub struct ERADR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERADR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERADR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Not mapped"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ERADR_A::NONE)
    }
    #[doc = "At 0x6000.0000"]
    #[inline(always)]
    pub fn _6000(self) -> &'a mut W {
        self.variant(ERADR_A::_6000)
    }
    #[doc = "At 0x8000.0000"]
    #[inline(always)]
    pub fn _8000(self) -> &'a mut W {
        self.variant(ERADR_A::_8000)
    }
    #[doc = "Only to be used with Host Bus quad chip select. In quad chip select mode, CS0n maps to 0x6000.0000 and CS1n maps to 0x8000.0000"]
    #[inline(always)]
    pub fn hbqs(self) -> &'a mut W {
        self.variant(ERADR_A::HBQS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "External RAM Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERSZ_A {
    #[doc = "0: 256 bytes; lower address range: 0x00 to 0xFF"]
    _256B,
    #[doc = "1: 64 KB; lower address range: 0x0000 to 0xFFFF"]
    _64KB,
    #[doc = "2: 16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    _16MB,
    #[doc = "3: 256 MB; lower address range: 0x000.0000 to 0xFFF.FFFF"]
    _256MB,
}
impl From<ERSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: ERSZ_A) -> Self {
        match variant {
            ERSZ_A::_256B => 0,
            ERSZ_A::_64KB => 1,
            ERSZ_A::_16MB => 2,
            ERSZ_A::_256MB => 3,
        }
    }
}
#[doc = "Reader of field `ERSZ`"]
pub type ERSZ_R = crate::R<u8, ERSZ_A>;
impl ERSZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERSZ_A {
        match self.bits {
            0 => ERSZ_A::_256B,
            1 => ERSZ_A::_64KB,
            2 => ERSZ_A::_16MB,
            3 => ERSZ_A::_256MB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_256B`"]
    #[inline(always)]
    pub fn is_256b(&self) -> bool {
        *self == ERSZ_A::_256B
    }
    #[doc = "Checks if the value of the field is `_64KB`"]
    #[inline(always)]
    pub fn is_64kb(&self) -> bool {
        *self == ERSZ_A::_64KB
    }
    #[doc = "Checks if the value of the field is `_16MB`"]
    #[inline(always)]
    pub fn is_16mb(&self) -> bool {
        *self == ERSZ_A::_16MB
    }
    #[doc = "Checks if the value of the field is `_256MB`"]
    #[inline(always)]
    pub fn is_256mb(&self) -> bool {
        *self == ERSZ_A::_256MB
    }
}
#[doc = "Write proxy for field `ERSZ`"]
pub struct ERSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> ERSZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERSZ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    #[inline(always)]
    pub fn _256b(self) -> &'a mut W {
        self.variant(ERSZ_A::_256B)
    }
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    #[inline(always)]
    pub fn _64kb(self) -> &'a mut W {
        self.variant(ERSZ_A::_64KB)
    }
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    #[inline(always)]
    pub fn _16mb(self) -> &'a mut W {
        self.variant(ERSZ_A::_16MB)
    }
    #[doc = "256 MB; lower address range: 0x000.0000 to 0xFFF.FFFF"]
    #[inline(always)]
    pub fn _256mb(self) -> &'a mut W {
        self.variant(ERSZ_A::_256MB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "External Peripheral Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPADR_A {
    #[doc = "0: Not mapped"]
    NONE,
    #[doc = "1: At 0xA000.0000"]
    A000,
    #[doc = "2: At 0xC000.0000"]
    C000,
    #[doc = "3: Only to be used with Host Bus quad chip select. In quad chip select mode, CS2n maps to 0xA000.0000 and CS3n maps to 0xC000.0000"]
    HBQS,
}
impl From<EPADR_A> for u8 {
    #[inline(always)]
    fn from(variant: EPADR_A) -> Self {
        match variant {
            EPADR_A::NONE => 0,
            EPADR_A::A000 => 1,
            EPADR_A::C000 => 2,
            EPADR_A::HBQS => 3,
        }
    }
}
#[doc = "Reader of field `EPADR`"]
pub type EPADR_R = crate::R<u8, EPADR_A>;
impl EPADR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPADR_A {
        match self.bits {
            0 => EPADR_A::NONE,
            1 => EPADR_A::A000,
            2 => EPADR_A::C000,
            3 => EPADR_A::HBQS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == EPADR_A::NONE
    }
    #[doc = "Checks if the value of the field is `A000`"]
    #[inline(always)]
    pub fn is_a000(&self) -> bool {
        *self == EPADR_A::A000
    }
    #[doc = "Checks if the value of the field is `C000`"]
    #[inline(always)]
    pub fn is_c000(&self) -> bool {
        *self == EPADR_A::C000
    }
    #[doc = "Checks if the value of the field is `HBQS`"]
    #[inline(always)]
    pub fn is_hbqs(&self) -> bool {
        *self == EPADR_A::HBQS
    }
}
#[doc = "Write proxy for field `EPADR`"]
pub struct EPADR_W<'a> {
    w: &'a mut W,
}
impl<'a> EPADR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPADR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Not mapped"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(EPADR_A::NONE)
    }
    #[doc = "At 0xA000.0000"]
    #[inline(always)]
    pub fn a000(self) -> &'a mut W {
        self.variant(EPADR_A::A000)
    }
    #[doc = "At 0xC000.0000"]
    #[inline(always)]
    pub fn c000(self) -> &'a mut W {
        self.variant(EPADR_A::C000)
    }
    #[doc = "Only to be used with Host Bus quad chip select. In quad chip select mode, CS2n maps to 0xA000.0000 and CS3n maps to 0xC000.0000"]
    #[inline(always)]
    pub fn hbqs(self) -> &'a mut W {
        self.variant(EPADR_A::HBQS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "External Peripheral Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPSZ_A {
    #[doc = "0: 256 bytes; lower address range: 0x00 to 0xFF"]
    _256B,
    #[doc = "1: 64 KB; lower address range: 0x0000 to 0xFFFF"]
    _64KB,
    #[doc = "2: 16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    _16MB,
    #[doc = "3: 256 MB; lower address range: 0x000.0000 to 0xFFF.FFFF"]
    _256MB,
}
impl From<EPSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: EPSZ_A) -> Self {
        match variant {
            EPSZ_A::_256B => 0,
            EPSZ_A::_64KB => 1,
            EPSZ_A::_16MB => 2,
            EPSZ_A::_256MB => 3,
        }
    }
}
#[doc = "Reader of field `EPSZ`"]
pub type EPSZ_R = crate::R<u8, EPSZ_A>;
impl EPSZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPSZ_A {
        match self.bits {
            0 => EPSZ_A::_256B,
            1 => EPSZ_A::_64KB,
            2 => EPSZ_A::_16MB,
            3 => EPSZ_A::_256MB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_256B`"]
    #[inline(always)]
    pub fn is_256b(&self) -> bool {
        *self == EPSZ_A::_256B
    }
    #[doc = "Checks if the value of the field is `_64KB`"]
    #[inline(always)]
    pub fn is_64kb(&self) -> bool {
        *self == EPSZ_A::_64KB
    }
    #[doc = "Checks if the value of the field is `_16MB`"]
    #[inline(always)]
    pub fn is_16mb(&self) -> bool {
        *self == EPSZ_A::_16MB
    }
    #[doc = "Checks if the value of the field is `_256MB`"]
    #[inline(always)]
    pub fn is_256mb(&self) -> bool {
        *self == EPSZ_A::_256MB
    }
}
#[doc = "Write proxy for field `EPSZ`"]
pub struct EPSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPSZ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    #[inline(always)]
    pub fn _256b(self) -> &'a mut W {
        self.variant(EPSZ_A::_256B)
    }
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    #[inline(always)]
    pub fn _64kb(self) -> &'a mut W {
        self.variant(EPSZ_A::_64KB)
    }
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    #[inline(always)]
    pub fn _16mb(self) -> &'a mut W {
        self.variant(EPSZ_A::_16MB)
    }
    #[doc = "256 MB; lower address range: 0x000.0000 to 0xFFF.FFFF"]
    #[inline(always)]
    pub fn _256mb(self) -> &'a mut W {
        self.variant(EPSZ_A::_256MB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "External Code Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECADR_A {
    #[doc = "0: Not mapped"]
    NONE,
    #[doc = "1: At 0x1000.0000"]
    _1000,
}
impl From<ECADR_A> for u8 {
    #[inline(always)]
    fn from(variant: ECADR_A) -> Self {
        match variant {
            ECADR_A::NONE => 0,
            ECADR_A::_1000 => 1,
        }
    }
}
#[doc = "Reader of field `ECADR`"]
pub type ECADR_R = crate::R<u8, ECADR_A>;
impl ECADR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ECADR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ECADR_A::NONE),
            1 => Val(ECADR_A::_1000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ECADR_A::NONE
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == ECADR_A::_1000
    }
}
#[doc = "Write proxy for field `ECADR`"]
pub struct ECADR_W<'a> {
    w: &'a mut W,
}
impl<'a> ECADR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECADR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Not mapped"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ECADR_A::NONE)
    }
    #[doc = "At 0x1000.0000"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(ECADR_A::_1000)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "External Code Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECSZ_A {
    #[doc = "0: 256 bytes; lower address range: 0x00 to 0xFF"]
    _256B,
    #[doc = "1: 64 KB; lower address range: 0x0000 to 0xFFFF"]
    _64KB,
    #[doc = "2: 16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    _16MB,
    #[doc = "3: 256MB; lower address range: 0x000.0000 to 0x0FFF.FFFF"]
    _256MB,
}
impl From<ECSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: ECSZ_A) -> Self {
        match variant {
            ECSZ_A::_256B => 0,
            ECSZ_A::_64KB => 1,
            ECSZ_A::_16MB => 2,
            ECSZ_A::_256MB => 3,
        }
    }
}
#[doc = "Reader of field `ECSZ`"]
pub type ECSZ_R = crate::R<u8, ECSZ_A>;
impl ECSZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECSZ_A {
        match self.bits {
            0 => ECSZ_A::_256B,
            1 => ECSZ_A::_64KB,
            2 => ECSZ_A::_16MB,
            3 => ECSZ_A::_256MB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_256B`"]
    #[inline(always)]
    pub fn is_256b(&self) -> bool {
        *self == ECSZ_A::_256B
    }
    #[doc = "Checks if the value of the field is `_64KB`"]
    #[inline(always)]
    pub fn is_64kb(&self) -> bool {
        *self == ECSZ_A::_64KB
    }
    #[doc = "Checks if the value of the field is `_16MB`"]
    #[inline(always)]
    pub fn is_16mb(&self) -> bool {
        *self == ECSZ_A::_16MB
    }
    #[doc = "Checks if the value of the field is `_256MB`"]
    #[inline(always)]
    pub fn is_256mb(&self) -> bool {
        *self == ECSZ_A::_256MB
    }
}
#[doc = "Write proxy for field `ECSZ`"]
pub struct ECSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> ECSZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECSZ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    #[inline(always)]
    pub fn _256b(self) -> &'a mut W {
        self.variant(ECSZ_A::_256B)
    }
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    #[inline(always)]
    pub fn _64kb(self) -> &'a mut W {
        self.variant(ECSZ_A::_64KB)
    }
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    #[inline(always)]
    pub fn _16mb(self) -> &'a mut W {
        self.variant(ECSZ_A::_16MB)
    }
    #[doc = "256MB; lower address range: 0x000.0000 to 0x0FFF.FFFF"]
    #[inline(always)]
    pub fn _256mb(self) -> &'a mut W {
        self.variant(ECSZ_A::_256MB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - External RAM Address"]
    #[inline(always)]
    pub fn eradr(&self) -> ERADR_R {
        ERADR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - External RAM Size"]
    #[inline(always)]
    pub fn ersz(&self) -> ERSZ_R {
        ERSZ_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - External Peripheral Address"]
    #[inline(always)]
    pub fn epadr(&self) -> EPADR_R {
        EPADR_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - External Peripheral Size"]
    #[inline(always)]
    pub fn epsz(&self) -> EPSZ_R {
        EPSZ_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - External Code Address"]
    #[inline(always)]
    pub fn ecadr(&self) -> ECADR_R {
        ECADR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - External Code Size"]
    #[inline(always)]
    pub fn ecsz(&self) -> ECSZ_R {
        ECSZ_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External RAM Address"]
    #[inline(always)]
    pub fn eradr(&mut self) -> ERADR_W {
        ERADR_W { w: self }
    }
    #[doc = "Bits 2:3 - External RAM Size"]
    #[inline(always)]
    pub fn ersz(&mut self) -> ERSZ_W {
        ERSZ_W { w: self }
    }
    #[doc = "Bits 4:5 - External Peripheral Address"]
    #[inline(always)]
    pub fn epadr(&mut self) -> EPADR_W {
        EPADR_W { w: self }
    }
    #[doc = "Bits 6:7 - External Peripheral Size"]
    #[inline(always)]
    pub fn epsz(&mut self) -> EPSZ_W {
        EPSZ_W { w: self }
    }
    #[doc = "Bits 8:9 - External Code Address"]
    #[inline(always)]
    pub fn ecadr(&mut self) -> ECADR_W {
        ECADR_W { w: self }
    }
    #[doc = "Bits 10:11 - External Code Size"]
    #[inline(always)]
    pub fn ecsz(&mut self) -> ECSZ_W {
        ECSZ_W { w: self }
    }
}
