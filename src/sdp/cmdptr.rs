#[doc = "Register `CMDPTR` reader"]
pub type R = crate::R<CMDPTR_SPEC>;
#[doc = "Register `CMDPTR` writer"]
pub type W = crate::W<CMDPTR_SPEC>;
#[doc = "Field `CMDPTR` reader - current command addresses the register points to the multiword descriptor that is to be executed (or is currently being executed)"]
pub type CMDPTR_R = crate::FieldReader<u32>;
#[doc = "Field `CMDPTR` writer - current command addresses the register points to the multiword descriptor that is to be executed (or is currently being executed)"]
pub type CMDPTR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - current command addresses the register points to the multiword descriptor that is to be executed (or is currently being executed)"]
    #[inline(always)]
    pub fn cmdptr(&self) -> CMDPTR_R {
        CMDPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - current command addresses the register points to the multiword descriptor that is to be executed (or is currently being executed)"]
    #[inline(always)]
    #[must_use]
    pub fn cmdptr(&mut self) -> CMDPTR_W<CMDPTR_SPEC> {
        CMDPTR_W::new(self, 0)
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
#[doc = "Command Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdptr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdptr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDPTR_SPEC;
impl crate::RegisterSpec for CMDPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdptr::R`](R) reader structure"]
impl crate::Readable for CMDPTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmdptr::W`](W) writer structure"]
impl crate::Writable for CMDPTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDPTR to value 0"]
impl crate::Resettable for CMDPTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
