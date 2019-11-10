#[doc = "Reader of register TBPR"]
pub type R = crate::R<u32, super::TBPR>;
#[doc = "Writer for register TBPR"]
pub type W = crate::W<u32, super::TBPR>;
#[doc = "Register TBPR `reset()`'s with value 0"]
impl crate::ResetValue for super::TBPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TBPSR`"]
pub type TBPSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TBPSR`"]
pub struct TBPSR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `TBPSRH`"]
pub type TBPSRH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TBPSRH`"]
pub struct TBPSRH_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPSRH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPTM Timer B Prescale"]
    #[inline(always)]
    pub fn tbpsr(&self) -> TBPSR_R {
        TBPSR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPTM Timer B Prescale High Byte"]
    #[inline(always)]
    pub fn tbpsrh(&self) -> TBPSRH_R {
        TBPSRH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM Timer B Prescale"]
    #[inline(always)]
    pub fn tbpsr(&mut self) -> TBPSR_W {
        TBPSR_W { w: self }
    }
    #[doc = "Bits 8:15 - GPTM Timer B Prescale High Byte"]
    #[inline(always)]
    pub fn tbpsrh(&mut self) -> TBPSRH_W {
        TBPSRH_W { w: self }
    }
}
