#[doc = "Register `time_sel` reader"]
pub type R = crate::R<TIME_SEL_SPEC>;
#[doc = "Register `time_sel` writer"]
pub type W = crate::W<TIME_SEL_SPEC>;
#[doc = "Field `CAN0_TIME_SEL` reader - set to use ptpc1 for canx clr to use ptpc0 for canx"]
pub type CAN0_TIME_SEL_R = crate::BitReader;
#[doc = "Field `CAN0_TIME_SEL` writer - set to use ptpc1 for canx clr to use ptpc0 for canx"]
pub type CAN0_TIME_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1_TIME_SEL` reader - No description avaiable"]
pub type CAN1_TIME_SEL_R = crate::BitReader;
#[doc = "Field `CAN1_TIME_SEL` writer - No description avaiable"]
pub type CAN1_TIME_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN2_TIME_SEL` reader - No description avaiable"]
pub type CAN2_TIME_SEL_R = crate::BitReader;
#[doc = "Field `CAN2_TIME_SEL` writer - No description avaiable"]
pub type CAN2_TIME_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN3_TIME_SEL` reader - No description avaiable"]
pub type CAN3_TIME_SEL_R = crate::BitReader;
#[doc = "Field `CAN3_TIME_SEL` writer - No description avaiable"]
pub type CAN3_TIME_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - set to use ptpc1 for canx clr to use ptpc0 for canx"]
    #[inline(always)]
    pub fn can0_time_sel(&self) -> CAN0_TIME_SEL_R {
        CAN0_TIME_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No description avaiable"]
    #[inline(always)]
    pub fn can1_time_sel(&self) -> CAN1_TIME_SEL_R {
        CAN1_TIME_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    pub fn can2_time_sel(&self) -> CAN2_TIME_SEL_R {
        CAN2_TIME_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - No description avaiable"]
    #[inline(always)]
    pub fn can3_time_sel(&self) -> CAN3_TIME_SEL_R {
        CAN3_TIME_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - set to use ptpc1 for canx clr to use ptpc0 for canx"]
    #[inline(always)]
    #[must_use]
    pub fn can0_time_sel(&mut self) -> CAN0_TIME_SEL_W<TIME_SEL_SPEC> {
        CAN0_TIME_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn can1_time_sel(&mut self) -> CAN1_TIME_SEL_W<TIME_SEL_SPEC> {
        CAN1_TIME_SEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn can2_time_sel(&mut self) -> CAN2_TIME_SEL_W<TIME_SEL_SPEC> {
        CAN2_TIME_SEL_W::new(self, 2)
    }
    #[doc = "Bit 3 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn can3_time_sel(&mut self) -> CAN3_TIME_SEL_W<TIME_SEL_SPEC> {
        CAN3_TIME_SEL_W::new(self, 3)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIME_SEL_SPEC;
impl crate::RegisterSpec for TIME_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time_sel::R`](R) reader structure"]
impl crate::Readable for TIME_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`time_sel::W`](W) writer structure"]
impl crate::Writable for TIME_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets time_sel to value 0"]
impl crate::Resettable for TIME_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
