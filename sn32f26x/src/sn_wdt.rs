#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Offset:0x00 WDT Configuration Register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x04 - Offset:0x04 WDT Clock Source Register"]
    pub clksource: crate::Reg<clksource::CLKSOURCE_SPEC>,
    #[doc = "0x08 - Offset:0x08 WDT Timer Constant Register"]
    pub tc: crate::Reg<tc::TC_SPEC>,
    #[doc = "0x0c - Offset:0x0C WDT Feed Register"]
    pub feed: crate::Reg<feed::FEED_SPEC>,
}
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Offset:0x00 WDT Configuration Register"]
pub mod cfg;
#[doc = "CLKSOURCE register accessor: an alias for `Reg<CLKSOURCE_SPEC>`"]
pub type CLKSOURCE = crate::Reg<clksource::CLKSOURCE_SPEC>;
#[doc = "Offset:0x04 WDT Clock Source Register"]
pub mod clksource;
#[doc = "TC register accessor: an alias for `Reg<TC_SPEC>`"]
pub type TC = crate::Reg<tc::TC_SPEC>;
#[doc = "Offset:0x08 WDT Timer Constant Register"]
pub mod tc;
#[doc = "FEED register accessor: an alias for `Reg<FEED_SPEC>`"]
pub type FEED = crate::Reg<feed::FEED_SPEC>;
#[doc = "Offset:0x0C WDT Feed Register"]
pub mod feed;
