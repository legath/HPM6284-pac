#[doc = "Register `CHN_CFG_ACTIVE[%s]` reader"]
pub type R = crate::R<CHN_CFG_ACTIVE_SPEC>;
#[doc = "Register `CHN_CFG_ACTIVE[%s]` writer"]
pub type W = crate::W<CHN_CFG_ACTIVE_SPEC>;
#[doc = "Field `CFG_ACTIVE` reader - write 0xF00D to enable all setting. Otherwire, all setting inactive."]
pub type CFG_ACTIVE_R = crate::FieldReader<u16>;
#[doc = "Field `CFG_ACTIVE` writer - write 0xF00D to enable all setting. Otherwire, all setting inactive."]
pub type CFG_ACTIVE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - write 0xF00D to enable all setting. Otherwire, all setting inactive."]
    #[inline(always)]
    pub fn cfg_active(&self) -> CFG_ACTIVE_R {
        CFG_ACTIVE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - write 0xF00D to enable all setting. Otherwire, all setting inactive."]
    #[inline(always)]
    #[must_use]
    pub fn cfg_active(&mut self) -> CFG_ACTIVE_W<CHN_CFG_ACTIVE_SPEC> {
        CFG_ACTIVE_W::new(self, 0)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chn_cfg_active::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chn_cfg_active::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHN_CFG_ACTIVE_SPEC;
impl crate::RegisterSpec for CHN_CFG_ACTIVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chn_cfg_active::R`](R) reader structure"]
impl crate::Readable for CHN_CFG_ACTIVE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chn_cfg_active::W`](W) writer structure"]
impl crate::Writable for CHN_CFG_ACTIVE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHN_CFG_ACTIVE[%s]
to value 0"]
impl crate::Resettable for CHN_CFG_ACTIVE_SPEC {
    const RESET_VALUE: u32 = 0;
}
