#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Reader of field `BOR1RIS`"]
pub type BOR1RIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MOFRIS`"]
pub type MOFRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLLLRIS`"]
pub type PLLLRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `USBPLLLRIS`"]
pub type USBPLLLRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MOSCPUPRIS`"]
pub type MOSCPUPRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `VDDARIS`"]
pub type VDDARIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOR0RIS`"]
pub type BOR0RIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - VDD under BOR1 Raw Interrupt Status"]
    #[inline(always)]
    pub fn bor1ris(&self) -> BOR1RIS_R {
        BOR1RIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Main Oscillator Failure Raw Interrupt Status"]
    #[inline(always)]
    pub fn mofris(&self) -> MOFRIS_R {
        MOFRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL Lock Raw Interrupt Status"]
    #[inline(always)]
    pub fn plllris(&self) -> PLLLRIS_R {
        PLLLRIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB PLL Lock Raw Interrupt Status"]
    #[inline(always)]
    pub fn usbplllris(&self) -> USBPLLLRIS_R {
        USBPLLLRIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MOSC Power Up Raw Interrupt Status"]
    #[inline(always)]
    pub fn moscpupris(&self) -> MOSCPUPRIS_R {
        MOSCPUPRIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - VDDA Power OK Event Raw Interrupt Status"]
    #[inline(always)]
    pub fn vddaris(&self) -> VDDARIS_R {
        VDDARIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - VDD under BOR0 Raw Interrupt Status"]
    #[inline(always)]
    pub fn bor0ris(&self) -> BOR0RIS_R {
        BOR0RIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
