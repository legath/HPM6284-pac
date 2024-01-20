#[doc = "Register `CMP[%s]` reader"]
pub type R = crate::R<CMP_SPEC>;
#[doc = "Register `CMP[%s]` writer"]
pub type W = crate::W<CMP_SPEC>;
#[doc = "Field `CMPJIT` reader - jitter counter compare value"]
pub type CMPJIT_R = crate::FieldReader;
#[doc = "Field `CMPJIT` writer - jitter counter compare value"]
pub type CMPJIT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMPHLF` reader - half clock counter compare value"]
pub type CMPHLF_R = crate::BitReader;
#[doc = "Field `CMPHLF` writer - half clock counter compare value"]
pub type CMPHLF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP` reader - clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
pub type CMP_R = crate::FieldReader<u32>;
#[doc = "Field `CMP` writer - clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
pub type CMP_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `XCMP` reader - extended counter compare value"]
pub type XCMP_R = crate::FieldReader;
#[doc = "Field `XCMP` writer - extended counter compare value"]
pub type XCMP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - jitter counter compare value"]
    #[inline(always)]
    pub fn cmpjit(&self) -> CMPJIT_R {
        CMPJIT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - half clock counter compare value"]
    #[inline(always)]
    pub fn cmphlf(&self) -> CMPHLF_R {
        CMPHLF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:27 - clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new((self.bits >> 4) & 0x00ff_ffff)
    }
    #[doc = "Bits 28:31 - extended counter compare value"]
    #[inline(always)]
    pub fn xcmp(&self) -> XCMP_R {
        XCMP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - jitter counter compare value"]
    #[inline(always)]
    #[must_use]
    pub fn cmpjit(&mut self) -> CMPJIT_W<CMP_SPEC> {
        CMPJIT_W::new(self, 0)
    }
    #[doc = "Bit 3 - half clock counter compare value"]
    #[inline(always)]
    #[must_use]
    pub fn cmphlf(&mut self) -> CMPHLF_W<CMP_SPEC> {
        CMPHLF_W::new(self, 3)
    }
    #[doc = "Bits 4:27 - clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    #[inline(always)]
    #[must_use]
    pub fn cmp(&mut self) -> CMP_W<CMP_SPEC> {
        CMP_W::new(self, 4)
    }
    #[doc = "Bits 28:31 - extended counter compare value"]
    #[inline(always)]
    #[must_use]
    pub fn xcmp(&mut self) -> XCMP_W<CMP_SPEC> {
        XCMP_W::new(self, 28)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP_SPEC;
impl crate::RegisterSpec for CMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp::R`](R) reader structure"]
impl crate::Readable for CMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmp::W`](W) writer structure"]
impl crate::Writable for CMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP[%s]
to value 0"]
impl crate::Resettable for CMP_SPEC {
    const RESET_VALUE: u32 = 0;
}
