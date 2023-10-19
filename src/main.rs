use raw_cpuid::{cpuid, CpuId, Hypervisor};

// https://elixir.bootlin.com/linux/v6.6-rc6/source/arch/x86/include/asm/hyperv-tlfs.h#L169
const HYPERV_CPUID_ISOLATION_CONFIG: u32 = 0x4000000C;
const HV_ISOLATION_TYPE: u32 = 0xF;
const HV_ISOLATION_TYPE_SNP: u32 = 2;
const HV_ISOLATION_TYPE_TDX: u32 = 3;
const HYPERV_CPUID_FEATURES: u32 = 0x40000003;
const HV_ISOLATION: u32 = 1 << 22;

fn main() {
    let cpuid = CpuId::new();
    let Some(hyper_info) = cpuid.get_hypervisor_info() else {
        panic!("Not a VM");
    };
    let hypervisor = hyper_info.identify();
    if hypervisor != Hypervisor::HyperV {
        panic!("Not running on Hyper-V");
    }

    let hv_features = cpuid!(HYPERV_CPUID_FEATURES);
    if hv_features.ebx & HV_ISOLATION == 0 {
        panic!("VM is not an CVM");
    }

    let hv_isol_config = cpuid!(HYPERV_CPUID_ISOLATION_CONFIG);
    let isolation_type = hv_isol_config.ebx & HV_ISOLATION_TYPE;
    match isolation_type {
        HV_ISOLATION_TYPE_SNP => println!("VM is an SEV-SNP CVM"),
        HV_ISOLATION_TYPE_TDX => println!("VM is a TDX CVM"),
        _ => panic!("Unknown CVM type"),
    }
}
