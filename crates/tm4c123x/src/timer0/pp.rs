#[doc = "Reader of register PP"]
pub type R = crate::R<u32, super::PP>;
#[doc = "Count Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZE_A {
    #[doc = "0: Timer A and Timer B counters are 16 bits each with an 8-bit prescale counter"]
    _16,
    #[doc = "1: Timer A and Timer B counters are 32 bits each with a 16-bit prescale counter"]
    _32,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        match variant {
            SIZE_A::_16 => 0,
            SIZE_A::_32 => 1,
        }
    }
}
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u8, SIZE_A>;
impl SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SIZE_A::_16),
            1 => Val(SIZE_A::_32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == SIZE_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == SIZE_A::_32
    }
}
impl R {
    #[doc = "Bits 0:3 - Count Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x0f) as u8)
    }
}
