#[doc = "Register `LOCK` reader"]
pub type R = crate::R<LOCK_SPEC>;
#[doc = "Register `LOCK` writer"]
pub type W = crate::W<LOCK_SPEC>;
#[doc = "Field `LOCK` reader - Lock bit for CPU_LOCK"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - Lock bit for CPU_LOCK"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPR` reader - Lock bit for CPU_DATA0 to CPU_DATA13, once set, this bit will not clear untile next reset"]
pub type GPR_R = crate::FieldReader<u16>;
#[doc = "Field `GPR` writer - Lock bit for CPU_DATA0 to CPU_DATA13, once set, this bit will not clear untile next reset"]
pub type GPR_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bit 1 - Lock bit for CPU_LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:15 - Lock bit for CPU_DATA0 to CPU_DATA13, once set, this bit will not clear untile next reset"]
    #[inline(always)]
    pub fn gpr(&self) -> GPR_R {
        GPR_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 1 - Lock bit for CPU_LOCK"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<LOCK_SPEC> {
        LOCK_W::new(self, 1)
    }
    #[doc = "Bits 2:15 - Lock bit for CPU_DATA0 to CPU_DATA13, once set, this bit will not clear untile next reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpr(&mut self) -> GPR_W<LOCK_SPEC> {
        GPR_W::new(self, 2)
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
#[doc = "CPU0 Lock GPR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOCK_SPEC;
impl crate::RegisterSpec for LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock::R`](R) reader structure"]
impl crate::Readable for LOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lock::W`](W) writer structure"]
impl crate::Writable for LOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOCK to value 0x02"]
impl crate::Resettable for LOCK_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
