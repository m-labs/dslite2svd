#[doc = "Reader of register INT"]
pub type R = crate::R<u32, super::INT>;
#[doc = "Interrupt Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum INTID_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "32768: Status Interrupt"]
    STATUS = 32768,
}
impl From<INTID_A> for u16 {
    #[inline(always)]
    fn from(variant: INTID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INTID`"]
pub type INTID_R = crate::R<u16, INTID_A>;
impl INTID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, INTID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INTID_A::NONE),
            32768 => Val(INTID_A::STATUS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == INTID_A::NONE
    }
    #[doc = "Checks if the value of the field is `STATUS`"]
    #[inline(always)]
    pub fn is_status(&self) -> bool {
        *self == INTID_A::STATUS
    }
}
impl R {
    #[doc = "Bits 0:15 - Interrupt Identifier"]
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new((self.bits & 0xffff) as u16)
    }
}
