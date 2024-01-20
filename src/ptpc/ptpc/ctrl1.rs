#[doc = "Register `ctrl1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `ctrl1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `SS_INCR` reader - constant value used to add ns counter; such as for 50MHz timer clock, set it to 8'd20"]
pub type SS_INCR_R = crate::FieldReader;
#[doc = "Field `SS_INCR` writer - constant value used to add ns counter; such as for 50MHz timer clock, set it to 8'd20"]
pub type SS_INCR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - constant value used to add ns counter; such as for 50MHz timer clock, set it to 8'd20"]
    #[inline(always)]
    pub fn ss_incr(&self) -> SS_INCR_R {
        SS_INCR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - constant value used to add ns counter; such as for 50MHz timer clock, set it to 8'd20"]
    #[inline(always)]
    #[must_use]
    pub fn ss_incr(&mut self) -> SS_INCR_W<CTRL1_SPEC> {
        SS_INCR_W::new(self, 0)
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
#[doc = "Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ctrl1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
