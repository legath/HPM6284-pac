#[doc = "Register `SOFTPKEY[%s]` reader"]
pub type R = crate::R<SOFTPKEY_SPEC>;
#[doc = "Register `SOFTPKEY[%s]` writer"]
pub type W = crate::W<SOFTPKEY_SPEC>;
#[doc = "Field `KEY` reader - software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0"]
pub type KEY_R = crate::FieldReader<u32>;
#[doc = "Field `KEY` writer - software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0"]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<SOFTPKEY_SPEC> {
        KEY_W::new(self, 0)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softpkey::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softpkey::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOFTPKEY_SPEC;
impl crate::RegisterSpec for SOFTPKEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`softpkey::R`](R) reader structure"]
impl crate::Readable for SOFTPKEY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`softpkey::W`](W) writer structure"]
impl crate::Writable for SOFTPKEY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOFTPKEY[%s]
to value 0"]
impl crate::Resettable for SOFTPKEY_SPEC {
    const RESET_VALUE: u32 = 0;
}
