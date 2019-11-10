#[doc = "Reader of register ADDR3L"]
pub type R = crate::R<u32, super::ADDR3L>;
#[doc = "Writer for register ADDR3L"]
pub type W = crate::W<u32, super::ADDR3L>;
#[doc = "Register ADDR3L `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDR3L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDRLO`"]
pub type ADDRLO_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADDRLO`"]
pub struct ADDRLO_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRLO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - MAC Address3 \\[31:0\\]"]
    #[inline(always)]
    pub fn addrlo(&self) -> ADDRLO_R {
        ADDRLO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Address3 \\[31:0\\]"]
    #[inline(always)]
    pub fn addrlo(&mut self) -> ADDRLO_W {
        ADDRLO_W { w: self }
    }
}
