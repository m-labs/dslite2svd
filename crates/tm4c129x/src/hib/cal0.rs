#[doc = "Reader of register CAL0"]
pub type R = crate::R<u32, super::CAL0>;
#[doc = "Reader of field `SEC`"]
pub type SEC_R = crate::R<u8, u8>;
#[doc = "Reader of field `MIN`"]
pub type MIN_R = crate::R<u8, u8>;
#[doc = "Reader of field `HR`"]
pub type HR_R = crate::R<u8, u8>;
#[doc = "Reader of field `AMPM`"]
pub type AMPM_R = crate::R<bool, bool>;
#[doc = "Reader of field `VALID`"]
pub type VALID_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:5 - Seconds"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Minutes"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Hours"]
    #[inline(always)]
    pub fn hr(&self) -> HR_R {
        HR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - AM/PM Designation"]
    #[inline(always)]
    pub fn ampm(&self) -> AMPM_R {
        AMPM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Valid Calendar Load"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
