#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Reader of field `FPIDCRIS`"]
pub type FPIDCRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `FPDZCRIS`"]
pub type FPDZCRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `FPIOCRIS`"]
pub type FPIOCRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `FPUFCRIS`"]
pub type FPUFCRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `FPOFCRIS`"]
pub type FPOFCRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `FPIXCRIS`"]
pub type FPIXCRIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn fpidcris(&self) -> FPIDCRIS_R {
        FPIDCRIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn fpdzcris(&self) -> FPDZCRIS_R {
        FPDZCRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Raw Interrupt Status"]
    #[inline(always)]
    pub fn fpiocris(&self) -> FPIOCRIS_R {
        FPIOCRIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn fpufcris(&self) -> FPUFCRIS_R {
        FPUFCRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn fpofcris(&self) -> FPOFCRIS_R {
        FPOFCRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn fpixcris(&self) -> FPIXCRIS_R {
        FPIXCRIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
