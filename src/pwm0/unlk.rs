#[doc = "Register `unlk` reader"]
pub type R = crate::R<UNLK_SPEC>;
#[doc = "Register `unlk` writer"]
pub type W = crate::W<UNLK_SPEC>;
#[doc = "Field `SHUNLK` reader - write 0xB0382607 to unlock the shadow registers of register offset from 0x04 to 0x78, otherwise the shadow registers can not be written."]
pub type SHUNLK_R = crate::FieldReader<u32>;
#[doc = "Field `SHUNLK` writer - write 0xB0382607 to unlock the shadow registers of register offset from 0x04 to 0x78, otherwise the shadow registers can not be written."]
pub type SHUNLK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - write 0xB0382607 to unlock the shadow registers of register offset from 0x04 to 0x78, otherwise the shadow registers can not be written."]
    #[inline(always)]
    pub fn shunlk(&self) -> SHUNLK_R {
        SHUNLK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - write 0xB0382607 to unlock the shadow registers of register offset from 0x04 to 0x78, otherwise the shadow registers can not be written."]
    #[inline(always)]
    #[must_use]
    pub fn shunlk(&mut self) -> SHUNLK_W<UNLK_SPEC> {
        SHUNLK_W::new(self, 0)
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
#[doc = "Shadow registers unlock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`unlk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unlk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNLK_SPEC;
impl crate::RegisterSpec for UNLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`unlk::R`](R) reader structure"]
impl crate::Readable for UNLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`unlk::W`](W) writer structure"]
impl crate::Writable for UNLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets unlk to value 0"]
impl crate::Resettable for UNLK_SPEC {
    const RESET_VALUE: u32 = 0;
}
