#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::_3_GENB {
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
#[doc = "Possible values of the field `ACTZERO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTZEROR {
    #[doc = "Do nothing"]
    NONE,
    #[doc = "Invert pwmB"]
    INV,
    #[doc = "Drive pwmB Low"]
    ZERO,
    #[doc = "Drive pwmB High"]
    ONE,
}
impl ACTZEROR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACTZEROR::NONE => 0,
            ACTZEROR::INV => 1,
            ACTZEROR::ZERO => 2,
            ACTZEROR::ONE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACTZEROR {
        match value {
            0 => ACTZEROR::NONE,
            1 => ACTZEROR::INV,
            2 => ACTZEROR::ZERO,
            3 => ACTZEROR::ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ACTZEROR::NONE
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == ACTZEROR::INV
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == ACTZEROR::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == ACTZEROR::ONE
    }
}
#[doc = "Possible values of the field `ACTLOAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTLOADR {
    #[doc = "Do nothing"]
    NONE,
    #[doc = "Invert pwmB"]
    INV,
    #[doc = "Drive pwmB Low"]
    ZERO,
    #[doc = "Drive pwmB High"]
    ONE,
}
impl ACTLOADR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACTLOADR::NONE => 0,
            ACTLOADR::INV => 1,
            ACTLOADR::ZERO => 2,
            ACTLOADR::ONE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACTLOADR {
        match value {
            0 => ACTLOADR::NONE,
            1 => ACTLOADR::INV,
            2 => ACTLOADR::ZERO,
            3 => ACTLOADR::ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ACTLOADR::NONE
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == ACTLOADR::INV
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == ACTLOADR::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == ACTLOADR::ONE
    }
}
#[doc = "Possible values of the field `ACTCMPAU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTCMPAUR {
    #[doc = "Do nothing"]
    NONE,
    #[doc = "Invert pwmB"]
    INV,
    #[doc = "Drive pwmB Low"]
    ZERO,
    #[doc = "Drive pwmB High"]
    ONE,
}
impl ACTCMPAUR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACTCMPAUR::NONE => 0,
            ACTCMPAUR::INV => 1,
            ACTCMPAUR::ZERO => 2,
            ACTCMPAUR::ONE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACTCMPAUR {
        match value {
            0 => ACTCMPAUR::NONE,
            1 => ACTCMPAUR::INV,
            2 => ACTCMPAUR::ZERO,
            3 => ACTCMPAUR::ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ACTCMPAUR::NONE
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == ACTCMPAUR::INV
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == ACTCMPAUR::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == ACTCMPAUR::ONE
    }
}
#[doc = "Possible values of the field `ACTCMPAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTCMPADR {
    #[doc = "Do nothing"]
    NONE,
    #[doc = "Invert pwmB"]
    INV,
    #[doc = "Drive pwmB Low"]
    ZERO,
    #[doc = "Drive pwmB High"]
    ONE,
}
impl ACTCMPADR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACTCMPADR::NONE => 0,
            ACTCMPADR::INV => 1,
            ACTCMPADR::ZERO => 2,
            ACTCMPADR::ONE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACTCMPADR {
        match value {
            0 => ACTCMPADR::NONE,
            1 => ACTCMPADR::INV,
            2 => ACTCMPADR::ZERO,
            3 => ACTCMPADR::ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ACTCMPADR::NONE
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == ACTCMPADR::INV
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == ACTCMPADR::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == ACTCMPADR::ONE
    }
}
#[doc = "Possible values of the field `ACTCMPBU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTCMPBUR {
    #[doc = "Do nothing"]
    NONE,
    #[doc = "Invert pwmB"]
    INV,
    #[doc = "Drive pwmB Low"]
    ZERO,
    #[doc = "Drive pwmB High"]
    ONE,
}
impl ACTCMPBUR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACTCMPBUR::NONE => 0,
            ACTCMPBUR::INV => 1,
            ACTCMPBUR::ZERO => 2,
            ACTCMPBUR::ONE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACTCMPBUR {
        match value {
            0 => ACTCMPBUR::NONE,
            1 => ACTCMPBUR::INV,
            2 => ACTCMPBUR::ZERO,
            3 => ACTCMPBUR::ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ACTCMPBUR::NONE
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == ACTCMPBUR::INV
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == ACTCMPBUR::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == ACTCMPBUR::ONE
    }
}
#[doc = "Possible values of the field `ACTCMPBD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTCMPBDR {
    #[doc = "Do nothing"]
    NONE,
    #[doc = "Invert pwmB"]
    INV,
    #[doc = "Drive pwmB Low"]
    ZERO,
    #[doc = "Drive pwmB High"]
    ONE,
}
impl ACTCMPBDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACTCMPBDR::NONE => 0,
            ACTCMPBDR::INV => 1,
            ACTCMPBDR::ZERO => 2,
            ACTCMPBDR::ONE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACTCMPBDR {
        match value {
            0 => ACTCMPBDR::NONE,
            1 => ACTCMPBDR::INV,
            2 => ACTCMPBDR::ZERO,
            3 => ACTCMPBDR::ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ACTCMPBDR::NONE
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == ACTCMPBDR::INV
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == ACTCMPBDR::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == ACTCMPBDR::ONE
    }
}
#[doc = "Values that can be written to the field `ACTZERO`"]
pub enum ACTZEROW {
    #[doc = "Do nothing"]
    NONE,
    #[doc = "Invert pwmB"]
    INV,
    #[doc = "Drive pwmB Low"]
    ZERO,
    #[doc = "Drive pwmB High"]
    ONE,
}
impl ACTZEROW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACTZEROW::NONE => 0,
            ACTZEROW::INV => 1,
            ACTZEROW::ZERO => 2,
            ACTZEROW::ONE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACTZEROW<'a> {
    w: &'a mut W,
}
impl<'a> _ACTZEROW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACTZEROW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do nothing"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(ACTZEROW::NONE)
    }
    #[doc = "Invert pwmB"]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(ACTZEROW::INV)
    }
    #[doc = "Drive pwmB Low"]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(ACTZEROW::ZERO)
    }
    #[doc = "Drive pwmB High"]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(ACTZEROW::ONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACTLOAD`"]
