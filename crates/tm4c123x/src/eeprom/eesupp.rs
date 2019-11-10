#[doc = "Reader of register EESUPP"]
pub type R = crate::R<u32, super::EESUPP>;
#[doc = "Writer for register EESUPP"]
pub type W = crate::W<u32, super::EESUPP>;
#[doc = "Register EESUPP `reset()`'s with value 0"]
impl crate::ResetValue for super::EESUPP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ERETRY`"]
pub type ERETRY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERETRY`"]
pub struct ERETRY_W<'a> {
    w: &'a mut W,
}
impl<'a> ERETRY_W<'a> {
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
#[doc = "Reader of field `PRETRY`"]
pub type PRETRY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRETRY`"]
pub struct PRETRY_W<'a> {
    w: &'a mut W,
}
impl<'a> PRETRY_W<'a> {
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
impl R {
    #[doc = "Bit 2 - Erase Must Be Retried"]
    #[inline(always)]
    pub fn eretry(&self) -> ERETRY_R {
        ERETRY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Programming Must Be Retried"]
    #[inline(always)]
    pub fn pretry(&self) -> PRETRY_R {
        PRETRY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Erase Must Be Retried"]
    #[inline(always)]
    pub fn eretry(&mut self) -> ERETRY_W {
        ERETRY_W { w: self }
    }
    #[doc = "Bit 3 - Programming Must Be Retried"]
    #[inline(always)]
    pub fn pretry(&mut self) -> PRETRY_W {
        PRETRY_W { w: self }
    }
}
