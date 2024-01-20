#[doc = "Register `UNLOCK` reader"]
pub type R = crate::R<UNLOCK_SPEC>;
#[doc = "Register `UNLOCK` writer"]
pub type W = crate::W<UNLOCK_SPEC>;
#[doc = "Field `UNLOCK` reader - unlock word for fuse array operation write \"OPEN\" to unlock fuse array, write any other value will lock write to fuse. Please make sure 24M crystal is running and 2.5V LDO working properly"]
pub type UNLOCK_R = crate::FieldReader<u32>;
#[doc = "Field `UNLOCK` writer - unlock word for fuse array operation write \"OPEN\" to unlock fuse array, write any other value will lock write to fuse. Please make sure 24M crystal is running and 2.5V LDO working properly"]
pub type UNLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - unlock word for fuse array operation write \"OPEN\" to unlock fuse array, write any other value will lock write to fuse. Please make sure 24M crystal is running and 2.5V LDO working properly"]
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - unlock word for fuse array operation write \"OPEN\" to unlock fuse array, write any other value will lock write to fuse. Please make sure 24M crystal is running and 2.5V LDO working properly"]
    #[inline(always)]
    #[must_use]
    pub fn unlock(&mut self) -> UNLOCK_W<UNLOCK_SPEC> {
        UNLOCK_W::new(self, 0)
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
#[doc = "UNLOCK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`unlock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unlock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNLOCK_SPEC;
impl crate::RegisterSpec for UNLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`unlock::R`](R) reader structure"]
impl crate::Readable for UNLOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`unlock::W`](W) writer structure"]
impl crate::Writable for UNLOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UNLOCK to value 0"]
impl crate::Resettable for UNLOCK_SPEC {
    const RESET_VALUE: u32 = 0;
}
