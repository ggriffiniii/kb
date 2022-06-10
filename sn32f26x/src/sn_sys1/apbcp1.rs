#[doc = "Register `APBCP1` reader"]
pub struct R(crate::R<APBCP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBCP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBCP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBCP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBCP1` writer"]
pub struct W(crate::W<APBCP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBCP1_SPEC>;
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
impl From<crate::W<APBCP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBCP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SysTick APB clock source prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSTICKPRE_A {
    #[doc = "0: HCLK/1"]
    _00 = 0,
    #[doc = "1: HCLK/2"]
    _01 = 1,
    #[doc = "2: HCLK/4"]
    _10 = 2,
    #[doc = "3: HCLK/8"]
    _11 = 3,
}
impl From<SYSTICKPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSTICKPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSTICKPRE` reader - SysTick APB clock source prescaler"]
pub type SYSTICKPRE_R = crate::FieldReader<u8, SYSTICKPRE_A>;
impl SYSTICKPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSTICKPRE_A {
        match self.bits {
            0 => SYSTICKPRE_A::_00,
            1 => SYSTICKPRE_A::_01,
            2 => SYSTICKPRE_A::_10,
            3 => SYSTICKPRE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SYSTICKPRE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SYSTICKPRE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SYSTICKPRE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SYSTICKPRE_A::_11
    }
}
#[doc = "Field `SYSTICKPRE` writer - SysTick APB clock source prescaler"]
pub type SYSTICKPRE_W<'a> = crate::FieldWriterSafe<'a, u32, APBCP1_SPEC, u8, SYSTICKPRE_A, 2, 16>;
impl<'a> SYSTICKPRE_W<'a> {
    #[doc = "HCLK/1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SYSTICKPRE_A::_00)
    }
    #[doc = "HCLK/2"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SYSTICKPRE_A::_01)
    }
    #[doc = "HCLK/4"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SYSTICKPRE_A::_10)
    }
    #[doc = "HCLK/8"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SYSTICKPRE_A::_11)
    }
}
#[doc = "WDT APB clock source prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDTPRE_A {
    #[doc = "0: HCLK/1"]
    _000 = 0,
    #[doc = "1: HCLK/2"]
    _001 = 1,
    #[doc = "2: HCLK/4"]
    _010 = 2,
    #[doc = "3: HCLK/8"]
    _011 = 3,
    #[doc = "4: HCLK/16"]
    _100 = 4,
    #[doc = "5: HCLK/32"]
    _101 = 5,
}
impl From<WDTPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WDTPRE` reader - WDT APB clock source prescaler"]
pub type WDTPRE_R = crate::FieldReader<u8, WDTPRE_A>;
impl WDTPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WDTPRE_A> {
        match self.bits {
            0 => Some(WDTPRE_A::_000),
            1 => Some(WDTPRE_A::_001),
            2 => Some(WDTPRE_A::_010),
            3 => Some(WDTPRE_A::_011),
            4 => Some(WDTPRE_A::_100),
            5 => Some(WDTPRE_A::_101),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == WDTPRE_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == WDTPRE_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == WDTPRE_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == WDTPRE_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == WDTPRE_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == WDTPRE_A::_101
    }
}
#[doc = "Field `WDTPRE` writer - WDT APB clock source prescaler"]
pub type WDTPRE_W<'a> = crate::FieldWriter<'a, u32, APBCP1_SPEC, u8, WDTPRE_A, 3, 20>;
impl<'a> WDTPRE_W<'a> {
    #[doc = "HCLK/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(WDTPRE_A::_000)
    }
    #[doc = "HCLK/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(WDTPRE_A::_001)
    }
    #[doc = "HCLK/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(WDTPRE_A::_010)
    }
    #[doc = "HCLK/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(WDTPRE_A::_011)
    }
    #[doc = "HCLK/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(WDTPRE_A::_100)
    }
    #[doc = "HCLK/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(WDTPRE_A::_101)
    }
}
#[doc = "CLKOUT APB clock source prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKOUTPRE_A {
    #[doc = "0: FCLKOUT/1"]
    _000 = 0,
    #[doc = "1: FCLKOUT/2"]
    _001 = 1,
    #[doc = "2: FCLKOUT/4"]
    _010 = 2,
    #[doc = "3: FCLKOUT/8"]
    _011 = 3,
    #[doc = "4: FCLKOUT/16"]
    _100 = 4,
    #[doc = "5: FCLKOUT/32"]
    _101 = 5,
    #[doc = "6: FCLKOUT/64"]
    _110 = 6,
    #[doc = "7: FCLKOUT/128"]
    _111 = 7,
}
impl From<CLKOUTPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKOUTPRE` reader - CLKOUT APB clock source prescaler"]
pub type CLKOUTPRE_R = crate::FieldReader<u8, CLKOUTPRE_A>;
impl CLKOUTPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKOUTPRE_A {
        match self.bits {
            0 => CLKOUTPRE_A::_000,
            1 => CLKOUTPRE_A::_001,
            2 => CLKOUTPRE_A::_010,
            3 => CLKOUTPRE_A::_011,
            4 => CLKOUTPRE_A::_100,
            5 => CLKOUTPRE_A::_101,
            6 => CLKOUTPRE_A::_110,
            7 => CLKOUTPRE_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CLKOUTPRE_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CLKOUTPRE_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CLKOUTPRE_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CLKOUTPRE_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CLKOUTPRE_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CLKOUTPRE_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CLKOUTPRE_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == CLKOUTPRE_A::_111
    }
}
#[doc = "Field `CLKOUTPRE` writer - CLKOUT APB clock source prescaler"]
pub type CLKOUTPRE_W<'a> = crate::FieldWriterSafe<'a, u32, APBCP1_SPEC, u8, CLKOUTPRE_A, 3, 28>;
impl<'a> CLKOUTPRE_W<'a> {
    #[doc = "FCLKOUT/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CLKOUTPRE_A::_000)
    }
    #[doc = "FCLKOUT/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CLKOUTPRE_A::_001)
    }
    #[doc = "FCLKOUT/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CLKOUTPRE_A::_010)
    }
    #[doc = "FCLKOUT/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CLKOUTPRE_A::_011)
    }
    #[doc = "FCLKOUT/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CLKOUTPRE_A::_100)
    }
    #[doc = "FCLKOUT/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CLKOUTPRE_A::_101)
    }
    #[doc = "FCLKOUT/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CLKOUTPRE_A::_110)
    }
    #[doc = "FCLKOUT/128"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(CLKOUTPRE_A::_111)
    }
}
impl R {
    #[doc = "Bits 16:17 - SysTick APB clock source prescaler"]
    #[inline(always)]
    pub fn systickpre(&self) -> SYSTICKPRE_R {
        SYSTICKPRE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:22 - WDT APB clock source prescaler"]
    #[inline(always)]
    pub fn wdtpre(&self) -> WDTPRE_R {
        WDTPRE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 28:30 - CLKOUT APB clock source prescaler"]
    #[inline(always)]
    pub fn clkoutpre(&self) -> CLKOUTPRE_R {
        CLKOUTPRE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - SysTick APB clock source prescaler"]
    #[inline(always)]
    pub fn systickpre(&mut self) -> SYSTICKPRE_W {
        SYSTICKPRE_W::new(self)
    }
    #[doc = "Bits 20:22 - WDT APB clock source prescaler"]
    #[inline(always)]
    pub fn wdtpre(&mut self) -> WDTPRE_W {
        WDTPRE_W::new(self)
    }
    #[doc = "Bits 28:30 - CLKOUT APB clock source prescaler"]
    #[inline(always)]
    pub fn clkoutpre(&mut self) -> CLKOUTPRE_W {
        CLKOUTPRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x08 APB Clock Prescale Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbcp1](index.html) module"]
pub struct APBCP1_SPEC;
impl crate::RegisterSpec for APBCP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbcp1::R](R) reader structure"]
impl crate::Readable for APBCP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbcp1::W](W) writer structure"]
impl crate::Writable for APBCP1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APBCP1 to value 0"]
impl crate::Resettable for APBCP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
