#[doc = "Register `FILTCFG[%s]` reader"]
pub type R = crate::R<FILTCFG_SPEC>;
#[doc = "Register `FILTCFG[%s]` writer"]
pub type W = crate::W<FILTCFG_SPEC>;
#[doc = "Field `FILTLEN` reader - This bitfields defines the filter counter length."]
pub type FILTLEN_R = crate::FieldReader<u16>;
#[doc = "Field `FILTLEN` writer - This bitfields defines the filter counter length."]
pub type FILTLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SYNCEN` reader - set to enable sychronization input signal with TRGM clock"]
pub type SYNCEN_R = crate::BitReader;
#[doc = "Field `SYNCEN` writer - set to enable sychronization input signal with TRGM clock"]
pub type SYNCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OUTINV` reader - 1- Filter will invert the output 0- Filter will not invert the output"]
pub type OUTINV_R = crate::BitReader;
#[doc = "Field `OUTINV` writer - 1- Filter will invert the output 0- Filter will not invert the output"]
pub type OUTINV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - This bitfields defines the filter counter length."]
    #[inline(always)]
    pub fn filtlen(&self) -> FILTLEN_R {
        FILTLEN_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - set to enable sychronization input signal with TRGM clock"]
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - 1- Filter will invert the output 0- Filter will not invert the output"]
    #[inline(always)]
    pub fn outinv(&self) -> OUTINV_R {
        OUTINV_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - This bitfields defines the filter counter length."]
    #[inline(always)]
    #[must_use]
    pub fn filtlen(&mut self) -> FILTLEN_W<FILTCFG_SPEC> {
        FILTLEN_W::new(self, 0)
    }
    #[doc = "Bit 12 - set to enable sychronization input signal with TRGM clock"]
    #[inline(always)]
    #[must_use]
    pub fn syncen(&mut self) -> SYNCEN_W<FILTCFG_SPEC> {
        SYNCEN_W::new(self, 12)
    }
    #[doc = "Bits 13:15 - This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<FILTCFG_SPEC> {
        MODE_W::new(self, 13)
    }
    #[doc = "Bit 16 - 1- Filter will invert the output 0- Filter will not invert the output"]
    #[inline(always)]
    #[must_use]
    pub fn outinv(&mut self) -> OUTINV_W<FILTCFG_SPEC> {
        OUTINV_W::new(self, 16)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filtcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filtcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTCFG_SPEC;
impl crate::RegisterSpec for FILTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filtcfg::R`](R) reader structure"]
impl crate::Readable for FILTCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`filtcfg::W`](W) writer structure"]
impl crate::Writable for FILTCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILTCFG[%s]
to value 0"]
impl crate::Resettable for FILTCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
