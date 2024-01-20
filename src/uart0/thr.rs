#[doc = "Register `THR` reader"]
pub type R = crate::R<THR_SPEC>;
#[doc = "Register `THR` writer"]
pub type W = crate::W<THR_SPEC>;
#[doc = "Field `THR` writer - Transmit data write port"]
pub type THR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Transmit data write port"]
    #[inline(always)]
    #[must_use]
    pub fn thr(&mut self) -> THR_W<THR_SPEC> {
        THR_W::new(self, 0)
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
#[doc = "Transmitter Holding Register (when DLAB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`thr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THR_SPEC;
impl crate::RegisterSpec for THR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`thr::R`](R) reader structure"]
impl crate::Readable for THR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`thr::W`](W) writer structure"]
impl crate::Writable for THR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets THR to value 0"]
impl crate::Resettable for THR_SPEC {
    const RESET_VALUE: u32 = 0;
}
