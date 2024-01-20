#[doc = "Register `CLEAR` reader"]
pub type R = crate::R<CLEAR_SPEC>;
#[doc = "Register `CLEAR` writer"]
pub type W = crate::W<CLEAR_SPEC>;
#[doc = "Field `LINK` reader - retention setting while CPU0 enter stop mode, each bit represents a resource 0: no effect 1: no keep"]
pub type LINK_R = crate::FieldReader<u16>;
#[doc = "Field `LINK` writer - retention setting while CPU0 enter stop mode, each bit represents a resource 0: no effect 1: no keep"]
pub type LINK_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - retention setting while CPU0 enter stop mode, each bit represents a resource 0: no effect 1: no keep"]
    #[inline(always)]
    pub fn link(&self) -> LINK_R {
        LINK_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - retention setting while CPU0 enter stop mode, each bit represents a resource 0: no effect 1: no keep"]
    #[inline(always)]
    #[must_use]
    pub fn link(&mut self) -> LINK_W<CLEAR_SPEC> {
        LINK_W::new(self, 0)
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
#[doc = "Retention Contol\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
