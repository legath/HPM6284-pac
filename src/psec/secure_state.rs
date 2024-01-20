#[doc = "Register `SECURE_STATE` reader"]
pub type R = crate::R<SECURE_STATE_SPEC>;
#[doc = "Register `SECURE_STATE` writer"]
pub type W = crate::W<SECURE_STATE_SPEC>;
#[doc = "Field `PMIC_INS` reader - PMIC secure state one hot indicator 0: secure state is not in inspect state 1: secure state is in inspect state"]
pub type PMIC_INS_R = crate::BitReader;
#[doc = "Field `PMIC_INS` writer - PMIC secure state one hot indicator 0: secure state is not in inspect state 1: secure state is in inspect state"]
pub type PMIC_INS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMIC_SEC` reader - PMIC secure state one hot indicator 0: secure state is not in secure state 1: secure state is in secure state"]
pub type PMIC_SEC_R = crate::BitReader;
#[doc = "Field `PMIC_SEC` writer - PMIC secure state one hot indicator 0: secure state is not in secure state 1: secure state is in secure state"]
pub type PMIC_SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMIC_NSC` reader - PMIC secure state one hot indicator 0: secure state is not in non-secure state 1: secure state is in non-secure state"]
pub type PMIC_NSC_R = crate::BitReader;
#[doc = "Field `PMIC_NSC` writer - PMIC secure state one hot indicator 0: secure state is not in non-secure state 1: secure state is in non-secure state"]
pub type PMIC_NSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMIC_FAIL` reader - PMIC secure state one hot indicator 0: secure state is not in fail state 1: secure state is in fail state"]
pub type PMIC_FAIL_R = crate::BitReader;
#[doc = "Field `PMIC_FAIL` writer - PMIC secure state one hot indicator 0: secure state is not in fail state 1: secure state is in fail state"]
pub type PMIC_FAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALLOW_SEC` reader - Secure state allow 0: system is not healthy to enter secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter secure state"]
pub type ALLOW_SEC_R = crate::BitReader;
#[doc = "Field `ALLOW_NSC` reader - Non-secure state allow 0: system is not healthy to enter non-secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter non-secure state"]
pub type ALLOW_NSC_R = crate::BitReader;
impl R {
    #[doc = "Bit 4 - PMIC secure state one hot indicator 0: secure state is not in inspect state 1: secure state is in inspect state"]
    #[inline(always)]
    pub fn pmic_ins(&self) -> PMIC_INS_R {
        PMIC_INS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PMIC secure state one hot indicator 0: secure state is not in secure state 1: secure state is in secure state"]
    #[inline(always)]
    pub fn pmic_sec(&self) -> PMIC_SEC_R {
        PMIC_SEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PMIC secure state one hot indicator 0: secure state is not in non-secure state 1: secure state is in non-secure state"]
    #[inline(always)]
    pub fn pmic_nsc(&self) -> PMIC_NSC_R {
        PMIC_NSC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PMIC secure state one hot indicator 0: secure state is not in fail state 1: secure state is in fail state"]
    #[inline(always)]
    pub fn pmic_fail(&self) -> PMIC_FAIL_R {
        PMIC_FAIL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Secure state allow 0: system is not healthy to enter secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter secure state"]
    #[inline(always)]
    pub fn allow_sec(&self) -> ALLOW_SEC_R {
        ALLOW_SEC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Non-secure state allow 0: system is not healthy to enter non-secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter non-secure state"]
    #[inline(always)]
    pub fn allow_nsc(&self) -> ALLOW_NSC_R {
        ALLOW_NSC_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - PMIC secure state one hot indicator 0: secure state is not in inspect state 1: secure state is in inspect state"]
    #[inline(always)]
    #[must_use]
    pub fn pmic_ins(&mut self) -> PMIC_INS_W<SECURE_STATE_SPEC> {
        PMIC_INS_W::new(self, 4)
    }
    #[doc = "Bit 5 - PMIC secure state one hot indicator 0: secure state is not in secure state 1: secure state is in secure state"]
    #[inline(always)]
    #[must_use]
    pub fn pmic_sec(&mut self) -> PMIC_SEC_W<SECURE_STATE_SPEC> {
        PMIC_SEC_W::new(self, 5)
    }
    #[doc = "Bit 6 - PMIC secure state one hot indicator 0: secure state is not in non-secure state 1: secure state is in non-secure state"]
    #[inline(always)]
    #[must_use]
    pub fn pmic_nsc(&mut self) -> PMIC_NSC_W<SECURE_STATE_SPEC> {
        PMIC_NSC_W::new(self, 6)
    }
    #[doc = "Bit 7 - PMIC secure state one hot indicator 0: secure state is not in fail state 1: secure state is in fail state"]
    #[inline(always)]
    #[must_use]
    pub fn pmic_fail(&mut self) -> PMIC_FAIL_W<SECURE_STATE_SPEC> {
        PMIC_FAIL_W::new(self, 7)
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
#[doc = "Secure state\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secure_state::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secure_state::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECURE_STATE_SPEC;
impl crate::RegisterSpec for SECURE_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure_state::R`](R) reader structure"]
impl crate::Readable for SECURE_STATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`secure_state::W`](W) writer structure"]
impl crate::Writable for SECURE_STATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECURE_STATE to value 0"]
impl crate::Resettable for SECURE_STATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
