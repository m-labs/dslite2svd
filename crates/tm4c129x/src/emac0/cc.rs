#[doc = "Reader of register CC"]
pub type R = crate::R<u32, super::CC>;
#[doc = "Reader of field `POL`"]
pub type POL_R = crate::R<bool, bool>;
#[doc = "Reader of field `PTPCEN`"]
pub type PTPCEN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 17 - LED Polarity Control"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PTP Clock Reference Enable"]
    #[inline(always)]
    pub fn ptpcen(&self) -> PTPCEN_R {
        PTPCEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
