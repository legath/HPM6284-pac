#[doc = "Register `SHADOW[%s]` reader"]
pub type R = crate::R<SHADOW_SPEC>;
#[doc = "Register `SHADOW[%s]` writer"]
pub type W = crate::W<SHADOW_SPEC>;
#[doc = "Field `SHADOW` reader - shadow register of fuse for pmic area for PMIC, index valid for 0-15, for SOC index valid for 16-128"]
pub type SHADOW_R = crate::FieldReader<u32>;
#[doc = "Field `SHADOW` writer - shadow register of fuse for pmic area for PMIC, index valid for 0-15, for SOC index valid for 16-128"]
pub type SHADOW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - shadow register of fuse for pmic area for PMIC, index valid for 0-15, for SOC index valid for 16-128"]
    #[inline(always)]
    pub fn shadow(&self) -> SHADOW_R {
        SHADOW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - shadow register of fuse for pmic area for PMIC, index valid for 0-15, for SOC index valid for 16-128"]
    #[inline(always)]
    #[must_use]
    pub fn shadow(&mut self) -> SHADOW_W<SHADOW_SPEC> {
        SHADOW_W::new(self, 0)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shadow::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shadow::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHADOW_SPEC;
impl crate::RegisterSpec for SHADOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shadow::R`](R) reader structure"]
impl crate::Readable for SHADOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shadow::W`](W) writer structure"]
impl crate::Writable for SHADOW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHADOW[%s]
to value 0"]
impl crate::Resettable for SHADOW_SPEC {
    const RESET_VALUE: u32 = 0;
}
