#[doc = "Reader of register SSIZE"]
pub type R = crate::R<u32, super::SSIZE>;
#[doc = "SRAM Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SIZE_A {
    #[doc = "1023: 256 KB of SRAM"]
    _256KB = 1023,
}
impl From<SIZE_A> for u16 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u16, SIZE_A>;
impl SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, SIZE_A> {
        use crate::Variant::*;
        match self.bits {
            1023 => Val(SIZE_A::_256KB),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_256KB`"]
    #[inline(always)]
    pub fn is_256kb(&self) -> bool {
        *self == SIZE_A::_256KB
    }
}
impl R {
    #[doc = "Bits 0:15 - SRAM Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
