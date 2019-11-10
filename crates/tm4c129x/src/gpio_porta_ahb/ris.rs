#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Reader of field `GPIO`"]
pub type GPIO_R = crate::R<u8, u8>;
#[doc = "Reader of field `DMARIS`"]
pub type DMARIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:7 - GPIO Interrupt Raw Status"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - GPIO uDMA Done Interrupt Raw Status"]
    #[inline(always)]
    pub fn dmaris(&self) -> DMARIS_R {
        DMARIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
