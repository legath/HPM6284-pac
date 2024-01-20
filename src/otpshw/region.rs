#[doc = "Register `REGION[%s]` reader"]
pub type R = crate::R<REGION_SPEC>;
#[doc = "Register `REGION[%s]` writer"]
pub type W = crate::W<REGION_SPEC>;
#[doc = "Field `START` reader - start address of load region, fuse word at start address will be reloaded region0: fixed at 0 region1: fixed at 8 region2: fixed at 16, region3: usrer configurable"]
pub type START_R = crate::FieldReader;
#[doc = "Field `START` writer - start address of load region, fuse word at start address will be reloaded region0: fixed at 0 region1: fixed at 8 region2: fixed at 16, region3: usrer configurable"]
pub type START_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `STOP` reader - stop address of load region, fuse word at end address will NOT be reloaded region0: fixed at 8 region1: fixed at 16 region2: fixed at 0, region3: usrer configurable"]
pub type STOP_R = crate::FieldReader;
#[doc = "Field `STOP` writer - stop address of load region, fuse word at end address will NOT be reloaded region0: fixed at 8 region1: fixed at 16 region2: fixed at 0, region3: usrer configurable"]
pub type STOP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - start address of load region, fuse word at start address will be reloaded region0: fixed at 0 region1: fixed at 8 region2: fixed at 16, region3: usrer configurable"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - stop address of load region, fuse word at end address will NOT be reloaded region0: fixed at 8 region1: fixed at 16 region2: fixed at 0, region3: usrer configurable"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - start address of load region, fuse word at start address will be reloaded region0: fixed at 0 region1: fixed at 8 region2: fixed at 16, region3: usrer configurable"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<REGION_SPEC> {
        START_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - stop address of load region, fuse word at end address will NOT be reloaded region0: fixed at 8 region1: fixed at 16 region2: fixed at 0, region3: usrer configurable"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<REGION_SPEC> {
        STOP_W::new(self, 8)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGION_SPEC;
impl crate::RegisterSpec for REGION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region::R`](R) reader structure"]
impl crate::Readable for REGION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`region::W`](W) writer structure"]
impl crate::Writable for REGION_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGION[%s]
to value 0x0800"]
impl crate::Resettable for REGION_SPEC {
    const RESET_VALUE: u32 = 0x0800;
}
