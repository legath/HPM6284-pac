#[doc = "Register `FUNC_CTL` reader"]
pub type R = crate::R<FUNC_CTL_SPEC>;
#[doc = "Register `FUNC_CTL` writer"]
pub type W = crate::W<FUNC_CTL_SPEC>;
#[doc = "Field `ALT_SELECT` reader - alt select 0: ALT0 1: ALT1 … 31:ALT31"]
pub type ALT_SELECT_R = crate::FieldReader;
#[doc = "Field `ALT_SELECT` writer - alt select 0: ALT0 1: ALT1 … 31:ALT31"]
pub type ALT_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ANALOG` reader - select analog pin in pad 0: disable 1: enable"]
pub type ANALOG_R = crate::BitReader;
#[doc = "Field `ANALOG` writer - select analog pin in pad 0: disable 1: enable"]
pub type ANALOG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOP_BACK` reader - force input on 0: disable 1: enable"]
pub type LOOP_BACK_R = crate::BitReader;
#[doc = "Field `LOOP_BACK` writer - force input on 0: disable 1: enable"]
pub type LOOP_BACK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - alt select 0: ALT0 1: ALT1 … 31:ALT31"]
    #[inline(always)]
    pub fn alt_select(&self) -> ALT_SELECT_R {
        ALT_SELECT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - select analog pin in pad 0: disable 1: enable"]
    #[inline(always)]
    pub fn analog(&self) -> ANALOG_R {
        ANALOG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - force input on 0: disable 1: enable"]
    #[inline(always)]
    pub fn loop_back(&self) -> LOOP_BACK_R {
        LOOP_BACK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - alt select 0: ALT0 1: ALT1 … 31:ALT31"]
    #[inline(always)]
    #[must_use]
    pub fn alt_select(&mut self) -> ALT_SELECT_W<FUNC_CTL_SPEC> {
        ALT_SELECT_W::new(self, 0)
    }
    #[doc = "Bit 8 - select analog pin in pad 0: disable 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn analog(&mut self) -> ANALOG_W<FUNC_CTL_SPEC> {
        ANALOG_W::new(self, 8)
    }
    #[doc = "Bit 16 - force input on 0: disable 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn loop_back(&mut self) -> LOOP_BACK_W<FUNC_CTL_SPEC> {
        LOOP_BACK_W::new(self, 16)
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
#[doc = "ALT SELECT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC_CTL_SPEC;
impl crate::RegisterSpec for FUNC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func_ctl::R`](R) reader structure"]
impl crate::Readable for FUNC_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func_ctl::W`](W) writer structure"]
impl crate::Writable for FUNC_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FUNC_CTL to value 0"]
impl crate::Resettable for FUNC_CTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
