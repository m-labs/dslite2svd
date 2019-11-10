#[doc = "Reader of register HOSTXDESC"]
pub type R = crate::R<u32, super::HOSTXDESC>;
#[doc = "Reader of field `CURTXDESC`"]
pub type CURTXDESC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Descriptor Address Pointer"]
    #[inline(always)]
    pub fn curtxdesc(&self) -> CURTXDESC_R {
        CURTXDESC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
