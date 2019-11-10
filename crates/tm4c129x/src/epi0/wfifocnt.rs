#[doc = "Reader of register WFIFOCNT"]
pub type R = crate::R<u32, super::WFIFOCNT>;
#[doc = "Reader of field `WTAV`"]
pub type WTAV_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Available Write Transactions"]
    #[inline(always)]
    pub fn wtav(&self) -> WTAV_R {
        WTAV_R::new((self.bits & 0x07) as u8)
    }
}