pub enum ACTLOADW {
    #[doc = "Do nothing"]
    NONE,
    #[doc = "Invert pwmB"]
    INV,
    #[doc = "Drive pwmB Low"]
    ZERO,
    #[doc = "Drive pwmB High"]
    ONE,
}
impl ACTLOADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACTLOADW::NONE => 0,
            ACTLOADW::INV => 1,
            ACTLOADW::ZERO => 2,
            ACTLOADW::ONE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACTLOADW<'a> {
    w: &'a mut W,
}
impl<'a> _ACTLOADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACTLOADW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do nothing"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(ACTLOADW::NONE)
    }
    #[doc = "Invert pwmB"]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(ACTLOADW::INV)
    }
    #[doc = "Drive pwmB Low"]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(ACTLOADW::ZERO)
    }
    #[doc = "Drive pwmB High"]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(ACTLOADW::ONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACTCMPAU`"]
pub enum ACTCMPAUW {
    #[doc = "Do nothing"]
    NONE,
    #[doc = "Invert pwmB"]
    INV,
    #[doc = "Drive pwmB Low"]
    ZERO,
    #[doc = "Drive pwmB High"]
    ONE,
}
impl ACTCMPAUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACTCMPAUW::NONE => 0,
            ACTCMPAUW::INV => 1,
            ACTCMPAUW::ZERO => 2,
            ACTCMPAUW::ONE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACTCMPAUW<'a> {
    w: &'a mut W,
}
impl<'a> _ACTCMPAUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACTCMPAUW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do nothing"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(ACTCMPAUW::NONE)
    }
    #[doc = "Invert pwmB"]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(ACTCMPAUW::INV)
    }
    #[doc = "Drive pwmB Low"]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(ACTCMPAUW::ZERO)
    }
    #[doc = "Drive pwmB High"]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(ACTCMPAUW::ONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACTCMPAD`"]
pub enum ACTCMPADW {
    #[doc = "Do nothing"]
    NONE,
    #[doc = "Invert pwmB"]
    INV,
    #[doc = "Drive pwmB Low"]
    ZERO,
    #[doc = "Drive pwmB High"]
    ONE,
}
impl ACTCMPADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACTCMPADW::NONE => 0,
            ACTCMPADW::INV => 1,
            ACTCMPADW::ZERO => 2,
            ACTCMPADW::ONE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACTCMPADW<'a> {
    w: &'a mut W,
}
impl<'a> _ACTCMPADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACTCMPADW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do nothing"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(ACTCMPADW::NONE)
    }
    #[doc = "Invert pwmB"]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(ACTCMPADW::INV)
    }
    #[doc = "Drive pwmB Low"]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(ACTCMPADW::ZERO)
    }
    #[doc = "Drive pwmB High"]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(ACTCMPADW::ONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACTCMPBU`"]
