#[doc = "Register `IC` writer"]
pub struct W(crate::W<IC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_SPEC>;
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
impl From<crate::W<IC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "MR0IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR0IF"]
    CLEAR = 1,
}
impl From<MR0IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR0IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0IC` writer - MR0IF clear bit"]
pub type MR0IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR0IC_AW, 0>;
impl<'a> MR0IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR0IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR0IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR0IC_AW::CLEAR)
    }
}
#[doc = "MR1IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR1IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR1IF"]
    CLEAR = 1,
}
impl From<MR1IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR1IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR1IC` writer - MR1IF clear bit"]
pub type MR1IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR1IC_AW, 1>;
impl<'a> MR1IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR1IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR1IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR1IC_AW::CLEAR)
    }
}
#[doc = "MR2IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR2IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR2IF"]
    CLEAR = 1,
}
impl From<MR2IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR2IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR2IC` writer - MR2IF clear bit"]
pub type MR2IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR2IC_AW, 2>;
impl<'a> MR2IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR2IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR2IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR2IC_AW::CLEAR)
    }
}
#[doc = "MR3IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR3IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR3IF"]
    CLEAR = 1,
}
impl From<MR3IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR3IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR3IC` writer - MR3IF clear bit"]
pub type MR3IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR3IC_AW, 3>;
impl<'a> MR3IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR3IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR3IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR3IC_AW::CLEAR)
    }
}
#[doc = "MR4IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR4IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR4IF"]
    CLEAR = 1,
}
impl From<MR4IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR4IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR4IC` writer - MR4IF clear bit"]
pub type MR4IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR4IC_AW, 4>;
impl<'a> MR4IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR4IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR4IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR4IC_AW::CLEAR)
    }
}
#[doc = "MR5IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR5IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR5IF"]
    CLEAR = 1,
}
impl From<MR5IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR5IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR5IC` writer - MR5IF clear bit"]
pub type MR5IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR5IC_AW, 5>;
impl<'a> MR5IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR5IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR5IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR5IC_AW::CLEAR)
    }
}
#[doc = "MR6IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR6IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR6IF"]
    CLEAR = 1,
}
impl From<MR6IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR6IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR6IC` writer - MR6IF clear bit"]
pub type MR6IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR6IC_AW, 6>;
impl<'a> MR6IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR6IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR6IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR6IC_AW::CLEAR)
    }
}
#[doc = "MR7IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR7IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR7IF"]
    CLEAR = 1,
}
impl From<MR7IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR7IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR7IC` writer - MR7IF clear bit"]
pub type MR7IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR7IC_AW, 7>;
impl<'a> MR7IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR7IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR7IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR7IC_AW::CLEAR)
    }
}
#[doc = "MR8IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR8IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR8IF"]
    CLEAR = 1,
}
impl From<MR8IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR8IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR8IC` writer - MR8IF clear bit"]
pub type MR8IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR8IC_AW, 8>;
impl<'a> MR8IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR8IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR8IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR8IC_AW::CLEAR)
    }
}
#[doc = "MR9IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR9IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR9IF"]
    CLEAR = 1,
}
impl From<MR9IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR9IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR9IC` writer - MR9IF clear bit"]
pub type MR9IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR9IC_AW, 9>;
impl<'a> MR9IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR9IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR9IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR9IC_AW::CLEAR)
    }
}
#[doc = "MR10IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR10IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR10IF"]
    CLEAR = 1,
}
impl From<MR10IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR10IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR10IC` writer - MR10IF clear bit"]
pub type MR10IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR10IC_AW, 10>;
impl<'a> MR10IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR10IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR10IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR10IC_AW::CLEAR)
    }
}
#[doc = "MR11IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR11IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR11IF"]
    CLEAR = 1,
}
impl From<MR11IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR11IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR11IC` writer - MR11IF clear bit"]
pub type MR11IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR11IC_AW, 11>;
impl<'a> MR11IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR11IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR11IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR11IC_AW::CLEAR)
    }
}
#[doc = "MR12IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR12IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR12IF"]
    CLEAR = 1,
}
impl From<MR12IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR12IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR12IC` writer - MR12IF clear bit"]
pub type MR12IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR12IC_AW, 12>;
impl<'a> MR12IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR12IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR12IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR12IC_AW::CLEAR)
    }
}
#[doc = "MR13IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR13IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR13IF"]
    CLEAR = 1,
}
impl From<MR13IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR13IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR13IC` writer - MR13IF clear bit"]
pub type MR13IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR13IC_AW, 13>;
impl<'a> MR13IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR13IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR13IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR13IC_AW::CLEAR)
    }
}
#[doc = "MR14IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR14IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR14IF"]
    CLEAR = 1,
}
impl From<MR14IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR14IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR14IC` writer - MR14IF clear bit"]
pub type MR14IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR14IC_AW, 14>;
impl<'a> MR14IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR14IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR14IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR14IC_AW::CLEAR)
    }
}
#[doc = "MR15IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR15IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR15IF"]
    CLEAR = 1,
}
impl From<MR15IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR15IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR15IC` writer - MR15IF clear bit"]
pub type MR15IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR15IC_AW, 15>;
impl<'a> MR15IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR15IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR15IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR15IC_AW::CLEAR)
    }
}
#[doc = "MR16IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR16IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR16IF"]
    CLEAR = 1,
}
impl From<MR16IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR16IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR16IC` writer - MR16IF clear bit"]
pub type MR16IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR16IC_AW, 16>;
impl<'a> MR16IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR16IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR16IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR16IC_AW::CLEAR)
    }
}
#[doc = "MR17IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR17IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR17IF"]
    CLEAR = 1,
}
impl From<MR17IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR17IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR17IC` writer - MR17IF clear bit"]
pub type MR17IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR17IC_AW, 17>;
impl<'a> MR17IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR17IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR17IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR17IC_AW::CLEAR)
    }
}
#[doc = "MR18IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR18IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR18IF"]
    CLEAR = 1,
}
impl From<MR18IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR18IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR18IC` writer - MR18IF clear bit"]
pub type MR18IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR18IC_AW, 18>;
impl<'a> MR18IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR18IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR18IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR18IC_AW::CLEAR)
    }
}
#[doc = "MR19IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR19IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR19IF"]
    CLEAR = 1,
}
impl From<MR19IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR19IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR19IC` writer - MR19IF clear bit"]
pub type MR19IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR19IC_AW, 19>;
impl<'a> MR19IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR19IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR19IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR19IC_AW::CLEAR)
    }
}
#[doc = "MR20IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR20IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR20IF"]
    CLEAR = 1,
}
impl From<MR20IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR20IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR20IC` writer - MR20IF clear bit"]
pub type MR20IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR20IC_AW, 20>;
impl<'a> MR20IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR20IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR20IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR20IC_AW::CLEAR)
    }
}
#[doc = "MR21IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR21IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR21IF"]
    CLEAR = 1,
}
impl From<MR21IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR21IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR21IC` writer - MR21IF clear bit"]
pub type MR21IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR21IC_AW, 21>;
impl<'a> MR21IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR21IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR21IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR21IC_AW::CLEAR)
    }
}
#[doc = "MR22IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR22IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR22IF"]
    CLEAR = 1,
}
impl From<MR22IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR22IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR22IC` writer - MR22IF clear bit"]
pub type MR22IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR22IC_AW, 22>;
impl<'a> MR22IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR22IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR22IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR22IC_AW::CLEAR)
    }
}
#[doc = "MR23IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR23IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR23IF"]
    CLEAR = 1,
}
impl From<MR23IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR23IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR23IC` writer - MR23IF clear bit"]
pub type MR23IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR23IC_AW, 23>;
impl<'a> MR23IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR23IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR23IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR23IC_AW::CLEAR)
    }
}
impl W {
    #[doc = "Bit 0 - MR0IF clear bit"]
    #[inline(always)]
    pub fn mr0ic(&mut self) -> MR0IC_W {
        MR0IC_W::new(self)
    }
    #[doc = "Bit 1 - MR1IF clear bit"]
    #[inline(always)]
    pub fn mr1ic(&mut self) -> MR1IC_W {
        MR1IC_W::new(self)
    }
    #[doc = "Bit 2 - MR2IF clear bit"]
    #[inline(always)]
    pub fn mr2ic(&mut self) -> MR2IC_W {
        MR2IC_W::new(self)
    }
    #[doc = "Bit 3 - MR3IF clear bit"]
    #[inline(always)]
    pub fn mr3ic(&mut self) -> MR3IC_W {
        MR3IC_W::new(self)
    }
    #[doc = "Bit 4 - MR4IF clear bit"]
    #[inline(always)]
    pub fn mr4ic(&mut self) -> MR4IC_W {
        MR4IC_W::new(self)
    }
    #[doc = "Bit 5 - MR5IF clear bit"]
    #[inline(always)]
    pub fn mr5ic(&mut self) -> MR5IC_W {
        MR5IC_W::new(self)
    }
    #[doc = "Bit 6 - MR6IF clear bit"]
    #[inline(always)]
    pub fn mr6ic(&mut self) -> MR6IC_W {
        MR6IC_W::new(self)
    }
    #[doc = "Bit 7 - MR7IF clear bit"]
    #[inline(always)]
    pub fn mr7ic(&mut self) -> MR7IC_W {
        MR7IC_W::new(self)
    }
    #[doc = "Bit 8 - MR8IF clear bit"]
    #[inline(always)]
    pub fn mr8ic(&mut self) -> MR8IC_W {
        MR8IC_W::new(self)
    }
    #[doc = "Bit 9 - MR9IF clear bit"]
    #[inline(always)]
    pub fn mr9ic(&mut self) -> MR9IC_W {
        MR9IC_W::new(self)
    }
    #[doc = "Bit 10 - MR10IF clear bit"]
    #[inline(always)]
    pub fn mr10ic(&mut self) -> MR10IC_W {
        MR10IC_W::new(self)
    }
    #[doc = "Bit 11 - MR11IF clear bit"]
    #[inline(always)]
    pub fn mr11ic(&mut self) -> MR11IC_W {
        MR11IC_W::new(self)
    }
    #[doc = "Bit 12 - MR12IF clear bit"]
    #[inline(always)]
    pub fn mr12ic(&mut self) -> MR12IC_W {
        MR12IC_W::new(self)
    }
    #[doc = "Bit 13 - MR13IF clear bit"]
    #[inline(always)]
    pub fn mr13ic(&mut self) -> MR13IC_W {
        MR13IC_W::new(self)
    }
    #[doc = "Bit 14 - MR14IF clear bit"]
    #[inline(always)]
    pub fn mr14ic(&mut self) -> MR14IC_W {
        MR14IC_W::new(self)
    }
    #[doc = "Bit 15 - MR15IF clear bit"]
    #[inline(always)]
    pub fn mr15ic(&mut self) -> MR15IC_W {
        MR15IC_W::new(self)
    }
    #[doc = "Bit 16 - MR16IF clear bit"]
    #[inline(always)]
    pub fn mr16ic(&mut self) -> MR16IC_W {
        MR16IC_W::new(self)
    }
    #[doc = "Bit 17 - MR17IF clear bit"]
    #[inline(always)]
    pub fn mr17ic(&mut self) -> MR17IC_W {
        MR17IC_W::new(self)
    }
    #[doc = "Bit 18 - MR18IF clear bit"]
    #[inline(always)]
    pub fn mr18ic(&mut self) -> MR18IC_W {
        MR18IC_W::new(self)
    }
    #[doc = "Bit 19 - MR19IF clear bit"]
    #[inline(always)]
    pub fn mr19ic(&mut self) -> MR19IC_W {
        MR19IC_W::new(self)
    }
    #[doc = "Bit 20 - MR20IF clear bit"]
    #[inline(always)]
    pub fn mr20ic(&mut self) -> MR20IC_W {
        MR20IC_W::new(self)
    }
    #[doc = "Bit 21 - MR21IF clear bit"]
    #[inline(always)]
    pub fn mr21ic(&mut self) -> MR21IC_W {
        MR21IC_W::new(self)
    }
    #[doc = "Bit 22 - MR22IF clear bit"]
    #[inline(always)]
    pub fn mr22ic(&mut self) -> MR22IC_W {
        MR22IC_W::new(self)
    }
    #[doc = "Bit 23 - MR23IF clear bit"]
    #[inline(always)]
    pub fn mr23ic(&mut self) -> MR23IC_W {
        MR23IC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0xA8 CT16Bn Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic](index.html) module"]
pub struct IC_SPEC;
impl crate::RegisterSpec for IC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ic::W](W) writer structure"]
impl crate::Writable for IC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC to value 0"]
impl crate::Resettable for IC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
