#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    #[doc = "0x40 - Offset:0x40 PMU Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Offset:0x40 PMU Control Register"]
pub mod ctrl;
