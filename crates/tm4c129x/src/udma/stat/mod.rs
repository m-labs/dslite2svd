#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct MASTENR {
    bits: bool,
}
impl MASTENR {
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
#[doc = "Possible values of the field `STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATER {
    #[doc = "Idle"]
    IDLE,
    #[doc = "Reading channel controller data"]
    RD_CTRL,
    #[doc = "Reading source end pointer"]
    RD_SRCENDP,
    #[doc = "Reading destination end pointer"]
    RD_DSTENDP,
    #[doc = "Reading source data"]
    RD_SRCDAT,
    #[doc = "Writing destination data"]
    WR_DSTDAT,
    #[doc = "Waiting for uDMA request to clear"]
    WAIT,
    #[doc = "Writing channel controller data"]
    WR_CTRL,
    #[doc = "Stalled"]
    STALL,
    #[doc = "Done"]
    DONE,
    #[doc = "Undefined"]
    UNDEF,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STATER::IDLE => 0,
            STATER::RD_CTRL => 1,
            STATER::RD_SRCENDP => 2,
            STATER::RD_DSTENDP => 3,
            STATER::RD_SRCDAT => 4,
            STATER::WR_DSTDAT => 5,
            STATER::WAIT => 6,
            STATER::WR_CTRL => 7,
            STATER::STALL => 8,
            STATER::DONE => 9,
            STATER::UNDEF => 10,
            STATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STATER {
        match value {
            0 => STATER::IDLE,
            1 => STATER::RD_CTRL,
            2 => STATER::RD_SRCENDP,
            3 => STATER::RD_DSTENDP,
            4 => STATER::RD_SRCDAT,
            5 => STATER::WR_DSTDAT,
            6 => STATER::WAIT,
            7 => STATER::WR_CTRL,
            8 => STATER::STALL,
            9 => STATER::DONE,
            10 => STATER::UNDEF,
            i => STATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == STATER::IDLE
    }
    #[doc = "Checks if the value of the field is `RD_CTRL`"]
    #[inline]
    pub fn is_rd_ctrl(&self) -> bool {
        *self == STATER::RD_CTRL
    }
    #[doc = "Checks if the value of the field is `RD_SRCENDP`"]
    #[inline]
    pub fn is_rd_srcendp(&self) -> bool {
        *self == STATER::RD_SRCENDP
    }
    #[doc = "Checks if the value of the field is `RD_DSTENDP`"]
    #[inline]
    pub fn is_rd_dstendp(&self) -> bool {
        *self == STATER::RD_DSTENDP
    }
    #[doc = "Checks if the value of the field is `RD_SRCDAT`"]
    #[inline]
    pub fn is_rd_srcdat(&self) -> bool {
        *self == STATER::RD_SRCDAT
    }
    #[doc = "Checks if the value of the field is `WR_DSTDAT`"]
    #[inline]
    pub fn is_wr_dstdat(&self) -> bool {
        *self == STATER::WR_DSTDAT
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline]
    pub fn is_wait(&self) -> bool {
        *self == STATER::WAIT
    }
    #[doc = "Checks if the value of the field is `WR_CTRL`"]
    #[inline]
    pub fn is_wr_ctrl(&self) -> bool {
        *self == STATER::WR_CTRL
    }
    #[doc = "Checks if the value of the field is `STALL`"]
    #[inline]
    pub fn is_stall(&self) -> bool {
        *self == STATER::STALL
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline]
    pub fn is_done(&self) -> bool {
        *self == STATER::DONE
    }
    #[doc = "Checks if the value of the field is `UNDEF`"]
    #[inline]
    pub fn is_undef(&self) -> bool {
        *self == STATER::UNDEF
    }
}
#[doc = r" Value of the field"]
pub struct DMACHANSR {
    bits: u8,
}
impl DMACHANSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Master Enable Status"]
    #[inline]
    pub fn masten(&self) -> MASTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MASTENR { bits }
    }
    #[doc = "Bits 4:7 - Control State Machine Status"]
    #[inline]
    pub fn state(&self) -> STATER {
        STATER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:20 - Available uDMA Channels Minus 1"]
    #[inline]
    pub fn dmachans(&self) -> DMACHANSR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DMACHANSR { bits }
    }
}
