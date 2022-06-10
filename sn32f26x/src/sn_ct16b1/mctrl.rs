#[doc = "Register `MCTRL` reader"]
pub struct R(crate::R<MCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTRL` writer"]
pub struct W(crate::W<MCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTRL_SPEC>;
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
impl From<crate::W<MCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable generating an interrupt when MR0 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR0 matches TC"]
    ENABLE = 1,
}
impl From<MR0IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR0IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0IE` reader - Enable generating an interrupt when MR0 matches TC"]
pub type MR0IE_R = crate::BitReader<MR0IE_A>;
impl MR0IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR0IE_A {
        match self.bits {
            false => MR0IE_A::DISABLE,
            true => MR0IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR0IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR0IE_A::ENABLE
    }
}
#[doc = "Field `MR0IE` writer - Enable generating an interrupt when MR0 matches TC"]
pub type MR0IE_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR0IE_A, 0>;
impl<'a> MR0IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR0IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR0 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR0IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR0 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR0 matches TC"]
    ENABLE = 1,
}
impl From<MR0RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR0RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0RST` reader - Enable reset TC when MR0 matches TC"]
pub type MR0RST_R = crate::BitReader<MR0RST_A>;
impl MR0RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR0RST_A {
        match self.bits {
            false => MR0RST_A::DISABLE,
            true => MR0RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR0RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR0RST_A::ENABLE
    }
}
#[doc = "Field `MR0RST` writer - Enable reset TC when MR0 matches TC"]
pub type MR0RST_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR0RST_A, 1>;
impl<'a> MR0RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR0RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR0 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR0RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR0 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR0 matches TC"]
    ENABLE = 1,
}
impl From<MR0STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR0STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0STOP` reader - Stop TC and PC and clear CEN bit when MR0 matches TC"]
pub type MR0STOP_R = crate::BitReader<MR0STOP_A>;
impl MR0STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR0STOP_A {
        match self.bits {
            false => MR0STOP_A::DISABLE,
            true => MR0STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR0STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR0STOP_A::ENABLE
    }
}
#[doc = "Field `MR0STOP` writer - Stop TC and PC and clear CEN bit when MR0 matches TC"]
pub type MR0STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR0STOP_A, 2>;
impl<'a> MR0STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR0STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR0 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR0STOP_A::ENABLE)
    }
}
#[doc = "Enable generating an interrupt when MR1 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR1IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR1 matches TC"]
    ENABLE = 1,
}
impl From<MR1IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR1IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR1IE` reader - Enable generating an interrupt when MR1 matches TC"]
pub type MR1IE_R = crate::BitReader<MR1IE_A>;
impl MR1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR1IE_A {
        match self.bits {
            false => MR1IE_A::DISABLE,
            true => MR1IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR1IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR1IE_A::ENABLE
    }
}
#[doc = "Field `MR1IE` writer - Enable generating an interrupt when MR1 matches TC"]
pub type MR1IE_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR1IE_A, 3>;
impl<'a> MR1IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR1IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR1 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR1IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR1 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR1RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR1 matches TC"]
    ENABLE = 1,
}
impl From<MR1RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR1RST` reader - Enable reset TC when MR1 matches TC"]
pub type MR1RST_R = crate::BitReader<MR1RST_A>;
impl MR1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR1RST_A {
        match self.bits {
            false => MR1RST_A::DISABLE,
            true => MR1RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR1RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR1RST_A::ENABLE
    }
}
#[doc = "Field `MR1RST` writer - Enable reset TC when MR1 matches TC"]
pub type MR1RST_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR1RST_A, 4>;
impl<'a> MR1RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR1RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR1 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR1RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR1 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR1STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR1 matches TC"]
    ENABLE = 1,
}
impl From<MR1STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR1STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR1STOP` reader - Stop TC and PC and clear CEN bit when MR1 matches TC"]
pub type MR1STOP_R = crate::BitReader<MR1STOP_A>;
impl MR1STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR1STOP_A {
        match self.bits {
            false => MR1STOP_A::DISABLE,
            true => MR1STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR1STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR1STOP_A::ENABLE
    }
}
#[doc = "Field `MR1STOP` writer - Stop TC and PC and clear CEN bit when MR1 matches TC"]
pub type MR1STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR1STOP_A, 5>;
impl<'a> MR1STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR1STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR1 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR1STOP_A::ENABLE)
    }
}
#[doc = "Enable generating an interrupt when MR2 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR2IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR2 matches TC"]
    ENABLE = 1,
}
impl From<MR2IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR2IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR2IE` reader - Enable generating an interrupt when MR2 matches TC"]
pub type MR2IE_R = crate::BitReader<MR2IE_A>;
impl MR2IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR2IE_A {
        match self.bits {
            false => MR2IE_A::DISABLE,
            true => MR2IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR2IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR2IE_A::ENABLE
    }
}
#[doc = "Field `MR2IE` writer - Enable generating an interrupt when MR2 matches TC"]
pub type MR2IE_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR2IE_A, 6>;
impl<'a> MR2IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR2IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR2 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR2IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR2 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR2RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR2 matches TC"]
    ENABLE = 1,
}
impl From<MR2RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR2RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR2RST` reader - Enable reset TC when MR2 matches TC"]
pub type MR2RST_R = crate::BitReader<MR2RST_A>;
impl MR2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR2RST_A {
        match self.bits {
            false => MR2RST_A::DISABLE,
            true => MR2RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR2RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR2RST_A::ENABLE
    }
}
#[doc = "Field `MR2RST` writer - Enable reset TC when MR2 matches TC"]
pub type MR2RST_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR2RST_A, 7>;
impl<'a> MR2RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR2RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR2 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR2RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR2 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR2STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR2 matches TC"]
    ENABLE = 1,
}
impl From<MR2STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR2STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR2STOP` reader - Stop TC and PC and clear CEN bit when MR2 matches TC"]
pub type MR2STOP_R = crate::BitReader<MR2STOP_A>;
impl MR2STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR2STOP_A {
        match self.bits {
            false => MR2STOP_A::DISABLE,
            true => MR2STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR2STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR2STOP_A::ENABLE
    }
}
#[doc = "Field `MR2STOP` writer - Stop TC and PC and clear CEN bit when MR2 matches TC"]
pub type MR2STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR2STOP_A, 8>;
impl<'a> MR2STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR2STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR2 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR2STOP_A::ENABLE)
    }
}
#[doc = "Enable generating an interrupt when MR3 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR3IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR3 matches TC"]
    ENABLE = 1,
}
impl From<MR3IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR3IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR3IE` reader - Enable generating an interrupt when MR3 matches TC"]
pub type MR3IE_R = crate::BitReader<MR3IE_A>;
impl MR3IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR3IE_A {
        match self.bits {
            false => MR3IE_A::DISABLE,
            true => MR3IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR3IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR3IE_A::ENABLE
    }
}
#[doc = "Field `MR3IE` writer - Enable generating an interrupt when MR3 matches TC"]
pub type MR3IE_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR3IE_A, 9>;
impl<'a> MR3IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR3IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR3 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR3IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR3 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR3RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR3 matches TC"]
    ENABLE = 1,
}
impl From<MR3RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR3RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR3RST` reader - Enable reset TC when MR3 matches TC"]
pub type MR3RST_R = crate::BitReader<MR3RST_A>;
impl MR3RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR3RST_A {
        match self.bits {
            false => MR3RST_A::DISABLE,
            true => MR3RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR3RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR3RST_A::ENABLE
    }
}
#[doc = "Field `MR3RST` writer - Enable reset TC when MR3 matches TC"]
pub type MR3RST_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR3RST_A, 10>;
impl<'a> MR3RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR3RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR3 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR3RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR3 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR3STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR3 matches TC"]
    ENABLE = 1,
}
impl From<MR3STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR3STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR3STOP` reader - Stop TC and PC and clear CEN bit when MR3 matches TC"]
pub type MR3STOP_R = crate::BitReader<MR3STOP_A>;
impl MR3STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR3STOP_A {
        match self.bits {
            false => MR3STOP_A::DISABLE,
            true => MR3STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR3STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR3STOP_A::ENABLE
    }
}
#[doc = "Field `MR3STOP` writer - Stop TC and PC and clear CEN bit when MR3 matches TC"]
pub type MR3STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR3STOP_A, 11>;
impl<'a> MR3STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR3STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR3 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR3STOP_A::ENABLE)
    }
}
#[doc = "Enable generating an interrupt when MR4 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR4IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR4 matches TC"]
    ENABLE = 1,
}
impl From<MR4IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR4IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR4IE` reader - Enable generating an interrupt when MR4 matches TC"]
pub type MR4IE_R = crate::BitReader<MR4IE_A>;
impl MR4IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR4IE_A {
        match self.bits {
            false => MR4IE_A::DISABLE,
            true => MR4IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR4IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR4IE_A::ENABLE
    }
}
#[doc = "Field `MR4IE` writer - Enable generating an interrupt when MR4 matches TC"]
pub type MR4IE_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR4IE_A, 12>;
impl<'a> MR4IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR4IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR4 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR4IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR4 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR4RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR4 matches TC"]
    ENABLE = 1,
}
impl From<MR4RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR4RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR4RST` reader - Enable reset TC when MR4 matches TC"]
pub type MR4RST_R = crate::BitReader<MR4RST_A>;
impl MR4RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR4RST_A {
        match self.bits {
            false => MR4RST_A::DISABLE,
            true => MR4RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR4RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR4RST_A::ENABLE
    }
}
#[doc = "Field `MR4RST` writer - Enable reset TC when MR4 matches TC"]
pub type MR4RST_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR4RST_A, 13>;
impl<'a> MR4RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR4RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR4 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR4RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR4 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR4STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR4 matches TC"]
    ENABLE = 1,
}
impl From<MR4STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR4STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR4STOP` reader - Stop TC and PC and clear CEN bit when MR4 matches TC"]
pub type MR4STOP_R = crate::BitReader<MR4STOP_A>;
impl MR4STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR4STOP_A {
        match self.bits {
            false => MR4STOP_A::DISABLE,
            true => MR4STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR4STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR4STOP_A::ENABLE
    }
}
#[doc = "Field `MR4STOP` writer - Stop TC and PC and clear CEN bit when MR4 matches TC"]
pub type MR4STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR4STOP_A, 14>;
impl<'a> MR4STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR4STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR4 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR4STOP_A::ENABLE)
    }
}
#[doc = "Enable generating an interrupt when MR5 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR5IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR5 matches TC"]
    ENABLE = 1,
}
impl From<MR5IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR5IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR5IE` reader - Enable generating an interrupt when MR5 matches TC"]
pub type MR5IE_R = crate::BitReader<MR5IE_A>;
impl MR5IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR5IE_A {
        match self.bits {
            false => MR5IE_A::DISABLE,
            true => MR5IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR5IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR5IE_A::ENABLE
    }
}
#[doc = "Field `MR5IE` writer - Enable generating an interrupt when MR5 matches TC"]
pub type MR5IE_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR5IE_A, 15>;
impl<'a> MR5IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR5IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR5 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR5IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR5 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR5RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR5 matches TC"]
    ENABLE = 1,
}
impl From<MR5RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR5RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR5RST` reader - Enable reset TC when MR5 matches TC"]
pub type MR5RST_R = crate::BitReader<MR5RST_A>;
impl MR5RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR5RST_A {
        match self.bits {
            false => MR5RST_A::DISABLE,
            true => MR5RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR5RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR5RST_A::ENABLE
    }
}
#[doc = "Field `MR5RST` writer - Enable reset TC when MR5 matches TC"]
pub type MR5RST_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR5RST_A, 16>;
impl<'a> MR5RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR5RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR5 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR5RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR5 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR5STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR5 matches TC"]
    ENABLE = 1,
}
impl From<MR5STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR5STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR5STOP` reader - Stop TC and PC and clear CEN bit when MR5 matches TC"]
pub type MR5STOP_R = crate::BitReader<MR5STOP_A>;
impl MR5STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR5STOP_A {
        match self.bits {
            false => MR5STOP_A::DISABLE,
            true => MR5STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR5STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR5STOP_A::ENABLE
    }
}
#[doc = "Field `MR5STOP` writer - Stop TC and PC and clear CEN bit when MR5 matches TC"]
pub type MR5STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR5STOP_A, 17>;
impl<'a> MR5STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR5STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR5 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR5STOP_A::ENABLE)
    }
}
#[doc = "Enable generating an interrupt when MR6 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR6IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR6 matches TC"]
    ENABLE = 1,
}
impl From<MR6IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR6IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR6IE` reader - Enable generating an interrupt when MR6 matches TC"]
pub type MR6IE_R = crate::BitReader<MR6IE_A>;
impl MR6IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR6IE_A {
        match self.bits {
            false => MR6IE_A::DISABLE,
            true => MR6IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR6IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR6IE_A::ENABLE
    }
}
#[doc = "Field `MR6IE` writer - Enable generating an interrupt when MR6 matches TC"]
pub type MR6IE_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR6IE_A, 18>;
impl<'a> MR6IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR6IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR6 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR6IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR6 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR6RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR6 matches TC"]
    ENABLE = 1,
}
impl From<MR6RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR6RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR6RST` reader - Enable reset TC when MR6 matches TC"]
pub type MR6RST_R = crate::BitReader<MR6RST_A>;
impl MR6RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR6RST_A {
        match self.bits {
            false => MR6RST_A::DISABLE,
            true => MR6RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR6RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR6RST_A::ENABLE
    }
}
#[doc = "Field `MR6RST` writer - Enable reset TC when MR6 matches TC"]
pub type MR6RST_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR6RST_A, 19>;
impl<'a> MR6RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR6RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR6 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR6RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR6 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR6STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR6 matches TC"]
    ENABLE = 1,
}
impl From<MR6STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR6STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR6STOP` reader - Stop TC and PC and clear CEN bit when MR6 matches TC"]
pub type MR6STOP_R = crate::BitReader<MR6STOP_A>;
impl MR6STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR6STOP_A {
        match self.bits {
            false => MR6STOP_A::DISABLE,
            true => MR6STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR6STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR6STOP_A::ENABLE
    }
}
#[doc = "Field `MR6STOP` writer - Stop TC and PC and clear CEN bit when MR6 matches TC"]
pub type MR6STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR6STOP_A, 20>;
impl<'a> MR6STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR6STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR6 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR6STOP_A::ENABLE)
    }
}
#[doc = "Enable generating an interrupt when MR7 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR7IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR7 matches TC"]
    ENABLE = 1,
}
impl From<MR7IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR7IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR7IE` reader - Enable generating an interrupt when MR7 matches TC"]
pub type MR7IE_R = crate::BitReader<MR7IE_A>;
impl MR7IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR7IE_A {
        match self.bits {
            false => MR7IE_A::DISABLE,
            true => MR7IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR7IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR7IE_A::ENABLE
    }
}
#[doc = "Field `MR7IE` writer - Enable generating an interrupt when MR7 matches TC"]
pub type MR7IE_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR7IE_A, 21>;
impl<'a> MR7IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR7IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR7 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR7IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR7 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR7RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR7 matches TC"]
    ENABLE = 1,
}
impl From<MR7RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR7RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR7RST` reader - Enable reset TC when MR7 matches TC"]
pub type MR7RST_R = crate::BitReader<MR7RST_A>;
impl MR7RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR7RST_A {
        match self.bits {
            false => MR7RST_A::DISABLE,
            true => MR7RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR7RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR7RST_A::ENABLE
    }
}
#[doc = "Field `MR7RST` writer - Enable reset TC when MR7 matches TC"]
pub type MR7RST_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR7RST_A, 22>;
impl<'a> MR7RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR7RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR7 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR7RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR7 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR7STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR7 matches TC"]
    ENABLE = 1,
}
impl From<MR7STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR7STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR7STOP` reader - Stop TC and PC and clear CEN bit when MR7 matches TC"]
pub type MR7STOP_R = crate::BitReader<MR7STOP_A>;
impl MR7STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR7STOP_A {
        match self.bits {
            false => MR7STOP_A::DISABLE,
            true => MR7STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR7STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR7STOP_A::ENABLE
    }
}
#[doc = "Field `MR7STOP` writer - Stop TC and PC and clear CEN bit when MR7 matches TC"]
pub type MR7STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR7STOP_A, 23>;
impl<'a> MR7STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR7STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR7 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR7STOP_A::ENABLE)
    }
}
#[doc = "Enable generating an interrupt when MR8 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR8IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR8 matches TC"]
    ENABLE = 1,
}
impl From<MR8IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR8IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR8IE` reader - Enable generating an interrupt when MR8 matches TC"]
pub type MR8IE_R = crate::BitReader<MR8IE_A>;
impl MR8IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR8IE_A {
        match self.bits {
            false => MR8IE_A::DISABLE,
            true => MR8IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR8IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR8IE_A::ENABLE
    }
}
#[doc = "Field `MR8IE` writer - Enable generating an interrupt when MR8 matches TC"]
pub type MR8IE_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR8IE_A, 24>;
impl<'a> MR8IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR8IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR8 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR8IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR8 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR8RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR8 matches TC"]
    ENABLE = 1,
}
impl From<MR8RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR8RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR8RST` reader - Enable reset TC when MR8 matches TC"]
pub type MR8RST_R = crate::BitReader<MR8RST_A>;
impl MR8RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR8RST_A {
        match self.bits {
            false => MR8RST_A::DISABLE,
            true => MR8RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR8RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR8RST_A::ENABLE
    }
}
#[doc = "Field `MR8RST` writer - Enable reset TC when MR8 matches TC"]
pub type MR8RST_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR8RST_A, 25>;
impl<'a> MR8RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR8RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR8 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR8RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR8 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR8STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR8 matches TC"]
    ENABLE = 1,
}
impl From<MR8STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR8STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR8STOP` reader - Stop TC and PC and clear CEN bit when MR8 matches TC"]
pub type MR8STOP_R = crate::BitReader<MR8STOP_A>;
impl MR8STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR8STOP_A {
        match self.bits {
            false => MR8STOP_A::DISABLE,
            true => MR8STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR8STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR8STOP_A::ENABLE
    }
}
#[doc = "Field `MR8STOP` writer - Stop TC and PC and clear CEN bit when MR8 matches TC"]
pub type MR8STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR8STOP_A, 26>;
impl<'a> MR8STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR8STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR8 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR8STOP_A::ENABLE)
    }
}
#[doc = "Enable generating an interrupt based on CM\\[2:0\\]
when MR9 matches the value in the TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR9IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR9 matches TC"]
    ENABLE = 1,
}
impl From<MR9IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR9IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR9IE` reader - Enable generating an interrupt based on CM\\[2:0\\]
when MR9 matches the value in the TC"]
pub type MR9IE_R = crate::BitReader<MR9IE_A>;
impl MR9IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR9IE_A {
        match self.bits {
            false => MR9IE_A::DISABLE,
            true => MR9IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR9IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR9IE_A::ENABLE
    }
}
#[doc = "Field `MR9IE` writer - Enable generating an interrupt based on CM\\[2:0\\]
when MR9 matches the value in the TC"]
pub type MR9IE_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR9IE_A, 27>;
impl<'a> MR9IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR9IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR9 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR9IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR9 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR9RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR9 matches TC"]
    ENABLE = 1,
}
impl From<MR9RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR9RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR9RST` reader - Enable reset TC when MR9 matches TC"]
pub type MR9RST_R = crate::BitReader<MR9RST_A>;
impl MR9RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR9RST_A {
        match self.bits {
            false => MR9RST_A::DISABLE,
            true => MR9RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR9RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR9RST_A::ENABLE
    }
}
#[doc = "Field `MR9RST` writer - Enable reset TC when MR9 matches TC"]
pub type MR9RST_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR9RST_A, 28>;
impl<'a> MR9RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR9RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR9 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR9RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR9 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR9STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR9 matches TC"]
    ENABLE = 1,
}
impl From<MR9STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR9STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR9STOP` reader - Stop TC and PC and clear CEN bit when MR9 matches TC"]
pub type MR9STOP_R = crate::BitReader<MR9STOP_A>;
impl MR9STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR9STOP_A {
        match self.bits {
            false => MR9STOP_A::DISABLE,
            true => MR9STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR9STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR9STOP_A::ENABLE
    }
}
#[doc = "Field `MR9STOP` writer - Stop TC and PC and clear CEN bit when MR9 matches TC"]
pub type MR9STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR9STOP_A, 29>;
impl<'a> MR9STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR9STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR9 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR9STOP_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable generating an interrupt when MR0 matches TC"]
    #[inline(always)]
    pub fn mr0ie(&self) -> MR0IE_R {
        MR0IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable reset TC when MR0 matches TC"]
    #[inline(always)]
    pub fn mr0rst(&self) -> MR0RST_R {
        MR0RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop TC and PC and clear CEN bit when MR0 matches TC"]
    #[inline(always)]
    pub fn mr0stop(&self) -> MR0STOP_R {
        MR0STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable generating an interrupt when MR1 matches TC"]
    #[inline(always)]
    pub fn mr1ie(&self) -> MR1IE_R {
        MR1IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable reset TC when MR1 matches TC"]
    #[inline(always)]
    pub fn mr1rst(&self) -> MR1RST_R {
        MR1RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop TC and PC and clear CEN bit when MR1 matches TC"]
    #[inline(always)]
    pub fn mr1stop(&self) -> MR1STOP_R {
        MR1STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable generating an interrupt when MR2 matches TC"]
    #[inline(always)]
    pub fn mr2ie(&self) -> MR2IE_R {
        MR2IE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable reset TC when MR2 matches TC"]
    #[inline(always)]
    pub fn mr2rst(&self) -> MR2RST_R {
        MR2RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Stop TC and PC and clear CEN bit when MR2 matches TC"]
    #[inline(always)]
    pub fn mr2stop(&self) -> MR2STOP_R {
        MR2STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable generating an interrupt when MR3 matches TC"]
    #[inline(always)]
    pub fn mr3ie(&self) -> MR3IE_R {
        MR3IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable reset TC when MR3 matches TC"]
    #[inline(always)]
    pub fn mr3rst(&self) -> MR3RST_R {
        MR3RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stop TC and PC and clear CEN bit when MR3 matches TC"]
    #[inline(always)]
    pub fn mr3stop(&self) -> MR3STOP_R {
        MR3STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable generating an interrupt when MR4 matches TC"]
    #[inline(always)]
    pub fn mr4ie(&self) -> MR4IE_R {
        MR4IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable reset TC when MR4 matches TC"]
    #[inline(always)]
    pub fn mr4rst(&self) -> MR4RST_R {
        MR4RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stop TC and PC and clear CEN bit when MR4 matches TC"]
    #[inline(always)]
    pub fn mr4stop(&self) -> MR4STOP_R {
        MR4STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable generating an interrupt when MR5 matches TC"]
    #[inline(always)]
    pub fn mr5ie(&self) -> MR5IE_R {
        MR5IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable reset TC when MR5 matches TC"]
    #[inline(always)]
    pub fn mr5rst(&self) -> MR5RST_R {
        MR5RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Stop TC and PC and clear CEN bit when MR5 matches TC"]
    #[inline(always)]
    pub fn mr5stop(&self) -> MR5STOP_R {
        MR5STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable generating an interrupt when MR6 matches TC"]
    #[inline(always)]
    pub fn mr6ie(&self) -> MR6IE_R {
        MR6IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable reset TC when MR6 matches TC"]
    #[inline(always)]
    pub fn mr6rst(&self) -> MR6RST_R {
        MR6RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Stop TC and PC and clear CEN bit when MR6 matches TC"]
    #[inline(always)]
    pub fn mr6stop(&self) -> MR6STOP_R {
        MR6STOP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable generating an interrupt when MR7 matches TC"]
    #[inline(always)]
    pub fn mr7ie(&self) -> MR7IE_R {
        MR7IE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable reset TC when MR7 matches TC"]
    #[inline(always)]
    pub fn mr7rst(&self) -> MR7RST_R {
        MR7RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Stop TC and PC and clear CEN bit when MR7 matches TC"]
    #[inline(always)]
    pub fn mr7stop(&self) -> MR7STOP_R {
        MR7STOP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable generating an interrupt when MR8 matches TC"]
    #[inline(always)]
    pub fn mr8ie(&self) -> MR8IE_R {
        MR8IE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable reset TC when MR8 matches TC"]
    #[inline(always)]
    pub fn mr8rst(&self) -> MR8RST_R {
        MR8RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Stop TC and PC and clear CEN bit when MR8 matches TC"]
    #[inline(always)]
    pub fn mr8stop(&self) -> MR8STOP_R {
        MR8STOP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable generating an interrupt based on CM\\[2:0\\]
when MR9 matches the value in the TC"]
    #[inline(always)]
    pub fn mr9ie(&self) -> MR9IE_R {
        MR9IE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable reset TC when MR9 matches TC"]
    #[inline(always)]
    pub fn mr9rst(&self) -> MR9RST_R {
        MR9RST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Stop TC and PC and clear CEN bit when MR9 matches TC"]
    #[inline(always)]
    pub fn mr9stop(&self) -> MR9STOP_R {
        MR9STOP_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable generating an interrupt when MR0 matches TC"]
    #[inline(always)]
    pub fn mr0ie(&mut self) -> MR0IE_W {
        MR0IE_W::new(self)
    }
    #[doc = "Bit 1 - Enable reset TC when MR0 matches TC"]
    #[inline(always)]
    pub fn mr0rst(&mut self) -> MR0RST_W {
        MR0RST_W::new(self)
    }
    #[doc = "Bit 2 - Stop TC and PC and clear CEN bit when MR0 matches TC"]
    #[inline(always)]
    pub fn mr0stop(&mut self) -> MR0STOP_W {
        MR0STOP_W::new(self)
    }
    #[doc = "Bit 3 - Enable generating an interrupt when MR1 matches TC"]
    #[inline(always)]
    pub fn mr1ie(&mut self) -> MR1IE_W {
        MR1IE_W::new(self)
    }
    #[doc = "Bit 4 - Enable reset TC when MR1 matches TC"]
    #[inline(always)]
    pub fn mr1rst(&mut self) -> MR1RST_W {
        MR1RST_W::new(self)
    }
    #[doc = "Bit 5 - Stop TC and PC and clear CEN bit when MR1 matches TC"]
    #[inline(always)]
    pub fn mr1stop(&mut self) -> MR1STOP_W {
        MR1STOP_W::new(self)
    }
    #[doc = "Bit 6 - Enable generating an interrupt when MR2 matches TC"]
    #[inline(always)]
    pub fn mr2ie(&mut self) -> MR2IE_W {
        MR2IE_W::new(self)
    }
    #[doc = "Bit 7 - Enable reset TC when MR2 matches TC"]
    #[inline(always)]
    pub fn mr2rst(&mut self) -> MR2RST_W {
        MR2RST_W::new(self)
    }
    #[doc = "Bit 8 - Stop TC and PC and clear CEN bit when MR2 matches TC"]
    #[inline(always)]
    pub fn mr2stop(&mut self) -> MR2STOP_W {
        MR2STOP_W::new(self)
    }
    #[doc = "Bit 9 - Enable generating an interrupt when MR3 matches TC"]
    #[inline(always)]
    pub fn mr3ie(&mut self) -> MR3IE_W {
        MR3IE_W::new(self)
    }
    #[doc = "Bit 10 - Enable reset TC when MR3 matches TC"]
    #[inline(always)]
    pub fn mr3rst(&mut self) -> MR3RST_W {
        MR3RST_W::new(self)
    }
    #[doc = "Bit 11 - Stop TC and PC and clear CEN bit when MR3 matches TC"]
    #[inline(always)]
    pub fn mr3stop(&mut self) -> MR3STOP_W {
        MR3STOP_W::new(self)
    }
    #[doc = "Bit 12 - Enable generating an interrupt when MR4 matches TC"]
    #[inline(always)]
    pub fn mr4ie(&mut self) -> MR4IE_W {
        MR4IE_W::new(self)
    }
    #[doc = "Bit 13 - Enable reset TC when MR4 matches TC"]
    #[inline(always)]
    pub fn mr4rst(&mut self) -> MR4RST_W {
        MR4RST_W::new(self)
    }
    #[doc = "Bit 14 - Stop TC and PC and clear CEN bit when MR4 matches TC"]
    #[inline(always)]
    pub fn mr4stop(&mut self) -> MR4STOP_W {
        MR4STOP_W::new(self)
    }
    #[doc = "Bit 15 - Enable generating an interrupt when MR5 matches TC"]
    #[inline(always)]
    pub fn mr5ie(&mut self) -> MR5IE_W {
        MR5IE_W::new(self)
    }
    #[doc = "Bit 16 - Enable reset TC when MR5 matches TC"]
    #[inline(always)]
    pub fn mr5rst(&mut self) -> MR5RST_W {
        MR5RST_W::new(self)
    }
    #[doc = "Bit 17 - Stop TC and PC and clear CEN bit when MR5 matches TC"]
    #[inline(always)]
    pub fn mr5stop(&mut self) -> MR5STOP_W {
        MR5STOP_W::new(self)
    }
    #[doc = "Bit 18 - Enable generating an interrupt when MR6 matches TC"]
    #[inline(always)]
    pub fn mr6ie(&mut self) -> MR6IE_W {
        MR6IE_W::new(self)
    }
    #[doc = "Bit 19 - Enable reset TC when MR6 matches TC"]
    #[inline(always)]
    pub fn mr6rst(&mut self) -> MR6RST_W {
        MR6RST_W::new(self)
    }
    #[doc = "Bit 20 - Stop TC and PC and clear CEN bit when MR6 matches TC"]
    #[inline(always)]
    pub fn mr6stop(&mut self) -> MR6STOP_W {
        MR6STOP_W::new(self)
    }
    #[doc = "Bit 21 - Enable generating an interrupt when MR7 matches TC"]
    #[inline(always)]
    pub fn mr7ie(&mut self) -> MR7IE_W {
        MR7IE_W::new(self)
    }
    #[doc = "Bit 22 - Enable reset TC when MR7 matches TC"]
    #[inline(always)]
    pub fn mr7rst(&mut self) -> MR7RST_W {
        MR7RST_W::new(self)
    }
    #[doc = "Bit 23 - Stop TC and PC and clear CEN bit when MR7 matches TC"]
    #[inline(always)]
    pub fn mr7stop(&mut self) -> MR7STOP_W {
        MR7STOP_W::new(self)
    }
    #[doc = "Bit 24 - Enable generating an interrupt when MR8 matches TC"]
    #[inline(always)]
    pub fn mr8ie(&mut self) -> MR8IE_W {
        MR8IE_W::new(self)
    }
    #[doc = "Bit 25 - Enable reset TC when MR8 matches TC"]
    #[inline(always)]
    pub fn mr8rst(&mut self) -> MR8RST_W {
        MR8RST_W::new(self)
    }
    #[doc = "Bit 26 - Stop TC and PC and clear CEN bit when MR8 matches TC"]
    #[inline(always)]
    pub fn mr8stop(&mut self) -> MR8STOP_W {
        MR8STOP_W::new(self)
    }
    #[doc = "Bit 27 - Enable generating an interrupt based on CM\\[2:0\\]
when MR9 matches the value in the TC"]
    #[inline(always)]
    pub fn mr9ie(&mut self) -> MR9IE_W {
        MR9IE_W::new(self)
    }
    #[doc = "Bit 28 - Enable reset TC when MR9 matches TC"]
    #[inline(always)]
    pub fn mr9rst(&mut self) -> MR9RST_W {
        MR9RST_W::new(self)
    }
    #[doc = "Bit 29 - Stop TC and PC and clear CEN bit when MR9 matches TC"]
    #[inline(always)]
    pub fn mr9stop(&mut self) -> MR9STOP_W {
        MR9STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x14 CT16Bn Match Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctrl](index.html) module"]
pub struct MCTRL_SPEC;
impl crate::RegisterSpec for MCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mctrl::R](R) reader structure"]
impl crate::Readable for MCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctrl::W](W) writer structure"]
impl crate::Writable for MCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCTRL to value 0"]
impl crate::Resettable for MCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
