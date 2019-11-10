#[doc = "Reader of register PC"]
pub type R = crate::R<u32, super::PC>;
#[doc = "Writer for register PC"]
pub type W = crate::W<u32, super::PC>;
#[doc = "Register PC `reset()`'s with value 0"]
impl crate::ResetValue for super::PC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PHYHOLD`"]
pub type PHYHOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHYHOLD`"]
pub struct PHYHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYHOLD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Auto Negotiation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANMODE_A {
    #[doc = "0: When ANEN = 0x0, the mode is 10Base-T, Half-Duplex"]
    _10HD,
    #[doc = "1: When ANEN = 0x0, the mode is 10Base-T, Full-Duplex"]
    _10FD,
    #[doc = "2: When ANEN = 0x0, the mode is 100Base-TX, Half-Duplex"]
    _100HD,
    #[doc = "3: When ANEN = 0x0, the mode is 100Base-TX, Full-Duplex"]
    _100FD,
}
impl From<ANMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ANMODE_A) -> Self {
        match variant {
            ANMODE_A::_10HD => 0,
            ANMODE_A::_10FD => 1,
            ANMODE_A::_100HD => 2,
            ANMODE_A::_100FD => 3,
        }
    }
}
#[doc = "Reader of field `ANMODE`"]
pub type ANMODE_R = crate::R<u8, ANMODE_A>;
impl ANMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANMODE_A {
        match self.bits {
            0 => ANMODE_A::_10HD,
            1 => ANMODE_A::_10FD,
            2 => ANMODE_A::_100HD,
            3 => ANMODE_A::_100FD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_10HD`"]
    #[inline(always)]
    pub fn is_10hd(&self) -> bool {
        *self == ANMODE_A::_10HD
    }
    #[doc = "Checks if the value of the field is `_10FD`"]
    #[inline(always)]
    pub fn is_10fd(&self) -> bool {
        *self == ANMODE_A::_10FD
    }
    #[doc = "Checks if the value of the field is `_100HD`"]
    #[inline(always)]
    pub fn is_100hd(&self) -> bool {
        *self == ANMODE_A::_100HD
    }
    #[doc = "Checks if the value of the field is `_100FD`"]
    #[inline(always)]
    pub fn is_100fd(&self) -> bool {
        *self == ANMODE_A::_100FD
    }
}
#[doc = "Write proxy for field `ANMODE`"]
pub struct ANMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ANMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "When ANEN = 0x0, the mode is 10Base-T, Half-Duplex"]
    #[inline(always)]
    pub fn _10hd(self) -> &'a mut W {
        self.variant(ANMODE_A::_10HD)
    }
    #[doc = "When ANEN = 0x0, the mode is 10Base-T, Full-Duplex"]
    #[inline(always)]
    pub fn _10fd(self) -> &'a mut W {
        self.variant(ANMODE_A::_10FD)
    }
    #[doc = "When ANEN = 0x0, the mode is 100Base-TX, Half-Duplex"]
    #[inline(always)]
    pub fn _100hd(self) -> &'a mut W {
        self.variant(ANMODE_A::_100HD)
    }
    #[doc = "When ANEN = 0x0, the mode is 100Base-TX, Full-Duplex"]
    #[inline(always)]
    pub fn _100fd(self) -> &'a mut W {
        self.variant(ANMODE_A::_100FD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `ANEN`"]
pub type ANEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANEN`"]
pub struct ANEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ANEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `FASTANSEL`"]
pub type FASTANSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FASTANSEL`"]
pub struct FASTANSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTANSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `FASTANEN`"]
pub type FASTANEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FASTANEN`"]
pub struct FASTANEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTANEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `EXTFD`"]
pub type EXTFD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTFD`"]
pub struct EXTFD_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTFD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `FASTLUPD`"]
pub type FASTLUPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FASTLUPD`"]
pub struct FASTLUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTLUPD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `FASTRXDV`"]
pub type FASTRXDV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FASTRXDV`"]
pub struct FASTRXDV_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTRXDV_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `MDIXEN`"]
pub type MDIXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MDIXEN`"]
pub struct MDIXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIXEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `FASTMDIX`"]
pub type FASTMDIX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FASTMDIX`"]
pub struct FASTMDIX_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTMDIX_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `RBSTMDIX`"]
pub type RBSTMDIX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RBSTMDIX`"]
pub struct RBSTMDIX_W<'a> {
    w: &'a mut W,
}
impl<'a> RBSTMDIX_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `MDISWAP`"]
pub type MDISWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MDISWAP`"]
pub struct MDISWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> MDISWAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `POLSWAP`"]
pub type POLSWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POLSWAP`"]
pub struct POLSWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> POLSWAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `FASTLDMODE`"]
pub type FASTLDMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FASTLDMODE`"]
pub struct FASTLDMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTLDMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = "Reader of field `TDRRUN`"]
pub type TDRRUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TDRRUN`"]
pub struct TDRRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> TDRRUN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `LRR`"]
pub type LRR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LRR`"]
pub struct LRR_W<'a> {
    w: &'a mut W,
}
impl<'a> LRR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `ISOMIILL`"]
pub type ISOMIILL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISOMIILL`"]
pub struct ISOMIILL_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOMIILL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `RXERIDLE`"]
pub type RXERIDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXERIDLE`"]
pub struct RXERIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXERIDLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `NIBDETDIS`"]
pub type NIBDETDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NIBDETDIS`"]
pub struct NIBDETDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NIBDETDIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `DIGRESTART`"]
pub type DIGRESTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIGRESTART`"]
pub struct DIGRESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> DIGRESTART_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Ethernet Interface Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINTFS_A {
    #[doc = "0: MII (default) Used for internal PHY or external PHY connected via MII"]
    IMII,
    #[doc = "4: RMII: Used for external PHY connected via RMII"]
    RMII,
}
impl From<PINTFS_A> for u8 {
    #[inline(always)]
    fn from(variant: PINTFS_A) -> Self {
        match variant {
            PINTFS_A::IMII => 0,
            PINTFS_A::RMII => 4,
        }
    }
}
#[doc = "Reader of field `PINTFS`"]
pub type PINTFS_R = crate::R<u8, PINTFS_A>;
impl PINTFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PINTFS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PINTFS_A::IMII),
            4 => Val(PINTFS_A::RMII),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMII`"]
    #[inline(always)]
    pub fn is_imii(&self) -> bool {
        *self == PINTFS_A::IMII
    }
    #[doc = "Checks if the value of the field is `RMII`"]
    #[inline(always)]
    pub fn is_rmii(&self) -> bool {
        *self == PINTFS_A::RMII
    }
}
#[doc = "Write proxy for field `PINTFS`"]
pub struct PINTFS_W<'a> {
    w: &'a mut W,
}
impl<'a> PINTFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINTFS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "MII (default) Used for internal PHY or external PHY connected via MII"]
    #[inline(always)]
    pub fn imii(self) -> &'a mut W {
        self.variant(PINTFS_A::IMII)
    }
    #[doc = "RMII: Used for external PHY connected via RMII"]
    #[inline(always)]
    pub fn rmii(self) -> &'a mut W {
        self.variant(PINTFS_A::RMII)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `PHYEXT`"]
