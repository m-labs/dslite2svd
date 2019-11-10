#[doc = "Reader of register SSPRI"]
pub type R = crate::R<u32, super::SSPRI>;
#[doc = "Writer for register SSPRI"]
pub type W = crate::W<u32, super::SSPRI>;
#[doc = "Register SSPRI `reset()`'s with value 0"]
impl crate::ResetValue for super::SSPRI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SS0`"]
pub type SS0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SS0`"]
pub struct SS0_W<'a> {
    w: &'a mut W,
}
impl<'a> SS0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `SS1`"]
pub type SS1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SS1`"]
pub struct SS1_W<'a> {
    w: &'a mut W,
}
impl<'a> SS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SS2`"]
pub type SS2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SS2`"]
pub struct SS2_W<'a> {
    w: &'a mut W,
}
impl<'a> SS2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `SS3`"]
pub type SS3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SS3`"]
pub struct SS3_W<'a> {
    w: &'a mut W,
}
impl<'a> SS3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SS0 Priority"]
    #[inline(always)]
    pub fn ss0(&self) -> SS0_R {
        SS0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - SS1 Priority"]
    #[inline(always)]
    pub fn ss1(&self) -> SS1_R {
        SS1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - SS2 Priority"]
    #[inline(always)]
    pub fn ss2(&self) -> SS2_R {
        SS2_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - SS3 Priority"]
    #[inline(always)]
    pub fn ss3(&self) -> SS3_R {
        SS3_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SS0 Priority"]
    #[inline(always)]
    pub fn ss0(&mut self) -> SS0_W {
        SS0_W { w: self }
    }
    #[doc = "Bits 4:5 - SS1 Priority"]
    #[inline(always)]
    pub fn ss1(&mut self) -> SS1_W {
        SS1_W { w: self }
    }
    #[doc = "Bits 8:9 - SS2 Priority"]
    #[inline(always)]
    pub fn ss2(&mut self) -> SS2_W {
        SS2_W { w: self }
    }
    #[doc = "Bits 12:13 - SS3 Priority"]
    #[inline(always)]
    pub fn ss3(&mut self) -> SS3_W {
        SS3_W { w: self }
    }
}
