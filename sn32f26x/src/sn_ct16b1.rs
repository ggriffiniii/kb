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
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - Offset:0x14 CT16Bn Match Control Register"]
    pub mctrl: crate::Reg<mctrl::MCTRL_SPEC>,
    #[doc = "0x18 - Offset:0x18 CT16Bn Match Control Register"]
    pub mctrl2: crate::Reg<mctrl2::MCTRL2_SPEC>,
    #[doc = "0x1c - Offset:0x1C CT16Bn Match Control Register"]
    pub mctrl3: crate::Reg<mctrl3::MCTRL3_SPEC>,
    #[doc = "0x20 - Offset:0x20 CT16Bn MR0 Register"]
    pub mr0: crate::Reg<mr0::MR0_SPEC>,
    #[doc = "0x24 - Offset:0x24 CT16Bn MR1 Register"]
    pub mr1: crate::Reg<mr1::MR1_SPEC>,
    #[doc = "0x28 - Offset:0x28 CT16Bn MR2 Register"]
    pub mr2: crate::Reg<mr2::MR2_SPEC>,
    #[doc = "0x2c - Offset:0x2C CT16Bn MR3 Register"]
    pub mr3: crate::Reg<mr3::MR3_SPEC>,
    #[doc = "0x30 - Offset:0x30 CT16Bn MR4 Register"]
    pub mr4: crate::Reg<mr4::MR4_SPEC>,
    #[doc = "0x34 - Offset:0x34 CT16Bn MR5 Register"]
    pub mr5: crate::Reg<mr5::MR5_SPEC>,
    #[doc = "0x38 - Offset:0x38 CT16Bn MR6 Register"]
    pub mr6: crate::Reg<mr6::MR6_SPEC>,
    #[doc = "0x3c - Offset:0x3C CT16Bn MR7 Register"]
    pub mr7: crate::Reg<mr7::MR7_SPEC>,
    #[doc = "0x40 - Offset:0x40 CT16Bn MR8 Register"]
    pub mr8: crate::Reg<mr8::MR8_SPEC>,
    #[doc = "0x44 - Offset:0x44 CT16Bn MR9 Register"]
    pub mr9: crate::Reg<mr9::MR9_SPEC>,
    #[doc = "0x48 - Offset:0x48 CT16Bn MR10 Register"]
    pub mr10: crate::Reg<mr10::MR10_SPEC>,
    #[doc = "0x4c - Offset:0x4C CT16Bn MR11 Register"]
    pub mr11: crate::Reg<mr11::MR11_SPEC>,
    #[doc = "0x50 - Offset:0x50 CT16Bn MR12 Register"]
    pub mr12: crate::Reg<mr12::MR12_SPEC>,
    #[doc = "0x54 - Offset:0x54 CT16Bn MR13 Register"]
    pub mr13: crate::Reg<mr13::MR13_SPEC>,
    #[doc = "0x58 - Offset:0x58 CT16Bn MR14 Register"]
    pub mr14: crate::Reg<mr14::MR14_SPEC>,
    #[doc = "0x5c - Offset:0x5C CT16Bn MR15 Register"]
    pub mr15: crate::Reg<mr15::MR15_SPEC>,
    #[doc = "0x60 - Offset:0x60 CT16Bn MR16 Register"]
    pub mr16: crate::Reg<mr16::MR16_SPEC>,
    #[doc = "0x64 - Offset:0x64 CT16Bn MR17 Register"]
    pub mr17: crate::Reg<mr17::MR17_SPEC>,
    #[doc = "0x68 - Offset:0x68 CT16Bn MR18 Register"]
    pub mr18: crate::Reg<mr18::MR18_SPEC>,
    #[doc = "0x6c - Offset:0x6C CT16Bn MR19 Register"]
    pub mr19: crate::Reg<mr19::MR19_SPEC>,
    #[doc = "0x70 - Offset:0x70 CT16Bn MR20 Register"]
    pub mr20: crate::Reg<mr20::MR20_SPEC>,
    #[doc = "0x74 - Offset:0x74 CT16Bn MR21 Register"]
    pub mr21: crate::Reg<mr21::MR21_SPEC>,
    #[doc = "0x78 - Offset:0x78 CT16Bn MR22 Register"]
    pub mr22: crate::Reg<mr22::MR22_SPEC>,
    #[doc = "0x7c - Offset:0x7C CT16Bn MR23 Register"]
    pub mr23: crate::Reg<mr23::MR23_SPEC>,
    _reserved31: [u8; 0x08],
    #[doc = "0x88 - Offset:0x88 CT16Bn External Match Register"]
    pub em: crate::Reg<em::EM_SPEC>,
    #[doc = "0x8c - Offset:0x8C CT16Bn External Match Control register"]
    pub emc: crate::Reg<emc::EMC_SPEC>,
    #[doc = "0x90 - Offset:0x90 CT16Bn External Match Control register 2"]
    pub emc2: crate::Reg<emc2::EMC2_SPEC>,
    #[doc = "0x94 - Offset:0x94 CT16Bn PWM Control Register"]
    pub pwmctrl: crate::Reg<pwmctrl::PWMCTRL_SPEC>,
    #[doc = "0x98 - Offset:0x98 CT16Bn PWM Control Register 2"]
    pub pwmctrl2: crate::Reg<pwmctrl2::PWMCTRL2_SPEC>,
    #[doc = "0x9c - Offset:0x9C CT16Bn PWM Enable register"]
    pub pwmenb: crate::Reg<pwmenb::PWMENB_SPEC>,
    #[doc = "0xa0 - Offset:0xA0 CT16Bn PWM IO Enable register"]
    pub pwmioenb: crate::Reg<pwmioenb::PWMIOENB_SPEC>,
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
#[doc = "MCTRL register accessor: an alias for `Reg<MCTRL_SPEC>`"]
pub type MCTRL = crate::Reg<mctrl::MCTRL_SPEC>;
#[doc = "Offset:0x14 CT16Bn Match Control Register"]
pub mod mctrl;
#[doc = "MCTRL2 register accessor: an alias for `Reg<MCTRL2_SPEC>`"]
pub type MCTRL2 = crate::Reg<mctrl2::MCTRL2_SPEC>;
#[doc = "Offset:0x18 CT16Bn Match Control Register"]
pub mod mctrl2;
#[doc = "MCTRL3 register accessor: an alias for `Reg<MCTRL3_SPEC>`"]
pub type MCTRL3 = crate::Reg<mctrl3::MCTRL3_SPEC>;
#[doc = "Offset:0x1C CT16Bn Match Control Register"]
pub mod mctrl3;
#[doc = "MR0 register accessor: an alias for `Reg<MR0_SPEC>`"]
pub type MR0 = crate::Reg<mr0::MR0_SPEC>;
#[doc = "Offset:0x20 CT16Bn MR0 Register"]
pub mod mr0;
#[doc = "MR1 register accessor: an alias for `Reg<MR1_SPEC>`"]
pub type MR1 = crate::Reg<mr1::MR1_SPEC>;
#[doc = "Offset:0x24 CT16Bn MR1 Register"]
pub mod mr1;
#[doc = "MR2 register accessor: an alias for `Reg<MR2_SPEC>`"]
pub type MR2 = crate::Reg<mr2::MR2_SPEC>;
#[doc = "Offset:0x28 CT16Bn MR2 Register"]
pub mod mr2;
#[doc = "MR3 register accessor: an alias for `Reg<MR3_SPEC>`"]
pub type MR3 = crate::Reg<mr3::MR3_SPEC>;
#[doc = "Offset:0x2C CT16Bn MR3 Register"]
pub mod mr3;
#[doc = "MR4 register accessor: an alias for `Reg<MR4_SPEC>`"]
pub type MR4 = crate::Reg<mr4::MR4_SPEC>;
#[doc = "Offset:0x30 CT16Bn MR4 Register"]
pub mod mr4;
#[doc = "MR5 register accessor: an alias for `Reg<MR5_SPEC>`"]
pub type MR5 = crate::Reg<mr5::MR5_SPEC>;
#[doc = "Offset:0x34 CT16Bn MR5 Register"]
pub mod mr5;
#[doc = "MR6 register accessor: an alias for `Reg<MR6_SPEC>`"]
pub type MR6 = crate::Reg<mr6::MR6_SPEC>;
#[doc = "Offset:0x38 CT16Bn MR6 Register"]
pub mod mr6;
#[doc = "MR7 register accessor: an alias for `Reg<MR7_SPEC>`"]
pub type MR7 = crate::Reg<mr7::MR7_SPEC>;
#[doc = "Offset:0x3C CT16Bn MR7 Register"]
pub mod mr7;
#[doc = "MR8 register accessor: an alias for `Reg<MR8_SPEC>`"]
pub type MR8 = crate::Reg<mr8::MR8_SPEC>;
#[doc = "Offset:0x40 CT16Bn MR8 Register"]
pub mod mr8;
#[doc = "MR9 register accessor: an alias for `Reg<MR9_SPEC>`"]
pub type MR9 = crate::Reg<mr9::MR9_SPEC>;
#[doc = "Offset:0x44 CT16Bn MR9 Register"]
pub mod mr9;
#[doc = "MR10 register accessor: an alias for `Reg<MR10_SPEC>`"]
pub type MR10 = crate::Reg<mr10::MR10_SPEC>;
#[doc = "Offset:0x48 CT16Bn MR10 Register"]
pub mod mr10;
#[doc = "MR11 register accessor: an alias for `Reg<MR11_SPEC>`"]
pub type MR11 = crate::Reg<mr11::MR11_SPEC>;
#[doc = "Offset:0x4C CT16Bn MR11 Register"]
pub mod mr11;
#[doc = "MR12 register accessor: an alias for `Reg<MR12_SPEC>`"]
pub type MR12 = crate::Reg<mr12::MR12_SPEC>;
#[doc = "Offset:0x50 CT16Bn MR12 Register"]
pub mod mr12;
#[doc = "MR13 register accessor: an alias for `Reg<MR13_SPEC>`"]
pub type MR13 = crate::Reg<mr13::MR13_SPEC>;
#[doc = "Offset:0x54 CT16Bn MR13 Register"]
pub mod mr13;
#[doc = "MR14 register accessor: an alias for `Reg<MR14_SPEC>`"]
pub type MR14 = crate::Reg<mr14::MR14_SPEC>;
#[doc = "Offset:0x58 CT16Bn MR14 Register"]
pub mod mr14;
#[doc = "MR15 register accessor: an alias for `Reg<MR15_SPEC>`"]
pub type MR15 = crate::Reg<mr15::MR15_SPEC>;
#[doc = "Offset:0x5C CT16Bn MR15 Register"]
pub mod mr15;
#[doc = "MR16 register accessor: an alias for `Reg<MR16_SPEC>`"]
pub type MR16 = crate::Reg<mr16::MR16_SPEC>;
#[doc = "Offset:0x60 CT16Bn MR16 Register"]
pub mod mr16;
#[doc = "MR17 register accessor: an alias for `Reg<MR17_SPEC>`"]
pub type MR17 = crate::Reg<mr17::MR17_SPEC>;
#[doc = "Offset:0x64 CT16Bn MR17 Register"]
pub mod mr17;
#[doc = "MR18 register accessor: an alias for `Reg<MR18_SPEC>`"]
pub type MR18 = crate::Reg<mr18::MR18_SPEC>;
#[doc = "Offset:0x68 CT16Bn MR18 Register"]
pub mod mr18;
#[doc = "MR19 register accessor: an alias for `Reg<MR19_SPEC>`"]
pub type MR19 = crate::Reg<mr19::MR19_SPEC>;
#[doc = "Offset:0x6C CT16Bn MR19 Register"]
pub mod mr19;
#[doc = "MR20 register accessor: an alias for `Reg<MR20_SPEC>`"]
pub type MR20 = crate::Reg<mr20::MR20_SPEC>;
#[doc = "Offset:0x70 CT16Bn MR20 Register"]
pub mod mr20;
#[doc = "MR21 register accessor: an alias for `Reg<MR21_SPEC>`"]
pub type MR21 = crate::Reg<mr21::MR21_SPEC>;
#[doc = "Offset:0x74 CT16Bn MR21 Register"]
pub mod mr21;
#[doc = "MR22 register accessor: an alias for `Reg<MR22_SPEC>`"]
pub type MR22 = crate::Reg<mr22::MR22_SPEC>;
#[doc = "Offset:0x78 CT16Bn MR22 Register"]
pub mod mr22;
#[doc = "MR23 register accessor: an alias for `Reg<MR23_SPEC>`"]
pub type MR23 = crate::Reg<mr23::MR23_SPEC>;
#[doc = "Offset:0x7C CT16Bn MR23 Register"]
pub mod mr23;
#[doc = "EM register accessor: an alias for `Reg<EM_SPEC>`"]
pub type EM = crate::Reg<em::EM_SPEC>;
#[doc = "Offset:0x88 CT16Bn External Match Register"]
pub mod em;
#[doc = "EMC register accessor: an alias for `Reg<EMC_SPEC>`"]
pub type EMC = crate::Reg<emc::EMC_SPEC>;
#[doc = "Offset:0x8C CT16Bn External Match Control register"]
pub mod emc;
#[doc = "EMC2 register accessor: an alias for `Reg<EMC2_SPEC>`"]
pub type EMC2 = crate::Reg<emc2::EMC2_SPEC>;
#[doc = "Offset:0x90 CT16Bn External Match Control register 2"]
pub mod emc2;
#[doc = "PWMCTRL register accessor: an alias for `Reg<PWMCTRL_SPEC>`"]
pub type PWMCTRL = crate::Reg<pwmctrl::PWMCTRL_SPEC>;
#[doc = "Offset:0x94 CT16Bn PWM Control Register"]
pub mod pwmctrl;
#[doc = "PWMCTRL2 register accessor: an alias for `Reg<PWMCTRL2_SPEC>`"]
pub type PWMCTRL2 = crate::Reg<pwmctrl2::PWMCTRL2_SPEC>;
#[doc = "Offset:0x98 CT16Bn PWM Control Register 2"]
pub mod pwmctrl2;
#[doc = "PWMENB register accessor: an alias for `Reg<PWMENB_SPEC>`"]
pub type PWMENB = crate::Reg<pwmenb::PWMENB_SPEC>;
#[doc = "Offset:0x9C CT16Bn PWM Enable register"]
pub mod pwmenb;
#[doc = "PWMIOENB register accessor: an alias for `Reg<PWMIOENB_SPEC>`"]
pub type PWMIOENB = crate::Reg<pwmioenb::PWMIOENB_SPEC>;
#[doc = "Offset:0xA0 CT16Bn PWM IO Enable register"]
pub mod pwmioenb;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Offset:0xA4 CT16Bn Raw Interrupt Status Register"]
pub mod ris;
#[doc = "IC register accessor: an alias for `Reg<IC_SPEC>`"]
pub type IC = crate::Reg<ic::IC_SPEC>;
#[doc = "Offset:0xA8 CT16Bn Interrupt Clear Register"]
pub mod ic;
