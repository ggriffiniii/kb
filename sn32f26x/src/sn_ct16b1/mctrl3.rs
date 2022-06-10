#[doc = "Register `MCTRL3` reader"]
pub struct R(crate::R<MCTRL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTRL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTRL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTRL3` writer"]
pub struct W(crate::W<MCTRL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTRL3_SPEC>;
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
impl From<crate::W<MCTRL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTRL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable generating an interrupt when MR20 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR20IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR20 matches TC"]
    ENABLE = 1,
}
impl From<MR20IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR20IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR20IE` reader - Enable generating an interrupt when MR20 matches TC"]
pub type MR20IE_R = crate::BitReader<MR20IE_A>;
impl MR20IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR20IE_A {
        match self.bits {
            false => MR20IE_A::DISABLE,
            true => MR20IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR20IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR20IE_A::ENABLE
    }
}
#[doc = "Field `MR20IE` writer - Enable generating an interrupt when MR20 matches TC"]
pub type MR20IE_W<'a> = crate::BitWriter<'a, u32, MCTRL3_SPEC, MR20IE_A, 0>;
impl<'a> MR20IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR20IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR20 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR20IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR20 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR20RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR20 matches TC"]
    ENABLE = 1,
}
impl From<MR20RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR20RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR20RST` reader - Enable reset TC when MR20 matches TC"]
pub type MR20RST_R = crate::BitReader<MR20RST_A>;
impl MR20RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR20RST_A {
        match self.bits {
            false => MR20RST_A::DISABLE,
            true => MR20RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR20RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR20RST_A::ENABLE
    }
}
#[doc = "Field `MR20RST` writer - Enable reset TC when MR20 matches TC"]
pub type MR20RST_W<'a> = crate::BitWriter<'a, u32, MCTRL3_SPEC, MR20RST_A, 1>;
impl<'a> MR20RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR20RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR20 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR20RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR20 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR20STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR20 matches TC"]
    ENABLE = 1,
}
impl From<MR20STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR20STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR20STOP` reader - Stop TC and PC and clear CEN bit when MR20 matches TC"]
pub type MR20STOP_R = crate::BitReader<MR20STOP_A>;
impl MR20STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR20STOP_A {
        match self.bits {
            false => MR20STOP_A::DISABLE,
            true => MR20STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR20STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR20STOP_A::ENABLE
    }
}
#[doc = "Field `MR20STOP` writer - Stop TC and PC and clear CEN bit when MR20 matches TC"]
pub type MR20STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL3_SPEC, MR20STOP_A, 2>;
impl<'a> MR20STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR20STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR20 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR20STOP_A::ENABLE)
    }
}
#[doc = "Enable generating an interrupt when MR21 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR21IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR21 matches TC"]
    ENABLE = 1,
}
impl From<MR21IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR21IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR21IE` reader - Enable generating an interrupt when MR21 matches TC"]
pub type MR21IE_R = crate::BitReader<MR21IE_A>;
impl MR21IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR21IE_A {
        match self.bits {
            false => MR21IE_A::DISABLE,
            true => MR21IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR21IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR21IE_A::ENABLE
    }
}
#[doc = "Field `MR21IE` writer - Enable generating an interrupt when MR21 matches TC"]
pub type MR21IE_W<'a> = crate::BitWriter<'a, u32, MCTRL3_SPEC, MR21IE_A, 3>;
impl<'a> MR21IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR21IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR21 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR21IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR21 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR21RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR21 matches TC"]
    ENABLE = 1,
}
impl From<MR21RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR21RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR21RST` reader - Enable reset TC when MR21 matches TC"]
pub type MR21RST_R = crate::BitReader<MR21RST_A>;
impl MR21RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR21RST_A {
        match self.bits {
            false => MR21RST_A::DISABLE,
            true => MR21RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR21RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR21RST_A::ENABLE
    }
}
#[doc = "Field `MR21RST` writer - Enable reset TC when MR21 matches TC"]
pub type MR21RST_W<'a> = crate::BitWriter<'a, u32, MCTRL3_SPEC, MR21RST_A, 4>;
impl<'a> MR21RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR21RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR21 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR21RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR21 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR21STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR21 matches TC"]
    ENABLE = 1,
}
impl From<MR21STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR21STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR21STOP` reader - Stop TC and PC and clear CEN bit when MR21 matches TC"]
pub type MR21STOP_R = crate::BitReader<MR21STOP_A>;
impl MR21STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR21STOP_A {
        match self.bits {
            false => MR21STOP_A::DISABLE,
            true => MR21STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR21STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR21STOP_A::ENABLE
    }
}
#[doc = "Field `MR21STOP` writer - Stop TC and PC and clear CEN bit when MR21 matches TC"]
pub type MR21STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL3_SPEC, MR21STOP_A, 5>;
impl<'a> MR21STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR21STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR21 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR21STOP_A::ENABLE)
    }
}
#[doc = "Enable generating an interrupt when MR22 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR22IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR22 matches TC"]
    ENABLE = 1,
}
impl From<MR22IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR22IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR22IE` reader - Enable generating an interrupt when MR22 matches TC"]
pub type MR22IE_R = crate::BitReader<MR22IE_A>;
impl MR22IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR22IE_A {
        match self.bits {
            false => MR22IE_A::DISABLE,
            true => MR22IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR22IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR22IE_A::ENABLE
    }
}
#[doc = "Field `MR22IE` writer - Enable generating an interrupt when MR22 matches TC"]
pub type MR22IE_W<'a> = crate::BitWriter<'a, u32, MCTRL3_SPEC, MR22IE_A, 6>;
impl<'a> MR22IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR22IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR22 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR22IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR22 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR22RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR22 matches TC"]
    ENABLE = 1,
}
impl From<MR22RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR22RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR22RST` reader - Enable reset TC when MR22 matches TC"]
pub type MR22RST_R = crate::BitReader<MR22RST_A>;
impl MR22RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR22RST_A {
        match self.bits {
            false => MR22RST_A::DISABLE,
            true => MR22RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR22RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR22RST_A::ENABLE
    }
}
#[doc = "Field `MR22RST` writer - Enable reset TC when MR22 matches TC"]
pub type MR22RST_W<'a> = crate::BitWriter<'a, u32, MCTRL3_SPEC, MR22RST_A, 7>;
impl<'a> MR22RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR22RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR22 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR22RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR22 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR22STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR22 matches TC"]
    ENABLE = 1,
}
impl From<MR22STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR22STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR22STOP` reader - Stop TC and PC and clear CEN bit when MR22 matches TC"]
pub type MR22STOP_R = crate::BitReader<MR22STOP_A>;
impl MR22STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR22STOP_A {
        match self.bits {
            false => MR22STOP_A::DISABLE,
            true => MR22STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR22STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR22STOP_A::ENABLE
    }
}
#[doc = "Field `MR22STOP` writer - Stop TC and PC and clear CEN bit when MR22 matches TC"]
pub type MR22STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL3_SPEC, MR22STOP_A, 8>;
impl<'a> MR22STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR22STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR22 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR22STOP_A::ENABLE)
    }
}
#[doc = "Enable generating an interrupt when MR23 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR23IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR23 matches TC"]
    ENABLE = 1,
}
impl From<MR23IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR23IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR23IE` reader - Enable generating an interrupt when MR23 matches TC"]
pub type MR23IE_R = crate::BitReader<MR23IE_A>;
impl MR23IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR23IE_A {
        match self.bits {
            false => MR23IE_A::DISABLE,
            true => MR23IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR23IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR23IE_A::ENABLE
    }
}
#[doc = "Field `MR23IE` writer - Enable generating an interrupt when MR23 matches TC"]
pub type MR23IE_W<'a> = crate::BitWriter<'a, u32, MCTRL3_SPEC, MR23IE_A, 9>;
impl<'a> MR23IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR23IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR23 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR23IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR23 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR23RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR23 matches TC"]
    ENABLE = 1,
}
impl From<MR23RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR23RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR23RST` reader - Enable reset TC when MR23 matches TC"]
pub type MR23RST_R = crate::BitReader<MR23RST_A>;
impl MR23RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR23RST_A {
        match self.bits {
            false => MR23RST_A::DISABLE,
            true => MR23RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR23RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR23RST_A::ENABLE
    }
}
#[doc = "Field `MR23RST` writer - Enable reset TC when MR23 matches TC"]
pub type MR23RST_W<'a> = crate::BitWriter<'a, u32, MCTRL3_SPEC, MR23RST_A, 10>;
impl<'a> MR23RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR23RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR23 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR23RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR23 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR23STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR23 matches TC"]
    ENABLE = 1,
}
impl From<MR23STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR23STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR23STOP` reader - Stop TC and PC and clear CEN bit when MR23 matches TC"]
pub type MR23STOP_R = crate::BitReader<MR23STOP_A>;
impl MR23STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR23STOP_A {
        match self.bits {
            false => MR23STOP_A::DISABLE,
            true => MR23STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR23STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR23STOP_A::ENABLE
    }
}
#[doc = "Field `MR23STOP` writer - Stop TC and PC and clear CEN bit when MR23 matches TC"]
pub type MR23STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL3_SPEC, MR23STOP_A, 11>;
impl<'a> MR23STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR23STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR23 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR23STOP_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable generating an interrupt when MR20 matches TC"]
    #[inline(always)]
    pub fn mr20ie(&self) -> MR20IE_R {
        MR20IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable reset TC when MR20 matches TC"]
    #[inline(always)]
    pub fn mr20rst(&self) -> MR20RST_R {
        MR20RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop TC and PC and clear CEN bit when MR20 matches TC"]
    #[inline(always)]
    pub fn mr20stop(&self) -> MR20STOP_R {
        MR20STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable generating an interrupt when MR21 matches TC"]
    #[inline(always)]
    pub fn mr21ie(&self) -> MR21IE_R {
        MR21IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable reset TC when MR21 matches TC"]
    #[inline(always)]
    pub fn mr21rst(&self) -> MR21RST_R {
        MR21RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop TC and PC and clear CEN bit when MR21 matches TC"]
    #[inline(always)]
    pub fn mr21stop(&self) -> MR21STOP_R {
        MR21STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable generating an interrupt when MR22 matches TC"]
    #[inline(always)]
    pub fn mr22ie(&self) -> MR22IE_R {
        MR22IE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable reset TC when MR22 matches TC"]
    #[inline(always)]
    pub fn mr22rst(&self) -> MR22RST_R {
        MR22RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Stop TC and PC and clear CEN bit when MR22 matches TC"]
    #[inline(always)]
    pub fn mr22stop(&self) -> MR22STOP_R {
        MR22STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable generating an interrupt when MR23 matches TC"]
    #[inline(always)]
    pub fn mr23ie(&self) -> MR23IE_R {
        MR23IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable reset TC when MR23 matches TC"]
    #[inline(always)]
    pub fn mr23rst(&self) -> MR23RST_R {
        MR23RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stop TC and PC and clear CEN bit when MR23 matches TC"]
    #[inline(always)]
    pub fn mr23stop(&self) -> MR23STOP_R {
        MR23STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable generating an interrupt when MR20 matches TC"]
    #[inline(always)]
    pub fn mr20ie(&mut self) -> MR20IE_W {
        MR20IE_W::new(self)
    }
    #[doc = "Bit 1 - Enable reset TC when MR20 matches TC"]
    #[inline(always)]
    pub fn mr20rst(&mut self) -> MR20RST_W {
        MR20RST_W::new(self)
    }
    #[doc = "Bit 2 - Stop TC and PC and clear CEN bit when MR20 matches TC"]
    #[inline(always)]
    pub fn mr20stop(&mut self) -> MR20STOP_W {
        MR20STOP_W::new(self)
    }
    #[doc = "Bit 3 - Enable generating an interrupt when MR21 matches TC"]
    #[inline(always)]
    pub fn mr21ie(&mut self) -> MR21IE_W {
        MR21IE_W::new(self)
    }
    #[doc = "Bit 4 - Enable reset TC when MR21 matches TC"]
    #[inline(always)]
    pub fn mr21rst(&mut self) -> MR21RST_W {
        MR21RST_W::new(self)
    }
    #[doc = "Bit 5 - Stop TC and PC and clear CEN bit when MR21 matches TC"]
    #[inline(always)]
    pub fn mr21stop(&mut self) -> MR21STOP_W {
        MR21STOP_W::new(self)
    }
    #[doc = "Bit 6 - Enable generating an interrupt when MR22 matches TC"]
    #[inline(always)]
    pub fn mr22ie(&mut self) -> MR22IE_W {
        MR22IE_W::new(self)
    }
    #[doc = "Bit 7 - Enable reset TC when MR22 matches TC"]
    #[inline(always)]
    pub fn mr22rst(&mut self) -> MR22RST_W {
        MR22RST_W::new(self)
    }
    #[doc = "Bit 8 - Stop TC and PC and clear CEN bit when MR22 matches TC"]
    #[inline(always)]
    pub fn mr22stop(&mut self) -> MR22STOP_W {
        MR22STOP_W::new(self)
    }
    #[doc = "Bit 9 - Enable generating an interrupt when MR23 matches TC"]
    #[inline(always)]
    pub fn mr23ie(&mut self) -> MR23IE_W {
        MR23IE_W::new(self)
    }
    #[doc = "Bit 10 - Enable reset TC when MR23 matches TC"]
    #[inline(always)]
    pub fn mr23rst(&mut self) -> MR23RST_W {
        MR23RST_W::new(self)
    }
    #[doc = "Bit 11 - Stop TC and PC and clear CEN bit when MR23 matches TC"]
    #[inline(always)]
    pub fn mr23stop(&mut self) -> MR23STOP_W {
        MR23STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x1C CT16Bn Match Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctrl3](index.html) module"]
pub struct MCTRL3_SPEC;
impl crate::RegisterSpec for MCTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mctrl3::R](R) reader structure"]
impl crate::Readable for MCTRL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctrl3::W](W) writer structure"]
impl crate::Writable for MCTRL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCTRL3 to value 0"]
impl crate::Resettable for MCTRL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
