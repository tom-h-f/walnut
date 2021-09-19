use core::arch::x86_64::__cpuid;

pub fn apic_support() -> bool {
    check(1, feature_constants::CPUID_FEAT_EDX_APIC, Reg::Edx)
}

enum Reg {
    Eax,
    Ebx,
    Ecx,
    Edx,
}

fn check(leaf: u32, bit: u32, reg: Reg) -> bool {
    let res = unsafe { __cpuid(leaf) };
    let out = match reg {
        Reg::Eax => res.eax,
        Reg::Ebx => res.ebx,
        Reg::Ecx => res.ecx,
        Reg::Edx => res.edx,
    };
    out & bit == 1
}

#[allow(dead_code)]
mod feature_constants {
    pub const CPUID_FEAT_ECX_SSE3: u32 = 1 << 0;
    pub const CPUID_FEAT_ECX_PCLMUL: u32 = 1 << 1;
    pub const CPUID_FEAT_ECX_DTES64: u32 = 1 << 2;
    pub const CPUID_FEAT_ECX_MONITOR: u32 = 1 << 3;
    pub const CPUID_FEAT_ECX_DS_CPL: u32 = 1 << 4;
    pub const CPUID_FEAT_ECX_VMX: u32 = 1 << 5;
    pub const CPUID_FEAT_ECX_SMX: u32 = 1 << 6;
    pub const CPUID_FEAT_ECX_EST: u32 = 1 << 7;
    pub const CPUID_FEAT_ECX_TM2: u32 = 1 << 8;
    pub const CPUID_FEAT_ECX_SSSE3: u32 = 1 << 9;
    pub const CPUID_FEAT_ECX_CID: u32 = 1 << 10;
    pub const CPUID_FEAT_ECX_FMA: u32 = 1 << 12;
    pub const CPUID_FEAT_ECX_CX16: u32 = 1 << 13;
    pub const CPUID_FEAT_ECX_ETPRD: u32 = 1 << 14;
    pub const CPUID_FEAT_ECX_PDCM: u32 = 1 << 15;
    pub const CPUID_FEAT_ECX_PCIDE: u32 = 1 << 17;
    pub const CPUID_FEAT_ECX_DCA: u32 = 1 << 18;
    pub const CPUID_FEAT_ECX_SSE4_1: u32 = 1 << 19;
    pub const CPUID_FEAT_ECX_SSE4_2: u32 = 1 << 20;
    pub const CPUID_FEAT_ECX_x2APIC: u32 = 1 << 21;
    pub const CPUID_FEAT_ECX_MOVBE: u32 = 1 << 22;
    pub const CPUID_FEAT_ECX_POPCNT: u32 = 1 << 23;
    pub const CPUID_FEAT_ECX_AES: u32 = 1 << 25;
    pub const CPUID_FEAT_ECX_XSAVE: u32 = 1 << 26;
    pub const CPUID_FEAT_ECX_OSXSAVE: u32 = 1 << 27;
    pub const CPUID_FEAT_ECX_AVX: u32 = 1 << 28;
    pub const CPUID_FEAT_EDX_FPU: u32 = 1 << 0;
    pub const CPUID_FEAT_EDX_VME: u32 = 1 << 1;
    pub const CPUID_FEAT_EDX_DE: u32 = 1 << 2;
    pub const CPUID_FEAT_EDX_PSE: u32 = 1 << 3;
    pub const CPUID_FEAT_EDX_TSC: u32 = 1 << 4;
    pub const CPUID_FEAT_EDX_MSR: u32 = 1 << 5;
    pub const CPUID_FEAT_EDX_PAE: u32 = 1 << 6;
    pub const CPUID_FEAT_EDX_MCE: u32 = 1 << 7;
    pub const CPUID_FEAT_EDX_CX8: u32 = 1 << 8;
    pub const CPUID_FEAT_EDX_APIC: u32 = 1 << 9;
    pub const CPUID_FEAT_EDX_SEP: u32 = 1 << 11;
    pub const CPUID_FEAT_EDX_MTRR: u32 = 1 << 12;
    pub const CPUID_FEAT_EDX_PGE: u32 = 1 << 13;
    pub const CPUID_FEAT_EDX_MCA: u32 = 1 << 14;
    pub const CPUID_FEAT_EDX_CMOV: u32 = 1 << 15;
    pub const CPUID_FEAT_EDX_PAT: u32 = 1 << 16;
    pub const CPUID_FEAT_EDX_PSE36: u32 = 1 << 17;
    pub const CPUID_FEAT_EDX_PSN: u32 = 1 << 18;
    pub const CPUID_FEAT_EDX_CLF: u32 = 1 << 19;
    pub const CPUID_FEAT_EDX_DTES: u32 = 1 << 21;
    pub const CPUID_FEAT_EDX_ACPI: u32 = 1 << 22;
    pub const CPUID_FEAT_EDX_MMX: u32 = 1 << 23;
    pub const CPUID_FEAT_EDX_FXSR: u32 = 1 << 24;
    pub const CPUID_FEAT_EDX_SSE: u32 = 1 << 25;
    pub const CPUID_FEAT_EDX_SSE2: u32 = 1 << 26;
    pub const CPUID_FEAT_EDX_SS: u32 = 1 << 27;
    pub const CPUID_FEAT_EDX_HTT: u32 = 1 << 28;
    pub const CPUID_FEAT_EDX_TM1: u32 = 1 << 29;
    pub const CPUID_FEAT_EDX_IA64: u32 = 1 << 30;
    pub const CPUID_FEAT_EDX_PBE: u32 = 1 << 31;
}
