#[doc = "Register `phcmp` reader"]
pub type R = crate::R<PHCMP_SPEC>;
#[doc = "Register `phcmp` writer"]
pub type W = crate::W<PHCMP_SPEC>;
#[doc = "Field `PHCMP` reader - phcnt position compare value"]
pub type PHCMP_R = crate::FieldReader<u32>;
#[doc = "Field `PHCMP` writer - phcnt position compare value"]
pub type PHCMP_W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `DIRCMP` reader - 0- position compare need positive rotation 1- position compare need negative rotation"]
pub type DIRCMP_R = crate::BitReader;
#[doc = "Field `DIRCMP` writer - 0- position compare need positive rotation 1- position compare need negative rotation"]
pub type DIRCMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCMPDIS` reader - 1- postion compare not include rotation direction"]
pub type DIRCMPDIS_R = crate::BitReader;
#[doc = "Field `DIRCMPDIS` writer - 1- postion compare not include rotation direction"]
pub type DIRCMPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZCMPDIS` reader - 1- postion compare not include zcnt"]
pub type ZCMPDIS_R = crate::BitReader;
#[doc = "Field `ZCMPDIS` writer - 1- postion compare not include zcnt"]
pub type ZCMPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:20 - phcnt position compare value"]
    #[inline(always)]
    pub fn phcmp(&self) -> PHCMP_R {
        PHCMP_R::new(self.bits & 0x001f_ffff)
    }
    #[doc = "Bit 29 - 0- position compare need positive rotation 1- position compare need negative rotation"]
    #[inline(always)]
    pub fn dircmp(&self) -> DIRCMP_R {
        DIRCMP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1- postion compare not include rotation direction"]
    #[inline(always)]
    pub fn dircmpdis(&self) -> DIRCMPDIS_R {
        DIRCMPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1- postion compare not include zcnt"]
    #[inline(always)]
    pub fn zcmpdis(&self) -> ZCMPDIS_R {
        ZCMPDIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:20 - phcnt position compare value"]
    #[inline(always)]
    #[must_use]
    pub fn phcmp(&mut self) -> PHCMP_W<PHCMP_SPEC> {
        PHCMP_W::new(self, 0)
    }
    #[doc = "Bit 29 - 0- position compare need positive rotation 1- position compare need negative rotation"]
    #[inline(always)]
    #[must_use]
    pub fn dircmp(&mut self) -> DIRCMP_W<PHCMP_SPEC> {
        DIRCMP_W::new(self, 29)
    }
    #[doc = "Bit 30 - 1- postion compare not include rotation direction"]
    #[inline(always)]
    #[must_use]
    pub fn dircmpdis(&mut self) -> DIRCMPDIS_W<PHCMP_SPEC> {
        DIRCMPDIS_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1- postion compare not include zcnt"]
    #[inline(always)]
    #[must_use]
    pub fn zcmpdis(&mut self) -> ZCMPDIS_W<PHCMP_SPEC> {
        ZCMPDIS_W::new(self, 31)
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
#[doc = "Phase comparator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phcmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phcmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHCMP_SPEC;
impl crate::RegisterSpec for PHCMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phcmp::R`](R) reader structure"]
impl crate::Readable for PHCMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phcmp::W`](W) writer structure"]
impl crate::Writable for PHCMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets phcmp to value 0"]
impl crate::Resettable for PHCMP_SPEC {
    const RESET_VALUE: u32 = 0;
}
