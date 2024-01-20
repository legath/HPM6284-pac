#[doc = "Register `PMIC_GPR01` reader"]
pub type R = crate::R<PMIC_GPR01_SPEC>;
#[doc = "Register `PMIC_GPR01` writer"]
pub type W = crate::W<PMIC_GPR01_SPEC>;
#[doc = "Field `GPR` reader - Generic control"]
pub type GPR_R = crate::FieldReader<u32>;
#[doc = "Field `GPR` writer - Generic control"]
pub type GPR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Generic control"]
    #[inline(always)]
    pub fn gpr(&self) -> GPR_R {
        GPR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Generic control"]
    #[inline(always)]
    #[must_use]
    pub fn gpr(&mut self) -> GPR_W<PMIC_GPR01_SPEC> {
        GPR_W::new(self, 0)
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
#[doc = "Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmic_gpr01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmic_gpr01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMIC_GPR01_SPEC;
impl crate::RegisterSpec for PMIC_GPR01_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmic_gpr01::R`](R) reader structure"]
impl crate::Readable for PMIC_GPR01_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmic_gpr01::W`](W) writer structure"]
impl crate::Writable for PMIC_GPR01_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMIC_GPR01 to value 0"]
impl crate::Resettable for PMIC_GPR01_SPEC {
    const RESET_VALUE: u32 = 0;
}
