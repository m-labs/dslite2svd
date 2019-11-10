#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `RPE`"]
pub type RPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RFCFC`"]
pub type RFCFC_R = crate::R<u8, u8>;
#[doc = "Reader of field `RWC`"]
pub type RWC_R = crate::R<bool, bool>;
#[doc = "TX/RX Controller Read Controller State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRC_A {
    #[doc = "0: IDLE state"]
    IDLE,
    #[doc = "1: Reading frame data"]
    STATUS,
    #[doc = "2: Reading frame status (or timestamp)"]
    DATA,
    #[doc = "3: Flushing the frame data and status"]
    FLUSH,
}
impl From<RRC_A> for u8 {
    #[inline(always)]
    fn from(variant: RRC_A) -> Self {
        match variant {
            RRC_A::IDLE => 0,
            RRC_A::STATUS => 1,
            RRC_A::DATA => 2,
            RRC_A::FLUSH => 3,
        }
    }
}
#[doc = "Reader of field `RRC`"]
pub type RRC_R = crate::R<u8, RRC_A>;
impl RRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRC_A {
        match self.bits {
            0 => RRC_A::IDLE,
            1 => RRC_A::STATUS,
            2 => RRC_A::DATA,
            3 => RRC_A::FLUSH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == RRC_A::IDLE
    }
    #[doc = "Checks if the value of the field is `STATUS`"]
    #[inline(always)]
    pub fn is_status(&self) -> bool {
        *self == RRC_A::STATUS
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == RRC_A::DATA
    }
    #[doc = "Checks if the value of the field is `FLUSH`"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == RRC_A::FLUSH
    }
}
#[doc = "TX/RX Controller RX FIFO Fill-level Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXF_A {
    #[doc = "0: RX FIFO Empty"]
    EMPTY,
    #[doc = "1: RX FIFO fill level is below the flow-control deactivate threshold"]
    BELOW,
    #[doc = "2: RX FIFO fill level is above the flow-control activate threshold"]
    ABOVE,
    #[doc = "3: RX FIFO Full"]
    FULL,
}
impl From<RXF_A> for u8 {
    #[inline(always)]
    fn from(variant: RXF_A) -> Self {
        match variant {
            RXF_A::EMPTY => 0,
            RXF_A::BELOW => 1,
            RXF_A::ABOVE => 2,
            RXF_A::FULL => 3,
        }
    }
}
#[doc = "Reader of field `RXF`"]
pub type RXF_R = crate::R<u8, RXF_A>;
impl RXF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXF_A {
        match self.bits {
            0 => RXF_A::EMPTY,
            1 => RXF_A::BELOW,
            2 => RXF_A::ABOVE,
            3 => RXF_A::FULL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXF_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == RXF_A::BELOW
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == RXF_A::ABOVE
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RXF_A::FULL
    }
}
#[doc = "Reader of field `TPE`"]
pub type TPE_R = crate::R<bool, bool>;
#[doc = "MAC Transmit Frame Controller Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFC_A {
    #[doc = "0: IDLE state"]
    IDLE,
    #[doc = "1: Waiting for status of previous frame or IFG or backoff period to be over"]
    STATUS,
    #[doc = "2: Generating and transmitting a PAUSE control frame (in the full-duplex mode)"]
    PAUSE,
    #[doc = "3: Transferring input frame for transmission"]
    INPUT,
}
impl From<TFC_A> for u8 {
    #[inline(always)]
    fn from(variant: TFC_A) -> Self {
        match variant {
            TFC_A::IDLE => 0,
            TFC_A::STATUS => 1,
            TFC_A::PAUSE => 2,
            TFC_A::INPUT => 3,
        }
    }
}
#[doc = "Reader of field `TFC`"]
pub type TFC_R = crate::R<u8, TFC_A>;
impl TFC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFC_A {
        match self.bits {
            0 => TFC_A::IDLE,
            1 => TFC_A::STATUS,
            2 => TFC_A::PAUSE,
            3 => TFC_A::INPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == TFC_A::IDLE
    }
    #[doc = "Checks if the value of the field is `STATUS`"]
    #[inline(always)]
    pub fn is_status(&self) -> bool {
        *self == TFC_A::STATUS
    }
    #[doc = "Checks if the value of the field is `PAUSE`"]
    #[inline(always)]
    pub fn is_pause(&self) -> bool {
        *self == TFC_A::PAUSE
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == TFC_A::INPUT
    }
}
#[doc = "Reader of field `TXPAUSED`"]
pub type TXPAUSED_R = crate::R<bool, bool>;
#[doc = "TX/RX Controller's TX FIFO Read Controller Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRC_A {
    #[doc = "0: IDLE state"]
    IDLE,
    #[doc = "1: READ state (transferring data to MAC transmitter)"]
    READ,
    #[doc = "2: Waiting for TX Status from MAC transmitter"]
    WAIT,
    #[doc = "3: Writing the received TX Status or flushing the TX FIFO"]
    WRFLUSH,
}
impl From<TRC_A> for u8 {
    #[inline(always)]
    fn from(variant: TRC_A) -> Self {
        match variant {
            TRC_A::IDLE => 0,
            TRC_A::READ => 1,
            TRC_A::WAIT => 2,
            TRC_A::WRFLUSH => 3,
        }
    }
}
#[doc = "Reader of field `TRC`"]
pub type TRC_R = crate::R<u8, TRC_A>;
impl TRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRC_A {
        match self.bits {
            0 => TRC_A::IDLE,
            1 => TRC_A::READ,
            2 => TRC_A::WAIT,
            3 => TRC_A::WRFLUSH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == TRC_A::IDLE
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == TRC_A::READ
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == TRC_A::WAIT
    }
    #[doc = "Checks if the value of the field is `WRFLUSH`"]
    #[inline(always)]
    pub fn is_wrflush(&self) -> bool {
        *self == TRC_A::WRFLUSH
    }
}
#[doc = "Reader of field `TWC`"]
pub type TWC_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFE`"]
pub type TXFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFF`"]
pub type TXFF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - MAC MII Receive Protocol Engine Status"]
    #[inline(always)]
    pub fn rpe(&self) -> RPE_R {
        RPE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - MAC Receive Frame Controller FIFO Status"]
    #[inline(always)]
    pub fn rfcfc(&self) -> RFCFC_R {
        RFCFC_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 4 - TX/RX Controller RX FIFO Write Controller Active Status"]
    #[inline(always)]
    pub fn rwc(&self) -> RWC_R {
        RWC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - TX/RX Controller Read Controller State"]
    #[inline(always)]
    pub fn rrc(&self) -> RRC_R {
        RRC_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - TX/RX Controller RX FIFO Fill-level Status"]
    #[inline(always)]
    pub fn rxf(&self) -> RXF_R {
        RXF_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 16 - MAC MII Transmit Protocol Engine Status"]
    #[inline(always)]
    pub fn tpe(&self) -> TPE_R {
        TPE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - MAC Transmit Frame Controller Status"]
    #[inline(always)]
    pub fn tfc(&self) -> TFC_R {
        TFC_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 19 - MAC Transmitter PAUSE"]
    #[inline(always)]
    pub fn txpaused(&self) -> TXPAUSED_R {
        TXPAUSED_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - TX/RX Controller's TX FIFO Read Controller Status"]
    #[inline(always)]
    pub fn trc(&self) -> TRC_R {
        TRC_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 22 - TX/RX Controller TX FIFO Write Controller Active Status"]
    #[inline(always)]
    pub fn twc(&self) -> TWC_R {
        TWC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TX/RX Controller TX FIFO Not Empty Status"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TX/RX Controller TX FIFO Full Status"]
    #[inline(always)]
    pub fn txff(&self) -> TXFF_R {
        TXFF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
