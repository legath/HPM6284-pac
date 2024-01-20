#[doc = "Register `VIOLATION_CONFIG` reader"]
pub type R = crate::R<VIOLATION_CONFIG_SPEC>;
#[doc = "Register `VIOLATION_CONFIG` writer"]
pub type W = crate::W<VIOLATION_CONFIG_SPEC>;
#[doc = "Field `SEC_VIO_CFG` reader - configuration of secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation"]
pub type SEC_VIO_CFG_R = crate::FieldReader<u16>;
#[doc = "Field `SEC_VIO_CFG` writer - configuration of secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation"]
pub type SEC_VIO_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `LOCK_SEC` reader - Lock bit secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored"]
pub type LOCK_SEC_R = crate::BitReader;
#[doc = "Field `LOCK_SEC` writer - Lock bit secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored"]
pub type LOCK_SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSC_VIO_CFG` reader - configuration of non-secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation"]
pub type NSC_VIO_CFG_R = crate::FieldReader<u16>;
#[doc = "Field `NSC_VIO_CFG` writer - configuration of non-secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation"]
pub type NSC_VIO_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `LOCK_NSC` reader - Lock bit non-secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored"]
pub type LOCK_NSC_R = crate::BitReader;
#[doc = "Field `LOCK_NSC` writer - Lock bit non-secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored"]
pub type LOCK_NSC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - configuration of secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation"]
    #[inline(always)]
    pub fn sec_vio_cfg(&self) -> SEC_VIO_CFG_R {
        SEC_VIO_CFG_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Lock bit secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored"]
    #[inline(always)]
    pub fn lock_sec(&self) -> LOCK_SEC_R {
        LOCK_SEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:30 - configuration of non-secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation"]
    #[inline(always)]
    pub fn nsc_vio_cfg(&self) -> NSC_VIO_CFG_R {
        NSC_VIO_CFG_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Lock bit non-secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored"]
    #[inline(always)]
    pub fn lock_nsc(&self) -> LOCK_NSC_R {
        LOCK_NSC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - configuration of secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation"]
    #[inline(always)]
    #[must_use]
    pub fn sec_vio_cfg(&mut self) -> SEC_VIO_CFG_W<VIOLATION_CONFIG_SPEC> {
        SEC_VIO_CFG_W::new(self, 0)
    }
    #[doc = "Bit 15 - Lock bit secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored"]
    #[inline(always)]
    #[must_use]
    pub fn lock_sec(&mut self) -> LOCK_SEC_W<VIOLATION_CONFIG_SPEC> {
        LOCK_SEC_W::new(self, 15)
    }
    #[doc = "Bits 16:30 - configuration of non-secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation"]
    #[inline(always)]
    #[must_use]
    pub fn nsc_vio_cfg(&mut self) -> NSC_VIO_CFG_W<VIOLATION_CONFIG_SPEC> {
        NSC_VIO_CFG_W::new(self, 16)
    }
    #[doc = "Bit 31 - Lock bit non-secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored"]
    #[inline(always)]
    #[must_use]
    pub fn lock_nsc(&mut self) -> LOCK_NSC_W<VIOLATION_CONFIG_SPEC> {
        LOCK_NSC_W::new(self, 31)
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
#[doc = "Security violation config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`violation_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`violation_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VIOLATION_CONFIG_SPEC;
impl crate::RegisterSpec for VIOLATION_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`violation_config::R`](R) reader structure"]
impl crate::Readable for VIOLATION_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`violation_config::W`](W) writer structure"]
impl crate::Writable for VIOLATION_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VIOLATION_CONFIG to value 0"]
impl crate::Resettable for VIOLATION_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
