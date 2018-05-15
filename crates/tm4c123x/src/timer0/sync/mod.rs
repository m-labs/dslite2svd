#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYNC {
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
#[doc = "Possible values of the field `SYNCT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCT0R {
    #[doc = "GPTM0 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM0 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM0 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM0 is triggered"]
    TATB,
}
impl SYNCT0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNCT0R::NONE => 0,
            SYNCT0R::TA => 1,
            SYNCT0R::TB => 2,
            SYNCT0R::TATB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNCT0R {
        match value {
            0 => SYNCT0R::NONE,
            1 => SYNCT0R::TA,
            2 => SYNCT0R::TB,
            3 => SYNCT0R::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SYNCT0R::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline]
    pub fn is_ta(&self) -> bool {
        *self == SYNCT0R::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline]
    pub fn is_tb(&self) -> bool {
        *self == SYNCT0R::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCT0R::TATB
    }
}
#[doc = "Possible values of the field `SYNCT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCT1R {
    #[doc = "GPTM1 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM1 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM1 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM1 is triggered"]
    TATB,
}
impl SYNCT1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNCT1R::NONE => 0,
            SYNCT1R::TA => 1,
            SYNCT1R::TB => 2,
            SYNCT1R::TATB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNCT1R {
        match value {
            0 => SYNCT1R::NONE,
            1 => SYNCT1R::TA,
            2 => SYNCT1R::TB,
            3 => SYNCT1R::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SYNCT1R::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline]
    pub fn is_ta(&self) -> bool {
        *self == SYNCT1R::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline]
    pub fn is_tb(&self) -> bool {
        *self == SYNCT1R::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCT1R::TATB
    }
}
#[doc = "Possible values of the field `SYNCT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCT2R {
    #[doc = "GPTM2 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM2 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM2 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM2 is triggered"]
    TATB,
}
impl SYNCT2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNCT2R::NONE => 0,
            SYNCT2R::TA => 1,
            SYNCT2R::TB => 2,
            SYNCT2R::TATB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNCT2R {
        match value {
            0 => SYNCT2R::NONE,
            1 => SYNCT2R::TA,
            2 => SYNCT2R::TB,
            3 => SYNCT2R::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SYNCT2R::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline]
    pub fn is_ta(&self) -> bool {
        *self == SYNCT2R::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline]
    pub fn is_tb(&self) -> bool {
        *self == SYNCT2R::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCT2R::TATB
    }
}
#[doc = "Possible values of the field `SYNCT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCT3R {
    #[doc = "GPTM3 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM3 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM3 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM3 is triggered"]
    TATB,
}
impl SYNCT3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNCT3R::NONE => 0,
            SYNCT3R::TA => 1,
            SYNCT3R::TB => 2,
            SYNCT3R::TATB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNCT3R {
        match value {
            0 => SYNCT3R::NONE,
            1 => SYNCT3R::TA,
            2 => SYNCT3R::TB,
            3 => SYNCT3R::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SYNCT3R::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline]
    pub fn is_ta(&self) -> bool {
        *self == SYNCT3R::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline]
    pub fn is_tb(&self) -> bool {
        *self == SYNCT3R::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCT3R::TATB
    }
}
#[doc = "Possible values of the field `SYNCT4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCT4R {
    #[doc = "GPTM4 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM4 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM4 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM4 is triggered"]
    TATB,
}
impl SYNCT4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNCT4R::NONE => 0,
            SYNCT4R::TA => 1,
            SYNCT4R::TB => 2,
            SYNCT4R::TATB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNCT4R {
        match value {
            0 => SYNCT4R::NONE,
            1 => SYNCT4R::TA,
            2 => SYNCT4R::TB,
            3 => SYNCT4R::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SYNCT4R::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline]
    pub fn is_ta(&self) -> bool {
        *self == SYNCT4R::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline]
    pub fn is_tb(&self) -> bool {
        *self == SYNCT4R::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCT4R::TATB
    }
}
#[doc = "Possible values of the field `SYNCT5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCT5R {
    #[doc = "GPTM5 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM5 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM5 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM5 is triggered"]
    TATB,
}
impl SYNCT5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNCT5R::NONE => 0,
            SYNCT5R::TA => 1,
            SYNCT5R::TB => 2,
            SYNCT5R::TATB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNCT5R {
        match value {
            0 => SYNCT5R::NONE,
            1 => SYNCT5R::TA,
            2 => SYNCT5R::TB,
            3 => SYNCT5R::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SYNCT5R::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline]
    pub fn is_ta(&self) -> bool {
        *self == SYNCT5R::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline]
    pub fn is_tb(&self) -> bool {
        *self == SYNCT5R::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCT5R::TATB
    }
}
#[doc = "Possible values of the field `SYNCWT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCWT0R {
    #[doc = "GPTM 32/64-Bit Timer 0 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 0 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 0 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 0 is triggered"]
    TATB,
}
impl SYNCWT0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNCWT0R::NONE => 0,
            SYNCWT0R::TA => 1,
            SYNCWT0R::TB => 2,
            SYNCWT0R::TATB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNCWT0R {
        match value {
            0 => SYNCWT0R::NONE,
            1 => SYNCWT0R::TA,
            2 => SYNCWT0R::TB,
            3 => SYNCWT0R::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SYNCWT0R::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline]
    pub fn is_ta(&self) -> bool {
        *self == SYNCWT0R::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline]
    pub fn is_tb(&self) -> bool {
        *self == SYNCWT0R::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCWT0R::TATB
    }
}
#[doc = "Possible values of the field `SYNCWT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCWT1R {
    #[doc = "GPTM 32/64-Bit Timer 1 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 1 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 1 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 1 is triggered"]
    TATB,
}
impl SYNCWT1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNCWT1R::NONE => 0,
            SYNCWT1R::TA => 1,
            SYNCWT1R::TB => 2,
            SYNCWT1R::TATB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNCWT1R {
        match value {
            0 => SYNCWT1R::NONE,
            1 => SYNCWT1R::TA,
            2 => SYNCWT1R::TB,
            3 => SYNCWT1R::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SYNCWT1R::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline]
    pub fn is_ta(&self) -> bool {
        *self == SYNCWT1R::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline]
    pub fn is_tb(&self) -> bool {
        *self == SYNCWT1R::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCWT1R::TATB
    }
}
#[doc = "Possible values of the field `SYNCWT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCWT2R {
    #[doc = "GPTM 32/64-Bit Timer 2 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 2 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 2 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 2 is triggered"]
    TATB,
}
impl SYNCWT2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNCWT2R::NONE => 0,
            SYNCWT2R::TA => 1,
            SYNCWT2R::TB => 2,
            SYNCWT2R::TATB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNCWT2R {
        match value {
            0 => SYNCWT2R::NONE,
            1 => SYNCWT2R::TA,
            2 => SYNCWT2R::TB,
            3 => SYNCWT2R::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SYNCWT2R::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline]
    pub fn is_ta(&self) -> bool {
        *self == SYNCWT2R::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline]
    pub fn is_tb(&self) -> bool {
        *self == SYNCWT2R::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCWT2R::TATB
    }
}
#[doc = "Possible values of the field `SYNCWT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCWT3R {
    #[doc = "GPTM 32/64-Bit Timer 3 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 3 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 3 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 3 is triggered"]
    TATB,
}
impl SYNCWT3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNCWT3R::NONE => 0,
            SYNCWT3R::TA => 1,
            SYNCWT3R::TB => 2,
            SYNCWT3R::TATB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNCWT3R {
        match value {
            0 => SYNCWT3R::NONE,
            1 => SYNCWT3R::TA,
            2 => SYNCWT3R::TB,
            3 => SYNCWT3R::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SYNCWT3R::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline]
    pub fn is_ta(&self) -> bool {
        *self == SYNCWT3R::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline]
    pub fn is_tb(&self) -> bool {
        *self == SYNCWT3R::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCWT3R::TATB
    }
}
#[doc = "Possible values of the field `SYNCWT4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCWT4R {
    #[doc = "GPTM 32/64-Bit Timer 4 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 4 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 4 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 4 is triggered"]
    TATB,
}
impl SYNCWT4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNCWT4R::NONE => 0,
            SYNCWT4R::TA => 1,
            SYNCWT4R::TB => 2,
            SYNCWT4R::TATB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNCWT4R {
        match value {
            0 => SYNCWT4R::NONE,
            1 => SYNCWT4R::TA,
            2 => SYNCWT4R::TB,
            3 => SYNCWT4R::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SYNCWT4R::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline]
    pub fn is_ta(&self) -> bool {
        *self == SYNCWT4R::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline]
    pub fn is_tb(&self) -> bool {
        *self == SYNCWT4R::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCWT4R::TATB
    }
}
#[doc = "Possible values of the field `SYNCWT5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCWT5R {
    #[doc = "GPTM 32/64-Bit Timer 5 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 5 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 5 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 5 is triggered"]
    TATB,
}
impl SYNCWT5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNCWT5R::NONE => 0,
            SYNCWT5R::TA => 1,
            SYNCWT5R::TB => 2,
            SYNCWT5R::TATB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNCWT5R {
        match value {
            0 => SYNCWT5R::NONE,
            1 => SYNCWT5R::TA,
            2 => SYNCWT5R::TB,
            3 => SYNCWT5R::TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SYNCWT5R::NONE
    }
    #[doc = "Checks if the value of the field is `TA`"]
    #[inline]
    pub fn is_ta(&self) -> bool {
        *self == SYNCWT5R::TA
    }
    #[doc = "Checks if the value of the field is `TB`"]
    #[inline]
    pub fn is_tb(&self) -> bool {
        *self == SYNCWT5R::TB
    }
    #[doc = "Checks if the value of the field is `TATB`"]
    #[inline]
    pub fn is_tatb(&self) -> bool {
        *self == SYNCWT5R::TATB
    }
}
#[doc = "Values that can be written to the field `SYNCT0`"]
pub enum SYNCT0W {
    #[doc = "GPTM0 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM0 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM0 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM0 is triggered"]
    TATB,
}
impl SYNCT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNCT0W::NONE => 0,
            SYNCT0W::TA => 1,
            SYNCT0W::TB => 2,
            SYNCT0W::TATB => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCT0W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCT0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPTM0 is not affected"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCT0W::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM0 is triggered"]
    #[inline]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCT0W::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM0 is triggered"]
    #[inline]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCT0W::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM0 is triggered"]
    #[inline]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCT0W::TATB)
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
#[doc = "Values that can be written to the field `SYNCT1`"]
pub enum SYNCT1W {
    #[doc = "GPTM1 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM1 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM1 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM1 is triggered"]
    TATB,
}
impl SYNCT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNCT1W::NONE => 0,
            SYNCT1W::TA => 1,
            SYNCT1W::TB => 2,
            SYNCT1W::TATB => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCT1W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCT1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPTM1 is not affected"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCT1W::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM1 is triggered"]
    #[inline]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCT1W::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM1 is triggered"]
    #[inline]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCT1W::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM1 is triggered"]
    #[inline]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCT1W::TATB)
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
#[doc = "Values that can be written to the field `SYNCT2`"]
pub enum SYNCT2W {
    #[doc = "GPTM2 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM2 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM2 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM2 is triggered"]
    TATB,
}
impl SYNCT2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNCT2W::NONE => 0,
            SYNCT2W::TA => 1,
            SYNCT2W::TB => 2,
            SYNCT2W::TATB => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCT2W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCT2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCT2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPTM2 is not affected"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCT2W::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM2 is triggered"]
    #[inline]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCT2W::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM2 is triggered"]
    #[inline]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCT2W::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM2 is triggered"]
    #[inline]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCT2W::TATB)
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
#[doc = "Values that can be written to the field `SYNCT3`"]
pub enum SYNCT3W {
    #[doc = "GPTM3 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM3 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM3 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM3 is triggered"]
    TATB,
}
impl SYNCT3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNCT3W::NONE => 0,
            SYNCT3W::TA => 1,
            SYNCT3W::TB => 2,
            SYNCT3W::TATB => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCT3W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCT3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCT3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPTM3 is not affected"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCT3W::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM3 is triggered"]
    #[inline]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCT3W::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM3 is triggered"]
    #[inline]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCT3W::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM3 is triggered"]
    #[inline]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCT3W::TATB)
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
#[doc = "Values that can be written to the field `SYNCT4`"]
pub enum SYNCT4W {
    #[doc = "GPTM4 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM4 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM4 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM4 is triggered"]
    TATB,
}
impl SYNCT4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNCT4W::NONE => 0,
            SYNCT4W::TA => 1,
            SYNCT4W::TB => 2,
            SYNCT4W::TATB => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCT4W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCT4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCT4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPTM4 is not affected"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCT4W::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM4 is triggered"]
    #[inline]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCT4W::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM4 is triggered"]
    #[inline]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCT4W::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM4 is triggered"]
    #[inline]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCT4W::TATB)
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
#[doc = "Values that can be written to the field `SYNCT5`"]
pub enum SYNCT5W {
    #[doc = "GPTM5 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM5 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM5 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM5 is triggered"]
    TATB,
}
impl SYNCT5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNCT5W::NONE => 0,
            SYNCT5W::TA => 1,
            SYNCT5W::TB => 2,
            SYNCT5W::TATB => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCT5W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCT5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCT5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPTM5 is not affected"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCT5W::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM5 is triggered"]
    #[inline]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCT5W::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM5 is triggered"]
    #[inline]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCT5W::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM5 is triggered"]
    #[inline]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCT5W::TATB)
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
#[doc = "Values that can be written to the field `SYNCWT0`"]
pub enum SYNCWT0W {
    #[doc = "GPTM 32/64-Bit Timer 0 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 0 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 0 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 0 is triggered"]
    TATB,
}
impl SYNCWT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNCWT0W::NONE => 0,
            SYNCWT0W::TA => 1,
            SYNCWT0W::TB => 2,
            SYNCWT0W::TATB => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCWT0W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCWT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCWT0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPTM 32/64-Bit Timer 0 is not affected"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCWT0W::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 0 is triggered"]
    #[inline]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCWT0W::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 0 is triggered"]
    #[inline]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCWT0W::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 0 is triggered"]
    #[inline]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCWT0W::TATB)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNCWT1`"]
