#[doc = "Reader of register FIFOSTATUS"]
pub type R = crate::R<u32, super::FIFOSTATUS>;
#[doc = "Reader of field `TXFE`"]
pub type TXFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFF`"]
pub type TXFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXBLWTRIG`"]
pub type TXBLWTRIG_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFE`"]
pub type RXFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFF`"]
pub type RXFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXABVTRIG`"]
pub type RXABVTRIG_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TX FIFO Empty"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Full"]
    #[inline(always)]
    pub fn txff(&self) -> TXFF_R {
        TXFF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TX FIFO Below Trigger Level"]
    #[inline(always)]
    pub fn txblwtrig(&self) -> TXBLWTRIG_R {
        TXBLWTRIG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RX FIFO Empty"]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RX FIFO Full"]
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - RX FIFO Above Trigger Level"]
    #[inline(always)]
    pub fn rxabvtrig(&self) -> RXABVTRIG_R {
        RXABVTRIG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
