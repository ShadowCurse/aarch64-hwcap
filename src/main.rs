use clap::Parser;

const HWCAP_FP: u64 = 1 << 0;
const HWCAP_ASIMD: u64 = 1 << 1;
const HWCAP_EVTSTRM: u64 = 1 << 2;
const HWCAP_AES: u64 = 1 << 3;
const HWCAP_PMULL: u64 = 1 << 4;
const HWCAP_SHA1: u64 = 1 << 5;
const HWCAP_SHA2: u64 = 1 << 6;
const HWCAP_CRC32: u64 = 1 << 7;
const HWCAP_ATOMICS: u64 = 1 << 8;
const HWCAP_FPHP: u64 = 1 << 9;
const HWCAP_ASIMDHP: u64 = 1 << 10;
const HWCAP_CPUID: u64 = 1 << 11;
const HWCAP_ASIMDRDM: u64 = 1 << 12;
const HWCAP_JSCVT: u64 = 1 << 13;
const HWCAP_FCMA: u64 = 1 << 14;
const HWCAP_LRCPC: u64 = 1 << 15;
const HWCAP_DCPOP: u64 = 1 << 16;
const HWCAP_SHA3: u64 = 1 << 17;
const HWCAP_SM3: u64 = 1 << 18;
const HWCAP_SM4: u64 = 1 << 19;
const HWCAP_ASIMDDP: u64 = 1 << 20;
const HWCAP_SHA512: u64 = 1 << 21;
const HWCAP_SVE: u64 = 1 << 22;
const HWCAP_ASIMDFHM: u64 = 1 << 23;
const HWCAP_DIT: u64 = 1 << 24;
const HWCAP_USCAT: u64 = 1 << 25;
const HWCAP_ILRCPC: u64 = 1 << 26;
const HWCAP_FLAGM: u64 = 1 << 27;
const HWCAP_SSBS: u64 = 1 << 28;
const HWCAP_SB: u64 = 1 << 29;
const HWCAP_PACA: u64 = 1 << 30;
const HWCAP_PACG: u64 = 1 << 31;

const HWCAP_ALL: [(u64, &str); 32] = [
    (HWCAP_FP, "HWCAP_FP"),
    (HWCAP_ASIMD, "HWCAP_ASIMD"),
    (HWCAP_EVTSTRM, "HWCAP_EVTSTRM"),
    (HWCAP_AES, "HWCAP_AES"),
    (HWCAP_PMULL, "HWCAP_PMULL"),
    (HWCAP_SHA1, "HWCAP_SHA1"),
    (HWCAP_SHA2, "HWCAP_SHA2"),
    (HWCAP_CRC32, "HWCAP_CRC32"),
    (HWCAP_ATOMICS, "HWCAP_ATOMICS"),
    (HWCAP_FPHP, "HWCAP_FPHP"),
    (HWCAP_ASIMDHP, "HWCAP_ASIMDHP"),
    (HWCAP_CPUID, "HWCAP_CPUID"),
    (HWCAP_ASIMDRDM, "HWCAP_ASIMDRDM"),
    (HWCAP_JSCVT, "HWCAP_JSCVT"),
    (HWCAP_FCMA, "HWCAP_FCMA"),
    (HWCAP_LRCPC, "HWCAP_LRCPC"),
    (HWCAP_DCPOP, "HWCAP_DCPOP"),
    (HWCAP_SHA3, "HWCAP_SHA3"),
    (HWCAP_SM3, "HWCAP_SM3"),
    (HWCAP_SM4, "HWCAP_SM4"),
    (HWCAP_ASIMDDP, "HWCAP_ASIMDDP"),
    (HWCAP_SHA512, "HWCAP_SHA512"),
    (HWCAP_SVE, "HWCAP_SVE"),
    (HWCAP_ASIMDFHM, "HWCAP_ASIMDFHM"),
    (HWCAP_DIT, "HWCAP_DIT"),
    (HWCAP_USCAT, "HWCAP_USCAT"),
    (HWCAP_ILRCPC, "HWCAP_ILRCPC"),
    (HWCAP_FLAGM, "HWCAP_FLAGM"),
    (HWCAP_SSBS, "HWCAP_SSBS"),
    (HWCAP_SB, "HWCAP_SB"),
    (HWCAP_PACA, "HWCAP_PACA"),
    (HWCAP_PACG, "HWCAP_PACG"),
];

