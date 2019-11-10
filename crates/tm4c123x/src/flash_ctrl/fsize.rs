#[doc = "Reader of register FSIZE"]
pub type R = crate::R<u32, super::FSIZE>;
#[doc = "Flash Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZE_A {
    #[doc = "127: 256 KB of Flash"]
    _256KB,
}
impl From<SIZE_A> for u16 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        match variant {
            SIZE_A::_256KB => 127,
        }
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
            127 => Val(SIZE_A::_256KB),
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
    #[doc = "Bits 0:15 - Flash Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
