#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Offset:0x00 USB Interrupt Enable Register"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x04 - Offset:0x04 USB Interrupt Event Status Register"]
    pub insts: crate::Reg<insts::INSTS_SPEC>,
    #[doc = "0x08 - Offset:0x08 USB Interrupt Event Status Clear Register"]
    pub instsc: crate::Reg<instsc::INSTSC_SPEC>,
    #[doc = "0x0c - Offset:0x0C USB Device Address Register"]
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    #[doc = "0x10 - Offset:0x10 USB Configuration Register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x14 - Offset:0x14 USB Signal Control Register"]
    pub sgctl: crate::Reg<sgctl::SGCTL_SPEC>,
    #[doc = "0x18 - Offset:0x18 USB Endpoint 0 Control Register"]
    pub ep0ctl: crate::Reg<ep0ctl::EP0CTL_SPEC>,
    #[doc = "0x1c - Offset:0x1C USB Endpoint 1 Control Register"]
    pub ep1ctl: crate::Reg<ep1ctl::EP1CTL_SPEC>,
    #[doc = "0x20 - Offset:0x20 USB Endpoint 2 Control Register"]
    pub ep2ctl: crate::Reg<ep2ctl::EP2CTL_SPEC>,
    #[doc = "0x24 - Offset:0x24 USB Endpoint 3 Control Register"]
    pub ep3ctl: crate::Reg<ep3ctl::EP3CTL_SPEC>,
    #[doc = "0x28 - Offset:0x28 USB Endpoint 4 Control Register"]
    pub ep4ctl: crate::Reg<ep4ctl::EP4CTL_SPEC>,
    _reserved11: [u8; 0x10],
    #[doc = "0x3c - Offset:0x3C USB Endpoint Data Toggle Register"]
    pub eptoggle: crate::Reg<eptoggle::EPTOGGLE_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x48 - Offset:0x48 USB Endpoint 1 Buffer Offset Register"]
    pub ep1bufos: crate::Reg<ep1bufos::EP1BUFOS_SPEC>,
    #[doc = "0x4c - Offset:0x4C USB Endpoint 2 Buffer Offset Register"]
    pub ep2bufos: crate::Reg<ep2bufos::EP2BUFOS_SPEC>,
    #[doc = "0x50 - Offset:0x50 USB Endpoint 3 Buffer Offset Register"]
    pub ep3bufos: crate::Reg<ep3bufos::EP3BUFOS_SPEC>,
    #[doc = "0x54 - Offset:0x54 USB Endpoint 4 Buffer Offset Register"]
    pub ep4bufos: crate::Reg<ep4bufos::EP4BUFOS_SPEC>,
    _reserved16: [u8; 0x08],
    #[doc = "0x60 - Offset:0x60 USB Frame Number Register"]
    pub frmno: crate::Reg<frmno::FRMNO_SPEC>,
    #[doc = "0x64 - Offset:0x64 USB PHY Parameter Register"]
    pub phyprm: crate::Reg<phyprm::PHYPRM_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x6c - Offset:0x6C USB PHY Parameter Register 2"]
    pub phyprm2: crate::Reg<phyprm2::PHYPRM2_SPEC>,
    #[doc = "0x70 - Offset:0x70 PS/2 Control Register"]
    pub ps2ctl: crate::Reg<ps2ctl::PS2CTL_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x78 - Offset:0x78 USB Read/Write Address Register"]
    pub rwaddr: crate::Reg<rwaddr::RWADDR_SPEC>,
    #[doc = "0x7c - Offset:0x7C USB Read/Write Data Register"]
    pub rwdata: crate::Reg<rwdata::RWDATA_SPEC>,
    #[doc = "0x80 - Offset:0x80 USB Read/Write Status Register"]
    pub rwstatus: crate::Reg<rwstatus::RWSTATUS_SPEC>,
    #[doc = "0x84 - Offset:0x84 USB Read/Write Address Register 2"]
    pub rwaddr2: crate::Reg<rwaddr2::RWADDR2_SPEC>,
    #[doc = "0x88 - Offset:0x88 USB Read/Write Data Register 2"]
    pub rwdata2: crate::Reg<rwdata2::RWDATA2_SPEC>,
    #[doc = "0x8c - Offset:0x8C USB Read/Write Status Register 2"]
    pub rwstatus2: crate::Reg<rwstatus2::RWSTATUS2_SPEC>,
}
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Offset:0x00 USB Interrupt Enable Register"]
pub mod inten;
#[doc = "INSTS register accessor: an alias for `Reg<INSTS_SPEC>`"]
pub type INSTS = crate::Reg<insts::INSTS_SPEC>;
#[doc = "Offset:0x04 USB Interrupt Event Status Register"]
pub mod insts;
#[doc = "INSTSC register accessor: an alias for `Reg<INSTSC_SPEC>`"]
pub type INSTSC = crate::Reg<instsc::INSTSC_SPEC>;
#[doc = "Offset:0x08 USB Interrupt Event Status Clear Register"]
pub mod instsc;
#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Offset:0x0C USB Device Address Register"]
pub mod addr;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Offset:0x10 USB Configuration Register"]
pub mod cfg;
#[doc = "SGCTL register accessor: an alias for `Reg<SGCTL_SPEC>`"]
pub type SGCTL = crate::Reg<sgctl::SGCTL_SPEC>;
#[doc = "Offset:0x14 USB Signal Control Register"]
pub mod sgctl;
#[doc = "EP0CTL register accessor: an alias for `Reg<EP0CTL_SPEC>`"]
pub type EP0CTL = crate::Reg<ep0ctl::EP0CTL_SPEC>;
#[doc = "Offset:0x18 USB Endpoint 0 Control Register"]
pub mod ep0ctl;
#[doc = "EP1CTL register accessor: an alias for `Reg<EP1CTL_SPEC>`"]
pub type EP1CTL = crate::Reg<ep1ctl::EP1CTL_SPEC>;
#[doc = "Offset:0x1C USB Endpoint 1 Control Register"]
pub mod ep1ctl;
#[doc = "EP2CTL register accessor: an alias for `Reg<EP2CTL_SPEC>`"]
pub type EP2CTL = crate::Reg<ep2ctl::EP2CTL_SPEC>;
#[doc = "Offset:0x20 USB Endpoint 2 Control Register"]
pub mod ep2ctl;
#[doc = "EP3CTL register accessor: an alias for `Reg<EP3CTL_SPEC>`"]
pub type EP3CTL = crate::Reg<ep3ctl::EP3CTL_SPEC>;
#[doc = "Offset:0x24 USB Endpoint 3 Control Register"]
pub mod ep3ctl;
#[doc = "EP4CTL register accessor: an alias for `Reg<EP4CTL_SPEC>`"]
pub type EP4CTL = crate::Reg<ep4ctl::EP4CTL_SPEC>;
#[doc = "Offset:0x28 USB Endpoint 4 Control Register"]
pub mod ep4ctl;
#[doc = "EPTOGGLE register accessor: an alias for `Reg<EPTOGGLE_SPEC>`"]
pub type EPTOGGLE = crate::Reg<eptoggle::EPTOGGLE_SPEC>;
#[doc = "Offset:0x3C USB Endpoint Data Toggle Register"]
pub mod eptoggle;
#[doc = "EP1BUFOS register accessor: an alias for `Reg<EP1BUFOS_SPEC>`"]
pub type EP1BUFOS = crate::Reg<ep1bufos::EP1BUFOS_SPEC>;
#[doc = "Offset:0x48 USB Endpoint 1 Buffer Offset Register"]
pub mod ep1bufos;
#[doc = "EP2BUFOS register accessor: an alias for `Reg<EP2BUFOS_SPEC>`"]
pub type EP2BUFOS = crate::Reg<ep2bufos::EP2BUFOS_SPEC>;
#[doc = "Offset:0x4C USB Endpoint 2 Buffer Offset Register"]
pub mod ep2bufos;
#[doc = "EP3BUFOS register accessor: an alias for `Reg<EP3BUFOS_SPEC>`"]
pub type EP3BUFOS = crate::Reg<ep3bufos::EP3BUFOS_SPEC>;
#[doc = "Offset:0x50 USB Endpoint 3 Buffer Offset Register"]
pub mod ep3bufos;
#[doc = "EP4BUFOS register accessor: an alias for `Reg<EP4BUFOS_SPEC>`"]
pub type EP4BUFOS = crate::Reg<ep4bufos::EP4BUFOS_SPEC>;
#[doc = "Offset:0x54 USB Endpoint 4 Buffer Offset Register"]
pub mod ep4bufos;
#[doc = "FRMNO register accessor: an alias for `Reg<FRMNO_SPEC>`"]
pub type FRMNO = crate::Reg<frmno::FRMNO_SPEC>;
#[doc = "Offset:0x60 USB Frame Number Register"]
pub mod frmno;
#[doc = "PHYPRM register accessor: an alias for `Reg<PHYPRM_SPEC>`"]
pub type PHYPRM = crate::Reg<phyprm::PHYPRM_SPEC>;
#[doc = "Offset:0x64 USB PHY Parameter Register"]
pub mod phyprm;
#[doc = "PHYPRM2 register accessor: an alias for `Reg<PHYPRM2_SPEC>`"]
pub type PHYPRM2 = crate::Reg<phyprm2::PHYPRM2_SPEC>;
#[doc = "Offset:0x6C USB PHY Parameter Register 2"]
pub mod phyprm2;
#[doc = "PS2CTL register accessor: an alias for `Reg<PS2CTL_SPEC>`"]
pub type PS2CTL = crate::Reg<ps2ctl::PS2CTL_SPEC>;
#[doc = "Offset:0x70 PS/2 Control Register"]
pub mod ps2ctl;
#[doc = "RWADDR register accessor: an alias for `Reg<RWADDR_SPEC>`"]
pub type RWADDR = crate::Reg<rwaddr::RWADDR_SPEC>;
#[doc = "Offset:0x78 USB Read/Write Address Register"]
pub mod rwaddr;
#[doc = "RWDATA register accessor: an alias for `Reg<RWDATA_SPEC>`"]
pub type RWDATA = crate::Reg<rwdata::RWDATA_SPEC>;
#[doc = "Offset:0x7C USB Read/Write Data Register"]
pub mod rwdata;
#[doc = "RWSTATUS register accessor: an alias for `Reg<RWSTATUS_SPEC>`"]
pub type RWSTATUS = crate::Reg<rwstatus::RWSTATUS_SPEC>;
#[doc = "Offset:0x80 USB Read/Write Status Register"]
pub mod rwstatus;
#[doc = "RWADDR2 register accessor: an alias for `Reg<RWADDR2_SPEC>`"]
pub type RWADDR2 = crate::Reg<rwaddr2::RWADDR2_SPEC>;
#[doc = "Offset:0x84 USB Read/Write Address Register 2"]
pub mod rwaddr2;
#[doc = "RWDATA2 register accessor: an alias for `Reg<RWDATA2_SPEC>`"]
pub type RWDATA2 = crate::Reg<rwdata2::RWDATA2_SPEC>;
#[doc = "Offset:0x88 USB Read/Write Data Register 2"]
pub mod rwdata2;
#[doc = "RWSTATUS2 register accessor: an alias for `Reg<RWSTATUS2_SPEC>`"]
pub type RWSTATUS2 = crate::Reg<rwstatus2::RWSTATUS2_SPEC>;
#[doc = "Offset:0x8C USB Read/Write Status Register 2"]
pub mod rwstatus2;
