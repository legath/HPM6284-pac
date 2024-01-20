#[doc = "Register `TXWRD[%s]` reader"]
pub type R = crate::R<TXWRD_SPEC>;
#[doc = "Register `TXWRD[%s]` writer"]
pub type W = crate::W<TXWRD_SPEC>;
#[doc = "Field `TXFIFO` writer - TXFIFO for sending message to other core, FIFO size, 4x32 can write one of the word address to push data to the FIFO; can also use 4x32 burst write from 0x010 to push 4 words to the FIFO."]
pub type TXFIFO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - TXFIFO for sending message to other core, FIFO size, 4x32 can write one of the word address to push data to the FIFO; can also use 4x32 burst write from 0x010 to push 4 words to the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn txfifo(&mut self) -> TXFIFO_W<TXWRD_SPEC> {
        TXFIFO_W::new(self, 0)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txwrd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txwrd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXWRD_SPEC;
impl crate::RegisterSpec for TXWRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txwrd::R`](R) reader structure"]
impl crate::Readable for TXWRD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txwrd::W`](W) writer structure"]
impl crate::Writable for TXWRD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXWRD[%s]
to value 0"]
impl crate::Resettable for TXWRD_SPEC {
    const RESET_VALUE: u32 = 0;
}
