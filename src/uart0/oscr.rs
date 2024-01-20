#[doc = "Register `OSCR` reader"]
pub type R = crate::R<OSCR_SPEC>;
#[doc = "Register `OSCR` writer"]
pub type W = crate::W<OSCR_SPEC>;
#[doc = "Field `OSC` reader - Over-sample control The value must be an even number; any odd value writes to this field will be converted to an even value. OSC=0: The over-sample ratio is 32 OSC&lt;=8: The over-sample ratio is 8 8 &lt; OSC&lt; 32: The over sample ratio is OSC"]
pub type OSC_R = crate::FieldReader;
#[doc = "Field `OSC` writer - Over-sample control The value must be an even number; any odd value writes to this field will be converted to an even value. OSC=0: The over-sample ratio is 32 OSC&lt;=8: The over-sample ratio is 8 8 &lt; OSC&lt; 32: The over sample ratio is OSC"]
pub type OSC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Over-sample control The value must be an even number; any odd value writes to this field will be converted to an even value. OSC=0: The over-sample ratio is 32 OSC&lt;=8: The over-sample ratio is 8 8 &lt; OSC&lt; 32: The over sample ratio is OSC"]
    #[inline(always)]
    pub fn osc(&self) -> OSC_R {
        OSC_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Over-sample control The value must be an even number; any odd value writes to this field will be converted to an even value. OSC=0: The over-sample ratio is 32 OSC&lt;=8: The over-sample ratio is 8 8 &lt; OSC&lt; 32: The over sample ratio is OSC"]
    #[inline(always)]
    #[must_use]
    pub fn osc(&mut self) -> OSC_W<OSCR_SPEC> {
        OSC_W::new(self, 0)
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
#[doc = "Over Sample Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCR_SPEC;
impl crate::RegisterSpec for OSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oscr::R`](R) reader structure"]
impl crate::Readable for OSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oscr::W`](W) writer structure"]
impl crate::Writable for OSCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSCR to value 0x10"]
impl crate::Resettable for OSCR_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
