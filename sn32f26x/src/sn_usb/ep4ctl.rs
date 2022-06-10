#[doc = "Register `EP4CTL` reader"]
pub struct R(crate::R<EP4CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP4CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP4CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP4CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP4CTL` writer"]
pub struct W(crate::W<EP4CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP4CTL_SPEC>;
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
impl From<crate::W<EP4CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP4CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENDP_CNT` reader - Endpoint byte count"]
pub type ENDP_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENDP_CNT` writer - Endpoint byte count"]
pub type ENDP_CNT_W<'a> = crate::FieldWriter<'a, u32, EP4CTL_SPEC, u8, u8, 7, 0>;
#[doc = "Endpoint Handshake State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ENDP_STATE_A {
    #[doc = "0: NAK"]
    _0 = 0,
    #[doc = "1: ACK"]
    _1 = 1,
    #[doc = "2: STALL"]
    _2 = 2,
    #[doc = "3: STALL"]
    _3 = 3,
}
impl From<ENDP_STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: ENDP_STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ENDP_STATE` reader - Endpoint Handshake State"]
pub type ENDP_STATE_R = crate::FieldReader<u8, ENDP_STATE_A>;
impl ENDP_STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDP_STATE_A {
        match self.bits {
            0 => ENDP_STATE_A::_0,
            1 => ENDP_STATE_A::_1,
            2 => ENDP_STATE_A::_2,
            3 => ENDP_STATE_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENDP_STATE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENDP_STATE_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == ENDP_STATE_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == ENDP_STATE_A::_3
    }
}
#[doc = "Field `ENDP_STATE` writer - Endpoint Handshake State"]
pub type ENDP_STATE_W<'a> = crate::FieldWriterSafe<'a, u32, EP4CTL_SPEC, u8, ENDP_STATE_A, 2, 29>;
impl<'a> ENDP_STATE_W<'a> {
    #[doc = "NAK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENDP_STATE_A::_0)
    }
    #[doc = "ACK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENDP_STATE_A::_1)
    }
    #[doc = "STALL"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(ENDP_STATE_A::_2)
    }
    #[doc = "STALL"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(ENDP_STATE_A::_3)
    }
}
#[doc = "Endpoint 4 Function enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDP_EN_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<ENDP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ENDP_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDP_EN` reader - Endpoint 4 Function enable bit"]
pub type ENDP_EN_R = crate::BitReader<ENDP_EN_A>;
impl ENDP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDP_EN_A {
        match self.bits {
            false => ENDP_EN_A::_0,
            true => ENDP_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENDP_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENDP_EN_A::_1
    }
}
#[doc = "Field `ENDP_EN` writer - Endpoint 4 Function enable bit"]
pub type ENDP_EN_W<'a> = crate::BitWriter<'a, u32, EP4CTL_SPEC, ENDP_EN_A, 31>;
impl<'a> ENDP_EN_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENDP_EN_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENDP_EN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:6 - Endpoint byte count"]
    #[inline(always)]
    pub fn endp_cnt(&self) -> ENDP_CNT_R {
        ENDP_CNT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 29:30 - Endpoint Handshake State"]
    #[inline(always)]
    pub fn endp_state(&self) -> ENDP_STATE_R {
        ENDP_STATE_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Endpoint 4 Function enable bit"]
    #[inline(always)]
    pub fn endp_en(&self) -> ENDP_EN_R {
        ENDP_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Endpoint byte count"]
    #[inline(always)]
    pub fn endp_cnt(&mut self) -> ENDP_CNT_W {
        ENDP_CNT_W::new(self)
    }
    #[doc = "Bits 29:30 - Endpoint Handshake State"]
    #[inline(always)]
    pub fn endp_state(&mut self) -> ENDP_STATE_W {
        ENDP_STATE_W::new(self)
    }
    #[doc = "Bit 31 - Endpoint 4 Function enable bit"]
    #[inline(always)]
    pub fn endp_en(&mut self) -> ENDP_EN_W {
        ENDP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x28 USB Endpoint 4 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep4ctl](index.html) module"]
pub struct EP4CTL_SPEC;
impl crate::RegisterSpec for EP4CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep4ctl::R](R) reader structure"]
impl crate::Readable for EP4CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep4ctl::W](W) writer structure"]
impl crate::Writable for EP4CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EP4CTL to value 0"]
impl crate::Resettable for EP4CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
