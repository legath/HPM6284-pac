#[doc = "Register `int_sts` reader"]
pub type R = crate::R<INT_STS_SPEC>;
#[doc = "Register `int_sts` writer"]
pub type W = crate::W<INT_STS_SPEC>;
#[doc = "Field `PPS_INT_STS0` writer - No description avaiable"]
pub type PPS_INT_STS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPTURE_INT_STS0` writer - No description avaiable"]
pub type CAPTURE_INT_STS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP_INT_STS0` writer - No description avaiable"]
pub type COMP_INT_STS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPS_INT_STS1` writer - No description avaiable"]
pub type PPS_INT_STS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPTURE_INT_STS1` writer - No description avaiable"]
pub type CAPTURE_INT_STS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP_INT_STS1` writer - No description avaiable"]
pub type COMP_INT_STS1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn pps_int_sts0(&mut self) -> PPS_INT_STS0_W<INT_STS_SPEC> {
        PPS_INT_STS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn capture_int_sts0(&mut self) -> CAPTURE_INT_STS0_W<INT_STS_SPEC> {
        CAPTURE_INT_STS0_W::new(self, 1)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn comp_int_sts0(&mut self) -> COMP_INT_STS0_W<INT_STS_SPEC> {
        COMP_INT_STS0_W::new(self, 2)
    }
    #[doc = "Bit 16 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn pps_int_sts1(&mut self) -> PPS_INT_STS1_W<INT_STS_SPEC> {
        PPS_INT_STS1_W::new(self, 16)
    }
    #[doc = "Bit 17 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn capture_int_sts1(&mut self) -> CAPTURE_INT_STS1_W<INT_STS_SPEC> {
        CAPTURE_INT_STS1_W::new(self, 17)
    }
    #[doc = "Bit 18 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn comp_int_sts1(&mut self) -> COMP_INT_STS1_W<INT_STS_SPEC> {
        COMP_INT_STS1_W::new(self, 18)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_STS_SPEC;
impl crate::RegisterSpec for INT_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_sts::R`](R) reader structure"]
impl crate::Readable for INT_STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_sts::W`](W) writer structure"]
impl crate::Writable for INT_STS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets int_sts to value 0"]
impl crate::Resettable for INT_STS_SPEC {
    const RESET_VALUE: u32 = 0;
}
