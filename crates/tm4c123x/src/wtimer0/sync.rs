#[doc = "Reader of register SYNC"]
pub type R = crate::R<u32, super::SYNC>;
#[doc = "Writer for register SYNC"]
pub type W = crate::W<u32, super::SYNC>;
#[doc = "Register SYNC `reset()`'s with value 0"]
impl crate::ResetValue for super::SYNC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Synchronize GPTM Timer 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNCT0_A {
    #[doc = "0: GPTM0 is not affected"]
    NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM0 is triggered"]
    TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM0 is triggered"]
    TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM0 is triggered"]
    TATB = 3,
}
impl From<SYNCT0_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCT0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNCT0`"]
pub type SYNCT0_R = crate::R<u8, SYNCT0_A>;
impl SYNCT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCT0_A {
        match self.bits {
            0 => SYNCT0_A::NONE,
            1 => SYNCT0_A::TA,
            2 => SYNCT0_A::TB,
            3 => SYNCT0_A::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SYNCT0_A::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline(always)]
    pub fn is_ta(&self) -> bool {
        *self == SYNCT0_A::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline(always)]
    pub fn is_tb(&self) -> bool {
        *self == SYNCT0_A::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline(always)]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCT0_A::TATB
    }
}
#[doc = "Write proxy for field `SYNCT0`"]
pub struct SYNCT0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCT0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPTM0 is not affected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCT0_A::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM0 is triggered"]
    #[inline(always)]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCT0_A::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM0 is triggered"]
    #[inline(always)]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCT0_A::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM0 is triggered"]
    #[inline(always)]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCT0_A::TATB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Synchronize GPTM Timer 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNCT1_A {
    #[doc = "0: GPTM1 is not affected"]
    NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM1 is triggered"]
    TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM1 is triggered"]
    TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM1 is triggered"]
    TATB = 3,
}
impl From<SYNCT1_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCT1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNCT1`"]
pub type SYNCT1_R = crate::R<u8, SYNCT1_A>;
impl SYNCT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCT1_A {
        match self.bits {
            0 => SYNCT1_A::NONE,
            1 => SYNCT1_A::TA,
            2 => SYNCT1_A::TB,
            3 => SYNCT1_A::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SYNCT1_A::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline(always)]
    pub fn is_ta(&self) -> bool {
        *self == SYNCT1_A::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline(always)]
    pub fn is_tb(&self) -> bool {
        *self == SYNCT1_A::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline(always)]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCT1_A::TATB
    }
}
#[doc = "Write proxy for field `SYNCT1`"]
pub struct SYNCT1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCT1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPTM1 is not affected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCT1_A::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM1 is triggered"]
    #[inline(always)]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCT1_A::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM1 is triggered"]
    #[inline(always)]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCT1_A::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM1 is triggered"]
    #[inline(always)]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCT1_A::TATB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Synchronize GPTM Timer 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNCT2_A {
    #[doc = "0: GPTM2 is not affected"]
    NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM2 is triggered"]
    TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM2 is triggered"]
    TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM2 is triggered"]
    TATB = 3,
}
impl From<SYNCT2_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCT2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNCT2`"]
pub type SYNCT2_R = crate::R<u8, SYNCT2_A>;
impl SYNCT2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCT2_A {
        match self.bits {
            0 => SYNCT2_A::NONE,
            1 => SYNCT2_A::TA,
            2 => SYNCT2_A::TB,
            3 => SYNCT2_A::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SYNCT2_A::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline(always)]
    pub fn is_ta(&self) -> bool {
        *self == SYNCT2_A::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline(always)]
    pub fn is_tb(&self) -> bool {
        *self == SYNCT2_A::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline(always)]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCT2_A::TATB
    }
}
#[doc = "Write proxy for field `SYNCT2`"]
pub struct SYNCT2_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCT2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPTM2 is not affected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCT2_A::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM2 is triggered"]
    #[inline(always)]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCT2_A::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM2 is triggered"]
    #[inline(always)]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCT2_A::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM2 is triggered"]
    #[inline(always)]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCT2_A::TATB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Synchronize GPTM Timer 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNCT3_A {
    #[doc = "0: GPTM3 is not affected"]
    NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM3 is triggered"]
    TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM3 is triggered"]
    TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM3 is triggered"]
    TATB = 3,
}
impl From<SYNCT3_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCT3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNCT3`"]
pub type SYNCT3_R = crate::R<u8, SYNCT3_A>;
impl SYNCT3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCT3_A {
        match self.bits {
            0 => SYNCT3_A::NONE,
            1 => SYNCT3_A::TA,
            2 => SYNCT3_A::TB,
            3 => SYNCT3_A::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SYNCT3_A::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline(always)]
    pub fn is_ta(&self) -> bool {
        *self == SYNCT3_A::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline(always)]
    pub fn is_tb(&self) -> bool {
        *self == SYNCT3_A::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline(always)]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCT3_A::TATB
    }
}
#[doc = "Write proxy for field `SYNCT3`"]
pub struct SYNCT3_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCT3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPTM3 is not affected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCT3_A::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM3 is triggered"]
    #[inline(always)]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCT3_A::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM3 is triggered"]
    #[inline(always)]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCT3_A::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM3 is triggered"]
    #[inline(always)]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCT3_A::TATB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Synchronize GPTM Timer 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNCT4_A {
    #[doc = "0: GPTM4 is not affected"]
    NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM4 is triggered"]
    TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM4 is triggered"]
    TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM4 is triggered"]
    TATB = 3,
}
impl From<SYNCT4_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCT4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNCT4`"]
pub type SYNCT4_R = crate::R<u8, SYNCT4_A>;
impl SYNCT4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCT4_A {
        match self.bits {
            0 => SYNCT4_A::NONE,
            1 => SYNCT4_A::TA,
            2 => SYNCT4_A::TB,
            3 => SYNCT4_A::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SYNCT4_A::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline(always)]
    pub fn is_ta(&self) -> bool {
        *self == SYNCT4_A::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline(always)]
    pub fn is_tb(&self) -> bool {
        *self == SYNCT4_A::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline(always)]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCT4_A::TATB
    }
}
#[doc = "Write proxy for field `SYNCT4`"]
pub struct SYNCT4_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCT4_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPTM4 is not affected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCT4_A::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM4 is triggered"]
    #[inline(always)]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCT4_A::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM4 is triggered"]
    #[inline(always)]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCT4_A::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM4 is triggered"]
    #[inline(always)]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCT4_A::TATB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Synchronize GPTM Timer 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNCT5_A {
    #[doc = "0: GPTM5 is not affected"]
    NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM5 is triggered"]
    TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM5 is triggered"]
    TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM5 is triggered"]
    TATB = 3,
}
impl From<SYNCT5_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCT5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNCT5`"]
pub type SYNCT5_R = crate::R<u8, SYNCT5_A>;
impl SYNCT5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCT5_A {
        match self.bits {
            0 => SYNCT5_A::NONE,
            1 => SYNCT5_A::TA,
            2 => SYNCT5_A::TB,
            3 => SYNCT5_A::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SYNCT5_A::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline(always)]
    pub fn is_ta(&self) -> bool {
        *self == SYNCT5_A::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline(always)]
    pub fn is_tb(&self) -> bool {
        *self == SYNCT5_A::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline(always)]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCT5_A::TATB
    }
}
#[doc = "Write proxy for field `SYNCT5`"]
pub struct SYNCT5_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCT5_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPTM5 is not affected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCT5_A::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM5 is triggered"]
    #[inline(always)]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCT5_A::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM5 is triggered"]
    #[inline(always)]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCT5_A::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM5 is triggered"]
    #[inline(always)]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCT5_A::TATB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Synchronize GPTM 32/64-Bit Timer 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNCWT0_A {
    #[doc = "0: GPTM 32/64-Bit Timer 0 is not affected"]
    NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM 32/64-Bit Timer 0 is triggered"]
    TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM 32/64-Bit Timer 0 is triggered"]
    TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 0 is triggered"]
    TATB = 3,
}
impl From<SYNCWT0_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCWT0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNCWT0`"]
pub type SYNCWT0_R = crate::R<u8, SYNCWT0_A>;
impl SYNCWT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCWT0_A {
        match self.bits {
            0 => SYNCWT0_A::NONE,
            1 => SYNCWT0_A::TA,
            2 => SYNCWT0_A::TB,
            3 => SYNCWT0_A::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SYNCWT0_A::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline(always)]
    pub fn is_ta(&self) -> bool {
        *self == SYNCWT0_A::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline(always)]
    pub fn is_tb(&self) -> bool {
        *self == SYNCWT0_A::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline(always)]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCWT0_A::TATB
    }
}
#[doc = "Write proxy for field `SYNCWT0`"]
pub struct SYNCWT0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCWT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCWT0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPTM 32/64-Bit Timer 0 is not affected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCWT0_A::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 0 is triggered"]
    #[inline(always)]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCWT0_A::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 0 is triggered"]
    #[inline(always)]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCWT0_A::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 0 is triggered"]
    #[inline(always)]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCWT0_A::TATB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Synchronize GPTM 32/64-Bit Timer 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNCWT1_A {
    #[doc = "0: GPTM 32/64-Bit Timer 1 is not affected"]
    NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM 32/64-Bit Timer 1 is triggered"]
    TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM 32/64-Bit Timer 1 is triggered"]
    TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 1 is triggered"]
    TATB = 3,
}
impl From<SYNCWT1_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCWT1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNCWT1`"]
pub type SYNCWT1_R = crate::R<u8, SYNCWT1_A>;
impl SYNCWT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCWT1_A {
        match self.bits {
            0 => SYNCWT1_A::NONE,
            1 => SYNCWT1_A::TA,
            2 => SYNCWT1_A::TB,
            3 => SYNCWT1_A::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SYNCWT1_A::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline(always)]
    pub fn is_ta(&self) -> bool {
        *self == SYNCWT1_A::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline(always)]
    pub fn is_tb(&self) -> bool {
        *self == SYNCWT1_A::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline(always)]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCWT1_A::TATB
    }
}
#[doc = "Write proxy for field `SYNCWT1`"]
pub struct SYNCWT1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCWT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCWT1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPTM 32/64-Bit Timer 1 is not affected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCWT1_A::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 1 is triggered"]
    #[inline(always)]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCWT1_A::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 1 is triggered"]
    #[inline(always)]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCWT1_A::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 1 is triggered"]
    #[inline(always)]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCWT1_A::TATB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Synchronize GPTM 32/64-Bit Timer 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNCWT2_A {
    #[doc = "0: GPTM 32/64-Bit Timer 2 is not affected"]
    NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM 32/64-Bit Timer 2 is triggered"]
    TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM 32/64-Bit Timer 2 is triggered"]
    TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 2 is triggered"]
    TATB = 3,
}
impl From<SYNCWT2_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCWT2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNCWT2`"]
pub type SYNCWT2_R = crate::R<u8, SYNCWT2_A>;
impl SYNCWT2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCWT2_A {
        match self.bits {
            0 => SYNCWT2_A::NONE,
            1 => SYNCWT2_A::TA,
            2 => SYNCWT2_A::TB,
            3 => SYNCWT2_A::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SYNCWT2_A::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline(always)]
    pub fn is_ta(&self) -> bool {
        *self == SYNCWT2_A::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline(always)]
    pub fn is_tb(&self) -> bool {
        *self == SYNCWT2_A::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline(always)]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCWT2_A::TATB
    }
}
#[doc = "Write proxy for field `SYNCWT2`"]
pub struct SYNCWT2_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCWT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCWT2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPTM 32/64-Bit Timer 2 is not affected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCWT2_A::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 2 is triggered"]
    #[inline(always)]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCWT2_A::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 2 is triggered"]
    #[inline(always)]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCWT2_A::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 2 is triggered"]
    #[inline(always)]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCWT2_A::TATB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Synchronize GPTM 32/64-Bit Timer 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNCWT3_A {
    #[doc = "0: GPTM 32/64-Bit Timer 3 is not affected"]
    NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM 32/64-Bit Timer 3 is triggered"]
    TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM 32/64-Bit Timer 3 is triggered"]
    TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 3 is triggered"]
    TATB = 3,
}
impl From<SYNCWT3_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCWT3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNCWT3`"]
pub type SYNCWT3_R = crate::R<u8, SYNCWT3_A>;
impl SYNCWT3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCWT3_A {
        match self.bits {
            0 => SYNCWT3_A::NONE,
            1 => SYNCWT3_A::TA,
            2 => SYNCWT3_A::TB,
            3 => SYNCWT3_A::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SYNCWT3_A::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline(always)]
    pub fn is_ta(&self) -> bool {
        *self == SYNCWT3_A::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline(always)]
    pub fn is_tb(&self) -> bool {
        *self == SYNCWT3_A::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline(always)]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCWT3_A::TATB
    }
}
#[doc = "Write proxy for field `SYNCWT3`"]
pub struct SYNCWT3_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCWT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCWT3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPTM 32/64-Bit Timer 3 is not affected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCWT3_A::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 3 is triggered"]
    #[inline(always)]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCWT3_A::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 3 is triggered"]
    #[inline(always)]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCWT3_A::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 3 is triggered"]
    #[inline(always)]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCWT3_A::TATB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Synchronize GPTM 32/64-Bit Timer 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNCWT4_A {
    #[doc = "0: GPTM 32/64-Bit Timer 4 is not affected"]
    NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM 32/64-Bit Timer 4 is triggered"]
    TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM 32/64-Bit Timer 4 is triggered"]
    TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 4 is triggered"]
    TATB = 3,
}
impl From<SYNCWT4_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCWT4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNCWT4`"]
pub type SYNCWT4_R = crate::R<u8, SYNCWT4_A>;
impl SYNCWT4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCWT4_A {
        match self.bits {
            0 => SYNCWT4_A::NONE,
            1 => SYNCWT4_A::TA,
            2 => SYNCWT4_A::TB,
            3 => SYNCWT4_A::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SYNCWT4_A::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline(always)]
    pub fn is_ta(&self) -> bool {
        *self == SYNCWT4_A::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline(always)]
    pub fn is_tb(&self) -> bool {
        *self == SYNCWT4_A::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline(always)]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCWT4_A::TATB
    }
}
#[doc = "Write proxy for field `SYNCWT4`"]
pub struct SYNCWT4_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCWT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCWT4_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPTM 32/64-Bit Timer 4 is not affected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCWT4_A::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 4 is triggered"]
    #[inline(always)]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCWT4_A::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 4 is triggered"]
    #[inline(always)]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCWT4_A::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 4 is triggered"]
    #[inline(always)]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCWT4_A::TATB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Synchronize GPTM 32/64-Bit Timer 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNCWT5_A {
    #[doc = "0: GPTM 32/64-Bit Timer 5 is not affected"]
    NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM 32/64-Bit Timer 5 is triggered"]
    TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM 32/64-Bit Timer 5 is triggered"]
    TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 5 is triggered"]
    TATB = 3,
}
impl From<SYNCWT5_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCWT5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNCWT5`"]
pub type SYNCWT5_R = crate::R<u8, SYNCWT5_A>;
impl SYNCWT5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCWT5_A {
        match self.bits {
            0 => SYNCWT5_A::NONE,
            1 => SYNCWT5_A::TA,
            2 => SYNCWT5_A::TB,
            3 => SYNCWT5_A::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SYNCWT5_A::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline(always)]
    pub fn is_ta(&self) -> bool {
        *self == SYNCWT5_A::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline(always)]
    pub fn is_tb(&self) -> bool {
        *self == SYNCWT5_A::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline(always)]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCWT5_A::TATB
    }
}
#[doc = "Write proxy for field `SYNCWT5`"]
pub struct SYNCWT5_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCWT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCWT5_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPTM 32/64-Bit Timer 5 is not affected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCWT5_A::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 5 is triggered"]
    #[inline(always)]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCWT5_A::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 5 is triggered"]
    #[inline(always)]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCWT5_A::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 5 is triggered"]
    #[inline(always)]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCWT5_A::TATB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Synchronize GPTM Timer 0"]
    #[inline(always)]
    pub fn synct0(&self) -> SYNCT0_R {
        SYNCT0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Synchronize GPTM Timer 1"]
    #[inline(always)]
    pub fn synct1(&self) -> SYNCT1_R {
        SYNCT1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Synchronize GPTM Timer 2"]
    #[inline(always)]
    pub fn synct2(&self) -> SYNCT2_R {
        SYNCT2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Synchronize GPTM Timer 3"]
    #[inline(always)]
    pub fn synct3(&self) -> SYNCT3_R {
        SYNCT3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Synchronize GPTM Timer 4"]
    #[inline(always)]
    pub fn synct4(&self) -> SYNCT4_R {
        SYNCT4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Synchronize GPTM Timer 5"]
    #[inline(always)]
    pub fn synct5(&self) -> SYNCT5_R {
        SYNCT5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Synchronize GPTM 32/64-Bit Timer 0"]
    #[inline(always)]
    pub fn syncwt0(&self) -> SYNCWT0_R {
        SYNCWT0_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Synchronize GPTM 32/64-Bit Timer 1"]
    #[inline(always)]
    pub fn syncwt1(&self) -> SYNCWT1_R {
        SYNCWT1_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Synchronize GPTM 32/64-Bit Timer 2"]
    #[inline(always)]
    pub fn syncwt2(&self) -> SYNCWT2_R {
        SYNCWT2_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Synchronize GPTM 32/64-Bit Timer 3"]
    #[inline(always)]
    pub fn syncwt3(&self) -> SYNCWT3_R {
        SYNCWT3_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Synchronize GPTM 32/64-Bit Timer 4"]
    #[inline(always)]
    pub fn syncwt4(&self) -> SYNCWT4_R {
        SYNCWT4_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Synchronize GPTM 32/64-Bit Timer 5"]
    #[inline(always)]
    pub fn syncwt5(&self) -> SYNCWT5_R {
        SYNCWT5_R::new(((self.bits >> 22) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Synchronize GPTM Timer 0"]
    #[inline(always)]
    pub fn synct0(&mut self) -> SYNCT0_W {
        SYNCT0_W { w: self }
    }
    #[doc = "Bits 2:3 - Synchronize GPTM Timer 1"]
    #[inline(always)]
    pub fn synct1(&mut self) -> SYNCT1_W {
        SYNCT1_W { w: self }
    }
    #[doc = "Bits 4:5 - Synchronize GPTM Timer 2"]
    #[inline(always)]
    pub fn synct2(&mut self) -> SYNCT2_W {
        SYNCT2_W { w: self }
    }
    #[doc = "Bits 6:7 - Synchronize GPTM Timer 3"]
    #[inline(always)]
    pub fn synct3(&mut self) -> SYNCT3_W {
        SYNCT3_W { w: self }
    }
    #[doc = "Bits 8:9 - Synchronize GPTM Timer 4"]
    #[inline(always)]
    pub fn synct4(&mut self) -> SYNCT4_W {
        SYNCT4_W { w: self }
    }
    #[doc = "Bits 10:11 - Synchronize GPTM Timer 5"]
    #[inline(always)]
    pub fn synct5(&mut self) -> SYNCT5_W {
        SYNCT5_W { w: self }
    }
    #[doc = "Bits 12:13 - Synchronize GPTM 32/64-Bit Timer 0"]
    #[inline(always)]
    pub fn syncwt0(&mut self) -> SYNCWT0_W {
        SYNCWT0_W { w: self }
    }
    #[doc = "Bits 14:15 - Synchronize GPTM 32/64-Bit Timer 1"]
    #[inline(always)]
    pub fn syncwt1(&mut self) -> SYNCWT1_W {
        SYNCWT1_W { w: self }
    }
    #[doc = "Bits 16:17 - Synchronize GPTM 32/64-Bit Timer 2"]
    #[inline(always)]
    pub fn syncwt2(&mut self) -> SYNCWT2_W {
        SYNCWT2_W { w: self }
    }
    #[doc = "Bits 18:19 - Synchronize GPTM 32/64-Bit Timer 3"]
    #[inline(always)]
    pub fn syncwt3(&mut self) -> SYNCWT3_W {
        SYNCWT3_W { w: self }
    }
    #[doc = "Bits 20:21 - Synchronize GPTM 32/64-Bit Timer 4"]
    #[inline(always)]
    pub fn syncwt4(&mut self) -> SYNCWT4_W {
        SYNCWT4_W { w: self }
    }
    #[doc = "Bits 22:23 - Synchronize GPTM 32/64-Bit Timer 5"]
    #[inline(always)]
    pub fn syncwt5(&mut self) -> SYNCWT5_W {
        SYNCWT5_W { w: self }
    }
}
