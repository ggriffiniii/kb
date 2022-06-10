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
#[doc = "RX FIFO overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOVFIE_A {
    #[doc = "0: Disable RX FIFO overflow interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable RX FIFO overflow interrupt"]
    ENABLE = 1,
}
impl From<RXOVFIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXOVFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOVFIE` reader - RX FIFO overflow interrupt enable"]
pub type RXOVFIE_R = crate::BitReader<RXOVFIE_A>;
impl RXOVFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOVFIE_A {
        match self.bits {
            false => RXOVFIE_A::DISABLE,
            true => RXOVFIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXOVFIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXOVFIE_A::ENABLE
    }
}
#[doc = "Field `RXOVFIE` writer - RX FIFO overflow interrupt enable"]
pub type RXOVFIE_W<'a> = crate::BitWriter<'a, u32, IE_SPEC, RXOVFIE_A, 0>;
impl<'a> RXOVFIE_W<'a> {
    #[doc = "Disable RX FIFO overflow interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXOVFIE_A::DISABLE)
    }
    #[doc = "Enable RX FIFO overflow interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXOVFIE_A::ENABLE)
    }
}
#[doc = "RX time-out interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTOIE_A {
    #[doc = "0: Disable RX time-out interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable RX time-out interrupt"]
    ENABLE = 1,
}
impl From<RXTOIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXTOIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTOIE` reader - RX time-out interrupt enable"]
pub type RXTOIE_R = crate::BitReader<RXTOIE_A>;
impl RXTOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXTOIE_A {
        match self.bits {
            false => RXTOIE_A::DISABLE,
            true => RXTOIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXTOIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXTOIE_A::ENABLE
    }
}
#[doc = "Field `RXTOIE` writer - RX time-out interrupt enable"]
pub type RXTOIE_W<'a> = crate::BitWriter<'a, u32, IE_SPEC, RXTOIE_A, 1>;
impl<'a> RXTOIE_W<'a> {
    #[doc = "Disable RX time-out interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXTOIE_A::DISABLE)
    }
    #[doc = "Enable RX time-out interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXTOIE_A::ENABLE)
    }
}
#[doc = "RX FIFO threshold interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFOTHIE_A {
    #[doc = "0: Disable RX FIFO threshold interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable RX FIFO threshold interrupt"]
    ENABLE = 1,
}
impl From<RXFIFOTHIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFIFOTHIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFOTHIE` reader - RX FIFO threshold interrupt enable"]
pub type RXFIFOTHIE_R = crate::BitReader<RXFIFOTHIE_A>;
impl RXFIFOTHIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFIFOTHIE_A {
        match self.bits {
            false => RXFIFOTHIE_A::DISABLE,
            true => RXFIFOTHIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXFIFOTHIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXFIFOTHIE_A::ENABLE
    }
}
#[doc = "Field `RXFIFOTHIE` writer - RX FIFO threshold interrupt enable"]
pub type RXFIFOTHIE_W<'a> = crate::BitWriter<'a, u32, IE_SPEC, RXFIFOTHIE_A, 2>;
impl<'a> RXFIFOTHIE_W<'a> {
    #[doc = "Disable RX FIFO threshold interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXFIFOTHIE_A::DISABLE)
    }
    #[doc = "Enable RX FIFO threshold interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXFIFOTHIE_A::ENABLE)
    }
}
#[doc = "TX FIFO threshold interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIFOTHIE_A {
    #[doc = "0: Disable TX FIFO threshold interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable TX FIFO threshold interrupt"]
    ENABLE = 1,
}
impl From<TXFIFOTHIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFIFOTHIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIFOTHIE` reader - TX FIFO threshold interrupt enable"]
pub type TXFIFOTHIE_R = crate::BitReader<TXFIFOTHIE_A>;
impl TXFIFOTHIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFIFOTHIE_A {
        match self.bits {
            false => TXFIFOTHIE_A::DISABLE,
            true => TXFIFOTHIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXFIFOTHIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXFIFOTHIE_A::ENABLE
    }
}
#[doc = "Field `TXFIFOTHIE` writer - TX FIFO threshold interrupt enable"]
pub type TXFIFOTHIE_W<'a> = crate::BitWriter<'a, u32, IE_SPEC, TXFIFOTHIE_A, 3>;
impl<'a> TXFIFOTHIE_W<'a> {
    #[doc = "Disable TX FIFO threshold interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXFIFOTHIE_A::DISABLE)
    }
    #[doc = "Enable TX FIFO threshold interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXFIFOTHIE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - RX FIFO overflow interrupt enable"]
    #[inline(always)]
    pub fn rxovfie(&self) -> RXOVFIE_R {
        RXOVFIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX time-out interrupt enable"]
    #[inline(always)]
    pub fn rxtoie(&self) -> RXTOIE_R {
        RXTOIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO threshold interrupt enable"]
    #[inline(always)]
    pub fn rxfifothie(&self) -> RXFIFOTHIE_R {
        RXFIFOTHIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX FIFO threshold interrupt enable"]
    #[inline(always)]
    pub fn txfifothie(&self) -> TXFIFOTHIE_R {
        TXFIFOTHIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX FIFO overflow interrupt enable"]
    #[inline(always)]
    pub fn rxovfie(&mut self) -> RXOVFIE_W {
        RXOVFIE_W::new(self)
    }
    #[doc = "Bit 1 - RX time-out interrupt enable"]
    #[inline(always)]
    pub fn rxtoie(&mut self) -> RXTOIE_W {
        RXTOIE_W::new(self)
    }
    #[doc = "Bit 2 - RX FIFO threshold interrupt enable"]
    #[inline(always)]
    pub fn rxfifothie(&mut self) -> RXFIFOTHIE_W {
        RXFIFOTHIE_W::new(self)
    }
    #[doc = "Bit 3 - TX FIFO threshold interrupt enable"]
    #[inline(always)]
    pub fn txfifothie(&mut self) -> TXFIFOTHIE_W {
        TXFIFOTHIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x10 SPIn Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](index.html) module"]
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
