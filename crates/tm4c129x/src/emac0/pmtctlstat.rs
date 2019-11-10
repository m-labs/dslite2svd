#[doc = "Reader of register PMTCTLSTAT"]
pub type R = crate::R<u32, super::PMTCTLSTAT>;
#[doc = "Writer for register PMTCTLSTAT"]
pub type W = crate::W<u32, super::PMTCTLSTAT>;
#[doc = "Register PMTCTLSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::PMTCTLSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWRDWN`"]
pub type PWRDWN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRDWN`"]
pub struct PWRDWN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRDWN_W<'a> {
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
#[doc = "Reader of field `MGKPKTEN`"]
pub type MGKPKTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MGKPKTEN`"]
pub struct MGKPKTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MGKPKTEN_W<'a> {
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
#[doc = "Reader of field `WUPFREN`"]
pub type WUPFREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUPFREN`"]
pub struct WUPFREN_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPFREN_W<'a> {
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
#[doc = "Reader of field `MGKPRX`"]
pub type MGKPRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MGKPRX`"]
pub struct MGKPRX_W<'a> {
    w: &'a mut W,
}
impl<'a> MGKPRX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `WUPRX`"]
pub type WUPRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUPRX`"]
pub struct WUPRX_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPRX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `GLBLUCAST`"]
pub type GLBLUCAST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GLBLUCAST`"]
pub struct GLBLUCAST_W<'a> {
    w: &'a mut W,
}
impl<'a> GLBLUCAST_W<'a> {
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
#[doc = "Reader of field `RWKPTR`"]
pub type RWKPTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RWKPTR`"]
pub struct RWKPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RWKPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `WUPFRRST`"]
pub type WUPFRRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUPFRRST`"]
pub struct WUPFRRST_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPFRRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Power Down"]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Magic Packet Enable"]
    #[inline(always)]
    pub fn mgkpkten(&self) -> MGKPKTEN_R {
        MGKPKTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wake-Up Frame Enable"]
    #[inline(always)]
    pub fn wupfren(&self) -> WUPFREN_R {
        WUPFREN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Magic Packet Received"]
    #[inline(always)]
    pub fn mgkprx(&self) -> MGKPRX_R {
        MGKPRX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wake-Up Frame Received"]
    #[inline(always)]
    pub fn wuprx(&self) -> WUPRX_R {
        WUPRX_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Global Unicast"]
    #[inline(always)]
    pub fn glblucast(&self) -> GLBLUCAST_R {
        GLBLUCAST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Remote Wake-Up FIFO Pointer"]
    #[inline(always)]
    pub fn rwkptr(&self) -> RWKPTR_R {
        RWKPTR_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 31 - Wake-Up Frame Filter Register Pointer Reset"]
    #[inline(always)]
    pub fn wupfrrst(&self) -> WUPFRRST_R {
        WUPFRRST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Down"]
    #[inline(always)]
    pub fn pwrdwn(&mut self) -> PWRDWN_W {
        PWRDWN_W { w: self }
    }
    #[doc = "Bit 1 - Magic Packet Enable"]
    #[inline(always)]
    pub fn mgkpkten(&mut self) -> MGKPKTEN_W {
        MGKPKTEN_W { w: self }
    }
    #[doc = "Bit 2 - Wake-Up Frame Enable"]
    #[inline(always)]
    pub fn wupfren(&mut self) -> WUPFREN_W {
        WUPFREN_W { w: self }
    }
    #[doc = "Bit 5 - Magic Packet Received"]
    #[inline(always)]
    pub fn mgkprx(&mut self) -> MGKPRX_W {
        MGKPRX_W { w: self }
    }
    #[doc = "Bit 6 - Wake-Up Frame Received"]
    #[inline(always)]
    pub fn wuprx(&mut self) -> WUPRX_W {
        WUPRX_W { w: self }
    }
    #[doc = "Bit 9 - Global Unicast"]
    #[inline(always)]
    pub fn glblucast(&mut self) -> GLBLUCAST_W {
        GLBLUCAST_W { w: self }
    }
    #[doc = "Bits 24:26 - Remote Wake-Up FIFO Pointer"]
    #[inline(always)]
    pub fn rwkptr(&mut self) -> RWKPTR_W {
        RWKPTR_W { w: self }
    }
    #[doc = "Bit 31 - Wake-Up Frame Filter Register Pointer Reset"]
    #[inline(always)]
    pub fn wupfrrst(&mut self) -> WUPFRRST_W {
        WUPFRRST_W { w: self }
    }
}
