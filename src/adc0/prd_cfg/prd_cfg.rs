#[doc = "Register `prd_cfg` reader"]
pub type R = crate::R<PRD_CFG_SPEC>;
#[doc = "Register `prd_cfg` writer"]
pub type W = crate::W<PRD_CFG_SPEC>;
#[doc = "Field `PRD` reader - conver period, with prescale. Set to 0 means disable current channel"]
pub type PRD_R = crate::FieldReader;
#[doc = "Field `PRD` writer - conver period, with prescale. Set to 0 means disable current channel"]
pub type PRD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRESCALE` reader - 0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
pub type PRESCALE_R = crate::FieldReader;
#[doc = "Field `PRESCALE` writer - 0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
pub type PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - conver period, with prescale. Set to 0 means disable current channel"]
    #[inline(always)]
    pub fn prd(&self) -> PRD_R {
        PRD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - 0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - conver period, with prescale. Set to 0 means disable current channel"]
    #[inline(always)]
    #[must_use]
    pub fn prd(&mut self) -> PRD_W<PRD_CFG_SPEC> {
        PRD_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    #[inline(always)]
    #[must_use]
    pub fn prescale(&mut self) -> PRESCALE_W<PRD_CFG_SPEC> {
        PRESCALE_W::new(self, 8)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prd_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prd_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRD_CFG_SPEC;
impl crate::RegisterSpec for PRD_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prd_cfg::R`](R) reader structure"]
impl crate::Readable for PRD_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prd_cfg::W`](W) writer structure"]
impl crate::Writable for PRD_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets prd_cfg to value 0"]
impl crate::Resettable for PRD_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
