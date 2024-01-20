#[doc = "Register `sr` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Register `sr` writer"]
pub type W = crate::W<SR_SPEC>;
#[doc = "Field `WF` reader - w flag, will set when w signal toggle"]
pub type WF_R = crate::BitReader;
#[doc = "Field `WF` writer - w flag, will set when w signal toggle"]
pub type WF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VF` reader - v flag, will set when v signal toggle"]
pub type VF_R = crate::BitReader;
#[doc = "Field `VF` writer - v flag, will set when v signal toggle"]
pub type VF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UF` reader - u flag, will set when u signal toggle"]
pub type UF_R = crate::BitReader;
#[doc = "Field `UF` writer - u flag, will set when u signal toggle"]
pub type UF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHDLYF` reader - phase update delay flag, will set DLYCNT cycles after any of u, v, w signal toggle or after the phpre flag depands on DLYSEL setting"]
pub type PHDLYF_R = crate::BitReader;
#[doc = "Field `PHDLYF` writer - phase update delay flag, will set DLYCNT cycles after any of u, v, w signal toggle or after the phpre flag depands on DLYSEL setting"]
pub type PHDLYF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHPREF` reader - phase update pre flag, will set PRECNT cycles before any of u, v, w signal toggle"]
pub type PHPREF_R = crate::BitReader;
#[doc = "Field `PHPREF` writer - phase update pre flag, will set PRECNT cycles before any of u, v, w signal toggle"]
pub type PHPREF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHUPTF` reader - phase update flag, will set when any of u, v, w signal toggle"]
pub type PHUPTF_R = crate::BitReader;
#[doc = "Field `PHUPTF` writer - phase update flag, will set when any of u, v, w signal toggle"]
pub type PHUPTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDGF` reader - watchdog count timeout flag"]
pub type WDGF_R = crate::BitReader;
#[doc = "Field `WDGF` writer - watchdog count timeout flag"]
pub type WDGF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 21 - w flag, will set when w signal toggle"]
    #[inline(always)]
    pub fn wf(&self) -> WF_R {
        WF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - v flag, will set when v signal toggle"]
    #[inline(always)]
    pub fn vf(&self) -> VF_R {
        VF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - u flag, will set when u signal toggle"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - phase update delay flag, will set DLYCNT cycles after any of u, v, w signal toggle or after the phpre flag depands on DLYSEL setting"]
    #[inline(always)]
    pub fn phdlyf(&self) -> PHDLYF_R {
        PHDLYF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - phase update pre flag, will set PRECNT cycles before any of u, v, w signal toggle"]
    #[inline(always)]
    pub fn phpref(&self) -> PHPREF_R {
        PHPREF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - phase update flag, will set when any of u, v, w signal toggle"]
    #[inline(always)]
    pub fn phuptf(&self) -> PHUPTF_R {
        PHUPTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - watchdog count timeout flag"]
    #[inline(always)]
    pub fn wdgf(&self) -> WDGF_R {
        WDGF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - w flag, will set when w signal toggle"]
    #[inline(always)]
    #[must_use]
    pub fn wf(&mut self) -> WF_W<SR_SPEC> {
        WF_W::new(self, 21)
    }
    #[doc = "Bit 22 - v flag, will set when v signal toggle"]
    #[inline(always)]
    #[must_use]
    pub fn vf(&mut self) -> VF_W<SR_SPEC> {
        VF_W::new(self, 22)
    }
    #[doc = "Bit 23 - u flag, will set when u signal toggle"]
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UF_W<SR_SPEC> {
        UF_W::new(self, 23)
    }
    #[doc = "Bit 28 - phase update delay flag, will set DLYCNT cycles after any of u, v, w signal toggle or after the phpre flag depands on DLYSEL setting"]
    #[inline(always)]
    #[must_use]
    pub fn phdlyf(&mut self) -> PHDLYF_W<SR_SPEC> {
        PHDLYF_W::new(self, 28)
    }
    #[doc = "Bit 29 - phase update pre flag, will set PRECNT cycles before any of u, v, w signal toggle"]
    #[inline(always)]
    #[must_use]
    pub fn phpref(&mut self) -> PHPREF_W<SR_SPEC> {
        PHPREF_W::new(self, 29)
    }
    #[doc = "Bit 30 - phase update flag, will set when any of u, v, w signal toggle"]
    #[inline(always)]
    #[must_use]
    pub fn phuptf(&mut self) -> PHUPTF_W<SR_SPEC> {
        PHUPTF_W::new(self, 30)
    }
    #[doc = "Bit 31 - watchdog count timeout flag"]
    #[inline(always)]
    #[must_use]
    pub fn wdgf(&mut self) -> WDGF_W<SR_SPEC> {
        WDGF_W::new(self, 31)
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
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sr to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: u32 = 0;
}
