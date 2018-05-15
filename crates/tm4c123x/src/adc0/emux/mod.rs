#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EMUX {
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
#[doc = "Possible values of the field `EM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM0R {
    #[doc = "Processor (default)"]
    PROCESSOR,
    #[doc = "Analog Comparator 0"]
    COMP0,
    #[doc = "Analog Comparator 1"]
    COMP1,
    #[doc = "External (GPIO Pins)"]
    EXTERNAL,
    #[doc = "Timer"]
    TIMER,
    #[doc = "PWM generator 0"]
    PWM0,
    #[doc = "PWM generator 1"]
    PWM1,
    #[doc = "PWM generator 2"]
    PWM2,
    #[doc = "PWM generator 3"]
    PWM3,
    #[doc = "Always (continuously sample)"]
    ALWAYS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EM0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EM0R::PROCESSOR => 0,
            EM0R::COMP0 => 1,
            EM0R::COMP1 => 2,
            EM0R::EXTERNAL => 4,
            EM0R::TIMER => 5,
            EM0R::PWM0 => 6,
            EM0R::PWM1 => 7,
            EM0R::PWM2 => 8,
            EM0R::PWM3 => 9,
            EM0R::ALWAYS => 15,
            EM0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EM0R {
        match value {
            0 => EM0R::PROCESSOR,
            1 => EM0R::COMP0,
            2 => EM0R::COMP1,
            4 => EM0R::EXTERNAL,
            5 => EM0R::TIMER,
            6 => EM0R::PWM0,
            7 => EM0R::PWM1,
            8 => EM0R::PWM2,
            9 => EM0R::PWM3,
            15 => EM0R::ALWAYS,
            i => EM0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PROCESSOR`"]
    #[inline]
    pub fn is_processor(&self) -> bool {
        *self == EM0R::PROCESSOR
    }
    #[doc = "Checks if the value of the field is `COMP0`"]
    #[inline]
    pub fn is_comp0(&self) -> bool {
        *self == EM0R::COMP0
    }
    #[doc = "Checks if the value of the field is `COMP1`"]
    #[inline]
    pub fn is_comp1(&self) -> bool {
        *self == EM0R::COMP1
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline]
    pub fn is_external(&self) -> bool {
        *self == EM0R::EXTERNAL
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline]
    pub fn is_timer(&self) -> bool {
        *self == EM0R::TIMER
    }
    #[doc = "Checks if the value of the field is `PWM0`"]
    #[inline]
    pub fn is_pwm0(&self) -> bool {
        *self == EM0R::PWM0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline]
    pub fn is_pwm1(&self) -> bool {
        *self == EM0R::PWM1
    }
    #[doc = "Checks if the value of the field is `PWM2`"]
    #[inline]
    pub fn is_pwm2(&self) -> bool {
        *self == EM0R::PWM2
    }
    #[doc = "Checks if the value of the field is `PWM3`"]
    #[inline]
    pub fn is_pwm3(&self) -> bool {
        *self == EM0R::PWM3
    }
    #[doc = "Checks if the value of the field is `ALWAYS`"]
    #[inline]
    pub fn is_always(&self) -> bool {
        *self == EM0R::ALWAYS
    }
}
#[doc = "Possible values of the field `EM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM1R {
    #[doc = "Processor (default)"]
    PROCESSOR,
    #[doc = "Analog Comparator 0"]
    COMP0,
    #[doc = "Analog Comparator 1"]
    COMP1,
    #[doc = "External (GPIO Pins)"]
    EXTERNAL,
    #[doc = "Timer"]
    TIMER,
    #[doc = "PWM generator 0"]
    PWM0,
    #[doc = "PWM generator 1"]
    PWM1,
    #[doc = "PWM generator 2"]
    PWM2,
    #[doc = "PWM generator 3"]
    PWM3,
    #[doc = "Always (continuously sample)"]
    ALWAYS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EM1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EM1R::PROCESSOR => 0,
            EM1R::COMP0 => 1,
            EM1R::COMP1 => 2,
            EM1R::EXTERNAL => 4,
            EM1R::TIMER => 5,
            EM1R::PWM0 => 6,
            EM1R::PWM1 => 7,
            EM1R::PWM2 => 8,
            EM1R::PWM3 => 9,
            EM1R::ALWAYS => 15,
            EM1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EM1R {
        match value {
            0 => EM1R::PROCESSOR,
            1 => EM1R::COMP0,
            2 => EM1R::COMP1,
            4 => EM1R::EXTERNAL,
            5 => EM1R::TIMER,
            6 => EM1R::PWM0,
            7 => EM1R::PWM1,
            8 => EM1R::PWM2,
            9 => EM1R::PWM3,
            15 => EM1R::ALWAYS,
            i => EM1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PROCESSOR`"]
    #[inline]
    pub fn is_processor(&self) -> bool {
        *self == EM1R::PROCESSOR
    }
    #[doc = "Checks if the value of the field is `COMP0`"]
    #[inline]
    pub fn is_comp0(&self) -> bool {
        *self == EM1R::COMP0
    }
    #[doc = "Checks if the value of the field is `COMP1`"]
    #[inline]
    pub fn is_comp1(&self) -> bool {
        *self == EM1R::COMP1
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline]
    pub fn is_external(&self) -> bool {
        *self == EM1R::EXTERNAL
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline]
    pub fn is_timer(&self) -> bool {
        *self == EM1R::TIMER
    }
    #[doc = "Checks if the value of the field is `PWM0`"]
    #[inline]
    pub fn is_pwm0(&self) -> bool {
        *self == EM1R::PWM0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline]
    pub fn is_pwm1(&self) -> bool {
        *self == EM1R::PWM1
    }
    #[doc = "Checks if the value of the field is `PWM2`"]
    #[inline]
    pub fn is_pwm2(&self) -> bool {
        *self == EM1R::PWM2
    }
    #[doc = "Checks if the value of the field is `PWM3`"]
    #[inline]
    pub fn is_pwm3(&self) -> bool {
        *self == EM1R::PWM3
    }
    #[doc = "Checks if the value of the field is `ALWAYS`"]
    #[inline]
    pub fn is_always(&self) -> bool {
        *self == EM1R::ALWAYS
    }
}
#[doc = "Possible values of the field `EM2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM2R {
    #[doc = "Processor (default)"]
    PROCESSOR,
    #[doc = "Analog Comparator 0"]
    COMP0,
    #[doc = "Analog Comparator 1"]
    COMP1,
    #[doc = "External (GPIO Pins)"]
    EXTERNAL,
    #[doc = "Timer"]
    TIMER,
    #[doc = "PWM generator 0"]
    PWM0,
    #[doc = "PWM generator 1"]
    PWM1,
    #[doc = "PWM generator 2"]
    PWM2,
    #[doc = "PWM generator 3"]
    PWM3,
    #[doc = "Always (continuously sample)"]
    ALWAYS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EM2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EM2R::PROCESSOR => 0,
            EM2R::COMP0 => 1,
            EM2R::COMP1 => 2,
            EM2R::EXTERNAL => 4,
            EM2R::TIMER => 5,
            EM2R::PWM0 => 6,
            EM2R::PWM1 => 7,
            EM2R::PWM2 => 8,
            EM2R::PWM3 => 9,
            EM2R::ALWAYS => 15,
            EM2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EM2R {
        match value {
            0 => EM2R::PROCESSOR,
            1 => EM2R::COMP0,
            2 => EM2R::COMP1,
            4 => EM2R::EXTERNAL,
            5 => EM2R::TIMER,
            6 => EM2R::PWM0,
            7 => EM2R::PWM1,
            8 => EM2R::PWM2,
            9 => EM2R::PWM3,
            15 => EM2R::ALWAYS,
            i => EM2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PROCESSOR`"]
    #[inline]
    pub fn is_processor(&self) -> bool {
        *self == EM2R::PROCESSOR
    }
    #[doc = "Checks if the value of the field is `COMP0`"]
    #[inline]
    pub fn is_comp0(&self) -> bool {
        *self == EM2R::COMP0
    }
    #[doc = "Checks if the value of the field is `COMP1`"]
    #[inline]
    pub fn is_comp1(&self) -> bool {
        *self == EM2R::COMP1
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline]
    pub fn is_external(&self) -> bool {
        *self == EM2R::EXTERNAL
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline]
    pub fn is_timer(&self) -> bool {
        *self == EM2R::TIMER
    }
    #[doc = "Checks if the value of the field is `PWM0`"]
    #[inline]
    pub fn is_pwm0(&self) -> bool {
        *self == EM2R::PWM0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline]
    pub fn is_pwm1(&self) -> bool {
        *self == EM2R::PWM1
    }
    #[doc = "Checks if the value of the field is `PWM2`"]
    #[inline]
    pub fn is_pwm2(&self) -> bool {
        *self == EM2R::PWM2
    }
    #[doc = "Checks if the value of the field is `PWM3`"]
    #[inline]
    pub fn is_pwm3(&self) -> bool {
        *self == EM2R::PWM3
    }
    #[doc = "Checks if the value of the field is `ALWAYS`"]
    #[inline]
    pub fn is_always(&self) -> bool {
        *self == EM2R::ALWAYS
    }
}
#[doc = "Possible values of the field `EM3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM3R {
    #[doc = "Processor (default)"]
    PROCESSOR,
    #[doc = "Analog Comparator 0"]
    COMP0,
    #[doc = "Analog Comparator 1"]
    COMP1,
    #[doc = "External (GPIO Pins)"]
    EXTERNAL,
    #[doc = "Timer"]
    TIMER,
    #[doc = "PWM generator 0"]
    PWM0,
    #[doc = "PWM generator 1"]
    PWM1,
    #[doc = "PWM generator 2"]
    PWM2,
    #[doc = "PWM generator 3"]
    PWM3,
    #[doc = "Always (continuously sample)"]
    ALWAYS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EM3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EM3R::PROCESSOR => 0,
            EM3R::COMP0 => 1,
            EM3R::COMP1 => 2,
            EM3R::EXTERNAL => 4,
            EM3R::TIMER => 5,
            EM3R::PWM0 => 6,
            EM3R::PWM1 => 7,
            EM3R::PWM2 => 8,
            EM3R::PWM3 => 9,
            EM3R::ALWAYS => 15,
            EM3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EM3R {
        match value {
            0 => EM3R::PROCESSOR,
            1 => EM3R::COMP0,
            2 => EM3R::COMP1,
            4 => EM3R::EXTERNAL,
            5 => EM3R::TIMER,
            6 => EM3R::PWM0,
            7 => EM3R::PWM1,
            8 => EM3R::PWM2,
            9 => EM3R::PWM3,
            15 => EM3R::ALWAYS,
            i => EM3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PROCESSOR`"]
    #[inline]
    pub fn is_processor(&self) -> bool {
        *self == EM3R::PROCESSOR
    }
    #[doc = "Checks if the value of the field is `COMP0`"]
    #[inline]
    pub fn is_comp0(&self) -> bool {
        *self == EM3R::COMP0
    }
    #[doc = "Checks if the value of the field is `COMP1`"]
    #[inline]
    pub fn is_comp1(&self) -> bool {
        *self == EM3R::COMP1
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline]
    pub fn is_external(&self) -> bool {
        *self == EM3R::EXTERNAL
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline]
    pub fn is_timer(&self) -> bool {
        *self == EM3R::TIMER
    }
    #[doc = "Checks if the value of the field is `PWM0`"]
    #[inline]
    pub fn is_pwm0(&self) -> bool {
        *self == EM3R::PWM0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline]
    pub fn is_pwm1(&self) -> bool {
        *self == EM3R::PWM1
    }
    #[doc = "Checks if the value of the field is `PWM2`"]
    #[inline]
    pub fn is_pwm2(&self) -> bool {
        *self == EM3R::PWM2
    }
    #[doc = "Checks if the value of the field is `PWM3`"]
    #[inline]
    pub fn is_pwm3(&self) -> bool {
        *self == EM3R::PWM3
    }
    #[doc = "Checks if the value of the field is `ALWAYS`"]
    #[inline]
    pub fn is_always(&self) -> bool {
        *self == EM3R::ALWAYS
    }
}
#[doc = "Values that can be written to the field `EM0`"]
pub enum EM0W {
    #[doc = "Processor (default)"]
    PROCESSOR,
    #[doc = "Analog Comparator 0"]
    COMP0,
    #[doc = "Analog Comparator 1"]
    COMP1,
    #[doc = "External (GPIO Pins)"]
    EXTERNAL,
    #[doc = "Timer"]
    TIMER,
    #[doc = "PWM generator 0"]
    PWM0,
    #[doc = "PWM generator 1"]
    PWM1,
    #[doc = "PWM generator 2"]
    PWM2,
    #[doc = "PWM generator 3"]
    PWM3,
    #[doc = "Always (continuously sample)"]
    ALWAYS,
}
impl EM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EM0W::PROCESSOR => 0,
            EM0W::COMP0 => 1,
            EM0W::COMP1 => 2,
            EM0W::EXTERNAL => 4,
            EM0W::TIMER => 5,
            EM0W::PWM0 => 6,
            EM0W::PWM1 => 7,
            EM0W::PWM2 => 8,
            EM0W::PWM3 => 9,
            EM0W::ALWAYS => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EM0W<'a> {
    w: &'a mut W,
}
impl<'a> _EM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EM0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Processor (default)"]
    #[inline]
    pub fn processor(self) -> &'a mut W {
        self.variant(EM0W::PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline]
    pub fn comp0(self) -> &'a mut W {
        self.variant(EM0W::COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline]
    pub fn comp1(self) -> &'a mut W {
        self.variant(EM0W::COMP1)
    }
    #[doc = "External (GPIO Pins)"]
    #[inline]
    pub fn external(self) -> &'a mut W {
        self.variant(EM0W::EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline]
    pub fn timer(self) -> &'a mut W {
        self.variant(EM0W::TIMER)
    }
    #[doc = "PWM generator 0"]
    #[inline]
    pub fn pwm0(self) -> &'a mut W {
        self.variant(EM0W::PWM0)
    }
    #[doc = "PWM generator 1"]
    #[inline]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(EM0W::PWM1)
    }
    #[doc = "PWM generator 2"]
    #[inline]
    pub fn pwm2(self) -> &'a mut W {
        self.variant(EM0W::PWM2)
    }
    #[doc = "PWM generator 3"]
    #[inline]
    pub fn pwm3(self) -> &'a mut W {
        self.variant(EM0W::PWM3)
    }
    #[doc = "Always (continuously sample)"]
    #[inline]
    pub fn always(self) -> &'a mut W {
        self.variant(EM0W::ALWAYS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EM1`"]
pub enum EM1W {
    #[doc = "Processor (default)"]
    PROCESSOR,
    #[doc = "Analog Comparator 0"]
    COMP0,
    #[doc = "Analog Comparator 1"]
    COMP1,
    #[doc = "External (GPIO Pins)"]
    EXTERNAL,
    #[doc = "Timer"]
    TIMER,
    #[doc = "PWM generator 0"]
    PWM0,
    #[doc = "PWM generator 1"]
    PWM1,
    #[doc = "PWM generator 2"]
    PWM2,
    #[doc = "PWM generator 3"]
    PWM3,
    #[doc = "Always (continuously sample)"]
    ALWAYS,
}
impl EM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EM1W::PROCESSOR => 0,
            EM1W::COMP0 => 1,
            EM1W::COMP1 => 2,
            EM1W::EXTERNAL => 4,
            EM1W::TIMER => 5,
            EM1W::PWM0 => 6,
            EM1W::PWM1 => 7,
            EM1W::PWM2 => 8,
            EM1W::PWM3 => 9,
            EM1W::ALWAYS => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EM1W<'a> {
    w: &'a mut W,
}
impl<'a> _EM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EM1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Processor (default)"]
    #[inline]
    pub fn processor(self) -> &'a mut W {
        self.variant(EM1W::PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline]
    pub fn comp0(self) -> &'a mut W {
        self.variant(EM1W::COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline]
    pub fn comp1(self) -> &'a mut W {
        self.variant(EM1W::COMP1)
    }
    #[doc = "External (GPIO Pins)"]
    #[inline]
    pub fn external(self) -> &'a mut W {
        self.variant(EM1W::EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline]
    pub fn timer(self) -> &'a mut W {
        self.variant(EM1W::TIMER)
    }
    #[doc = "PWM generator 0"]
    #[inline]
    pub fn pwm0(self) -> &'a mut W {
        self.variant(EM1W::PWM0)
    }
    #[doc = "PWM generator 1"]
    #[inline]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(EM1W::PWM1)
    }
    #[doc = "PWM generator 2"]
    #[inline]
    pub fn pwm2(self) -> &'a mut W {
        self.variant(EM1W::PWM2)
    }
    #[doc = "PWM generator 3"]
    #[inline]
    pub fn pwm3(self) -> &'a mut W {
        self.variant(EM1W::PWM3)
    }
    #[doc = "Always (continuously sample)"]
    #[inline]
    pub fn always(self) -> &'a mut W {
        self.variant(EM1W::ALWAYS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EM2`"]
pub enum EM2W {
    #[doc = "Processor (default)"]
    PROCESSOR,
    #[doc = "Analog Comparator 0"]
    COMP0,
    #[doc = "Analog Comparator 1"]
    COMP1,
    #[doc = "External (GPIO Pins)"]
    EXTERNAL,
    #[doc = "Timer"]
    TIMER,
    #[doc = "PWM generator 0"]
    PWM0,
    #[doc = "PWM generator 1"]
    PWM1,
    #[doc = "PWM generator 2"]
    PWM2,
    #[doc = "PWM generator 3"]
    PWM3,
    #[doc = "Always (continuously sample)"]
    ALWAYS,
}
impl EM2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EM2W::PROCESSOR => 0,
            EM2W::COMP0 => 1,
            EM2W::COMP1 => 2,
            EM2W::EXTERNAL => 4,
            EM2W::TIMER => 5,
            EM2W::PWM0 => 6,
            EM2W::PWM1 => 7,
            EM2W::PWM2 => 8,
            EM2W::PWM3 => 9,
            EM2W::ALWAYS => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EM2W<'a> {
    w: &'a mut W,
}
impl<'a> _EM2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EM2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Processor (default)"]
    #[inline]
    pub fn processor(self) -> &'a mut W {
        self.variant(EM2W::PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline]
    pub fn comp0(self) -> &'a mut W {
        self.variant(EM2W::COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline]
    pub fn comp1(self) -> &'a mut W {
        self.variant(EM2W::COMP1)
    }
    #[doc = "External (GPIO Pins)"]
    #[inline]
    pub fn external(self) -> &'a mut W {
        self.variant(EM2W::EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline]
    pub fn timer(self) -> &'a mut W {
        self.variant(EM2W::TIMER)
    }
    #[doc = "PWM generator 0"]
    #[inline]
    pub fn pwm0(self) -> &'a mut W {
        self.variant(EM2W::PWM0)
    }
    #[doc = "PWM generator 1"]
    #[inline]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(EM2W::PWM1)
    }
    #[doc = "PWM generator 2"]
    #[inline]
    pub fn pwm2(self) -> &'a mut W {
        self.variant(EM2W::PWM2)
    }
    #[doc = "PWM generator 3"]
    #[inline]
    pub fn pwm3(self) -> &'a mut W {
        self.variant(EM2W::PWM3)
    }
    #[doc = "Always (continuously sample)"]
    #[inline]
    pub fn always(self) -> &'a mut W {
        self.variant(EM2W::ALWAYS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EM3`"]
pub enum EM3W {
    #[doc = "Processor (default)"]
    PROCESSOR,
    #[doc = "Analog Comparator 0"]
    COMP0,
    #[doc = "Analog Comparator 1"]
    COMP1,
    #[doc = "External (GPIO Pins)"]
    EXTERNAL,
    #[doc = "Timer"]
    TIMER,
    #[doc = "PWM generator 0"]
    PWM0,
    #[doc = "PWM generator 1"]
    PWM1,
    #[doc = "PWM generator 2"]
    PWM2,
    #[doc = "PWM generator 3"]
    PWM3,
    #[doc = "Always (continuously sample)"]
    ALWAYS,
}
impl EM3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EM3W::PROCESSOR => 0,
            EM3W::COMP0 => 1,
            EM3W::COMP1 => 2,
            EM3W::EXTERNAL => 4,
            EM3W::TIMER => 5,
            EM3W::PWM0 => 6,
            EM3W::PWM1 => 7,
            EM3W::PWM2 => 8,
            EM3W::PWM3 => 9,
            EM3W::ALWAYS => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EM3W<'a> {
    w: &'a mut W,
}
impl<'a> _EM3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EM3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Processor (default)"]
    #[inline]
    pub fn processor(self) -> &'a mut W {
        self.variant(EM3W::PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline]
    pub fn comp0(self) -> &'a mut W {
        self.variant(EM3W::COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline]
    pub fn comp1(self) -> &'a mut W {
        self.variant(EM3W::COMP1)
    }
    #[doc = "External (GPIO Pins)"]
    #[inline]
    pub fn external(self) -> &'a mut W {
        self.variant(EM3W::EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline]
    pub fn timer(self) -> &'a mut W {
        self.variant(EM3W::TIMER)
    }
    #[doc = "PWM generator 0"]
    #[inline]
    pub fn pwm0(self) -> &'a mut W {
        self.variant(EM3W::PWM0)
    }
    #[doc = "PWM generator 1"]
    #[inline]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(EM3W::PWM1)
    }
    #[doc = "PWM generator 2"]
    #[inline]
    pub fn pwm2(self) -> &'a mut W {
        self.variant(EM3W::PWM2)
    }
    #[doc = "PWM generator 3"]
    #[inline]
    pub fn pwm3(self) -> &'a mut W {
        self.variant(EM3W::PWM3)
    }
    #[doc = "Always (continuously sample)"]
    #[inline]
    pub fn always(self) -> &'a mut W {
        self.variant(EM3W::ALWAYS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:3 - SS0 Trigger Select"]
    #[inline]
    pub fn em0(&self) -> EM0R {
        EM0R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - SS1 Trigger Select"]
    #[inline]
    pub fn em1(&self) -> EM1R {
        EM1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - SS2 Trigger Select"]
    #[inline]
    pub fn em2(&self) -> EM2R {
        EM2R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - SS3 Trigger Select"]
    #[inline]
    pub fn em3(&self) -> EM3R {
        EM3R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:3 - SS0 Trigger Select"]
    #[inline]
    pub fn em0(&mut self) -> _EM0W {
        _EM0W { w: self }
    }
    #[doc = "Bits 4:7 - SS1 Trigger Select"]
    #[inline]
    pub fn em1(&mut self) -> _EM1W {
        _EM1W { w: self }
    }
    #[doc = "Bits 8:11 - SS2 Trigger Select"]
    #[inline]
    pub fn em2(&mut self) -> _EM2W {
        _EM2W { w: self }
    }
    #[doc = "Bits 12:15 - SS3 Trigger Select"]
    #[inline]
    pub fn em3(&mut self) -> _EM3W {
        _EM3W { w: self }
    }
}
