#[doc = "Register `RWSTATUS2` reader"]
pub struct R(crate::R<RWSTATUS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RWSTATUS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RWSTATUS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RWSTATUS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RWSTATUS2` writer"]
pub struct W(crate::W<RWSTATUS2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RWSTATUS2_SPEC>;
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
impl From<crate::W<RWSTATUS2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RWSTATUS2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Write status of USB FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum W_STATUS_A {
    #[doc = "0: this bit is automatically cleared as ��0�� by hardware"]
    OPERATIONREADY = 0,
    #[doc = "1: F/W is to write data into USB FIFO now"]
    NOTREADY = 1,
}
impl From<W_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: W_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `W_STATUS` reader - Write status of USB FIFO"]
pub type W_STATUS_R = crate::BitReader<W_STATUS_A>;
impl W_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> W_STATUS_A {
        match self.bits {
            false => W_STATUS_A::OPERATIONREADY,
            true => W_STATUS_A::NOTREADY,
        }
    }
    #[doc = "Checks if the value of the field is `OPERATIONREADY`"]
    #[inline(always)]
    pub fn is_operationready(&self) -> bool {
        *self == W_STATUS_A::OPERATIONREADY
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_notready(&self) -> bool {
        *self == W_STATUS_A::NOTREADY
    }
}
#[doc = "Field `W_STATUS` writer - Write status of USB FIFO"]
pub type W_STATUS_W<'a> = crate::BitWriter<'a, u32, RWSTATUS2_SPEC, W_STATUS_A, 0>;
impl<'a> W_STATUS_W<'a> {
    #[doc = "this bit is automatically cleared as ��0�� by hardware"]
    #[inline(always)]
    pub fn operationready(self) -> &'a mut W {
        self.variant(W_STATUS_A::OPERATIONREADY)
    }
    #[doc = "F/W is to write data into USB FIFO now"]
    #[inline(always)]
    pub fn notready(self) -> &'a mut W {
        self.variant(W_STATUS_A::NOTREADY)
    }
}
#[doc = "WRead status of USB FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R_STATUS_A {
    #[doc = "0: this bit is automatically cleared as ��0�� by hardware"]
    OPERATIONREADY = 0,
    #[doc = "1: If F/W is to read the data from USB FIFO now"]
    NOTREADY = 1,
}
impl From<R_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: R_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R_STATUS` reader - WRead status of USB FIFO"]
pub type R_STATUS_R = crate::BitReader<R_STATUS_A>;
impl R_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R_STATUS_A {
        match self.bits {
            false => R_STATUS_A::OPERATIONREADY,
            true => R_STATUS_A::NOTREADY,
        }
    }
    #[doc = "Checks if the value of the field is `OPERATIONREADY`"]
    #[inline(always)]
    pub fn is_operationready(&self) -> bool {
        *self == R_STATUS_A::OPERATIONREADY
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_notready(&self) -> bool {
        *self == R_STATUS_A::NOTREADY
    }
}
#[doc = "Field `R_STATUS` writer - WRead status of USB FIFO"]
pub type R_STATUS_W<'a> = crate::BitWriter<'a, u32, RWSTATUS2_SPEC, R_STATUS_A, 1>;
impl<'a> R_STATUS_W<'a> {
    #[doc = "this bit is automatically cleared as ��0�� by hardware"]
    #[inline(always)]
    pub fn operationready(self) -> &'a mut W {
        self.variant(R_STATUS_A::OPERATIONREADY)
    }
    #[doc = "If F/W is to read the data from USB FIFO now"]
    #[inline(always)]
    pub fn notready(self) -> &'a mut W {
        self.variant(R_STATUS_A::NOTREADY)
    }
}
impl R {
    #[doc = "Bit 0 - Write status of USB FIFO"]
    #[inline(always)]
    pub fn w_status(&self) -> W_STATUS_R {
        W_STATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WRead status of USB FIFO"]
    #[inline(always)]
    pub fn r_status(&self) -> R_STATUS_R {
        R_STATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write status of USB FIFO"]
    #[inline(always)]
    pub fn w_status(&mut self) -> W_STATUS_W {
        W_STATUS_W::new(self)
    }
    #[doc = "Bit 1 - WRead status of USB FIFO"]
    #[inline(always)]
    pub fn r_status(&mut self) -> R_STATUS_W {
        R_STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x8C USB Read/Write Status Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rwstatus2](index.html) module"]
pub struct RWSTATUS2_SPEC;
impl crate::RegisterSpec for RWSTATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rwstatus2::R](R) reader structure"]
impl crate::Readable for RWSTATUS2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rwstatus2::W](W) writer structure"]
impl crate::Writable for RWSTATUS2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RWSTATUS2 to value 0"]
impl crate::Resettable for RWSTATUS2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
