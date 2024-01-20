#[doc = "Register `buf_length` reader"]
pub type R = crate::R<BUF_LENGTH_SPEC>;
#[doc = "Register `buf_length` writer"]
pub type W = crate::W<BUF_LENGTH_SPEC>;
#[doc = "Field `BUF0_LEN` reader - No description avaiable"]
pub type BUF0_LEN_R = crate::FieldReader<u16>;
#[doc = "Field `BUF0_LEN` writer - No description avaiable"]
pub type BUF0_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BUF1_LEN` reader - buffer length, 1 indicate one 32bit date, 256K-byte max for one buffer"]
pub type BUF1_LEN_R = crate::FieldReader<u16>;
#[doc = "Field `BUF1_LEN` writer - buffer length, 1 indicate one 32bit date, 256K-byte max for one buffer"]
pub type BUF1_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - No description avaiable"]
    #[inline(always)]
    pub fn buf0_len(&self) -> BUF0_LEN_R {
        BUF0_LEN_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - buffer length, 1 indicate one 32bit date, 256K-byte max for one buffer"]
    #[inline(always)]
    pub fn buf1_len(&self) -> BUF1_LEN_R {
        BUF1_LEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn buf0_len(&mut self) -> BUF0_LEN_W<BUF_LENGTH_SPEC> {
        BUF0_LEN_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - buffer length, 1 indicate one 32bit date, 256K-byte max for one buffer"]
    #[inline(always)]
    #[must_use]
    pub fn buf1_len(&mut self) -> BUF1_LEN_W<BUF_LENGTH_SPEC> {
        BUF1_LEN_W::new(self, 16)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_length::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_length::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUF_LENGTH_SPEC;
impl crate::RegisterSpec for BUF_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_length::R`](R) reader structure"]
impl crate::Readable for BUF_LENGTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buf_length::W`](W) writer structure"]
impl crate::Writable for BUF_LENGTH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets buf_length to value 0"]
impl crate::Resettable for BUF_LENGTH_SPEC {
    const RESET_VALUE: u32 = 0;
}
