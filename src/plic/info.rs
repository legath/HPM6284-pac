#[doc = "Register `INFO` reader"]
pub type R = crate::R<INFO_SPEC>;
#[doc = "Register `INFO` writer"]
pub type W = crate::W<INFO_SPEC>;
#[doc = "Field `VERSION` reader - The version of the PLIC design"]
pub type VERSION_R = crate::FieldReader<u16>;
#[doc = "Field `MAX_PRIORITY` reader - The maximum priority supported"]
pub type MAX_PRIORITY_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The version of the PLIC design"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The maximum priority supported"]
    #[inline(always)]
    pub fn max_priority(&self) -> MAX_PRIORITY_R {
        MAX_PRIORITY_R::new(((self.bits >> 16) & 0xffff) as u16)
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
#[doc = "Version and the maximum priority\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INFO_SPEC;
impl crate::RegisterSpec for INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`info::R`](R) reader structure"]
impl crate::Readable for INFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`info::W`](W) writer structure"]
impl crate::Writable for INFO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INFO to value 0"]
impl crate::Resettable for INFO_SPEC {
    const RESET_VALUE: u32 = 0;
}
