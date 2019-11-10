#[doc = "Reader of register PSSI"]
pub type R = crate::R<u32, super::PSSI>;
#[doc = "Writer for register PSSI"]
pub type W = crate::W<u32, super::PSSI>;
#[doc = "Register PSSI `reset()`'s with value 0"]
impl crate::ResetValue for super::PSSI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SS0`"]
pub type SS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SS0`"]
pub struct SS0_W<'a> {
    w: &'a mut W,
}
impl<'a> SS0_W<'a> {
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
#[doc = "Reader of field `SS1`"]
pub type SS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SS1`"]
pub struct SS1_W<'a> {
    w: &'a mut W,
}
impl<'a> SS1_W<'a> {
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
#[doc = "Reader of field `SS2`"]
pub type SS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SS2`"]
pub struct SS2_W<'a> {
    w: &'a mut W,
}
impl<'a> SS2_W<'a> {
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
#[doc = "Reader of field `SS3`"]
pub type SS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SS3`"]
pub struct SS3_W<'a> {
    w: &'a mut W,
}
impl<'a> SS3_W<'a> {
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
#[doc = "Reader of field `SYNCWAIT`"]
pub type SYNCWAIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNCWAIT`"]
pub struct SYNCWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCWAIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `GSYNC`"]
pub type GSYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GSYNC`"]
pub struct GSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> GSYNC_W<'a> {
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
    #[doc = "Bit 0 - SS0 Initiate"]
    #[inline(always)]
    pub fn ss0(&self) -> SS0_R {
        SS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SS1 Initiate"]
    #[inline(always)]
    pub fn ss1(&self) -> SS1_R {
        SS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SS2 Initiate"]
    #[inline(always)]
    pub fn ss2(&self) -> SS2_R {
        SS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SS3 Initiate"]
    #[inline(always)]
    pub fn ss3(&self) -> SS3_R {
        SS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Synchronize Wait"]
    #[inline(always)]
    pub fn syncwait(&self) -> SYNCWAIT_R {
        SYNCWAIT_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Global Synchronize"]
    #[inline(always)]
    pub fn gsync(&self) -> GSYNC_R {
        GSYNC_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS0 Initiate"]
    #[inline(always)]
    pub fn ss0(&mut self) -> SS0_W {
        SS0_W { w: self }
    }
    #[doc = "Bit 1 - SS1 Initiate"]
    #[inline(always)]
    pub fn ss1(&mut self) -> SS1_W {
        SS1_W { w: self }
    }
    #[doc = "Bit 2 - SS2 Initiate"]
    #[inline(always)]
    pub fn ss2(&mut self) -> SS2_W {
        SS2_W { w: self }
    }
    #[doc = "Bit 3 - SS3 Initiate"]
    #[inline(always)]
    pub fn ss3(&mut self) -> SS3_W {
        SS3_W { w: self }
    }
    #[doc = "Bit 27 - Synchronize Wait"]
    #[inline(always)]
    pub fn syncwait(&mut self) -> SYNCWAIT_W {
        SYNCWAIT_W { w: self }
    }
    #[doc = "Bit 31 - Global Synchronize"]
    #[inline(always)]
    pub fn gsync(&mut self) -> GSYNC_W {
        GSYNC_W { w: self }
    }
}
