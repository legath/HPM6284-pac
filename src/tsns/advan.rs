#[doc = "Register `ADVAN` reader"]
pub type R = crate::R<ADVAN_SPEC>;
#[doc = "Register `ADVAN` writer"]
pub type W = crate::W<ADVAN_SPEC>;
#[doc = "Field `POS_ONLY` reader - use positive compare polarity only"]
pub type POS_ONLY_R = crate::BitReader;
#[doc = "Field `POS_ONLY` writer - use positive compare polarity only"]
pub type POS_ONLY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEG_ONLY` reader - use negative compare polarity only"]
pub type NEG_ONLY_R = crate::BitReader;
#[doc = "Field `NEG_ONLY` writer - use negative compare polarity only"]
pub type NEG_ONLY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMPLING` reader - temperature sampling is working"]
pub type SAMPLING_R = crate::BitReader;
#[doc = "Field `ACTIVE_IRQ` reader - interrupt status of active mode"]
pub type ACTIVE_IRQ_R = crate::BitReader;
#[doc = "Field `ASYNC_IRQ` reader - interrupt status of asynchronous mode"]
pub type ASYNC_IRQ_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - use positive compare polarity only"]
    #[inline(always)]
    pub fn pos_only(&self) -> POS_ONLY_R {
        POS_ONLY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - use negative compare polarity only"]
    #[inline(always)]
    pub fn neg_only(&self) -> NEG_ONLY_R {
        NEG_ONLY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - temperature sampling is working"]
    #[inline(always)]
    pub fn sampling(&self) -> SAMPLING_R {
        SAMPLING_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - interrupt status of active mode"]
    #[inline(always)]
    pub fn active_irq(&self) -> ACTIVE_IRQ_R {
        ACTIVE_IRQ_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - interrupt status of asynchronous mode"]
    #[inline(always)]
    pub fn async_irq(&self) -> ASYNC_IRQ_R {
        ASYNC_IRQ_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - use positive compare polarity only"]
    #[inline(always)]
    #[must_use]
    pub fn pos_only(&mut self) -> POS_ONLY_W<ADVAN_SPEC> {
        POS_ONLY_W::new(self, 0)
    }
    #[doc = "Bit 1 - use negative compare polarity only"]
    #[inline(always)]
    #[must_use]
    pub fn neg_only(&mut self) -> NEG_ONLY_W<ADVAN_SPEC> {
        NEG_ONLY_W::new(self, 1)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Advance configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`advan::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`advan::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADVAN_SPEC;
impl crate::RegisterSpec for ADVAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`advan::R`](R) reader structure"]
impl crate::Readable for ADVAN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`advan::W`](W) writer structure"]
impl crate::Writable for ADVAN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADVAN to value 0"]
impl crate::Resettable for ADVAN_SPEC {
    const RESET_VALUE: u32 = 0;
}
