#[doc = "Reader of register TPCTL"]
pub type R = crate::R<u32, super::TPCTL>;
#[doc = "Writer for register TPCTL"]
pub type W = crate::W<u32, super::TPCTL>;
#[doc = "Register TPCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::TPCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TPEN`"]
pub type TPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TPEN`"]
pub struct TPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TPEN_W<'a> {
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
#[doc = "Reader of field `TPCLR`"]
pub type TPCLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TPCLR`"]
pub struct TPCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TPCLR_W<'a> {
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
#[doc = "HIB Memory Clear on Tamper Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMCLR_A {
    #[doc = "0: Do not Clear HIB memory on tamper event"]
    NONE,
    #[doc = "1: Clear Lower 32 Bytes of HIB memory on tamper event"]
    LOW32,
    #[doc = "2: Clear upper 32 Bytes of HIB memory on tamper event"]
    HIGH32,
    #[doc = "3: Clear all HIB memory on tamper event"]
    ALL,
}
impl From<MEMCLR_A> for u8 {
    #[inline(always)]
    fn from(variant: MEMCLR_A) -> Self {
        match variant {
            MEMCLR_A::NONE => 0,
            MEMCLR_A::LOW32 => 1,
            MEMCLR_A::HIGH32 => 2,
            MEMCLR_A::ALL => 3,
        }
    }
}
#[doc = "Reader of field `MEMCLR`"]
pub type MEMCLR_R = crate::R<u8, MEMCLR_A>;
impl MEMCLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMCLR_A {
        match self.bits {
            0 => MEMCLR_A::NONE,
            1 => MEMCLR_A::LOW32,
            2 => MEMCLR_A::HIGH32,
            3 => MEMCLR_A::ALL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == MEMCLR_A::NONE
    }
    #[doc = "Checks if the value of the field is `LOW32`"]
    #[inline(always)]
    pub fn is_low32(&self) -> bool {
        *self == MEMCLR_A::LOW32
    }
    #[doc = "Checks if the value of the field is `HIGH32`"]
    #[inline(always)]
    pub fn is_high32(&self) -> bool {
        *self == MEMCLR_A::HIGH32
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == MEMCLR_A::ALL
    }
}
#[doc = "Write proxy for field `MEMCLR`"]
pub struct MEMCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEMCLR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Do not Clear HIB memory on tamper event"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(MEMCLR_A::NONE)
    }
    #[doc = "Clear Lower 32 Bytes of HIB memory on tamper event"]
    #[inline(always)]
    pub fn low32(self) -> &'a mut W {
        self.variant(MEMCLR_A::LOW32)
    }
    #[doc = "Clear upper 32 Bytes of HIB memory on tamper event"]
    #[inline(always)]
    pub fn high32(self) -> &'a mut W {
        self.variant(MEMCLR_A::HIGH32)
    }
    #[doc = "Clear all HIB memory on tamper event"]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(MEMCLR_A::ALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `WAKE`"]
pub type WAKE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKE`"]
pub struct WAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Tamper Module Enable"]
    #[inline(always)]
    pub fn tpen(&self) -> TPEN_R {
        TPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Tamper Event Clear"]
    #[inline(always)]
    pub fn tpclr(&self) -> TPCLR_R {
        TPCLR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - HIB Memory Clear on Tamper Event"]
    #[inline(always)]
    pub fn memclr(&self) -> MEMCLR_R {
        MEMCLR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Wake from Hibernate on a Tamper Event"]
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper Module Enable"]
    #[inline(always)]
    pub fn tpen(&mut self) -> TPEN_W {
        TPEN_W { w: self }
    }
    #[doc = "Bit 4 - Tamper Event Clear"]
    #[inline(always)]
    pub fn tpclr(&mut self) -> TPCLR_W {
        TPCLR_W { w: self }
    }
    #[doc = "Bits 8:9 - HIB Memory Clear on Tamper Event"]
    #[inline(always)]
    pub fn memclr(&mut self) -> MEMCLR_W {
        MEMCLR_W { w: self }
    }
    #[doc = "Bit 11 - Wake from Hibernate on a Tamper Event"]
    #[inline(always)]
    pub fn wake(&mut self) -> WAKE_W {
        WAKE_W { w: self }
    }
}
