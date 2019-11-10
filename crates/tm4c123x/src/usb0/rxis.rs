#[doc = "Reader of register RXIS"]
pub type R = crate::R<u16, super::RXIS>;
#[doc = "Reader of field `EP1`"]
pub type EP1_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP2`"]
pub type EP2_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP3`"]
pub type EP3_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP4`"]
pub type EP4_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP5`"]
pub type EP5_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP6`"]
pub type EP6_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP7`"]
pub type EP7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - RX Endpoint 1 Interrupt"]
    #[inline(always)]
    pub fn ep1(&self) -> EP1_R {
        EP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX Endpoint 2 Interrupt"]
    #[inline(always)]
    pub fn ep2(&self) -> EP2_R {
        EP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RX Endpoint 3 Interrupt"]
    #[inline(always)]
    pub fn ep3(&self) -> EP3_R {
        EP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RX Endpoint 4 Interrupt"]
    #[inline(always)]
    pub fn ep4(&self) -> EP4_R {
        EP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RX Endpoint 5 Interrupt"]
    #[inline(always)]
    pub fn ep5(&self) -> EP5_R {
        EP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RX Endpoint 6 Interrupt"]
    #[inline(always)]
    pub fn ep6(&self) -> EP6_R {
        EP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RX Endpoint 7 Interrupt"]
    #[inline(always)]
    pub fn ep7(&self) -> EP7_R {
        EP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
