#[doc = "Register `irqen` reader"]
pub type R = crate::R<IRQEN_SPEC>;
#[doc = "Register `irqen` writer"]
pub type W = crate::W<IRQEN_SPEC>;
#[doc = "Field `ZPHIE` reader - 1- generate interrupt when zphf flag set"]
pub type ZPHIE_R = crate::BitReader;
#[doc = "Field `ZPHIE` writer - 1- generate interrupt when zphf flag set"]
pub type ZPHIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POSCMPIE` reader - 1- generate interrupt when poscmpf flag set"]
pub type POSCMPIE_R = crate::BitReader;
#[doc = "Field `POSCMPIE` writer - 1- generate interrupt when poscmpf flag set"]
pub type POSCMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOMEIE` reader - 1- generate interrupt when homef flag set"]
pub type HOMEIE_R = crate::BitReader;
#[doc = "Field `HOMEIE` writer - 1- generate interrupt when homef flag set"]
pub type HOMEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDGIE` reader - 1- generate interrupt when wdg flag set"]
pub type WDGIE_R = crate::BitReader;
#[doc = "Field `WDGIE` writer - 1- generate interrupt when wdg flag set"]
pub type WDGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 28 - 1- generate interrupt when zphf flag set"]
    #[inline(always)]
    pub fn zphie(&self) -> ZPHIE_R {
        ZPHIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1- generate interrupt when poscmpf flag set"]
    #[inline(always)]
    pub fn poscmpie(&self) -> POSCMPIE_R {
        POSCMPIE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1- generate interrupt when homef flag set"]
    #[inline(always)]
    pub fn homeie(&self) -> HOMEIE_R {
        HOMEIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1- generate interrupt when wdg flag set"]
    #[inline(always)]
    pub fn wdgie(&self) -> WDGIE_R {
        WDGIE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - 1- generate interrupt when zphf flag set"]
    #[inline(always)]
    #[must_use]
    pub fn zphie(&mut self) -> ZPHIE_W<IRQEN_SPEC> {
        ZPHIE_W::new(self, 28)
    }
    #[doc = "Bit 29 - 1- generate interrupt when poscmpf flag set"]
    #[inline(always)]
    #[must_use]
    pub fn poscmpie(&mut self) -> POSCMPIE_W<IRQEN_SPEC> {
        POSCMPIE_W::new(self, 29)
    }
    #[doc = "Bit 30 - 1- generate interrupt when homef flag set"]
    #[inline(always)]
    #[must_use]
    pub fn homeie(&mut self) -> HOMEIE_W<IRQEN_SPEC> {
        HOMEIE_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1- generate interrupt when wdg flag set"]
    #[inline(always)]
    #[must_use]
    pub fn wdgie(&mut self) -> WDGIE_W<IRQEN_SPEC> {
        WDGIE_W::new(self, 31)
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
#[doc = "Interrupt request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQEN_SPEC;
impl crate::RegisterSpec for IRQEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqen::R`](R) reader structure"]
impl crate::Readable for IRQEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irqen::W`](W) writer structure"]
impl crate::Writable for IRQEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets irqen to value 0"]
impl crate::Resettable for IRQEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
