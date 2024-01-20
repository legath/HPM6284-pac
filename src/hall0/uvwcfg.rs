#[doc = "Register `uvwcfg` reader"]
pub type R = crate::R<UVWCFG_SPEC>;
#[doc = "Register `uvwcfg` writer"]
pub type W = crate::W<UVWCFG_SPEC>;
#[doc = "Field `PRECNT` reader - the clock cycle number which the pre flag will set before the next uvw transition"]
pub type PRECNT_R = crate::FieldReader<u32>;
#[doc = "Field `PRECNT` writer - the clock cycle number which the pre flag will set before the next uvw transition"]
pub type PRECNT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - the clock cycle number which the pre flag will set before the next uvw transition"]
    #[inline(always)]
    pub fn precnt(&self) -> PRECNT_R {
        PRECNT_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - the clock cycle number which the pre flag will set before the next uvw transition"]
    #[inline(always)]
    #[must_use]
    pub fn precnt(&mut self) -> PRECNT_W<UVWCFG_SPEC> {
        PRECNT_W::new(self, 0)
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
#[doc = "U,V,W configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uvwcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uvwcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UVWCFG_SPEC;
impl crate::RegisterSpec for UVWCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uvwcfg::R`](R) reader structure"]
impl crate::Readable for UVWCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uvwcfg::W`](W) writer structure"]
impl crate::Writable for UVWCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets uvwcfg to value 0"]
impl crate::Resettable for UVWCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
