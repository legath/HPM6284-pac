#[doc = "Register `SECURE_STATE_CONFIG` reader"]
pub type R = crate::R<SECURE_STATE_CONFIG_SPEC>;
#[doc = "Register `SECURE_STATE_CONFIG` writer"]
pub type W = crate::W<SECURE_STATE_CONFIG_SPEC>;
#[doc = "Field `ALLOW_RESTART` reader - allow secure state restart from fail state 0: restart is not allowed, only hardware reset can recover secure state 1: software is allowed to switch to inspect state from fail state"]
pub type ALLOW_RESTART_R = crate::BitReader;
#[doc = "Field `ALLOW_RESTART` writer - allow secure state restart from fail state 0: restart is not allowed, only hardware reset can recover secure state 1: software is allowed to switch to inspect state from fail state"]
pub type ALLOW_RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - Lock bit of allow restart setting, once locked, lock bit itself and configuration register will keep value until next reset 0: not locked, register can be modified 1: register locked, write access to the register is ignored"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - Lock bit of allow restart setting, once locked, lock bit itself and configuration register will keep value until next reset 0: not locked, register can be modified 1: register locked, write access to the register is ignored"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - allow secure state restart from fail state 0: restart is not allowed, only hardware reset can recover secure state 1: software is allowed to switch to inspect state from fail state"]
    #[inline(always)]
    pub fn allow_restart(&self) -> ALLOW_RESTART_R {
        ALLOW_RESTART_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Lock bit of allow restart setting, once locked, lock bit itself and configuration register will keep value until next reset 0: not locked, register can be modified 1: register locked, write access to the register is ignored"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - allow secure state restart from fail state 0: restart is not allowed, only hardware reset can recover secure state 1: software is allowed to switch to inspect state from fail state"]
    #[inline(always)]
    #[must_use]
    pub fn allow_restart(&mut self) -> ALLOW_RESTART_W<SECURE_STATE_CONFIG_SPEC> {
        ALLOW_RESTART_W::new(self, 0)
    }
    #[doc = "Bit 3 - Lock bit of allow restart setting, once locked, lock bit itself and configuration register will keep value until next reset 0: not locked, register can be modified 1: register locked, write access to the register is ignored"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<SECURE_STATE_CONFIG_SPEC> {
        LOCK_W::new(self, 3)
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
#[doc = "secure state configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secure_state_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secure_state_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECURE_STATE_CONFIG_SPEC;
impl crate::RegisterSpec for SECURE_STATE_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure_state_config::R`](R) reader structure"]
impl crate::Readable for SECURE_STATE_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`secure_state_config::W`](W) writer structure"]
impl crate::Writable for SECURE_STATE_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECURE_STATE_CONFIG to value 0"]
impl crate::Resettable for SECURE_STATE_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
