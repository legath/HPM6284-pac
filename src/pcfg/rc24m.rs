#[doc = "Register `RC24M` reader"]
pub type R = crate::R<RC24M_SPEC>;
#[doc = "Register `RC24M` writer"]
pub type W = crate::W<RC24M_SPEC>;
#[doc = "Field `TRIM_F` reader - Fine trim for RC24M, bigger value means faster"]
pub type TRIM_F_R = crate::FieldReader;
#[doc = "Field `TRIM_F` writer - Fine trim for RC24M, bigger value means faster"]
pub type TRIM_F_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIM_C` reader - Coarse trim for RC24M, bigger value means faster"]
pub type TRIM_C_R = crate::FieldReader;
#[doc = "Field `TRIM_C` writer - Coarse trim for RC24M, bigger value means faster"]
pub type TRIM_C_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RC_TRIMMED` reader - RC24M trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: RC is not trimmed 1: RC is trimmed"]
pub type RC_TRIMMED_R = crate::BitReader;
#[doc = "Field `RC_TRIMMED` writer - RC24M trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: RC is not trimmed 1: RC is trimmed"]
pub type RC_TRIMMED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Fine trim for RC24M, bigger value means faster"]
    #[inline(always)]
    pub fn trim_f(&self) -> TRIM_F_R {
        TRIM_F_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - Coarse trim for RC24M, bigger value means faster"]
    #[inline(always)]
    pub fn trim_c(&self) -> TRIM_C_R {
        TRIM_C_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 31 - RC24M trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: RC is not trimmed 1: RC is trimmed"]
    #[inline(always)]
    pub fn rc_trimmed(&self) -> RC_TRIMMED_R {
        RC_TRIMMED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Fine trim for RC24M, bigger value means faster"]
    #[inline(always)]
    #[must_use]
    pub fn trim_f(&mut self) -> TRIM_F_W<RC24M_SPEC> {
        TRIM_F_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Coarse trim for RC24M, bigger value means faster"]
    #[inline(always)]
    #[must_use]
    pub fn trim_c(&mut self) -> TRIM_C_W<RC24M_SPEC> {
        TRIM_C_W::new(self, 8)
    }
    #[doc = "Bit 31 - RC24M trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: RC is not trimmed 1: RC is trimmed"]
    #[inline(always)]
    #[must_use]
    pub fn rc_trimmed(&mut self) -> RC_TRIMMED_W<RC24M_SPEC> {
        RC_TRIMMED_W::new(self, 31)
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
#[doc = "RC 24M config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc24m::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc24m::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RC24M_SPEC;
impl crate::RegisterSpec for RC24M_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc24m::R`](R) reader structure"]
impl crate::Readable for RC24M_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rc24m::W`](W) writer structure"]
impl crate::Writable for RC24M_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RC24M to value 0x0310"]
impl crate::Resettable for RC24M_SPEC {
    const RESET_VALUE: u32 = 0x0310;
}
