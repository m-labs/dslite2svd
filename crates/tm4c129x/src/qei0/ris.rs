#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Reader of field `INDEX`"]
pub type INDEX_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIMER`"]
pub type TIMER_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERROR`"]
pub type ERROR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Index Pulse Asserted"]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Velocity Timer Expired"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Direction Change Detected"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Phase Error Detected"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