pub enum ACTCMPBUW {
    #[doc = "Do nothing"]
    NONE,
    #[doc = "Invert pwmB"]
    INV,
    #[doc = "Drive pwmB Low"]
    ZERO,
    #[doc = "Drive pwmB High"]
    ONE,
}
impl ACTCMPBUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACTCMPBUW::NONE => 0,
            ACTCMPBUW::INV => 1,
            ACTCMPBUW::ZERO => 2,
            ACTCMPBUW::ONE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACTCMPBUW<'a> {
    w: &'a mut W,
}
impl<'a> _ACTCMPBUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACTCMPBUW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do nothing"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(ACTCMPBUW::NONE)
    }
    #[doc = "Invert pwmB"]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(ACTCMPBUW::INV)
    }
    #[doc = "Drive pwmB Low"]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(ACTCMPBUW::ZERO)
    }
    #[doc = "Drive pwmB High"]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(ACTCMPBUW::ONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACTCMPBD`"]
pub enum ACTCMPBDW {
    #[doc = "Do nothing"]
    NONE,
    #[doc = "Invert pwmB"]
    INV,
    #[doc = "Drive pwmB Low"]
    ZERO,
    #[doc = "Drive pwmB High"]
    ONE,
}
impl ACTCMPBDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACTCMPBDW::NONE => 0,
            ACTCMPBDW::INV => 1,
            ACTCMPBDW::ZERO => 2,
            ACTCMPBDW::ONE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACTCMPBDW<'a> {
    w: &'a mut W,
}
impl<'a> _ACTCMPBDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACTCMPBDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do nothing"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(ACTCMPBDW::NONE)
    }
    #[doc = "Invert pwmB"]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(ACTCMPBDW::INV)
    }
    #[doc = "Drive pwmB Low"]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(ACTCMPBDW::ZERO)
    }
    #[doc = "Drive pwmB High"]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(ACTCMPBDW::ONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
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
    #[doc = "Bits 0:1 - Action for Counter=0"]
    #[inline]
    pub fn actzero(&self) -> ACTZEROR {
        ACTZEROR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Action for Counter=LOAD"]
    #[inline]
    pub fn actload(&self) -> ACTLOADR {
        ACTLOADR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Action for Comparator A Up"]
    #[inline]
    pub fn actcmpau(&self) -> ACTCMPAUR {
        ACTCMPAUR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Action for Comparator A Down"]
    #[inline]
    pub fn actcmpad(&self) -> ACTCMPADR {
        ACTCMPADR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Action for Comparator B Up"]
    #[inline]
    pub fn actcmpbu(&self) -> ACTCMPBUR {
        ACTCMPBUR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Action for Comparator B Down"]
    #[inline]
    pub fn actcmpbd(&self) -> ACTCMPBDR {
        ACTCMPBDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:1 - Action for Counter=0"]
    #[inline]
    pub fn actzero(&mut self) -> _ACTZEROW {
        _ACTZEROW { w: self }
    }
    #[doc = "Bits 2:3 - Action for Counter=LOAD"]
    #[inline]
    pub fn actload(&mut self) -> _ACTLOADW {
        _ACTLOADW { w: self }
    }
    #[doc = "Bits 4:5 - Action for Comparator A Up"]
    #[inline]
    pub fn actcmpau(&mut self) -> _ACTCMPAUW {
        _ACTCMPAUW { w: self }
    }
    #[doc = "Bits 6:7 - Action for Comparator A Down"]
    #[inline]
    pub fn actcmpad(&mut self) -> _ACTCMPADW {
        _ACTCMPADW { w: self }
    }
    #[doc = "Bits 8:9 - Action for Comparator B Up"]
    #[inline]
    pub fn actcmpbu(&mut self) -> _ACTCMPBUW {
        _ACTCMPBUW { w: self }
    }
    #[doc = "Bits 10:11 - Action for Comparator B Down"]
    #[inline]
    pub fn actcmpbd(&mut self) -> _ACTCMPBDW {
        _ACTCMPBDW { w: self }
    }
}
