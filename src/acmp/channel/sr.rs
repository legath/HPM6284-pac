#[doc = "Register `sr` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Register `sr` writer"]
pub type W = crate::W<SR_SPEC>;
#[doc = "Field `REDGF` reader - Output rising edge flag. Write 1 to clear this flag."]
pub type REDGF_R = crate::BitReader;
#[doc = "Field `REDGF` writer - Output rising edge flag. Write 1 to clear this flag."]
pub type REDGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEDGF` reader - Output falling edge flag. Write 1 to clear this flag."]
pub type FEDGF_R = crate::BitReader;
#[doc = "Field `FEDGF` writer - Output falling edge flag. Write 1 to clear this flag."]
pub type FEDGF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Output rising edge flag. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn redgf(&self) -> REDGF_R {
        REDGF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output falling edge flag. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn fedgf(&self) -> FEDGF_R {
        FEDGF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output rising edge flag. Write 1 to clear this flag."]
    #[inline(always)]
    #[must_use]
    pub fn redgf(&mut self) -> REDGF_W<SR_SPEC> {
        REDGF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Output falling edge flag. Write 1 to clear this flag."]
    #[inline(always)]
    #[must_use]
    pub fn fedgf(&mut self) -> FEDGF_W<SR_SPEC> {
        FEDGF_W::new(self, 1)
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
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sr to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: u32 = 0;
}
