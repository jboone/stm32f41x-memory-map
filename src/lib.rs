//! Memory map for STM32F41X microcontrollers

// #![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate volatile_register;

#[allow(missing_docs)]
pub mod dma2;
#[allow(missing_docs)]
pub mod dma1;
#[allow(missing_docs)]
pub mod rcc;
#[allow(missing_docs)]
pub mod gpioi;
#[allow(missing_docs)]
pub mod gpioh;
#[allow(missing_docs)]
pub mod gpiog;
#[allow(missing_docs)]
pub mod gpiof;
#[allow(missing_docs)]
pub mod gpioe;
#[allow(missing_docs)]
pub mod gpiod;
#[allow(missing_docs)]
pub mod gpioc;
#[allow(missing_docs)]
pub mod gpiob;
#[allow(missing_docs)]
pub mod gpioa;
#[allow(missing_docs)]
pub mod tim1;
#[allow(missing_docs)]
pub mod tim8;
#[allow(missing_docs)]
pub mod flash;

use dma2::Dma2;
use dma1::Dma1;
use rcc::Rcc;
use gpioi::Gpioi;
use gpioh::Gpioh;
use gpiog::Gpiog;
use gpiof::Gpiof;
use gpioe::Gpioe;
use gpiod::Gpiod;
use gpioc::Gpioc;
use gpiob::Gpiob;
use gpioa::Gpioa;
use tim1::Tim1;
use tim8::Tim8;
use flash::Flash;

// const RNG: usize = 0x50060800;
// const HASH: usize = 0x50060400;
// const CRYP: usize = 0x50060000;
// const DCMI: usize = 0x50050000;
// const FSMC: usize = 0xa0000000;
// const DBG: usize = 0xe0042000;
const DMA2: usize = 0x40026400;
const DMA1: usize = 0x40026000;
const RCC: usize = 0x40023800;
const GPIOI: usize = 0x40022000;
const GPIOH: usize = 0x40021c00;
const GPIOG: usize = 0x40021800;
const GPIOF: usize = 0x40021400;
const GPIOE: usize = 0x40021000;
const GPIOD: usize = 0x40020c00;
const GPIOC: usize = 0x40020800;
const GPIOB: usize = 0x40020400;
const GPIOA: usize = 0x40020000;
// const SYSCFG: usize = 0x40013800;
// const SPI1: usize = 0x40013000;
// const SPI2: usize = 0x40003800;
// const SPI3: usize = 0x40003c00;
// const I2S2ext: usize = 0x40003400;
// const I2S3ext: usize = 0x40004000;
// const SDIO: usize = 0x40012c00;
// const ADC1: usize = 0x40012000;
// const ADC2: usize = 0x40012100;
// const ADC3: usize = 0x40012200;
// const USART6: usize = 0x40011400;
// const USART1: usize = 0x40011000;
// const USART2: usize = 0x40004400;
// const USART3: usize = 0x40004800;
// const DAC: usize = 0x40007400;
// const PWR: usize = 0x40007000;
// const I2C3: usize = 0x40005c00;
// const I2C2: usize = 0x40005800;
// const I2C1: usize = 0x40005400;
// const IWDG: usize = 0x40003000;
// const WWDG: usize = 0x40002c00;
// const RTC: usize = 0x40002800;
// const UART4: usize = 0x40004c00;
// const UART5: usize = 0x40005000;
// const C_ADC: usize = 0x40012300;
const TIM1: usize = 0x40010000;
const TIM8: usize = 0x40010400;
// const TIM2: usize = 0x40000000;
// const TIM3: usize = 0x40000400;
// const TIM4: usize = 0x40000800;
// const TIM5: usize = 0x40000c00;
// const TIM9: usize = 0x40014000;
// const TIM12: usize = 0x40001800;
// const TIM10: usize = 0x40014400;
// const TIM13: usize = 0x40001c00;
// const TIM14: usize = 0x40002000;
// const TIM11: usize = 0x40014800;
// const TIM6: usize = 0x40001000;
// const TIM7: usize = 0x40001400;
// const Ethernet_MAC: usize = 0x40028000;
// const Ethernet_MMC: usize = 0x40028100;
// const Ethernet_PTP: usize = 0x40028700;
// const Ethernet_DMA: usize = 0x40029000;
// const CRC: usize = 0x40023000;
// const OTG_FS_GLOBAL: usize = 0x50000000;
// const OTG_FS_HOST: usize = 0x50000400;
// const OTG_FS_DEVICE: usize = 0x50000800;
// const OTG_FS_PWRCLK: usize = 0x50000e00;
// const CAN1: usize = 0x40006400;
// const CAN2: usize = 0x40006800;
// const NVIC: usize = 0xe000e000;
const FLASH: usize = 0x40023c00;
// const EXTI: usize = 0x40013c00;
// const OTG_HS_GLOBAL: usize = 0x40040000;
// const OTG_HS_HOST: usize = 0x40040400;
// const OTG_HS_DEVICE: usize = 0x40040800;
// const OTG_HS_PWRCLK: usize = 0x40040e00;

