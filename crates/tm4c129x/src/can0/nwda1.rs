#[doc = "Reader of register NWDA1"]
pub type R = crate::R<u32, super::NWDA1>;
#[doc = "Reader of field `NEWDAT`"]
pub type NEWDAT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - New Data Bits"]
    #[inline(always)]
    pub fn newdat(&self) -> NEWDAT_R {
        NEWDAT_R::new((self.bits & 0xffff) as u16)
    }
}
