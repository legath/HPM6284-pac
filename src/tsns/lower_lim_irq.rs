#[doc = "Register `LOWER_LIM_IRQ` reader"]
pub type R = crate::R<LOWER_LIM_IRQ_SPEC>;
#[doc = "Register `LOWER_LIM_IRQ` writer"]
pub type W = crate::W<LOWER_LIM_IRQ_SPEC>;
#[doc = "Field `T` reader - Minimum temperature for compare"]
pub type T_R = crate::FieldReader<u32>;
#[doc = "Field `T` writer - Minimum temperature for compare"]
pub type T_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Minimum temperature for compare"]
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Minimum temperature for compare"]
    #[inline(always)]
    #[must_use]
    pub fn t(&mut self) -> T_W<LOWER_LIM_IRQ_SPEC> {
        T_W::new(self, 0)
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
#[doc = "Minimum temperature to interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lower_lim_irq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lower_lim_irq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOWER_LIM_IRQ_SPEC;
impl crate::RegisterSpec for LOWER_LIM_IRQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lower_lim_irq::R`](R) reader structure"]
impl crate::Readable for LOWER_LIM_IRQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lower_lim_irq::W`](W) writer structure"]
impl crate::Writable for LOWER_LIM_IRQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOWER_LIM_IRQ to value 0"]
impl crate::Resettable for LOWER_LIM_IRQ_SPEC {
    const RESET_VALUE: u32 = 0;
}
