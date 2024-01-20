#[doc = "Register `high_limit` reader"]
pub type R = crate::R<HIGH_LIMIT_SPEC>;
#[doc = "Register `high_limit` writer"]
pub type W = crate::W<HIGH_LIMIT_SPEC>;
#[doc = "Field `FREQUENCY` reader - upper frequency"]
pub type FREQUENCY_R = crate::FieldReader<u32>;
#[doc = "Field `FREQUENCY` writer - upper frequency"]
pub type FREQUENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - upper frequency"]
    #[inline(always)]
    pub fn frequency(&self) -> FREQUENCY_R {
        FREQUENCY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - upper frequency"]
    #[inline(always)]
    #[must_use]
    pub fn frequency(&mut self) -> FREQUENCY_W<HIGH_LIMIT_SPEC> {
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
#[doc = "Clock upper limit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`high_limit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`high_limit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIGH_LIMIT_SPEC;
impl crate::RegisterSpec for HIGH_LIMIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`high_limit::R`](R) reader structure"]
impl crate::Readable for HIGH_LIMIT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`high_limit::W`](W) writer structure"]
impl crate::Writable for HIGH_LIMIT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets high_limit to value 0"]
impl crate::Resettable for HIGH_LIMIT_SPEC {
    const RESET_VALUE: u32 = 0;
}
