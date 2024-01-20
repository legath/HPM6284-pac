#[doc = "Register `prd_result` reader"]
pub type R = crate::R<PRD_RESULT_SPEC>;
#[doc = "Register `prd_result` writer"]
pub type W = crate::W<PRD_RESULT_SPEC>;
#[doc = "Field `CHAN_RESULT` reader - adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
pub type CHAN_RESULT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
    #[inline(always)]
    pub fn chan_result(&self) -> CHAN_RESULT_R {
        CHAN_RESULT_R::new((self.bits & 0xffff) as u16)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prd_result::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prd_result::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRD_RESULT_SPEC;
impl crate::RegisterSpec for PRD_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prd_result::R`](R) reader structure"]
impl crate::Readable for PRD_RESULT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prd_result::W`](W) writer structure"]
impl crate::Writable for PRD_RESULT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets prd_result to value 0"]
impl crate::Resettable for PRD_RESULT_SPEC {
    const RESET_VALUE: u32 = 0;
}
