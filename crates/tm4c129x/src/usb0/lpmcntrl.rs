#[doc = "Reader of register LPMCNTRL"]
pub type R = crate::R<u8, super::LPMCNTRL>;
#[doc = "Reader of field `TXLPM`"]
pub type TXLPM_R = crate::R<bool, bool>;
#[doc = "Reader of field `RES`"]
pub type RES_R = crate::R<bool, bool>;
#[doc = "LPM Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EN_A {
    #[doc = "0: LPM and Extended transactions are not supported. In this case, the USB does not respond to LPM transactions and LPM transactions cause a timeout"]
    NONE = 0,
    #[doc = "1: LPM is not supported but extended transactions are supported. In this case, the USB does respond to an LPM transaction with a STALL"]
    EXT = 1,
    #[doc = "3: The USB supports LPM extended transactions. In this case, the USB responds with a NYET or an ACK as determined by the value of TXLPM and other conditions"]
    LPMEXT = 3,
}
impl From<EN_A> for u8 {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<u8, EN_A>;
impl EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EN_A::NONE),
            1 => Val(EN_A::EXT),
            3 => Val(EN_A::LPMEXT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == EN_A::NONE
    }
    #[doc = "Checks if the value of the field is `EXT`"]
    #[inline(always)]
    pub fn is_ext(&self) -> bool {
        *self == EN_A::EXT
    }
    #[doc = "Checks if the value of the field is `LPMEXT`"]
    #[inline(always)]
    pub fn is_lpmext(&self) -> bool {
        *self == EN_A::LPMEXT
    }
}
#[doc = "Reader of field `NAK`"]
pub type NAK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Transmit LPM Transaction Enable"]
    #[inline(always)]
    pub fn txlpm(&self) -> TXLPM_R {
        TXLPM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LPM Resume"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - LPM Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - LPM NAK"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
