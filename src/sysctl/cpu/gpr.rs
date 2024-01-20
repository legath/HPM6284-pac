#[doc = "Register `GPR[%s]` reader"]
pub type R = crate::R<GPR_SPEC>;
#[doc = "Register `GPR[%s]` writer"]
pub type W = crate::W<GPR_SPEC>;
#[doc = "Field `GPR` reader - register for software to handle resume, can save resume address or status"]
pub type GPR_R = crate::FieldReader<u32>;
#[doc = "Field `GPR` writer - register for software to handle resume, can save resume address or status"]
pub type GPR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - register for software to handle resume, can save resume address or status"]
    #[inline(always)]
    pub fn gpr(&self) -> GPR_R {
        GPR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - register for software to handle resume, can save resume address or status"]
    #[inline(always)]
    #[must_use]
    pub fn gpr(&mut self) -> GPR_W<GPR_SPEC> {
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPR_SPEC;
impl crate::RegisterSpec for GPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpr::R`](R) reader structure"]
impl crate::Readable for GPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpr::W`](W) writer structure"]
impl crate::Writable for GPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPR[%s]
to value 0"]
impl crate::Resettable for GPR_SPEC {
    const RESET_VALUE: u32 = 0;
}
