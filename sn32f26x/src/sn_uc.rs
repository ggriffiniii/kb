#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Offset:0x00 UC Low 4 Byte Register"]
    pub l4byte: crate::Reg<l4byte::L4BYTE_SPEC>,
    #[doc = "0x04 - Offset:0x04 UC High 4 Byte Register"]
    pub h4byte: crate::Reg<h4byte::H4BYTE_SPEC>,
}
#[doc = "L4BYTE register accessor: an alias for `Reg<L4BYTE_SPEC>`"]
pub type L4BYTE = crate::Reg<l4byte::L4BYTE_SPEC>;
#[doc = "Offset:0x00 UC Low 4 Byte Register"]
pub mod l4byte;
#[doc = "H4BYTE register accessor: an alias for `Reg<H4BYTE_SPEC>`"]
pub type H4BYTE = crate::Reg<h4byte::H4BYTE_SPEC>;
#[doc = "Offset:0x04 UC High 4 Byte Register"]
pub mod h4byte;
