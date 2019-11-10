#[doc = "Reader of register FCRIS"]
pub type R = crate::R<u32, super::FCRIS>;
#[doc = "Reader of field `ARIS`"]
pub type ARIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PRIS`"]
pub type PRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERIS`"]
pub type ERIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `VOLTRIS`"]
pub type VOLTRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `INVDRIS`"]
pub type INVDRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERRIS`"]
pub type ERRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PROGRIS`"]
pub type PROGRIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Access Raw Interrupt Status"]
    #[inline(always)]
    pub fn aris(&self) -> ARIS_R {
        ARIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Programming Raw Interrupt Status"]
    #[inline(always)]
    pub fn pris(&self) -> PRIS_R {
        PRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EEPROM Raw Interrupt Status"]
    #[inline(always)]
    pub fn eris(&self) -> ERIS_R {
        ERIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pump Voltage Raw Interrupt Status"]
    #[inline(always)]
    pub fn voltris(&self) -> VOLTRIS_R {
        VOLTRIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Invalid Data Raw Interrupt Status"]
    #[inline(always)]
    pub fn invdris(&self) -> INVDRIS_R {
        INVDRIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Erase Verify Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn erris(&self) -> ERRIS_R {
        ERRIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Program Verify Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn progris(&self) -> PROGRIS_R {
        PROGRIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
