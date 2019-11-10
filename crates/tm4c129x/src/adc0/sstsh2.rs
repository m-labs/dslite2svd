#[doc = "Reader of register SSTSH2"]
pub type R = crate::R<u32, super::SSTSH2>;
#[doc = "Writer for register SSTSH2"]
pub type W = crate::W<u32, super::SSTSH2>;
#[doc = "Register SSTSH2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSTSH2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSH0`"]
pub type TSH0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TSH0`"]
pub struct TSH0_W<'a> {
    w: &'a mut W,
}
impl<'a> TSH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `TSH1`"]
pub type TSH1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TSH1`"]
pub struct TSH1_W<'a> {
    w: &'a mut W,
}
impl<'a> TSH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `TSH2`"]
pub type TSH2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TSH2`"]
pub struct TSH2_W<'a> {
    w: &'a mut W,
}
impl<'a> TSH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TSH3`"]
pub type TSH3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TSH3`"]
pub struct TSH3_W<'a> {
    w: &'a mut W,
}
impl<'a> TSH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - 1st Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh0(&self) -> TSH0_R {
        TSH0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 2nd Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh1(&self) -> TSH1_R {
        TSH1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 3rd Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh2(&self) -> TSH2_R {
        TSH2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 4th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh3(&self) -> TSH3_R {
        TSH3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1st Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh0(&mut self) -> TSH0_W {
        TSH0_W { w: self }
    }
    #[doc = "Bits 4:7 - 2nd Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh1(&mut self) -> TSH1_W {
        TSH1_W { w: self }
    }
    #[doc = "Bits 8:11 - 3rd Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh2(&mut self) -> TSH2_W {
        TSH2_W { w: self }
    }
    #[doc = "Bits 12:15 - 4th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh3(&mut self) -> TSH3_W {
        TSH3_W { w: self }
    }
}
