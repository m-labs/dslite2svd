#[doc = "Reader of register DSLPCLKCFG"]
pub type R = crate::R<u32, super::DSLPCLKCFG>;
#[doc = "Writer for register DSLPCLKCFG"]
pub type W = crate::W<u32, super::DSLPCLKCFG>;
#[doc = "Register DSLPCLKCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DSLPCLKCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PIOSCPD`"]
pub type PIOSCPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIOSCPD`"]
pub struct PIOSCPD_W<'a> {
    w: &'a mut W,
}
impl<'a> PIOSCPD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum O_A {
    #[doc = "0: MOSC"]
    IGN = 0,
    #[doc = "1: PIOSC"]
    IO = 1,
    #[doc = "3: LFIOSC"]
    _30 = 3,
    #[doc = "7: 32.768 kHz"]
    _32 = 7,
}
impl From<O_A> for u8 {
    #[inline(always)]
    fn from(variant: O_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `O`"]
pub type O_R = crate::R<u8, O_A>;
impl O_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, O_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(O_A::IGN),
            1 => Val(O_A::IO),
            3 => Val(O_A::_30),
            7 => Val(O_A::_32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IGN`"]
    #[inline(always)]
    pub fn is_ign(&self) -> bool {
        *self == O_A::IGN
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == O_A::IO
    }
    #[doc = "Checks if the value of the field is `_30`"]
    #[inline(always)]
    pub fn is_30(&self) -> bool {
        *self == O_A::_30
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == O_A::_32
    }
}
#[doc = "Write proxy for field `O`"]
pub struct O_W<'a> {
    w: &'a mut W,
}
impl<'a> O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn ign(self) -> &'a mut W {
        self.variant(O_A::IGN)
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(O_A::IO)
    }
    #[doc = "LFIOSC"]
    #[inline(always)]
    pub fn _30(self) -> &'a mut W {
        self.variant(O_A::_30)
    }
    #[doc = "32.768 kHz"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(O_A::_32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `D`"]
pub type D_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `D`"]
pub struct D_W<'a> {
    w: &'a mut W,
}
impl<'a> D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 23)) | (((value as u32) & 0x3f) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - PIOSC Power Down Request"]
    #[inline(always)]
    pub fn pioscpd(&self) -> PIOSCPD_R {
        PIOSCPD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Clock Source"]
    #[inline(always)]
    pub fn o(&self) -> O_R {
        O_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 23:28 - Divider Field Override"]
    #[inline(always)]
    pub fn d(&self) -> D_R {
        D_R::new(((self.bits >> 23) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - PIOSC Power Down Request"]
    #[inline(always)]
    pub fn pioscpd(&mut self) -> PIOSCPD_W {
        PIOSCPD_W { w: self }
    }
    #[doc = "Bits 4:6 - Clock Source"]
    #[inline(always)]
    pub fn o(&mut self) -> O_W {
        O_W { w: self }
    }
    #[doc = "Bits 23:28 - Divider Field Override"]
    #[inline(always)]
    pub fn d(&mut self) -> D_W {
        D_W { w: self }
    }
}
