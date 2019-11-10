#[doc = "Reader of register MIS"]
pub type R = crate::R<u32, super::MIS>;
#[doc = "Reader of field `RTCALT0`"]
pub type RTCALT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOWBAT`"]
pub type LOWBAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTW`"]
pub type EXTW_R = crate::R<bool, bool>;
#[doc = "Reader of field `WC`"]
pub type WC_R = crate::R<bool, bool>;
#[doc = "Reader of field `PADIOWK`"]
pub type PADIOWK_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSTWK`"]
pub type RSTWK_R = crate::R<bool, bool>;
#[doc = "Reader of field `VDDFAIL`"]
pub type VDDFAIL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - RTC Alert 0 Masked Interrupt Status"]
    #[inline(always)]
    pub fn rtcalt0(&self) -> RTCALT0_R {
        RTCALT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Low Battery Voltage Masked Interrupt Status"]
    #[inline(always)]
    pub fn lowbat(&self) -> LOWBAT_R {
        LOWBAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Wake-Up Masked Interrupt Status"]
    #[inline(always)]
    pub fn extw(&self) -> EXTW_R {
        EXTW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write Complete/Capable Masked Interrupt Status"]
    #[inline(always)]
    pub fn wc(&self) -> WC_R {
        WC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pad I/O Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn padiowk(&self) -> PADIOWK_R {
        PADIOWK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Reset Pad I/O Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn rstwk(&self) -> RSTWK_R {
        RSTWK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - VDD Fail Interrupt Mask"]
    #[inline(always)]
    pub fn vddfail(&self) -> VDDFAIL_R {
        VDDFAIL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
