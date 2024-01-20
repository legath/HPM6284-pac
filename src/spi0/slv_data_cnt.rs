#[doc = "Register `SlvDataCnt` reader"]
pub type R = crate::R<SLV_DATA_CNT_SPEC>;
#[doc = "Register `SlvDataCnt` writer"]
pub type W = crate::W<SLV_DATA_CNT_SPEC>;
#[doc = "Field `RCNT` reader - Slave received data count"]
pub type RCNT_R = crate::FieldReader<u16>;
#[doc = "Field `WCNT` reader - Slave transmitted data count"]
pub type WCNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Slave received data count"]
    #[inline(always)]
    pub fn rcnt(&self) -> RCNT_R {
        RCNT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Slave transmitted data count"]
    #[inline(always)]
    pub fn wcnt(&self) -> WCNT_R {
        WCNT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
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
#[doc = "Slave Data Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slv_data_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slv_data_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLV_DATA_CNT_SPEC;
impl crate::RegisterSpec for SLV_DATA_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slv_data_cnt::R`](R) reader structure"]
impl crate::Readable for SLV_DATA_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slv_data_cnt::W`](W) writer structure"]
impl crate::Writable for SLV_DATA_CNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SlvDataCnt to value 0"]
impl crate::Resettable for SLV_DATA_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
