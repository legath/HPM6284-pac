#[doc = "Register `NDAT2` reader"]
pub type R = crate::R<NDAT2_SPEC>;
#[doc = "Register `NDAT2` writer"]
pub type W = crate::W<NDAT2_SPEC>;
#[doc = "Field `ND2` reader - New Data\\[63:32\\]
The register holds the New Data flags of Rx Buffers 32 to 63. The flags are set when the respective Rx Buffer has been updated from a received frame. The flags remain set until the Host clears them. A flag is cleared by writing a ’1’ to the corresponding bit position. Writing a ’0’ has no effect. A hard reset will clear the register. 0= Rx Buffer not updated 1= Rx Buffer updated from new message"]
pub type ND2_R = crate::FieldReader<u32>;
#[doc = "Field `ND2` writer - New Data\\[63:32\\]
The register holds the New Data flags of Rx Buffers 32 to 63. The flags are set when the respective Rx Buffer has been updated from a received frame. The flags remain set until the Host clears them. A flag is cleared by writing a ’1’ to the corresponding bit position. Writing a ’0’ has no effect. A hard reset will clear the register. 0= Rx Buffer not updated 1= Rx Buffer updated from new message"]
pub type ND2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - New Data\\[63:32\\]
The register holds the New Data flags of Rx Buffers 32 to 63. The flags are set when the respective Rx Buffer has been updated from a received frame. The flags remain set until the Host clears them. A flag is cleared by writing a ’1’ to the corresponding bit position. Writing a ’0’ has no effect. A hard reset will clear the register. 0= Rx Buffer not updated 1= Rx Buffer updated from new message"]
    #[inline(always)]
    pub fn nd2(&self) -> ND2_R {
        ND2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - New Data\\[63:32\\]
The register holds the New Data flags of Rx Buffers 32 to 63. The flags are set when the respective Rx Buffer has been updated from a received frame. The flags remain set until the Host clears them. A flag is cleared by writing a ’1’ to the corresponding bit position. Writing a ’0’ has no effect. A hard reset will clear the register. 0= Rx Buffer not updated 1= Rx Buffer updated from new message"]
    #[inline(always)]
    #[must_use]
    pub fn nd2(&mut self) -> ND2_W<NDAT2_SPEC> {
        ND2_W::new(self, 0)
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
#[doc = "new data2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ndat2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ndat2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NDAT2_SPEC;
impl crate::RegisterSpec for NDAT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ndat2::R`](R) reader structure"]
impl crate::Readable for NDAT2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ndat2::W`](W) writer structure"]
impl crate::Writable for NDAT2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NDAT2 to value 0"]
impl crate::Resettable for NDAT2_SPEC {
    const RESET_VALUE: u32 = 0;
}
