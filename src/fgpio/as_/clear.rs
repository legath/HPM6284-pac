#[doc = "Register `CLEAR` reader"]
pub type R = crate::R<CLEAR_SPEC>;
#[doc = "Register `CLEAR` writer"]
pub type W = crate::W<CLEAR_SPEC>;
#[doc = "Field `IRQ_ASYNC` reader - GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
pub type IRQ_ASYNC_R = crate::FieldReader<u32>;
#[doc = "Field `IRQ_ASYNC` writer - GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
pub type IRQ_ASYNC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    #[inline(always)]
    pub fn irq_async(&self) -> IRQ_ASYNC_R {
        IRQ_ASYNC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    #[inline(always)]
    #[must_use]
    pub fn irq_async(&mut self) -> IRQ_ASYNC_W<CLEAR_SPEC> {
        IRQ_ASYNC_W::new(self, 0)
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
#[doc = "GPIO interrupt asynchronous clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLEAR_SPEC;
impl crate::RegisterSpec for CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clear::R`](R) reader structure"]
impl crate::Readable for CLEAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clear::W`](W) writer structure"]
impl crate::Writable for CLEAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLEAR to value 0"]
impl crate::Resettable for CLEAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