pub enum SYNCWT1W {
    #[doc = "GPTM 32/64-Bit Timer 1 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 1 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 1 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 1 is triggered"]
    TATB,
}
impl SYNCWT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNCWT1W::NONE => 0,
            SYNCWT1W::TA => 1,
            SYNCWT1W::TB => 2,
            SYNCWT1W::TATB => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCWT1W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCWT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCWT1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPTM 32/64-Bit Timer 1 is not affected"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCWT1W::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 1 is triggered"]
    #[inline]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCWT1W::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 1 is triggered"]
    #[inline]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCWT1W::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 1 is triggered"]
    #[inline]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCWT1W::TATB)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNCWT2`"]
pub enum SYNCWT2W {
    #[doc = "GPTM 32/64-Bit Timer 2 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 2 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 2 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 2 is triggered"]
    TATB,
}
impl SYNCWT2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNCWT2W::NONE => 0,
            SYNCWT2W::TA => 1,
            SYNCWT2W::TB => 2,
            SYNCWT2W::TATB => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCWT2W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCWT2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCWT2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPTM 32/64-Bit Timer 2 is not affected"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCWT2W::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 2 is triggered"]
    #[inline]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCWT2W::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 2 is triggered"]
    #[inline]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCWT2W::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 2 is triggered"]
    #[inline]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCWT2W::TATB)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNCWT3`"]
