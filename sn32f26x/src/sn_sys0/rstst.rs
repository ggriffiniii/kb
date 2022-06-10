#[doc = "Register `RSTST` reader"]
pub struct R(crate::R<RSTST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTST` writer"]
pub struct W(crate::W<RSTST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTST_SPEC>;
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
impl From<crate::W<RSTST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Software reset flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRSTF_A {
    #[doc = "0: No SW reset occurred"]
    _0 = 0,
    #[doc = "1: SW reset occurred"]
    _1 = 1,
}
impl From<SWRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: SWRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRSTF` reader - Software reset flag"]
pub type SWRSTF_R = crate::BitReader<SWRSTF_A>;
impl SWRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRSTF_A {
        match self.bits {
            false => SWRSTF_A::_0,
            true => SWRSTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWRSTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWRSTF_A::_1
    }
}
#[doc = "Field `SWRSTF` writer - Software reset flag"]
pub type SWRSTF_W<'a> = crate::BitWriter<'a, u32, RSTST_SPEC, SWRSTF_A, 0>;
impl<'a> SWRSTF_W<'a> {
    #[doc = "No SW reset occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWRSTF_A::_0)
    }
    #[doc = "SW reset occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWRSTF_A::_1)
    }
}
#[doc = "WDT reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTRSTF_A {
    #[doc = "0: No WDT reset occurred"]
    _0 = 0,
    #[doc = "1: WDT reset occurred"]
    _1 = 1,
}
impl From<WDTRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: WDTRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTRSTF` reader - WDT reset flag"]
pub type WDTRSTF_R = crate::BitReader<WDTRSTF_A>;
impl WDTRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTRSTF_A {
        match self.bits {
            false => WDTRSTF_A::_0,
            true => WDTRSTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WDTRSTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WDTRSTF_A::_1
    }
}
#[doc = "Field `WDTRSTF` writer - WDT reset flag"]
pub type WDTRSTF_W<'a> = crate::BitWriter<'a, u32, RSTST_SPEC, WDTRSTF_A, 1>;
impl<'a> WDTRSTF_W<'a> {
    #[doc = "No WDT reset occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WDTRSTF_A::_0)
    }
    #[doc = "WDT reset occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WDTRSTF_A::_1)
    }
}
#[doc = "LVD reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDRSTF_A {
    #[doc = "0: No LVD reset occurred"]
    _0 = 0,
    #[doc = "1: LVD reset occurred"]
    _1 = 1,
}
impl From<LVDRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: LVDRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDRSTF` reader - LVD reset flag"]
pub type LVDRSTF_R = crate::BitReader<LVDRSTF_A>;
impl LVDRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDRSTF_A {
        match self.bits {
            false => LVDRSTF_A::_0,
            true => LVDRSTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVDRSTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVDRSTF_A::_1
    }
}
#[doc = "Field `LVDRSTF` writer - LVD reset flag"]
pub type LVDRSTF_W<'a> = crate::BitWriter<'a, u32, RSTST_SPEC, LVDRSTF_A, 2>;
impl<'a> LVDRSTF_W<'a> {
    #[doc = "No LVD reset occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVDRSTF_A::_0)
    }
    #[doc = "LVD reset occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVDRSTF_A::_1)
    }
}
#[doc = "External reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTRSTF_A {
    #[doc = "0: No Extenral reset occurred"]
    _0 = 0,
    #[doc = "1: External reset occurred"]
    _1 = 1,
}
impl From<EXTRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: EXTRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTRSTF` reader - External reset flag"]
pub type EXTRSTF_R = crate::BitReader<EXTRSTF_A>;
impl EXTRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTRSTF_A {
        match self.bits {
            false => EXTRSTF_A::_0,
            true => EXTRSTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXTRSTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXTRSTF_A::_1
    }
}
#[doc = "Field `EXTRSTF` writer - External reset flag"]
pub type EXTRSTF_W<'a> = crate::BitWriter<'a, u32, RSTST_SPEC, EXTRSTF_A, 3>;
impl<'a> EXTRSTF_W<'a> {
    #[doc = "No Extenral reset occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXTRSTF_A::_0)
    }
    #[doc = "External reset occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXTRSTF_A::_1)
    }
}
#[doc = "POR reset flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORRSTF_A {
    #[doc = "0: No POR occurred"]
    _0 = 0,
    #[doc = "1: POR occurred"]
    _1 = 1,
}
impl From<PORRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: PORRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORRSTF` reader - POR reset flag"]
pub type PORRSTF_R = crate::BitReader<PORRSTF_A>;
impl PORRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORRSTF_A {
        match self.bits {
            false => PORRSTF_A::_0,
            true => PORRSTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORRSTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORRSTF_A::_1
    }
}
#[doc = "Field `PORRSTF` writer - POR reset flag"]
pub type PORRSTF_W<'a> = crate::BitWriter<'a, u32, RSTST_SPEC, PORRSTF_A, 4>;
impl<'a> PORRSTF_W<'a> {
    #[doc = "No POR occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORRSTF_A::_0)
    }
    #[doc = "POR occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORRSTF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Software reset flag"]
    #[inline(always)]
    pub fn swrstf(&self) -> SWRSTF_R {
        SWRSTF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDT reset flag"]
    #[inline(always)]
    pub fn wdtrstf(&self) -> WDTRSTF_R {
        WDTRSTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LVD reset flag"]
    #[inline(always)]
    pub fn lvdrstf(&self) -> LVDRSTF_R {
        LVDRSTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External reset flag"]
    #[inline(always)]
    pub fn extrstf(&self) -> EXTRSTF_R {
        EXTRSTF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - POR reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset flag"]
    #[inline(always)]
    pub fn swrstf(&mut self) -> SWRSTF_W {
        SWRSTF_W::new(self)
    }
    #[doc = "Bit 1 - WDT reset flag"]
    #[inline(always)]
    pub fn wdtrstf(&mut self) -> WDTRSTF_W {
        WDTRSTF_W::new(self)
    }
    #[doc = "Bit 2 - LVD reset flag"]
    #[inline(always)]
    pub fn lvdrstf(&mut self) -> LVDRSTF_W {
        LVDRSTF_W::new(self)
    }
    #[doc = "Bit 3 - External reset flag"]
    #[inline(always)]
    pub fn extrstf(&mut self) -> EXTRSTF_W {
        EXTRSTF_W::new(self)
    }
    #[doc = "Bit 4 - POR reset flag"]
    #[inline(always)]
    pub fn porrstf(&mut self) -> PORRSTF_W {
        PORRSTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x14 System Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstst](index.html) module"]
pub struct RSTST_SPEC;
impl crate::RegisterSpec for RSTST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstst::R](R) reader structure"]
impl crate::Readable for RSTST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstst::W](W) writer structure"]
impl crate::Writable for RSTST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTST to value 0x11"]
impl crate::Resettable for RSTST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x11
    }
}
