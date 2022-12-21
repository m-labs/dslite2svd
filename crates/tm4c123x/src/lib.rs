#![doc = "Peripheral access API for TM4C123X microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn GPIOA();
    fn GPIOB();
    fn GPIOC();
    fn GPIOD();
    fn GPIOE();
    fn UART0();
    fn UART1();
    fn SSI0();
    fn I2C0();
    fn PWM0_FAULT();
    fn PWM0_0();
    fn PWM0_1();
    fn PWM0_2();
    fn QEI0();
    fn ADC0SS0();
    fn ADC0SS1();
    fn ADC0SS2();
    fn ADC0SS3();
    fn WATCHDOG();
    fn TIMER0A();
    fn TIMER0B();
    fn TIMER1A();
    fn TIMER1B();
    fn TIMER2A();
    fn TIMER2B();
    fn COMP0();
    fn COMP1();
    fn SYSCTL();
    fn FLASH();
    fn GPIOF();
    fn UART2();
    fn SSI1();
    fn TIMER3A();
    fn TIMER3B();
    fn I2C1();
    fn QEI1();
    fn CAN0();
    fn CAN1();
    fn HIBERNATE();
    fn USB0();
    fn PWM0_3();
    fn UDMA();
    fn UDMAERR();
    fn ADC1SS0();
    fn ADC1SS1();
    fn ADC1SS2();
    fn ADC1SS3();
    fn SSI2();
    fn SSI3();
    fn UART3();
    fn UART4();
    fn UART5();
    fn UART6();
    fn UART7();
    fn I2C2();
    fn I2C3();
    fn TIMER4A();
    fn TIMER4B();
    fn TIMER5A();
    fn TIMER5B();
    fn WTIMER0A();
    fn WTIMER0B();
    fn WTIMER1A();
    fn WTIMER1B();
    fn WTIMER2A();
    fn WTIMER2B();
    fn WTIMER3A();
    fn WTIMER3B();
    fn WTIMER4A();
    fn WTIMER4B();
    fn WTIMER5A();
    fn WTIMER5B();
    fn SYSEXC();
    fn PWM1_0();
    fn PWM1_1();
    fn PWM1_2();
    fn PWM1_3();
    fn PWM1_FAULT();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 139] = [
    Vector { _handler: GPIOA },
    Vector { _handler: GPIOB },
    Vector { _handler: GPIOC },
    Vector { _handler: GPIOD },
    Vector { _handler: GPIOE },
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _handler: SSI0 },
    Vector { _handler: I2C0 },
    Vector { _handler: PWM0_FAULT },
    Vector { _handler: PWM0_0 },
    Vector { _handler: PWM0_1 },
    Vector { _handler: PWM0_2 },
    Vector { _handler: QEI0 },
    Vector { _handler: ADC0SS0 },
    Vector { _handler: ADC0SS1 },
    Vector { _handler: ADC0SS2 },
    Vector { _handler: ADC0SS3 },
    Vector { _handler: WATCHDOG },
    Vector { _handler: TIMER0A },
    Vector { _handler: TIMER0B },
    Vector { _handler: TIMER1A },
    Vector { _handler: TIMER1B },
    Vector { _handler: TIMER2A },
    Vector { _handler: TIMER2B },
    Vector { _handler: COMP0 },
    Vector { _handler: COMP1 },
    Vector { _reserved: 0 },
    Vector { _handler: SYSCTL },
    Vector { _handler: FLASH },
    Vector { _handler: GPIOF },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: UART2 },
    Vector { _handler: SSI1 },
    Vector { _handler: TIMER3A },
    Vector { _handler: TIMER3B },
    Vector { _handler: I2C1 },
    Vector { _handler: QEI1 },
    Vector { _handler: CAN0 },
    Vector { _handler: CAN1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: HIBERNATE },
    Vector { _handler: USB0 },
    Vector { _handler: PWM0_3 },
    Vector { _handler: UDMA },
    Vector { _handler: UDMAERR },
    Vector { _handler: ADC1SS0 },
    Vector { _handler: ADC1SS1 },
    Vector { _handler: ADC1SS2 },
    Vector { _handler: ADC1SS3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SSI2 },
    Vector { _handler: SSI3 },
    Vector { _handler: UART3 },
    Vector { _handler: UART4 },
    Vector { _handler: UART5 },
    Vector { _handler: UART6 },
    Vector { _handler: UART7 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C2 },
    Vector { _handler: I2C3 },
    Vector { _handler: TIMER4A },
    Vector { _handler: TIMER4B },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TIMER5A },
    Vector { _handler: TIMER5B },
    Vector { _handler: WTIMER0A },
    Vector { _handler: WTIMER0B },
    Vector { _handler: WTIMER1A },
    Vector { _handler: WTIMER1B },
    Vector { _handler: WTIMER2A },
    Vector { _handler: WTIMER2B },
    Vector { _handler: WTIMER3A },
    Vector { _handler: WTIMER3B },
    Vector { _handler: WTIMER4A },
    Vector { _handler: WTIMER4B },
    Vector { _handler: WTIMER5A },
    Vector { _handler: WTIMER5B },
    Vector { _handler: SYSEXC },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PWM1_0 },
    Vector { _handler: PWM1_1 },
    Vector { _handler: PWM1_2 },
    Vector { _handler: PWM1_3 },
    Vector { _handler: PWM1_FAULT },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - GPIO Port A"]
    GPIOA = 0,
    #[doc = "1 - GPIO Port B"]
    GPIOB = 1,
    #[doc = "2 - GPIO Port C"]
    GPIOC = 2,
    #[doc = "3 - GPIO Port D"]
    GPIOD = 3,
    #[doc = "4 - GPIO Port E"]
    GPIOE = 4,
    #[doc = "5 - UART0"]
    UART0 = 5,
    #[doc = "6 - UART1"]
    UART1 = 6,
    #[doc = "7 - SSI0"]
    SSI0 = 7,
    #[doc = "8 - I2C0"]
    I2C0 = 8,
    #[doc = "9 - PWM0 Fault"]
    PWM0_FAULT = 9,
    #[doc = "10 - PWM0 Generator 0"]
    PWM0_0 = 10,
    #[doc = "11 - PWM0 Generator 1"]
    PWM0_1 = 11,
    #[doc = "12 - PWM0 Generator 2"]
    PWM0_2 = 12,
    #[doc = "13 - QEI0"]
    QEI0 = 13,
    #[doc = "14 - ADC0 Sequence 0"]
    ADC0SS0 = 14,
    #[doc = "15 - ADC0 Sequence 1"]
    ADC0SS1 = 15,
    #[doc = "16 - ADC0 Sequence 2"]
    ADC0SS2 = 16,
    #[doc = "17 - ADC0 Sequence 3"]
    ADC0SS3 = 17,
    #[doc = "18 - Watchdog Timers 0 and 1"]
    WATCHDOG = 18,
    #[doc = "19 - 16/32-Bit Timer 0A"]
    TIMER0A = 19,
    #[doc = "20 - 16/32-Bit Timer 0B"]
    TIMER0B = 20,
    #[doc = "21 - 16/32-Bit Timer 1A"]
    TIMER1A = 21,
    #[doc = "22 - 16/32-Bit Timer 1B"]
    TIMER1B = 22,
    #[doc = "23 - 16/32-Bit Timer 2A"]
    TIMER2A = 23,
    #[doc = "24 - 16/32-Bit Timer 2B"]
    TIMER2B = 24,
    #[doc = "25 - Analog Comparator 0"]
    COMP0 = 25,
    #[doc = "26 - Analog Comparator 1"]
    COMP1 = 26,
    #[doc = "28 - System Control"]
    SYSCTL = 28,
    #[doc = "29 - Flash Memory Control and EEPROM Control"]
    FLASH = 29,
    #[doc = "30 - GPIO Port F"]
    GPIOF = 30,
    #[doc = "33 - UART2"]
    UART2 = 33,
    #[doc = "34 - SSI1"]
    SSI1 = 34,
    #[doc = "35 - Timer 3A"]
    TIMER3A = 35,
    #[doc = "36 - Timer 3B"]
    TIMER3B = 36,
    #[doc = "37 - I2C1"]
    I2C1 = 37,
    #[doc = "38 - QEI1"]
    QEI1 = 38,
    #[doc = "39 - CAN0"]
    CAN0 = 39,
    #[doc = "40 - CAN1"]
    CAN1 = 40,
    #[doc = "43 - Hibernation Module"]
    HIBERNATE = 43,
    #[doc = "44 - USB"]
    USB0 = 44,
    #[doc = "45 - PWM Generator 3"]
    PWM0_3 = 45,
    #[doc = "46 - uDMA Software"]
    UDMA = 46,
    #[doc = "47 - uDMA Error"]
    UDMAERR = 47,
    #[doc = "48 - ADC1 Sequence 0"]
    ADC1SS0 = 48,
    #[doc = "49 - ADC1 Sequence 1"]
    ADC1SS1 = 49,
    #[doc = "50 - ADC1 Sequence 2"]
    ADC1SS2 = 50,
    #[doc = "51 - ADC1 Sequence 3"]
    ADC1SS3 = 51,
    #[doc = "57 - SSI2"]
    SSI2 = 57,
    #[doc = "58 - SSI3"]
    SSI3 = 58,
    #[doc = "59 - UART3"]
    UART3 = 59,
    #[doc = "60 - UART4"]
    UART4 = 60,
    #[doc = "61 - UART5"]
    UART5 = 61,
    #[doc = "62 - UART6"]
    UART6 = 62,
    #[doc = "63 - UART7"]
    UART7 = 63,
    #[doc = "68 - I2C2"]
    I2C2 = 68,
    #[doc = "69 - I2C3"]
    I2C3 = 69,
    #[doc = "70 - 16/32-Bit Timer 4A"]
    TIMER4A = 70,
    #[doc = "71 - 16/32-Bit Timer 4B"]
    TIMER4B = 71,
    #[doc = "92 - 16/32-Bit Timer 5A"]
    TIMER5A = 92,
    #[doc = "93 - 16/32-Bit Timer 5B"]
    TIMER5B = 93,
    #[doc = "94 - 32/64-Bit Timer 0A"]
    WTIMER0A = 94,
    #[doc = "95 - 32/64-Bit Timer 0B"]
    WTIMER0B = 95,
    #[doc = "96 - 32/64-Bit Timer 1A"]
    WTIMER1A = 96,
    #[doc = "97 - 32/64-Bit Timer 1B"]
    WTIMER1B = 97,
    #[doc = "98 - 32/64-Bit Timer 2A"]
    WTIMER2A = 98,
    #[doc = "99 - 32/64-Bit Timer 2B"]
    WTIMER2B = 99,
    #[doc = "100 - 32/64-Bit Timer 3A"]
    WTIMER3A = 100,
    #[doc = "101 - 32/64-Bit Timer 3B"]
    WTIMER3B = 101,
    #[doc = "102 - 32/64-Bit Timer 4A"]
    WTIMER4A = 102,
    #[doc = "103 - 32/64-Bit Timer 4B"]
    WTIMER4B = 103,
    #[doc = "104 - 32/64-Bit Timer 5A"]
    WTIMER5A = 104,
    #[doc = "105 - 32/64-Bit Timer 5B"]
    WTIMER5B = 105,
    #[doc = "106 - System Exception (imprecise)"]
    SYSEXC = 106,
    #[doc = "134 - PWM1 Generator 0"]
    PWM1_0 = 134,
    #[doc = "135 - PWM1 Generator 1"]
    PWM1_1 = 135,
    #[doc = "136 - PWM1 Generator 2"]
    PWM1_2 = 136,
    #[doc = "137 - PWM1 Generator 3"]
    PWM1_3 = 137,
    #[doc = "138 - PWM1 Fault"]
    PWM1_FAULT = 138,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Watchdog Timer register offsets"]
