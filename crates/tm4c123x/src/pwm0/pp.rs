#[doc = "Reader of register PP"]
pub type R = crate::R<u32, super::PP>;
#[doc = "Reader of field `GCNT`"]
pub type GCNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `FCNT`"]
pub type FCNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `ESYNC`"]
pub type ESYNC_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFAULT`"]
pub type EFAULT_R = crate::R<bool, bool>;
#[doc = "Reader of field `ONE`"]
pub type ONE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - Generators"]
    #[inline(always)]
    pub fn gcnt(&self) -> GCNT_R {
        GCNT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Fault Inputs (per PWM unit)"]
    #[inline(always)]
    pub fn fcnt(&self) -> FCNT_R {
        FCNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Extended Synchronization"]
    #[inline(always)]
    pub fn esync(&self) -> ESYNC_R {
        ESYNC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Extended Fault"]
    #[inline(always)]
    pub fn efault(&self) -> EFAULT_R {
        EFAULT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - One-Shot Mode"]
    #[inline(always)]
    pub fn one(&self) -> ONE_R {
        ONE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
