#[doc = "Register `DIV[%s]` reader"]
pub type R = crate::R<DIV_SPEC>;
#[doc = "Register `DIV[%s]` writer"]
pub type W = crate::W<DIV_SPEC>;
#[doc = "Field `DIV` reader - Divider factor, divider factor is DIV/5 + 1 0: divide by 1 1: divide by 1.2 2: divide by 1.4 . . . 63: divide by 13.6"]
pub type DIV_R = crate::FieldReader;
#[doc = "Field `DIV` writer - Divider factor, divider factor is DIV/5 + 1 0: divide by 1 1: divide by 1.2 2: divide by 1.4 . . . 63: divide by 13.6"]
pub type DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ENABLE` reader - Divider enable status 0: Divider is off 1: Divider is on"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `RESPONSE` reader - Divider response status 0: Divider is not stable 1: Divider is stable for use"]
pub type RESPONSE_R = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy flag 0: divider is working 1: divider is changing status"]
pub type BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Divider factor, divider factor is DIV/5 + 1 0: divide by 1 1: divide by 1.2 2: divide by 1.4 . . . 63: divide by 13.6"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 28 - Divider enable status 0: Divider is off 1: Divider is on"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Divider response status 0: Divider is not stable 1: Divider is stable for use"]
    #[inline(always)]
    pub fn response(&self) -> RESPONSE_R {
        RESPONSE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Busy flag 0: divider is working 1: divider is changing status"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Divider factor, divider factor is DIV/5 + 1 0: divide by 1 1: divide by 1.2 2: divide by 1.4 . . . 63: divide by 13.6"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<DIV_SPEC> {
        DIV_W::new(self, 0)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIV_SPEC;
impl crate::RegisterSpec for DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div::R`](R) reader structure"]
impl crate::Readable for DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`div::W`](W) writer structure"]
impl crate::Writable for DIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIV[%s]
to value 0"]
impl crate::Resettable for DIV_SPEC {
    const RESET_VALUE: u32 = 0;
}
