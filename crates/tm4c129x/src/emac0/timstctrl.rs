#[doc = "Reader of register TIMSTCTRL"]
pub type R = crate::R<u32, super::TIMSTCTRL>;
#[doc = "Writer for register TIMSTCTRL"]
pub type W = crate::W<u32, super::TIMSTCTRL>;
#[doc = "Register TIMSTCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMSTCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSEN`"]
pub type TSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSEN`"]
pub struct TSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEN_W<'a> {
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
#[doc = "Reader of field `TSFCUPDT`"]
pub type TSFCUPDT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSFCUPDT`"]
pub struct TSFCUPDT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSFCUPDT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TSINIT`"]
pub type TSINIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSINIT`"]
pub struct TSINIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSINIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TSUPDT`"]
pub type TSUPDT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSUPDT`"]
pub struct TSUPDT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSUPDT_W<'a> {
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
#[doc = "Reader of field `INTTRIG`"]
pub type INTTRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTTRIG`"]
pub struct INTTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> INTTRIG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ADDREGUP`"]
pub type ADDREGUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDREGUP`"]
pub struct ADDREGUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDREGUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `ALLF`"]
pub type ALLF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALLF`"]
pub struct ALLF_W<'a> {
    w: &'a mut W,
}
impl<'a> ALLF_W<'a> {
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
#[doc = "Reader of field `DGTLBIN`"]
pub type DGTLBIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DGTLBIN`"]
pub struct DGTLBIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DGTLBIN_W<'a> {
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
#[doc = "Reader of field `PTPVER2`"]
pub type PTPVER2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTPVER2`"]
pub struct PTPVER2_W<'a> {
    w: &'a mut W,
}
impl<'a> PTPVER2_W<'a> {
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
#[doc = "Reader of field `PTPETH`"]
pub type PTPETH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTPETH`"]
pub struct PTPETH_W<'a> {
    w: &'a mut W,
}
impl<'a> PTPETH_W<'a> {
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
#[doc = "Reader of field `PTPIPV6`"]
pub type PTPIPV6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTPIPV6`"]
pub struct PTPIPV6_W<'a> {
    w: &'a mut W,
}
impl<'a> PTPIPV6_W<'a> {
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
#[doc = "Reader of field `PTPIPV4`"]
pub type PTPIPV4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTPIPV4`"]
pub struct PTPIPV4_W<'a> {
    w: &'a mut W,
}
impl<'a> PTPIPV4_W<'a> {
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
#[doc = "Reader of field `TSEVNT`"]
pub type TSEVNT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSEVNT`"]
pub struct TSEVNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEVNT_W<'a> {
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
#[doc = "Reader of field `TSMAST`"]
pub type TSMAST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSMAST`"]
pub struct TSMAST_W<'a> {
    w: &'a mut W,
}
impl<'a> TSMAST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `SELPTP`"]
pub type SELPTP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SELPTP`"]
pub struct SELPTP_W<'a> {
    w: &'a mut W,
}
impl<'a> SELPTP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `PTPFLTR`"]
pub type PTPFLTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTPFLTR`"]
pub struct PTPFLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PTPFLTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Timestamp Enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timestamp Fine or Coarse Update"]
    #[inline(always)]
    pub fn tsfcupdt(&self) -> TSFCUPDT_R {
        TSFCUPDT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timestamp Initialize"]
    #[inline(always)]
    pub fn tsinit(&self) -> TSINIT_R {
        TSINIT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timestamp Update"]
    #[inline(always)]
    pub fn tsupdt(&self) -> TSUPDT_R {
        TSUPDT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timestamp Interrupt Trigger Enable"]
    #[inline(always)]
    pub fn inttrig(&self) -> INTTRIG_R {
        INTTRIG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Addend Register Update"]
    #[inline(always)]
    pub fn addregup(&self) -> ADDREGUP_R {
        ADDREGUP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable Timestamp For All Frames"]
    #[inline(always)]
    pub fn allf(&self) -> ALLF_R {
        ALLF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control"]
    #[inline(always)]
    pub fn dgtlbin(&self) -> DGTLBIN_R {
        DGTLBIN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable PTP Packet Processing For Version 2 Format"]
    #[inline(always)]
    pub fn ptpver2(&self) -> PTPVER2_R {
        PTPVER2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable Processing of PTP Over Ethernet Frames"]
    #[inline(always)]
    pub fn ptpeth(&self) -> PTPETH_R {
        PTPETH_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable Processing of PTP Frames Sent Over IPv6-UDP"]
    #[inline(always)]
    pub fn ptpipv6(&self) -> PTPIPV6_R {
        PTPIPV6_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable Processing of PTP Frames Sent over IPv4-UDP"]
    #[inline(always)]
    pub fn ptpipv4(&self) -> PTPIPV4_R {
        PTPIPV4_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages"]
    #[inline(always)]
    pub fn tsevnt(&self) -> TSEVNT_R {
        TSEVNT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master"]
    #[inline(always)]
    pub fn tsmast(&self) -> TSMAST_R {
        TSMAST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots"]
    #[inline(always)]
    pub fn selptp(&self) -> SELPTP_R {
        SELPTP_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Enable MAC address for PTP Frame Filtering"]
    #[inline(always)]
    pub fn ptpfltr(&self) -> PTPFLTR_R {
        PTPFLTR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timestamp Enable"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W {
        TSEN_W { w: self }
    }
    #[doc = "Bit 1 - Timestamp Fine or Coarse Update"]
    #[inline(always)]
    pub fn tsfcupdt(&mut self) -> TSFCUPDT_W {
        TSFCUPDT_W { w: self }
    }
    #[doc = "Bit 2 - Timestamp Initialize"]
    #[inline(always)]
    pub fn tsinit(&mut self) -> TSINIT_W {
        TSINIT_W { w: self }
    }
    #[doc = "Bit 3 - Timestamp Update"]
    #[inline(always)]
    pub fn tsupdt(&mut self) -> TSUPDT_W {
        TSUPDT_W { w: self }
    }
    #[doc = "Bit 4 - Timestamp Interrupt Trigger Enable"]
    #[inline(always)]
    pub fn inttrig(&mut self) -> INTTRIG_W {
        INTTRIG_W { w: self }
    }
    #[doc = "Bit 5 - Addend Register Update"]
    #[inline(always)]
    pub fn addregup(&mut self) -> ADDREGUP_W {
        ADDREGUP_W { w: self }
    }
    #[doc = "Bit 8 - Enable Timestamp For All Frames"]
    #[inline(always)]
    pub fn allf(&mut self) -> ALLF_W {
        ALLF_W { w: self }
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control"]
    #[inline(always)]
    pub fn dgtlbin(&mut self) -> DGTLBIN_W {
        DGTLBIN_W { w: self }
    }
    #[doc = "Bit 10 - Enable PTP Packet Processing For Version 2 Format"]
    #[inline(always)]
    pub fn ptpver2(&mut self) -> PTPVER2_W {
        PTPVER2_W { w: self }
    }
    #[doc = "Bit 11 - Enable Processing of PTP Over Ethernet Frames"]
    #[inline(always)]
    pub fn ptpeth(&mut self) -> PTPETH_W {
        PTPETH_W { w: self }
    }
    #[doc = "Bit 12 - Enable Processing of PTP Frames Sent Over IPv6-UDP"]
    #[inline(always)]
    pub fn ptpipv6(&mut self) -> PTPIPV6_W {
        PTPIPV6_W { w: self }
    }
    #[doc = "Bit 13 - Enable Processing of PTP Frames Sent over IPv4-UDP"]
    #[inline(always)]
    pub fn ptpipv4(&mut self) -> PTPIPV4_W {
        PTPIPV4_W { w: self }
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages"]
    #[inline(always)]
    pub fn tsevnt(&mut self) -> TSEVNT_W {
        TSEVNT_W { w: self }
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master"]
    #[inline(always)]
    pub fn tsmast(&mut self) -> TSMAST_W {
        TSMAST_W { w: self }
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots"]
    #[inline(always)]
    pub fn selptp(&mut self) -> SELPTP_W {
        SELPTP_W { w: self }
    }
    #[doc = "Bit 18 - Enable MAC address for PTP Frame Filtering"]
    #[inline(always)]
    pub fn ptpfltr(&mut self) -> PTPFLTR_W {
        PTPFLTR_W { w: self }
    }
}
