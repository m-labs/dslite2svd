#[doc = "Reader of register EEDONE"]
pub type R = crate::R<u32, super::EEDONE>;
#[doc = "Reader of field `WORKING`"]
pub type WORKING_R = crate::R<bool, bool>;
#[doc = "Reader of field `WKERASE`"]
pub type WKERASE_R = crate::R<bool, bool>;
#[doc = "Reader of field `WKCOPY`"]
pub type WKCOPY_R = crate::R<bool, bool>;
#[doc = "Reader of field `NOPERM`"]
pub type NOPERM_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRBUSY`"]
pub type WRBUSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - EEPROM Working"]
    #[inline(always)]
    pub fn working(&self) -> WORKING_R {
        WORKING_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Working on an Erase"]
    #[inline(always)]
    pub fn wkerase(&self) -> WKERASE_R {
        WKERASE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Working on a Copy"]
    #[inline(always)]
    pub fn wkcopy(&self) -> WKCOPY_R {
        WKCOPY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write Without Permission"]
    #[inline(always)]
    pub fn noperm(&self) -> NOPERM_R {
        NOPERM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write Busy"]
    #[inline(always)]
    pub fn wrbusy(&self) -> WRBUSY_R {
        WRBUSY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
