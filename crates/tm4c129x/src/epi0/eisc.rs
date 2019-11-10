#[doc = "Reader of register EISC"]
pub type R = crate::R<u32, super::EISC>;
#[doc = "Writer for register EISC"]
pub type W = crate::W<u32, super::EISC>;
#[doc = "Register EISC `reset()`'s with value 0"]
impl crate::ResetValue for super::EISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOUT`"]
pub type TOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOUT`"]
pub struct TOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUT_W<'a> {
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
#[doc = "Reader of field `RSTALL`"]
pub type RSTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTALL`"]
pub struct RSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTALL_W<'a> {
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
#[doc = "Reader of field `WTFULL`"]
pub type WTFULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTFULL`"]
pub struct WTFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> WTFULL_W<'a> {
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
#[doc = "Reader of field `DMARDIC`"]
pub type DMARDIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMARDIC`"]
pub struct DMARDIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARDIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DMAWRIC`"]
pub type DMAWRIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAWRIC`"]
pub struct DMAWRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAWRIC_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Timeout Error"]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read Stalled Error"]
    #[inline(always)]
    pub fn rstall(&self) -> RSTALL_R {
        RSTALL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write FIFO Full Error"]
    #[inline(always)]
    pub fn wtfull(&self) -> WTFULL_R {
        WTFULL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Read uDMA Interrupt Clear"]
    #[inline(always)]
    pub fn dmardic(&self) -> DMARDIC_R {
        DMARDIC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write uDMA Interrupt Clear"]
    #[inline(always)]
    pub fn dmawric(&self) -> DMAWRIC_R {
        DMAWRIC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timeout Error"]
    #[inline(always)]
    pub fn tout(&mut self) -> TOUT_W {
        TOUT_W { w: self }
    }
    #[doc = "Bit 1 - Read Stalled Error"]
    #[inline(always)]
    pub fn rstall(&mut self) -> RSTALL_W {
        RSTALL_W { w: self }
    }
    #[doc = "Bit 2 - Write FIFO Full Error"]
    #[inline(always)]
    pub fn wtfull(&mut self) -> WTFULL_W {
        WTFULL_W { w: self }
    }
    #[doc = "Bit 3 - Read uDMA Interrupt Clear"]
    #[inline(always)]
    pub fn dmardic(&mut self) -> DMARDIC_W {
        DMARDIC_W { w: self }
    }
    #[doc = "Bit 4 - Write uDMA Interrupt Clear"]
    #[inline(always)]
    pub fn dmawric(&mut self) -> DMAWRIC_W {
        DMAWRIC_W { w: self }
    }
}
