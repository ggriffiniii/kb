#[doc = "Register `LVDCTRL` reader"]
pub struct R(crate::R<LVDCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LVDCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LVDCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LVDCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LVDCTRL` writer"]
pub struct W(crate::W<LVDCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVDCTRL_SPEC>;
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
impl From<crate::W<LVDCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVDCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LVD reset level\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LVDRSTLVL_A {
    #[doc = "0: LVD reset threshold is 1.80V"]
    _1_80V = 0,
    #[doc = "2: LVD reset threshold is 2.40V"]
    _2_40V = 2,
    #[doc = "4: LVD reset threshold is 3.30V"]
    _3_30V = 4,
}
impl From<LVDRSTLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: LVDRSTLVL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LVDRSTLVL` reader - LVD reset level"]
pub type LVDRSTLVL_R = crate::FieldReader<u8, LVDRSTLVL_A>;
impl LVDRSTLVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LVDRSTLVL_A> {
        match self.bits {
            0 => Some(LVDRSTLVL_A::_1_80V),
            2 => Some(LVDRSTLVL_A::_2_40V),
            4 => Some(LVDRSTLVL_A::_3_30V),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1_80V`"]
    #[inline(always)]
    pub fn is_1_80v(&self) -> bool {
        *self == LVDRSTLVL_A::_1_80V
    }
    #[doc = "Checks if the value of the field is `_2_40V`"]
    #[inline(always)]
    pub fn is_2_40v(&self) -> bool {
        *self == LVDRSTLVL_A::_2_40V
    }
    #[doc = "Checks if the value of the field is `_3_30V`"]
    #[inline(always)]
    pub fn is_3_30v(&self) -> bool {
        *self == LVDRSTLVL_A::_3_30V
    }
}
#[doc = "Field `LVDRSTLVL` writer - LVD reset level"]
pub type LVDRSTLVL_W<'a> = crate::FieldWriter<'a, u32, LVDCTRL_SPEC, u8, LVDRSTLVL_A, 3, 0>;
impl<'a> LVDRSTLVL_W<'a> {
    #[doc = "LVD reset threshold is 1.80V"]
    #[inline(always)]
    pub fn _1_80v(self) -> &'a mut W {
        self.variant(LVDRSTLVL_A::_1_80V)
    }
    #[doc = "LVD reset threshold is 2.40V"]
    #[inline(always)]
    pub fn _2_40v(self) -> &'a mut W {
        self.variant(LVDRSTLVL_A::_2_40V)
    }
    #[doc = "LVD reset threshold is 3.30V"]
    #[inline(always)]
    pub fn _3_30v(self) -> &'a mut W {
        self.variant(LVDRSTLVL_A::_3_30V)
    }
}
#[doc = "LVD interrupt level\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LVDINTLVL_A {
    #[doc = "1: LVD interrupt threshold is 2.40V"]
    _2_40V = 1,
    #[doc = "2: LVD interrupt threshold is 3.30V"]
    _3_30V = 2,
}
impl From<LVDINTLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: LVDINTLVL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LVDINTLVL` reader - LVD interrupt level"]
pub type LVDINTLVL_R = crate::FieldReader<u8, LVDINTLVL_A>;
impl LVDINTLVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LVDINTLVL_A> {
        match self.bits {
            1 => Some(LVDINTLVL_A::_2_40V),
            2 => Some(LVDINTLVL_A::_3_30V),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_2_40V`"]
    #[inline(always)]
    pub fn is_2_40v(&self) -> bool {
        *self == LVDINTLVL_A::_2_40V
    }
    #[doc = "Checks if the value of the field is `_3_30V`"]
    #[inline(always)]
    pub fn is_3_30v(&self) -> bool {
        *self == LVDINTLVL_A::_3_30V
    }
}
#[doc = "Field `LVDINTLVL` writer - LVD interrupt level"]
pub type LVDINTLVL_W<'a> = crate::FieldWriter<'a, u32, LVDCTRL_SPEC, u8, LVDINTLVL_A, 2, 5>;
impl<'a> LVDINTLVL_W<'a> {
    #[doc = "LVD interrupt threshold is 2.40V"]
    #[inline(always)]
    pub fn _2_40v(self) -> &'a mut W {
        self.variant(LVDINTLVL_A::_2_40V)
    }
    #[doc = "LVD interrupt threshold is 3.30V"]
    #[inline(always)]
    pub fn _3_30v(self) -> &'a mut W {
        self.variant(LVDINTLVL_A::_3_30V)
    }
}
#[doc = "LVD Reset enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDRSTEN_A {
    #[doc = "0: Disable LVD reset, set LVD flag"]
    DIABLE = 0,
    #[doc = "1: Enable LVD reset"]
    ENABLE = 1,
}
impl From<LVDRSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: LVDRSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDRSTEN` reader - LVD Reset enable"]
pub type LVDRSTEN_R = crate::BitReader<LVDRSTEN_A>;
impl LVDRSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDRSTEN_A {
        match self.bits {
            false => LVDRSTEN_A::DIABLE,
            true => LVDRSTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DIABLE`"]
    #[inline(always)]
    pub fn is_diable(&self) -> bool {
        *self == LVDRSTEN_A::DIABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LVDRSTEN_A::ENABLE
    }
}
#[doc = "Field `LVDRSTEN` writer - LVD Reset enable"]
pub type LVDRSTEN_W<'a> = crate::BitWriter<'a, u32, LVDCTRL_SPEC, LVDRSTEN_A, 14>;
impl<'a> LVDRSTEN_W<'a> {
    #[doc = "Disable LVD reset, set LVD flag"]
    #[inline(always)]
    pub fn diable(self) -> &'a mut W {
        self.variant(LVDRSTEN_A::DIABLE)
    }
    #[doc = "Enable LVD reset"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LVDRSTEN_A::ENABLE)
    }
}
#[doc = "LVD enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDEN_A {
    #[doc = "0: Disable LVD"]
    DIABLE = 0,
    #[doc = "1: Enable LVD"]
    ENABLE = 1,
}
impl From<LVDEN_A> for bool {
    #[inline(always)]
    fn from(variant: LVDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDEN` reader - LVD enable"]
pub type LVDEN_R = crate::BitReader<LVDEN_A>;
impl LVDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDEN_A {
        match self.bits {
            false => LVDEN_A::DIABLE,
            true => LVDEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DIABLE`"]
    #[inline(always)]
    pub fn is_diable(&self) -> bool {
        *self == LVDEN_A::DIABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LVDEN_A::ENABLE
    }
}
#[doc = "Field `LVDEN` writer - LVD enable"]
pub type LVDEN_W<'a> = crate::BitWriter<'a, u32, LVDCTRL_SPEC, LVDEN_A, 15>;
impl<'a> LVDEN_W<'a> {
    #[doc = "Disable LVD"]
    #[inline(always)]
    pub fn diable(self) -> &'a mut W {
        self.variant(LVDEN_A::DIABLE)
    }
    #[doc = "Enable LVD"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LVDEN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:2 - LVD reset level"]
    #[inline(always)]
    pub fn lvdrstlvl(&self) -> LVDRSTLVL_R {
        LVDRSTLVL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 5:6 - LVD interrupt level"]
    #[inline(always)]
    pub fn lvdintlvl(&self) -> LVDINTLVL_R {
        LVDINTLVL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 14 - LVD Reset enable"]
    #[inline(always)]
    pub fn lvdrsten(&self) -> LVDRSTEN_R {
        LVDRSTEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LVD enable"]
    #[inline(always)]
    pub fn lvden(&self) -> LVDEN_R {
        LVDEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - LVD reset level"]
    #[inline(always)]
    pub fn lvdrstlvl(&mut self) -> LVDRSTLVL_W {
        LVDRSTLVL_W::new(self)
    }
    #[doc = "Bits 5:6 - LVD interrupt level"]
    #[inline(always)]
    pub fn lvdintlvl(&mut self) -> LVDINTLVL_W {
        LVDINTLVL_W::new(self)
    }
    #[doc = "Bit 14 - LVD Reset enable"]
    #[inline(always)]
    pub fn lvdrsten(&mut self) -> LVDRSTEN_W {
        LVDRSTEN_W::new(self)
    }
    #[doc = "Bit 15 - LVD enable"]
    #[inline(always)]
    pub fn lvden(&mut self) -> LVDEN_W {
        LVDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x18 LVD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvdctrl](index.html) module"]
pub struct LVDCTRL_SPEC;
impl crate::RegisterSpec for LVDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lvdctrl::R](R) reader structure"]
impl crate::Readable for LVDCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lvdctrl::W](W) writer structure"]
impl crate::Writable for LVDCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LVDCTRL to value 0x42"]
impl crate::Resettable for LVDCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x42
    }
}
