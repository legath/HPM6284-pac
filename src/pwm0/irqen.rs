#[doc = "Register `irqen` reader"]
pub type R = crate::R<IRQEN_SPEC>;
#[doc = "Register `irqen` writer"]
pub type W = crate::W<IRQEN_SPEC>;
#[doc = "Field `CMPIRQEX` reader - comparator output compare or input capture flag interrupt enable"]
pub type CMPIRQEX_R = crate::FieldReader<u32>;
#[doc = "Field `CMPIRQEX` writer - comparator output compare or input capture flag interrupt enable"]
pub type CMPIRQEX_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `RLDIRQE` reader - reload flag interrupt enable"]
pub type RLDIRQE_R = crate::BitReader;
#[doc = "Field `RLDIRQE` writer - reload flag interrupt enable"]
pub type RLDIRQE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALFRLDIRQE` reader - half reload flag interrupt enable"]
pub type HALFRLDIRQE_R = crate::BitReader;
#[doc = "Field `HALFRLDIRQE` writer - half reload flag interrupt enable"]
pub type HALFRLDIRQE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XRLDIRQE` reader - extended reload flag interrupt enable"]
pub type XRLDIRQE_R = crate::BitReader;
#[doc = "Field `XRLDIRQE` writer - extended reload flag interrupt enable"]
pub type XRLDIRQE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULTIRQE` reader - fault condition interrupt enable"]
pub type FAULTIRQE_R = crate::BitReader;
#[doc = "Field `FAULTIRQE` writer - fault condition interrupt enable"]
pub type FAULTIRQE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - comparator output compare or input capture flag interrupt enable"]
    #[inline(always)]
    pub fn cmpirqex(&self) -> CMPIRQEX_R {
        CMPIRQEX_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - reload flag interrupt enable"]
    #[inline(always)]
    pub fn rldirqe(&self) -> RLDIRQE_R {
        RLDIRQE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - half reload flag interrupt enable"]
    #[inline(always)]
    pub fn halfrldirqe(&self) -> HALFRLDIRQE_R {
        HALFRLDIRQE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - extended reload flag interrupt enable"]
    #[inline(always)]
    pub fn xrldirqe(&self) -> XRLDIRQE_R {
        XRLDIRQE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - fault condition interrupt enable"]
    #[inline(always)]
    pub fn faultirqe(&self) -> FAULTIRQE_R {
        FAULTIRQE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - comparator output compare or input capture flag interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpirqex(&mut self) -> CMPIRQEX_W<IRQEN_SPEC> {
        CMPIRQEX_W::new(self, 0)
    }
    #[doc = "Bit 24 - reload flag interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rldirqe(&mut self) -> RLDIRQE_W<IRQEN_SPEC> {
        RLDIRQE_W::new(self, 24)
    }
    #[doc = "Bit 25 - half reload flag interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn halfrldirqe(&mut self) -> HALFRLDIRQE_W<IRQEN_SPEC> {
        HALFRLDIRQE_W::new(self, 25)
    }
    #[doc = "Bit 26 - extended reload flag interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn xrldirqe(&mut self) -> XRLDIRQE_W<IRQEN_SPEC> {
        XRLDIRQE_W::new(self, 26)
    }
    #[doc = "Bit 27 - fault condition interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn faultirqe(&mut self) -> FAULTIRQE_W<IRQEN_SPEC> {
        FAULTIRQE_W::new(self, 27)
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
