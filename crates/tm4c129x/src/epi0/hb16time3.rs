#[doc = "Reader of register HB16TIME3"]
pub type R = crate::R<u32, super::HB16TIME3>;
#[doc = "Writer for register HB16TIME3"]
pub type W = crate::W<u32, super::HB16TIME3>;
#[doc = "Register HB16TIME3 `reset()`'s with value 0"]
impl crate::ResetValue for super::HB16TIME3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDWSM`"]
pub type RDWSM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDWSM`"]
pub struct RDWSM_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWSM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `WRWSM`"]
pub type WRWSM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRWSM`"]
pub struct WRWSM_W<'a> {
    w: &'a mut W,
}
impl<'a> WRWSM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CAPWIDTH`"]
pub type CAPWIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAPWIDTH`"]
pub struct CAPWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPWIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "PSRAM Row Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSRAMSZ_A {
    #[doc = "0: No row size limitation"]
    _0,
    #[doc = "1: 128 B"]
    _128B,
    #[doc = "2: 256 B"]
    _256B,
    #[doc = "3: 512 B"]
    _512B,
    #[doc = "4: 1024 B"]
    _1KB,
    #[doc = "5: 2048 B"]
    _2KB,
    #[doc = "6: 4096 B"]
    _4KB,
    #[doc = "7: 8192 B"]
    _8KB,
}
impl From<PSRAMSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: PSRAMSZ_A) -> Self {
        match variant {
            PSRAMSZ_A::_0 => 0,
            PSRAMSZ_A::_128B => 1,
            PSRAMSZ_A::_256B => 2,
            PSRAMSZ_A::_512B => 3,
            PSRAMSZ_A::_1KB => 4,
            PSRAMSZ_A::_2KB => 5,
            PSRAMSZ_A::_4KB => 6,
            PSRAMSZ_A::_8KB => 7,
        }
    }
}
#[doc = "Reader of field `PSRAMSZ`"]
pub type PSRAMSZ_R = crate::R<u8, PSRAMSZ_A>;
impl PSRAMSZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSRAMSZ_A {
        match self.bits {
            0 => PSRAMSZ_A::_0,
            1 => PSRAMSZ_A::_128B,
            2 => PSRAMSZ_A::_256B,
            3 => PSRAMSZ_A::_512B,
            4 => PSRAMSZ_A::_1KB,
            5 => PSRAMSZ_A::_2KB,
            6 => PSRAMSZ_A::_4KB,
            7 => PSRAMSZ_A::_8KB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSRAMSZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_128B`"]
    #[inline(always)]
    pub fn is_128b(&self) -> bool {
        *self == PSRAMSZ_A::_128B
    }
    #[doc = "Checks if the value of the field is `_256B`"]
    #[inline(always)]
    pub fn is_256b(&self) -> bool {
        *self == PSRAMSZ_A::_256B
    }
    #[doc = "Checks if the value of the field is `_512B`"]
    #[inline(always)]
    pub fn is_512b(&self) -> bool {
        *self == PSRAMSZ_A::_512B
    }
    #[doc = "Checks if the value of the field is `_1KB`"]
    #[inline(always)]
    pub fn is_1kb(&self) -> bool {
        *self == PSRAMSZ_A::_1KB
    }
    #[doc = "Checks if the value of the field is `_2KB`"]
    #[inline(always)]
    pub fn is_2kb(&self) -> bool {
        *self == PSRAMSZ_A::_2KB
    }
    #[doc = "Checks if the value of the field is `_4KB`"]
    #[inline(always)]
    pub fn is_4kb(&self) -> bool {
        *self == PSRAMSZ_A::_4KB
    }
    #[doc = "Checks if the value of the field is `_8KB`"]
    #[inline(always)]
    pub fn is_8kb(&self) -> bool {
        *self == PSRAMSZ_A::_8KB
    }
}
#[doc = "Write proxy for field `PSRAMSZ`"]
pub struct PSRAMSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> PSRAMSZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSRAMSZ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No row size limitation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSRAMSZ_A::_0)
    }
    #[doc = "128 B"]
    #[inline(always)]
    pub fn _128b(self) -> &'a mut W {
        self.variant(PSRAMSZ_A::_128B)
    }
    #[doc = "256 B"]
    #[inline(always)]
    pub fn _256b(self) -> &'a mut W {
        self.variant(PSRAMSZ_A::_256B)
    }
    #[doc = "512 B"]
    #[inline(always)]
    pub fn _512b(self) -> &'a mut W {
        self.variant(PSRAMSZ_A::_512B)
    }
    #[doc = "1024 B"]
    #[inline(always)]
    pub fn _1kb(self) -> &'a mut W {
        self.variant(PSRAMSZ_A::_1KB)
    }
    #[doc = "2048 B"]
    #[inline(always)]
    pub fn _2kb(self) -> &'a mut W {
        self.variant(PSRAMSZ_A::_2KB)
    }
    #[doc = "4096 B"]
    #[inline(always)]
    pub fn _4kb(self) -> &'a mut W {
        self.variant(PSRAMSZ_A::_4KB)
    }
    #[doc = "8192 B"]
    #[inline(always)]
    pub fn _8kb(self) -> &'a mut W {
        self.variant(PSRAMSZ_A::_8KB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `IRDYDLY`"]
pub type IRDYDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IRDYDLY`"]
pub struct IRDYDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDYDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CS2n Read Wait State Minus One"]
    #[inline(always)]
    pub fn rdwsm(&self) -> RDWSM_R {
        RDWSM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CS2n Write Wait State Minus One"]
    #[inline(always)]
    pub fn wrwsm(&self) -> WRWSM_R {
        WRWSM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - CS2n Inter-transfer Capture Width"]
    #[inline(always)]
    pub fn capwidth(&self) -> CAPWIDTH_R {
        CAPWIDTH_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:18 - PSRAM Row Size"]
    #[inline(always)]
    pub fn psramsz(&self) -> PSRAMSZ_R {
        PSRAMSZ_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 24:25 - CS2n Input Ready Delay"]
    #[inline(always)]
    pub fn irdydly(&self) -> IRDYDLY_R {
        IRDYDLY_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CS2n Read Wait State Minus One"]
    #[inline(always)]
    pub fn rdwsm(&mut self) -> RDWSM_W {
        RDWSM_W { w: self }
    }
    #[doc = "Bit 4 - CS2n Write Wait State Minus One"]
    #[inline(always)]
    pub fn wrwsm(&mut self) -> WRWSM_W {
        WRWSM_W { w: self }
    }
    #[doc = "Bits 12:13 - CS2n Inter-transfer Capture Width"]
    #[inline(always)]
    pub fn capwidth(&mut self) -> CAPWIDTH_W {
        CAPWIDTH_W { w: self }
    }
    #[doc = "Bits 16:18 - PSRAM Row Size"]
    #[inline(always)]
    pub fn psramsz(&mut self) -> PSRAMSZ_W {
        PSRAMSZ_W { w: self }
    }
    #[doc = "Bits 24:25 - CS2n Input Ready Delay"]
    #[inline(always)]
    pub fn irdydly(&mut self) -> IRDYDLY_W {
        IRDYDLY_W { w: self }
    }
}
