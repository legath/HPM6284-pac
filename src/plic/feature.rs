#[doc = "Register `feature` reader"]
pub type R = crate::R<FEATURE_SPEC>;
#[doc = "Register `feature` writer"]
pub type W = crate::W<FEATURE_SPEC>;
#[doc = "Field `PREEMPT` reader - Preemptive priority interrupt enable 0: Disabled 1: Enabled"]
pub type PREEMPT_R = crate::BitReader;
#[doc = "Field `PREEMPT` writer - Preemptive priority interrupt enable 0: Disabled 1: Enabled"]
pub type PREEMPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VECTORED` reader - Vector mode enable 0: Disabled 1: Enabled"]
pub type VECTORED_R = crate::BitReader;
#[doc = "Field `VECTORED` writer - Vector mode enable 0: Disabled 1: Enabled"]
pub type VECTORED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Preemptive priority interrupt enable 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn preempt(&self) -> PREEMPT_R {
        PREEMPT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Vector mode enable 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn vectored(&self) -> VECTORED_R {
        VECTORED_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Preemptive priority interrupt enable 0: Disabled 1: Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn preempt(&mut self) -> PREEMPT_W<FEATURE_SPEC> {
        PREEMPT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Vector mode enable 0: Disabled 1: Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn vectored(&mut self) -> VECTORED_W<FEATURE_SPEC> {
        VECTORED_W::new(self, 1)
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
#[doc = "Feature enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`feature::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`feature::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FEATURE_SPEC;
impl crate::RegisterSpec for FEATURE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`feature::R`](R) reader structure"]
impl crate::Readable for FEATURE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`feature::W`](W) writer structure"]
impl crate::Writable for FEATURE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets feature to value 0"]
impl crate::Resettable for FEATURE_SPEC {
    const RESET_VALUE: u32 = 0;
}
