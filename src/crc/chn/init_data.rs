#[doc = "Register `init_data` reader"]
pub type R = crate::R<INIT_DATA_SPEC>;
#[doc = "Register `init_data` writer"]
pub type W = crate::W<INIT_DATA_SPEC>;
#[doc = "Field `INIT_DATA` reader - initial data of CRC"]
pub type INIT_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `INIT_DATA` writer - initial data of CRC"]
pub type INIT_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - initial data of CRC"]
    #[inline(always)]
    pub fn init_data(&self) -> INIT_DATA_R {
        INIT_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - initial data of CRC"]
    #[inline(always)]
    #[must_use]
    pub fn init_data(&mut self) -> INIT_DATA_W<INIT_DATA_SPEC> {
        INIT_DATA_W::new(self, 0)
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
#[doc = "chn&amp;index0 init_data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`init_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`init_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INIT_DATA_SPEC;
impl crate::RegisterSpec for INIT_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`init_data::R`](R) reader structure"]
impl crate::Readable for INIT_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`init_data::W`](W) writer structure"]
impl crate::Writable for INIT_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets init_data to value 0"]
impl crate::Resettable for INIT_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
