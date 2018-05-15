#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RPER {
    bits: bool,
}
impl RPER {
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
pub struct RFCFCR {
    bits: u8,
}
impl RFCFCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RWCR {
    bits: bool,
}
impl RWCR {
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
#[doc = "Possible values of the field `RRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRCR {
    #[doc = "IDLE state"]
    IDLE,
    #[doc = "Reading frame data"]
    STATUS,
    #[doc = "Reading frame status (or timestamp)"]
    DATA,
    #[doc = "Flushing the frame data and status"]
    FLUSH,
}
impl RRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RRCR::IDLE => 0,
            RRCR::STATUS => 1,
            RRCR::DATA => 2,
            RRCR::FLUSH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RRCR {
        match value {
            0 => RRCR::IDLE,
            1 => RRCR::STATUS,
            2 => RRCR::DATA,
            3 => RRCR::FLUSH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == RRCR::IDLE
    }
    #[doc = "Checks if the value of the field is `STATUS`"]
    #[inline]
    pub fn is_status(&self) -> bool {
        *self == RRCR::STATUS
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline]
    pub fn is_data(&self) -> bool {
        *self == RRCR::DATA
    }
    #[doc = "Checks if the value of the field is `FLUSH`"]
    #[inline]
    pub fn is_flush(&self) -> bool {
        *self == RRCR::FLUSH
    }
}
#[doc = "Possible values of the field `RXF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFR {
    #[doc = "RX FIFO Empty"]
    EMPTY,
    #[doc = "RX FIFO fill level is below the flow-control deactivate threshold"]
    BELOW,
    #[doc = "RX FIFO fill level is above the flow-control activate threshold"]
    ABOVE,
    #[doc = "RX FIFO Full"]
    FULL,
}
impl RXFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXFR::EMPTY => 0,
            RXFR::BELOW => 1,
            RXFR::ABOVE => 2,
            RXFR::FULL => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXFR {
        match value {
            0 => RXFR::EMPTY,
            1 => RXFR::BELOW,
            2 => RXFR::ABOVE,
            3 => RXFR::FULL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline]
    pub fn is_empty(&self) -> bool {
        *self == RXFR::EMPTY
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline]
    pub fn is_below(&self) -> bool {
        *self == RXFR::BELOW
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline]
    pub fn is_above(&self) -> bool {
        *self == RXFR::ABOVE
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline]
    pub fn is_full(&self) -> bool {
        *self == RXFR::FULL
    }
}
#[doc = r" Value of the field"]
pub struct TPER {
    bits: bool,
}
impl TPER {
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
#[doc = "Possible values of the field `TFC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFCR {
    #[doc = "IDLE state"]
    IDLE,
    #[doc = "Waiting for status of previous frame or IFG or backoff period to be over"]
    STATUS,
    #[doc = "Generating and transmitting a PAUSE control frame (in the full-duplex mode)"]
    PAUSE,
    #[doc = "Transferring input frame for transmission"]
    INPUT,
}
impl TFCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TFCR::IDLE => 0,
            TFCR::STATUS => 1,
            TFCR::PAUSE => 2,
            TFCR::INPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TFCR {
        match value {
            0 => TFCR::IDLE,
            1 => TFCR::STATUS,
            2 => TFCR::PAUSE,
            3 => TFCR::INPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == TFCR::IDLE
    }
    #[doc = "Checks if the value of the field is `STATUS`"]
    #[inline]
    pub fn is_status(&self) -> bool {
        *self == TFCR::STATUS
    }
    #[doc = "Checks if the value of the field is `PAUSE`"]
    #[inline]
    pub fn is_pause(&self) -> bool {
        *self == TFCR::PAUSE
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == TFCR::INPUT
    }
}
#[doc = r" Value of the field"]
pub struct TXPAUSEDR {
    bits: bool,
}
impl TXPAUSEDR {
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
#[doc = "Possible values of the field `TRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRCR {
    #[doc = "IDLE state"]
    IDLE,
    #[doc = "READ state (transferring data to MAC transmitter)"]
    READ,
    #[doc = "Waiting for TX Status from MAC transmitter"]
    WAIT,
    #[doc = "Writing the received TX Status or flushing the TX FIFO"]
    WRFLUSH,
}
impl TRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRCR::IDLE => 0,
            TRCR::READ => 1,
            TRCR::WAIT => 2,
            TRCR::WRFLUSH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRCR {
        match value {
            0 => TRCR::IDLE,
            1 => TRCR::READ,
            2 => TRCR::WAIT,
            3 => TRCR::WRFLUSH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == TRCR::IDLE
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == TRCR::READ
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline]
    pub fn is_wait(&self) -> bool {
        *self == TRCR::WAIT
    }
    #[doc = "Checks if the value of the field is `WRFLUSH`"]
    #[inline]
    pub fn is_wrflush(&self) -> bool {
        *self == TRCR::WRFLUSH
    }
}
#[doc = r" Value of the field"]
pub struct TWCR {
    bits: bool,
}
impl TWCR {
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
pub struct TXFER {
    bits: bool,
}
impl TXFER {
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
pub struct TXFFR {
    bits: bool,
}
impl TXFFR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - MAC MII Receive Protocol Engine Status"]
    #[inline]
    pub fn rpe(&self) -> RPER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RPER { bits }
    }
    #[doc = "Bits 1:2 - MAC Receive Frame Controller FIFO Status"]
    #[inline]
    pub fn rfcfc(&self) -> RFCFCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RFCFCR { bits }
    }
    #[doc = "Bit 4 - TX/RX Controller RX FIFO Write Controller Active Status"]
    #[inline]
    pub fn rwc(&self) -> RWCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RWCR { bits }
    }
    #[doc = "Bits 5:6 - TX/RX Controller Read Controller State"]
    #[inline]
    pub fn rrc(&self) -> RRCR {
        RRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - TX/RX Controller RX FIFO Fill-level Status"]
    #[inline]
    pub fn rxf(&self) -> RXFR {
        RXFR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - MAC MII Transmit Protocol Engine Status"]
    #[inline]
    pub fn tpe(&self) -> TPER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TPER { bits }
    }
    #[doc = "Bits 17:18 - MAC Transmit Frame Controller Status"]
    #[inline]
    pub fn tfc(&self) -> TFCR {
        TFCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 19 - MAC Transmitter PAUSE"]
    #[inline]
    pub fn txpaused(&self) -> TXPAUSEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXPAUSEDR { bits }
    }
    #[doc = "Bits 20:21 - TX/RX Controller's TX FIFO Read Controller Status"]
    #[inline]
    pub fn trc(&self) -> TRCR {
        TRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - TX/RX Controller TX FIFO Write Controller Active Status"]
    #[inline]
    pub fn twc(&self) -> TWCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TWCR { bits }
    }
    #[doc = "Bit 24 - TX/RX Controller TX FIFO Not Empty Status"]
    #[inline]
    pub fn txfe(&self) -> TXFER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXFER { bits }
    }
    #[doc = "Bit 25 - TX/RX Controller TX FIFO Full Status"]
    #[inline]
    pub fn txff(&self) -> TXFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXFFR { bits }
    }
}
