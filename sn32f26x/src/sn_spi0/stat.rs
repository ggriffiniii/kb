#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "TX FIFO empty flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_EMPTY_A {
    #[doc = "0: TX FIFO is not empty"]
    _0 = 0,
    #[doc = "1: TX FIFO is empty"]
    _1 = 1,
}
impl From<TX_EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: TX_EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_EMPTY` reader - TX FIFO empty flag"]
pub type TX_EMPTY_R = crate::BitReader<TX_EMPTY_A>;
impl TX_EMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_EMPTY_A {
        match self.bits {
            false => TX_EMPTY_A::_0,
            true => TX_EMPTY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TX_EMPTY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TX_EMPTY_A::_1
    }
}
#[doc = "TX FIFO full flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_FULL_A {
    #[doc = "0: TX FIFO is not full"]
    _0 = 0,
    #[doc = "1: TX FIFO is full"]
    _1 = 1,
}
impl From<TX_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: TX_FULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_FULL` reader - TX FIFO full flag"]
pub type TX_FULL_R = crate::BitReader<TX_FULL_A>;
impl TX_FULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_FULL_A {
        match self.bits {
            false => TX_FULL_A::_0,
            true => TX_FULL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TX_FULL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TX_FULL_A::_1
    }
}
#[doc = "RX FIFO empty flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_EMPTY_A {
    #[doc = "0: RX FIFO is not empty"]
    _0 = 0,
    #[doc = "1: RX FIFO is empty"]
    _1 = 1,
}
impl From<RX_EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: RX_EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_EMPTY` reader - RX FIFO empty flag"]
pub type RX_EMPTY_R = crate::BitReader<RX_EMPTY_A>;
impl RX_EMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_EMPTY_A {
        match self.bits {
            false => RX_EMPTY_A::_0,
            true => RX_EMPTY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX_EMPTY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX_EMPTY_A::_1
    }
}
#[doc = "RX FIFO full flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FULL_A {
    #[doc = "0: RX FIFO is not full"]
    _0 = 0,
    #[doc = "1: RX FIFO is full"]
    _1 = 1,
}
impl From<RX_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_FULL` reader - RX FIFO full flag"]
pub type RX_FULL_R = crate::BitReader<RX_FULL_A>;
impl RX_FULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FULL_A {
        match self.bits {
            false => RX_FULL_A::_0,
            true => RX_FULL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX_FULL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX_FULL_A::_1
    }
}
#[doc = "Busy flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: SPIn is idle"]
    IDLE = 0,
    #[doc = "1: SPIn is transfering"]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Busy flag"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::IDLE,
            true => BUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
}
#[doc = "TX FIFO threshold flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIFOTHF_A {
    #[doc = "0: Data count in TX FIFO is larger than TXFIFOTH"]
    _0 = 0,
    #[doc = "1: Data count in TX FIFO is less equal than TXFIFOTH"]
    _1 = 1,
}
impl From<TXFIFOTHF_A> for bool {
    #[inline(always)]
    fn from(variant: TXFIFOTHF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIFOTHF` reader - TX FIFO threshold flag"]
pub type TXFIFOTHF_R = crate::BitReader<TXFIFOTHF_A>;
impl TXFIFOTHF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFIFOTHF_A {
        match self.bits {
            false => TXFIFOTHF_A::_0,
            true => TXFIFOTHF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXFIFOTHF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXFIFOTHF_A::_1
    }
}
#[doc = "RX FIFO threshold flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFOTHF_A {
    #[doc = "0: Data count in RX FIFO is less equal than RXFIFOTH"]
    _0 = 0,
    #[doc = "1: Data count in RX FIFO is larger than RXFIFOTH"]
    _1 = 1,
}
impl From<RXFIFOTHF_A> for bool {
    #[inline(always)]
    fn from(variant: RXFIFOTHF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFOTHF` reader - RX FIFO threshold flag"]
pub type RXFIFOTHF_R = crate::BitReader<RXFIFOTHF_A>;
impl RXFIFOTHF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFIFOTHF_A {
        match self.bits {
            false => RXFIFOTHF_A::_0,
            true => RXFIFOTHF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXFIFOTHF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXFIFOTHF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - TX FIFO empty flag"]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO full flag"]
    #[inline(always)]
    pub fn tx_full(&self) -> TX_FULL_R {
        TX_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO empty flag"]
    #[inline(always)]
    pub fn rx_empty(&self) -> RX_EMPTY_R {
        RX_EMPTY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO full flag"]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Busy flag"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX FIFO threshold flag"]
    #[inline(always)]
    pub fn txfifothf(&self) -> TXFIFOTHF_R {
        TXFIFOTHF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX FIFO threshold flag"]
    #[inline(always)]
    pub fn rxfifothf(&self) -> RXFIFOTHF_R {
        RXFIFOTHF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Offset:0x0C SPIn Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT to value 0x25"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x25
    }
}
