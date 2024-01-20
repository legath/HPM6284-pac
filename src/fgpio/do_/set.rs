#[doc = "Register `SET` reader"]
pub type R = crate::R<SET_SPEC>;
#[doc = "Register `SET` writer"]
pub type W = crate::W<SET_SPEC>;
#[doc = "Field `OUTPUT` reader - GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
pub type OUTPUT_R = crate::FieldReader<u32>;
#[doc = "Field `OUTPUT` writer - GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
pub type OUTPUT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    #[inline(always)]
    pub fn output(&self) -> OUTPUT_R {
        OUTPUT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    #[inline(always)]
    #[must_use]
    pub fn output(&mut self) -> OUTPUT_W<SET_SPEC> {
        OUTPUT_W::new(self, 0)
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
#[doc = "GPIO output set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_SPEC;
impl crate::RegisterSpec for SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`set::R`](R) reader structure"]
impl crate::Readable for SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`set::W`](W) writer structure"]
impl crate::Writable for SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SET to value 0"]
impl crate::Resettable for SET_SPEC {
    const RESET_VALUE: u32 = 0;
}
