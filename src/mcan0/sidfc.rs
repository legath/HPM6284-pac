#[doc = "Register `SIDFC` reader"]
pub type R = crate::R<SIDFC_SPEC>;
#[doc = "Register `SIDFC` writer"]
pub type W = crate::W<SIDFC_SPEC>;
#[doc = "Field `FLSSA` reader - Filter List Standard Start Address Start address of standard Message ID filter list (32-bit word address)"]
pub type FLSSA_R = crate::FieldReader<u16>;
#[doc = "Field `FLSSA` writer - Filter List Standard Start Address Start address of standard Message ID filter list (32-bit word address)"]
pub type FLSSA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `LSS` reader - List Size Standard 0= No standard Message ID filter 1-128= Number of standard Message ID filter elements >128= Values greater than 128 are interpreted as 128"]
pub type LSS_R = crate::FieldReader;
#[doc = "Field `LSS` writer - List Size Standard 0= No standard Message ID filter 1-128= Number of standard Message ID filter elements >128= Values greater than 128 are interpreted as 128"]
pub type LSS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 2:15 - Filter List Standard Start Address Start address of standard Message ID filter list (32-bit word address)"]
    #[inline(always)]
    pub fn flssa(&self) -> FLSSA_R {
        FLSSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:23 - List Size Standard 0= No standard Message ID filter 1-128= Number of standard Message ID filter elements >128= Values greater than 128 are interpreted as 128"]
    #[inline(always)]
    pub fn lss(&self) -> LSS_R {
        LSS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Filter List Standard Start Address Start address of standard Message ID filter list (32-bit word address)"]
    #[inline(always)]
    #[must_use]
    pub fn flssa(&mut self) -> FLSSA_W<SIDFC_SPEC> {
        FLSSA_W::new(self, 2)
    }
    #[doc = "Bits 16:23 - List Size Standard 0= No standard Message ID filter 1-128= Number of standard Message ID filter elements >128= Values greater than 128 are interpreted as 128"]
    #[inline(always)]
    #[must_use]
    pub fn lss(&mut self) -> LSS_W<SIDFC_SPEC> {
        LSS_W::new(self, 16)
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
#[doc = "standard ID filter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sidfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIDFC_SPEC;
impl crate::RegisterSpec for SIDFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sidfc::R`](R) reader structure"]
impl crate::Readable for SIDFC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sidfc::W`](W) writer structure"]
impl crate::Writable for SIDFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIDFC to value 0"]
impl crate::Resettable for SIDFC_SPEC {
    const RESET_VALUE: u32 = 0;
}
