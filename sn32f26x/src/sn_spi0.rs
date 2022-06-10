#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Offset:0x00 SPIn Control Register 0"]
    pub ctrl0: crate::Reg<ctrl0::CTRL0_SPEC>,
    #[doc = "0x04 - Offset:0x04 SPIn Control Register 1"]
    pub ctrl1: crate::Reg<ctrl1::CTRL1_SPEC>,
    #[doc = "0x08 - Offset:0x08 SPIn Clock Divider Register"]
    pub clkdiv: crate::Reg<clkdiv::CLKDIV_SPEC>,
    #[doc = "0x0c - Offset:0x0C SPIn Status Register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x10 - Offset:0x10 SPIn Interrupt Enable Register"]
    pub ie: crate::Reg<ie::IE_SPEC>,
    #[doc = "0x14 - Offset:0x14 SPIn Raw Interrupt Status Register"]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x18 - Offset:0x18 SPIn Interrupt Clear Register"]
    pub ic: crate::Reg<ic::IC_SPEC>,
    #[doc = "0x1c - Offset:0x1C SPIn Data Register"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x20 - Offset:0x20 SPIn Data Fetch Register"]
    pub df: crate::Reg<df::DF_SPEC>,
}
#[doc = "CTRL0 register accessor: an alias for `Reg<CTRL0_SPEC>`"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "Offset:0x00 SPIn Control Register 0"]
pub mod ctrl0;
#[doc = "CTRL1 register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Offset:0x04 SPIn Control Register 1"]
pub mod ctrl1;
#[doc = "CLKDIV register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Offset:0x08 SPIn Clock Divider Register"]
pub mod clkdiv;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Offset:0x0C SPIn Status Register"]
pub mod stat;
#[doc = "IE register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "Offset:0x10 SPIn Interrupt Enable Register"]
pub mod ie;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Offset:0x14 SPIn Raw Interrupt Status Register"]
pub mod ris;
#[doc = "IC register accessor: an alias for `Reg<IC_SPEC>`"]
pub type IC = crate::Reg<ic::IC_SPEC>;
#[doc = "Offset:0x18 SPIn Interrupt Clear Register"]
pub mod ic;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Offset:0x1C SPIn Data Register"]
pub mod data;
#[doc = "DF register accessor: an alias for `Reg<DF_SPEC>`"]
pub type DF = crate::Reg<df::DF_SPEC>;
#[doc = "Offset:0x20 SPIn Data Fetch Register"]
pub mod df;
