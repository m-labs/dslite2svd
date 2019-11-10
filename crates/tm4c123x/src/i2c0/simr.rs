#[doc = "Reader of register SIMR"]
pub type R = crate::R<u32, super::SIMR>;
#[doc = "Writer for register SIMR"]
pub type W = crate::W<u32, super::SIMR>;
#[doc = "Register SIMR `reset()`'s with value 0"]
impl crate::ResetValue for super::SIMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATAIM`"]
pub type DATAIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAIM`"]
pub struct DATAIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAIM_W<'a> {
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
#[doc = "Reader of field `STARTIM`"]
pub type STARTIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STARTIM`"]
pub struct STARTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTIM_W<'a> {
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
#[doc = "Reader of field `STOPIM`"]
pub type STOPIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOPIM`"]
pub struct STOPIM_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPIM_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Data Interrupt Mask"]
    #[inline(always)]
    pub fn dataim(&self) -> DATAIM_R {
        DATAIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start Condition Interrupt Mask"]
    #[inline(always)]
    pub fn startim(&self) -> STARTIM_R {
        STARTIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stop Condition Interrupt Mask"]
    #[inline(always)]
    pub fn stopim(&self) -> STOPIM_R {
        STOPIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Interrupt Mask"]
    #[inline(always)]
    pub fn dataim(&mut self) -> DATAIM_W {
        DATAIM_W { w: self }
    }
    #[doc = "Bit 1 - Start Condition Interrupt Mask"]
    #[inline(always)]
    pub fn startim(&mut self) -> STARTIM_W {
        STARTIM_W { w: self }
    }
    #[doc = "Bit 2 - Stop Condition Interrupt Mask"]
    #[inline(always)]
    pub fn stopim(&mut self) -> STOPIM_W {
        STOPIM_W { w: self }
    }
}
