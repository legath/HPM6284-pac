#[doc = "Register `SHADOW_LOCK[%s]` reader"]
pub type R = crate::R<SHADOW_LOCK_SPEC>;
#[doc = "Register `SHADOW_LOCK[%s]` writer"]
pub type W = crate::W<SHADOW_LOCK_SPEC>;
#[doc = "Field `LOCK` reader - lock for pmic part shadow registers, 2 bits per 32 bit word, lock behavior is different between different fuse types 00: not locked 01: soft locked 10: not locked, and cannot lock in furture 11: double locked"]
pub type LOCK_R = crate::FieldReader<u32>;
#[doc = "Field `LOCK` writer - lock for pmic part shadow registers, 2 bits per 32 bit word, lock behavior is different between different fuse types 00: not locked 01: soft locked 10: not locked, and cannot lock in furture 11: double locked"]
pub type LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - lock for pmic part shadow registers, 2 bits per 32 bit word, lock behavior is different between different fuse types 00: not locked 01: soft locked 10: not locked, and cannot lock in furture 11: double locked"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - lock for pmic part shadow registers, 2 bits per 32 bit word, lock behavior is different between different fuse types 00: not locked 01: soft locked 10: not locked, and cannot lock in furture 11: double locked"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<SHADOW_LOCK_SPEC> {
        LOCK_W::new(self, 0)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shadow_lock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shadow_lock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHADOW_LOCK_SPEC;
impl crate::RegisterSpec for SHADOW_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shadow_lock::R`](R) reader structure"]
impl crate::Readable for SHADOW_LOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shadow_lock::W`](W) writer structure"]
impl crate::Writable for SHADOW_LOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHADOW_LOCK[%s]
to value 0"]
impl crate::Resettable for SHADOW_LOCK_SPEC {
    const RESET_VALUE: u32 = 0;
}
