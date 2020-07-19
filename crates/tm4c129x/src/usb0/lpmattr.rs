#[doc = "Reader of register LPMATTR"]
pub type R = crate::R<u16, super::LPMATTR>;
#[doc = "Link State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LS_A {
    #[doc = "1: Sleep State (L1)"]
    L1 = 1,
}
impl From<LS_A> for u8 {
    #[inline(always)]
    fn from(variant: LS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LS`"]
pub type LS_R = crate::R<u8, LS_A>;
impl LS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LS_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(LS_A::L1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        *self == LS_A::L1
    }
}
#[doc = "Reader of field `HIRD`"]
pub type HIRD_R = crate::R<u8, u8>;
#[doc = "Reader of field `RMTWAK`"]
pub type RMTWAK_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENDPT`"]
pub type ENDPT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Link State"]
    #[inline(always)]
    pub fn ls(&self) -> LS_R {
        LS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Host Initiated Resume Duration"]
    #[inline(always)]
    pub fn hird(&self) -> HIRD_R {
        HIRD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Remote Wake"]
    #[inline(always)]
    pub fn rmtwak(&self) -> RMTWAK_R {
        RMTWAK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - Endpoint"]
    #[inline(always)]
    pub fn endpt(&self) -> ENDPT_R {
        ENDPT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
