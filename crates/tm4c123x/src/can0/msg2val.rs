#[doc = "Reader of register MSG2VAL"]
pub type R = crate::R<u32, super::MSG2VAL>;
#[doc = "Reader of field `MSGVAL`"]
pub type MSGVAL_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Message Valid Bits"]
    #[inline(always)]
    pub fn msgval(&self) -> MSGVAL_R {
        MSGVAL_R::new((self.bits & 0xffff) as u16)
    }
}
