#[doc = "Reader of register BAUD"]
pub type R = crate::R<u32, super::BAUD>;
#[doc = "Writer for register BAUD"]
pub type W = crate::W<u32, super::BAUD>;
#[doc = "Register BAUD `reset()`'s with value 0"]
impl crate::ResetValue for super::BAUD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COUNT0`"]
pub type COUNT0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COUNT0`"]
pub struct COUNT0_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `COUNT1`"]
pub type COUNT1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COUNT1`"]
pub struct COUNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Baud Rate Counter 0"]
    #[inline(always)]
    pub fn count0(&self) -> COUNT0_R {
        COUNT0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Baud Rate Counter 1"]
    #[inline(always)]
    pub fn count1(&self) -> COUNT1_R {
        COUNT1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Baud Rate Counter 0"]
    #[inline(always)]
    pub fn count0(&mut self) -> COUNT0_W {
        COUNT0_W { w: self }
    }
    #[doc = "Bits 16:31 - Baud Rate Counter 1"]
    #[inline(always)]
    pub fn count1(&mut self) -> COUNT1_W {
        COUNT1_W { w: self }
    }
}
