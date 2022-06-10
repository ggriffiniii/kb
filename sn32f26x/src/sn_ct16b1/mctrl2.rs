#[doc = "Register `MCTRL2` reader"]
pub struct R(crate::R<MCTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTRL2` writer"]
pub struct W(crate::W<MCTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTRL2_SPEC>;
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
impl From<crate::W<MCTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable generating an interrupt when MR10 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR10IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR10 matches TC"]
    ENABLE = 1,
}
impl From<MR10IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR10IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR10IE` reader - Enable generating an interrupt when MR10 matches TC"]
pub type MR10IE_R = crate::BitReader<MR10IE_A>;
impl MR10IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR10IE_A {
        match self.bits {
            false => MR10IE_A::DISABLE,
            true => MR10IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR10IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR10IE_A::ENABLE
    }
}
#[doc = "Field `MR10IE` writer - Enable generating an interrupt when MR10 matches TC"]
pub type MR10IE_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR10IE_A, 0>;
impl<'a> MR10IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR10IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR10 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR10IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR10 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR10RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR10 matches TC"]
    ENABLE = 1,
}
impl From<MR10RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR10RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR10RST` reader - Enable reset TC when MR10 matches TC"]
pub type MR10RST_R = crate::BitReader<MR10RST_A>;
impl MR10RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR10RST_A {
        match self.bits {
            false => MR10RST_A::DISABLE,
            true => MR10RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR10RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR10RST_A::ENABLE
    }
}
#[doc = "Field `MR10RST` writer - Enable reset TC when MR10 matches TC"]
pub type MR10RST_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR10RST_A, 1>;
impl<'a> MR10RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR10RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR10 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR10RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR10 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR10STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR10 matches TC"]
    ENABLE = 1,
}
impl From<MR10STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR10STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR10STOP` reader - Stop TC and PC and clear CEN bit when MR10 matches TC"]
pub type MR10STOP_R = crate::BitReader<MR10STOP_A>;
impl MR10STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR10STOP_A {
        match self.bits {
            false => MR10STOP_A::DISABLE,
            true => MR10STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR10STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR10STOP_A::ENABLE
    }
}
#[doc = "Field `MR10STOP` writer - Stop TC and PC and clear CEN bit when MR10 matches TC"]
pub type MR10STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR10STOP_A, 2>;
impl<'a> MR10STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR10STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR10 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR10STOP_A::ENABLE)
    }
}
#[doc = "Enable generating an interrupt when MR11 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR11IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR11 matches TC"]
    ENABLE = 1,
}
impl From<MR11IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR11IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR11IE` reader - Enable generating an interrupt when MR11 matches TC"]
pub type MR11IE_R = crate::BitReader<MR11IE_A>;
impl MR11IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR11IE_A {
        match self.bits {
            false => MR11IE_A::DISABLE,
            true => MR11IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR11IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR11IE_A::ENABLE
    }
}
#[doc = "Field `MR11IE` writer - Enable generating an interrupt when MR11 matches TC"]
pub type MR11IE_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR11IE_A, 3>;
impl<'a> MR11IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR11IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR11 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR11IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR11 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR11RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR11 matches TC"]
    ENABLE = 1,
}
impl From<MR11RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR11RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR11RST` reader - Enable reset TC when MR11 matches TC"]
pub type MR11RST_R = crate::BitReader<MR11RST_A>;
impl MR11RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR11RST_A {
        match self.bits {
            false => MR11RST_A::DISABLE,
            true => MR11RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR11RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR11RST_A::ENABLE
    }
}
#[doc = "Field `MR11RST` writer - Enable reset TC when MR11 matches TC"]
pub type MR11RST_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR11RST_A, 4>;
impl<'a> MR11RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR11RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR11 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR11RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR11 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR11STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR11 matches TC"]
    ENABLE = 1,
}
impl From<MR11STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR11STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR11STOP` reader - Stop TC and PC and clear CEN bit when MR11 matches TC"]
pub type MR11STOP_R = crate::BitReader<MR11STOP_A>;
impl MR11STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR11STOP_A {
        match self.bits {
            false => MR11STOP_A::DISABLE,
            true => MR11STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR11STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR11STOP_A::ENABLE
    }
}
#[doc = "Field `MR11STOP` writer - Stop TC and PC and clear CEN bit when MR11 matches TC"]
pub type MR11STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR11STOP_A, 5>;
impl<'a> MR11STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR11STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR11 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR11STOP_A::ENABLE)
    }
}
#[doc = "Enable generating an interrupt when MR12 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR12IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR12 matches TC"]
    ENABLE = 1,
}
impl From<MR12IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR12IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR12IE` reader - Enable generating an interrupt when MR12 matches TC"]
pub type MR12IE_R = crate::BitReader<MR12IE_A>;
impl MR12IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR12IE_A {
        match self.bits {
            false => MR12IE_A::DISABLE,
            true => MR12IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR12IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR12IE_A::ENABLE
    }
}
#[doc = "Field `MR12IE` writer - Enable generating an interrupt when MR12 matches TC"]
pub type MR12IE_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR12IE_A, 6>;
impl<'a> MR12IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR12IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR12 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR12IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR12 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR12RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR12 matches TC"]
    ENABLE = 1,
}
impl From<MR12RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR12RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR12RST` reader - Enable reset TC when MR12 matches TC"]
pub type MR12RST_R = crate::BitReader<MR12RST_A>;
impl MR12RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR12RST_A {
        match self.bits {
            false => MR12RST_A::DISABLE,
            true => MR12RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR12RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR12RST_A::ENABLE
    }
}
#[doc = "Field `MR12RST` writer - Enable reset TC when MR12 matches TC"]
pub type MR12RST_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR12RST_A, 7>;
impl<'a> MR12RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR12RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR12 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR12RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR12 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR12STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR12 matches TC"]
    ENABLE = 1,
}
impl From<MR12STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR12STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR12STOP` reader - Stop TC and PC and clear CEN bit when MR12 matches TC"]
pub type MR12STOP_R = crate::BitReader<MR12STOP_A>;
impl MR12STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR12STOP_A {
        match self.bits {
            false => MR12STOP_A::DISABLE,
            true => MR12STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR12STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR12STOP_A::ENABLE
    }
}
#[doc = "Field `MR12STOP` writer - Stop TC and PC and clear CEN bit when MR12 matches TC"]
pub type MR12STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR12STOP_A, 8>;
impl<'a> MR12STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR12STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR12 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR12STOP_A::ENABLE)
    }
}
#[doc = "Enable generating an interrupt when MR13 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR13IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR13 matches TC"]
    ENABLE = 1,
}
impl From<MR13IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR13IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR13IE` reader - Enable generating an interrupt when MR13 matches TC"]
pub type MR13IE_R = crate::BitReader<MR13IE_A>;
impl MR13IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR13IE_A {
        match self.bits {
            false => MR13IE_A::DISABLE,
            true => MR13IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR13IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR13IE_A::ENABLE
    }
}
#[doc = "Field `MR13IE` writer - Enable generating an interrupt when MR13 matches TC"]
pub type MR13IE_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR13IE_A, 9>;
impl<'a> MR13IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR13IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR13 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR13IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR13 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR13RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR13 matches TC"]
    ENABLE = 1,
}
impl From<MR13RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR13RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR13RST` reader - Enable reset TC when MR13 matches TC"]
pub type MR13RST_R = crate::BitReader<MR13RST_A>;
impl MR13RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR13RST_A {
        match self.bits {
            false => MR13RST_A::DISABLE,
            true => MR13RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR13RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR13RST_A::ENABLE
    }
}
#[doc = "Field `MR13RST` writer - Enable reset TC when MR13 matches TC"]
pub type MR13RST_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR13RST_A, 10>;
impl<'a> MR13RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR13RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR13 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR13RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR13 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR13STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR13 matches TC"]
    ENABLE = 1,
}
impl From<MR13STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR13STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR13STOP` reader - Stop TC and PC and clear CEN bit when MR13 matches TC"]
pub type MR13STOP_R = crate::BitReader<MR13STOP_A>;
impl MR13STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR13STOP_A {
        match self.bits {
            false => MR13STOP_A::DISABLE,
            true => MR13STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR13STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR13STOP_A::ENABLE
    }
}
#[doc = "Field `MR13STOP` writer - Stop TC and PC and clear CEN bit when MR13 matches TC"]
pub type MR13STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR13STOP_A, 11>;
impl<'a> MR13STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR13STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR13 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR13STOP_A::ENABLE)
    }
}
#[doc = "Enable generating an interrupt when MR14 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR14IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR14 matches TC"]
    ENABLE = 1,
}
impl From<MR14IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR14IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR14IE` reader - Enable generating an interrupt when MR14 matches TC"]
pub type MR14IE_R = crate::BitReader<MR14IE_A>;
impl MR14IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR14IE_A {
        match self.bits {
            false => MR14IE_A::DISABLE,
            true => MR14IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR14IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR14IE_A::ENABLE
    }
}
#[doc = "Field `MR14IE` writer - Enable generating an interrupt when MR14 matches TC"]
pub type MR14IE_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR14IE_A, 12>;
impl<'a> MR14IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR14IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR14 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR14IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR14 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR14RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR14 matches TC"]
    ENABLE = 1,
}
impl From<MR14RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR14RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR14RST` reader - Enable reset TC when MR14 matches TC"]
pub type MR14RST_R = crate::BitReader<MR14RST_A>;
impl MR14RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR14RST_A {
        match self.bits {
            false => MR14RST_A::DISABLE,
            true => MR14RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR14RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR14RST_A::ENABLE
    }
}
#[doc = "Field `MR14RST` writer - Enable reset TC when MR14 matches TC"]
pub type MR14RST_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR14RST_A, 13>;
impl<'a> MR14RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR14RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR14 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR14RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR14 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR14STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR14 matches TC"]
    ENABLE = 1,
}
impl From<MR14STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR14STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR14STOP` reader - Stop TC and PC and clear CEN bit when MR14 matches TC"]
pub type MR14STOP_R = crate::BitReader<MR14STOP_A>;
impl MR14STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR14STOP_A {
        match self.bits {
            false => MR14STOP_A::DISABLE,
            true => MR14STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR14STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR14STOP_A::ENABLE
    }
}
#[doc = "Field `MR14STOP` writer - Stop TC and PC and clear CEN bit when MR14 matches TC"]
pub type MR14STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR14STOP_A, 14>;
impl<'a> MR14STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR14STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR14 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR14STOP_A::ENABLE)
    }
}
#[doc = "Enable generating an interrupt when MR15 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR15IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR15 matches TC"]
    ENABLE = 1,
}
impl From<MR15IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR15IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR15IE` reader - Enable generating an interrupt when MR15 matches TC"]
pub type MR15IE_R = crate::BitReader<MR15IE_A>;
impl MR15IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR15IE_A {
        match self.bits {
            false => MR15IE_A::DISABLE,
            true => MR15IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR15IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR15IE_A::ENABLE
    }
}
#[doc = "Field `MR15IE` writer - Enable generating an interrupt when MR15 matches TC"]
pub type MR15IE_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR15IE_A, 15>;
impl<'a> MR15IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR15IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR15 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR15IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR15 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR15RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR15 matches TC"]
    ENABLE = 1,
}
impl From<MR15RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR15RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR15RST` reader - Enable reset TC when MR15 matches TC"]
pub type MR15RST_R = crate::BitReader<MR15RST_A>;
impl MR15RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR15RST_A {
        match self.bits {
            false => MR15RST_A::DISABLE,
            true => MR15RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR15RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR15RST_A::ENABLE
    }
}
#[doc = "Field `MR15RST` writer - Enable reset TC when MR15 matches TC"]
pub type MR15RST_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR15RST_A, 16>;
impl<'a> MR15RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR15RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR15 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR15RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR15 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR15STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR15 matches TC"]
    ENABLE = 1,
}
impl From<MR15STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR15STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR15STOP` reader - Stop TC and PC and clear CEN bit when MR15 matches TC"]
pub type MR15STOP_R = crate::BitReader<MR15STOP_A>;
impl MR15STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR15STOP_A {
        match self.bits {
            false => MR15STOP_A::DISABLE,
            true => MR15STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR15STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR15STOP_A::ENABLE
    }
}
#[doc = "Field `MR15STOP` writer - Stop TC and PC and clear CEN bit when MR15 matches TC"]
pub type MR15STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR15STOP_A, 17>;
impl<'a> MR15STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR15STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR15 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR15STOP_A::ENABLE)
    }
}
#[doc = "Enable generating an interrupt when MR16 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR16IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR16 matches TC"]
    ENABLE = 1,
}
impl From<MR16IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR16IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR16IE` reader - Enable generating an interrupt when MR16 matches TC"]
pub type MR16IE_R = crate::BitReader<MR16IE_A>;
impl MR16IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR16IE_A {
        match self.bits {
            false => MR16IE_A::DISABLE,
            true => MR16IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR16IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR16IE_A::ENABLE
    }
}
#[doc = "Field `MR16IE` writer - Enable generating an interrupt when MR16 matches TC"]
pub type MR16IE_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR16IE_A, 18>;
impl<'a> MR16IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR16IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR16 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR16IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR16 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR16RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR16 matches TC"]
    ENABLE = 1,
}
impl From<MR16RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR16RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR16RST` reader - Enable reset TC when MR16 matches TC"]
pub type MR16RST_R = crate::BitReader<MR16RST_A>;
impl MR16RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR16RST_A {
        match self.bits {
            false => MR16RST_A::DISABLE,
            true => MR16RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR16RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR16RST_A::ENABLE
    }
}
#[doc = "Field `MR16RST` writer - Enable reset TC when MR16 matches TC"]
pub type MR16RST_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR16RST_A, 19>;
impl<'a> MR16RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR16RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR16 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR16RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR16 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR16STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR16 matches TC"]
    ENABLE = 1,
}
impl From<MR16STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR16STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR16STOP` reader - Stop TC and PC and clear CEN bit when MR16 matches TC"]
pub type MR16STOP_R = crate::BitReader<MR16STOP_A>;
impl MR16STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR16STOP_A {
        match self.bits {
            false => MR16STOP_A::DISABLE,
            true => MR16STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR16STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR16STOP_A::ENABLE
    }
}
#[doc = "Field `MR16STOP` writer - Stop TC and PC and clear CEN bit when MR16 matches TC"]
pub type MR16STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR16STOP_A, 20>;
impl<'a> MR16STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR16STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR16 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR16STOP_A::ENABLE)
    }
}
#[doc = "Enable generating an interrupt when MR17 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR17IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR17 matches TC"]
    ENABLE = 1,
}
impl From<MR17IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR17IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR17IE` reader - Enable generating an interrupt when MR17 matches TC"]
pub type MR17IE_R = crate::BitReader<MR17IE_A>;
impl MR17IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR17IE_A {
        match self.bits {
            false => MR17IE_A::DISABLE,
            true => MR17IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR17IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR17IE_A::ENABLE
    }
}
#[doc = "Field `MR17IE` writer - Enable generating an interrupt when MR17 matches TC"]
pub type MR17IE_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR17IE_A, 21>;
impl<'a> MR17IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR17IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR17 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR17IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR17 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR17RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR17 matches TC"]
    ENABLE = 1,
}
impl From<MR17RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR17RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR17RST` reader - Enable reset TC when MR17 matches TC"]
pub type MR17RST_R = crate::BitReader<MR17RST_A>;
impl MR17RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR17RST_A {
        match self.bits {
            false => MR17RST_A::DISABLE,
            true => MR17RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR17RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR17RST_A::ENABLE
    }
}
#[doc = "Field `MR17RST` writer - Enable reset TC when MR17 matches TC"]
pub type MR17RST_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR17RST_A, 22>;
impl<'a> MR17RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR17RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR17 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR17RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR17 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR17STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR17 matches TC"]
    ENABLE = 1,
}
impl From<MR17STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR17STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR17STOP` reader - Stop TC and PC and clear CEN bit when MR17 matches TC"]
pub type MR17STOP_R = crate::BitReader<MR17STOP_A>;
impl MR17STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR17STOP_A {
        match self.bits {
            false => MR17STOP_A::DISABLE,
            true => MR17STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR17STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR17STOP_A::ENABLE
    }
}
#[doc = "Field `MR17STOP` writer - Stop TC and PC and clear CEN bit when MR17 matches TC"]
pub type MR17STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR17STOP_A, 23>;
impl<'a> MR17STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR17STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR17 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR17STOP_A::ENABLE)
    }
}
#[doc = "Enable generating an interrupt when MR18 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR18IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR18 matches TC"]
    ENABLE = 1,
}
impl From<MR18IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR18IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR18IE` reader - Enable generating an interrupt when MR18 matches TC"]
pub type MR18IE_R = crate::BitReader<MR18IE_A>;
impl MR18IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR18IE_A {
        match self.bits {
            false => MR18IE_A::DISABLE,
            true => MR18IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR18IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR18IE_A::ENABLE
    }
}
#[doc = "Field `MR18IE` writer - Enable generating an interrupt when MR18 matches TC"]
pub type MR18IE_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR18IE_A, 24>;
impl<'a> MR18IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR18IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR18 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR18IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR18 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR18RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR18 matches TC"]
    ENABLE = 1,
}
impl From<MR18RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR18RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR18RST` reader - Enable reset TC when MR18 matches TC"]
pub type MR18RST_R = crate::BitReader<MR18RST_A>;
impl MR18RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR18RST_A {
        match self.bits {
            false => MR18RST_A::DISABLE,
            true => MR18RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR18RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR18RST_A::ENABLE
    }
}
#[doc = "Field `MR18RST` writer - Enable reset TC when MR18 matches TC"]
pub type MR18RST_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR18RST_A, 25>;
impl<'a> MR18RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR18RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR18 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR18RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR18 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR18STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR18 matches TC"]
    ENABLE = 1,
}
impl From<MR18STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR18STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR18STOP` reader - Stop TC and PC and clear CEN bit when MR18 matches TC"]
pub type MR18STOP_R = crate::BitReader<MR18STOP_A>;
impl MR18STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR18STOP_A {
        match self.bits {
            false => MR18STOP_A::DISABLE,
            true => MR18STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR18STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR18STOP_A::ENABLE
    }
}
#[doc = "Field `MR18STOP` writer - Stop TC and PC and clear CEN bit when MR18 matches TC"]
pub type MR18STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR18STOP_A, 26>;
impl<'a> MR18STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR18STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR18 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR18STOP_A::ENABLE)
    }
}
#[doc = "Enable generating an interrupt based on CM\\[2:0\\]
when MR19 matches the value in the TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR19IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR19 matches TC"]
    ENABLE = 1,
}
impl From<MR19IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR19IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR19IE` reader - Enable generating an interrupt based on CM\\[2:0\\]
when MR19 matches the value in the TC"]
pub type MR19IE_R = crate::BitReader<MR19IE_A>;
impl MR19IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR19IE_A {
        match self.bits {
            false => MR19IE_A::DISABLE,
            true => MR19IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR19IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR19IE_A::ENABLE
    }
}
#[doc = "Field `MR19IE` writer - Enable generating an interrupt based on CM\\[2:0\\]
when MR19 matches the value in the TC"]
pub type MR19IE_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR19IE_A, 27>;
impl<'a> MR19IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR19IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR19 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR19IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR19 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR19RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR19 matches TC"]
    ENABLE = 1,
}
impl From<MR19RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR19RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR19RST` reader - Enable reset TC when MR19 matches TC"]
pub type MR19RST_R = crate::BitReader<MR19RST_A>;
impl MR19RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR19RST_A {
        match self.bits {
            false => MR19RST_A::DISABLE,
            true => MR19RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR19RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR19RST_A::ENABLE
    }
}
#[doc = "Field `MR19RST` writer - Enable reset TC when MR19 matches TC"]
pub type MR19RST_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR19RST_A, 28>;
impl<'a> MR19RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR19RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR19 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR19RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR19 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR19STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR19 matches TC"]
    ENABLE = 1,
}
impl From<MR19STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR19STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR19STOP` reader - Stop TC and PC and clear CEN bit when MR19 matches TC"]
pub type MR19STOP_R = crate::BitReader<MR19STOP_A>;
impl MR19STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR19STOP_A {
        match self.bits {
            false => MR19STOP_A::DISABLE,
            true => MR19STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR19STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR19STOP_A::ENABLE
    }
}
#[doc = "Field `MR19STOP` writer - Stop TC and PC and clear CEN bit when MR19 matches TC"]
pub type MR19STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL2_SPEC, MR19STOP_A, 29>;
impl<'a> MR19STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR19STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR19 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR19STOP_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable generating an interrupt when MR10 matches TC"]
    #[inline(always)]
    pub fn mr10ie(&self) -> MR10IE_R {
        MR10IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable reset TC when MR10 matches TC"]
    #[inline(always)]
    pub fn mr10rst(&self) -> MR10RST_R {
        MR10RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop TC and PC and clear CEN bit when MR10 matches TC"]
    #[inline(always)]
    pub fn mr10stop(&self) -> MR10STOP_R {
        MR10STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable generating an interrupt when MR11 matches TC"]
    #[inline(always)]
    pub fn mr11ie(&self) -> MR11IE_R {
        MR11IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable reset TC when MR11 matches TC"]
    #[inline(always)]
    pub fn mr11rst(&self) -> MR11RST_R {
        MR11RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop TC and PC and clear CEN bit when MR11 matches TC"]
    #[inline(always)]
    pub fn mr11stop(&self) -> MR11STOP_R {
        MR11STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable generating an interrupt when MR12 matches TC"]
    #[inline(always)]
    pub fn mr12ie(&self) -> MR12IE_R {
        MR12IE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable reset TC when MR12 matches TC"]
    #[inline(always)]
    pub fn mr12rst(&self) -> MR12RST_R {
        MR12RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Stop TC and PC and clear CEN bit when MR12 matches TC"]
    #[inline(always)]
    pub fn mr12stop(&self) -> MR12STOP_R {
        MR12STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable generating an interrupt when MR13 matches TC"]
    #[inline(always)]
    pub fn mr13ie(&self) -> MR13IE_R {
        MR13IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable reset TC when MR13 matches TC"]
    #[inline(always)]
    pub fn mr13rst(&self) -> MR13RST_R {
        MR13RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stop TC and PC and clear CEN bit when MR13 matches TC"]
    #[inline(always)]
    pub fn mr13stop(&self) -> MR13STOP_R {
        MR13STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable generating an interrupt when MR14 matches TC"]
    #[inline(always)]
    pub fn mr14ie(&self) -> MR14IE_R {
        MR14IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable reset TC when MR14 matches TC"]
    #[inline(always)]
    pub fn mr14rst(&self) -> MR14RST_R {
        MR14RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stop TC and PC and clear CEN bit when MR14 matches TC"]
    #[inline(always)]
    pub fn mr14stop(&self) -> MR14STOP_R {
        MR14STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable generating an interrupt when MR15 matches TC"]
    #[inline(always)]
    pub fn mr15ie(&self) -> MR15IE_R {
        MR15IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable reset TC when MR15 matches TC"]
    #[inline(always)]
    pub fn mr15rst(&self) -> MR15RST_R {
        MR15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Stop TC and PC and clear CEN bit when MR15 matches TC"]
    #[inline(always)]
    pub fn mr15stop(&self) -> MR15STOP_R {
        MR15STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable generating an interrupt when MR16 matches TC"]
    #[inline(always)]
    pub fn mr16ie(&self) -> MR16IE_R {
        MR16IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable reset TC when MR16 matches TC"]
    #[inline(always)]
    pub fn mr16rst(&self) -> MR16RST_R {
        MR16RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Stop TC and PC and clear CEN bit when MR16 matches TC"]
    #[inline(always)]
    pub fn mr16stop(&self) -> MR16STOP_R {
        MR16STOP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable generating an interrupt when MR17 matches TC"]
    #[inline(always)]
    pub fn mr17ie(&self) -> MR17IE_R {
        MR17IE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable reset TC when MR17 matches TC"]
    #[inline(always)]
    pub fn mr17rst(&self) -> MR17RST_R {
        MR17RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Stop TC and PC and clear CEN bit when MR17 matches TC"]
    #[inline(always)]
    pub fn mr17stop(&self) -> MR17STOP_R {
        MR17STOP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable generating an interrupt when MR18 matches TC"]
    #[inline(always)]
    pub fn mr18ie(&self) -> MR18IE_R {
        MR18IE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable reset TC when MR18 matches TC"]
    #[inline(always)]
    pub fn mr18rst(&self) -> MR18RST_R {
        MR18RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Stop TC and PC and clear CEN bit when MR18 matches TC"]
    #[inline(always)]
    pub fn mr18stop(&self) -> MR18STOP_R {
        MR18STOP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable generating an interrupt based on CM\\[2:0\\]
when MR19 matches the value in the TC"]
    #[inline(always)]
    pub fn mr19ie(&self) -> MR19IE_R {
        MR19IE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable reset TC when MR19 matches TC"]
    #[inline(always)]
    pub fn mr19rst(&self) -> MR19RST_R {
        MR19RST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Stop TC and PC and clear CEN bit when MR19 matches TC"]
    #[inline(always)]
    pub fn mr19stop(&self) -> MR19STOP_R {
        MR19STOP_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable generating an interrupt when MR10 matches TC"]
    #[inline(always)]
    pub fn mr10ie(&mut self) -> MR10IE_W {
        MR10IE_W::new(self)
    }
    #[doc = "Bit 1 - Enable reset TC when MR10 matches TC"]
    #[inline(always)]
    pub fn mr10rst(&mut self) -> MR10RST_W {
        MR10RST_W::new(self)
    }
    #[doc = "Bit 2 - Stop TC and PC and clear CEN bit when MR10 matches TC"]
    #[inline(always)]
    pub fn mr10stop(&mut self) -> MR10STOP_W {
        MR10STOP_W::new(self)
    }
    #[doc = "Bit 3 - Enable generating an interrupt when MR11 matches TC"]
    #[inline(always)]
    pub fn mr11ie(&mut self) -> MR11IE_W {
        MR11IE_W::new(self)
    }
    #[doc = "Bit 4 - Enable reset TC when MR11 matches TC"]
    #[inline(always)]
    pub fn mr11rst(&mut self) -> MR11RST_W {
        MR11RST_W::new(self)
    }
    #[doc = "Bit 5 - Stop TC and PC and clear CEN bit when MR11 matches TC"]
    #[inline(always)]
    pub fn mr11stop(&mut self) -> MR11STOP_W {
        MR11STOP_W::new(self)
    }
    #[doc = "Bit 6 - Enable generating an interrupt when MR12 matches TC"]
    #[inline(always)]
    pub fn mr12ie(&mut self) -> MR12IE_W {
        MR12IE_W::new(self)
    }
    #[doc = "Bit 7 - Enable reset TC when MR12 matches TC"]
    #[inline(always)]
    pub fn mr12rst(&mut self) -> MR12RST_W {
        MR12RST_W::new(self)
    }
    #[doc = "Bit 8 - Stop TC and PC and clear CEN bit when MR12 matches TC"]
    #[inline(always)]
    pub fn mr12stop(&mut self) -> MR12STOP_W {
        MR12STOP_W::new(self)
    }
    #[doc = "Bit 9 - Enable generating an interrupt when MR13 matches TC"]
    #[inline(always)]
    pub fn mr13ie(&mut self) -> MR13IE_W {
        MR13IE_W::new(self)
    }
    #[doc = "Bit 10 - Enable reset TC when MR13 matches TC"]
    #[inline(always)]
    pub fn mr13rst(&mut self) -> MR13RST_W {
        MR13RST_W::new(self)
    }
    #[doc = "Bit 11 - Stop TC and PC and clear CEN bit when MR13 matches TC"]
    #[inline(always)]
    pub fn mr13stop(&mut self) -> MR13STOP_W {
        MR13STOP_W::new(self)
    }
    #[doc = "Bit 12 - Enable generating an interrupt when MR14 matches TC"]
    #[inline(always)]
    pub fn mr14ie(&mut self) -> MR14IE_W {
        MR14IE_W::new(self)
    }
    #[doc = "Bit 13 - Enable reset TC when MR14 matches TC"]
    #[inline(always)]
    pub fn mr14rst(&mut self) -> MR14RST_W {
        MR14RST_W::new(self)
    }
    #[doc = "Bit 14 - Stop TC and PC and clear CEN bit when MR14 matches TC"]
    #[inline(always)]
    pub fn mr14stop(&mut self) -> MR14STOP_W {
        MR14STOP_W::new(self)
    }
    #[doc = "Bit 15 - Enable generating an interrupt when MR15 matches TC"]
    #[inline(always)]
    pub fn mr15ie(&mut self) -> MR15IE_W {
        MR15IE_W::new(self)
    }
    #[doc = "Bit 16 - Enable reset TC when MR15 matches TC"]
    #[inline(always)]
    pub fn mr15rst(&mut self) -> MR15RST_W {
        MR15RST_W::new(self)
    }
    #[doc = "Bit 17 - Stop TC and PC and clear CEN bit when MR15 matches TC"]
    #[inline(always)]
    pub fn mr15stop(&mut self) -> MR15STOP_W {
        MR15STOP_W::new(self)
    }
    #[doc = "Bit 18 - Enable generating an interrupt when MR16 matches TC"]
    #[inline(always)]
    pub fn mr16ie(&mut self) -> MR16IE_W {
        MR16IE_W::new(self)
    }
    #[doc = "Bit 19 - Enable reset TC when MR16 matches TC"]
    #[inline(always)]
    pub fn mr16rst(&mut self) -> MR16RST_W {
        MR16RST_W::new(self)
    }
    #[doc = "Bit 20 - Stop TC and PC and clear CEN bit when MR16 matches TC"]
    #[inline(always)]
    pub fn mr16stop(&mut self) -> MR16STOP_W {
        MR16STOP_W::new(self)
    }
    #[doc = "Bit 21 - Enable generating an interrupt when MR17 matches TC"]
    #[inline(always)]
    pub fn mr17ie(&mut self) -> MR17IE_W {
        MR17IE_W::new(self)
    }
    #[doc = "Bit 22 - Enable reset TC when MR17 matches TC"]
    #[inline(always)]
    pub fn mr17rst(&mut self) -> MR17RST_W {
        MR17RST_W::new(self)
    }
    #[doc = "Bit 23 - Stop TC and PC and clear CEN bit when MR17 matches TC"]
    #[inline(always)]
    pub fn mr17stop(&mut self) -> MR17STOP_W {
        MR17STOP_W::new(self)
    }
    #[doc = "Bit 24 - Enable generating an interrupt when MR18 matches TC"]
    #[inline(always)]
    pub fn mr18ie(&mut self) -> MR18IE_W {
        MR18IE_W::new(self)
    }
    #[doc = "Bit 25 - Enable reset TC when MR18 matches TC"]
    #[inline(always)]
    pub fn mr18rst(&mut self) -> MR18RST_W {
        MR18RST_W::new(self)
    }
    #[doc = "Bit 26 - Stop TC and PC and clear CEN bit when MR18 matches TC"]
    #[inline(always)]
    pub fn mr18stop(&mut self) -> MR18STOP_W {
        MR18STOP_W::new(self)
    }
    #[doc = "Bit 27 - Enable generating an interrupt based on CM\\[2:0\\]
when MR19 matches the value in the TC"]
    #[inline(always)]
    pub fn mr19ie(&mut self) -> MR19IE_W {
        MR19IE_W::new(self)
    }
    #[doc = "Bit 28 - Enable reset TC when MR19 matches TC"]
    #[inline(always)]
    pub fn mr19rst(&mut self) -> MR19RST_W {
        MR19RST_W::new(self)
    }
    #[doc = "Bit 29 - Stop TC and PC and clear CEN bit when MR19 matches TC"]
    #[inline(always)]
    pub fn mr19stop(&mut self) -> MR19STOP_W {
        MR19STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x18 CT16Bn Match Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctrl2](index.html) module"]
pub struct MCTRL2_SPEC;
impl crate::RegisterSpec for MCTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mctrl2::R](R) reader structure"]
impl crate::Readable for MCTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctrl2::W](W) writer structure"]
impl crate::Writable for MCTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCTRL2 to value 0"]
impl crate::Resettable for MCTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
