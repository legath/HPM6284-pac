#[doc = "Register `TXBTIE` reader"]
pub type R = crate::R<TXBTIE_SPEC>;
#[doc = "Register `TXBTIE` writer"]
pub type W = crate::W<TXBTIE_SPEC>;
#[doc = "Field `TIE` reader - Transmission Interrupt Enable Each Tx Buffer has its own Transmission Interrupt Enable bit. 0= Transmission interrupt disabled 1= Transmission interrupt enable"]
pub type TIE_R = crate::FieldReader<u32>;
#[doc = "Field `TIE` writer - Transmission Interrupt Enable Each Tx Buffer has its own Transmission Interrupt Enable bit. 0= Transmission interrupt disabled 1= Transmission interrupt enable"]
pub type TIE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmission Interrupt Enable Each Tx Buffer has its own Transmission Interrupt Enable bit. 0= Transmission interrupt disabled 1= Transmission interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmission Interrupt Enable Each Tx Buffer has its own Transmission Interrupt Enable bit. 0= Transmission interrupt disabled 1= Transmission interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<TXBTIE_SPEC> {
        TIE_W::new(self, 0)
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
#[doc = "tx buffer transmission interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbtie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbtie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBTIE_SPEC;
impl crate::RegisterSpec for TXBTIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbtie::R`](R) reader structure"]
impl crate::Readable for TXBTIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txbtie::W`](W) writer structure"]
impl crate::Writable for TXBTIE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXBTIE to value 0"]
impl crate::Resettable for TXBTIE_SPEC {
    const RESET_VALUE: u32 = 0;
}
