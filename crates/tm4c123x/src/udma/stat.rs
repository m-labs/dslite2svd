#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Reader of field `MASTEN`"]
pub type MASTEN_R = crate::R<bool, bool>;
#[doc = "Control State Machine Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATE_A {
    #[doc = "0: Idle"]
    IDLE,
    #[doc = "1: Reading channel controller data"]
    RD_CTRL,
    #[doc = "2: Reading source end pointer"]
    RD_SRCENDP,
    #[doc = "3: Reading destination end pointer"]
    RD_DSTENDP,
    #[doc = "4: Reading source data"]
    RD_SRCDAT,
    #[doc = "5: Writing destination data"]
    WR_DSTDAT,
    #[doc = "6: Waiting for uDMA request to clear"]
    WAIT,
    #[doc = "7: Writing channel controller data"]
    WR_CTRL,
    #[doc = "8: Stalled"]
    STALL,
    #[doc = "9: Done"]
    DONE,
    #[doc = "10: Undefined"]
    UNDEF,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        match variant {
            STATE_A::IDLE => 0,
            STATE_A::RD_CTRL => 1,
            STATE_A::RD_SRCENDP => 2,
            STATE_A::RD_DSTENDP => 3,
            STATE_A::RD_SRCDAT => 4,
            STATE_A::WR_DSTDAT => 5,
            STATE_A::WAIT => 6,
            STATE_A::WR_CTRL => 7,
            STATE_A::STALL => 8,
            STATE_A::DONE => 9,
            STATE_A::UNDEF => 10,
        }
    }
}
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, STATE_A>;
impl STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STATE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STATE_A::IDLE),
            1 => Val(STATE_A::RD_CTRL),
            2 => Val(STATE_A::RD_SRCENDP),
            3 => Val(STATE_A::RD_DSTENDP),
            4 => Val(STATE_A::RD_SRCDAT),
            5 => Val(STATE_A::WR_DSTDAT),
            6 => Val(STATE_A::WAIT),
            7 => Val(STATE_A::WR_CTRL),
            8 => Val(STATE_A::STALL),
            9 => Val(STATE_A::DONE),
            10 => Val(STATE_A::UNDEF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == STATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `RD_CTRL`"]
    #[inline(always)]
    pub fn is_rd_ctrl(&self) -> bool {
        *self == STATE_A::RD_CTRL
    }
    #[doc = "Checks if the value of the field is `RD_SRCENDP`"]
    #[inline(always)]
    pub fn is_rd_srcendp(&self) -> bool {
        *self == STATE_A::RD_SRCENDP
    }
    #[doc = "Checks if the value of the field is `RD_DSTENDP`"]
    #[inline(always)]
    pub fn is_rd_dstendp(&self) -> bool {
        *self == STATE_A::RD_DSTENDP
    }
    #[doc = "Checks if the value of the field is `RD_SRCDAT`"]
    #[inline(always)]
    pub fn is_rd_srcdat(&self) -> bool {
        *self == STATE_A::RD_SRCDAT
    }
    #[doc = "Checks if the value of the field is `WR_DSTDAT`"]
    #[inline(always)]
    pub fn is_wr_dstdat(&self) -> bool {
        *self == STATE_A::WR_DSTDAT
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == STATE_A::WAIT
    }
    #[doc = "Checks if the value of the field is `WR_CTRL`"]
    #[inline(always)]
    pub fn is_wr_ctrl(&self) -> bool {
        *self == STATE_A::WR_CTRL
    }
    #[doc = "Checks if the value of the field is `STALL`"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == STATE_A::STALL
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == STATE_A::DONE
    }
    #[doc = "Checks if the value of the field is `UNDEF`"]
    #[inline(always)]
    pub fn is_undef(&self) -> bool {
        *self == STATE_A::UNDEF
    }
}
#[doc = "Reader of field `DMACHANS`"]
pub type DMACHANS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Master Enable Status"]
    #[inline(always)]
    pub fn masten(&self) -> MASTEN_R {
        MASTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Control State Machine Status"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Available uDMA Channels Minus 1"]
    #[inline(always)]
    pub fn dmachans(&self) -> DMACHANS_R {
        DMACHANS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