pub struct WATCHDOG0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WATCHDOG0 {}
impl WATCHDOG0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const watchdog0::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for WATCHDOG0 {
    type Target = watchdog0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WATCHDOG0::ptr() }
    }
}
#[doc = "Watchdog Timer register offsets"]
pub mod watchdog0;
#[doc = "Watchdog Timer register offsets"]
pub struct WATCHDOG1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WATCHDOG1 {}
impl WATCHDOG1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const watchdog0::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for WATCHDOG1 {
    type Target = watchdog0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WATCHDOG1::ptr() }
    }
}
#[doc = "GPIO register offsets"]
pub struct GPIO_PORTA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTA {}
impl GPIO_PORTA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for GPIO_PORTA {
    type Target = gpio_porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTA::ptr() }
    }
}
#[doc = "GPIO register offsets"]
pub mod gpio_porta;
#[doc = "GPIO register offsets"]
pub struct GPIO_PORTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTB {}
impl GPIO_PORTB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta::RegisterBlock {
        0x4000_5000 as *const _
    }
}
impl Deref for GPIO_PORTB {
    type Target = gpio_porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTB::ptr() }
    }
}
#[doc = "GPIO register offsets"]
pub struct GPIO_PORTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTC {}
impl GPIO_PORTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta::RegisterBlock {
        0x4000_6000 as *const _
    }
}
impl Deref for GPIO_PORTC {
    type Target = gpio_porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTC::ptr() }
    }
}
#[doc = "GPIO register offsets"]
pub struct GPIO_PORTD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTD {}
impl GPIO_PORTD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta::RegisterBlock {
        0x4000_7000 as *const _
    }
}
impl Deref for GPIO_PORTD {
    type Target = gpio_porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTD::ptr() }
    }
}
#[doc = "SSI register offsets"]
pub struct SSI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSI0 {}
impl SSI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssi0::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for SSI0 {
    type Target = ssi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSI0::ptr() }
    }
}
#[doc = "SSI register offsets"]
pub mod ssi0;
#[doc = "SSI register offsets"]
pub struct SSI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSI1 {}
impl SSI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssi0::RegisterBlock {
        0x4000_9000 as *const _
    }
}
impl Deref for SSI1 {
    type Target = ssi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSI1::ptr() }
    }
}
#[doc = "SSI register offsets"]
pub struct SSI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSI2 {}
impl SSI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssi0::RegisterBlock {
        0x4000_a000 as *const _
    }
}
impl Deref for SSI2 {
    type Target = ssi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSI2::ptr() }
    }
}
#[doc = "SSI register offsets"]
pub struct SSI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSI3 {}
impl SSI3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssi0::RegisterBlock {
        0x4000_b000 as *const _
    }
}
impl Deref for SSI3 {
    type Target = ssi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSI3::ptr() }
    }
}
#[doc = "UART register offsets"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4000_c000 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "UART register offsets"]
pub mod uart0;
#[doc = "UART register offsets"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4000_d000 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "UART register offsets"]
pub struct UART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART2 {}
impl UART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4000_e000 as *const _
    }
}
impl Deref for UART2 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART2::ptr() }
    }
}
#[doc = "UART register offsets"]
pub struct UART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART3 {}
impl UART3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4000_f000 as *const _
    }
}
impl Deref for UART3 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART3::ptr() }
    }
}
#[doc = "UART register offsets"]
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART4 {}
impl UART4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for UART4 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART4::ptr() }
    }
}
#[doc = "UART register offsets"]
pub struct UART5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART5 {}
impl UART5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4001_1000 as *const _
    }
}
impl Deref for UART5 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART5::ptr() }
    }
}
#[doc = "UART register offsets"]
pub struct UART6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART6 {}
impl UART6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4001_2000 as *const _
    }
}
impl Deref for UART6 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART6::ptr() }
    }
}
#[doc = "UART register offsets"]
pub struct UART7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART7 {}
impl UART7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4001_3000 as *const _
    }
}
impl Deref for UART7 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART7::ptr() }
    }
}
#[doc = "I2C register offsets"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "I2C register offsets"]
pub mod i2c0;
#[doc = "I2C register offsets"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4002_1000 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "I2C register offsets"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4002_2000 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "I2C register offsets"]
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4002_3000 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C3::ptr() }
    }
}
#[doc = "GPIO register offsets"]
pub struct GPIO_PORTE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTE {}
impl GPIO_PORTE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta::RegisterBlock {
        0x4002_4000 as *const _
    }
}
impl Deref for GPIO_PORTE {
    type Target = gpio_porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTE::ptr() }
    }
}
#[doc = "GPIO register offsets"]
pub struct GPIO_PORTF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTF {}
impl GPIO_PORTF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta::RegisterBlock {
        0x4002_5000 as *const _
    }
}
impl Deref for GPIO_PORTF {
    type Target = gpio_porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTF::ptr() }
    }
}
#[doc = "PWM register offsets"]
pub struct PWM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM0 {}
impl PWM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm0::RegisterBlock {
        0x4002_8000 as *const _
    }
}
impl Deref for PWM0 {
    type Target = pwm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM0::ptr() }
    }
}
#[doc = "PWM register offsets"]
pub mod pwm0;
#[doc = "PWM register offsets"]
pub struct PWM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM1 {}
impl PWM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm0::RegisterBlock {
        0x4002_9000 as *const _
    }
}
impl Deref for PWM1 {
    type Target = pwm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM1::ptr() }
    }
}
#[doc = "QEI register offsets"]
pub struct QEI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QEI0 {}
impl QEI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qei0::RegisterBlock {
        0x4002_c000 as *const _
    }
}
impl Deref for QEI0 {
    type Target = qei0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*QEI0::ptr() }
    }
}
#[doc = "QEI register offsets"]
pub mod qei0;
#[doc = "QEI register offsets"]
pub struct QEI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QEI1 {}
impl QEI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qei0::RegisterBlock {
        0x4002_d000 as *const _
    }
}
impl Deref for QEI1 {
    type Target = qei0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*QEI1::ptr() }
    }
}
#[doc = "Timer register offsets"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4003_0000 as *const _
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER0::ptr() }
    }
}
#[doc = "Timer register offsets"]
pub mod timer0;
#[doc = "Timer register offsets"]
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1 {}
impl TIMER1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4003_1000 as *const _
    }
}
impl Deref for TIMER1 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER1::ptr() }
    }
}
#[doc = "Timer register offsets"]
pub struct TIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2 {}
impl TIMER2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4003_2000 as *const _
    }
}
impl Deref for TIMER2 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER2::ptr() }
    }
}
#[doc = "Timer register offsets"]
pub struct TIMER3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER3 {}
impl TIMER3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4003_3000 as *const _
    }
}
impl Deref for TIMER3 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER3::ptr() }
    }
}
#[doc = "Timer register offsets"]
pub struct TIMER4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER4 {}
impl TIMER4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4003_4000 as *const _
    }
}
impl Deref for TIMER4 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER4::ptr() }
    }
}
#[doc = "Timer register offsets"]
pub struct TIMER5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER5 {}
impl TIMER5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4003_5000 as *const _
    }
}
impl Deref for TIMER5 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER5::ptr() }
    }
}
#[doc = "Timer register offsets"]
pub struct WTIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WTIMER0 {}
impl WTIMER0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wtimer0::RegisterBlock {
        0x4003_6000 as *const _
    }
}
impl Deref for WTIMER0 {
    type Target = wtimer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WTIMER0::ptr() }
    }
}
#[doc = "Timer register offsets"]
pub mod wtimer0;
#[doc = "Timer register offsets"]
pub struct WTIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WTIMER1 {}
impl WTIMER1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wtimer0::RegisterBlock {
        0x4003_7000 as *const _
    }
}
impl Deref for WTIMER1 {
    type Target = wtimer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WTIMER1::ptr() }
    }
}
#[doc = "ADC register offsets"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc0::RegisterBlock {
        0x4003_8000 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "ADC register offsets"]
