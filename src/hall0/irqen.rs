#[doc = "Register `irqen` reader"]
pub type R = crate::R<IRQEN_SPEC>;
#[doc = "Register `irqen` writer"]
pub type W = crate::W<IRQEN_SPEC>;
#[doc = "Field `WFIE` reader - 1- generate interrupt request when w flag set"]
pub type WFIE_R = crate::BitReader;
#[doc = "Field `WFIE` writer - 1- generate interrupt request when w flag set"]
pub type WFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VFIE` reader - 1- generate interrupt request when v flag set"]
pub type VFIE_R = crate::BitReader;
#[doc = "Field `VFIE` writer - 1- generate interrupt request when v flag set"]
pub type VFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UFIE` reader - 1- generate interrupt request when u flag set"]
pub type UFIE_R = crate::BitReader;
#[doc = "Field `UFIE` writer - 1- generate interrupt request when u flag set"]
pub type UFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHDLYIE` reader - 1- generate interrupt request when phdly flag set"]
pub type PHDLYIE_R = crate::BitReader;
#[doc = "Field `PHDLYIE` writer - 1- generate interrupt request when phdly flag set"]
pub type PHDLYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHPREIE` reader - 1- generate interrupt request when phpre flag set"]
pub type PHPREIE_R = crate::BitReader;
#[doc = "Field `PHPREIE` writer - 1- generate interrupt request when phpre flag set"]
pub type PHPREIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHUPTIE` reader - 1- generate interrupt request when phupt flag set"]
pub type PHUPTIE_R = crate::BitReader;
#[doc = "Field `PHUPTIE` writer - 1- generate interrupt request when phupt flag set"]
pub type PHUPTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDGIE` reader - 1- generate interrupt request when wdg flag set"]
pub type WDGIE_R = crate::BitReader;
#[doc = "Field `WDGIE` writer - 1- generate interrupt request when wdg flag set"]
pub type WDGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 21 - 1- generate interrupt request when w flag set"]
    #[inline(always)]
    pub fn wfie(&self) -> WFIE_R {
        WFIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1- generate interrupt request when v flag set"]
    #[inline(always)]
    pub fn vfie(&self) -> VFIE_R {
        VFIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1- generate interrupt request when u flag set"]
    #[inline(always)]
    pub fn ufie(&self) -> UFIE_R {
        UFIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - 1- generate interrupt request when phdly flag set"]
    #[inline(always)]
    pub fn phdlyie(&self) -> PHDLYIE_R {
        PHDLYIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1- generate interrupt request when phpre flag set"]
    #[inline(always)]
    pub fn phpreie(&self) -> PHPREIE_R {
        PHPREIE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1- generate interrupt request when phupt flag set"]
    #[inline(always)]
    pub fn phuptie(&self) -> PHUPTIE_R {
        PHUPTIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1- generate interrupt request when wdg flag set"]
    #[inline(always)]
    pub fn wdgie(&self) -> WDGIE_R {
        WDGIE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - 1- generate interrupt request when w flag set"]
    #[inline(always)]
    #[must_use]
    pub fn wfie(&mut self) -> WFIE_W<IRQEN_SPEC> {
        WFIE_W::new(self, 21)
    }
    #[doc = "Bit 22 - 1- generate interrupt request when v flag set"]
    #[inline(always)]
    #[must_use]
    pub fn vfie(&mut self) -> VFIE_W<IRQEN_SPEC> {
        VFIE_W::new(self, 22)
    }
    #[doc = "Bit 23 - 1- generate interrupt request when u flag set"]
    #[inline(always)]
    #[must_use]
    pub fn ufie(&mut self) -> UFIE_W<IRQEN_SPEC> {
        UFIE_W::new(self, 23)
    }
    #[doc = "Bit 28 - 1- generate interrupt request when phdly flag set"]
    #[inline(always)]
    #[must_use]
    pub fn phdlyie(&mut self) -> PHDLYIE_W<IRQEN_SPEC> {
        PHDLYIE_W::new(self, 28)
    }
    #[doc = "Bit 29 - 1- generate interrupt request when phpre flag set"]
    #[inline(always)]
    #[must_use]
    pub fn phpreie(&mut self) -> PHPREIE_W<IRQEN_SPEC> {
        PHPREIE_W::new(self, 29)
    }
    #[doc = "Bit 30 - 1- generate interrupt request when phupt flag set"]
    #[inline(always)]
    #[must_use]
    pub fn phuptie(&mut self) -> PHUPTIE_W<IRQEN_SPEC> {
        PHUPTIE_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1- generate interrupt request when wdg flag set"]
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
#[doc = "Interrupt request enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
