#[doc = "Register `tarh` reader"]
pub type R = crate::R<TARH_SPEC>;
#[doc = "Register `tarh` writer"]
pub type W = crate::W<TARH_SPEC>;
#[doc = "Field `TARGET_TIME_HIGH` reader - used for generate compare signal if enabled"]
pub type TARGET_TIME_HIGH_R = crate::FieldReader<u32>;
#[doc = "Field `TARGET_TIME_HIGH` writer - used for generate compare signal if enabled"]
pub type TARGET_TIME_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - used for generate compare signal if enabled"]
    #[inline(always)]
    pub fn target_time_high(&self) -> TARGET_TIME_HIGH_R {
        TARGET_TIME_HIGH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - used for generate compare signal if enabled"]
    #[inline(always)]
    #[must_use]
    pub fn target_time_high(&mut self) -> TARGET_TIME_HIGH_W<TARH_SPEC> {
        TARGET_TIME_HIGH_W::new(self, 0)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tarh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tarh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TARH_SPEC;
impl crate::RegisterSpec for TARH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tarh::R`](R) reader structure"]
impl crate::Readable for TARH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tarh::W`](W) writer structure"]
impl crate::Writable for TARH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tarh to value 0"]
impl crate::Resettable for TARH_SPEC {
    const RESET_VALUE: u32 = 0;
}
