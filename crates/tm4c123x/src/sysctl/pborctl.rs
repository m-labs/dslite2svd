#[doc = "Reader of register PBORCTL"]
pub type R = crate::R<u32, super::PBORCTL>;
#[doc = "Writer for register PBORCTL"]
pub type W = crate::W<u32, super::PBORCTL>;
#[doc = "Register PBORCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::PBORCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BOR1`"]
pub type BOR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOR1`"]
pub struct BOR1_W<'a> {
    w: &'a mut W,
}
impl<'a> BOR1_W<'a> {
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
#[doc = "Reader of field `BOR0`"]
pub type BOR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOR0`"]
pub struct BOR0_W<'a> {
    w: &'a mut W,
}
impl<'a> BOR0_W<'a> {
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
    #[doc = "Bit 1 - VDD under BOR1 Event Action"]
    #[inline(always)]
    pub fn bor1(&self) -> BOR1_R {
        BOR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - VDD under BOR0 Event Action"]
    #[inline(always)]
    pub fn bor0(&self) -> BOR0_R {
        BOR0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - VDD under BOR1 Event Action"]
    #[inline(always)]
    pub fn bor1(&mut self) -> BOR1_W {
        BOR1_W { w: self }
    }
    #[doc = "Bit 2 - VDD under BOR0 Event Action"]
    #[inline(always)]
    pub fn bor0(&mut self) -> BOR0_W {
        BOR0_W { w: self }
    }
}
