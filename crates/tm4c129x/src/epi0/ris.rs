#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Reader of field `ERRRIS`"]
pub type ERRRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RDRIS`"]
pub type RDRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRRIS`"]
pub type WRRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMARDRIS`"]
pub type DMARDRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMAWRRIS`"]
pub type DMAWRRIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn errris(&self) -> ERRRIS_R {
        ERRRIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read Raw Interrupt Status"]
    #[inline(always)]
    pub fn rdris(&self) -> RDRIS_R {
        RDRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write Raw Interrupt Status"]
    #[inline(always)]
    pub fn wrris(&self) -> WRRIS_R {
        WRRIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Read uDMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn dmardris(&self) -> DMARDRIS_R {
        DMARDRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write uDMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn dmawrris(&self) -> DMAWRRIS_R {
        DMAWRRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
