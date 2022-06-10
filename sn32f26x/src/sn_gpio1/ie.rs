#[doc = "Register `IE` reader"]
pub struct R(crate::R<IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IE` writer"]
pub struct W(crate::W<IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IE_SPEC>;
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
impl From<crate::W<IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Interrupt on Pn.0 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE0_A {
    #[doc = "0: Disable interrupt on Pn.0"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt on Pn.0"]
    ENABLE = 1,
}
impl From<IE0_A> for bool {
    #[inline(always)]
    fn from(variant: IE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IE0` reader - Interrupt on Pn.0 enable"]
pub type IE0_R = crate::BitReader<IE0_A>;
impl IE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE0_A {
        match self.bits {
            false => IE0_A::DISABLE,
            true => IE0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IE0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IE0_A::ENABLE
    }
}
#[doc = "Field `IE0` writer - Interrupt on Pn.0 enable"]
pub type IE0_W<'a> = crate::BitWriter<'a, u32, IE_SPEC, IE0_A, 0>;
impl<'a> IE0_W<'a> {
    #[doc = "Disable interrupt on Pn.0"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IE0_A::DISABLE)
    }
    #[doc = "Enable interrupt on Pn.0"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IE0_A::ENABLE)
    }
}
#[doc = "Interrupt on Pn.1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE1_A {
    #[doc = "0: Disable interrupt on Pn.1"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt on Pn.1"]
    ENABLE = 1,
}
impl From<IE1_A> for bool {
    #[inline(always)]
    fn from(variant: IE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IE1` reader - Interrupt on Pn.1 enable"]
pub type IE1_R = crate::BitReader<IE1_A>;
impl IE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE1_A {
        match self.bits {
            false => IE1_A::DISABLE,
            true => IE1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IE1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IE1_A::ENABLE
    }
}
#[doc = "Field `IE1` writer - Interrupt on Pn.1 enable"]
pub type IE1_W<'a> = crate::BitWriter<'a, u32, IE_SPEC, IE1_A, 1>;
impl<'a> IE1_W<'a> {
    #[doc = "Disable interrupt on Pn.1"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IE1_A::DISABLE)
    }
    #[doc = "Enable interrupt on Pn.1"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IE1_A::ENABLE)
    }
}
#[doc = "Interrupt on Pn.2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE2_A {
    #[doc = "0: Disable interrupt on Pn.2"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt on Pn.2"]
    ENABLE = 1,
}
impl From<IE2_A> for bool {
    #[inline(always)]
    fn from(variant: IE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IE2` reader - Interrupt on Pn.2 enable"]
pub type IE2_R = crate::BitReader<IE2_A>;
impl IE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE2_A {
        match self.bits {
            false => IE2_A::DISABLE,
            true => IE2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IE2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IE2_A::ENABLE
    }
}
#[doc = "Field `IE2` writer - Interrupt on Pn.2 enable"]
pub type IE2_W<'a> = crate::BitWriter<'a, u32, IE_SPEC, IE2_A, 2>;
impl<'a> IE2_W<'a> {
    #[doc = "Disable interrupt on Pn.2"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IE2_A::DISABLE)
    }
    #[doc = "Enable interrupt on Pn.2"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IE2_A::ENABLE)
    }
}
#[doc = "Interrupt on Pn.3 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE3_A {
    #[doc = "0: Disable interrupt on Pn.3"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt on Pn.3"]
    ENABLE = 1,
}
impl From<IE3_A> for bool {
    #[inline(always)]
    fn from(variant: IE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IE3` reader - Interrupt on Pn.3 enable"]
pub type IE3_R = crate::BitReader<IE3_A>;
impl IE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE3_A {
        match self.bits {
            false => IE3_A::DISABLE,
            true => IE3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IE3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IE3_A::ENABLE
    }
}
#[doc = "Field `IE3` writer - Interrupt on Pn.3 enable"]
pub type IE3_W<'a> = crate::BitWriter<'a, u32, IE_SPEC, IE3_A, 3>;
impl<'a> IE3_W<'a> {
    #[doc = "Disable interrupt on Pn.3"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IE3_A::DISABLE)
    }
    #[doc = "Enable interrupt on Pn.3"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IE3_A::ENABLE)
    }
}
#[doc = "Interrupt on Pn.4 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE4_A {
    #[doc = "0: Disable interrupt on Pn.4"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt on Pn.4"]
    ENABLE = 1,
}
impl From<IE4_A> for bool {
    #[inline(always)]
    fn from(variant: IE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IE4` reader - Interrupt on Pn.4 enable"]
pub type IE4_R = crate::BitReader<IE4_A>;
impl IE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE4_A {
        match self.bits {
            false => IE4_A::DISABLE,
            true => IE4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IE4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IE4_A::ENABLE
    }
}
#[doc = "Field `IE4` writer - Interrupt on Pn.4 enable"]
pub type IE4_W<'a> = crate::BitWriter<'a, u32, IE_SPEC, IE4_A, 4>;
impl<'a> IE4_W<'a> {
    #[doc = "Disable interrupt on Pn.4"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IE4_A::DISABLE)
    }
    #[doc = "Enable interrupt on Pn.4"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IE4_A::ENABLE)
    }
}
#[doc = "Interrupt on Pn.5 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE5_A {
    #[doc = "0: Disable interrupt on Pn.5"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt on Pn.5"]
    ENABLE = 1,
}
impl From<IE5_A> for bool {
    #[inline(always)]
    fn from(variant: IE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IE5` reader - Interrupt on Pn.5 enable"]
pub type IE5_R = crate::BitReader<IE5_A>;
impl IE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE5_A {
        match self.bits {
            false => IE5_A::DISABLE,
            true => IE5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IE5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IE5_A::ENABLE
    }
}
#[doc = "Field `IE5` writer - Interrupt on Pn.5 enable"]
pub type IE5_W<'a> = crate::BitWriter<'a, u32, IE_SPEC, IE5_A, 5>;
impl<'a> IE5_W<'a> {
    #[doc = "Disable interrupt on Pn.5"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IE5_A::DISABLE)
    }
    #[doc = "Enable interrupt on Pn.5"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IE5_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt on Pn.0 enable"]
    #[inline(always)]
    pub fn ie0(&self) -> IE0_R {
        IE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt on Pn.1 enable"]
    #[inline(always)]
    pub fn ie1(&self) -> IE1_R {
        IE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt on Pn.2 enable"]
    #[inline(always)]
    pub fn ie2(&self) -> IE2_R {
        IE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt on Pn.3 enable"]
    #[inline(always)]
    pub fn ie3(&self) -> IE3_R {
        IE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt on Pn.4 enable"]
    #[inline(always)]
    pub fn ie4(&self) -> IE4_R {
        IE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt on Pn.5 enable"]
    #[inline(always)]
    pub fn ie5(&self) -> IE5_R {
        IE5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt on Pn.0 enable"]
    #[inline(always)]
    pub fn ie0(&mut self) -> IE0_W {
        IE0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt on Pn.1 enable"]
    #[inline(always)]
    pub fn ie1(&mut self) -> IE1_W {
        IE1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt on Pn.2 enable"]
    #[inline(always)]
    pub fn ie2(&mut self) -> IE2_W {
        IE2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt on Pn.3 enable"]
    #[inline(always)]
    pub fn ie3(&mut self) -> IE3_W {
        IE3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt on Pn.4 enable"]
    #[inline(always)]
    pub fn ie4(&mut self) -> IE4_W {
        IE4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt on Pn.5 enable"]
    #[inline(always)]
    pub fn ie5(&mut self) -> IE5_W {
        IE5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x18 GPIO Port n Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](index.html) module"]
pub struct IE_SPEC;
impl crate::RegisterSpec for IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ie::R](R) reader structure"]
impl crate::Readable for IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ie::W](W) writer structure"]
impl crate::Writable for IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
