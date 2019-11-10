#[doc = "Reader of register HOSRXDESC"]
pub type R = crate::R<u32, super::HOSRXDESC>;
#[doc = "Reader of field `CURRXDESC`"]
pub type CURRXDESC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub fn currxdesc(&self) -> CURRXDESC_R {
        CURRXDESC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
