#[doc = "Reader of register DMARIS"]
pub type R = crate::R<u32, super::DMARIS>;
#[doc = "Writer for register DMARIS"]
pub type W = crate::W<u32, super::DMARIS>;
#[doc = "Register DMARIS `reset()`'s with value 0"]
impl crate::ResetValue for super::DMARIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TI`"]
pub type TI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TI`"]
pub struct TI_W<'a> {
    w: &'a mut W,
}
impl<'a> TI_W<'a> {
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
#[doc = "Reader of field `TPS`"]
pub type TPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TPS`"]
pub struct TPS_W<'a> {
    w: &'a mut W,
}
impl<'a> TPS_W<'a> {
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
#[doc = "Reader of field `TU`"]
pub type TU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TU`"]
pub struct TU_W<'a> {
    w: &'a mut W,
}
impl<'a> TU_W<'a> {
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
#[doc = "Reader of field `TJT`"]
pub type TJT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TJT`"]
pub struct TJT_W<'a> {
    w: &'a mut W,
}
impl<'a> TJT_W<'a> {
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
#[doc = "Reader of field `OVF`"]
pub type OVF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVF`"]
pub struct OVF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_W<'a> {
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
#[doc = "Reader of field `UNF`"]
pub type UNF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNF`"]
pub struct UNF_W<'a> {
    w: &'a mut W,
}
impl<'a> UNF_W<'a> {
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
#[doc = "Reader of field `RI`"]
pub type RI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RI`"]
pub struct RI_W<'a> {
    w: &'a mut W,
}
impl<'a> RI_W<'a> {
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
#[doc = "Reader of field `RU`"]
pub type RU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RU`"]
pub struct RU_W<'a> {
    w: &'a mut W,
}
impl<'a> RU_W<'a> {
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
#[doc = "Reader of field `RPS`"]
pub type RPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPS`"]
pub struct RPS_W<'a> {
    w: &'a mut W,
}
impl<'a> RPS_W<'a> {
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
#[doc = "Reader of field `RWT`"]
pub type RWT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWT`"]
pub struct RWT_W<'a> {
    w: &'a mut W,
}
impl<'a> RWT_W<'a> {
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
#[doc = "Reader of field `ETI`"]
pub type ETI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETI`"]
pub struct ETI_W<'a> {
    w: &'a mut W,
}
impl<'a> ETI_W<'a> {
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
#[doc = "Reader of field `FBI`"]
pub type FBI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBI`"]
pub struct FBI_W<'a> {
    w: &'a mut W,
}
impl<'a> FBI_W<'a> {
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
#[doc = "Reader of field `ERI`"]
pub type ERI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERI`"]
pub struct ERI_W<'a> {
    w: &'a mut W,
}
impl<'a> ERI_W<'a> {
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
#[doc = "Reader of field `AIS`"]
pub type AIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AIS`"]
pub struct AIS_W<'a> {
    w: &'a mut W,
}
impl<'a> AIS_W<'a> {
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
#[doc = "Reader of field `NIS`"]
pub type NIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NIS`"]
pub struct NIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Received Process State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RS_A {
    #[doc = "0: Stopped: Reset or stop receive command issued"]
    STOP = 0,
    #[doc = "1: Running: Fetching receive transfer descriptor"]
    RUNRXTD = 1,
    #[doc = "3: Running: Waiting for receive packet"]
    RUNRXD = 3,
    #[doc = "4: Suspended: Receive descriptor unavailable"]
    SUSPEND = 4,
    #[doc = "5: Running: Closing receive descriptor"]
    RUNCRD = 5,
    #[doc = "6: Writing Timestamp"]
    TSWS = 6,
    #[doc = "7: Running: Transferring the receive packet data from receive buffer to host memory"]
    RUNTXD = 7,
}
impl From<RS_A> for u8 {
    #[inline(always)]
    fn from(variant: RS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RS`"]
pub type RS_R = crate::R<u8, RS_A>;
impl RS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RS_A::STOP),
            1 => Val(RS_A::RUNRXTD),
            3 => Val(RS_A::RUNRXD),
            4 => Val(RS_A::SUSPEND),
            5 => Val(RS_A::RUNCRD),
            6 => Val(RS_A::TSWS),
            7 => Val(RS_A::RUNTXD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == RS_A::STOP
    }
    #[doc = "Checks if the value of the field is `RUNRXTD`"]
    #[inline(always)]
    pub fn is_runrxtd(&self) -> bool {
        *self == RS_A::RUNRXTD
    }
    #[doc = "Checks if the value of the field is `RUNRXD`"]
    #[inline(always)]
    pub fn is_runrxd(&self) -> bool {
        *self == RS_A::RUNRXD
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == RS_A::SUSPEND
    }
    #[doc = "Checks if the value of the field is `RUNCRD`"]
    #[inline(always)]
    pub fn is_runcrd(&self) -> bool {
        *self == RS_A::RUNCRD
    }
    #[doc = "Checks if the value of the field is `TSWS`"]
    #[inline(always)]
    pub fn is_tsws(&self) -> bool {
        *self == RS_A::TSWS
    }
    #[doc = "Checks if the value of the field is `RUNTXD`"]
    #[inline(always)]
    pub fn is_runtxd(&self) -> bool {
        *self == RS_A::RUNTXD
    }
}
#[doc = "Write proxy for field `RS`"]
pub struct RS_W<'a> {
    w: &'a mut W,
}
impl<'a> RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Stopped: Reset or stop receive command issued"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(RS_A::STOP)
    }
    #[doc = "Running: Fetching receive transfer descriptor"]
    #[inline(always)]
    pub fn runrxtd(self) -> &'a mut W {
        self.variant(RS_A::RUNRXTD)
    }
    #[doc = "Running: Waiting for receive packet"]
    #[inline(always)]
    pub fn runrxd(self) -> &'a mut W {
        self.variant(RS_A::RUNRXD)
    }
    #[doc = "Suspended: Receive descriptor unavailable"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut W {
        self.variant(RS_A::SUSPEND)
    }
    #[doc = "Running: Closing receive descriptor"]
    #[inline(always)]
    pub fn runcrd(self) -> &'a mut W {
        self.variant(RS_A::RUNCRD)
    }
    #[doc = "Writing Timestamp"]
    #[inline(always)]
    pub fn tsws(self) -> &'a mut W {
        self.variant(RS_A::TSWS)
    }
    #[doc = "Running: Transferring the receive packet data from receive buffer to host memory"]
    #[inline(always)]
    pub fn runtxd(self) -> &'a mut W {
        self.variant(RS_A::RUNTXD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
#[doc = "Transmit Process State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TS_A {
    #[doc = "0: Stopped; Reset or Stop transmit command processed"]
    STOP = 0,
    #[doc = "1: Running; Fetching transmit transfer descriptor"]
    RUNTXTD = 1,
    #[doc = "2: Running; Waiting for status"]
    STATUS = 2,
    #[doc = "3: Running; Reading data from host memory buffer and queuing it to transmit buffer (TX FIFO)"]
    RUNTX = 3,
    #[doc = "4: Writing Timestamp"]
    TSTAMP = 4,
    #[doc = "6: Suspended; Transmit descriptor unavailable or transmit buffer underflow"]
    SUSPEND = 6,
    #[doc = "7: Running; Closing transmit descriptor"]
    RUNCTD = 7,
}
impl From<TS_A> for u8 {
    #[inline(always)]
    fn from(variant: TS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TS`"]
pub type TS_R = crate::R<u8, TS_A>;
impl TS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TS_A::STOP),
            1 => Val(TS_A::RUNTXTD),
            2 => Val(TS_A::STATUS),
            3 => Val(TS_A::RUNTX),
            4 => Val(TS_A::TSTAMP),
            6 => Val(TS_A::SUSPEND),
            7 => Val(TS_A::RUNCTD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == TS_A::STOP
    }
    #[doc = "Checks if the value of the field is `RUNTXTD`"]
    #[inline(always)]
    pub fn is_runtxtd(&self) -> bool {
        *self == TS_A::RUNTXTD
    }
    #[doc = "Checks if the value of the field is `STATUS`"]
    #[inline(always)]
    pub fn is_status(&self) -> bool {
        *self == TS_A::STATUS
    }
    #[doc = "Checks if the value of the field is `RUNTX`"]
    #[inline(always)]
    pub fn is_runtx(&self) -> bool {
        *self == TS_A::RUNTX
    }
    #[doc = "Checks if the value of the field is `TSTAMP`"]
    #[inline(always)]
    pub fn is_tstamp(&self) -> bool {
        *self == TS_A::TSTAMP
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == TS_A::SUSPEND
    }
    #[doc = "Checks if the value of the field is `RUNCTD`"]
    #[inline(always)]
    pub fn is_runctd(&self) -> bool {
        *self == TS_A::RUNCTD
    }
}
#[doc = "Write proxy for field `TS`"]
pub struct TS_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Stopped; Reset or Stop transmit command processed"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(TS_A::STOP)
    }
    #[doc = "Running; Fetching transmit transfer descriptor"]
    #[inline(always)]
    pub fn runtxtd(self) -> &'a mut W {
        self.variant(TS_A::RUNTXTD)
    }
    #[doc = "Running; Waiting for status"]
    #[inline(always)]
    pub fn status(self) -> &'a mut W {
        self.variant(TS_A::STATUS)
    }
    #[doc = "Running; Reading data from host memory buffer and queuing it to transmit buffer (TX FIFO)"]
    #[inline(always)]
    pub fn runtx(self) -> &'a mut W {
        self.variant(TS_A::RUNTX)
    }
    #[doc = "Writing Timestamp"]
    #[inline(always)]
    pub fn tstamp(self) -> &'a mut W {
        self.variant(TS_A::TSTAMP)
    }
    #[doc = "Suspended; Transmit descriptor unavailable or transmit buffer underflow"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut W {
        self.variant(TS_A::SUSPEND)
    }
    #[doc = "Running; Closing transmit descriptor"]
    #[inline(always)]
    pub fn runctd(self) -> &'a mut W {
        self.variant(TS_A::RUNCTD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Access Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AE_A {
    #[doc = "0: Error during RX DMA Write Data Transfer"]
    RXDMAWD = 0,
    #[doc = "3: Error during TX DMA Read Data Transfer"]
    TXDMARD = 3,
    #[doc = "4: Error during RX DMA Descriptor Write Access"]
    RXDMADW = 4,
    #[doc = "5: Error during TX DMA Descriptor Write Access"]
    TXDMADW = 5,
    #[doc = "6: Error during RX DMA Descriptor Read Access"]
    RXDMADR = 6,
    #[doc = "7: Error during TX DMA Descriptor Read Access"]
    TXDMADR = 7,
}
impl From<AE_A> for u8 {
    #[inline(always)]
    fn from(variant: AE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AE`"]
pub type AE_R = crate::R<u8, AE_A>;
impl AE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AE_A::RXDMAWD),
            3 => Val(AE_A::TXDMARD),
            4 => Val(AE_A::RXDMADW),
            5 => Val(AE_A::TXDMADW),
            6 => Val(AE_A::RXDMADR),
            7 => Val(AE_A::TXDMADR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RXDMAWD`"]
    #[inline(always)]
    pub fn is_rxdmawd(&self) -> bool {
        *self == AE_A::RXDMAWD
    }
    #[doc = "Checks if the value of the field is `TXDMARD`"]
    #[inline(always)]
    pub fn is_txdmard(&self) -> bool {
        *self == AE_A::TXDMARD
    }
    #[doc = "Checks if the value of the field is `RXDMADW`"]
    #[inline(always)]
    pub fn is_rxdmadw(&self) -> bool {
        *self == AE_A::RXDMADW
    }
    #[doc = "Checks if the value of the field is `TXDMADW`"]
    #[inline(always)]
    pub fn is_txdmadw(&self) -> bool {
        *self == AE_A::TXDMADW
    }
    #[doc = "Checks if the value of the field is `RXDMADR`"]
    #[inline(always)]
    pub fn is_rxdmadr(&self) -> bool {
        *self == AE_A::RXDMADR
    }
    #[doc = "Checks if the value of the field is `TXDMADR`"]
    #[inline(always)]
    pub fn is_txdmadr(&self) -> bool {
        *self == AE_A::TXDMADR
    }
}
#[doc = "Write proxy for field `AE`"]
pub struct AE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Error during RX DMA Write Data Transfer"]
    #[inline(always)]
    pub fn rxdmawd(self) -> &'a mut W {
        self.variant(AE_A::RXDMAWD)
    }
    #[doc = "Error during TX DMA Read Data Transfer"]
    #[inline(always)]
    pub fn txdmard(self) -> &'a mut W {
        self.variant(AE_A::TXDMARD)
    }
    #[doc = "Error during RX DMA Descriptor Write Access"]
    #[inline(always)]
    pub fn rxdmadw(self) -> &'a mut W {
        self.variant(AE_A::RXDMADW)
    }
    #[doc = "Error during TX DMA Descriptor Write Access"]
    #[inline(always)]
    pub fn txdmadw(self) -> &'a mut W {
        self.variant(AE_A::TXDMADW)
    }
    #[doc = "Error during RX DMA Descriptor Read Access"]
    #[inline(always)]
    pub fn rxdmadr(self) -> &'a mut W {
        self.variant(AE_A::RXDMADR)
    }
    #[doc = "Error during TX DMA Descriptor Read Access"]
    #[inline(always)]
    pub fn txdmadr(self) -> &'a mut W {
        self.variant(AE_A::TXDMADR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | (((value as u32) & 0x07) << 23);
        self.w
    }
}
#[doc = "Reader of field `MMC`"]
pub type MMC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MMC`"]
pub struct MMC_W<'a> {
    w: &'a mut W,
}
impl<'a> MMC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `PMT`"]
pub type PMT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMT`"]
pub struct PMT_W<'a> {
    w: &'a mut W,
}
impl<'a> PMT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `TT`"]
pub type TT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TT`"]
pub struct TT_W<'a> {
    w: &'a mut W,
}
impl<'a> TT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    pub fn tu(&self) -> TU_R {
        TU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout"]
    #[inline(always)]
    pub fn tjt(&self) -> TJT_R {
        TJT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Underflow"]
    #[inline(always)]
    pub fn unf(&self) -> UNF_R {
        UNF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    pub fn ru(&self) -> RU_R {
        RU_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn eti(&self) -> ETI_R {
        ETI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt"]
    #[inline(always)]
    pub fn fbi(&self) -> FBI_R {
        FBI_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn eri(&self) -> ERI_R {
        ERI_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - Received Process State"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Transmit Process State"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:25 - Access Error"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bit 27 - MAC MMC Interrupt"]
    #[inline(always)]
    pub fn mmc(&self) -> MMC_R {
        MMC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - MAC PMT Interrupt Status"]
    #[inline(always)]
    pub fn pmt(&self) -> PMT_R {
        PMT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Timestamp Trigger Interrupt Status"]
    #[inline(always)]
    pub fn tt(&self) -> TT_R {
        TT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn ti(&mut self) -> TI_W {
        TI_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    pub fn tps(&mut self) -> TPS_W {
        TPS_W { w: self }
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    pub fn tu(&mut self) -> TU_W {
        TU_W { w: self }
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout"]
    #[inline(always)]
    pub fn tjt(&mut self) -> TJT_W {
        TJT_W { w: self }
    }
    #[doc = "Bit 4 - Receive Overflow"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OVF_W {
        OVF_W { w: self }
    }
    #[doc = "Bit 5 - Transmit Underflow"]
    #[inline(always)]
    pub fn unf(&mut self) -> UNF_W {
        UNF_W { w: self }
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    pub fn ri(&mut self) -> RI_W {
        RI_W { w: self }
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    pub fn ru(&mut self) -> RU_W {
        RU_W { w: self }
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    pub fn rps(&mut self) -> RPS_W {
        RPS_W { w: self }
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn rwt(&mut self) -> RWT_W {
        RWT_W { w: self }
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn eti(&mut self) -> ETI_W {
        ETI_W { w: self }
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt"]
    #[inline(always)]
    pub fn fbi(&mut self) -> FBI_W {
        FBI_W { w: self }
    }
    #[doc = "Bit 14 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn eri(&mut self) -> ERI_W {
        ERI_W { w: self }
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn ais(&mut self) -> AIS_W {
        AIS_W { w: self }
    }
    #[doc = "Bit 16 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn nis(&mut self) -> NIS_W {
        NIS_W { w: self }
    }
    #[doc = "Bits 17:19 - Received Process State"]
    #[inline(always)]
    pub fn rs(&mut self) -> RS_W {
        RS_W { w: self }
    }
    #[doc = "Bits 20:22 - Transmit Process State"]
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W {
        TS_W { w: self }
    }
    #[doc = "Bits 23:25 - Access Error"]
    #[inline(always)]
    pub fn ae(&mut self) -> AE_W {
        AE_W { w: self }
    }
    #[doc = "Bit 27 - MAC MMC Interrupt"]
    #[inline(always)]
    pub fn mmc(&mut self) -> MMC_W {
        MMC_W { w: self }
    }
    #[doc = "Bit 28 - MAC PMT Interrupt Status"]
    #[inline(always)]
    pub fn pmt(&mut self) -> PMT_W {
        PMT_W { w: self }
    }
    #[doc = "Bit 29 - Timestamp Trigger Interrupt Status"]
    #[inline(always)]
    pub fn tt(&mut self) -> TT_W {
        TT_W { w: self }
    }
}