/// DMA2 register block (&'static)
pub fn dma2() -> &'static Dma2 {
    unsafe { deref(DMA2) }
}

/// DMA2 register block (&'static mut)
pub unsafe fn dma2_mut() -> &'static mut Dma2 {
    deref_mut(DMA2)
}

/// DMA1 register block (&'static)
pub fn dma1() -> &'static Dma1 {
    unsafe { deref(DMA1) }
}

/// DMA1 register block (&'static mut)
pub unsafe fn dma1_mut() -> &'static mut Dma1 {
    deref_mut(DMA1)
}

/// RCC register block (&'static)
pub fn rcc() -> &'static Rcc {
    unsafe { deref(RCC) }
}

/// RCC register block (&'static mut)
pub unsafe fn rcc_mut() -> &'static mut Rcc {
    deref_mut(RCC)
}

/// GPIOI register block (&'static)
pub fn gpioi() -> &'static Gpioi {
    unsafe { deref(GPIOI) }
}

/// GPIOI register block (&'static mut)
pub unsafe fn gpioi_mut() -> &'static mut Gpioi {
    deref_mut(GPIOI)
}

// GPIOH register block (&'static)
pub fn gpioh() -> &'static Gpioh {
    unsafe { deref(GPIOH) }
}

// GPIOH register block (&'static mut)
pub unsafe fn gpioh_mut() -> &'static mut Gpioh {
    deref_mut(GPIOH)
}

// GPIOG register block (&'static)
pub fn gpiog() -> &'static Gpiog {
    unsafe { deref(GPIOG) }
}

// GPIOG register block (&'static mut)
pub unsafe fn gpiog_mut() -> &'static mut Gpiog {
    deref_mut(GPIOG)
}

// GPIOF register block (&'static)
pub fn gpiof() -> &'static Gpiof {
    unsafe { deref(GPIOF) }
}

// GPIOF register block (&'static mut)
pub unsafe fn gpiof_mut() -> &'static mut Gpiof {
    deref_mut(GPIOF)
}

// GPIOE register block (&'static)
pub fn gpioe() -> &'static Gpioe {
    unsafe { deref(GPIOE) }
}

// GPIOE register block (&'static mut)
pub unsafe fn gpioe_mut() -> &'static mut Gpioe {
    deref_mut(GPIOE)
}

// GPIOD register block (&'static)
pub fn gpiod() -> &'static Gpiod {
    unsafe { deref(GPIOD) }
}

// GPIOD register block (&'static mut)
pub unsafe fn gpiod_mut() -> &'static mut Gpiod {
    deref_mut(GPIOD)
}

/// GPIOC register block (&'static)
pub fn gpioc() -> &'static Gpioc {
    unsafe { deref(GPIOC) }
}

/// GPIOC register block (&'static mut)
pub unsafe fn gpioc_mut() -> &'static mut Gpioc {
    deref_mut(GPIOC)
}

/// GPIOB register block (&'static)
pub fn gpiob() -> &'static Gpiob {
    unsafe { deref(GPIOB) }
}

/// GPIOB register block (&'static mut)
pub unsafe fn gpiob_mut() -> &'static mut Gpiob {
    deref_mut(GPIOB)
}

/// GPIOA register block (&'static)
pub fn gpioa() -> &'static Gpioa {
    unsafe { deref(GPIOA) }
}

/// GPIOA register block (&'static mut)
pub unsafe fn gpioa_mut() -> &'static mut Gpioa {
    deref_mut(GPIOA)
}

/// TIM1 register block (&'static)
pub fn tim1() -> &'static Tim1 {
    unsafe { deref(TIM1) }
}

/// TIM1 register block (&'static mut)
pub unsafe fn tim1_mut() -> &'static mut Tim1 {
    deref_mut(TIM1)
}

/// TIM8 register block (&'static)
pub fn tim8() -> &'static Tim8 {
    unsafe { deref(TIM8) }
}

/// TIM8 register block (&'static mut)
pub unsafe fn tim8_mut() -> &'static mut Tim8 {
    deref_mut(TIM8)
}

/// FLASH register block (&'static)
pub fn flash() -> &'static Flash {
    unsafe { deref(FLASH) }
}

/// FLASH register block (&'static mut)
pub unsafe fn flash_mut() -> &'static mut Flash {
    deref_mut(FLASH)
}

unsafe fn deref<T>(address: usize) -> &'static T {
    &*(address as *const T)
}

unsafe fn deref_mut<T>(address: usize) -> &'static mut T {
    &mut *(address as *mut T)
}

// Here we extend the peripheral API -- AKA ~~svd2rust is~~ SVD files are great
// but not perfect
