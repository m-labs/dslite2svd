#[doc = "Writer for register CSRH0"]
pub type W = crate::W<u8, super::CSRH0>;
#[doc = "Register CSRH0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSRH0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `FLUSH`"]
pub struct FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUSH_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Write proxy for field `DT`"]
pub struct DT_W<'a> {
    w: &'a mut W,
}
impl<'a> DT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write proxy for field `DTWE`"]
pub struct DTWE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTWE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Flush FIFO"]
    #[inline(always)]
    pub fn flush(&mut self) -> FLUSH_W {
        FLUSH_W { w: self }
    }
    #[doc = "Bit 1 - Data Toggle"]
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W {
        DT_W { w: self }
    }
    #[doc = "Bit 2 - Data Toggle Write Enable"]
    #[inline(always)]
    pub fn dtwe(&mut self) -> DTWE_W {
        DTWE_W { w: self }
    }
}