pub type PHYEXT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHYEXT`"]
pub struct PHYEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYEXT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Ethernet PHY Hold"]
    #[inline(always)]
    pub fn phyhold(&self) -> PHYHOLD_R {
        PHYHOLD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Auto Negotiation Mode"]
    #[inline(always)]
    pub fn anmode(&self) -> ANMODE_R {
        ANMODE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Auto Negotiation Enable"]
    #[inline(always)]
    pub fn anen(&self) -> ANEN_R {
        ANEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Fast Auto Negotiation Select"]
    #[inline(always)]
    pub fn fastansel(&self) -> FASTANSEL_R {
        FASTANSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Fast Auto Negotiation Enable"]
    #[inline(always)]
    pub fn fastanen(&self) -> FASTANEN_R {
        FASTANEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Extended Full Duplex Ability"]
    #[inline(always)]
    pub fn extfd(&self) -> EXTFD_R {
        EXTFD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FAST Link-Up in Parallel Detect"]
    #[inline(always)]
    pub fn fastlupd(&self) -> FASTLUPD_R {
        FASTLUPD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Fast RXDV Detection"]
    #[inline(always)]
    pub fn fastrxdv(&self) -> FASTRXDV_R {
        FASTRXDV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MDIX Enable"]
    #[inline(always)]
    pub fn mdixen(&self) -> MDIXEN_R {
        MDIXEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Fast Auto MDI-X"]
    #[inline(always)]
    pub fn fastmdix(&self) -> FASTMDIX_R {
        FASTMDIX_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Robust Auto MDI-X"]
    #[inline(always)]
    pub fn rbstmdix(&self) -> RBSTMDIX_R {
        RBSTMDIX_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MDI Swap"]
    #[inline(always)]
    pub fn mdiswap(&self) -> MDISWAP_R {
        MDISWAP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Polarity Swap"]
    #[inline(always)]
    pub fn polswap(&self) -> POLSWAP_R {
        POLSWAP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 15:19 - Fast Link Down Mode"]
    #[inline(always)]
    pub fn fastldmode(&self) -> FASTLDMODE_R {
        FASTLDMODE_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bit 20 - TDR Auto Run"]
    #[inline(always)]
    pub fn tdrrun(&self) -> TDRRUN_R {
        TDRRUN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Link Loss Recovery"]
    #[inline(always)]
    pub fn lrr(&self) -> LRR_R {
        LRR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Isolate MII in Link Loss"]
    #[inline(always)]
    pub fn isomiill(&self) -> ISOMIILL_R {
        ISOMIILL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - RXER Detection During Idle"]
    #[inline(always)]
    pub fn rxeridle(&self) -> RXERIDLE_R {
        RXERIDLE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Odd Nibble TXER Detection Disable"]
    #[inline(always)]
    pub fn nibdetdis(&self) -> NIBDETDIS_R {
        NIBDETDIS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - PHY Soft Restart"]
    #[inline(always)]
    pub fn digrestart(&self) -> DIGRESTART_R {
        DIGRESTART_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - Ethernet Interface Select"]
    #[inline(always)]
    pub fn pintfs(&self) -> PINTFS_R {
        PINTFS_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 31 - PHY Select"]
    #[inline(always)]
    pub fn phyext(&self) -> PHYEXT_R {
        PHYEXT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ethernet PHY Hold"]
    #[inline(always)]
    pub fn phyhold(&mut self) -> PHYHOLD_W {
        PHYHOLD_W { w: self }
    }
    #[doc = "Bits 1:2 - Auto Negotiation Mode"]
    #[inline(always)]
    pub fn anmode(&mut self) -> ANMODE_W {
        ANMODE_W { w: self }
    }
    #[doc = "Bit 3 - Auto Negotiation Enable"]
    #[inline(always)]
    pub fn anen(&mut self) -> ANEN_W {
        ANEN_W { w: self }
    }
    #[doc = "Bits 4:5 - Fast Auto Negotiation Select"]
    #[inline(always)]
    pub fn fastansel(&mut self) -> FASTANSEL_W {
        FASTANSEL_W { w: self }
    }
    #[doc = "Bit 6 - Fast Auto Negotiation Enable"]
    #[inline(always)]
    pub fn fastanen(&mut self) -> FASTANEN_W {
        FASTANEN_W { w: self }
    }
    #[doc = "Bit 7 - Extended Full Duplex Ability"]
    #[inline(always)]
    pub fn extfd(&mut self) -> EXTFD_W {
        EXTFD_W { w: self }
    }
    #[doc = "Bit 8 - FAST Link-Up in Parallel Detect"]
    #[inline(always)]
    pub fn fastlupd(&mut self) -> FASTLUPD_W {
        FASTLUPD_W { w: self }
    }
    #[doc = "Bit 9 - Fast RXDV Detection"]
    #[inline(always)]
    pub fn fastrxdv(&mut self) -> FASTRXDV_W {
        FASTRXDV_W { w: self }
    }
    #[doc = "Bit 10 - MDIX Enable"]
    #[inline(always)]
    pub fn mdixen(&mut self) -> MDIXEN_W {
        MDIXEN_W { w: self }
    }
    #[doc = "Bit 11 - Fast Auto MDI-X"]
    #[inline(always)]
    pub fn fastmdix(&mut self) -> FASTMDIX_W {
        FASTMDIX_W { w: self }
    }
    #[doc = "Bit 12 - Robust Auto MDI-X"]
    #[inline(always)]
    pub fn rbstmdix(&mut self) -> RBSTMDIX_W {
        RBSTMDIX_W { w: self }
    }
    #[doc = "Bit 13 - MDI Swap"]
    #[inline(always)]
    pub fn mdiswap(&mut self) -> MDISWAP_W {
        MDISWAP_W { w: self }
    }
    #[doc = "Bit 14 - Polarity Swap"]
    #[inline(always)]
    pub fn polswap(&mut self) -> POLSWAP_W {
        POLSWAP_W { w: self }
    }
    #[doc = "Bits 15:19 - Fast Link Down Mode"]
    #[inline(always)]
    pub fn fastldmode(&mut self) -> FASTLDMODE_W {
        FASTLDMODE_W { w: self }
    }
    #[doc = "Bit 20 - TDR Auto Run"]
    #[inline(always)]
    pub fn tdrrun(&mut self) -> TDRRUN_W {
        TDRRUN_W { w: self }
    }
    #[doc = "Bit 21 - Link Loss Recovery"]
    #[inline(always)]
    pub fn lrr(&mut self) -> LRR_W {
        LRR_W { w: self }
    }
    #[doc = "Bit 22 - Isolate MII in Link Loss"]
    #[inline(always)]
    pub fn isomiill(&mut self) -> ISOMIILL_W {
        ISOMIILL_W { w: self }
    }
    #[doc = "Bit 23 - RXER Detection During Idle"]
    #[inline(always)]
    pub fn rxeridle(&mut self) -> RXERIDLE_W {
        RXERIDLE_W { w: self }
    }
    #[doc = "Bit 24 - Odd Nibble TXER Detection Disable"]
    #[inline(always)]
    pub fn nibdetdis(&mut self) -> NIBDETDIS_W {
        NIBDETDIS_W { w: self }
    }
    #[doc = "Bit 25 - PHY Soft Restart"]
    #[inline(always)]
    pub fn digrestart(&mut self) -> DIGRESTART_W {
        DIGRESTART_W { w: self }
    }
    #[doc = "Bits 28:30 - Ethernet Interface Select"]
    #[inline(always)]
    pub fn pintfs(&mut self) -> PINTFS_W {
        PINTFS_W { w: self }
    }
    #[doc = "Bit 31 - PHY Select"]
    #[inline(always)]
    pub fn phyext(&mut self) -> PHYEXT_W {
        PHYEXT_W { w: self }
    }
}
