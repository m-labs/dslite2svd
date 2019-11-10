#[doc = "Reader of register PP"]
pub type R = crate::R<u32, super::PP>;
#[doc = "Reader of field `SC`"]
pub type SC_R = crate::R<bool, bool>;
#[doc = "Reader of field `NB`"]
pub type NB_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Smart Card Support"]
    #[inline(always)]
    pub fn sc(&self) -> SC_R {
        SC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 9-Bit Support"]
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
