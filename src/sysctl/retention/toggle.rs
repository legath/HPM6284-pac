#[doc = "Register `TOGGLE` reader"]
pub type R = crate::R<TOGGLE_SPEC>;
#[doc = "Register `TOGGLE` writer"]
pub type W = crate::W<TOGGLE_SPEC>;
#[doc = "Field `LINK` reader - retention setting while CPU0 enter stop mode, each bit represents a resource 0: no effect 1: toggle the result that whether the resource is kept on while CPU0 stop before"]
pub type LINK_R = crate::FieldReader<u16>;
#[doc = "Field `LINK` writer - retention setting while CPU0 enter stop mode, each bit represents a resource 0: no effect 1: toggle the result that whether the resource is kept on while CPU0 stop before"]
pub type LINK_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - retention setting while CPU0 enter stop mode, each bit represents a resource 0: no effect 1: toggle the result that whether the resource is kept on while CPU0 stop before"]
    #[inline(always)]
    pub fn link(&self) -> LINK_R {
        LINK_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - retention setting while CPU0 enter stop mode, each bit represents a resource 0: no effect 1: toggle the result that whether the resource is kept on while CPU0 stop before"]
    #[inline(always)]
    #[must_use]
    pub fn link(&mut self) -> LINK_W<TOGGLE_SPEC> {
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
#[doc = "Retention Contol\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`toggle::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`toggle::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOGGLE_SPEC;
impl crate::RegisterSpec for TOGGLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`toggle::R`](R) reader structure"]
impl crate::Readable for TOGGLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`toggle::W`](W) writer structure"]
impl crate::Writable for TOGGLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOGGLE to value 0"]
impl crate::Resettable for TOGGLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