pub enum SYNCWT3W {
    #[doc = "GPTM 32/64-Bit Timer 3 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 3 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 3 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 3 is triggered"]
    TATB,
}
impl SYNCWT3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNCWT3W::NONE => 0,
            SYNCWT3W::TA => 1,
            SYNCWT3W::TB => 2,
            SYNCWT3W::TATB => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCWT3W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCWT3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCWT3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPTM 32/64-Bit Timer 3 is not affected"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCWT3W::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 3 is triggered"]
    #[inline]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCWT3W::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 3 is triggered"]
    #[inline]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCWT3W::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 3 is triggered"]
    #[inline]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCWT3W::TATB)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNCWT4`"]
pub enum SYNCWT4W {
    #[doc = "GPTM 32/64-Bit Timer 4 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 4 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 4 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 4 is triggered"]
    TATB,
}
impl SYNCWT4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNCWT4W::NONE => 0,
            SYNCWT4W::TA => 1,
            SYNCWT4W::TB => 2,
            SYNCWT4W::TATB => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCWT4W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCWT4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCWT4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPTM 32/64-Bit Timer 4 is not affected"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCWT4W::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 4 is triggered"]
    #[inline]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCWT4W::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 4 is triggered"]
    #[inline]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCWT4W::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 4 is triggered"]
    #[inline]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCWT4W::TATB)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNCWT5`"]
