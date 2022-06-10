#[doc = "Register `AHBCLKEN` reader"]
pub struct R(crate::R<AHBCLKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBCLKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBCLKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBCLKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBCLKEN` writer"]
pub struct W(crate::W<AHBCLKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBCLKEN_SPEC>;
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
impl From<crate::W<AHBCLKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBCLKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable AHB clock for P0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0CLKEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<P0CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: P0CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0CLKEN` reader - Enable AHB clock for P0"]
pub type P0CLKEN_R = crate::BitReader<P0CLKEN_A>;
impl P0CLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0CLKEN_A {
        match self.bits {
            false => P0CLKEN_A::DISABLE,
            true => P0CLKEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == P0CLKEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == P0CLKEN_A::ENABLE
    }
}
#[doc = "Field `P0CLKEN` writer - Enable AHB clock for P0"]
pub type P0CLKEN_W<'a> = crate::BitWriter<'a, u32, AHBCLKEN_SPEC, P0CLKEN_A, 0>;
impl<'a> P0CLKEN_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(P0CLKEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(P0CLKEN_A::ENABLE)
    }
}
#[doc = "Enable AHB clock for P1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1CLKEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<P1CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: P1CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1CLKEN` reader - Enable AHB clock for P1"]
pub type P1CLKEN_R = crate::BitReader<P1CLKEN_A>;
impl P1CLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1CLKEN_A {
        match self.bits {
            false => P1CLKEN_A::DISABLE,
            true => P1CLKEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == P1CLKEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == P1CLKEN_A::ENABLE
    }
}
#[doc = "Field `P1CLKEN` writer - Enable AHB clock for P1"]
pub type P1CLKEN_W<'a> = crate::BitWriter<'a, u32, AHBCLKEN_SPEC, P1CLKEN_A, 1>;
impl<'a> P1CLKEN_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(P1CLKEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(P1CLKEN_A::ENABLE)
    }
}
#[doc = "Enable AHB clock for P2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2CLKEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<P2CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: P2CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2CLKEN` reader - Enable AHB clock for P2"]
pub type P2CLKEN_R = crate::BitReader<P2CLKEN_A>;
impl P2CLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2CLKEN_A {
        match self.bits {
            false => P2CLKEN_A::DISABLE,
            true => P2CLKEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == P2CLKEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == P2CLKEN_A::ENABLE
    }
}
#[doc = "Field `P2CLKEN` writer - Enable AHB clock for P2"]
pub type P2CLKEN_W<'a> = crate::BitWriter<'a, u32, AHBCLKEN_SPEC, P2CLKEN_A, 2>;
impl<'a> P2CLKEN_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(P2CLKEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(P2CLKEN_A::ENABLE)
    }
}
#[doc = "Enable AHB clock for P3\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P3CLKEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<P3CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: P3CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P3CLKEN` reader - Enable AHB clock for P3"]
pub type P3CLKEN_R = crate::BitReader<P3CLKEN_A>;
impl P3CLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P3CLKEN_A {
        match self.bits {
            false => P3CLKEN_A::DISABLE,
            true => P3CLKEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == P3CLKEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == P3CLKEN_A::ENABLE
    }
}
#[doc = "Field `P3CLKEN` writer - Enable AHB clock for P3"]
pub type P3CLKEN_W<'a> = crate::BitWriter<'a, u32, AHBCLKEN_SPEC, P3CLKEN_A, 3>;
impl<'a> P3CLKEN_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(P3CLKEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(P3CLKEN_A::ENABLE)
    }
}
#[doc = "Enable AHB clock for USB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBCLKEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<USBCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBCLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCLKEN` reader - Enable AHB clock for USB"]
pub type USBCLKEN_R = crate::BitReader<USBCLKEN_A>;
impl USBCLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBCLKEN_A {
        match self.bits {
            false => USBCLKEN_A::DISABLE,
            true => USBCLKEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USBCLKEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USBCLKEN_A::ENABLE
    }
}
#[doc = "Field `USBCLKEN` writer - Enable AHB clock for USB"]
pub type USBCLKEN_W<'a> = crate::BitWriter<'a, u32, AHBCLKEN_SPEC, USBCLKEN_A, 4>;
impl<'a> USBCLKEN_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USBCLKEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USBCLKEN_A::ENABLE)
    }
}
#[doc = "Enable AHB clock for CT16B0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT16B0CLKEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<CT16B0CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CT16B0CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT16B0CLKEN` reader - Enable AHB clock for CT16B0"]
pub type CT16B0CLKEN_R = crate::BitReader<CT16B0CLKEN_A>;
impl CT16B0CLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT16B0CLKEN_A {
        match self.bits {
            false => CT16B0CLKEN_A::DISABLE,
            true => CT16B0CLKEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CT16B0CLKEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CT16B0CLKEN_A::ENABLE
    }
}
#[doc = "Field `CT16B0CLKEN` writer - Enable AHB clock for CT16B0"]
pub type CT16B0CLKEN_W<'a> = crate::BitWriter<'a, u32, AHBCLKEN_SPEC, CT16B0CLKEN_A, 6>;
impl<'a> CT16B0CLKEN_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CT16B0CLKEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CT16B0CLKEN_A::ENABLE)
    }
}
#[doc = "Enable AHB clock for CT16B1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT16B1CLKEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<CT16B1CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CT16B1CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT16B1CLKEN` reader - Enable AHB clock for CT16B1"]
pub type CT16B1CLKEN_R = crate::BitReader<CT16B1CLKEN_A>;
impl CT16B1CLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT16B1CLKEN_A {
        match self.bits {
            false => CT16B1CLKEN_A::DISABLE,
            true => CT16B1CLKEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CT16B1CLKEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CT16B1CLKEN_A::ENABLE
    }
}
#[doc = "Field `CT16B1CLKEN` writer - Enable AHB clock for CT16B1"]
pub type CT16B1CLKEN_W<'a> = crate::BitWriter<'a, u32, AHBCLKEN_SPEC, CT16B1CLKEN_A, 7>;
impl<'a> CT16B1CLKEN_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CT16B1CLKEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CT16B1CLKEN_A::ENABLE)
    }
}
#[doc = "Enable AHB clock for SPI0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0CLKEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<SPI0CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI0CLKEN` reader - Enable AHB clock for SPI0"]
pub type SPI0CLKEN_R = crate::BitReader<SPI0CLKEN_A>;
impl SPI0CLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0CLKEN_A {
        match self.bits {
            false => SPI0CLKEN_A::DISABLE,
            true => SPI0CLKEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SPI0CLKEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SPI0CLKEN_A::ENABLE
    }
}
#[doc = "Field `SPI0CLKEN` writer - Enable AHB clock for SPI0"]
pub type SPI0CLKEN_W<'a> = crate::BitWriter<'a, u32, AHBCLKEN_SPEC, SPI0CLKEN_A, 12>;
impl<'a> SPI0CLKEN_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SPI0CLKEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SPI0CLKEN_A::ENABLE)
    }
}
#[doc = "Enable AHB clock for I2C0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0CLKEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<I2C0CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C0CLKEN` reader - Enable AHB clock for I2C0"]
pub type I2C0CLKEN_R = crate::BitReader<I2C0CLKEN_A>;
impl I2C0CLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0CLKEN_A {
        match self.bits {
            false => I2C0CLKEN_A::DISABLE,
            true => I2C0CLKEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == I2C0CLKEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == I2C0CLKEN_A::ENABLE
    }
}
#[doc = "Field `I2C0CLKEN` writer - Enable AHB clock for I2C0"]
pub type I2C0CLKEN_W<'a> = crate::BitWriter<'a, u32, AHBCLKEN_SPEC, I2C0CLKEN_A, 21>;
impl<'a> I2C0CLKEN_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(I2C0CLKEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2C0CLKEN_A::ENABLE)
    }
}
#[doc = "Enable AHB clock for WDT\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTCLKEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<WDTCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: WDTCLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCLKEN` reader - Enable AHB clock for WDT"]
pub type WDTCLKEN_R = crate::BitReader<WDTCLKEN_A>;
impl WDTCLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTCLKEN_A {
        match self.bits {
            false => WDTCLKEN_A::DISABLE,
            true => WDTCLKEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WDTCLKEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WDTCLKEN_A::ENABLE
    }
}
#[doc = "Field `WDTCLKEN` writer - Enable AHB clock for WDT"]
pub type WDTCLKEN_W<'a> = crate::BitWriter<'a, u32, AHBCLKEN_SPEC, WDTCLKEN_A, 24>;
impl<'a> WDTCLKEN_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WDTCLKEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WDTCLKEN_A::ENABLE)
    }
}
#[doc = "Clock output source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKOUTSEL_A {
    #[doc = "0: Disable"]
    _000 = 0,
    #[doc = "1: ILRC"]
    _001 = 1,
    #[doc = "4: HCLK"]
    _100 = 4,
    #[doc = "5: IHRC"]
    _101 = 5,
}
impl From<CLKOUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKOUTSEL` reader - Clock output source selection"]
pub type CLKOUTSEL_R = crate::FieldReader<u8, CLKOUTSEL_A>;
impl CLKOUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKOUTSEL_A> {
        match self.bits {
            0 => Some(CLKOUTSEL_A::_000),
            1 => Some(CLKOUTSEL_A::_001),
            4 => Some(CLKOUTSEL_A::_100),
            5 => Some(CLKOUTSEL_A::_101),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CLKOUTSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CLKOUTSEL_A::_001
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CLKOUTSEL_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CLKOUTSEL_A::_101
    }
}
#[doc = "Field `CLKOUTSEL` writer - Clock output source selection"]
pub type CLKOUTSEL_W<'a> = crate::FieldWriter<'a, u32, AHBCLKEN_SPEC, u8, CLKOUTSEL_A, 3, 28>;
impl<'a> CLKOUTSEL_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_000)
    }
    #[doc = "ILRC"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_001)
    }
    #[doc = "HCLK"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_100)
    }
    #[doc = "IHRC"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_101)
    }
}
impl R {
    #[doc = "Bit 0 - Enable AHB clock for P0"]
    #[inline(always)]
    pub fn p0clken(&self) -> P0CLKEN_R {
        P0CLKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable AHB clock for P1"]
    #[inline(always)]
    pub fn p1clken(&self) -> P1CLKEN_R {
        P1CLKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable AHB clock for P2"]
    #[inline(always)]
    pub fn p2clken(&self) -> P2CLKEN_R {
        P2CLKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable AHB clock for P3"]
    #[inline(always)]
    pub fn p3clken(&self) -> P3CLKEN_R {
        P3CLKEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable AHB clock for USB"]
    #[inline(always)]
    pub fn usbclken(&self) -> USBCLKEN_R {
        USBCLKEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable AHB clock for CT16B0"]
    #[inline(always)]
    pub fn ct16b0clken(&self) -> CT16B0CLKEN_R {
        CT16B0CLKEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable AHB clock for CT16B1"]
    #[inline(always)]
    pub fn ct16b1clken(&self) -> CT16B1CLKEN_R {
        CT16B1CLKEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable AHB clock for SPI0"]
    #[inline(always)]
    pub fn spi0clken(&self) -> SPI0CLKEN_R {
        SPI0CLKEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable AHB clock for I2C0"]
    #[inline(always)]
    pub fn i2c0clken(&self) -> I2C0CLKEN_R {
        I2C0CLKEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable AHB clock for WDT"]
    #[inline(always)]
    pub fn wdtclken(&self) -> WDTCLKEN_R {
        WDTCLKEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Clock output source selection"]
    #[inline(always)]
    pub fn clkoutsel(&self) -> CLKOUTSEL_R {
        CLKOUTSEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable AHB clock for P0"]
    #[inline(always)]
    pub fn p0clken(&mut self) -> P0CLKEN_W {
        P0CLKEN_W::new(self)
    }
    #[doc = "Bit 1 - Enable AHB clock for P1"]
    #[inline(always)]
    pub fn p1clken(&mut self) -> P1CLKEN_W {
        P1CLKEN_W::new(self)
    }
    #[doc = "Bit 2 - Enable AHB clock for P2"]
    #[inline(always)]
    pub fn p2clken(&mut self) -> P2CLKEN_W {
        P2CLKEN_W::new(self)
    }
    #[doc = "Bit 3 - Enable AHB clock for P3"]
    #[inline(always)]
    pub fn p3clken(&mut self) -> P3CLKEN_W {
        P3CLKEN_W::new(self)
    }
    #[doc = "Bit 4 - Enable AHB clock for USB"]
    #[inline(always)]
    pub fn usbclken(&mut self) -> USBCLKEN_W {
        USBCLKEN_W::new(self)
    }
    #[doc = "Bit 6 - Enable AHB clock for CT16B0"]
    #[inline(always)]
    pub fn ct16b0clken(&mut self) -> CT16B0CLKEN_W {
        CT16B0CLKEN_W::new(self)
    }
    #[doc = "Bit 7 - Enable AHB clock for CT16B1"]
    #[inline(always)]
    pub fn ct16b1clken(&mut self) -> CT16B1CLKEN_W {
        CT16B1CLKEN_W::new(self)
    }
    #[doc = "Bit 12 - Enable AHB clock for SPI0"]
    #[inline(always)]
    pub fn spi0clken(&mut self) -> SPI0CLKEN_W {
        SPI0CLKEN_W::new(self)
    }
    #[doc = "Bit 21 - Enable AHB clock for I2C0"]
    #[inline(always)]
    pub fn i2c0clken(&mut self) -> I2C0CLKEN_W {
        I2C0CLKEN_W::new(self)
    }
    #[doc = "Bit 24 - Enable AHB clock for WDT"]
    #[inline(always)]
    pub fn wdtclken(&mut self) -> WDTCLKEN_W {
        WDTCLKEN_W::new(self)
    }
    #[doc = "Bits 28:30 - Clock output source selection"]
    #[inline(always)]
    pub fn clkoutsel(&mut self) -> CLKOUTSEL_W {
        CLKOUTSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x00 AHB Clock Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclken](index.html) module"]
pub struct AHBCLKEN_SPEC;
impl crate::RegisterSpec for AHBCLKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbclken::R](R) reader structure"]
impl crate::Readable for AHBCLKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbclken::W](W) writer structure"]
impl crate::Writable for AHBCLKEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBCLKEN to value 0x0100_000f"]
impl crate::Resettable for AHBCLKEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_000f
    }
}
