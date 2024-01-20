#[doc = "Register `ATB` reader"]
pub type R = crate::R<ATB_SPEC>;
#[doc = "Register `ATB` writer"]
pub type W = crate::W<ATB_SPEC>;
#[doc = "Field `TB` reader - timebase for timestamp generation 31-0"]
pub type TB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - timebase for timestamp generation 31-0"]
    #[inline(always)]
    pub fn tb(&self) -> TB_R {
        TB_R::new(self.bits)
    }
}
impl W {
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
#[doc = "actual timebase\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATB_SPEC;
impl crate::RegisterSpec for ATB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atb::R`](R) reader structure"]
impl crate::Readable for ATB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`atb::W`](W) writer structure"]
impl crate::Writable for ATB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATB to value 0"]
impl crate::Resettable for ATB_SPEC {
    const RESET_VALUE: u32 = 0;
}
