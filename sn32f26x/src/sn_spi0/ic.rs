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
#[doc = "RX FIFO overflow flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOVFIC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear RXOVF flag"]
    CLEAR = 1,
}
impl From<RXOVFIC_AW> for bool {
    #[inline(always)]
    fn from(variant: RXOVFIC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOVFIC` writer - RX FIFO overflow flag clear"]
pub type RXOVFIC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, RXOVFIC_AW, 0>;
impl<'a> RXOVFIC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(RXOVFIC_AW::NOEFFECT)
    }
    #[doc = "Clear RXOVF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXOVFIC_AW::CLEAR)
    }
}
#[doc = "RX time-out interrupt flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTOIC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear RXTO flag"]
    CLEAR = 1,
}
impl From<RXTOIC_AW> for bool {
    #[inline(always)]
    fn from(variant: RXTOIC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTOIC` writer - RX time-out interrupt flag clear"]
pub type RXTOIC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, RXTOIC_AW, 1>;
impl<'a> RXTOIC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(RXTOIC_AW::NOEFFECT)
    }
    #[doc = "Clear RXTO flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXTOIC_AW::CLEAR)
    }
}
#[doc = "RX Interrupt flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFOTHIC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear RXFIFOTH flag"]
    CLEAR = 1,
}
impl From<RXFIFOTHIC_AW> for bool {
    #[inline(always)]
    fn from(variant: RXFIFOTHIC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFOTHIC` writer - RX Interrupt flag Clear"]
pub type RXFIFOTHIC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, RXFIFOTHIC_AW, 2>;
impl<'a> RXFIFOTHIC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(RXFIFOTHIC_AW::NOEFFECT)
    }
    #[doc = "Clear RXFIFOTH flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXFIFOTHIC_AW::CLEAR)
    }
}
#[doc = "TX Interrupt flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIFOTHIC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear TXFIFOTH flag"]
    CLEAR = 1,
}
impl From<TXFIFOTHIC_AW> for bool {
    #[inline(always)]
    fn from(variant: TXFIFOTHIC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIFOTHIC` writer - TX Interrupt flag Clear"]
pub type TXFIFOTHIC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, TXFIFOTHIC_AW, 3>;
impl<'a> TXFIFOTHIC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(TXFIFOTHIC_AW::NOEFFECT)
    }
    #[doc = "Clear TXFIFOTH flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXFIFOTHIC_AW::CLEAR)
    }
}
impl W {
    #[doc = "Bit 0 - RX FIFO overflow flag clear"]
    #[inline(always)]
    pub fn rxovfic(&mut self) -> RXOVFIC_W {
        RXOVFIC_W::new(self)
    }
    #[doc = "Bit 1 - RX time-out interrupt flag clear"]
    #[inline(always)]
    pub fn rxtoic(&mut self) -> RXTOIC_W {
        RXTOIC_W::new(self)
    }
    #[doc = "Bit 2 - RX Interrupt flag Clear"]
    #[inline(always)]
    pub fn rxfifothic(&mut self) -> RXFIFOTHIC_W {
        RXFIFOTHIC_W::new(self)
    }
    #[doc = "Bit 3 - TX Interrupt flag Clear"]
    #[inline(always)]
    pub fn txfifothic(&mut self) -> TXFIFOTHIC_W {
        TXFIFOTHIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x18 SPIn Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic](index.html) module"]
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
