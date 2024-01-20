#[doc = "Register `daccfg` reader"]
pub type R = crate::R<DACCFG_SPEC>;
#[doc = "Register `daccfg` writer"]
pub type W = crate::W<DACCFG_SPEC>;
#[doc = "Field `DACCFG` reader - 8bit DAC digital value"]
pub type DACCFG_R = crate::FieldReader;
#[doc = "Field `DACCFG` writer - 8bit DAC digital value"]
pub type DACCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8bit DAC digital value"]
    #[inline(always)]
    pub fn daccfg(&self) -> DACCFG_R {
        DACCFG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8bit DAC digital value"]
    #[inline(always)]
    #[must_use]
    pub fn daccfg(&mut self) -> DACCFG_W<DACCFG_SPEC> {
        DACCFG_W::new(self, 0)
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
#[doc = "DAC configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DACCFG_SPEC;
impl crate::RegisterSpec for DACCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daccfg::R`](R) reader structure"]
impl crate::Readable for DACCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`daccfg::W`](W) writer structure"]
impl crate::Writable for DACCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets daccfg to value 0"]
impl crate::Resettable for DACCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
