#[doc = "Reader of register MFBOC"]
pub type R = crate::R<u32, super::MFBOC>;
#[doc = "Reader of field `MISFRMCNT`"]
pub type MISFRMCNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `MISCNTOVF`"]
pub type MISCNTOVF_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVFFRMCNT`"]
pub type OVFFRMCNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `OVFCNTOVF`"]
pub type OVFCNTOVF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - Missed Frame Counter"]
    #[inline(always)]
    pub fn misfrmcnt(&self) -> MISFRMCNT_R {
        MISFRMCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Overflow bit for Missed Frame Counter"]
    #[inline(always)]
    pub fn miscntovf(&self) -> MISCNTOVF_R {
        MISCNTOVF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:27 - Overflow Frame Counter"]
    #[inline(always)]
    pub fn ovffrmcnt(&self) -> OVFFRMCNT_R {
        OVFFRMCNT_R::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - Overflow Bit for FIFO Overflow Counter"]
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OVFCNTOVF_R {
        OVFCNTOVF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
