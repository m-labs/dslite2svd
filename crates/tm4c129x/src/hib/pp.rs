#[doc = "Reader of register PP"]
pub type R = crate::R<u32, super::PP>;
#[doc = "Reader of field `WAKENC`"]
pub type WAKENC_R = crate::R<bool, bool>;
#[doc = "Reader of field `TAMPER`"]
pub type TAMPER_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Wake Pin Presence"]
    #[inline(always)]
    pub fn wakenc(&self) -> WAKENC_R {
        WAKENC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Tamper Pin Presence"]
    #[inline(always)]
    pub fn tamper(&self) -> TAMPER_R {
        TAMPER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
