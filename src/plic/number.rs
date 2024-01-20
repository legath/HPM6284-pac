#[doc = "Register `NUMBER` reader"]
pub type R = crate::R<NUMBER_SPEC>;
#[doc = "Register `NUMBER` writer"]
pub type W = crate::W<NUMBER_SPEC>;
#[doc = "Field `NUM_INTERRUPT` reader - The number of supported interrupt sources"]
pub type NUM_INTERRUPT_R = crate::FieldReader<u16>;
#[doc = "Field `NUM_TARGET` reader - The number of supported targets"]
pub type NUM_TARGET_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The number of supported interrupt sources"]
    #[inline(always)]
    pub fn num_interrupt(&self) -> NUM_INTERRUPT_R {
        NUM_INTERRUPT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The number of supported targets"]
    #[inline(always)]
    pub fn num_target(&self) -> NUM_TARGET_R {
        NUM_TARGET_R::new(((self.bits >> 16) & 0xffff) as u16)
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
#[doc = "Number of supported interrupt sources and targets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`number::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`number::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NUMBER_SPEC;
impl crate::RegisterSpec for NUMBER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`number::R`](R) reader structure"]
impl crate::Readable for NUMBER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`number::W`](W) writer structure"]
impl crate::Writable for NUMBER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NUMBER to value 0"]
impl crate::Resettable for NUMBER_SPEC {
    const RESET_VALUE: u32 = 0;
}
