#[doc = "Register `SDST` reader"]
pub type R = crate::R<SDST_SPEC>;
#[doc = "Register `SDST` writer"]
pub type W = crate::W<SDST_SPEC>;
#[doc = "Field `FILL` reader - Data FIFO Fillings"]
pub type FILL_R = crate::FieldReader;
#[doc = "Field `WTSYNFLG` reader - Wait-for-sync event found"]
pub type WTSYNFLG_R = crate::BitReader;
#[doc = "Field `DSAT_ERR` writer - CIC out Data saturation err. Error flag."]
pub type DSAT_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOV_ERR` writer - Data FIFO Overflow Error. Error flag."]
pub type DOV_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AF` writer - Achnowledge flag"]
pub type AF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_DR` writer - FIFO data ready"]
pub type FIFO_DR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERIOD_MCLK` reader - maxim of mclk spacing in cycles, using edges of mclk signal. In manchester coding mode, it is just the period of MCLK. In other modes, it is almost the half period."]
pub type PERIOD_MCLK_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Data FIFO Fillings"]
    #[inline(always)]
    pub fn fill(&self) -> FILL_R {
        FILL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Wait-for-sync event found"]
    #[inline(always)]
    pub fn wtsynflg(&self) -> WTSYNFLG_R {
        WTSYNFLG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 23:30 - maxim of mclk spacing in cycles, using edges of mclk signal. In manchester coding mode, it is just the period of MCLK. In other modes, it is almost the half period."]
    #[inline(always)]
    pub fn period_mclk(&self) -> PERIOD_MCLK_R {
        PERIOD_MCLK_R::new(((self.bits >> 23) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - CIC out Data saturation err. Error flag."]
    #[inline(always)]
    #[must_use]
    pub fn dsat_err(&mut self) -> DSAT_ERR_W<SDST_SPEC> {
        DSAT_ERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Data FIFO Overflow Error. Error flag."]
    #[inline(always)]
    #[must_use]
    pub fn dov_err(&mut self) -> DOV_ERR_W<SDST_SPEC> {
        DOV_ERR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Achnowledge flag"]
    #[inline(always)]
    #[must_use]
    pub fn af(&mut self) -> AF_W<SDST_SPEC> {
        AF_W::new(self, 8)
    }
    #[doc = "Bit 9 - FIFO data ready"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_dr(&mut self) -> FIFO_DR_W<SDST_SPEC> {
        FIFO_DR_W::new(self, 9)
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
#[doc = "Data Path Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDST_SPEC;
impl crate::RegisterSpec for SDST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdst::R`](R) reader structure"]
impl crate::Readable for SDST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdst::W`](W) writer structure"]
impl crate::Writable for SDST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDST to value 0"]
impl crate::Resettable for SDST_SPEC {
    const RESET_VALUE: u32 = 0;
}
