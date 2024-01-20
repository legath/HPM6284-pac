#[doc = "Register `CLAIM` reader"]
pub type R = crate::R<CLAIM_SPEC>;
#[doc = "Register `CLAIM` writer"]
pub type W = crate::W<CLAIM_SPEC>;
#[doc = "Field `INTERRUPT_ID` reader - On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed)."]
pub type INTERRUPT_ID_R = crate::FieldReader<u16>;
#[doc = "Field `INTERRUPT_ID` writer - On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed)."]
pub type INTERRUPT_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed)."]
    #[inline(always)]
    pub fn interrupt_id(&self) -> INTERRUPT_ID_R {
        INTERRUPT_ID_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed)."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_id(&mut self) -> INTERRUPT_ID_W<CLAIM_SPEC> {
        INTERRUPT_ID_W::new(self, 0)
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
#[doc = "Target claim and complete\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLAIM_SPEC;
impl crate::RegisterSpec for CLAIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`claim::R`](R) reader structure"]
impl crate::Readable for CLAIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`claim::W`](W) writer structure"]
impl crate::Writable for CLAIM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLAIM to value 0"]
impl crate::Resettable for CLAIM_SPEC {
    const RESET_VALUE: u32 = 0;
}
