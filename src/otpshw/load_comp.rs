#[doc = "Register `LOAD_COMP` reader"]
pub type R = crate::R<LOAD_COMP_SPEC>;
#[doc = "Register `LOAD_COMP` writer"]
pub type W = crate::W<LOAD_COMP_SPEC>;
#[doc = "Field `COMPLETE` reader - reload complete sign for 4 regions bit0: region 0 bit1: region1 bit2: region2 bit3: region3"]
pub type COMPLETE_R = crate::FieldReader;
#[doc = "Field `COMPLETE` writer - reload complete sign for 4 regions bit0: region 0 bit1: region1 bit2: region2 bit3: region3"]
pub type COMPLETE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - reload complete sign for 4 regions bit0: region 0 bit1: region1 bit2: region2 bit3: region3"]
    #[inline(always)]
    pub fn complete(&self) -> COMPLETE_R {
        COMPLETE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - reload complete sign for 4 regions bit0: region 0 bit1: region1 bit2: region2 bit3: region3"]
    #[inline(always)]
    #[must_use]
    pub fn complete(&mut self) -> COMPLETE_W<LOAD_COMP_SPEC> {
        COMPLETE_W::new(self, 0)
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
#[doc = "LOAD complete\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`load_comp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load_comp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOAD_COMP_SPEC;
impl crate::RegisterSpec for LOAD_COMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`load_comp::R`](R) reader structure"]
impl crate::Readable for LOAD_COMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`load_comp::W`](W) writer structure"]
impl crate::Writable for LOAD_COMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOAD_COMP to value 0x07"]
impl crate::Resettable for LOAD_COMP_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
