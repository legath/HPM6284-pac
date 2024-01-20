#[doc = "Register `pps_ctrl` reader"]
pub type R = crate::R<PPS_CTRL_SPEC>;
#[doc = "Register `pps_ctrl` writer"]
pub type W = crate::W<PPS_CTRL_SPEC>;
#[doc = "Field `PPS_CTRL` reader - No description avaiable"]
pub type PPS_CTRL_R = crate::FieldReader;
#[doc = "Field `PPS_CTRL` writer - No description avaiable"]
pub type PPS_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - No description avaiable"]
    #[inline(always)]
    pub fn pps_ctrl(&self) -> PPS_CTRL_R {
        PPS_CTRL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn pps_ctrl(&mut self) -> PPS_CTRL_W<PPS_CTRL_SPEC> {
        PPS_CTRL_W::new(self, 0)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pps_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pps_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPS_CTRL_SPEC;
impl crate::RegisterSpec for PPS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pps_ctrl::R`](R) reader structure"]
impl crate::Readable for PPS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pps_ctrl::W`](W) writer structure"]
impl crate::Writable for PPS_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pps_ctrl to value 0"]
impl crate::Resettable for PPS_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
