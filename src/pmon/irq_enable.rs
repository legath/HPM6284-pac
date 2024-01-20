#[doc = "Register `IRQ_ENABLE` reader"]
pub type R = crate::R<IRQ_ENABLE_SPEC>;
#[doc = "Register `IRQ_ENABLE` writer"]
pub type W = crate::W<IRQ_ENABLE_SPEC>;
#[doc = "Field `ENABLE` reader - interrupt enable, each bit represents for one monitor 0: monitor interrupt disabled 1: monitor interrupt enabled"]
pub type ENABLE_R = crate::FieldReader;
#[doc = "Field `ENABLE` writer - interrupt enable, each bit represents for one monitor 0: monitor interrupt disabled 1: monitor interrupt enabled"]
pub type ENABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - interrupt enable, each bit represents for one monitor 0: monitor interrupt disabled 1: monitor interrupt enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - interrupt enable, each bit represents for one monitor 0: monitor interrupt disabled 1: monitor interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<IRQ_ENABLE_SPEC> {
        ENABLE_W::new(self, 0)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ_ENABLE_SPEC;
impl crate::RegisterSpec for IRQ_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_enable::R`](R) reader structure"]
impl crate::Readable for IRQ_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irq_enable::W`](W) writer structure"]
impl crate::Writable for IRQ_ENABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQ_ENABLE to value 0"]
impl crate::Resettable for IRQ_ENABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
