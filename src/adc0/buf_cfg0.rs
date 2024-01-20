#[doc = "Register `buf_cfg0` reader"]
pub type R = crate::R<BUF_CFG0_SPEC>;
#[doc = "Register `buf_cfg0` writer"]
pub type W = crate::W<BUF_CFG0_SPEC>;
#[doc = "Field `WAIT_DIS` reader - set to disable read waiting, get result immediately but maybe not current conversion result."]
pub type WAIT_DIS_R = crate::BitReader;
#[doc = "Field `WAIT_DIS` writer - set to disable read waiting, get result immediately but maybe not current conversion result."]
pub type WAIT_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - set to disable read waiting, get result immediately but maybe not current conversion result."]
    #[inline(always)]
    pub fn wait_dis(&self) -> WAIT_DIS_R {
        WAIT_DIS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - set to disable read waiting, get result immediately but maybe not current conversion result."]
    #[inline(always)]
    #[must_use]
    pub fn wait_dis(&mut self) -> WAIT_DIS_W<BUF_CFG0_SPEC> {
        WAIT_DIS_W::new(self, 0)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUF_CFG0_SPEC;
impl crate::RegisterSpec for BUF_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_cfg0::R`](R) reader structure"]
impl crate::Readable for BUF_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buf_cfg0::W`](W) writer structure"]
impl crate::Writable for BUF_CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets buf_cfg0 to value 0"]
impl crate::Resettable for BUF_CFG0_SPEC {
    const RESET_VALUE: u32 = 0;
}
