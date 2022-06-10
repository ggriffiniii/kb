#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Offset:0x00 I2Cn Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Offset:0x04 I2Cn Status Register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x08 - Offset:0x08 I2Cn TX Data Register"]
    pub txdata: crate::Reg<txdata::TXDATA_SPEC>,
    #[doc = "0x0c - Offset:0x0C I2Cn RX Data Register"]
    pub rxdata: crate::Reg<rxdata::RXDATA_SPEC>,
    #[doc = "0x10 - Offset:0x10 I2Cn Slave Address 0 Register"]
    pub slvaddr0: crate::Reg<slvaddr0::SLVADDR0_SPEC>,
    #[doc = "0x14 - Offset:0x14 I2Cn Slave Address 1 Register"]
    pub slvaddr1: crate::Reg<slvaddr1::SLVADDR1_SPEC>,
    #[doc = "0x18 - Offset:0x18 I2Cn Slave Address 2 Register"]
    pub slvaddr2: crate::Reg<slvaddr2::SLVADDR2_SPEC>,
    #[doc = "0x1c - Offset:0x1C I2Cn Slave Address 3 Register"]
    pub slvaddr3: crate::Reg<slvaddr3::SLVADDR3_SPEC>,
    #[doc = "0x20 - Offset:0x20 I2Cn SCL High Time Register"]
    pub sclht: crate::Reg<sclht::SCLHT_SPEC>,
    #[doc = "0x24 - Offset:0x24 I2Cn SCL Low Time Register"]
    pub scllt: crate::Reg<scllt::SCLLT_SPEC>,
    #[doc = "0x28 - Offset:0x28 I2C SCL Check Time register"]
    pub sclct: crate::Reg<sclct::SCLCT_SPEC>,
    #[doc = "0x2c - Offset:0x2C I2Cn Timeout Control Register"]
    pub toctrl: crate::Reg<toctrl::TOCTRL_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Offset:0x00 I2Cn Control Register"]
pub mod ctrl;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Offset:0x04 I2Cn Status Register"]
pub mod stat;
#[doc = "TXDATA register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "Offset:0x08 I2Cn TX Data Register"]
pub mod txdata;
#[doc = "RXDATA register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "Offset:0x0C I2Cn RX Data Register"]
pub mod rxdata;
#[doc = "SLVADDR0 register accessor: an alias for `Reg<SLVADDR0_SPEC>`"]
pub type SLVADDR0 = crate::Reg<slvaddr0::SLVADDR0_SPEC>;
#[doc = "Offset:0x10 I2Cn Slave Address 0 Register"]
pub mod slvaddr0;
#[doc = "SLVADDR1 register accessor: an alias for `Reg<SLVADDR1_SPEC>`"]
pub type SLVADDR1 = crate::Reg<slvaddr1::SLVADDR1_SPEC>;
#[doc = "Offset:0x14 I2Cn Slave Address 1 Register"]
pub mod slvaddr1;
#[doc = "SLVADDR2 register accessor: an alias for `Reg<SLVADDR2_SPEC>`"]
pub type SLVADDR2 = crate::Reg<slvaddr2::SLVADDR2_SPEC>;
#[doc = "Offset:0x18 I2Cn Slave Address 2 Register"]
pub mod slvaddr2;
#[doc = "SLVADDR3 register accessor: an alias for `Reg<SLVADDR3_SPEC>`"]
pub type SLVADDR3 = crate::Reg<slvaddr3::SLVADDR3_SPEC>;
#[doc = "Offset:0x1C I2Cn Slave Address 3 Register"]
pub mod slvaddr3;
#[doc = "SCLHT register accessor: an alias for `Reg<SCLHT_SPEC>`"]
pub type SCLHT = crate::Reg<sclht::SCLHT_SPEC>;
#[doc = "Offset:0x20 I2Cn SCL High Time Register"]
pub mod sclht;
#[doc = "SCLLT register accessor: an alias for `Reg<SCLLT_SPEC>`"]
pub type SCLLT = crate::Reg<scllt::SCLLT_SPEC>;
#[doc = "Offset:0x24 I2Cn SCL Low Time Register"]
pub mod scllt;
#[doc = "SCLCT register accessor: an alias for `Reg<SCLCT_SPEC>`"]
pub type SCLCT = crate::Reg<sclct::SCLCT_SPEC>;
#[doc = "Offset:0x28 I2C SCL Check Time register"]
pub mod sclct;
#[doc = "TOCTRL register accessor: an alias for `Reg<TOCTRL_SPEC>`"]
pub type TOCTRL = crate::Reg<toctrl::TOCTRL_SPEC>;
#[doc = "Offset:0x2C I2Cn Timeout Control Register"]
pub mod toctrl;
