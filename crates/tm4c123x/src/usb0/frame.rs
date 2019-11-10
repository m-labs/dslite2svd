#[doc = "Reader of register FRAME"]
pub type R = crate::R<u16, super::FRAME>;
#[doc = "Reader of field `FRAME`"]
pub type FRAME_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - Frame Number"]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new((self.bits & 0x07ff) as u16)
    }
}
