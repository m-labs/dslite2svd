#[doc = "Reader of register WAITSTAT"]
pub type R = crate::R<u32, super::WAITSTAT>;
#[doc = "Reader of field `WAITREQ`"]
pub type WAITREQ_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Wait Status"]
    #[inline(always)]
    pub fn waitreq(&self) -> WAITREQ_R {
        WAITREQ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
