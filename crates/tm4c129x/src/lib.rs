#![doc = "Peripheral access API for TM4C129X microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
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
    fn COMP2();
    fn SYSCTL();
    fn FLASH();
    fn GPIOF();
    fn GPIOG();
    fn GPIOH();
    fn UART2();
    fn SSI1();
    fn TIMER3A();
    fn TIMER3B();
    fn I2C1();
    fn CAN0();
    fn CAN1();
    fn EMAC0();
    fn HIBERNATE();
    fn USB0();
    fn PWM0_3();
    fn UDMA();
    fn UDMAERR();
    fn ADC1SS0();
    fn ADC1SS1();
    fn ADC1SS2();
    fn ADC1SS3();
    fn EPI0();
    fn GPIOJ();
    fn GPIOK();
    fn GPIOL();
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
    fn SYSEXC();
    fn I2C4();
    fn I2C5();
    fn GPIOM();
    fn GPION();
    fn TAMPER0();
    fn GPIOP0();
    fn GPIOP1();
    fn GPIOP2();
    fn GPIOP3();
    fn GPIOP4();
    fn GPIOP5();
    fn GPIOP6();
    fn GPIOP7();
    fn GPIOQ0();
    fn GPIOQ1();
    fn GPIOQ2();
    fn GPIOQ3();
    fn GPIOQ4();
    fn GPIOQ5();
    fn GPIOQ6();
    fn GPIOQ7();
    fn GPIOR();
    fn GPIOS();
    fn SHA0();
    fn AES0();
    fn DES0();
    fn LCD0();
    fn TIMER6A();
    fn TIMER6B();
    fn TIMER7A();
    fn TIMER7B();
    fn I2C6();
    fn I2C7();
    fn ONEWIRE0();
    fn I2C8();
    fn I2C9();
    fn GPIOT();
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
pub static __INTERRUPTS: [Vector; 112] = [
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
    Vector { _handler: COMP2 },
    Vector { _handler: SYSCTL },
    Vector { _handler: FLASH },
    Vector { _handler: GPIOF },
    Vector { _handler: GPIOG },
    Vector { _handler: GPIOH },
    Vector { _handler: UART2 },
    Vector { _handler: SSI1 },
    Vector { _handler: TIMER3A },
    Vector { _handler: TIMER3B },
    Vector { _handler: I2C1 },
    Vector { _handler: CAN0 },
    Vector { _handler: CAN1 },
    Vector { _handler: EMAC0 },
    Vector { _handler: HIBERNATE },
    Vector { _handler: USB0 },
    Vector { _handler: PWM0_3 },
    Vector { _handler: UDMA },
    Vector { _handler: UDMAERR },
    Vector { _handler: ADC1SS0 },
    Vector { _handler: ADC1SS1 },
    Vector { _handler: ADC1SS2 },
    Vector { _handler: ADC1SS3 },
    Vector { _handler: EPI0 },
    Vector { _handler: GPIOJ },
    Vector { _handler: GPIOK },
    Vector { _handler: GPIOL },
    Vector { _handler: SSI2 },
    Vector { _handler: SSI3 },
    Vector { _handler: UART3 },
    Vector { _handler: UART4 },
    Vector { _handler: UART5 },
    Vector { _handler: UART6 },
    Vector { _handler: UART7 },
    Vector { _handler: I2C2 },
    Vector { _handler: I2C3 },
    Vector { _handler: TIMER4A },
    Vector { _handler: TIMER4B },
    Vector { _handler: TIMER5A },
    Vector { _handler: TIMER5B },
    Vector { _handler: SYSEXC },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C4 },
    Vector { _handler: I2C5 },
    Vector { _handler: GPIOM },
    Vector { _handler: GPION },
    Vector { _reserved: 0 },
    Vector { _handler: TAMPER0 },
    Vector { _handler: GPIOP0 },
    Vector { _handler: GPIOP1 },
    Vector { _handler: GPIOP2 },
    Vector { _handler: GPIOP3 },
    Vector { _handler: GPIOP4 },
    Vector { _handler: GPIOP5 },
    Vector { _handler: GPIOP6 },
    Vector { _handler: GPIOP7 },
    Vector { _handler: GPIOQ0 },
    Vector { _handler: GPIOQ1 },
    Vector { _handler: GPIOQ2 },
    Vector { _handler: GPIOQ3 },
    Vector { _handler: GPIOQ4 },
    Vector { _handler: GPIOQ5 },
    Vector { _handler: GPIOQ6 },
    Vector { _handler: GPIOQ7 },
    Vector { _handler: GPIOR },
    Vector { _handler: GPIOS },
    Vector { _handler: SHA0 },
    Vector { _handler: AES0 },
    Vector { _handler: DES0 },
    Vector { _handler: LCD0 },
    Vector { _handler: TIMER6A },
    Vector { _handler: TIMER6B },
    Vector { _handler: TIMER7A },
    Vector { _handler: TIMER7B },
    Vector { _handler: I2C6 },
    Vector { _handler: I2C7 },
    Vector { _reserved: 0 },
    Vector { _handler: ONEWIRE0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C8 },
    Vector { _handler: I2C9 },
    Vector { _handler: GPIOT },
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
    #[doc = "9 - PWM Fault"]
    PWM0_FAULT = 9,
    #[doc = "10 - PWM Generator 0"]
    PWM0_0 = 10,
    #[doc = "11 - PWM Generator 1"]
    PWM0_1 = 11,
    #[doc = "12 - PWM Generator 2"]
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
    #[doc = "27 - Analog Comparator 2"]
    COMP2 = 27,
    #[doc = "28 - System Control"]
    SYSCTL = 28,
    #[doc = "29 - Flash Memory Control"]
    FLASH = 29,
    #[doc = "30 - GPIO Port F"]
    GPIOF = 30,
    #[doc = "31 - GPIO Port G"]
    GPIOG = 31,
    #[doc = "32 - GPIO Port H"]
    GPIOH = 32,
    #[doc = "33 - UART2"]
    UART2 = 33,
    #[doc = "34 - SSI1"]
    SSI1 = 34,
    #[doc = "35 - 16/32-Bit Timer 3A"]
    TIMER3A = 35,
    #[doc = "36 - 16/32-Bit Timer 3B"]
    TIMER3B = 36,
    #[doc = "37 - I2C1"]
    I2C1 = 37,
    #[doc = "38 - CAN 0"]
    CAN0 = 38,
    #[doc = "39 - CAN1"]
    CAN1 = 39,
    #[doc = "40 - Ethernet MAC"]
    EMAC0 = 40,
    #[doc = "41 - HIB (Power Island)"]
    HIBERNATE = 41,
    #[doc = "42 - USB MAC"]
    USB0 = 42,
    #[doc = "43 - PWM Generator 3"]
    PWM0_3 = 43,
    #[doc = "44 - uDMA 0 Software"]
    UDMA = 44,
    #[doc = "45 - uDMA 0 Error"]
    UDMAERR = 45,
    #[doc = "46 - ADC1 Sequence 0"]
    ADC1SS0 = 46,
    #[doc = "47 - ADC1 Sequence 1"]
    ADC1SS1 = 47,
    #[doc = "48 - ADC1 Sequence 2"]
    ADC1SS2 = 48,
    #[doc = "49 - ADC1 Sequence 3"]
    ADC1SS3 = 49,
    #[doc = "50 - EPI 0"]
    EPI0 = 50,
    #[doc = "51 - GPIO Port J"]
    GPIOJ = 51,
    #[doc = "52 - GPIO Port K"]
    GPIOK = 52,
    #[doc = "53 - GPIO Port L"]
    GPIOL = 53,
    #[doc = "54 - SSI 2"]
    SSI2 = 54,
    #[doc = "55 - SSI 3"]
    SSI3 = 55,
    #[doc = "56 - UART 3"]
    UART3 = 56,
    #[doc = "57 - UART 4"]
    UART4 = 57,
    #[doc = "58 - UART 5"]
    UART5 = 58,
    #[doc = "59 - UART 6"]
    UART6 = 59,
    #[doc = "60 - UART 7"]
    UART7 = 60,
    #[doc = "61 - I2C 2"]
    I2C2 = 61,
    #[doc = "62 - I2C 3"]
    I2C3 = 62,
    #[doc = "63 - Timer 4A"]
    TIMER4A = 63,
    #[doc = "64 - Timer 4B"]
    TIMER4B = 64,
    #[doc = "65 - Timer 5A"]
    TIMER5A = 65,
    #[doc = "66 - Timer 5B"]
    TIMER5B = 66,
    #[doc = "67 - Floating-Point Exception (imprecise)"]
    SYSEXC = 67,
    #[doc = "70 - I2C 4"]
    I2C4 = 70,
    #[doc = "71 - I2C 5"]
    I2C5 = 71,
    #[doc = "72 - GPIO Port M"]
    GPIOM = 72,
    #[doc = "73 - GPIO Port N"]
    GPION = 73,
    #[doc = "75 - Tamper"]
    TAMPER0 = 75,
    #[doc = "76 - GPIO Port P (Summary or P0)"]
    GPIOP0 = 76,
    #[doc = "77 - GPIO Port P1"]
    GPIOP1 = 77,
    #[doc = "78 - GPIO Port P2"]
    GPIOP2 = 78,
    #[doc = "79 - GPIO Port P3"]
    GPIOP3 = 79,
    #[doc = "80 - GPIO Port P4"]
    GPIOP4 = 80,
    #[doc = "81 - GPIO Port P5"]
    GPIOP5 = 81,
    #[doc = "82 - GPIO Port P6"]
    GPIOP6 = 82,
    #[doc = "83 - GPIO Port P7"]
    GPIOP7 = 83,
    #[doc = "84 - GPIO Port Q (Summary or Q0)"]
    GPIOQ0 = 84,
    #[doc = "85 - GPIO Port Q1"]
    GPIOQ1 = 85,
    #[doc = "86 - GPIO Port Q2"]
    GPIOQ2 = 86,
    #[doc = "87 - GPIO Port Q3"]
    GPIOQ3 = 87,
    #[doc = "88 - GPIO Port Q4"]
    GPIOQ4 = 88,
    #[doc = "89 - GPIO Port Q5"]
    GPIOQ5 = 89,
    #[doc = "90 - GPIO Port Q6"]
    GPIOQ6 = 90,
    #[doc = "91 - GPIO Port Q7"]
    GPIOQ7 = 91,
    #[doc = "92 - GPIO Port R"]
    GPIOR = 92,
    #[doc = "93 - GPIO Port S"]
    GPIOS = 93,
    #[doc = "94 - SHA/MD5"]
    SHA0 = 94,
    #[doc = "95 - AES"]
    AES0 = 95,
    #[doc = "96 - DES"]
    DES0 = 96,
    #[doc = "97 - LCD"]
    LCD0 = 97,
    #[doc = "98 - 16/32-Bit Timer 6A"]
    TIMER6A = 98,
    #[doc = "99 - 16/32-Bit Timer 6B"]
    TIMER6B = 99,
    #[doc = "100 - 16/32-Bit Timer 7A"]
    TIMER7A = 100,
    #[doc = "101 - 16/32-Bit Timer 7B"]
    TIMER7B = 101,
    #[doc = "102 - I2C 6"]
    I2C6 = 102,
    #[doc = "103 - I2C 7"]
    I2C7 = 103,
    #[doc = "105 - 1-Wire"]
    ONEWIRE0 = 105,
    #[doc = "109 - I2C 8"]
    I2C8 = 109,
    #[doc = "110 - I2C 9"]
    I2C9 = 110,
    #[doc = "111 - GPIO T"]
    GPIOT = 111,
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
    pub const fn ptr() -> *const gpio_porta_ahb::RegisterBlock {
        0x4005_8000 as *const _
    }
}
impl Deref for GPIO_PORTA_AHB {
    type Target = gpio_porta_ahb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTA_AHB::ptr() }
    }
}
#[doc = "GPIO register offsets"]
pub mod gpio_porta_ahb;
#[doc = "GPIO register offsets"]
pub struct GPIO_PORTB_AHB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTB_AHB {}
impl GPIO_PORTB_AHB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta_ahb::RegisterBlock {
        0x4005_9000 as *const _
    }
}
impl Deref for GPIO_PORTB_AHB {
    type Target = gpio_porta_ahb::RegisterBlock;
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
    pub const fn ptr() -> *const gpio_porta_ahb::RegisterBlock {
        0x4005_a000 as *const _
    }
}
impl Deref for GPIO_PORTC_AHB {
    type Target = gpio_porta_ahb::RegisterBlock;
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
    pub const fn ptr() -> *const gpio_porta_ahb::RegisterBlock {
        0x4005_b000 as *const _
    }
}
impl Deref for GPIO_PORTD_AHB {
    type Target = gpio_porta_ahb::RegisterBlock;
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
    pub const fn ptr() -> *const gpio_porta_ahb::RegisterBlock {
        0x4005_c000 as *const _
    }
}
impl Deref for GPIO_PORTE_AHB {
    type Target = gpio_porta_ahb::RegisterBlock;
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
    pub const fn ptr() -> *const gpio_porta_ahb::RegisterBlock {
        0x4005_d000 as *const _
    }
}
impl Deref for GPIO_PORTF_AHB {
    type Target = gpio_porta_ahb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTF_AHB::ptr() }
    }
}
#[doc = "GPIO register offsets"]
pub struct GPIO_PORTG_AHB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTG_AHB {}
impl GPIO_PORTG_AHB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta_ahb::RegisterBlock {
        0x4005_e000 as *const _
    }
}
impl Deref for GPIO_PORTG_AHB {
    type Target = gpio_porta_ahb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTG_AHB::ptr() }
    }
}
#[doc = "GPIO register offsets"]
pub struct GPIO_PORTH_AHB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTH_AHB {}
impl GPIO_PORTH_AHB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta_ahb::RegisterBlock {
        0x4005_f000 as *const _
    }
}
impl Deref for GPIO_PORTH_AHB {
    type Target = gpio_porta_ahb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTH_AHB::ptr() }
    }
}
#[doc = "GPIO register offsets"]
pub struct GPIO_PORTJ_AHB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTJ_AHB {}
impl GPIO_PORTJ_AHB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta_ahb::RegisterBlock {
        0x4006_0000 as *const _
    }
}
impl Deref for GPIO_PORTJ_AHB {
    type Target = gpio_porta_ahb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTJ_AHB::ptr() }
    }
}
#[doc = "GPIO register offsets"]
pub struct GPIO_PORTK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTK {}
impl GPIO_PORTK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta_ahb::RegisterBlock {
        0x4006_1000 as *const _
    }
}
impl Deref for GPIO_PORTK {
    type Target = gpio_porta_ahb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTK::ptr() }
    }
}
#[doc = "GPIO register offsets"]
pub struct GPIO_PORTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTL {}
impl GPIO_PORTL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta_ahb::RegisterBlock {
        0x4006_2000 as *const _
    }
}
impl Deref for GPIO_PORTL {
    type Target = gpio_porta_ahb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTL::ptr() }
    }
}
#[doc = "GPIO register offsets"]
pub struct GPIO_PORTM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTM {}
impl GPIO_PORTM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta_ahb::RegisterBlock {
        0x4006_3000 as *const _
    }
}
impl Deref for GPIO_PORTM {
    type Target = gpio_porta_ahb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTM::ptr() }
    }
}
#[doc = "GPIO register offsets"]
pub struct GPIO_PORTN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTN {}
impl GPIO_PORTN {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta_ahb::RegisterBlock {
        0x4006_4000 as *const _
    }
}
impl Deref for GPIO_PORTN {
    type Target = gpio_porta_ahb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTN::ptr() }
    }
}
#[doc = "GPIO register offsets"]
pub struct GPIO_PORTP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTP {}
impl GPIO_PORTP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta_ahb::RegisterBlock {
        0x4006_5000 as *const _
    }
}
impl Deref for GPIO_PORTP {
    type Target = gpio_porta_ahb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTP::ptr() }
    }
}
#[doc = "GPIO register offsets"]
pub struct GPIO_PORTQ {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTQ {}
impl GPIO_PORTQ {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta_ahb::RegisterBlock {
        0x4006_6000 as *const _
    }
}
impl Deref for GPIO_PORTQ {
    type Target = gpio_porta_ahb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTQ::ptr() }
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
#[doc = "I2C register offsets"]
pub struct I2C8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C8 {}
impl I2C8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x400b_8000 as *const _
    }
}
impl Deref for I2C8 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C8::ptr() }
    }
}
#[doc = "I2C register offsets"]
pub struct I2C9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C9 {}
impl I2C9 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x400b_9000 as *const _
    }
}
impl Deref for I2C9 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C9::ptr() }
    }
}
#[doc = "I2C register offsets"]
pub struct I2C4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C4 {}
impl I2C4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x400c_0000 as *const _
    }
}
impl Deref for I2C4 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C4::ptr() }
    }
}
#[doc = "I2C register offsets"]
pub struct I2C5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C5 {}
impl I2C5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x400c_1000 as *const _
    }
}
impl Deref for I2C5 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C5::ptr() }
    }
}
#[doc = "I2C register offsets"]
pub struct I2C6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C6 {}
impl I2C6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x400c_2000 as *const _
    }
}
impl Deref for I2C6 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C6::ptr() }
    }
}
#[doc = "I2C register offsets"]
pub struct I2C7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C7 {}
impl I2C7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x400c_3000 as *const _
    }
}
impl Deref for I2C7 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C7::ptr() }
    }
}
#[doc = "External Peripheral Interface register offsets"]
pub struct EPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EPI0 {}
impl EPI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const epi0::RegisterBlock {
        0x400d_0000 as *const _
    }
}
impl Deref for EPI0 {
    type Target = epi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EPI0::ptr() }
    }
}
#[doc = "External Peripheral Interface register offsets"]
pub mod epi0;
#[doc = "Timer register offsets"]
pub struct TIMER6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER6 {}
impl TIMER6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x400e_0000 as *const _
    }
}
impl Deref for TIMER6 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER6::ptr() }
    }
}
#[doc = "Timer register offsets"]
pub struct TIMER7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER7 {}
impl TIMER7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x400e_1000 as *const _
    }
}
impl Deref for TIMER7 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER7::ptr() }
    }
}
#[doc = "EMAC register offsets"]
pub struct EMAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMAC0 {}
impl EMAC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const emac0::RegisterBlock {
        0x400e_c000 as *const _
    }
}
impl Deref for EMAC0 {
    type Target = emac0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EMAC0::ptr() }
    }
}
#[doc = "EMAC register offsets"]
pub mod emac0;
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
#[doc = "EC register offsets"]
pub struct CCM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCM0 {}
impl CCM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccm0::RegisterBlock {
        0x4403_0000 as *const _
    }
}
impl Deref for CCM0 {
    type Target = ccm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCM0::ptr() }
    }
}
#[doc = "EC register offsets"]
pub mod ccm0;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "WATCHDOG0"]
    pub WATCHDOG0: WATCHDOG0,
    #[doc = "WATCHDOG1"]
    pub WATCHDOG1: WATCHDOG1,
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
    #[doc = "PWM0"]
    pub PWM0: PWM0,
    #[doc = "QEI0"]
    pub QEI0: QEI0,
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
    #[doc = "GPIO_PORTG_AHB"]
    pub GPIO_PORTG_AHB: GPIO_PORTG_AHB,
    #[doc = "GPIO_PORTH_AHB"]
    pub GPIO_PORTH_AHB: GPIO_PORTH_AHB,
    #[doc = "GPIO_PORTJ_AHB"]
    pub GPIO_PORTJ_AHB: GPIO_PORTJ_AHB,
    #[doc = "GPIO_PORTK"]
    pub GPIO_PORTK: GPIO_PORTK,
    #[doc = "GPIO_PORTL"]
    pub GPIO_PORTL: GPIO_PORTL,
    #[doc = "GPIO_PORTM"]
    pub GPIO_PORTM: GPIO_PORTM,
    #[doc = "GPIO_PORTN"]
    pub GPIO_PORTN: GPIO_PORTN,
    #[doc = "GPIO_PORTP"]
    pub GPIO_PORTP: GPIO_PORTP,
    #[doc = "GPIO_PORTQ"]
    pub GPIO_PORTQ: GPIO_PORTQ,
    #[doc = "EEPROM"]
    pub EEPROM: EEPROM,
    #[doc = "I2C8"]
    pub I2C8: I2C8,
    #[doc = "I2C9"]
    pub I2C9: I2C9,
    #[doc = "I2C4"]
    pub I2C4: I2C4,
    #[doc = "I2C5"]
    pub I2C5: I2C5,
    #[doc = "I2C6"]
    pub I2C6: I2C6,
    #[doc = "I2C7"]
    pub I2C7: I2C7,
    #[doc = "EPI0"]
    pub EPI0: EPI0,
    #[doc = "TIMER6"]
    pub TIMER6: TIMER6,
    #[doc = "TIMER7"]
    pub TIMER7: TIMER7,
    #[doc = "EMAC0"]
    pub EMAC0: EMAC0,
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
    #[doc = "CCM0"]
    pub CCM0: CCM0,
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
            PWM0: PWM0 { _marker: PhantomData },
            QEI0: QEI0 { _marker: PhantomData },
            TIMER0: TIMER0 { _marker: PhantomData },
            TIMER1: TIMER1 { _marker: PhantomData },
            TIMER2: TIMER2 { _marker: PhantomData },
            TIMER3: TIMER3 { _marker: PhantomData },
            TIMER4: TIMER4 { _marker: PhantomData },
            TIMER5: TIMER5 { _marker: PhantomData },
            ADC0: ADC0 { _marker: PhantomData },
            ADC1: ADC1 { _marker: PhantomData },
            COMP: COMP { _marker: PhantomData },
            CAN0: CAN0 { _marker: PhantomData },
            CAN1: CAN1 { _marker: PhantomData },
            USB0: USB0 { _marker: PhantomData },
            GPIO_PORTA_AHB: GPIO_PORTA_AHB { _marker: PhantomData },
            GPIO_PORTB_AHB: GPIO_PORTB_AHB { _marker: PhantomData },
            GPIO_PORTC_AHB: GPIO_PORTC_AHB { _marker: PhantomData },
            GPIO_PORTD_AHB: GPIO_PORTD_AHB { _marker: PhantomData },
            GPIO_PORTE_AHB: GPIO_PORTE_AHB { _marker: PhantomData },
            GPIO_PORTF_AHB: GPIO_PORTF_AHB { _marker: PhantomData },
            GPIO_PORTG_AHB: GPIO_PORTG_AHB { _marker: PhantomData },
            GPIO_PORTH_AHB: GPIO_PORTH_AHB { _marker: PhantomData },
            GPIO_PORTJ_AHB: GPIO_PORTJ_AHB { _marker: PhantomData },
            GPIO_PORTK: GPIO_PORTK { _marker: PhantomData },
            GPIO_PORTL: GPIO_PORTL { _marker: PhantomData },
            GPIO_PORTM: GPIO_PORTM { _marker: PhantomData },
            GPIO_PORTN: GPIO_PORTN { _marker: PhantomData },
            GPIO_PORTP: GPIO_PORTP { _marker: PhantomData },
            GPIO_PORTQ: GPIO_PORTQ { _marker: PhantomData },
            EEPROM: EEPROM { _marker: PhantomData },
            I2C8: I2C8 { _marker: PhantomData },
            I2C9: I2C9 { _marker: PhantomData },
            I2C4: I2C4 { _marker: PhantomData },
            I2C5: I2C5 { _marker: PhantomData },
            I2C6: I2C6 { _marker: PhantomData },
            I2C7: I2C7 { _marker: PhantomData },
            EPI0: EPI0 { _marker: PhantomData },
            TIMER6: TIMER6 { _marker: PhantomData },
            TIMER7: TIMER7 { _marker: PhantomData },
            EMAC0: EMAC0 { _marker: PhantomData },
            SYSEXC: SYSEXC { _marker: PhantomData },
            HIB: HIB { _marker: PhantomData },
            FLASH_CTRL: FLASH_CTRL { _marker: PhantomData },
            SYSCTL: SYSCTL { _marker: PhantomData },
            UDMA: UDMA { _marker: PhantomData },
            CCM0: CCM0 { _marker: PhantomData },
        }
    }
}
