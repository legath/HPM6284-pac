#[doc = "Register `WrEn` reader"]
pub type R = crate::R<WR_EN_SPEC>;
#[doc = "Register `WrEn` writer"]
pub type W = crate::W<WR_EN_SPEC>;
#[doc = "Field `WEN` writer - Write the magic code to disable the write protection of the Control Register and the Restart Register."]
pub type WEN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Write the magic code to disable the write protection of the Control Register and the Restart Register."]
    #[inline(always)]
    #[must_use]
    pub fn wen(&mut self) -> WEN_W<WR_EN_SPEC> {
        WEN_W::new(self, 0)
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
#[doc = "Write Protection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WR_EN_SPEC;
impl crate::RegisterSpec for WR_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wr_en::R`](R) reader structure"]
impl crate::Readable for WR_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wr_en::W`](W) writer structure"]
impl crate::Writable for WR_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WrEn to value 0"]
impl crate::Resettable for WR_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
