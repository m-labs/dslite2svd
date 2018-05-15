#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMARIS {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct TIR {
    bits: bool,
}
impl TIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TPSR {
    bits: bool,
}
impl TPSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TUR {
    bits: bool,
}
impl TUR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TJTR {
    bits: bool,
}
impl TJTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct OVFR {
    bits: bool,
}
impl OVFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct UNFR {
    bits: bool,
}
impl UNFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RIR {
    bits: bool,
}
impl RIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RUR {
    bits: bool,
}
impl RUR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RPSR {
    bits: bool,
}
impl RPSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RWTR {
    bits: bool,
}
impl RWTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ETIR {
    bits: bool,
}
impl ETIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FBIR {
    bits: bool,
}
impl FBIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ERIR {
    bits: bool,
}
impl ERIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct AISR {
    bits: bool,
}
impl AISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct NISR {
    bits: bool,
}
impl NISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSR {
    #[doc = "Stopped: Reset or stop receive command issued"]
    STOP,
    #[doc = "Running: Fetching receive transfer descriptor"]
    RUNRXTD,
    #[doc = "Running: Waiting for receive packet"]
    RUNRXD,
    #[doc = "Suspended: Receive descriptor unavailable"]
    SUSPEND,
    #[doc = "Running: Closing receive descriptor"]
    RUNCRD,
    #[doc = "Writing Timestamp"]
    TSWS,
    #[doc = "Running: Transferring the receive packet data from receive buffer to host memory"]
    RUNTXD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RSR::STOP => 0,
            RSR::RUNRXTD => 1,
            RSR::RUNRXD => 3,
            RSR::SUSPEND => 4,
            RSR::RUNCRD => 5,
            RSR::TSWS => 6,
            RSR::RUNTXD => 7,
            RSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RSR {
        match value {
            0 => RSR::STOP,
            1 => RSR::RUNRXTD,
            3 => RSR::RUNRXD,
            4 => RSR::SUSPEND,
            5 => RSR::RUNCRD,
            6 => RSR::TSWS,
            7 => RSR::RUNTXD,
            i => RSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == RSR::STOP
    }
    #[doc = "Checks if the value of the field is `RUNRXTD`"]
    #[inline]
    pub fn is_runrxtd(&self) -> bool {
        *self == RSR::RUNRXTD
    }
    #[doc = "Checks if the value of the field is `RUNRXD`"]
    #[inline]
    pub fn is_runrxd(&self) -> bool {
        *self == RSR::RUNRXD
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline]
    pub fn is_suspend(&self) -> bool {
        *self == RSR::SUSPEND
    }
    #[doc = "Checks if the value of the field is `RUNCRD`"]
    #[inline]
    pub fn is_runcrd(&self) -> bool {
        *self == RSR::RUNCRD
    }
    #[doc = "Checks if the value of the field is `TSWS`"]
    #[inline]
    pub fn is_tsws(&self) -> bool {
        *self == RSR::TSWS
    }
    #[doc = "Checks if the value of the field is `RUNTXD`"]
    #[inline]
    pub fn is_runtxd(&self) -> bool {
        *self == RSR::RUNTXD
    }
}
#[doc = "Possible values of the field `TS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSR {
    #[doc = "Stopped; Reset or Stop transmit command processed"]
    STOP,
    #[doc = "Running; Fetching transmit transfer descriptor"]
    RUNTXTD,
    #[doc = "Running; Waiting for status"]
    STATUS,
    #[doc = "Running; Reading data from host memory buffer and queuing it to transmit buffer (TX FIFO)"]
    RUNTX,
    #[doc = "Writing Timestamp"]
    TSTAMP,
    #[doc = "Suspended; Transmit descriptor unavailable or transmit buffer underflow"]
    SUSPEND,
    #[doc = "Running; Closing transmit descriptor"]
    RUNCTD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSR::STOP => 0,
            TSR::RUNTXTD => 1,
            TSR::STATUS => 2,
            TSR::RUNTX => 3,
            TSR::TSTAMP => 4,
            TSR::SUSPEND => 6,
            TSR::RUNCTD => 7,
            TSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSR {
        match value {
            0 => TSR::STOP,
            1 => TSR::RUNTXTD,
            2 => TSR::STATUS,
            3 => TSR::RUNTX,
            4 => TSR::TSTAMP,
            6 => TSR::SUSPEND,
            7 => TSR::RUNCTD,
            i => TSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == TSR::STOP
    }
    #[doc = "Checks if the value of the field is `RUNTXTD`"]
    #[inline]
    pub fn is_runtxtd(&self) -> bool {
        *self == TSR::RUNTXTD
    }
    #[doc = "Checks if the value of the field is `STATUS`"]
    #[inline]
    pub fn is_status(&self) -> bool {
        *self == TSR::STATUS
    }
    #[doc = "Checks if the value of the field is `RUNTX`"]
    #[inline]
    pub fn is_runtx(&self) -> bool {
        *self == TSR::RUNTX
    }
    #[doc = "Checks if the value of the field is `TSTAMP`"]
    #[inline]
    pub fn is_tstamp(&self) -> bool {
        *self == TSR::TSTAMP
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline]
    pub fn is_suspend(&self) -> bool {
        *self == TSR::SUSPEND
    }
    #[doc = "Checks if the value of the field is `RUNCTD`"]
    #[inline]
    pub fn is_runctd(&self) -> bool {
        *self == TSR::RUNCTD
    }
}
#[doc = "Possible values of the field `AE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AER {
    #[doc = "Error during RX DMA Write Data Transfer"]
    RXDMAWD,
    #[doc = "Error during TX DMA Read Data Transfer"]
    TXDMARD,
    #[doc = "Error during RX DMA Descriptor Write Access"]
    RXDMADW,
    #[doc = "Error during TX DMA Descriptor Write Access"]
    TXDMADW,
    #[doc = "Error during RX DMA Descriptor Read Access"]
    RXDMADR,
    #[doc = "Error during TX DMA Descriptor Read Access"]
    TXDMADR,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AER::RXDMAWD => 0,
            AER::TXDMARD => 3,
            AER::RXDMADW => 4,
            AER::TXDMADW => 5,
            AER::RXDMADR => 6,
            AER::TXDMADR => 7,
            AER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AER {
        match value {
            0 => AER::RXDMAWD,
            3 => AER::TXDMARD,
            4 => AER::RXDMADW,
            5 => AER::TXDMADW,
            6 => AER::RXDMADR,
            7 => AER::TXDMADR,
            i => AER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RXDMAWD`"]
    #[inline]
    pub fn is_rxdmawd(&self) -> bool {
        *self == AER::RXDMAWD
    }
    #[doc = "Checks if the value of the field is `TXDMARD`"]
    #[inline]
    pub fn is_txdmard(&self) -> bool {
        *self == AER::TXDMARD
    }
    #[doc = "Checks if the value of the field is `RXDMADW`"]
    #[inline]
    pub fn is_rxdmadw(&self) -> bool {
        *self == AER::RXDMADW
    }
    #[doc = "Checks if the value of the field is `TXDMADW`"]
    #[inline]
    pub fn is_txdmadw(&self) -> bool {
        *self == AER::TXDMADW
    }
    #[doc = "Checks if the value of the field is `RXDMADR`"]
    #[inline]
    pub fn is_rxdmadr(&self) -> bool {
        *self == AER::RXDMADR
    }
    #[doc = "Checks if the value of the field is `TXDMADR`"]
    #[inline]
    pub fn is_txdmadr(&self) -> bool {
        *self == AER::TXDMADR
    }
}
#[doc = r" Value of the field"]
pub struct MMCR {
    bits: bool,
}
impl MMCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PMTR {
    bits: bool,
}
impl PMTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TTR {
    bits: bool,
}
impl TTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _TIW<'a> {
    w: &'a mut W,
}
impl<'a> _TIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TPSW<'a> {
    w: &'a mut W,
}
impl<'a> _TPSW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TUW<'a> {
    w: &'a mut W,
}
impl<'a> _TUW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TJTW<'a> {
    w: &'a mut W,
}
impl<'a> _TJTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OVFW<'a> {
    w: &'a mut W,
}
impl<'a> _OVFW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UNFW<'a> {
    w: &'a mut W,
}
impl<'a> _UNFW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RIW<'a> {
    w: &'a mut W,
}
impl<'a> _RIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RUW<'a> {
    w: &'a mut W,
}
impl<'a> _RUW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RPSW<'a> {
    w: &'a mut W,
}
impl<'a> _RPSW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RWTW<'a> {
    w: &'a mut W,
}
impl<'a> _RWTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ETIW<'a> {
    w: &'a mut W,
}
impl<'a> _ETIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FBIW<'a> {
    w: &'a mut W,
}
impl<'a> _FBIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ERIW<'a> {
    w: &'a mut W,
}
impl<'a> _ERIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AISW<'a> {
    w: &'a mut W,
}
impl<'a> _AISW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NISW<'a> {
    w: &'a mut W,
}
impl<'a> _NISW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RS`"]
pub enum RSW {
    #[doc = "Stopped: Reset or stop receive command issued"]
    STOP,
    #[doc = "Running: Fetching receive transfer descriptor"]
    RUNRXTD,
    #[doc = "Running: Waiting for receive packet"]
    RUNRXD,
    #[doc = "Suspended: Receive descriptor unavailable"]
    SUSPEND,
    #[doc = "Running: Closing receive descriptor"]
    RUNCRD,
    #[doc = "Writing Timestamp"]
    TSWS,
    #[doc = "Running: Transferring the receive packet data from receive buffer to host memory"]
    RUNTXD,
}
impl RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RSW::STOP => 0,
            RSW::RUNRXTD => 1,
            RSW::RUNRXD => 3,
            RSW::SUSPEND => 4,
            RSW::RUNCRD => 5,
            RSW::TSWS => 6,
            RSW::RUNTXD => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSW<'a> {
    w: &'a mut W,
}
impl<'a> _RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Stopped: Reset or stop receive command issued"]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(RSW::STOP)
    }
    #[doc = "Running: Fetching receive transfer descriptor"]
    #[inline]
    pub fn runrxtd(self) -> &'a mut W {
        self.variant(RSW::RUNRXTD)
    }
    #[doc = "Running: Waiting for receive packet"]
    #[inline]
    pub fn runrxd(self) -> &'a mut W {
        self.variant(RSW::RUNRXD)
    }
    #[doc = "Suspended: Receive descriptor unavailable"]
    #[inline]
    pub fn suspend(self) -> &'a mut W {
        self.variant(RSW::SUSPEND)
    }
    #[doc = "Running: Closing receive descriptor"]
    #[inline]
    pub fn runcrd(self) -> &'a mut W {
        self.variant(RSW::RUNCRD)
    }
    #[doc = "Writing Timestamp"]
    #[inline]
    pub fn tsws(self) -> &'a mut W {
        self.variant(RSW::TSWS)
    }
    #[doc = "Running: Transferring the receive packet data from receive buffer to host memory"]
    #[inline]
    pub fn runtxd(self) -> &'a mut W {
        self.variant(RSW::RUNTXD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TS`"]
pub enum TSW {
    #[doc = "Stopped; Reset or Stop transmit command processed"]
    STOP,
    #[doc = "Running; Fetching transmit transfer descriptor"]
    RUNTXTD,
    #[doc = "Running; Waiting for status"]
    STATUS,
    #[doc = "Running; Reading data from host memory buffer and queuing it to transmit buffer (TX FIFO)"]
    RUNTX,
    #[doc = "Writing Timestamp"]
    TSTAMP,
    #[doc = "Suspended; Transmit descriptor unavailable or transmit buffer underflow"]
    SUSPEND,
    #[doc = "Running; Closing transmit descriptor"]
    RUNCTD,
}
impl TSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSW::STOP => 0,
            TSW::RUNTXTD => 1,
            TSW::STATUS => 2,
            TSW::RUNTX => 3,
            TSW::TSTAMP => 4,
            TSW::SUSPEND => 6,
            TSW::RUNCTD => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSW<'a> {
    w: &'a mut W,
}
impl<'a> _TSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Stopped; Reset or Stop transmit command processed"]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(TSW::STOP)
    }
    #[doc = "Running; Fetching transmit transfer descriptor"]
    #[inline]
    pub fn runtxtd(self) -> &'a mut W {
        self.variant(TSW::RUNTXTD)
    }
    #[doc = "Running; Waiting for status"]
    #[inline]
    pub fn status(self) -> &'a mut W {
        self.variant(TSW::STATUS)
    }
    #[doc = "Running; Reading data from host memory buffer and queuing it to transmit buffer (TX FIFO)"]
    #[inline]
    pub fn runtx(self) -> &'a mut W {
        self.variant(TSW::RUNTX)
    }
    #[doc = "Writing Timestamp"]
    #[inline]
    pub fn tstamp(self) -> &'a mut W {
        self.variant(TSW::TSTAMP)
    }
    #[doc = "Suspended; Transmit descriptor unavailable or transmit buffer underflow"]
    #[inline]
    pub fn suspend(self) -> &'a mut W {
        self.variant(TSW::SUSPEND)
    }
    #[doc = "Running; Closing transmit descriptor"]
    #[inline]
    pub fn runctd(self) -> &'a mut W {
        self.variant(TSW::RUNCTD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AE`"]
pub enum AEW {
    #[doc = "Error during RX DMA Write Data Transfer"]
    RXDMAWD,
    #[doc = "Error during TX DMA Read Data Transfer"]
    TXDMARD,
    #[doc = "Error during RX DMA Descriptor Write Access"]
    RXDMADW,
    #[doc = "Error during TX DMA Descriptor Write Access"]
    TXDMADW,
    #[doc = "Error during RX DMA Descriptor Read Access"]
    RXDMADR,
    #[doc = "Error during TX DMA Descriptor Read Access"]
    TXDMADR,
}
impl AEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AEW::RXDMAWD => 0,
            AEW::TXDMARD => 3,
            AEW::RXDMADW => 4,
            AEW::TXDMADW => 5,
            AEW::RXDMADR => 6,
            AEW::TXDMADR => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AEW<'a> {
    w: &'a mut W,
}
impl<'a> _AEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Error during RX DMA Write Data Transfer"]
    #[inline]
    pub fn rxdmawd(self) -> &'a mut W {
        self.variant(AEW::RXDMAWD)
    }
    #[doc = "Error during TX DMA Read Data Transfer"]
    #[inline]
    pub fn txdmard(self) -> &'a mut W {
        self.variant(AEW::TXDMARD)
    }
    #[doc = "Error during RX DMA Descriptor Write Access"]
    #[inline]
    pub fn rxdmadw(self) -> &'a mut W {
        self.variant(AEW::RXDMADW)
    }
    #[doc = "Error during TX DMA Descriptor Write Access"]
    #[inline]
    pub fn txdmadw(self) -> &'a mut W {
        self.variant(AEW::TXDMADW)
    }
    #[doc = "Error during RX DMA Descriptor Read Access"]
    #[inline]
    pub fn rxdmadr(self) -> &'a mut W {
        self.variant(AEW::RXDMADR)
    }
    #[doc = "Error during TX DMA Descriptor Read Access"]
    #[inline]
    pub fn txdmadr(self) -> &'a mut W {
        self.variant(AEW::TXDMADR)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MMCW<'a> {
    w: &'a mut W,
}
impl<'a> _MMCW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PMTW<'a> {
    w: &'a mut W,
}
impl<'a> _PMTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TTW<'a> {
    w: &'a mut W,
}
impl<'a> _TTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline]
    pub fn ti(&self) -> TIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TIR { bits }
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline]
    pub fn tps(&self) -> TPSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TPSR { bits }
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline]
    pub fn tu(&self) -> TUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TUR { bits }
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout"]
    #[inline]
    pub fn tjt(&self) -> TJTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TJTR { bits }
    }
    #[doc = "Bit 4 - Receive Overflow"]
    #[inline]
    pub fn ovf(&self) -> OVFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OVFR { bits }
    }
    #[doc = "Bit 5 - Transmit Underflow"]
    #[inline]
    pub fn unf(&self) -> UNFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UNFR { bits }
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline]
    pub fn ri(&self) -> RIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RIR { bits }
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline]
    pub fn ru(&self) -> RUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RUR { bits }
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline]
    pub fn rps(&self) -> RPSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RPSR { bits }
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline]
    pub fn rwt(&self) -> RWTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RWTR { bits }
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline]
    pub fn eti(&self) -> ETIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ETIR { bits }
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt"]
    #[inline]
    pub fn fbi(&self) -> FBIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FBIR { bits }
    }
    #[doc = "Bit 14 - Early Receive Interrupt"]
    #[inline]
    pub fn eri(&self) -> ERIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ERIR { bits }
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary"]
    #[inline]
    pub fn ais(&self) -> AISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AISR { bits }
    }
    #[doc = "Bit 16 - Normal Interrupt Summary"]
    #[inline]
    pub fn nis(&self) -> NISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NISR { bits }
    }
    #[doc = "Bits 17:19 - Received Process State"]
    #[inline]
    pub fn rs(&self) -> RSR {
        RSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:22 - Transmit Process State"]
    #[inline]
    pub fn ts(&self) -> TSR {
        TSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 23:25 - Access Error"]
    #[inline]
    pub fn ae(&self) -> AER {
        AER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 27 - MAC MMC Interrupt"]
    #[inline]
    pub fn mmc(&self) -> MMCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MMCR { bits }
    }
    #[doc = "Bit 28 - MAC PMT Interrupt Status"]
    #[inline]
    pub fn pmt(&self) -> PMTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PMTR { bits }
    }
    #[doc = "Bit 29 - Timestamp Trigger Interrupt Status"]
    #[inline]
    pub fn tt(&self) -> TTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline]
    pub fn ti(&mut self) -> _TIW {
        _TIW { w: self }
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline]
    pub fn tps(&mut self) -> _TPSW {
        _TPSW { w: self }
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline]
    pub fn tu(&mut self) -> _TUW {
        _TUW { w: self }
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout"]
    #[inline]
    pub fn tjt(&mut self) -> _TJTW {
        _TJTW { w: self }
    }
    #[doc = "Bit 4 - Receive Overflow"]
    #[inline]
    pub fn ovf(&mut self) -> _OVFW {
        _OVFW { w: self }
    }
    #[doc = "Bit 5 - Transmit Underflow"]
    #[inline]
    pub fn unf(&mut self) -> _UNFW {
        _UNFW { w: self }
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline]
    pub fn ri(&mut self) -> _RIW {
        _RIW { w: self }
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline]
    pub fn ru(&mut self) -> _RUW {
        _RUW { w: self }
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline]
    pub fn rps(&mut self) -> _RPSW {
        _RPSW { w: self }
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline]
    pub fn rwt(&mut self) -> _RWTW {
        _RWTW { w: self }
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline]
    pub fn eti(&mut self) -> _ETIW {
        _ETIW { w: self }
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt"]
    #[inline]
    pub fn fbi(&mut self) -> _FBIW {
        _FBIW { w: self }
    }
    #[doc = "Bit 14 - Early Receive Interrupt"]
    #[inline]
    pub fn eri(&mut self) -> _ERIW {
        _ERIW { w: self }
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary"]
    #[inline]
    pub fn ais(&mut self) -> _AISW {
        _AISW { w: self }
    }
    #[doc = "Bit 16 - Normal Interrupt Summary"]
    #[inline]
    pub fn nis(&mut self) -> _NISW {
        _NISW { w: self }
    }
    #[doc = "Bits 17:19 - Received Process State"]
    #[inline]
    pub fn rs(&mut self) -> _RSW {
        _RSW { w: self }
    }
    #[doc = "Bits 20:22 - Transmit Process State"]
    #[inline]
    pub fn ts(&mut self) -> _TSW {
        _TSW { w: self }
    }
    #[doc = "Bits 23:25 - Access Error"]
    #[inline]
    pub fn ae(&mut self) -> _AEW {
        _AEW { w: self }
    }
    #[doc = "Bit 27 - MAC MMC Interrupt"]
    #[inline]
    pub fn mmc(&mut self) -> _MMCW {
        _MMCW { w: self }
    }
    #[doc = "Bit 28 - MAC PMT Interrupt Status"]
    #[inline]
    pub fn pmt(&mut self) -> _PMTW {
        _PMTW { w: self }
    }
    #[doc = "Bit 29 - Timestamp Trigger Interrupt Status"]
    #[inline]
    pub fn tt(&mut self) -> _TTW {
        _TTW { w: self }
    }
}
