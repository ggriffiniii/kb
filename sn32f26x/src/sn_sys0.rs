#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Offset:0x00 Analog Block Control Register"]
    pub anbctrl: crate::Reg<anbctrl::ANBCTRL_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Offset:0x08 Clock Source Status Register"]
    pub csst: crate::Reg<csst::CSST_SPEC>,
    #[doc = "0x0c - Offset:0x0C System Clock Configuration Register"]
    pub clkcfg: crate::Reg<clkcfg::CLKCFG_SPEC>,
    #[doc = "0x10 - Offset:0x10 AHB Clock Prescale Register"]
    pub ahbcp: crate::Reg<ahbcp::AHBCP_SPEC>,
    #[doc = "0x14 - Offset:0x14 System Reset Status Register"]
    pub rstst: crate::Reg<rstst::RSTST_SPEC>,
    #[doc = "0x18 - Offset:0x18 LVD Control Register"]
    pub lvdctrl: crate::Reg<lvdctrl::LVDCTRL_SPEC>,
    #[doc = "0x1c - Offset:0x1C External Reset Pin Control Register"]
    pub exrstctrl: crate::Reg<exrstctrl::EXRSTCTRL_SPEC>,
    #[doc = "0x20 - Offset:0x20 SWD Pin Control Register"]
    pub swdctrl: crate::Reg<swdctrl::SWDCTRL_SPEC>,
    #[doc = "0x24 - Offset:0x24 Interrupt Vector Table Mapping Register"]
    pub ivtm: crate::Reg<ivtm::IVTM_SPEC>,
    #[doc = "0x28 - Offset:0x28 Noise Detect Control Register"]
    pub ndtctrl: crate::Reg<ndtctrl::NDTCTRL_SPEC>,
    #[doc = "0x2c - Offset:0x2C Noise Detect Status Register"]
    pub ndtsts: crate::Reg<ndtsts::NDTSTS_SPEC>,
    #[doc = "0x30 - Offset:0x30 Anti-EFT Ability Control Register"]
    pub antieft: crate::Reg<antieft::ANTIEFT_SPEC>,
}
#[doc = "ANBCTRL register accessor: an alias for `Reg<ANBCTRL_SPEC>`"]
pub type ANBCTRL = crate::Reg<anbctrl::ANBCTRL_SPEC>;
#[doc = "Offset:0x00 Analog Block Control Register"]
pub mod anbctrl;
#[doc = "CSST register accessor: an alias for `Reg<CSST_SPEC>`"]
pub type CSST = crate::Reg<csst::CSST_SPEC>;
#[doc = "Offset:0x08 Clock Source Status Register"]
pub mod csst;
#[doc = "CLKCFG register accessor: an alias for `Reg<CLKCFG_SPEC>`"]
pub type CLKCFG = crate::Reg<clkcfg::CLKCFG_SPEC>;
#[doc = "Offset:0x0C System Clock Configuration Register"]
pub mod clkcfg;
#[doc = "AHBCP register accessor: an alias for `Reg<AHBCP_SPEC>`"]
pub type AHBCP = crate::Reg<ahbcp::AHBCP_SPEC>;
#[doc = "Offset:0x10 AHB Clock Prescale Register"]
pub mod ahbcp;
#[doc = "RSTST register accessor: an alias for `Reg<RSTST_SPEC>`"]
pub type RSTST = crate::Reg<rstst::RSTST_SPEC>;
#[doc = "Offset:0x14 System Reset Status Register"]
pub mod rstst;
#[doc = "LVDCTRL register accessor: an alias for `Reg<LVDCTRL_SPEC>`"]
pub type LVDCTRL = crate::Reg<lvdctrl::LVDCTRL_SPEC>;
#[doc = "Offset:0x18 LVD Control Register"]
pub mod lvdctrl;
#[doc = "EXRSTCTRL register accessor: an alias for `Reg<EXRSTCTRL_SPEC>`"]
pub type EXRSTCTRL = crate::Reg<exrstctrl::EXRSTCTRL_SPEC>;
#[doc = "Offset:0x1C External Reset Pin Control Register"]
pub mod exrstctrl;
#[doc = "SWDCTRL register accessor: an alias for `Reg<SWDCTRL_SPEC>`"]
pub type SWDCTRL = crate::Reg<swdctrl::SWDCTRL_SPEC>;
#[doc = "Offset:0x20 SWD Pin Control Register"]
pub mod swdctrl;
#[doc = "IVTM register accessor: an alias for `Reg<IVTM_SPEC>`"]
pub type IVTM = crate::Reg<ivtm::IVTM_SPEC>;
#[doc = "Offset:0x24 Interrupt Vector Table Mapping Register"]
pub mod ivtm;
#[doc = "NDTCTRL register accessor: an alias for `Reg<NDTCTRL_SPEC>`"]
pub type NDTCTRL = crate::Reg<ndtctrl::NDTCTRL_SPEC>;
#[doc = "Offset:0x28 Noise Detect Control Register"]
pub mod ndtctrl;
#[doc = "NDTSTS register accessor: an alias for `Reg<NDTSTS_SPEC>`"]
pub type NDTSTS = crate::Reg<ndtsts::NDTSTS_SPEC>;
#[doc = "Offset:0x2C Noise Detect Status Register"]
pub mod ndtsts;
#[doc = "ANTIEFT register accessor: an alias for `Reg<ANTIEFT_SPEC>`"]
pub type ANTIEFT = crate::Reg<antieft::ANTIEFT_SPEC>;
#[doc = "Offset:0x30 Anti-EFT Ability Control Register"]
pub mod antieft;
