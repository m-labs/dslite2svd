#[doc = "Reader of register EESIZE"]
pub type R = crate::R<u32, super::EESIZE>;
#[doc = "Reader of field `WORDCNT`"]
pub type WORDCNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `BLKCNT`"]
pub type BLKCNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of 32-Bit Words"]
    #[inline(always)]
    pub fn wordcnt(&self) -> WORDCNT_R {
        WORDCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:26 - Number of 16-Word Blocks"]
    #[inline(always)]
    pub fn blkcnt(&self) -> BLKCNT_R {
        BLKCNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
