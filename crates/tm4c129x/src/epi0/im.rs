#[doc = "Reader of register IM"]
pub type R = crate::R<u32, super::IM>;
#[doc = "Writer for register IM"]
pub type W = crate::W<u32, super::IM>;
#[doc = "Register IM `reset()`'s with value 0"]
impl crate::ResetValue for super::IM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ERRIM`"]
pub type ERRIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERRIM`"]
pub struct ERRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIM_W<'a> {
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
#[doc = "Reader of field `RDIM`"]
pub type RDIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDIM`"]
pub struct RDIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RDIM_W<'a> {
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
#[doc = "Reader of field `WRIM`"]
pub type WRIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRIM`"]
pub struct WRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> WRIM_W<'a> {
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
#[doc = "Reader of field `DMARDIM`"]
pub type DMARDIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMARDIM`"]
pub struct DMARDIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARDIM_W<'a> {
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
#[doc = "Reader of field `DMAWRIM`"]
pub type DMAWRIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAWRIM`"]
pub struct DMAWRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAWRIM_W<'a> {
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
    #[doc = "Bit 0 - Error Interrupt Mask"]
    #[inline(always)]
    pub fn errim(&self) -> ERRIM_R {
        ERRIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn rdim(&self) -> RDIM_R {
        RDIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn wrim(&self) -> WRIM_R {
        WRIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Read uDMA Interrupt Mask"]
    #[inline(always)]
    pub fn dmardim(&self) -> DMARDIM_R {
        DMARDIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write uDMA Interrupt Mask"]
    #[inline(always)]
    pub fn dmawrim(&self) -> DMAWRIM_R {
        DMAWRIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error Interrupt Mask"]
    #[inline(always)]
    pub fn errim(&mut self) -> ERRIM_W {
        ERRIM_W { w: self }
    }
    #[doc = "Bit 1 - Read FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn rdim(&mut self) -> RDIM_W {
        RDIM_W { w: self }
    }
    #[doc = "Bit 2 - Write FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn wrim(&mut self) -> WRIM_W {
        WRIM_W { w: self }
    }
    #[doc = "Bit 3 - Read uDMA Interrupt Mask"]
    #[inline(always)]
    pub fn dmardim(&mut self) -> DMARDIM_W {
        DMARDIM_W { w: self }
    }
    #[doc = "Bit 4 - Write uDMA Interrupt Mask"]
    #[inline(always)]
    pub fn dmawrim(&mut self) -> DMAWRIM_W {
        DMAWRIM_W { w: self }
    }
}
