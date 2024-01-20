#[doc = "Register `low_limit` reader"]
pub type R = crate::R<LOW_LIMIT_SPEC>;
#[doc = "Register `low_limit` writer"]
pub type W = crate::W<LOW_LIMIT_SPEC>;
#[doc = "Field `FREQUENCY` reader - lower frequency"]
pub type FREQUENCY_R = crate::FieldReader<u32>;
#[doc = "Field `FREQUENCY` writer - lower frequency"]
pub type FREQUENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - lower frequency"]
    #[inline(always)]
    pub fn frequency(&self) -> FREQUENCY_R {
        FREQUENCY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - lower frequency"]
    #[inline(always)]
    #[must_use]
    pub fn frequency(&mut self) -> FREQUENCY_W<LOW_LIMIT_SPEC> {
        FREQUENCY_W::new(self, 0)
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
#[doc = "Clock lower limit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`low_limit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`low_limit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOW_LIMIT_SPEC;
impl crate::RegisterSpec for LOW_LIMIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`low_limit::R`](R) reader structure"]
impl crate::Readable for LOW_LIMIT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`low_limit::W`](W) writer structure"]
impl crate::Writable for LOW_LIMIT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets low_limit to value 0xffff_ffff"]
impl crate::Resettable for LOW_LIMIT_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
