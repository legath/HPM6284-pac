#[doc = "Register `POR_CONFIG` reader"]
pub type R = crate::R<POR_CONFIG_SPEC>;
#[doc = "Register `POR_CONFIG` writer"]
pub type W = crate::W<POR_CONFIG_SPEC>;
#[doc = "Field `RETENTION` reader - retention battery domain setting 0: battery reset on reset pin reset happen 1: battery domain retention when reset pin reset happen"]
pub type RETENTION_R = crate::BitReader;
#[doc = "Field `RETENTION` writer - retention battery domain setting 0: battery reset on reset pin reset happen 1: battery domain retention when reset pin reset happen"]
pub type RETENTION_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - retention battery domain setting 0: battery reset on reset pin reset happen 1: battery domain retention when reset pin reset happen"]
    #[inline(always)]
    pub fn retention(&self) -> RETENTION_R {
        RETENTION_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - retention battery domain setting 0: battery reset on reset pin reset happen 1: battery domain retention when reset pin reset happen"]
    #[inline(always)]
    #[must_use]
    pub fn retention(&mut self) -> RETENTION_W<POR_CONFIG_SPEC> {
        RETENTION_W::new(self, 0)
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
#[doc = "Power on reset config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`por_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`por_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POR_CONFIG_SPEC;
impl crate::RegisterSpec for POR_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`por_config::R`](R) reader structure"]
impl crate::Readable for POR_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`por_config::W`](W) writer structure"]
impl crate::Writable for POR_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POR_CONFIG to value 0"]
impl crate::Resettable for POR_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