pub enum SYNCWT5W {
    #[doc = "GPTM 32/64-Bit Timer 5 is not affected"]
    NONE,
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 5 is triggered"]
    TA,
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 5 is triggered"]
    TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 5 is triggered"]
    TATB,
}
impl SYNCWT5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNCWT5W::NONE => 0,
            SYNCWT5W::TA => 1,
            SYNCWT5W::TB => 2,
            SYNCWT5W::TATB => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCWT5W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCWT5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCWT5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPTM 32/64-Bit Timer 5 is not affected"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SYNCWT5W::NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 5 is triggered"]
    #[inline]
    pub fn ta(self) -> &'a mut W {
        self.variant(SYNCWT5W::TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 5 is triggered"]
    #[inline]
    pub fn tb(self) -> &'a mut W {
        self.variant(SYNCWT5W::TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 5 is triggered"]
    #[inline]
    pub fn tatb(self) -> &'a mut W {
        self.variant(SYNCWT5W::TATB)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
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
    #[doc = "Bits 0:1 - Synchronize GPTM Timer 0"]
    #[inline]
    pub fn synct0(&self) -> SYNCT0R {
        SYNCT0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Synchronize GPTM Timer 1"]
    #[inline]
    pub fn synct1(&self) -> SYNCT1R {
        SYNCT1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Synchronize GPTM Timer 2"]
    #[inline]
    pub fn synct2(&self) -> SYNCT2R {
        SYNCT2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Synchronize GPTM Timer 3"]
    #[inline]
    pub fn synct3(&self) -> SYNCT3R {
        SYNCT3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Synchronize GPTM Timer 4"]
    #[inline]
    pub fn synct4(&self) -> SYNCT4R {
        SYNCT4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Synchronize GPTM Timer 5"]
    #[inline]
    pub fn synct5(&self) -> SYNCT5R {
        SYNCT5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Synchronize GPTM 32/64-Bit Timer 0"]
    #[inline]
    pub fn syncwt0(&self) -> SYNCWT0R {
        SYNCWT0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Synchronize GPTM 32/64-Bit Timer 1"]
    #[inline]
    pub fn syncwt1(&self) -> SYNCWT1R {
        SYNCWT1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Synchronize GPTM 32/64-Bit Timer 2"]
    #[inline]
    pub fn syncwt2(&self) -> SYNCWT2R {
        SYNCWT2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Synchronize GPTM 32/64-Bit Timer 3"]
    #[inline]
    pub fn syncwt3(&self) -> SYNCWT3R {
        SYNCWT3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Synchronize GPTM 32/64-Bit Timer 4"]
    #[inline]
    pub fn syncwt4(&self) -> SYNCWT4R {
        SYNCWT4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Synchronize GPTM 32/64-Bit Timer 5"]
    #[inline]
    pub fn syncwt5(&self) -> SYNCWT5R {
        SYNCWT5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
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
    #[doc = "Bits 0:1 - Synchronize GPTM Timer 0"]
    #[inline]
    pub fn synct0(&mut self) -> _SYNCT0W {
        _SYNCT0W { w: self }
    }
    #[doc = "Bits 2:3 - Synchronize GPTM Timer 1"]
    #[inline]
    pub fn synct1(&mut self) -> _SYNCT1W {
        _SYNCT1W { w: self }
    }
    #[doc = "Bits 4:5 - Synchronize GPTM Timer 2"]
    #[inline]
    pub fn synct2(&mut self) -> _SYNCT2W {
        _SYNCT2W { w: self }
    }
    #[doc = "Bits 6:7 - Synchronize GPTM Timer 3"]
    #[inline]
    pub fn synct3(&mut self) -> _SYNCT3W {
        _SYNCT3W { w: self }
    }
    #[doc = "Bits 8:9 - Synchronize GPTM Timer 4"]
    #[inline]
    pub fn synct4(&mut self) -> _SYNCT4W {
        _SYNCT4W { w: self }
    }
    #[doc = "Bits 10:11 - Synchronize GPTM Timer 5"]
    #[inline]
    pub fn synct5(&mut self) -> _SYNCT5W {
        _SYNCT5W { w: self }
    }
    #[doc = "Bits 12:13 - Synchronize GPTM 32/64-Bit Timer 0"]
    #[inline]
    pub fn syncwt0(&mut self) -> _SYNCWT0W {
        _SYNCWT0W { w: self }
    }
    #[doc = "Bits 14:15 - Synchronize GPTM 32/64-Bit Timer 1"]
    #[inline]
    pub fn syncwt1(&mut self) -> _SYNCWT1W {
        _SYNCWT1W { w: self }
    }
    #[doc = "Bits 16:17 - Synchronize GPTM 32/64-Bit Timer 2"]
    #[inline]
    pub fn syncwt2(&mut self) -> _SYNCWT2W {
        _SYNCWT2W { w: self }
    }
    #[doc = "Bits 18:19 - Synchronize GPTM 32/64-Bit Timer 3"]
    #[inline]
    pub fn syncwt3(&mut self) -> _SYNCWT3W {
        _SYNCWT3W { w: self }
    }
    #[doc = "Bits 20:21 - Synchronize GPTM 32/64-Bit Timer 4"]
    #[inline]
    pub fn syncwt4(&mut self) -> _SYNCWT4W {
        _SYNCWT4W { w: self }
    }
    #[doc = "Bits 22:23 - Synchronize GPTM 32/64-Bit Timer 5"]
    #[inline]
    pub fn syncwt5(&mut self) -> _SYNCWT5W {
        _SYNCWT5W { w: self }
    }
}
