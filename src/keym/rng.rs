#[doc = "Register `RNG` reader"]
pub type R = crate::R<RNG_SPEC>;
#[doc = "Register `RNG` writer"]
pub type W = crate::W<RNG_SPEC>;
#[doc = "Field `RNG_XOR` reader - control how SFK is accepted from random number generator 0: SFK value replaced by random number input 1: SFK value exclusive or with random number input,this help generate random number using 2 rings inside RNG"]
pub type RNG_XOR_R = crate::BitReader;
#[doc = "Field `RNG_XOR` writer - control how SFK is accepted from random number generator 0: SFK value replaced by random number input 1: SFK value exclusive or with random number input,this help generate random number using 2 rings inside RNG"]
pub type RNG_XOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCK_RNG_XOR` reader - block RNG_XOR bit from changing, if this bit is written to 1, it will hold 1 until next reset 0: RNG_XOR can be changed by software 1: RNG_XOR ignore software change from software"]
pub type BLOCK_RNG_XOR_R = crate::BitReader;
#[doc = "Field `BLOCK_RNG_XOR` writer - block RNG_XOR bit from changing, if this bit is written to 1, it will hold 1 until next reset 0: RNG_XOR can be changed by software 1: RNG_XOR ignore software change from software"]
pub type BLOCK_RNG_XOR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - control how SFK is accepted from random number generator 0: SFK value replaced by random number input 1: SFK value exclusive or with random number input,this help generate random number using 2 rings inside RNG"]
    #[inline(always)]
    pub fn rng_xor(&self) -> RNG_XOR_R {
        RNG_XOR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - block RNG_XOR bit from changing, if this bit is written to 1, it will hold 1 until next reset 0: RNG_XOR can be changed by software 1: RNG_XOR ignore software change from software"]
    #[inline(always)]
    pub fn block_rng_xor(&self) -> BLOCK_RNG_XOR_R {
        BLOCK_RNG_XOR_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - control how SFK is accepted from random number generator 0: SFK value replaced by random number input 1: SFK value exclusive or with random number input,this help generate random number using 2 rings inside RNG"]
    #[inline(always)]
    #[must_use]
    pub fn rng_xor(&mut self) -> RNG_XOR_W<RNG_SPEC> {
        RNG_XOR_W::new(self, 0)
    }
    #[doc = "Bit 16 - block RNG_XOR bit from changing, if this bit is written to 1, it will hold 1 until next reset 0: RNG_XOR can be changed by software 1: RNG_XOR ignore software change from software"]
    #[inline(always)]
    #[must_use]
    pub fn block_rng_xor(&mut self) -> BLOCK_RNG_XOR_W<RNG_SPEC> {
        BLOCK_RNG_XOR_W::new(self, 16)
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
#[doc = "Random number interface behavior\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rng::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNG_SPEC;
impl crate::RegisterSpec for RNG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng::R`](R) reader structure"]
impl crate::Readable for RNG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rng::W`](W) writer structure"]
impl crate::Writable for RNG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNG to value 0"]
impl crate::Resettable for RNG_SPEC {
    const RESET_VALUE: u32 = 0;
}
