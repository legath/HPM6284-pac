#[doc = "Register `rld` reader"]
pub type R = crate::R<RLD_SPEC>;
#[doc = "Register `rld` writer"]
pub type W = crate::W<RLD_SPEC>;
#[doc = "Field `RLD` reader - pwm timer counter reload value"]
pub type RLD_R = crate::FieldReader<u32>;
#[doc = "Field `RLD` writer - pwm timer counter reload value"]
pub type RLD_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `XRLD` reader - timeout counter extended reload point, counter will reload to xsta after reach this point"]
pub type XRLD_R = crate::FieldReader;
#[doc = "Field `XRLD` writer - timeout counter extended reload point, counter will reload to xsta after reach this point"]
pub type XRLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 4:27 - pwm timer counter reload value"]
    #[inline(always)]
    pub fn rld(&self) -> RLD_R {
        RLD_R::new((self.bits >> 4) & 0x00ff_ffff)
    }
    #[doc = "Bits 28:31 - timeout counter extended reload point, counter will reload to xsta after reach this point"]
    #[inline(always)]
    pub fn xrld(&self) -> XRLD_R {
        XRLD_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:27 - pwm timer counter reload value"]
    #[inline(always)]
    #[must_use]
    pub fn rld(&mut self) -> RLD_W<RLD_SPEC> {
        RLD_W::new(self, 4)
    }
    #[doc = "Bits 28:31 - timeout counter extended reload point, counter will reload to xsta after reach this point"]
    #[inline(always)]
    #[must_use]
    pub fn xrld(&mut self) -> XRLD_W<RLD_SPEC> {
        XRLD_W::new(self, 28)
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
#[doc = "Counter reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rld::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rld::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RLD_SPEC;
impl crate::RegisterSpec for RLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rld::R`](R) reader structure"]
impl crate::Readable for RLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rld::W`](W) writer structure"]
impl crate::Writable for RLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rld to value 0"]
impl crate::Resettable for RLD_SPEC {
    const RESET_VALUE: u32 = 0;
}
