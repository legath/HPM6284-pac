#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<CONTROL_SPEC>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<CONTROL_SPEC>;
#[doc = "Field `ENABLE` reader - enable glitch detector 0: detector disabled 1: detector enabled"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - enable glitch detector 0: detector disabled 1: detector enabled"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE` reader - select glitch works in active mode or passve mode. 0: passive mode, depends on power glitch destory DFF value 1: active mode, check glitch by DFF chain"]
pub type ACTIVE_R = crate::BitReader;
#[doc = "Field `ACTIVE` writer - select glitch works in active mode or passve mode. 0: passive mode, depends on power glitch destory DFF value 1: active mode, check glitch by DFF chain"]
pub type ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable glitch detector 0: detector disabled 1: detector enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - select glitch works in active mode or passve mode. 0: passive mode, depends on power glitch destory DFF value 1: active mode, check glitch by DFF chain"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable glitch detector 0: detector disabled 1: detector enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CONTROL_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 4 - select glitch works in active mode or passve mode. 0: passive mode, depends on power glitch destory DFF value 1: active mode, check glitch by DFF chain"]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ACTIVE_W<CONTROL_SPEC> {
        ACTIVE_W::new(self, 4)
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
#[doc = "Glitch and clock monitor control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: u32 = 0;
}
