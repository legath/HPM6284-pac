#[doc = "Register `ASYNC` reader"]
pub type R = crate::R<ASYNC_SPEC>;
#[doc = "Register `ASYNC` writer"]
pub type W = crate::W<ASYNC_SPEC>;
#[doc = "Field `VALUE` reader - Value of async mode to compare"]
pub type VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `VALUE` writer - Value of async mode to compare"]
pub type VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `POLARITY` reader - Polarity of internal comparator"]
pub type POLARITY_R = crate::BitReader;
#[doc = "Field `POLARITY` writer - Polarity of internal comparator"]
pub type POLARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNC_TYPE` reader - Compare hotter than or colder than in asynchoronous mode 0: hotter than 1: colder than"]
pub type ASYNC_TYPE_R = crate::BitReader;
#[doc = "Field `ASYNC_TYPE` writer - Compare hotter than or colder than in asynchoronous mode 0: hotter than 1: colder than"]
pub type ASYNC_TYPE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Value of async mode to compare"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16 - Polarity of internal comparator"]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Compare hotter than or colder than in asynchoronous mode 0: hotter than 1: colder than"]
    #[inline(always)]
    pub fn async_type(&self) -> ASYNC_TYPE_R {
        ASYNC_TYPE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Value of async mode to compare"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<ASYNC_SPEC> {
        VALUE_W::new(self, 0)
    }
    #[doc = "Bit 16 - Polarity of internal comparator"]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> POLARITY_W<ASYNC_SPEC> {
        POLARITY_W::new(self, 16)
    }
    #[doc = "Bit 24 - Compare hotter than or colder than in asynchoronous mode 0: hotter than 1: colder than"]
    #[inline(always)]
    #[must_use]
    pub fn async_type(&mut self) -> ASYNC_TYPE_W<ASYNC_SPEC> {
        ASYNC_TYPE_W::new(self, 24)
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
#[doc = "Configuration in asynchronous mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`async_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`async_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASYNC_SPEC;
impl crate::RegisterSpec for ASYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`async_::R`](R) reader structure"]
impl crate::Readable for ASYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`async_::W`](W) writer structure"]
impl crate::Writable for ASYNC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ASYNC to value 0"]
impl crate::Resettable for ASYNC_SPEC {
    const RESET_VALUE: u32 = 0;
}
