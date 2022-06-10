#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Match channel 0 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0IF_A {
    #[doc = "0: No interrupt on match channel 0"]
    NOINTERRUPT = 0,
    #[doc = "1: Interrupt requirements met on match channel 0"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR0IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR0IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0IF` reader - Match channel 0 interrupt flag"]
pub type MR0IF_R = crate::BitReader<MR0IF_A>;
impl MR0IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR0IF_A {
        match self.bits {
            false => MR0IF_A::NOINTERRUPT,
            true => MR0IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NOINTERRUPT`"]
    #[inline(always)]
    pub fn is_nointerrupt(&self) -> bool {
        *self == MR0IF_A::NOINTERRUPT
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR0IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 1 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR1IF_A {
    #[doc = "0: No interrupt on match channel 1"]
    NO = 0,
    #[doc = "1: Interrupt requirements met on match channel 1"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR1IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR1IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR1IF` reader - Match channel 1 interrupt flag"]
pub type MR1IF_R = crate::BitReader<MR1IF_A>;
impl MR1IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR1IF_A {
        match self.bits {
            false => MR1IF_A::NO,
            true => MR1IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == MR1IF_A::NO
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR1IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 2 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR2IF_A {
    #[doc = "0: No interrupt on match channel 2"]
    NO = 0,
    #[doc = "1: Interrupt requirements met on match channel 2"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR2IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR2IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR2IF` reader - Match channel 2 interrupt flag"]
pub type MR2IF_R = crate::BitReader<MR2IF_A>;
impl MR2IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR2IF_A {
        match self.bits {
            false => MR2IF_A::NO,
            true => MR2IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == MR2IF_A::NO
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR2IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 3 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR3IF_A {
    #[doc = "0: No interrupt on match channel 3"]
    NO = 0,
    #[doc = "1: Interrupt requirements met on match channel 3"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR3IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR3IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR3IF` reader - Match channel 3 interrupt flag"]
pub type MR3IF_R = crate::BitReader<MR3IF_A>;
impl MR3IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR3IF_A {
        match self.bits {
            false => MR3IF_A::NO,
            true => MR3IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == MR3IF_A::NO
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR3IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 4 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR4IF_A {
    #[doc = "0: No interrupt on match channel 4"]
    NOINTERRUPT = 0,
    #[doc = "1: Interrupt requirements met on match channel 4"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR4IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR4IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR4IF` reader - Match channel 4 interrupt flag"]
pub type MR4IF_R = crate::BitReader<MR4IF_A>;
impl MR4IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR4IF_A {
        match self.bits {
            false => MR4IF_A::NOINTERRUPT,
            true => MR4IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NOINTERRUPT`"]
    #[inline(always)]
    pub fn is_nointerrupt(&self) -> bool {
        *self == MR4IF_A::NOINTERRUPT
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR4IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 5 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR5IF_A {
    #[doc = "0: No interrupt on match channel 5"]
    NO = 0,
    #[doc = "1: Interrupt requirements met on match channel 5"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR5IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR5IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR5IF` reader - Match channel 5 interrupt flag"]
pub type MR5IF_R = crate::BitReader<MR5IF_A>;
impl MR5IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR5IF_A {
        match self.bits {
            false => MR5IF_A::NO,
            true => MR5IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == MR5IF_A::NO
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR5IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 6 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR6IF_A {
    #[doc = "0: No interrupt on match channel 6"]
    NO = 0,
    #[doc = "1: Interrupt requirements met on match channel 6"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR6IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR6IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR6IF` reader - Match channel 6 interrupt flag"]
pub type MR6IF_R = crate::BitReader<MR6IF_A>;
impl MR6IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR6IF_A {
        match self.bits {
            false => MR6IF_A::NO,
            true => MR6IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == MR6IF_A::NO
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR6IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 7 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR7IF_A {
    #[doc = "0: No interrupt on match channel 7"]
    NO = 0,
    #[doc = "1: Interrupt requirements met on match channel 7"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR7IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR7IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR7IF` reader - Match channel 7 interrupt flag"]
pub type MR7IF_R = crate::BitReader<MR7IF_A>;
impl MR7IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR7IF_A {
        match self.bits {
            false => MR7IF_A::NO,
            true => MR7IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == MR7IF_A::NO
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR7IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 8 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR8IF_A {
    #[doc = "0: No interrupt on match channel 8"]
    NOINTERRUPT = 0,
    #[doc = "1: Interrupt requirements met on match channel 8"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR8IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR8IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR8IF` reader - Match channel 8 interrupt flag"]
pub type MR8IF_R = crate::BitReader<MR8IF_A>;
impl MR8IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR8IF_A {
        match self.bits {
            false => MR8IF_A::NOINTERRUPT,
            true => MR8IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NOINTERRUPT`"]
    #[inline(always)]
    pub fn is_nointerrupt(&self) -> bool {
        *self == MR8IF_A::NOINTERRUPT
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR8IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 9 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR9IF_A {
    #[doc = "0: No interrupt on match channel 9"]
    NO = 0,
    #[doc = "1: Interrupt requirements met on match channel 9"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR9IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR9IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR9IF` reader - Match channel 9 interrupt flag"]
pub type MR9IF_R = crate::BitReader<MR9IF_A>;
impl MR9IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR9IF_A {
        match self.bits {
            false => MR9IF_A::NO,
            true => MR9IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == MR9IF_A::NO
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR9IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 10 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR10IF_A {
    #[doc = "0: No interrupt on match channel 10"]
    NO = 0,
    #[doc = "1: Interrupt requirements met on match channel 10"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR10IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR10IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR10IF` reader - Match channel 10 interrupt flag"]
pub type MR10IF_R = crate::BitReader<MR10IF_A>;
impl MR10IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR10IF_A {
        match self.bits {
            false => MR10IF_A::NO,
            true => MR10IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == MR10IF_A::NO
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR10IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 11 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR11IF_A {
    #[doc = "0: No interrupt on match channel 11"]
    NO = 0,
    #[doc = "1: Interrupt requirements met on match channel 11"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR11IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR11IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR11IF` reader - Match channel 11 interrupt flag"]
pub type MR11IF_R = crate::BitReader<MR11IF_A>;
impl MR11IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR11IF_A {
        match self.bits {
            false => MR11IF_A::NO,
            true => MR11IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == MR11IF_A::NO
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR11IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 12 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR12IF_A {
    #[doc = "0: No interrupt on match channel 12"]
    NOINTERRUPT = 0,
    #[doc = "1: Interrupt requirements met on match channel 12"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR12IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR12IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR12IF` reader - Match channel 12 interrupt flag"]
pub type MR12IF_R = crate::BitReader<MR12IF_A>;
impl MR12IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR12IF_A {
        match self.bits {
            false => MR12IF_A::NOINTERRUPT,
            true => MR12IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NOINTERRUPT`"]
    #[inline(always)]
    pub fn is_nointerrupt(&self) -> bool {
        *self == MR12IF_A::NOINTERRUPT
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR12IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 13 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR13IF_A {
    #[doc = "0: No interrupt on match channel 13"]
    NO = 0,
    #[doc = "1: Interrupt requirements met on match channel 13"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR13IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR13IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR13IF` reader - Match channel 13 interrupt flag"]
pub type MR13IF_R = crate::BitReader<MR13IF_A>;
impl MR13IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR13IF_A {
        match self.bits {
            false => MR13IF_A::NO,
            true => MR13IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == MR13IF_A::NO
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR13IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 14 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR14IF_A {
    #[doc = "0: No interrupt on match channel 14"]
    NO = 0,
    #[doc = "1: Interrupt requirements met on match channel 14"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR14IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR14IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR14IF` reader - Match channel 14 interrupt flag"]
pub type MR14IF_R = crate::BitReader<MR14IF_A>;
impl MR14IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR14IF_A {
        match self.bits {
            false => MR14IF_A::NO,
            true => MR14IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == MR14IF_A::NO
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR14IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 15 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR15IF_A {
    #[doc = "0: No interrupt on match channel 15"]
    NO = 0,
    #[doc = "1: Interrupt requirements met on match channel 15"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR15IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR15IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR15IF` reader - Match channel 15 interrupt flag"]
pub type MR15IF_R = crate::BitReader<MR15IF_A>;
impl MR15IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR15IF_A {
        match self.bits {
            false => MR15IF_A::NO,
            true => MR15IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == MR15IF_A::NO
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR15IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 16 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR16IF_A {
    #[doc = "0: No interrupt on match channel 16"]
    NOINTERRUPT = 0,
    #[doc = "1: Interrupt requirements met on match channel 16"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR16IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR16IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR16IF` reader - Match channel 16 interrupt flag"]
pub type MR16IF_R = crate::BitReader<MR16IF_A>;
impl MR16IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR16IF_A {
        match self.bits {
            false => MR16IF_A::NOINTERRUPT,
            true => MR16IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NOINTERRUPT`"]
    #[inline(always)]
    pub fn is_nointerrupt(&self) -> bool {
        *self == MR16IF_A::NOINTERRUPT
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR16IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 17 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR17IF_A {
    #[doc = "0: No interrupt on match channel 17"]
    NO = 0,
    #[doc = "1: Interrupt requirements met on match channel 17"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR17IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR17IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR17IF` reader - Match channel 17 interrupt flag"]
pub type MR17IF_R = crate::BitReader<MR17IF_A>;
impl MR17IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR17IF_A {
        match self.bits {
            false => MR17IF_A::NO,
            true => MR17IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == MR17IF_A::NO
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR17IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 18 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR18IF_A {
    #[doc = "0: No interrupt on match channel 18"]
    NO = 0,
    #[doc = "1: Interrupt requirements met on match channel 18"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR18IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR18IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR18IF` reader - Match channel 18 interrupt flag"]
pub type MR18IF_R = crate::BitReader<MR18IF_A>;
impl MR18IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR18IF_A {
        match self.bits {
            false => MR18IF_A::NO,
            true => MR18IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == MR18IF_A::NO
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR18IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 19 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR19IF_A {
    #[doc = "0: No interrupt on match channel 19"]
    NO = 0,
    #[doc = "1: Interrupt requirements met on match channel 19"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR19IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR19IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR19IF` reader - Match channel 19 interrupt flag"]
pub type MR19IF_R = crate::BitReader<MR19IF_A>;
impl MR19IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR19IF_A {
        match self.bits {
            false => MR19IF_A::NO,
            true => MR19IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == MR19IF_A::NO
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR19IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 20 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR20IF_A {
    #[doc = "0: No interrupt on match channel 20"]
    NO = 0,
    #[doc = "1: Interrupt requirements met on match channel 20"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR20IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR20IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR20IF` reader - Match channel 20 interrupt flag"]
pub type MR20IF_R = crate::BitReader<MR20IF_A>;
impl MR20IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR20IF_A {
        match self.bits {
            false => MR20IF_A::NO,
            true => MR20IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == MR20IF_A::NO
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR20IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 21 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR21IF_A {
    #[doc = "0: No interrupt on match channel 21"]
    NO = 0,
    #[doc = "1: Interrupt requirements met on match channel 21"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR21IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR21IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR21IF` reader - Match channel 21 interrupt flag"]
pub type MR21IF_R = crate::BitReader<MR21IF_A>;
impl MR21IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR21IF_A {
        match self.bits {
            false => MR21IF_A::NO,
            true => MR21IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == MR21IF_A::NO
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR21IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 22 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR22IF_A {
    #[doc = "0: No interrupt on match channel 22"]
    NO = 0,
    #[doc = "1: Interrupt requirements met on match channel 22"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR22IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR22IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR22IF` reader - Match channel 22 interrupt flag"]
pub type MR22IF_R = crate::BitReader<MR22IF_A>;
impl MR22IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR22IF_A {
        match self.bits {
            false => MR22IF_A::NO,
            true => MR22IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == MR22IF_A::NO
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR22IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Match channel 23 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR23IF_A {
    #[doc = "0: No interrupt on match channel 23"]
    NO = 0,
    #[doc = "1: Interrupt requirements met on match channel 23"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR23IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR23IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR23IF` reader - Match channel 23 interrupt flag"]
pub type MR23IF_R = crate::BitReader<MR23IF_A>;
impl MR23IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR23IF_A {
        match self.bits {
            false => MR23IF_A::NO,
            true => MR23IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == MR23IF_A::NO
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR23IF_A::METINTERRUPTREQUIREMENTS
    }
}
impl R {
    #[doc = "Bit 0 - Match channel 0 interrupt flag"]
    #[inline(always)]
    pub fn mr0if(&self) -> MR0IF_R {
        MR0IF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Match channel 1 interrupt flag"]
    #[inline(always)]
    pub fn mr1if(&self) -> MR1IF_R {
        MR1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Match channel 2 interrupt flag"]
    #[inline(always)]
    pub fn mr2if(&self) -> MR2IF_R {
        MR2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Match channel 3 interrupt flag"]
    #[inline(always)]
    pub fn mr3if(&self) -> MR3IF_R {
        MR3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Match channel 4 interrupt flag"]
    #[inline(always)]
    pub fn mr4if(&self) -> MR4IF_R {
        MR4IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Match channel 5 interrupt flag"]
    #[inline(always)]
    pub fn mr5if(&self) -> MR5IF_R {
        MR5IF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Match channel 6 interrupt flag"]
    #[inline(always)]
    pub fn mr6if(&self) -> MR6IF_R {
        MR6IF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Match channel 7 interrupt flag"]
    #[inline(always)]
    pub fn mr7if(&self) -> MR7IF_R {
        MR7IF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Match channel 8 interrupt flag"]
    #[inline(always)]
    pub fn mr8if(&self) -> MR8IF_R {
        MR8IF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Match channel 9 interrupt flag"]
    #[inline(always)]
    pub fn mr9if(&self) -> MR9IF_R {
        MR9IF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Match channel 10 interrupt flag"]
    #[inline(always)]
    pub fn mr10if(&self) -> MR10IF_R {
        MR10IF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Match channel 11 interrupt flag"]
    #[inline(always)]
    pub fn mr11if(&self) -> MR11IF_R {
        MR11IF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Match channel 12 interrupt flag"]
    #[inline(always)]
    pub fn mr12if(&self) -> MR12IF_R {
        MR12IF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Match channel 13 interrupt flag"]
    #[inline(always)]
    pub fn mr13if(&self) -> MR13IF_R {
        MR13IF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Match channel 14 interrupt flag"]
    #[inline(always)]
    pub fn mr14if(&self) -> MR14IF_R {
        MR14IF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Match channel 15 interrupt flag"]
    #[inline(always)]
    pub fn mr15if(&self) -> MR15IF_R {
        MR15IF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Match channel 16 interrupt flag"]
    #[inline(always)]
    pub fn mr16if(&self) -> MR16IF_R {
        MR16IF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Match channel 17 interrupt flag"]
    #[inline(always)]
    pub fn mr17if(&self) -> MR17IF_R {
        MR17IF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Match channel 18 interrupt flag"]
    #[inline(always)]
    pub fn mr18if(&self) -> MR18IF_R {
        MR18IF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Match channel 19 interrupt flag"]
    #[inline(always)]
    pub fn mr19if(&self) -> MR19IF_R {
        MR19IF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Match channel 20 interrupt flag"]
    #[inline(always)]
    pub fn mr20if(&self) -> MR20IF_R {
        MR20IF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Match channel 21 interrupt flag"]
    #[inline(always)]
    pub fn mr21if(&self) -> MR21IF_R {
        MR21IF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Match channel 22 interrupt flag"]
    #[inline(always)]
    pub fn mr22if(&self) -> MR22IF_R {
        MR22IF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Match channel 23 interrupt flag"]
    #[inline(always)]
    pub fn mr23if(&self) -> MR23IF_R {
        MR23IF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "Offset:0xA4 CT16Bn Raw Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
