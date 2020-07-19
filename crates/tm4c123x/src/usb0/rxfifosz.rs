#[doc = "Reader of register RXFIFOSZ"]
pub type R = crate::R<u8, super::RXFIFOSZ>;
#[doc = "Writer for register RXFIFOSZ"]
pub type W = crate::W<u8, super::RXFIFOSZ>;
#[doc = "Register RXFIFOSZ `reset()`'s with value 0"]
impl crate::ResetValue for super::RXFIFOSZ {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Max Packet Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIZE_A {
    #[doc = "0: 8"]
    _8 = 0,
    #[doc = "1: 16"]
    _16 = 1,
    #[doc = "2: 32"]
    _32 = 2,
    #[doc = "3: 64"]
    _64 = 3,
    #[doc = "4: 128"]
    _128 = 4,
    #[doc = "5: 256"]
    _256 = 5,
    #[doc = "6: 512"]
    _512 = 6,
    #[doc = "7: 1024"]
    _1024 = 7,
    #[doc = "8: 2048"]
    _2048 = 8,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u8, SIZE_A>;
impl SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SIZE_A::_8),
            1 => Val(SIZE_A::_16),
            2 => Val(SIZE_A::_32),
            3 => Val(SIZE_A::_64),
            4 => Val(SIZE_A::_128),
            5 => Val(SIZE_A::_256),
            6 => Val(SIZE_A::_512),
            7 => Val(SIZE_A::_1024),
            8 => Val(SIZE_A::_2048),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == SIZE_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == SIZE_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == SIZE_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == SIZE_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == SIZE_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == SIZE_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == SIZE_A::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == SIZE_A::_1024
    }
    #[doc = "Checks if the value of the field is `_2048`"]
    #[inline(always)]
    pub fn is_2048(&self) -> bool {
        *self == SIZE_A::_2048
    }
}
#[doc = "Write proxy for field `SIZE`"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(SIZE_A::_8)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(SIZE_A::_16)
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(SIZE_A::_32)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(SIZE_A::_64)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(SIZE_A::_128)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(SIZE_A::_256)
    }
    #[doc = "512"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(SIZE_A::_512)
    }
    #[doc = "1024"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(SIZE_A::_1024)
    }
    #[doc = "2048"]
    #[inline(always)]
    pub fn _2048(self) -> &'a mut W {
        self.variant(SIZE_A::_2048)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DPB`"]
pub type DPB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPB`"]
pub struct DPB_W<'a> {
    w: &'a mut W,
}
impl<'a> DPB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Max Packet Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Double Packet Buffer Support"]
    #[inline(always)]
    pub fn dpb(&self) -> DPB_R {
        DPB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Max Packet Size"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
    #[doc = "Bit 4 - Double Packet Buffer Support"]
    #[inline(always)]
    pub fn dpb(&mut self) -> DPB_W {
        DPB_W { w: self }
    }
}
