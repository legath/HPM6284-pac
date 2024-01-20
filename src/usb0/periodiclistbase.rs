#[doc = "Register `PERIODICLISTBASE` reader"]
pub type R = crate::R<PERIODICLISTBASE_SPEC>;
#[doc = "Register `PERIODICLISTBASE` writer"]
pub type W = crate::W<PERIODICLISTBASE_SPEC>;
#[doc = "Field `BASEADR` reader - BASEADR Base Address (Low). These bits correspond to memory address signals \\[31:12\\], respectively. Only used by the host controller."]
pub type BASEADR_R = crate::FieldReader<u32>;
#[doc = "Field `BASEADR` writer - BASEADR Base Address (Low). These bits correspond to memory address signals \\[31:12\\], respectively. Only used by the host controller."]
pub type BASEADR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 12:31 - BASEADR Base Address (Low). These bits correspond to memory address signals \\[31:12\\], respectively. Only used by the host controller."]
    #[inline(always)]
    pub fn baseadr(&self) -> BASEADR_R {
        BASEADR_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - BASEADR Base Address (Low). These bits correspond to memory address signals \\[31:12\\], respectively. Only used by the host controller."]
    #[inline(always)]
    #[must_use]
    pub fn baseadr(&mut self) -> BASEADR_W<PERIODICLISTBASE_SPEC> {
        BASEADR_W::new(self, 12)
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
#[doc = "Frame List Base Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`periodiclistbase::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`periodiclistbase::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERIODICLISTBASE_SPEC;
impl crate::RegisterSpec for PERIODICLISTBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periodiclistbase::R`](R) reader structure"]
impl crate::Readable for PERIODICLISTBASE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`periodiclistbase::W`](W) writer structure"]
impl crate::Writable for PERIODICLISTBASE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERIODICLISTBASE to value 0"]
impl crate::Resettable for PERIODICLISTBASE_SPEC {
    const RESET_VALUE: u32 = 0;
}
