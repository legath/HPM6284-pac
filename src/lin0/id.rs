#[doc = "Register `id` reader"]
pub type R = crate::R<ID_SPEC>;
#[doc = "Register `id` writer"]
pub type W = crate::W<ID_SPEC>;
#[doc = "Field `ID` reader - id register"]
pub type ID_R = crate::FieldReader;
#[doc = "Field `ID` writer - id register"]
pub type ID_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - id register"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - id register"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<ID_SPEC> {
        ID_W::new(self, 0)
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
#[doc = "id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ID_SPEC;
impl crate::RegisterSpec for ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id::R`](R) reader structure"]
impl crate::Readable for ID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`id::W`](W) writer structure"]
impl crate::Writable for ID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets id to value 0"]
impl crate::Resettable for ID_SPEC {
    const RESET_VALUE: u32 = 0;
}
