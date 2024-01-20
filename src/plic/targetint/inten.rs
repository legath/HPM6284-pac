#[doc = "Register `INTEN[%s]` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN[%s]` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `INTERRUPT` reader - The interrupt enable bit for interrupt. Every interrupt source occupies 1 bit."]
pub type INTERRUPT_R = crate::FieldReader<u32>;
#[doc = "Field `INTERRUPT` writer - The interrupt enable bit for interrupt. Every interrupt source occupies 1 bit."]
pub type INTERRUPT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The interrupt enable bit for interrupt. Every interrupt source occupies 1 bit."]
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The interrupt enable bit for interrupt. Every interrupt source occupies 1 bit."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt(&mut self) -> INTERRUPT_W<INTEN_SPEC> {
        INTERRUPT_W::new(self, 0)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN[%s]
to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
