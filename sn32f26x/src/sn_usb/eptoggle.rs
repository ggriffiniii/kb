#[doc = "Register `EPTOGGLE` reader"]
pub struct R(crate::R<EPTOGGLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPTOGGLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPTOGGLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPTOGGLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPTOGGLE` writer"]
pub struct W(crate::W<EPTOGGLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPTOGGLE_SPEC>;
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
impl From<crate::W<EPTOGGLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPTOGGLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Endpoint 1 data toggle bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDP1_DATA01_A {
    #[doc = "0: Clear EP1 toggle bit to DATA0"]
    DISABLE = 0,
    #[doc = "1: HW sets toggle bit automatically"]
    TOGGLE = 1,
}
impl From<ENDP1_DATA01_A> for bool {
    #[inline(always)]
    fn from(variant: ENDP1_DATA01_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDP1_DATA01` reader - Endpoint 1 data toggle bit"]
pub type ENDP1_DATA01_R = crate::BitReader<ENDP1_DATA01_A>;
impl ENDP1_DATA01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDP1_DATA01_A {
        match self.bits {
            false => ENDP1_DATA01_A::DISABLE,
            true => ENDP1_DATA01_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENDP1_DATA01_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == ENDP1_DATA01_A::TOGGLE
    }
}
#[doc = "Field `ENDP1_DATA01` writer - Endpoint 1 data toggle bit"]
pub type ENDP1_DATA01_W<'a> = crate::BitWriter<'a, u32, EPTOGGLE_SPEC, ENDP1_DATA01_A, 0>;
impl<'a> ENDP1_DATA01_W<'a> {
    #[doc = "Clear EP1 toggle bit to DATA0"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENDP1_DATA01_A::DISABLE)
    }
    #[doc = "HW sets toggle bit automatically"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(ENDP1_DATA01_A::TOGGLE)
    }
}
#[doc = "Endpoint 2 data toggle bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDP2_DATA01_A {
    #[doc = "0: Clear EP2 toggle bit to DATA0"]
    DISABLE = 0,
    #[doc = "1: HW sets toggle bit automatically"]
    TOGGLE = 1,
}
impl From<ENDP2_DATA01_A> for bool {
    #[inline(always)]
    fn from(variant: ENDP2_DATA01_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDP2_DATA01` reader - Endpoint 2 data toggle bit"]
pub type ENDP2_DATA01_R = crate::BitReader<ENDP2_DATA01_A>;
impl ENDP2_DATA01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDP2_DATA01_A {
        match self.bits {
            false => ENDP2_DATA01_A::DISABLE,
            true => ENDP2_DATA01_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENDP2_DATA01_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == ENDP2_DATA01_A::TOGGLE
    }
}
#[doc = "Field `ENDP2_DATA01` writer - Endpoint 2 data toggle bit"]
pub type ENDP2_DATA01_W<'a> = crate::BitWriter<'a, u32, EPTOGGLE_SPEC, ENDP2_DATA01_A, 1>;
impl<'a> ENDP2_DATA01_W<'a> {
    #[doc = "Clear EP2 toggle bit to DATA0"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENDP2_DATA01_A::DISABLE)
    }
    #[doc = "HW sets toggle bit automatically"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(ENDP2_DATA01_A::TOGGLE)
    }
}
#[doc = "Endpoint 3 data toggle bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDP3_DATA01_A {
    #[doc = "0: Clear EP3 toggle bit to DATA0"]
    DISABLE = 0,
    #[doc = "1: HW sets toggle bit automatically"]
    TOGGLE = 1,
}
impl From<ENDP3_DATA01_A> for bool {
    #[inline(always)]
    fn from(variant: ENDP3_DATA01_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDP3_DATA01` reader - Endpoint 3 data toggle bit"]
pub type ENDP3_DATA01_R = crate::BitReader<ENDP3_DATA01_A>;
impl ENDP3_DATA01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDP3_DATA01_A {
        match self.bits {
            false => ENDP3_DATA01_A::DISABLE,
            true => ENDP3_DATA01_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENDP3_DATA01_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == ENDP3_DATA01_A::TOGGLE
    }
}
#[doc = "Field `ENDP3_DATA01` writer - Endpoint 3 data toggle bit"]
pub type ENDP3_DATA01_W<'a> = crate::BitWriter<'a, u32, EPTOGGLE_SPEC, ENDP3_DATA01_A, 2>;
impl<'a> ENDP3_DATA01_W<'a> {
    #[doc = "Clear EP3 toggle bit to DATA0"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENDP3_DATA01_A::DISABLE)
    }
    #[doc = "HW sets toggle bit automatically"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(ENDP3_DATA01_A::TOGGLE)
    }
}
#[doc = "Endpoint 4 data toggle bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDP4_DATA01_A {
    #[doc = "0: Clear EP4 toggle bit to DATA0"]
    DISABLE = 0,
    #[doc = "1: HW sets toggle bit automatically"]
    TOGGLE = 1,
}
impl From<ENDP4_DATA01_A> for bool {
    #[inline(always)]
    fn from(variant: ENDP4_DATA01_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDP4_DATA01` reader - Endpoint 4 data toggle bit"]
pub type ENDP4_DATA01_R = crate::BitReader<ENDP4_DATA01_A>;
impl ENDP4_DATA01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDP4_DATA01_A {
        match self.bits {
            false => ENDP4_DATA01_A::DISABLE,
            true => ENDP4_DATA01_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENDP4_DATA01_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == ENDP4_DATA01_A::TOGGLE
    }
}
#[doc = "Field `ENDP4_DATA01` writer - Endpoint 4 data toggle bit"]
pub type ENDP4_DATA01_W<'a> = crate::BitWriter<'a, u32, EPTOGGLE_SPEC, ENDP4_DATA01_A, 3>;
impl<'a> ENDP4_DATA01_W<'a> {
    #[doc = "Clear EP4 toggle bit to DATA0"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENDP4_DATA01_A::DISABLE)
    }
    #[doc = "HW sets toggle bit automatically"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(ENDP4_DATA01_A::TOGGLE)
    }
}
impl R {
    #[doc = "Bit 0 - Endpoint 1 data toggle bit"]
    #[inline(always)]
    pub fn endp1_data01(&self) -> ENDP1_DATA01_R {
        ENDP1_DATA01_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint 2 data toggle bit"]
    #[inline(always)]
    pub fn endp2_data01(&self) -> ENDP2_DATA01_R {
        ENDP2_DATA01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint 3 data toggle bit"]
    #[inline(always)]
    pub fn endp3_data01(&self) -> ENDP3_DATA01_R {
        ENDP3_DATA01_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint 4 data toggle bit"]
    #[inline(always)]
    pub fn endp4_data01(&self) -> ENDP4_DATA01_R {
        ENDP4_DATA01_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint 1 data toggle bit"]
    #[inline(always)]
    pub fn endp1_data01(&mut self) -> ENDP1_DATA01_W {
        ENDP1_DATA01_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint 2 data toggle bit"]
    #[inline(always)]
    pub fn endp2_data01(&mut self) -> ENDP2_DATA01_W {
        ENDP2_DATA01_W::new(self)
    }
    #[doc = "Bit 2 - Endpoint 3 data toggle bit"]
    #[inline(always)]
    pub fn endp3_data01(&mut self) -> ENDP3_DATA01_W {
        ENDP3_DATA01_W::new(self)
    }
    #[doc = "Bit 3 - Endpoint 4 data toggle bit"]
    #[inline(always)]
    pub fn endp4_data01(&mut self) -> ENDP4_DATA01_W {
        ENDP4_DATA01_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x3C USB Endpoint Data Toggle Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eptoggle](index.html) module"]
pub struct EPTOGGLE_SPEC;
impl crate::RegisterSpec for EPTOGGLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eptoggle::R](R) reader structure"]
impl crate::Readable for EPTOGGLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eptoggle::W](W) writer structure"]
impl crate::Writable for EPTOGGLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPTOGGLE to value 0x0f"]
impl crate::Resettable for EPTOGGLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
