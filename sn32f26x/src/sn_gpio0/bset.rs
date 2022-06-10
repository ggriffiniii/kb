#[doc = "Register `BSET` writer"]
pub struct W(crate::W<BSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSET_SPEC>;
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
impl From<crate::W<BSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set Pn.0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSET0_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Set Pn.0 to 1"]
    SET = 1,
}
impl From<BSET0_AW> for bool {
    #[inline(always)]
    fn from(variant: BSET0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSET0` writer - Set Pn.0"]
pub type BSET0_W<'a> = crate::BitWriter<'a, u32, BSET_SPEC, BSET0_AW, 0>;
impl<'a> BSET0_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BSET0_AW::NOEFFECT)
    }
    #[doc = "Set Pn.0 to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSET0_AW::SET)
    }
}
#[doc = "Set Pn.1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSET1_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Set Pn.1 to 1"]
    SET = 1,
}
impl From<BSET1_AW> for bool {
    #[inline(always)]
    fn from(variant: BSET1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSET1` writer - Set Pn.1"]
pub type BSET1_W<'a> = crate::BitWriter<'a, u32, BSET_SPEC, BSET1_AW, 1>;
impl<'a> BSET1_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BSET1_AW::NOEFFECT)
    }
    #[doc = "Set Pn.1 to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSET1_AW::SET)
    }
}
#[doc = "Set Pn.2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSET2_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Set Pn.2 to 1"]
    SET = 1,
}
impl From<BSET2_AW> for bool {
    #[inline(always)]
    fn from(variant: BSET2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSET2` writer - Set Pn.2"]
pub type BSET2_W<'a> = crate::BitWriter<'a, u32, BSET_SPEC, BSET2_AW, 2>;
impl<'a> BSET2_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BSET2_AW::NOEFFECT)
    }
    #[doc = "Set Pn.2 to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSET2_AW::SET)
    }
}
#[doc = "Set Pn.3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSET3_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Set Pn.3 to 1"]
    SET = 1,
}
impl From<BSET3_AW> for bool {
    #[inline(always)]
    fn from(variant: BSET3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSET3` writer - Set Pn.3"]
pub type BSET3_W<'a> = crate::BitWriter<'a, u32, BSET_SPEC, BSET3_AW, 3>;
impl<'a> BSET3_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BSET3_AW::NOEFFECT)
    }
    #[doc = "Set Pn.3 to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSET3_AW::SET)
    }
}
#[doc = "Set Pn.4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSET4_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Set Pn.4 to 1"]
    SET = 1,
}
impl From<BSET4_AW> for bool {
    #[inline(always)]
    fn from(variant: BSET4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSET4` writer - Set Pn.4"]
pub type BSET4_W<'a> = crate::BitWriter<'a, u32, BSET_SPEC, BSET4_AW, 4>;
impl<'a> BSET4_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BSET4_AW::NOEFFECT)
    }
    #[doc = "Set Pn.4 to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSET4_AW::SET)
    }
}
#[doc = "Set Pn.5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSET5_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Set Pn.5 to 1"]
    SET = 1,
}
impl From<BSET5_AW> for bool {
    #[inline(always)]
    fn from(variant: BSET5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSET5` writer - Set Pn.5"]
pub type BSET5_W<'a> = crate::BitWriter<'a, u32, BSET_SPEC, BSET5_AW, 5>;
impl<'a> BSET5_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BSET5_AW::NOEFFECT)
    }
    #[doc = "Set Pn.5 to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSET5_AW::SET)
    }
}
#[doc = "Set Pn.6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSET6_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Set Pn.6 to 1"]
    SET = 1,
}
impl From<BSET6_AW> for bool {
    #[inline(always)]
    fn from(variant: BSET6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSET6` writer - Set Pn.6"]
pub type BSET6_W<'a> = crate::BitWriter<'a, u32, BSET_SPEC, BSET6_AW, 6>;
impl<'a> BSET6_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BSET6_AW::NOEFFECT)
    }
    #[doc = "Set Pn.6 to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSET6_AW::SET)
    }
}
#[doc = "Set Pn.7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSET7_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Set Pn.7 to 1"]
    SET = 1,
}
impl From<BSET7_AW> for bool {
    #[inline(always)]
    fn from(variant: BSET7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSET7` writer - Set Pn.7"]
pub type BSET7_W<'a> = crate::BitWriter<'a, u32, BSET_SPEC, BSET7_AW, 7>;
impl<'a> BSET7_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BSET7_AW::NOEFFECT)
    }
    #[doc = "Set Pn.7 to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSET7_AW::SET)
    }
}
#[doc = "Set Pn.8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSET8_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Set Pn.8 to 1"]
    SET = 1,
}
impl From<BSET8_AW> for bool {
    #[inline(always)]
    fn from(variant: BSET8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSET8` writer - Set Pn.8"]
pub type BSET8_W<'a> = crate::BitWriter<'a, u32, BSET_SPEC, BSET8_AW, 8>;
impl<'a> BSET8_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BSET8_AW::NOEFFECT)
    }
    #[doc = "Set Pn.8 to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSET8_AW::SET)
    }
}
#[doc = "Set Pn.9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSET9_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Set Pn.9 to 1"]
    SET = 1,
}
impl From<BSET9_AW> for bool {
    #[inline(always)]
    fn from(variant: BSET9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSET9` writer - Set Pn.9"]
pub type BSET9_W<'a> = crate::BitWriter<'a, u32, BSET_SPEC, BSET9_AW, 9>;
impl<'a> BSET9_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BSET9_AW::NOEFFECT)
    }
    #[doc = "Set Pn.9 to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSET9_AW::SET)
    }
}
#[doc = "Set Pn.10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSET10_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Set Pn.10 to 1"]
    SET = 1,
}
impl From<BSET10_AW> for bool {
    #[inline(always)]
    fn from(variant: BSET10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSET10` writer - Set Pn.10"]
pub type BSET10_W<'a> = crate::BitWriter<'a, u32, BSET_SPEC, BSET10_AW, 10>;
impl<'a> BSET10_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BSET10_AW::NOEFFECT)
    }
    #[doc = "Set Pn.10 to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSET10_AW::SET)
    }
}
#[doc = "Set Pn.11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSET11_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Set Pn.11 to 1"]
    SET = 1,
}
impl From<BSET11_AW> for bool {
    #[inline(always)]
    fn from(variant: BSET11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSET11` writer - Set Pn.11"]
pub type BSET11_W<'a> = crate::BitWriter<'a, u32, BSET_SPEC, BSET11_AW, 11>;
impl<'a> BSET11_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BSET11_AW::NOEFFECT)
    }
    #[doc = "Set Pn.11 to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSET11_AW::SET)
    }
}
#[doc = "Set Pn.12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSET12_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Set Pn.12 to 1"]
    SET = 1,
}
impl From<BSET12_AW> for bool {
    #[inline(always)]
    fn from(variant: BSET12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSET12` writer - Set Pn.12"]
pub type BSET12_W<'a> = crate::BitWriter<'a, u32, BSET_SPEC, BSET12_AW, 12>;
impl<'a> BSET12_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BSET12_AW::NOEFFECT)
    }
    #[doc = "Set Pn.12 to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSET12_AW::SET)
    }
}
#[doc = "Set Pn.13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSET13_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Set Pn.13 to 1"]
    SET = 1,
}
impl From<BSET13_AW> for bool {
    #[inline(always)]
    fn from(variant: BSET13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSET13` writer - Set Pn.13"]
pub type BSET13_W<'a> = crate::BitWriter<'a, u32, BSET_SPEC, BSET13_AW, 13>;
impl<'a> BSET13_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BSET13_AW::NOEFFECT)
    }
    #[doc = "Set Pn.13 to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSET13_AW::SET)
    }
}
#[doc = "Set Pn.14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSET14_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Set Pn.14 to 1"]
    SET = 1,
}
impl From<BSET14_AW> for bool {
    #[inline(always)]
    fn from(variant: BSET14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSET14` writer - Set Pn.14"]
pub type BSET14_W<'a> = crate::BitWriter<'a, u32, BSET_SPEC, BSET14_AW, 14>;
impl<'a> BSET14_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BSET14_AW::NOEFFECT)
    }
    #[doc = "Set Pn.14 to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSET14_AW::SET)
    }
}
#[doc = "Set Pn.15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSET15_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Set Pn.15 to 1"]
    SET = 1,
}
impl From<BSET15_AW> for bool {
    #[inline(always)]
    fn from(variant: BSET15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSET15` writer - Set Pn.15"]
pub type BSET15_W<'a> = crate::BitWriter<'a, u32, BSET_SPEC, BSET15_AW, 15>;
impl<'a> BSET15_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BSET15_AW::NOEFFECT)
    }
    #[doc = "Set Pn.15 to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSET15_AW::SET)
    }
}
impl W {
    #[doc = "Bit 0 - Set Pn.0"]
    #[inline(always)]
    pub fn bset0(&mut self) -> BSET0_W {
        BSET0_W::new(self)
    }
    #[doc = "Bit 1 - Set Pn.1"]
    #[inline(always)]
    pub fn bset1(&mut self) -> BSET1_W {
        BSET1_W::new(self)
    }
    #[doc = "Bit 2 - Set Pn.2"]
    #[inline(always)]
    pub fn bset2(&mut self) -> BSET2_W {
        BSET2_W::new(self)
    }
    #[doc = "Bit 3 - Set Pn.3"]
    #[inline(always)]
    pub fn bset3(&mut self) -> BSET3_W {
        BSET3_W::new(self)
    }
    #[doc = "Bit 4 - Set Pn.4"]
    #[inline(always)]
    pub fn bset4(&mut self) -> BSET4_W {
        BSET4_W::new(self)
    }
    #[doc = "Bit 5 - Set Pn.5"]
    #[inline(always)]
    pub fn bset5(&mut self) -> BSET5_W {
        BSET5_W::new(self)
    }
    #[doc = "Bit 6 - Set Pn.6"]
    #[inline(always)]
    pub fn bset6(&mut self) -> BSET6_W {
        BSET6_W::new(self)
    }
    #[doc = "Bit 7 - Set Pn.7"]
    #[inline(always)]
    pub fn bset7(&mut self) -> BSET7_W {
        BSET7_W::new(self)
    }
    #[doc = "Bit 8 - Set Pn.8"]
    #[inline(always)]
    pub fn bset8(&mut self) -> BSET8_W {
        BSET8_W::new(self)
    }
    #[doc = "Bit 9 - Set Pn.9"]
    #[inline(always)]
    pub fn bset9(&mut self) -> BSET9_W {
        BSET9_W::new(self)
    }
    #[doc = "Bit 10 - Set Pn.10"]
    #[inline(always)]
    pub fn bset10(&mut self) -> BSET10_W {
        BSET10_W::new(self)
    }
    #[doc = "Bit 11 - Set Pn.11"]
    #[inline(always)]
    pub fn bset11(&mut self) -> BSET11_W {
        BSET11_W::new(self)
    }
    #[doc = "Bit 12 - Set Pn.12"]
    #[inline(always)]
    pub fn bset12(&mut self) -> BSET12_W {
        BSET12_W::new(self)
    }
    #[doc = "Bit 13 - Set Pn.13"]
    #[inline(always)]
    pub fn bset13(&mut self) -> BSET13_W {
        BSET13_W::new(self)
    }
    #[doc = "Bit 14 - Set Pn.14"]
    #[inline(always)]
    pub fn bset14(&mut self) -> BSET14_W {
        BSET14_W::new(self)
    }
    #[doc = "Bit 15 - Set Pn.15"]
    #[inline(always)]
    pub fn bset15(&mut self) -> BSET15_W {
        BSET15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x24 GPIO Port n Bits Set Operation Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bset](index.html) module"]
pub struct BSET_SPEC;
impl crate::RegisterSpec for BSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [bset::W](W) writer structure"]
impl crate::Writable for BSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BSET to value 0"]
impl crate::Resettable for BSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
