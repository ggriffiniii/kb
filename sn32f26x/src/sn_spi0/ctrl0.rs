#[doc = "Register `CTRL0` reader"]
pub struct R(crate::R<CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL0` writer"]
pub struct W(crate::W<CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SSP enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSPEN_A {
    #[doc = "0: Disable SPI"]
    DISABLE = 0,
    #[doc = "1: Enable SPI"]
    ENABLE = 1,
}
impl From<SSPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SSPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSPEN` reader - SSP enable bit"]
pub type SSPEN_R = crate::BitReader<SSPEN_A>;
impl SSPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSPEN_A {
        match self.bits {
            false => SSPEN_A::DISABLE,
            true => SSPEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SSPEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SSPEN_A::ENABLE
    }
}
#[doc = "Field `SSPEN` writer - SSP enable bit"]
pub type SSPEN_W<'a> = crate::BitWriter<'a, u32, CTRL0_SPEC, SSPEN_A, 0>;
impl<'a> SSPEN_W<'a> {
    #[doc = "Disable SPI"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SSPEN_A::DISABLE)
    }
    #[doc = "Enable SPI"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SSPEN_A::ENABLE)
    }
}
#[doc = "Loopback mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPBACK_A {
    #[doc = "0: Disable loopback mode"]
    DISABLE = 0,
    #[doc = "1: Enable loopback mode"]
    ENABLE = 1,
}
impl From<LOOPBACK_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPBACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOPBACK` reader - Loopback mode enable"]
pub type LOOPBACK_R = crate::BitReader<LOOPBACK_A>;
impl LOOPBACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPBACK_A {
        match self.bits {
            false => LOOPBACK_A::DISABLE,
            true => LOOPBACK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LOOPBACK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LOOPBACK_A::ENABLE
    }
}
#[doc = "Field `LOOPBACK` writer - Loopback mode enable"]
pub type LOOPBACK_W<'a> = crate::BitWriter<'a, u32, CTRL0_SPEC, LOOPBACK_A, 1>;
impl<'a> LOOPBACK_W<'a> {
    #[doc = "Disable loopback mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LOOPBACK_A::DISABLE)
    }
    #[doc = "Enable loopback mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LOOPBACK_A::ENABLE)
    }
}
#[doc = "Slave data out disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDODIS_A {
    #[doc = "0: Enable slave data out"]
    ENABLE = 0,
    #[doc = "1: Diable slave data out (MISO=0)"]
    DISBLE = 1,
}
impl From<SDODIS_A> for bool {
    #[inline(always)]
    fn from(variant: SDODIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDODIS` reader - Slave data out disable"]
