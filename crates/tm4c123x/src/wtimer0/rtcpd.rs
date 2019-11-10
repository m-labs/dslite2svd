#[doc = "Reader of register RTCPD"]
pub type R = crate::R<u32, super::RTCPD>;
#[doc = "Reader of field `RTCPD`"]
pub type RTCPD_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC Predivide Counter Value"]
    #[inline(always)]
    pub fn rtcpd(&self) -> RTCPD_R {
        RTCPD_R::new((self.bits & 0xffff) as u16)
    }
}
