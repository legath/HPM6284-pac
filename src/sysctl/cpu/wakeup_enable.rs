#[doc = "Register `WAKEUP_ENABLE[%s]` reader"]
pub type R = crate::R<WAKEUP_ENABLE_SPEC>;
#[doc = "Register `WAKEUP_ENABLE[%s]` writer"]
pub type W = crate::W<WAKEUP_ENABLE_SPEC>;
#[doc = "Field `ENABLE` reader - IRQ wakeup enable"]
pub type ENABLE_R = crate::FieldReader<u32>;
#[doc = "Field `ENABLE` writer - IRQ wakeup enable"]
pub type ENABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IRQ wakeup enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IRQ wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<WAKEUP_ENABLE_SPEC> {
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WAKEUP_ENABLE_SPEC;
impl crate::RegisterSpec for WAKEUP_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakeup_enable::R`](R) reader structure"]
impl crate::Readable for WAKEUP_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wakeup_enable::W`](W) writer structure"]
impl crate::Writable for WAKEUP_ENABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAKEUP_ENABLE[%s]
to value 0"]
impl crate::Resettable for WAKEUP_ENABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