const HWCAP2_DCPODP: u64 = 1 << 0;
const HWCAP2_SVE2: u64 = 1 << 1;
const HWCAP2_SVEAES: u64 = 1 << 2;
const HWCAP2_SVEPMULL: u64 = 1 << 3;
const HWCAP2_SVEBITPERM: u64 = 1 << 4;
const HWCAP2_SVESHA3: u64 = 1 << 5;
const HWCAP2_SVESM4: u64 = 1 << 6;
const HWCAP2_FLAGM2: u64 = 1 << 7;
const HWCAP2_FRINT: u64 = 1 << 8;
const HWCAP2_SVEI8MM: u64 = 1 << 9;
const HWCAP2_SVEF32MM: u64 = 1 << 10;
const HWCAP2_SVEF64MM: u64 = 1 << 11;
const HWCAP2_SVEBF16: u64 = 1 << 12;
const HWCAP2_I8MM: u64 = 1 << 13;
const HWCAP2_BF16: u64 = 1 << 14;
const HWCAP2_DGH: u64 = 1 << 15;
const HWCAP2_RNG: u64 = 1 << 16;
const HWCAP2_BTI: u64 = 1 << 17;
const HWCAP2_MTE: u64 = 1 << 18;
const HWCAP2_ECV: u64 = 1 << 19;
const HWCAP2_AFP: u64 = 1 << 20;
const HWCAP2_RPRES: u64 = 1 << 21;
const HWCAP2_MTE3: u64 = 1 << 22;
const HWCAP2_SME: u64 = 1 << 23;
const HWCAP2_SME_I16I64: u64 = 1 << 24;
const HWCAP2_SME_F64F64: u64 = 1 << 25;
const HWCAP2_SME_I8I32: u64 = 1 << 26;
const HWCAP2_SME_F16F32: u64 = 1 << 27;
const HWCAP2_SME_B16F32: u64 = 1 << 28;
const HWCAP2_SME_F32F32: u64 = 1 << 29;
const HWCAP2_SME_FA64: u64 = 1 << 30;
const HWCAP2_WFXT: u64 = 1 << 31;
const HWCAP2_EBF16: u64 = 1 << 32;
const HWCAP2_SVE_EBF16: u64 = 1 << 33;
const HWCAP2_CSSC: u64 = 1 << 34;
const HWCAP2_RPRFM: u64 = 1 << 35;
const HWCAP2_SVE2P1: u64 = 1 << 36;

const HWCAP2_ALL: [(u64, &str); 37] = [
    (HWCAP2_DCPODP, "HWCAP2_DCPODP"),
    (HWCAP2_SVE2, "HWCAP2_SVE2"),
    (HWCAP2_SVEAES, "HWCAP2_SVEAES"),
    (HWCAP2_SVEPMULL, "HWCAP2_SVEPMULL"),
    (HWCAP2_SVEBITPERM, "HWCAP2_SVEBITPERM"),
    (HWCAP2_SVESHA3, "HWCAP2_SVESHA3"),
    (HWCAP2_SVESM4, "HWCAP2_SVESM4"),
    (HWCAP2_FLAGM2, "HWCAP2_FLAGM2"),
    (HWCAP2_FRINT, "HWCAP2_FRINT"),
    (HWCAP2_SVEI8MM, "HWCAP2_SVEI8MM"),
    (HWCAP2_SVEF32MM, "HWCAP2_SVEF32MM"),
    (HWCAP2_SVEF64MM, "HWCAP2_SVEF64MM"),
    (HWCAP2_SVEBF16, "HWCAP2_SVEBF16"),
    (HWCAP2_I8MM, "HWCAP2_I8MM"),
    (HWCAP2_BF16, "HWCAP2_BF16"),
    (HWCAP2_DGH, "HWCAP2_DGH"),
    (HWCAP2_RNG, "HWCAP2_RNG"),
    (HWCAP2_BTI, "HWCAP2_BTI"),
    (HWCAP2_MTE, "HWCAP2_MTE"),
    (HWCAP2_ECV, "HWCAP2_ECV"),
    (HWCAP2_AFP, "HWCAP2_AFP"),
    (HWCAP2_RPRES, "HWCAP2_RPRES"),
    (HWCAP2_MTE3, "HWCAP2_MTE3"),
    (HWCAP2_SME, "HWCAP2_SME"),
    (HWCAP2_SME_I16I64, "HWCAP2_SME_I16I64"),
    (HWCAP2_SME_F64F64, "HWCAP2_SME_F64F64"),
    (HWCAP2_SME_I8I32, "HWCAP2_SME_I8I32"),
    (HWCAP2_SME_F16F32, "HWCAP2_SME_F16F32"),
    (HWCAP2_SME_B16F32, "HWCAP2_SME_B16F32"),
    (HWCAP2_SME_F32F32, "HWCAP2_SME_F32F32"),
    (HWCAP2_SME_FA64, "HWCAP2_SME_FA64"),
    (HWCAP2_WFXT, "HWCAP2_WFXT"),
    (HWCAP2_EBF16, "HWCAP2_EBF16"),
    (HWCAP2_SVE_EBF16, "HWCAP2_SVE_EBF16"),
    (HWCAP2_CSSC, "HWCAP2_CSSC"),
    (HWCAP2_RPRFM, "HWCAP2_RPRFM"),
    (HWCAP2_SVE2P1, "HWCAP2_SVE2P1"),
];

fn hwcap() -> Vec<[&'static str; 2]> {
    let hwcap = unsafe { libc::getauxval(libc::AT_HWCAP) };

    HWCAP_ALL
        .iter()
        .map(|(cap, cap_name)| {
            if hwcap & cap != 0 {
                [*cap_name, "Yes"]
            } else {
                [*cap_name, "No"]
            }
        })
        .collect()
}

fn hwcap2() -> Vec<[&'static str; 2]> {
    let hwcap2 = unsafe { libc::getauxval(libc::AT_HWCAP2) };

    HWCAP2_ALL
        .iter()
        .map(|(cap, cap_name)| {
            if hwcap2 & cap != 0 {
                [*cap_name, "Yes"]
            } else {
                [*cap_name, "No"]
            }
        })
        .collect()
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    table: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.table {
        let mut hwcap_table = comfy_table::Table::default();
        for cap in hwcap() {
            hwcap_table.add_row(cap);
        }
        println!("{hwcap_table}");

        let mut hwcap2_table = comfy_table::Table::default();
        for cap in hwcap2() {
            hwcap2_table.add_row(cap);
        }
        println!("{hwcap2_table}");
    } else {
        for cap in hwcap() {
            println!("{} {}", cap[0], cap[1])
        }
        for cap in hwcap2() {
            println!("{} {}", cap[0], cap[1])
        }
    }
}
