#[doc = "Register `AHBCP` reader"]
pub struct R(crate::R<AHBCP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBCP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBCP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBCP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBCP` writer"]
pub struct W(crate::W<AHBCP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBCP_SPEC>;
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
impl From<crate::W<AHBCP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBCP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "AHB clock source prescaler\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AHBPRE_A {
    #[doc = "0: FAHB=FSYSCLK/1"]
    _000 = 0,
    #[doc = "1: FAHB=FSYSCLK/2"]
    _001 = 1,
    #[doc = "2: FAHB=FSYSCLK/4"]
    _010 = 2,
    #[doc = "3: FAHB=FSYSCLK/8"]
    _011 = 3,
    #[doc = "4: FAHB=FSYSCLK/16"]
    _100 = 4,
    #[doc = "5: FAHB=FSYSCLK/32"]
    _101 = 5,
    #[doc = "6: FAHB=FSYSCLK/64"]
    _110 = 6,
    #[doc = "7: FAHB=FSYSCLK/128"]
    _111 = 7,
}
impl From<AHBPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: AHBPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AHBPRE` reader - AHB clock source prescaler"]
pub type AHBPRE_R = crate::FieldReader<u8, AHBPRE_A>;
impl AHBPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHBPRE_A {
        match self.bits {
            0 => AHBPRE_A::_000,
            1 => AHBPRE_A::_001,
            2 => AHBPRE_A::_010,
            3 => AHBPRE_A::_011,
            4 => AHBPRE_A::_100,
            5 => AHBPRE_A::_101,
            6 => AHBPRE_A::_110,
            7 => AHBPRE_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == AHBPRE_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == AHBPRE_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == AHBPRE_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == AHBPRE_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == AHBPRE_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == AHBPRE_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == AHBPRE_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == AHBPRE_A::_111
    }
}
#[doc = "Field `AHBPRE` writer - AHB clock source prescaler"]
pub type AHBPRE_W<'a> = crate::FieldWriterSafe<'a, u32, AHBCP_SPEC, u8, AHBPRE_A, 3, 0>;
impl<'a> AHBPRE_W<'a> {
    #[doc = "FAHB=FSYSCLK/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(AHBPRE_A::_000)
    }
    #[doc = "FAHB=FSYSCLK/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(AHBPRE_A::_001)
    }
    #[doc = "FAHB=FSYSCLK/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(AHBPRE_A::_010)
    }
    #[doc = "FAHB=FSYSCLK/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(AHBPRE_A::_011)
    }
    #[doc = "FAHB=FSYSCLK/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(AHBPRE_A::_100)
    }
    #[doc = "FAHB=FSYSCLK/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(AHBPRE_A::_101)
    }
    #[doc = "FAHB=FSYSCLK/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(AHBPRE_A::_110)
    }
    #[doc = "FAHB=FSYSCLK/128"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(AHBPRE_A::_111)
    }
}
impl R {
    #[doc = "Bits 0:2 - AHB clock source prescaler"]
    #[inline(always)]
    pub fn ahbpre(&self) -> AHBPRE_R {
        AHBPRE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - AHB clock source prescaler"]
    #[inline(always)]
    pub fn ahbpre(&mut self) -> AHBPRE_W {
        AHBPRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x10 AHB Clock Prescale Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbcp](index.html) module"]
pub struct AHBCP_SPEC;
impl crate::RegisterSpec for AHBCP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbcp::R](R) reader structure"]
impl crate::Readable for AHBCP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbcp::W](W) writer structure"]
impl crate::Writable for AHBCP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBCP to value 0x02"]
impl crate::Resettable for AHBCP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
