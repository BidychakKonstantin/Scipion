#![allow(dead_code)]

use core::arch::x86_64::{__cpuid, _mm_mfence};
use x86_64::registers::model_specific::Msr;

const MSR_IA32_APIC_BASE: u32 = 0x1B;

const APIC_BASE_BSP:    u64 = 1 << 8;   
const APIC_BASE_EXTD:   u64 = 1 << 10;  
const APIC_BASE_ENABLE: u64 = 1 << 11;  
const APIC_BASE_ADDR_MASK: u64 = 0xFFFF_FFFF_F000; 

const MSR_X2APIC_ID:       u32 = 0x802;
const MSR_X2APIC_VERSION:  u32 = 0x803;
const MSR_X2APIC_TPR:      u32 = 0x808; 
const MSR_X2APIC_PPR:      u32 = 0x80A; 
const MSR_X2APIC_EOI:      u32 = 0x80B; 
const MSR_X2APIC_LDR:      u32 = 0x80D; 