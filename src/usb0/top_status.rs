#[doc = "Register `TOP_STATUS` reader"]
pub type R = crate::R<TOP_STATUS_SPEC>;
#[doc = "Register `TOP_STATUS` writer"]
pub type W = crate::W<TOP_STATUS_SPEC>;
#[doc = "Field `WAKEUP_INT_STATUS` reader - No description avaiable"]
pub type WAKEUP_INT_STATUS_R = crate::BitReader;
#[doc = "Field `WAKEUP_INT_STATUS` writer - No description avaiable"]
pub type WAKEUP_INT_STATUS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - No description avaiable"]
    #[inline(always)]
    pub fn wakeup_int_status(&self) -> WAKEUP_INT_STATUS_R {
        WAKEUP_INT_STATUS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_int_status(&mut self) -> WAKEUP_INT_STATUS_W<TOP_STATUS_SPEC> {
        WAKEUP_INT_STATUS_W::new(self, 31)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`top_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`top_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOP_STATUS_SPEC;
impl crate::RegisterSpec for TOP_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`top_status::R`](R) reader structure"]
impl crate::Readable for TOP_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`top_status::W`](W) writer structure"]
impl crate::Writable for TOP_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOP_STATUS to value 0"]
impl crate::Resettable for TOP_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
