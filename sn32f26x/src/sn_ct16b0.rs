#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Offset:0x00 CT16Bn Timer Control Register"]
    pub tmrctrl: crate::Reg<tmrctrl::TMRCTRL_SPEC>,
    #[doc = "0x04 - Offset:0x04 CT16Bn Timer Counter Register"]
    pub tc: crate::Reg<tc::TC_SPEC>,
    #[doc = "0x08 - Offset:0x08 CT16Bn Prescale Register"]
    pub pre: crate::Reg<pre::PRE_SPEC>,
    #[doc = "0x0c - Offset:0x0C CT16Bn Prescale Counter Register"]
    pub pc: crate::Reg<pc::PC_SPEC>,
    #[doc = "0x10 - Offset:0x10 CT16Bn Counter Control Register"]
    pub cntctrl: crate::Reg<cntctrl::CNTCTRL_SPEC>,
    #[doc = "0x14 - Offset:0x14 CT16Bn Match Control Register"]
    pub mctrl: crate::Reg<mctrl::MCTRL_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - Offset:0x20 CT16Bn MR0 Register"]
    pub mr0: crate::Reg<mr0::MR0_SPEC>,
    _reserved7: [u8; 0x5c],
    #[doc = "0x80 - Offset:0x80 CT16Bn Capture Control Register"]
    pub capctrl: crate::Reg<capctrl::CAPCTRL_SPEC>,
    #[doc = "0x84 - Offset:0x84 CT16Bn CAP0 Register"]
    pub cap0: crate::Reg<cap0::CAP0_SPEC>,
    _reserved9: [u8; 0x1c],
    #[doc = "0xa4 - Offset:0xA4 CT16Bn Raw Interrupt Status Register"]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0xa8 - Offset:0xA8 CT16Bn Interrupt Clear Register"]
    pub ic: crate::Reg<ic::IC_SPEC>,
}
#[doc = "TMRCTRL register accessor: an alias for `Reg<TMRCTRL_SPEC>`"]
pub type TMRCTRL = crate::Reg<tmrctrl::TMRCTRL_SPEC>;
#[doc = "Offset:0x00 CT16Bn Timer Control Register"]
pub mod tmrctrl;
#[doc = "TC register accessor: an alias for `Reg<TC_SPEC>`"]
pub type TC = crate::Reg<tc::TC_SPEC>;
#[doc = "Offset:0x04 CT16Bn Timer Counter Register"]
pub mod tc;
#[doc = "PRE register accessor: an alias for `Reg<PRE_SPEC>`"]
pub type PRE = crate::Reg<pre::PRE_SPEC>;
#[doc = "Offset:0x08 CT16Bn Prescale Register"]
pub mod pre;
#[doc = "PC register accessor: an alias for `Reg<PC_SPEC>`"]
pub type PC = crate::Reg<pc::PC_SPEC>;
#[doc = "Offset:0x0C CT16Bn Prescale Counter Register"]
pub mod pc;
#[doc = "CNTCTRL register accessor: an alias for `Reg<CNTCTRL_SPEC>`"]
pub type CNTCTRL = crate::Reg<cntctrl::CNTCTRL_SPEC>;
#[doc = "Offset:0x10 CT16Bn Counter Control Register"]
pub mod cntctrl;
#[doc = "MCTRL register accessor: an alias for `Reg<MCTRL_SPEC>`"]
pub type MCTRL = crate::Reg<mctrl::MCTRL_SPEC>;
#[doc = "Offset:0x14 CT16Bn Match Control Register"]
pub mod mctrl;
#[doc = "MR0 register accessor: an alias for `Reg<MR0_SPEC>`"]
pub type MR0 = crate::Reg<mr0::MR0_SPEC>;
#[doc = "Offset:0x20 CT16Bn MR0 Register"]
pub mod mr0;
#[doc = "CAPCTRL register accessor: an alias for `Reg<CAPCTRL_SPEC>`"]
pub type CAPCTRL = crate::Reg<capctrl::CAPCTRL_SPEC>;
#[doc = "Offset:0x80 CT16Bn Capture Control Register"]
pub mod capctrl;
#[doc = "CAP0 register accessor: an alias for `Reg<CAP0_SPEC>`"]
pub type CAP0 = crate::Reg<cap0::CAP0_SPEC>;
#[doc = "Offset:0x84 CT16Bn CAP0 Register"]
pub mod cap0;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Offset:0xA4 CT16Bn Raw Interrupt Status Register"]
pub mod ris;
#[doc = "IC register accessor: an alias for `Reg<IC_SPEC>`"]
pub type IC = crate::Reg<ic::IC_SPEC>;
#[doc = "Offset:0xA8 CT16Bn Interrupt Clear Register"]
pub mod ic;
