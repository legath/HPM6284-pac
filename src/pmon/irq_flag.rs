#[doc = "Register `IRQ_FLAG` reader"]
pub type R = crate::R<IRQ_FLAG_SPEC>;
#[doc = "Register `IRQ_FLAG` writer"]
pub type W = crate::W<IRQ_FLAG_SPEC>;
#[doc = "Field `FLAG` reader - interrupt flag, each bit represents for one monitor, write 1 to clear interrupt flag 0: no monitor interrupt 1: monitor interrupt happened"]
pub type FLAG_R = crate::FieldReader;
#[doc = "Field `FLAG` writer - interrupt flag, each bit represents for one monitor, write 1 to clear interrupt flag 0: no monitor interrupt 1: monitor interrupt happened"]
pub type FLAG_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - interrupt flag, each bit represents for one monitor, write 1 to clear interrupt flag 0: no monitor interrupt 1: monitor interrupt happened"]
    #[inline(always)]
    pub fn flag(&self) -> FLAG_R {
        FLAG_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - interrupt flag, each bit represents for one monitor, write 1 to clear interrupt flag 0: no monitor interrupt 1: monitor interrupt happened"]
    #[inline(always)]
    #[must_use]
    pub fn flag(&mut self) -> FLAG_W<IRQ_FLAG_SPEC> {
        FLAG_W::new(self, 0)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_flag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_flag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ_FLAG_SPEC;
impl crate::RegisterSpec for IRQ_FLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_flag::R`](R) reader structure"]
impl crate::Readable for IRQ_FLAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irq_flag::W`](W) writer structure"]
impl crate::Writable for IRQ_FLAG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQ_FLAG to value 0"]
impl crate::Resettable for IRQ_FLAG_SPEC {
    const RESET_VALUE: u32 = 0;
}
