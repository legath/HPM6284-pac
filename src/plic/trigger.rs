#[doc = "Register `TRIGGER[%s]` reader"]
pub type R = crate::R<TRIGGER_SPEC>;
#[doc = "Register `TRIGGER[%s]` writer"]
pub type W = crate::W<TRIGGER_SPEC>;
#[doc = "Field `INTERRUPT` reader - The interrupt trigger type of interrupt sources. Every interrupt source occupies 1 bit. 0: Level-triggered interrupt 1: Edge-triggered interrupt"]
pub type INTERRUPT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The interrupt trigger type of interrupt sources. Every interrupt source occupies 1 bit. 0: Level-triggered interrupt 1: Edge-triggered interrupt"]
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new(self.bits)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trigger::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigger::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRIGGER_SPEC;
impl crate::RegisterSpec for TRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trigger::R`](R) reader structure"]
impl crate::Readable for TRIGGER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trigger::W`](W) writer structure"]
impl crate::Writable for TRIGGER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRIGGER[%s]
to value 0"]
impl crate::Resettable for TRIGGER_SPEC {
    const RESET_VALUE: u32 = 0;
}
