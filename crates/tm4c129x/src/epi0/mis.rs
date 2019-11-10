#[doc = "Reader of register MIS"]
pub type R = crate::R<u32, super::MIS>;
#[doc = "Reader of field `ERRMIS`"]
pub type ERRMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RDMIS`"]
pub type RDMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRMIS`"]
pub type WRMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMARDMIS`"]
pub type DMARDMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMAWRMIS`"]
pub type DMAWRMIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn errmis(&self) -> ERRMIS_R {
        ERRMIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read Masked Interrupt Status"]
    #[inline(always)]
    pub fn rdmis(&self) -> RDMIS_R {
        RDMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write Masked Interrupt Status"]
    #[inline(always)]
    pub fn wrmis(&self) -> WRMIS_R {
        WRMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Read uDMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn dmardmis(&self) -> DMARDMIS_R {
        DMARDMIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write uDMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn dmawrmis(&self) -> DMAWRMIS_R {
        DMAWRMIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
