#[doc = "Reader of register DMACTL6"]
pub type R = crate::R<u16, super::DMACTL6>;
#[doc = "Writer for register DMACTL6"]
pub type W = crate::W<u16, super::DMACTL6>;
#[doc = "Register DMACTL6 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACTL6 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR`"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `IE`"]
pub type IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IE`"]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `EP`"]
pub type EP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EP`"]
pub struct EP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u16) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERR`"]
pub struct ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Burst Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRSTM_A {
    #[doc = "0: Bursts of unspecified length"]
    ANY,
    #[doc = "1: INCR4 or unspecified length"]
    INC4,
    #[doc = "2: INCR8, INCR4 or unspecified length"]
    INC8,
    #[doc = "3: INCR16, INCR8, INCR4 or unspecified length"]
    INC16,
}
impl From<BRSTM_A> for u8 {
    #[inline(always)]
    fn from(variant: BRSTM_A) -> Self {
        match variant {
            BRSTM_A::ANY => 0,
            BRSTM_A::INC4 => 1,
            BRSTM_A::INC8 => 2,
            BRSTM_A::INC16 => 3,
        }
    }
}
#[doc = "Reader of field `BRSTM`"]
pub type BRSTM_R = crate::R<u8, BRSTM_A>;
impl BRSTM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRSTM_A {
        match self.bits {
            0 => BRSTM_A::ANY,
            1 => BRSTM_A::INC4,
            2 => BRSTM_A::INC8,
            3 => BRSTM_A::INC16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ANY`"]
    #[inline(always)]
    pub fn is_any(&self) -> bool {
        *self == BRSTM_A::ANY
    }
    #[doc = "Checks if the value of the field is `INC4`"]
    #[inline(always)]
    pub fn is_inc4(&self) -> bool {
        *self == BRSTM_A::INC4
    }
    #[doc = "Checks if the value of the field is `INC8`"]
    #[inline(always)]
    pub fn is_inc8(&self) -> bool {
        *self == BRSTM_A::INC8
    }
    #[doc = "Checks if the value of the field is `INC16`"]
    #[inline(always)]
    pub fn is_inc16(&self) -> bool {
        *self == BRSTM_A::INC16
    }
}
#[doc = "Write proxy for field `BRSTM`"]
pub struct BRSTM_W<'a> {
    w: &'a mut W,
}
impl<'a> BRSTM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRSTM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Bursts of unspecified length"]
    #[inline(always)]
    pub fn any(self) -> &'a mut W {
        self.variant(BRSTM_A::ANY)
    }
    #[doc = "INCR4 or unspecified length"]
    #[inline(always)]
    pub fn inc4(self) -> &'a mut W {
        self.variant(BRSTM_A::INC4)
    }
    #[doc = "INCR8, INCR4 or unspecified length"]
    #[inline(always)]
    pub fn inc8(self) -> &'a mut W {
        self.variant(BRSTM_A::INC8)
    }
    #[doc = "INCR16, INCR8, INCR4 or unspecified length"]
    #[inline(always)]
    pub fn inc16(self) -> &'a mut W {
        self.variant(BRSTM_A::INC16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u16) & 0x03) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DMA Transfer Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA Transfer Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Endpoint number"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Bus Error Bit"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - Burst Mode"]
    #[inline(always)]
    pub fn brstm(&self) -> BRSTM_R {
        BRSTM_R::new(((self.bits >> 9) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Transfer Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - DMA Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 2 - DMA Transfer Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 3 - DMA Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bits 4:7 - Endpoint number"]
    #[inline(always)]
    pub fn ep(&mut self) -> EP_W {
        EP_W { w: self }
    }
    #[doc = "Bit 8 - Bus Error Bit"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W { w: self }
    }
    #[doc = "Bits 9:10 - Burst Mode"]
    #[inline(always)]
    pub fn brstm(&mut self) -> BRSTM_W {
        BRSTM_W { w: self }
    }
}
