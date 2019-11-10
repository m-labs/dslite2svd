#[doc = "Reader of register CR1"]
pub type R = crate::R<u32, super::CR1>;
#[doc = "Writer for register CR1"]
pub type W = crate::W<u32, super::CR1>;
#[doc = "Register CR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LBM`"]
pub type LBM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LBM`"]
pub struct LBM_W<'a> {
    w: &'a mut W,
}
impl<'a> LBM_W<'a> {
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
#[doc = "Reader of field `SSE`"]
pub type SSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSE`"]
pub struct SSE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSE_W<'a> {
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
#[doc = "Reader of field `MS`"]
pub type MS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MS`"]
pub struct MS_W<'a> {
    w: &'a mut W,
}
impl<'a> MS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `EOT`"]
pub type EOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOT`"]
pub struct EOT_W<'a> {
    w: &'a mut W,
}
impl<'a> EOT_W<'a> {
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
#[doc = "SSI Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Legacy SSI mode"]
    LEGACY,
    #[doc = "1: Bi-SSI mode"]
    BI,
    #[doc = "2: Quad-SSI Mode"]
    QUAD,
    #[doc = "3: Advanced SSI Mode with 8-bit packet size"]
    ADVANCED,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        match variant {
            MODE_A::LEGACY => 0,
            MODE_A::BI => 1,
            MODE_A::QUAD => 2,
            MODE_A::ADVANCED => 3,
        }
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::LEGACY,
            1 => MODE_A::BI,
            2 => MODE_A::QUAD,
            3 => MODE_A::ADVANCED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEGACY`"]
    #[inline(always)]
    pub fn is_legacy(&self) -> bool {
        *self == MODE_A::LEGACY
    }
    #[doc = "Checks if the value of the field is `BI`"]
    #[inline(always)]
    pub fn is_bi(&self) -> bool {
        *self == MODE_A::BI
    }
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == MODE_A::QUAD
    }
    #[doc = "Checks if the value of the field is `ADVANCED`"]
    #[inline(always)]
    pub fn is_advanced(&self) -> bool {
        *self == MODE_A::ADVANCED
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Legacy SSI mode"]
    #[inline(always)]
    pub fn legacy(self) -> &'a mut W {
        self.variant(MODE_A::LEGACY)
    }
    #[doc = "Bi-SSI mode"]
    #[inline(always)]
    pub fn bi(self) -> &'a mut W {
        self.variant(MODE_A::BI)
    }
    #[doc = "Quad-SSI Mode"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut W {
        self.variant(MODE_A::QUAD)
    }
    #[doc = "Advanced SSI Mode with 8-bit packet size"]
    #[inline(always)]
    pub fn advanced(self) -> &'a mut W {
        self.variant(MODE_A::ADVANCED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `HSCLKEN`"]
pub type HSCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSCLKEN`"]
pub struct HSCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSCLKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `FSSHLDFRM`"]
pub type FSSHLDFRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSSHLDFRM`"]
pub struct FSSHLDFRM_W<'a> {
    w: &'a mut W,
}
impl<'a> FSSHLDFRM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `EOM`"]
pub type EOM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOM`"]
pub struct EOM_W<'a> {
    w: &'a mut W,
}
impl<'a> EOM_W<'a> {
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
    #[doc = "Bit 0 - SSI Loopback Mode"]
    #[inline(always)]
    pub fn lbm(&self) -> LBM_R {
        LBM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SSI Synchronous Serial Port Enable"]
    #[inline(always)]
    pub fn sse(&self) -> SSE_R {
        SSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SSI Master/Slave Select"]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - End of Transmission"]
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - SSI Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - SSI Direction of Operation"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - High Speed Clock Enable"]
    #[inline(always)]
    pub fn hsclken(&self) -> HSCLKEN_R {
        HSCLKEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FSS Hold Frame"]
    #[inline(always)]
    pub fn fsshldfrm(&self) -> FSSHLDFRM_R {
        FSSHLDFRM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Stop Frame (End of Message)"]
    #[inline(always)]
    pub fn eom(&self) -> EOM_R {
        EOM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Loopback Mode"]
    #[inline(always)]
    pub fn lbm(&mut self) -> LBM_W {
        LBM_W { w: self }
    }
    #[doc = "Bit 1 - SSI Synchronous Serial Port Enable"]
    #[inline(always)]
    pub fn sse(&mut self) -> SSE_W {
        SSE_W { w: self }
    }
    #[doc = "Bit 2 - SSI Master/Slave Select"]
    #[inline(always)]
    pub fn ms(&mut self) -> MS_W {
        MS_W { w: self }
    }
    #[doc = "Bit 4 - End of Transmission"]
    #[inline(always)]
    pub fn eot(&mut self) -> EOT_W {
        EOT_W { w: self }
    }
    #[doc = "Bits 6:7 - SSI Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 8 - SSI Direction of Operation"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 9 - High Speed Clock Enable"]
    #[inline(always)]
    pub fn hsclken(&mut self) -> HSCLKEN_W {
        HSCLKEN_W { w: self }
    }
    #[doc = "Bit 10 - FSS Hold Frame"]
    #[inline(always)]
    pub fn fsshldfrm(&mut self) -> FSSHLDFRM_W {
        FSSHLDFRM_W { w: self }
    }
    #[doc = "Bit 11 - Stop Frame (End of Message)"]
    #[inline(always)]
    pub fn eom(&mut self) -> EOM_W {
        EOM_W { w: self }
    }
}
