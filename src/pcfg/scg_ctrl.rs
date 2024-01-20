#[doc = "Register `SCG_CTRL` reader"]
pub type R = crate::R<SCG_CTRL_SPEC>;
#[doc = "Register `SCG_CTRL` writer"]
pub type W = crate::W<SCG_CTRL_SPEC>;
#[doc = "Field `SCG` reader - control whether clock being gated during PMIC low power flow, 2 bits for each peripheral 00,01: reserved 10: clock is always off 11: clock is always on bit0-1: fuse bit6-7:gpio bit8-9:ioc bit10-11: timer bit12-13:wdog bit14-15:uart bit16-17:debug"]
pub type SCG_R = crate::FieldReader<u32>;
#[doc = "Field `SCG` writer - control whether clock being gated during PMIC low power flow, 2 bits for each peripheral 00,01: reserved 10: clock is always off 11: clock is always on bit0-1: fuse bit6-7:gpio bit8-9:ioc bit10-11: timer bit12-13:wdog bit14-15:uart bit16-17:debug"]
pub type SCG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - control whether clock being gated during PMIC low power flow, 2 bits for each peripheral 00,01: reserved 10: clock is always off 11: clock is always on bit0-1: fuse bit6-7:gpio bit8-9:ioc bit10-11: timer bit12-13:wdog bit14-15:uart bit16-17:debug"]
    #[inline(always)]
    pub fn scg(&self) -> SCG_R {
        SCG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - control whether clock being gated during PMIC low power flow, 2 bits for each peripheral 00,01: reserved 10: clock is always off 11: clock is always on bit0-1: fuse bit6-7:gpio bit8-9:ioc bit10-11: timer bit12-13:wdog bit14-15:uart bit16-17:debug"]
    #[inline(always)]
    #[must_use]
    pub fn scg(&mut self) -> SCG_W<SCG_CTRL_SPEC> {
        SCG_W::new(self, 0)
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
#[doc = "Clock gate control in PMIC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scg_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scg_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCG_CTRL_SPEC;
impl crate::RegisterSpec for SCG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scg_ctrl::R`](R) reader structure"]
impl crate::Readable for SCG_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scg_ctrl::W`](W) writer structure"]
impl crate::Writable for SCG_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCG_CTRL to value 0xffff_ffff"]
impl crate::Resettable for SCG_CTRL_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
