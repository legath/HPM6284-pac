#[doc = "Register `config` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `config` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `POST_WAIT` reader - time guard band for reset release 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
pub type POST_WAIT_R = crate::FieldReader;
#[doc = "Field `POST_WAIT` writer - time guard band for reset release 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
pub type POST_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RSTCLK_NUM` reader - reset clock number(must be even number) 0: 0 cycle 1: 0 cycles 2: 2 cycles 3: 2 cycles . . . Note, clock cycle is base on 24M"]
pub type RSTCLK_NUM_R = crate::FieldReader;
#[doc = "Field `RSTCLK_NUM` writer - reset clock number(must be even number) 0: 0 cycle 1: 0 cycles 2: 2 cycles 3: 2 cycles . . . Note, clock cycle is base on 24M"]
pub type RSTCLK_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRE_WAIT` reader - wait cycle numbers before assert reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
pub type PRE_WAIT_R = crate::FieldReader;
#[doc = "Field `PRE_WAIT` writer - wait cycle numbers before assert reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
pub type PRE_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - time guard band for reset release 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    #[inline(always)]
    pub fn post_wait(&self) -> POST_WAIT_R {
        POST_WAIT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - reset clock number(must be even number) 0: 0 cycle 1: 0 cycles 2: 2 cycles 3: 2 cycles . . . Note, clock cycle is base on 24M"]
    #[inline(always)]
    pub fn rstclk_num(&self) -> RSTCLK_NUM_R {
        RSTCLK_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - wait cycle numbers before assert reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    #[inline(always)]
    pub fn pre_wait(&self) -> PRE_WAIT_R {
        PRE_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - time guard band for reset release 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    #[inline(always)]
    #[must_use]
    pub fn post_wait(&mut self) -> POST_WAIT_W<CONFIG_SPEC> {
        POST_WAIT_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - reset clock number(must be even number) 0: 0 cycle 1: 0 cycles 2: 2 cycles 3: 2 cycles . . . Note, clock cycle is base on 24M"]
    #[inline(always)]
    #[must_use]
    pub fn rstclk_num(&mut self) -> RSTCLK_NUM_W<CONFIG_SPEC> {
        RSTCLK_NUM_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - wait cycle numbers before assert reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    #[inline(always)]
    #[must_use]
    pub fn pre_wait(&mut self) -> PRE_WAIT_W<CONFIG_SPEC> {
        PRE_WAIT_W::new(self, 16)
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
#[doc = "Reset Setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets config to value 0x0064_3203"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: u32 = 0x0064_3203;
}
