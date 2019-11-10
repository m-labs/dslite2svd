#[doc = "Reader of register MIS"]
pub type R = crate::R<u32, super::MIS>;
#[doc = "Reader of field `GPIO`"]
pub type GPIO_R = crate::R<u8, u8>;
#[doc = "Reader of field `DMAMIS`"]
pub type DMAMIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:7 - GPIO Masked Interrupt Status"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - GPIO uDMA Done Masked Interrupt Status"]
    #[inline(always)]
    pub fn dmamis(&self) -> DMAMIS_R {
        DMAMIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
