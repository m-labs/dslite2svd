#[doc = "Reader of register MIS"]
pub type R = crate::R<u32, super::MIS>;
#[doc = "Reader of field `FPIDCMIS`"]
pub type FPIDCMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `FPDZCMIS`"]
pub type FPDZCMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `FPIOCMIS`"]
pub type FPIOCMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `FPUFCMIS`"]
pub type FPUFCMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `FPOFCMIS`"]
pub type FPOFCMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `FPIXCMIS`"]
pub type FPIXCMIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn fpidcmis(&self) -> FPIDCMIS_R {
        FPIDCMIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn fpdzcmis(&self) -> FPDZCMIS_R {
        FPDZCMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Masked Interrupt Status"]
    #[inline(always)]
    pub fn fpiocmis(&self) -> FPIOCMIS_R {
        FPIOCMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn fpufcmis(&self) -> FPUFCMIS_R {
        FPUFCMIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn fpofcmis(&self) -> FPOFCMIS_R {
        FPOFCMIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn fpixcmis(&self) -> FPIXCMIS_R {
        FPIXCMIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
