#[doc = "Reader of register MBCNT"]
pub type R = crate::R<u32, super::MBCNT>;
#[doc = "Reader of field `CNTL`"]
pub type CNTL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - I2C Master Burst Count"]
    #[inline(always)]
    pub fn cntl(&self) -> CNTL_R {
        CNTL_R::new((self.bits & 0xff) as u8)
    }
}
