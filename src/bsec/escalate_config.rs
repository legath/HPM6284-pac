#[doc = "Register `ESCALATE_CONFIG` reader"]
pub type R = crate::R<ESCALATE_CONFIG_SPEC>;
#[doc = "Register `ESCALATE_CONFIG` writer"]
pub type W = crate::W<ESCALATE_CONFIG_SPEC>;
#[doc = "Field `SEC_VIO_CFG` reader - configuration of secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate"]
pub type SEC_VIO_CFG_R = crate::FieldReader<u16>;
#[doc = "Field `SEC_VIO_CFG` writer - configuration of secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate"]
pub type SEC_VIO_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `LOCK_SEC` reader - Lock bit secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified1: register locked, write access to the configuration is ignored"]
pub type LOCK_SEC_R = crate::BitReader;
#[doc = "Field `LOCK_SEC` writer - Lock bit secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified1: register locked, write access to the configuration is ignored"]
pub type LOCK_SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSC_VIO_CFG` reader - configuration of non-secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate"]
pub type NSC_VIO_CFG_R = crate::FieldReader<u16>;
#[doc = "Field `NSC_VIO_CFG` writer - configuration of non-secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate"]
pub type NSC_VIO_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `LOCK_NSC` reader - Lock bit non-secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored"]
pub type LOCK_NSC_R = crate::BitReader;
#[doc = "Field `LOCK_NSC` writer - Lock bit non-secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored"]
pub type LOCK_NSC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - configuration of secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate"]
    #[inline(always)]
    pub fn sec_vio_cfg(&self) -> SEC_VIO_CFG_R {
        SEC_VIO_CFG_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Lock bit secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified1: register locked, write access to the configuration is ignored"]
    #[inline(always)]
    pub fn lock_sec(&self) -> LOCK_SEC_R {
        LOCK_SEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:30 - configuration of non-secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate"]
    #[inline(always)]
    pub fn nsc_vio_cfg(&self) -> NSC_VIO_CFG_R {
        NSC_VIO_CFG_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Lock bit non-secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored"]
    #[inline(always)]
    pub fn lock_nsc(&self) -> LOCK_NSC_R {
        LOCK_NSC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - configuration of secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate"]
    #[inline(always)]
    #[must_use]
    pub fn sec_vio_cfg(&mut self) -> SEC_VIO_CFG_W<ESCALATE_CONFIG_SPEC> {
        SEC_VIO_CFG_W::new(self, 0)
    }
    #[doc = "Bit 15 - Lock bit secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified1: register locked, write access to the configuration is ignored"]
    #[inline(always)]
    #[must_use]
    pub fn lock_sec(&mut self) -> LOCK_SEC_W<ESCALATE_CONFIG_SPEC> {
        LOCK_SEC_W::new(self, 15)
    }
    #[doc = "Bits 16:30 - configuration of non-secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate"]
    #[inline(always)]
    #[must_use]
    pub fn nsc_vio_cfg(&mut self) -> NSC_VIO_CFG_W<ESCALATE_CONFIG_SPEC> {
        NSC_VIO_CFG_W::new(self, 16)
    }
    #[doc = "Bit 31 - Lock bit non-secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored"]
    #[inline(always)]
    #[must_use]
    pub fn lock_nsc(&mut self) -> LOCK_NSC_W<ESCALATE_CONFIG_SPEC> {
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
#[doc = "Escalate behavior on security event\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`escalate_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`escalate_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ESCALATE_CONFIG_SPEC;
impl crate::RegisterSpec for ESCALATE_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`escalate_config::R`](R) reader structure"]
impl crate::Readable for ESCALATE_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`escalate_config::W`](W) writer structure"]
impl crate::Writable for ESCALATE_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESCALATE_CONFIG to value 0"]
impl crate::Resettable for ESCALATE_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
