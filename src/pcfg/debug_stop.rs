#[doc = "Register `DEBUG_STOP` reader"]
pub type R = crate::R<DEBUG_STOP_SPEC>;
#[doc = "Register `DEBUG_STOP` writer"]
pub type W = crate::W<DEBUG_STOP_SPEC>;
#[doc = "Field `CPU0` reader - Stop peripheral when CPU0 enter debug mode 0: peripheral keep running when CPU0 in debug mode 1: peripheral enter debug mode when CPU0 enter debug"]
pub type CPU0_R = crate::BitReader;
#[doc = "Field `CPU0` writer - Stop peripheral when CPU0 enter debug mode 0: peripheral keep running when CPU0 in debug mode 1: peripheral enter debug mode when CPU0 enter debug"]
pub type CPU0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU1` reader - Stop peripheral when CPU1 enter debug mode 0: peripheral keep running when CPU1 in debug mode 1: peripheral enter debug mode when CPU1 enter debug"]
pub type CPU1_R = crate::BitReader;
#[doc = "Field `CPU1` writer - Stop peripheral when CPU1 enter debug mode 0: peripheral keep running when CPU1 in debug mode 1: peripheral enter debug mode when CPU1 enter debug"]
pub type CPU1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stop peripheral when CPU0 enter debug mode 0: peripheral keep running when CPU0 in debug mode 1: peripheral enter debug mode when CPU0 enter debug"]
    #[inline(always)]
    pub fn cpu0(&self) -> CPU0_R {
        CPU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stop peripheral when CPU1 enter debug mode 0: peripheral keep running when CPU1 in debug mode 1: peripheral enter debug mode when CPU1 enter debug"]
    #[inline(always)]
    pub fn cpu1(&self) -> CPU1_R {
        CPU1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop peripheral when CPU0 enter debug mode 0: peripheral keep running when CPU0 in debug mode 1: peripheral enter debug mode when CPU0 enter debug"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0(&mut self) -> CPU0_W<DEBUG_STOP_SPEC> {
        CPU0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Stop peripheral when CPU1 enter debug mode 0: peripheral keep running when CPU1 in debug mode 1: peripheral enter debug mode when CPU1 enter debug"]
    #[inline(always)]
    #[must_use]
    pub fn cpu1(&mut self) -> CPU1_W<DEBUG_STOP_SPEC> {
        CPU1_W::new(self, 1)
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
#[doc = "Debug stop config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_stop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_stop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_STOP_SPEC;
impl crate::RegisterSpec for DEBUG_STOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_stop::R`](R) reader structure"]
impl crate::Readable for DEBUG_STOP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`debug_stop::W`](W) writer structure"]
impl crate::Writable for DEBUG_STOP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG_STOP to value 0x01"]
impl crate::Resettable for DEBUG_STOP_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
