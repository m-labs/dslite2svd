#[doc = "Reader of register SSTSH3"]
pub type R = crate::R<u32, super::SSTSH3>;
#[doc = "Writer for register SSTSH3"]
pub type W = crate::W<u32, super::SSTSH3>;
#[doc = "Register SSTSH3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSTSH3 {
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
impl R {
    #[doc = "Bits 0:3 - 1st Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh0(&self) -> TSH0_R {
        TSH0_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1st Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh0(&mut self) -> TSH0_W {
        TSH0_W { w: self }
    }
}
