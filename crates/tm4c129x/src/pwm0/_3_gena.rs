#[doc = "Reader of register _3_GENA"]
pub type R = crate::R<u32, super::_3_GENA>;
#[doc = "Writer for register _3_GENA"]
pub type W = crate::W<u32, super::_3_GENA>;
#[doc = "Register _3_GENA `reset()`'s with value 0"]
impl crate::ResetValue for super::_3_GENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Action for Counter=0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTZERO_A {
    #[doc = "0: Do nothing"]
    NONE,
    #[doc = "1: Invert pwmA"]
    INV,
    #[doc = "2: Drive pwmA Low"]
    ZERO,
    #[doc = "3: Drive pwmA High"]
    ONE,
}
impl From<ACTZERO_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTZERO_A) -> Self {
        match variant {
            ACTZERO_A::NONE => 0,
            ACTZERO_A::INV => 1,
            ACTZERO_A::ZERO => 2,
            ACTZERO_A::ONE => 3,
        }
    }
}
#[doc = "Reader of field `ACTZERO`"]
pub type ACTZERO_R = crate::R<u8, ACTZERO_A>;
impl ACTZERO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTZERO_A {
        match self.bits {
            0 => ACTZERO_A::NONE,
            1 => ACTZERO_A::INV,
            2 => ACTZERO_A::ZERO,
            3 => ACTZERO_A::ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ACTZERO_A::NONE
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == ACTZERO_A::INV
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == ACTZERO_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == ACTZERO_A::ONE
    }
}
#[doc = "Write proxy for field `ACTZERO`"]
pub struct ACTZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTZERO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACTZERO_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ACTZERO_A::NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(ACTZERO_A::INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(ACTZERO_A::ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(ACTZERO_A::ONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Action for Counter=LOAD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTLOAD_A {
    #[doc = "0: Do nothing"]
    NONE,
    #[doc = "1: Invert pwmA"]
    INV,
    #[doc = "2: Drive pwmA Low"]
    ZERO,
    #[doc = "3: Drive pwmA High"]
    ONE,
}
impl From<ACTLOAD_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTLOAD_A) -> Self {
        match variant {
            ACTLOAD_A::NONE => 0,
            ACTLOAD_A::INV => 1,
            ACTLOAD_A::ZERO => 2,
            ACTLOAD_A::ONE => 3,
        }
    }
}
#[doc = "Reader of field `ACTLOAD`"]
pub type ACTLOAD_R = crate::R<u8, ACTLOAD_A>;
impl ACTLOAD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTLOAD_A {
        match self.bits {
            0 => ACTLOAD_A::NONE,
            1 => ACTLOAD_A::INV,
            2 => ACTLOAD_A::ZERO,
            3 => ACTLOAD_A::ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ACTLOAD_A::NONE
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == ACTLOAD_A::INV
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == ACTLOAD_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == ACTLOAD_A::ONE
    }
}
#[doc = "Write proxy for field `ACTLOAD`"]
pub struct ACTLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTLOAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACTLOAD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ACTLOAD_A::NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(ACTLOAD_A::INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(ACTLOAD_A::ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(ACTLOAD_A::ONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Action for Comparator A Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTCMPAU_A {
    #[doc = "0: Do nothing"]
    NONE,
    #[doc = "1: Invert pwmA"]
    INV,
    #[doc = "2: Drive pwmA Low"]
    ZERO,
    #[doc = "3: Drive pwmA High"]
    ONE,
}
impl From<ACTCMPAU_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTCMPAU_A) -> Self {
        match variant {
            ACTCMPAU_A::NONE => 0,
            ACTCMPAU_A::INV => 1,
            ACTCMPAU_A::ZERO => 2,
            ACTCMPAU_A::ONE => 3,
        }
    }
}
#[doc = "Reader of field `ACTCMPAU`"]
pub type ACTCMPAU_R = crate::R<u8, ACTCMPAU_A>;
impl ACTCMPAU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTCMPAU_A {
        match self.bits {
            0 => ACTCMPAU_A::NONE,
            1 => ACTCMPAU_A::INV,
            2 => ACTCMPAU_A::ZERO,
            3 => ACTCMPAU_A::ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ACTCMPAU_A::NONE
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == ACTCMPAU_A::INV
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == ACTCMPAU_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == ACTCMPAU_A::ONE
    }
}
#[doc = "Write proxy for field `ACTCMPAU`"]
pub struct ACTCMPAU_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTCMPAU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACTCMPAU_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ACTCMPAU_A::NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(ACTCMPAU_A::INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(ACTCMPAU_A::ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(ACTCMPAU_A::ONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Action for Comparator A Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTCMPAD_A {
    #[doc = "0: Do nothing"]
    NONE,
    #[doc = "1: Invert pwmA"]
    INV,
    #[doc = "2: Drive pwmA Low"]
    ZERO,
    #[doc = "3: Drive pwmA High"]
    ONE,
}
impl From<ACTCMPAD_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTCMPAD_A) -> Self {
        match variant {
            ACTCMPAD_A::NONE => 0,
            ACTCMPAD_A::INV => 1,
            ACTCMPAD_A::ZERO => 2,
            ACTCMPAD_A::ONE => 3,
        }
    }
}
#[doc = "Reader of field `ACTCMPAD`"]
pub type ACTCMPAD_R = crate::R<u8, ACTCMPAD_A>;
impl ACTCMPAD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTCMPAD_A {
        match self.bits {
            0 => ACTCMPAD_A::NONE,
            1 => ACTCMPAD_A::INV,
            2 => ACTCMPAD_A::ZERO,
            3 => ACTCMPAD_A::ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ACTCMPAD_A::NONE
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == ACTCMPAD_A::INV
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == ACTCMPAD_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == ACTCMPAD_A::ONE
    }
}
#[doc = "Write proxy for field `ACTCMPAD`"]
pub struct ACTCMPAD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTCMPAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACTCMPAD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ACTCMPAD_A::NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(ACTCMPAD_A::INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(ACTCMPAD_A::ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(ACTCMPAD_A::ONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Action for Comparator B Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTCMPBU_A {
    #[doc = "0: Do nothing"]
    NONE,
    #[doc = "1: Invert pwmA"]
    INV,
    #[doc = "2: Drive pwmA Low"]
    ZERO,
    #[doc = "3: Drive pwmA High"]
    ONE,
}
impl From<ACTCMPBU_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTCMPBU_A) -> Self {
        match variant {
            ACTCMPBU_A::NONE => 0,
            ACTCMPBU_A::INV => 1,
            ACTCMPBU_A::ZERO => 2,
            ACTCMPBU_A::ONE => 3,
        }
    }
}
#[doc = "Reader of field `ACTCMPBU`"]
pub type ACTCMPBU_R = crate::R<u8, ACTCMPBU_A>;
impl ACTCMPBU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTCMPBU_A {
        match self.bits {
            0 => ACTCMPBU_A::NONE,
            1 => ACTCMPBU_A::INV,
            2 => ACTCMPBU_A::ZERO,
            3 => ACTCMPBU_A::ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ACTCMPBU_A::NONE
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == ACTCMPBU_A::INV
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == ACTCMPBU_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == ACTCMPBU_A::ONE
    }
}
#[doc = "Write proxy for field `ACTCMPBU`"]
pub struct ACTCMPBU_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTCMPBU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACTCMPBU_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ACTCMPBU_A::NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(ACTCMPBU_A::INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(ACTCMPBU_A::ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(ACTCMPBU_A::ONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Action for Comparator B Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTCMPBD_A {
    #[doc = "0: Do nothing"]
    NONE,
    #[doc = "1: Invert pwmA"]
    INV,
    #[doc = "2: Drive pwmA Low"]
    ZERO,
    #[doc = "3: Drive pwmA High"]
    ONE,
}
impl From<ACTCMPBD_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTCMPBD_A) -> Self {
        match variant {
            ACTCMPBD_A::NONE => 0,
            ACTCMPBD_A::INV => 1,
            ACTCMPBD_A::ZERO => 2,
            ACTCMPBD_A::ONE => 3,
        }
    }
}
#[doc = "Reader of field `ACTCMPBD`"]
pub type ACTCMPBD_R = crate::R<u8, ACTCMPBD_A>;
impl ACTCMPBD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTCMPBD_A {
        match self.bits {
            0 => ACTCMPBD_A::NONE,
            1 => ACTCMPBD_A::INV,
            2 => ACTCMPBD_A::ZERO,
            3 => ACTCMPBD_A::ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ACTCMPBD_A::NONE
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == ACTCMPBD_A::INV
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == ACTCMPBD_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == ACTCMPBD_A::ONE
    }
}
#[doc = "Write proxy for field `ACTCMPBD`"]
pub struct ACTCMPBD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTCMPBD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACTCMPBD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ACTCMPBD_A::NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(ACTCMPBD_A::INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(ACTCMPBD_A::ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(ACTCMPBD_A::ONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Action for Counter=0"]
    #[inline(always)]
    pub fn actzero(&self) -> ACTZERO_R {
        ACTZERO_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Action for Counter=LOAD"]
    #[inline(always)]
    pub fn actload(&self) -> ACTLOAD_R {
        ACTLOAD_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Action for Comparator A Up"]
    #[inline(always)]
    pub fn actcmpau(&self) -> ACTCMPAU_R {
        ACTCMPAU_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Action for Comparator A Down"]
    #[inline(always)]
    pub fn actcmpad(&self) -> ACTCMPAD_R {
        ACTCMPAD_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Action for Comparator B Up"]
    #[inline(always)]
    pub fn actcmpbu(&self) -> ACTCMPBU_R {
        ACTCMPBU_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Action for Comparator B Down"]
    #[inline(always)]
    pub fn actcmpbd(&self) -> ACTCMPBD_R {
        ACTCMPBD_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Action for Counter=0"]
    #[inline(always)]
    pub fn actzero(&mut self) -> ACTZERO_W {
        ACTZERO_W { w: self }
    }
    #[doc = "Bits 2:3 - Action for Counter=LOAD"]
    #[inline(always)]
    pub fn actload(&mut self) -> ACTLOAD_W {
        ACTLOAD_W { w: self }
    }
    #[doc = "Bits 4:5 - Action for Comparator A Up"]
    #[inline(always)]
    pub fn actcmpau(&mut self) -> ACTCMPAU_W {
        ACTCMPAU_W { w: self }
    }
    #[doc = "Bits 6:7 - Action for Comparator A Down"]
    #[inline(always)]
    pub fn actcmpad(&mut self) -> ACTCMPAD_W {
        ACTCMPAD_W { w: self }
    }
    #[doc = "Bits 8:9 - Action for Comparator B Up"]
    #[inline(always)]
    pub fn actcmpbu(&mut self) -> ACTCMPBU_W {
        ACTCMPBU_W { w: self }
    }
    #[doc = "Bits 10:11 - Action for Comparator B Down"]
    #[inline(always)]
    pub fn actcmpbd(&mut self) -> ACTCMPBD_W {
        ACTCMPBD_W { w: self }
    }
}
