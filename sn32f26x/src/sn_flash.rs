#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Offset:0x00 Flash Low Power Control Register"]
    pub lpctrl: crate::Reg<lpctrl::LPCTRL_SPEC>,
    #[doc = "0x04 - Offset:0x04 Flash Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - Offset:0x08 Flash Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x0c - Offset:0x0C Flash Data Register"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x10 - Offset:0x10 Flash Address Register"]
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    #[doc = "0x14 - Offset:0x14 Flash Checksum Register"]
    pub chksum: crate::Reg<chksum::CHKSUM_SPEC>,
}
#[doc = "LPCTRL register accessor: an alias for `Reg<LPCTRL_SPEC>`"]
pub type LPCTRL = crate::Reg<lpctrl::LPCTRL_SPEC>;
#[doc = "Offset:0x00 Flash Low Power Control Register"]
pub mod lpctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Offset:0x04 Flash Status Register"]
pub mod status;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Offset:0x08 Flash Control Register"]
pub mod ctrl;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Offset:0x0C Flash Data Register"]
pub mod data;
#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Offset:0x10 Flash Address Register"]
pub mod addr;
#[doc = "CHKSUM register accessor: an alias for `Reg<CHKSUM_SPEC>`"]
pub type CHKSUM = crate::Reg<chksum::CHKSUM_SPEC>;
#[doc = "Offset:0x14 Flash Checksum Register"]
pub mod chksum;