pub type SDODIS_R = crate::BitReader<SDODIS_A>;
impl SDODIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDODIS_A {
        match self.bits {
            false => SDODIS_A::ENABLE,
            true => SDODIS_A::DISBLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SDODIS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISBLE`"]
    #[inline(always)]
    pub fn is_disble(&self) -> bool {
        *self == SDODIS_A::DISBLE
    }
}
#[doc = "Field `SDODIS` writer - Slave data out disable"]
pub type SDODIS_W<'a> = crate::BitWriter<'a, u32, CTRL0_SPEC, SDODIS_A, 2>;
impl<'a> SDODIS_W<'a> {
    #[doc = "Enable slave data out"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDODIS_A::ENABLE)
    }
    #[doc = "Diable slave data out (MISO=0)"]
    #[inline(always)]
    pub fn disble(self) -> &'a mut W {
        self.variant(SDODIS_A::DISBLE)
    }
}
#[doc = "Master/Slave selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MS_A {
    #[doc = "0: Act as Master"]
    MASTER = 0,
    #[doc = "1: Act as Slave"]
    SLAVE = 1,
}
impl From<MS_A> for bool {
    #[inline(always)]
    fn from(variant: MS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MS` reader - Master/Slave selection"]
pub type MS_R = crate::BitReader<MS_A>;
impl MS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MS_A {
        match self.bits {
            false => MS_A::MASTER,
            true => MS_A::SLAVE,
        }
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MS_A::MASTER
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MS_A::SLAVE
    }
}
#[doc = "Field `MS` writer - Master/Slave selection"]
pub type MS_W<'a> = crate::BitWriter<'a, u32, CTRL0_SPEC, MS_A, 3>;
impl<'a> MS_W<'a> {
    #[doc = "Act as Master"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MS_A::MASTER)
    }
    #[doc = "Act as Slave"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MS_A::SLAVE)
    }
}
#[doc = "Interface format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORMAT_A {
    #[doc = "0: Act as SPI"]
    SPI = 0,
}
impl From<FORMAT_A> for bool {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORMAT` reader - Interface format"]
pub type FORMAT_R = crate::BitReader<FORMAT_A>;
impl FORMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FORMAT_A> {
        match self.bits {
            false => Some(FORMAT_A::SPI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == FORMAT_A::SPI
    }
}
#[doc = "Field `FORMAT` writer - Interface format"]
pub type FORMAT_W<'a> = crate::BitWriter<'a, u32, CTRL0_SPEC, FORMAT_A, 4>;
impl<'a> FORMAT_W<'a> {
    #[doc = "Act as SPI"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(FORMAT_A::SPI)
    }
}
#[doc = "SPI FSM and FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FRESET_AW {
    #[doc = "0: No effect"]
    _00 = 0,
    #[doc = "3: Reset FSM and FIFO"]
    _11 = 3,
}
impl From<FRESET_AW> for u8 {
    #[inline(always)]
    fn from(variant: FRESET_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `FRESET` writer - SPI FSM and FIFO Reset"]
pub type FRESET_W<'a> = crate::FieldWriter<'a, u32, CTRL0_SPEC, u8, FRESET_AW, 2, 6>;
impl<'a> FRESET_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FRESET_AW::_00)
    }
    #[doc = "Reset FSM and FIFO"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FRESET_AW::_11)
    }
}
#[doc = "Data length = DL\\[3:0\\]+1\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DL_A {
    #[doc = "2: Data length=3"]
    _0010 = 2,
    #[doc = "3: Data length=4"]
    _0011 = 3,
    #[doc = "4: Data length=5"]
    _0100 = 4,
    #[doc = "5: Data length=6"]
    _0101 = 5,
    #[doc = "6: Data length=7"]
    _0110 = 6,
    #[doc = "7: Data length=8"]
    _0111 = 7,
    #[doc = "8: Data length=9"]
    _1000 = 8,
    #[doc = "9: Data length=10"]
    _1001 = 9,
    #[doc = "10: Data length=11"]
    _1010 = 10,
    #[doc = "11: Data length=12"]
    _1011 = 11,
    #[doc = "12: Data length=13"]
    _1100 = 12,
    #[doc = "13: Data length=14"]
    _1101 = 13,
    #[doc = "14: Data length=15"]
    _1110 = 14,
    #[doc = "15: Data length=16"]
    _1111 = 15,
}
impl From<DL_A> for u8 {
    #[inline(always)]
    fn from(variant: DL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DL` reader - Data length = DL\\[3:0\\]+1"]
pub type DL_R = crate::FieldReader<u8, DL_A>;
impl DL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DL_A> {
        match self.bits {
            2 => Some(DL_A::_0010),
            3 => Some(DL_A::_0011),
            4 => Some(DL_A::_0100),
            5 => Some(DL_A::_0101),
            6 => Some(DL_A::_0110),
            7 => Some(DL_A::_0111),
            8 => Some(DL_A::_1000),
            9 => Some(DL_A::_1001),
            10 => Some(DL_A::_1010),
            11 => Some(DL_A::_1011),
            12 => Some(DL_A::_1100),
            13 => Some(DL_A::_1101),
            14 => Some(DL_A::_1110),
            15 => Some(DL_A::_1111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == DL_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == DL_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == DL_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == DL_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == DL_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == DL_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == DL_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == DL_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == DL_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == DL_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == DL_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == DL_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == DL_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == DL_A::_1111
    }
}
#[doc = "Field `DL` writer - Data length = DL\\[3:0\\]+1"]
pub type DL_W<'a> = crate::FieldWriter<'a, u32, CTRL0_SPEC, u8, DL_A, 4, 8>;
impl<'a> DL_W<'a> {
    #[doc = "Data length=3"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(DL_A::_0010)
    }
    #[doc = "Data length=4"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(DL_A::_0011)
    }
    #[doc = "Data length=5"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(DL_A::_0100)
    }
    #[doc = "Data length=6"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(DL_A::_0101)
    }
    #[doc = "Data length=7"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(DL_A::_0110)
    }
    #[doc = "Data length=8"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(DL_A::_0111)
    }
    #[doc = "Data length=9"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(DL_A::_1000)
    }
    #[doc = "Data length=10"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(DL_A::_1001)
    }
    #[doc = "Data length=11"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(DL_A::_1010)
    }
    #[doc = "Data length=12"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(DL_A::_1011)
    }
    #[doc = "Data length=13"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(DL_A::_1100)
    }
    #[doc = "Data length=14"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(DL_A::_1101)
    }
    #[doc = "Data length=15"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(DL_A::_1110)
    }
    #[doc = "Data length=16"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(DL_A::_1111)
    }
}
#[doc = "TX FIFO Threshold level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXFIFOTH_A {
    #[doc = "0: TX FIFO threshold level is 0"]
    _0 = 0,
    #[doc = "1: TX FIFO threshold level is 1"]
    _1 = 1,
    #[doc = "2: TX FIFO threshold level is 2"]
    _2 = 2,
    #[doc = "3: TX FIFO threshold level is 3"]
    _3 = 3,
    #[doc = "4: TX FIFO threshold level is 4"]
    _4 = 4,
    #[doc = "5: TX FIFO threshold level is 5"]
    _5 = 5,
    #[doc = "6: TX FIFO threshold level is 6"]
    _6 = 6,
    #[doc = "7: TX FIFO threshold level is 7"]
    _7 = 7,
}
impl From<TXFIFOTH_A> for u8 {
    #[inline(always)]
    fn from(variant: TXFIFOTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TXFIFOTH` reader - TX FIFO Threshold level"]
pub type TXFIFOTH_R = crate::FieldReader<u8, TXFIFOTH_A>;
impl TXFIFOTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFIFOTH_A {
        match self.bits {
            0 => TXFIFOTH_A::_0,
            1 => TXFIFOTH_A::_1,
            2 => TXFIFOTH_A::_2,
            3 => TXFIFOTH_A::_3,
            4 => TXFIFOTH_A::_4,
            5 => TXFIFOTH_A::_5,
            6 => TXFIFOTH_A::_6,
            7 => TXFIFOTH_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXFIFOTH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXFIFOTH_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == TXFIFOTH_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == TXFIFOTH_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == TXFIFOTH_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == TXFIFOTH_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == TXFIFOTH_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == TXFIFOTH_A::_7
    }
}
#[doc = "Field `TXFIFOTH` writer - TX FIFO Threshold level"]
pub type TXFIFOTH_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL0_SPEC, u8, TXFIFOTH_A, 3, 12>;
impl<'a> TXFIFOTH_W<'a> {
    #[doc = "TX FIFO threshold level is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXFIFOTH_A::_0)
    }
    #[doc = "TX FIFO threshold level is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXFIFOTH_A::_1)
    }
    #[doc = "TX FIFO threshold level is 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TXFIFOTH_A::_2)
    }
    #[doc = "TX FIFO threshold level is 3"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TXFIFOTH_A::_3)
    }
    #[doc = "TX FIFO threshold level is 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TXFIFOTH_A::_4)
    }
    #[doc = "TX FIFO threshold level is 5"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(TXFIFOTH_A::_5)
    }
    #[doc = "TX FIFO threshold level is 6"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(TXFIFOTH_A::_6)
    }
    #[doc = "TX FIFO threshold level is 7"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(TXFIFOTH_A::_7)
    }
}
#[doc = "RX FIFO Threshold level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXFIFOTH_A {
    #[doc = "0: RX FIFO threshold level is 0"]
    _0 = 0,
    #[doc = "1: RX FIFO threshold level is 1"]
    _1 = 1,
    #[doc = "2: RX FIFO threshold level is 2"]
    _2 = 2,
    #[doc = "3: RX FIFO threshold level is 3"]
    _3 = 3,
    #[doc = "4: RX FIFO threshold level is 4"]
    _4 = 4,
    #[doc = "5: RX FIFO threshold level is 5"]
    _5 = 5,
    #[doc = "6: RX FIFO threshold level is 6"]
    _6 = 6,
    #[doc = "7: RX FIFO threshold level is 7"]
    _7 = 7,
}
impl From<RXFIFOTH_A> for u8 {
    #[inline(always)]
    fn from(variant: RXFIFOTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RXFIFOTH` reader - RX FIFO Threshold level"]
pub type RXFIFOTH_R = crate::FieldReader<u8, RXFIFOTH_A>;
impl RXFIFOTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFIFOTH_A {
        match self.bits {
            0 => RXFIFOTH_A::_0,
            1 => RXFIFOTH_A::_1,
            2 => RXFIFOTH_A::_2,
            3 => RXFIFOTH_A::_3,
            4 => RXFIFOTH_A::_4,
            5 => RXFIFOTH_A::_5,
            6 => RXFIFOTH_A::_6,
            7 => RXFIFOTH_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXFIFOTH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXFIFOTH_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == RXFIFOTH_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == RXFIFOTH_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == RXFIFOTH_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == RXFIFOTH_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == RXFIFOTH_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == RXFIFOTH_A::_7
    }
}
#[doc = "Field `RXFIFOTH` writer - RX FIFO Threshold level"]
pub type RXFIFOTH_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL0_SPEC, u8, RXFIFOTH_A, 3, 15>;
impl<'a> RXFIFOTH_W<'a> {
    #[doc = "RX FIFO threshold level is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXFIFOTH_A::_0)
    }
    #[doc = "RX FIFO threshold level is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXFIFOTH_A::_1)
    }
    #[doc = "RX FIFO threshold level is 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(RXFIFOTH_A::_2)
    }
    #[doc = "RX FIFO threshold level is 3"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(RXFIFOTH_A::_3)
    }
    #[doc = "RX FIFO threshold level is 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(RXFIFOTH_A::_4)
    }
    #[doc = "RX FIFO threshold level is 5"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(RXFIFOTH_A::_5)
    }
    #[doc = "RX FIFO threshold level is 6"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(RXFIFOTH_A::_6)
    }
    #[doc = "RX FIFO threshold level is 7"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(RXFIFOTH_A::_7)
    }
}
#[doc = "Auto-SEL disable bit. For SPI mode only\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELDIS_A {
    #[doc = "0: Enable Auto-SEL flow control"]
    ENABLE = 0,
    #[doc = "1: Disable Auto-SEL flow control"]
    DISABLE = 1,
}
impl From<SELDIS_A> for bool {
    #[inline(always)]
    fn from(variant: SELDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELDIS` reader - Auto-SEL disable bit. For SPI mode only"]
pub type SELDIS_R = crate::BitReader<SELDIS_A>;
impl SELDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELDIS_A {
        match self.bits {
            false => SELDIS_A::ENABLE,
            true => SELDIS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SELDIS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SELDIS_A::DISABLE
    }
}
#[doc = "Field `SELDIS` writer - Auto-SEL disable bit. For SPI mode only"]
pub type SELDIS_W<'a> = crate::BitWriter<'a, u32, CTRL0_SPEC, SELDIS_A, 18>;
impl<'a> SELDIS_W<'a> {
    #[doc = "Enable Auto-SEL flow control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SELDIS_A::ENABLE)
    }
    #[doc = "Disable Auto-SEL flow control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SELDIS_A::DISABLE)
    }
}
impl R {
    #[doc = "Bit 0 - SSP enable bit"]
    #[inline(always)]
    pub fn sspen(&self) -> SSPEN_R {
        SSPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Loopback mode enable"]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave data out disable"]
    #[inline(always)]
    pub fn sdodis(&self) -> SDODIS_R {
        SDODIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master/Slave selection"]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interface format"]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Data length = DL\\[3:0\\]+1"]
    #[inline(always)]
    pub fn dl(&self) -> DL_R {
        DL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - TX FIFO Threshold level"]
    #[inline(always)]
    pub fn txfifoth(&self) -> TXFIFOTH_R {
        TXFIFOTH_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - RX FIFO Threshold level"]
    #[inline(always)]
    pub fn rxfifoth(&self) -> RXFIFOTH_R {
        RXFIFOTH_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bit 18 - Auto-SEL disable bit. For SPI mode only"]
    #[inline(always)]
    pub fn seldis(&self) -> SELDIS_R {
        SELDIS_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSP enable bit"]
    #[inline(always)]
    pub fn sspen(&mut self) -> SSPEN_W {
        SSPEN_W::new(self)
    }
    #[doc = "Bit 1 - Loopback mode enable"]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W {
        LOOPBACK_W::new(self)
    }
    #[doc = "Bit 2 - Slave data out disable"]
    #[inline(always)]
    pub fn sdodis(&mut self) -> SDODIS_W {
        SDODIS_W::new(self)
    }
    #[doc = "Bit 3 - Master/Slave selection"]
    #[inline(always)]
    pub fn ms(&mut self) -> MS_W {
        MS_W::new(self)
    }
    #[doc = "Bit 4 - Interface format"]
    #[inline(always)]
    pub fn format(&mut self) -> FORMAT_W {
        FORMAT_W::new(self)
    }
    #[doc = "Bits 6:7 - SPI FSM and FIFO Reset"]
    #[inline(always)]
    pub fn freset(&mut self) -> FRESET_W {
        FRESET_W::new(self)
    }
    #[doc = "Bits 8:11 - Data length = DL\\[3:0\\]+1"]
    #[inline(always)]
    pub fn dl(&mut self) -> DL_W {
        DL_W::new(self)
    }
    #[doc = "Bits 12:14 - TX FIFO Threshold level"]
    #[inline(always)]
    pub fn txfifoth(&mut self) -> TXFIFOTH_W {
        TXFIFOTH_W::new(self)
    }
    #[doc = "Bits 15:17 - RX FIFO Threshold level"]
    #[inline(always)]
    pub fn rxfifoth(&mut self) -> RXFIFOTH_W {
        RXFIFOTH_W::new(self)
    }
    #[doc = "Bit 18 - Auto-SEL disable bit. For SPI mode only"]
    #[inline(always)]
    pub fn seldis(&mut self) -> SELDIS_W {
        SELDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x00 SPIn Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl0](index.html) module"]
pub struct CTRL0_SPEC;
impl crate::RegisterSpec for CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl0::R](R) reader structure"]
impl crate::Readable for CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl0::W](W) writer structure"]
impl crate::Writable for CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL0 to value 0x0004_0f00"]
impl crate::Resettable for CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0004_0f00
    }
}
