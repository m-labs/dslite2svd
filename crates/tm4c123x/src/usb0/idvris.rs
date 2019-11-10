#[doc = "Reader of register IDVRIS"]
pub type R = crate::R<u32, super::IDVRIS>;
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ID Valid Detect Raw Interrupt Status"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x01) != 0)
    }
}
