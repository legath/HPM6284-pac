#[doc = "Register `cnt` reader"]
pub type R = crate::R<CNT_SPEC>;
#[doc = "Register `cnt` writer"]
pub type W = crate::W<CNT_SPEC>;
#[doc = "Field `CNT` reader - current clock counter value"]
pub type CNT_R = crate::FieldReader<u32>;
#[doc = "Field `XCNT` reader - current extended counter value"]
pub type XCNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 4:27 - current clock counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits >> 4) & 0x00ff_ffff)
    }
    #[doc = "Bits 28:31 - current extended counter value"]
    #[inline(always)]
    pub fn xcnt(&self) -> XCNT_R {
        XCNT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
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
#[doc = "Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNT_SPEC;
impl crate::RegisterSpec for CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cnt::W`](W) writer structure"]
impl crate::Writable for CNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cnt to value 0"]
impl crate::Resettable for CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
