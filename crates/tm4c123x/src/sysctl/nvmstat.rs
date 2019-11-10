#[doc = "Reader of register NVMSTAT"]
pub type R = crate::R<u32, super::NVMSTAT>;
#[doc = "Reader of field `FWB`"]
pub type FWB_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - 32 Word Flash Write Buffer Available"]
    #[inline(always)]
    pub fn fwb(&self) -> FWB_R {
        FWB_R::new((self.bits & 0x01) != 0)
    }
}
