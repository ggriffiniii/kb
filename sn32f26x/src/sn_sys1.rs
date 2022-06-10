#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Offset:0x00 AHB Clock Enable Register"]
    pub ahbclken: crate::Reg<ahbclken::AHBCLKEN_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Offset:0x08 APB Clock Prescale Register 1"]
    pub apbcp1: crate::Reg<apbcp1::APBCP1_SPEC>,
}
#[doc = "AHBCLKEN register accessor: an alias for `Reg<AHBCLKEN_SPEC>`"]
pub type AHBCLKEN = crate::Reg<ahbclken::AHBCLKEN_SPEC>;
#[doc = "Offset:0x00 AHB Clock Enable Register"]
pub mod ahbclken;
#[doc = "APBCP1 register accessor: an alias for `Reg<APBCP1_SPEC>`"]
pub type APBCP1 = crate::Reg<apbcp1::APBCP1_SPEC>;
#[doc = "Offset:0x08 APB Clock Prescale Register 1"]
pub mod apbcp1;
