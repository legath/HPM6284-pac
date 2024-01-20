#[doc = "Register `ENDPTLISTADDR` reader"]
pub type R = crate::R<ENDPTLISTADDR_SPEC>;
#[doc = "Register `ENDPTLISTADDR` writer"]
pub type W = crate::W<ENDPTLISTADDR_SPEC>;
#[doc = "Field `EPBASE` reader - EPBASE Endpoint List Pointer(Low). These bits correspond to memory address signals \\[31:11\\], respectively. This field will reference a list of up to 32 Queue Head (QH) (that is, one queue head per endpoint &amp; direction)."]
pub type EPBASE_R = crate::FieldReader<u32>;
#[doc = "Field `EPBASE` writer - EPBASE Endpoint List Pointer(Low). These bits correspond to memory address signals \\[31:11\\], respectively. This field will reference a list of up to 32 Queue Head (QH) (that is, one queue head per endpoint &amp; direction)."]
pub type EPBASE_W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 11:31 - EPBASE Endpoint List Pointer(Low). These bits correspond to memory address signals \\[31:11\\], respectively. This field will reference a list of up to 32 Queue Head (QH) (that is, one queue head per endpoint &amp; direction)."]
    #[inline(always)]
    pub fn epbase(&self) -> EPBASE_R {
        EPBASE_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 11:31 - EPBASE Endpoint List Pointer(Low). These bits correspond to memory address signals \\[31:11\\], respectively. This field will reference a list of up to 32 Queue Head (QH) (that is, one queue head per endpoint &amp; direction)."]
    #[inline(always)]
    #[must_use]
    pub fn epbase(&mut self) -> EPBASE_W<ENDPTLISTADDR_SPEC> {
        EPBASE_W::new(self, 11)
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
#[doc = "Endpoint List Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endptlistaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endptlistaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENDPTLISTADDR_SPEC;
impl crate::RegisterSpec for ENDPTLISTADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`endptlistaddr::R`](R) reader structure"]
impl crate::Readable for ENDPTLISTADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`endptlistaddr::W`](W) writer structure"]
impl crate::Writable for ENDPTLISTADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENDPTLISTADDR to value 0"]
impl crate::Resettable for ENDPTLISTADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
