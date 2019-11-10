#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Reader of field `RTCALT0`"]
pub type RTCALT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOWBAT`"]
pub type LOWBAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTW`"]
pub type EXTW_R = crate::R<bool, bool>;
#[doc = "Reader of field `WC`"]
pub type WC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - RTC Alert 0 Raw Interrupt Status"]
    #[inline(always)]
    pub fn rtcalt0(&self) -> RTCALT0_R {
        RTCALT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Low Battery Voltage Raw Interrupt Status"]
    #[inline(always)]
    pub fn lowbat(&self) -> LOWBAT_R {
        LOWBAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Wake-Up Raw Interrupt Status"]
    #[inline(always)]
    pub fn extw(&self) -> EXTW_R {
        EXTW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write Complete/Capable Raw Interrupt Status"]
    #[inline(always)]
    pub fn wc(&self) -> WC_R {
        WC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
