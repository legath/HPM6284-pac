#[doc = "Register `TRACK_TARGET` reader"]
pub type R = crate::R<TRACK_TARGET_SPEC>;
#[doc = "Register `TRACK_TARGET` writer"]
pub type W = crate::W<TRACK_TARGET_SPEC>;
#[doc = "Field `TARGET` reader - Target frequency multiplier of divided source"]
pub type TARGET_R = crate::FieldReader<u16>;
#[doc = "Field `TARGET` writer - Target frequency multiplier of divided source"]
pub type TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PRE_DIV` reader - Divider for reference source"]
pub type PRE_DIV_R = crate::FieldReader<u16>;
#[doc = "Field `PRE_DIV` writer - Divider for reference source"]
pub type PRE_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Target frequency multiplier of divided source"]
    #[inline(always)]
    pub fn target(&self) -> TARGET_R {
        TARGET_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Divider for reference source"]
    #[inline(always)]
    pub fn pre_div(&self) -> PRE_DIV_R {
        PRE_DIV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Target frequency multiplier of divided source"]
    #[inline(always)]
    #[must_use]
    pub fn target(&mut self) -> TARGET_W<TRACK_TARGET_SPEC> {
        TARGET_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Divider for reference source"]
    #[inline(always)]
    #[must_use]
    pub fn pre_div(&mut self) -> PRE_DIV_W<TRACK_TARGET_SPEC> {
        PRE_DIV_W::new(self, 16)
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
#[doc = "RC 24M track target\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`track_target::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`track_target::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRACK_TARGET_SPEC;
impl crate::RegisterSpec for TRACK_TARGET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`track_target::R`](R) reader structure"]
impl crate::Readable for TRACK_TARGET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`track_target::W`](W) writer structure"]
impl crate::Writable for TRACK_TARGET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRACK_TARGET to value 0"]
impl crate::Resettable for TRACK_TARGET_SPEC {
    const RESET_VALUE: u32 = 0;
}
