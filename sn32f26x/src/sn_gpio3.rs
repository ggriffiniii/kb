#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Offset:0x00 GPIO Port n Data Register"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x04 - Offset:0x04 GPIO Port n Mode Register"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x08 - Offset:0x08 GPIO Port n Configuration Register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x0c - Offset:0x0C GPIO Port n Interrupt Sense Register"]
    pub is: crate::Reg<is::IS_SPEC>,
    #[doc = "0x10 - Offset:0x10 GPIO Port n Interrupt Both-edge Sense Register"]
    pub ibs: crate::Reg<ibs::IBS_SPEC>,
    #[doc = "0x14 - Offset:0x14 GPIO Port n Interrupt Event Register"]
    pub iev: crate::Reg<iev::IEV_SPEC>,
    #[doc = "0x18 - Offset:0x18 GPIO Port n Interrupt Enable Register"]
    pub ie: crate::Reg<ie::IE_SPEC>,
    #[doc = "0x1c - Offset:0x1C GPIO Port n Raw Interrupt Status Register"]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x20 - Offset:0x20 GPIO Port n Interrupt Clear Register"]
    pub ic: crate::Reg<ic::IC_SPEC>,
    #[doc = "0x24 - Offset:0x24 GPIO Port n Bits Set Operation Register"]
    pub bset: crate::Reg<bset::BSET_SPEC>,
    #[doc = "0x28 - Offset:0x28 GPIO Port n Bits Clear Operation Register"]
    pub bclr: crate::Reg<bclr::BCLR_SPEC>,
}
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Offset:0x00 GPIO Port n Data Register"]
pub mod data;
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Offset:0x04 GPIO Port n Mode Register"]
pub mod mode;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Offset:0x08 GPIO Port n Configuration Register"]
pub mod cfg;
#[doc = "IS register accessor: an alias for `Reg<IS_SPEC>`"]
pub type IS = crate::Reg<is::IS_SPEC>;
#[doc = "Offset:0x0C GPIO Port n Interrupt Sense Register"]
pub mod is;
#[doc = "IBS register accessor: an alias for `Reg<IBS_SPEC>`"]
pub type IBS = crate::Reg<ibs::IBS_SPEC>;
#[doc = "Offset:0x10 GPIO Port n Interrupt Both-edge Sense Register"]
pub mod ibs;
#[doc = "IEV register accessor: an alias for `Reg<IEV_SPEC>`"]
pub type IEV = crate::Reg<iev::IEV_SPEC>;
#[doc = "Offset:0x14 GPIO Port n Interrupt Event Register"]
pub mod iev;
#[doc = "IE register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "Offset:0x18 GPIO Port n Interrupt Enable Register"]
pub mod ie;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Offset:0x1C GPIO Port n Raw Interrupt Status Register"]
pub mod ris;
#[doc = "IC register accessor: an alias for `Reg<IC_SPEC>`"]
pub type IC = crate::Reg<ic::IC_SPEC>;
#[doc = "Offset:0x20 GPIO Port n Interrupt Clear Register"]
pub mod ic;
#[doc = "BSET register accessor: an alias for `Reg<BSET_SPEC>`"]
pub type BSET = crate::Reg<bset::BSET_SPEC>;
#[doc = "Offset:0x24 GPIO Port n Bits Set Operation Register"]
pub mod bset;
#[doc = "BCLR register accessor: an alias for `Reg<BCLR_SPEC>`"]
pub type BCLR = crate::Reg<bclr::BCLR_SPEC>;
#[doc = "Offset:0x28 GPIO Port n Bits Clear Operation Register"]
pub mod bclr;
