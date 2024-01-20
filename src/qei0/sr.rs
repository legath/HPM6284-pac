#[doc = "Register `sr` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Register `sr` writer"]
pub type W = crate::W<SR_SPEC>;
#[doc = "Field `ZPHF` reader - z input flag"]
pub type ZPHF_R = crate::BitReader;
#[doc = "Field `ZPHF` writer - z input flag"]
pub type ZPHF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POSCMPF` reader - postion compare match flag"]
pub type POSCMPF_R = crate::BitReader;
#[doc = "Field `POSCMPF` writer - postion compare match flag"]
pub type POSCMPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOMEF` reader - home flag"]
pub type HOMEF_R = crate::BitReader;
#[doc = "Field `HOMEF` writer - home flag"]
pub type HOMEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDGF` reader - watchdog flag"]
pub type WDGF_R = crate::BitReader;
#[doc = "Field `WDGF` writer - watchdog flag"]
pub type WDGF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 28 - z input flag"]
    #[inline(always)]
    pub fn zphf(&self) -> ZPHF_R {
        ZPHF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - postion compare match flag"]
    #[inline(always)]
    pub fn poscmpf(&self) -> POSCMPF_R {
        POSCMPF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - home flag"]
    #[inline(always)]
    pub fn homef(&self) -> HOMEF_R {
        HOMEF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - watchdog flag"]
    #[inline(always)]
    pub fn wdgf(&self) -> WDGF_R {
        WDGF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - z input flag"]
    #[inline(always)]
    #[must_use]
    pub fn zphf(&mut self) -> ZPHF_W<SR_SPEC> {
        ZPHF_W::new(self, 28)
    }
    #[doc = "Bit 29 - postion compare match flag"]
    #[inline(always)]
    #[must_use]
    pub fn poscmpf(&mut self) -> POSCMPF_W<SR_SPEC> {
        POSCMPF_W::new(self, 29)
    }
    #[doc = "Bit 30 - home flag"]
    #[inline(always)]
    #[must_use]
    pub fn homef(&mut self) -> HOMEF_W<SR_SPEC> {
        HOMEF_W::new(self, 30)
    }
    #[doc = "Bit 31 - watchdog flag"]
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
