#[doc = "Register `sta` reader"]
pub type R = crate::R<STA_SPEC>;
#[doc = "Register `sta` writer"]
pub type W = crate::W<STA_SPEC>;
#[doc = "Field `STA` reader - pwm timer counter start value sta/rld will be loaded from shadow register to work register at main counter reload time, or software write unlk.shunlk"]
pub type STA_R = crate::FieldReader<u32>;
#[doc = "Field `STA` writer - pwm timer counter start value sta/rld will be loaded from shadow register to work register at main counter reload time, or software write unlk.shunlk"]
pub type STA_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `XSTA` reader - pwm timer counter extended start point, should back to this value after reach xrld"]
pub type XSTA_R = crate::FieldReader;
#[doc = "Field `XSTA` writer - pwm timer counter extended start point, should back to this value after reach xrld"]
pub type XSTA_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 4:27 - pwm timer counter start value sta/rld will be loaded from shadow register to work register at main counter reload time, or software write unlk.shunlk"]
    #[inline(always)]
    pub fn sta(&self) -> STA_R {
        STA_R::new((self.bits >> 4) & 0x00ff_ffff)
    }
    #[doc = "Bits 28:31 - pwm timer counter extended start point, should back to this value after reach xrld"]
    #[inline(always)]
    pub fn xsta(&self) -> XSTA_R {
        XSTA_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:27 - pwm timer counter start value sta/rld will be loaded from shadow register to work register at main counter reload time, or software write unlk.shunlk"]
    #[inline(always)]
    #[must_use]
    pub fn sta(&mut self) -> STA_W<STA_SPEC> {
        STA_W::new(self, 4)
    }
    #[doc = "Bits 28:31 - pwm timer counter extended start point, should back to this value after reach xrld"]
    #[inline(always)]
    #[must_use]
    pub fn xsta(&mut self) -> XSTA_W<STA_SPEC> {
        XSTA_W::new(self, 28)
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
#[doc = "Counter start register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STA_SPEC;
impl crate::RegisterSpec for STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sta::R`](R) reader structure"]
impl crate::Readable for STA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sta::W`](W) writer structure"]
impl crate::Writable for STA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sta to value 0"]
impl crate::Resettable for STA_SPEC {
    const RESET_VALUE: u32 = 0;
}
