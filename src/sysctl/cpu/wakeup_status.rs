#[doc = "Register `WAKEUP_STATUS[%s]` reader"]
pub type R = crate::R<WAKEUP_STATUS_SPEC>;
#[doc = "Register `WAKEUP_STATUS[%s]` writer"]
pub type W = crate::W<WAKEUP_STATUS_SPEC>;
#[doc = "Field `STATUS` reader - IRQ values"]
pub type STATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - IRQ values"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(self.bits)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WAKEUP_STATUS_SPEC;
impl crate::RegisterSpec for WAKEUP_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakeup_status::R`](R) reader structure"]
impl crate::Readable for WAKEUP_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wakeup_status::W`](W) writer structure"]
impl crate::Writable for WAKEUP_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAKEUP_STATUS[%s]
to value 0"]
impl crate::Resettable for WAKEUP_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