pub mod adc0;
#[doc = "ADC register offsets"]
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc0::RegisterBlock {
        0x4003_9000 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC1::ptr() }
    }
}
#[doc = "Comparator register offsets"]
pub struct COMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMP {}
impl COMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const comp::RegisterBlock {
        0x4003_c000 as *const _
    }
}
impl Deref for COMP {
    type Target = comp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*COMP::ptr() }
    }
}
#[doc = "Comparator register offsets"]
pub mod comp;
#[doc = "CAN register offsets"]
pub struct CAN0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN0 {}
impl CAN0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can0::RegisterBlock {
        0x4004_0000 as *const _
    }
}
impl Deref for CAN0 {
    type Target = can0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN0::ptr() }
    }
}
#[doc = "CAN register offsets"]
pub mod can0;
#[doc = "CAN register offsets"]
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN1 {}
impl CAN1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can0::RegisterBlock {
        0x4004_1000 as *const _
    }
}
impl Deref for CAN1 {
    type Target = can0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN1::ptr() }
    }
}
#[doc = "Timer register offsets"]
pub struct WTIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WTIMER2 {}
impl WTIMER2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wtimer0::RegisterBlock {
        0x4004_c000 as *const _
    }
}
impl Deref for WTIMER2 {
    type Target = wtimer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WTIMER2::ptr() }
    }
}
#[doc = "Timer register offsets"]
pub struct WTIMER3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WTIMER3 {}
impl WTIMER3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wtimer0::RegisterBlock {
        0x4004_d000 as *const _
    }
}
impl Deref for WTIMER3 {
    type Target = wtimer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WTIMER3::ptr() }
    }
}
#[doc = "Timer register offsets"]
pub struct WTIMER4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WTIMER4 {}
impl WTIMER4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wtimer0::RegisterBlock {
        0x4004_e000 as *const _
    }
}
impl Deref for WTIMER4 {
    type Target = wtimer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WTIMER4::ptr() }
    }
}
#[doc = "Timer register offsets"]
pub struct WTIMER5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WTIMER5 {}
impl WTIMER5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wtimer0::RegisterBlock {
        0x4004_f000 as *const _
    }
}
impl Deref for WTIMER5 {
    type Target = wtimer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WTIMER5::ptr() }
    }
}
#[doc = "Univeral Serial Bus register offsets"]
pub struct USB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0 {}
impl USB0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0::RegisterBlock {
        0x4005_0000 as *const _
    }
}
impl Deref for USB0 {
    type Target = usb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0::ptr() }
    }
}
#[doc = "Univeral Serial Bus register offsets"]
pub mod usb0;
#[doc = "GPIO register offsets"]
pub struct GPIO_PORTA_AHB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTA_AHB {}
impl GPIO_PORTA_AHB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta::RegisterBlock {
        0x4005_8000 as *const _
    }
}
impl Deref for GPIO_PORTA_AHB {
    type Target = gpio_porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTA_AHB::ptr() }
    }
}
#[doc = "GPIO register offsets"]
pub struct GPIO_PORTB_AHB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTB_AHB {}
impl GPIO_PORTB_AHB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta::RegisterBlock {
        0x4005_9000 as *const _
    }
}
impl Deref for GPIO_PORTB_AHB {
    type Target = gpio_porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTB_AHB::ptr() }
    }
}
#[doc = "GPIO register offsets"]
pub struct GPIO_PORTC_AHB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTC_AHB {}
impl GPIO_PORTC_AHB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta::RegisterBlock {
        0x4005_a000 as *const _
    }
}
impl Deref for GPIO_PORTC_AHB {
    type Target = gpio_porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTC_AHB::ptr() }
    }
}
#[doc = "GPIO register offsets"]
pub struct GPIO_PORTD_AHB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTD_AHB {}
impl GPIO_PORTD_AHB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta::RegisterBlock {
        0x4005_b000 as *const _
    }
}
impl Deref for GPIO_PORTD_AHB {
    type Target = gpio_porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTD_AHB::ptr() }
    }
}
#[doc = "GPIO register offsets"]
pub struct GPIO_PORTE_AHB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTE_AHB {}
impl GPIO_PORTE_AHB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta::RegisterBlock {
        0x4005_c000 as *const _
    }
}
impl Deref for GPIO_PORTE_AHB {
    type Target = gpio_porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTE_AHB::ptr() }
    }
}
#[doc = "GPIO register offsets"]
pub struct GPIO_PORTF_AHB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTF_AHB {}
impl GPIO_PORTF_AHB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta::RegisterBlock {
        0x4005_d000 as *const _
    }
}
impl Deref for GPIO_PORTF_AHB {
    type Target = gpio_porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTF_AHB::ptr() }
    }
}
#[doc = "EEPROM register offsets"]
pub struct EEPROM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EEPROM {}
impl EEPROM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eeprom::RegisterBlock {
        0x400a_f000 as *const _
    }
}
impl Deref for EEPROM {
    type Target = eeprom::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EEPROM::ptr() }
    }
}
#[doc = "EEPROM register offsets"]
pub mod eeprom;
#[doc = "System Exception Module register addresses"]
pub struct SYSEXC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSEXC {}
impl SYSEXC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sysexc::RegisterBlock {
        0x400f_9000 as *const _
    }
}
impl Deref for SYSEXC {
    type Target = sysexc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSEXC::ptr() }
    }
}
#[doc = "System Exception Module register addresses"]
pub mod sysexc;
#[doc = "Hibernation module register addresses"]
pub struct HIB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HIB {}
impl HIB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hib::RegisterBlock {
        0x400f_c000 as *const _
    }
}
impl Deref for HIB {
    type Target = hib::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HIB::ptr() }
    }
}
#[doc = "Hibernation module register addresses"]
pub mod hib;
#[doc = "FLASH register offsets"]
pub struct FLASH_CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH_CTRL {}
impl FLASH_CTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash_ctrl::RegisterBlock {
        0x400f_d000 as *const _
    }
}
impl Deref for FLASH_CTRL {
    type Target = flash_ctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH_CTRL::ptr() }
    }
}
#[doc = "FLASH register offsets"]
pub mod flash_ctrl;
#[doc = "System Control register addresses"]
pub struct SYSCTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCTL {}
impl SYSCTL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sysctl::RegisterBlock {
        0x400f_e000 as *const _
    }
}
impl Deref for SYSCTL {
    type Target = sysctl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCTL::ptr() }
    }
}
#[doc = "System Control register addresses"]
pub mod sysctl;
#[doc = "Micro Direct Memory Access register addresses"]
pub struct UDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UDMA {}
impl UDMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const udma::RegisterBlock {
        0x400f_f000 as *const _
    }
}
impl Deref for UDMA {
    type Target = udma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UDMA::ptr() }
    }
}
#[doc = "Micro Direct Memory Access register addresses"]
pub mod udma;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "WATCHDOG0"]
    pub WATCHDOG0: WATCHDOG0,
    #[doc = "WATCHDOG1"]
    pub WATCHDOG1: WATCHDOG1,
    #[doc = "GPIO_PORTA"]
    pub GPIO_PORTA: GPIO_PORTA,
    #[doc = "GPIO_PORTB"]
    pub GPIO_PORTB: GPIO_PORTB,
    #[doc = "GPIO_PORTC"]
    pub GPIO_PORTC: GPIO_PORTC,
    #[doc = "GPIO_PORTD"]
    pub GPIO_PORTD: GPIO_PORTD,
    #[doc = "SSI0"]
    pub SSI0: SSI0,
    #[doc = "SSI1"]
    pub SSI1: SSI1,
    #[doc = "SSI2"]
    pub SSI2: SSI2,
    #[doc = "SSI3"]
    pub SSI3: SSI3,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "UART2"]
    pub UART2: UART2,
    #[doc = "UART3"]
    pub UART3: UART3,
    #[doc = "UART4"]
    pub UART4: UART4,
    #[doc = "UART5"]
    pub UART5: UART5,
    #[doc = "UART6"]
    pub UART6: UART6,
    #[doc = "UART7"]
    pub UART7: UART7,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2C3"]
    pub I2C3: I2C3,
    #[doc = "GPIO_PORTE"]
    pub GPIO_PORTE: GPIO_PORTE,
    #[doc = "GPIO_PORTF"]
    pub GPIO_PORTF: GPIO_PORTF,
    #[doc = "PWM0"]
    pub PWM0: PWM0,
    #[doc = "PWM1"]
    pub PWM1: PWM1,
    #[doc = "QEI0"]
    pub QEI0: QEI0,
    #[doc = "QEI1"]
    pub QEI1: QEI1,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "TIMER2"]
    pub TIMER2: TIMER2,
    #[doc = "TIMER3"]
    pub TIMER3: TIMER3,
    #[doc = "TIMER4"]
    pub TIMER4: TIMER4,
    #[doc = "TIMER5"]
    pub TIMER5: TIMER5,
    #[doc = "WTIMER0"]
    pub WTIMER0: WTIMER0,
    #[doc = "WTIMER1"]
    pub WTIMER1: WTIMER1,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[doc = "COMP"]
    pub COMP: COMP,
    #[doc = "CAN0"]
    pub CAN0: CAN0,
    #[doc = "CAN1"]
    pub CAN1: CAN1,
    #[doc = "WTIMER2"]
    pub WTIMER2: WTIMER2,
    #[doc = "WTIMER3"]
    pub WTIMER3: WTIMER3,
    #[doc = "WTIMER4"]
    pub WTIMER4: WTIMER4,
    #[doc = "WTIMER5"]
    pub WTIMER5: WTIMER5,
    #[doc = "USB0"]
    pub USB0: USB0,
    #[doc = "GPIO_PORTA_AHB"]
    pub GPIO_PORTA_AHB: GPIO_PORTA_AHB,
    #[doc = "GPIO_PORTB_AHB"]
    pub GPIO_PORTB_AHB: GPIO_PORTB_AHB,
    #[doc = "GPIO_PORTC_AHB"]
    pub GPIO_PORTC_AHB: GPIO_PORTC_AHB,
    #[doc = "GPIO_PORTD_AHB"]
    pub GPIO_PORTD_AHB: GPIO_PORTD_AHB,
    #[doc = "GPIO_PORTE_AHB"]
    pub GPIO_PORTE_AHB: GPIO_PORTE_AHB,
    #[doc = "GPIO_PORTF_AHB"]
    pub GPIO_PORTF_AHB: GPIO_PORTF_AHB,
    #[doc = "EEPROM"]
    pub EEPROM: EEPROM,
    #[doc = "SYSEXC"]
    pub SYSEXC: SYSEXC,
    #[doc = "HIB"]
    pub HIB: HIB,
    #[doc = "FLASH_CTRL"]
    pub FLASH_CTRL: FLASH_CTRL,
    #[doc = "SYSCTL"]
    pub SYSCTL: SYSCTL,
    #[doc = "UDMA"]
    pub UDMA: UDMA,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| if unsafe { DEVICE_PERIPHERALS } { None } else { Some(unsafe { Peripherals::steal() }) })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            WATCHDOG0: WATCHDOG0 { _marker: PhantomData },
            WATCHDOG1: WATCHDOG1 { _marker: PhantomData },
            GPIO_PORTA: GPIO_PORTA { _marker: PhantomData },
            GPIO_PORTB: GPIO_PORTB { _marker: PhantomData },
            GPIO_PORTC: GPIO_PORTC { _marker: PhantomData },
            GPIO_PORTD: GPIO_PORTD { _marker: PhantomData },
            SSI0: SSI0 { _marker: PhantomData },
            SSI1: SSI1 { _marker: PhantomData },
            SSI2: SSI2 { _marker: PhantomData },
            SSI3: SSI3 { _marker: PhantomData },
            UART0: UART0 { _marker: PhantomData },
            UART1: UART1 { _marker: PhantomData },
            UART2: UART2 { _marker: PhantomData },
            UART3: UART3 { _marker: PhantomData },
            UART4: UART4 { _marker: PhantomData },
            UART5: UART5 { _marker: PhantomData },
            UART6: UART6 { _marker: PhantomData },
            UART7: UART7 { _marker: PhantomData },
            I2C0: I2C0 { _marker: PhantomData },
            I2C1: I2C1 { _marker: PhantomData },
            I2C2: I2C2 { _marker: PhantomData },
            I2C3: I2C3 { _marker: PhantomData },
            GPIO_PORTE: GPIO_PORTE { _marker: PhantomData },
            GPIO_PORTF: GPIO_PORTF { _marker: PhantomData },
            PWM0: PWM0 { _marker: PhantomData },
            PWM1: PWM1 { _marker: PhantomData },
            QEI0: QEI0 { _marker: PhantomData },
            QEI1: QEI1 { _marker: PhantomData },
            TIMER0: TIMER0 { _marker: PhantomData },
            TIMER1: TIMER1 { _marker: PhantomData },
            TIMER2: TIMER2 { _marker: PhantomData },
            TIMER3: TIMER3 { _marker: PhantomData },
            TIMER4: TIMER4 { _marker: PhantomData },
            TIMER5: TIMER5 { _marker: PhantomData },
            WTIMER0: WTIMER0 { _marker: PhantomData },
            WTIMER1: WTIMER1 { _marker: PhantomData },
            ADC0: ADC0 { _marker: PhantomData },
            ADC1: ADC1 { _marker: PhantomData },
            COMP: COMP { _marker: PhantomData },
            CAN0: CAN0 { _marker: PhantomData },
            CAN1: CAN1 { _marker: PhantomData },
            WTIMER2: WTIMER2 { _marker: PhantomData },
            WTIMER3: WTIMER3 { _marker: PhantomData },
            WTIMER4: WTIMER4 { _marker: PhantomData },
            WTIMER5: WTIMER5 { _marker: PhantomData },
            USB0: USB0 { _marker: PhantomData },
            GPIO_PORTA_AHB: GPIO_PORTA_AHB { _marker: PhantomData },
            GPIO_PORTB_AHB: GPIO_PORTB_AHB { _marker: PhantomData },
            GPIO_PORTC_AHB: GPIO_PORTC_AHB { _marker: PhantomData },
            GPIO_PORTD_AHB: GPIO_PORTD_AHB { _marker: PhantomData },
            GPIO_PORTE_AHB: GPIO_PORTE_AHB { _marker: PhantomData },
            GPIO_PORTF_AHB: GPIO_PORTF_AHB { _marker: PhantomData },
            EEPROM: EEPROM { _marker: PhantomData },
            SYSEXC: SYSEXC { _marker: PhantomData },
            HIB: HIB { _marker: PhantomData },
            FLASH_CTRL: FLASH_CTRL { _marker: PhantomData },
            SYSCTL: SYSCTL { _marker: PhantomData },
            UDMA: UDMA { _marker: PhantomData },
        }
    }
}
