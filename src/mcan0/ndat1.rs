#[doc = "Register `NDAT1` reader"]
pub type R = crate::R<NDAT1_SPEC>;
#[doc = "Register `NDAT1` writer"]
pub type W = crate::W<NDAT1_SPEC>;
#[doc = "Field `ND1` reader - New Data\\[31:0\\]
The register holds the New Data flags of Rx Buffers 0 to 31. The flags are set when the respective Rx Buffer has been updated from a received frame. The flags remain set until the Host clears them.A flag is cleared by writing a ’1’ to the corresponding bit position. Writing a ’0’ has no effect. A hard reset will clear the register. 0= Rx Buffer not updated 1= Rx Buffer updated from new message"]
pub type ND1_R = crate::FieldReader<u32>;
#[doc = "Field `ND1` writer - New Data\\[31:0\\]
The register holds the New Data flags of Rx Buffers 0 to 31. The flags are set when the respective Rx Buffer has been updated from a received frame. The flags remain set until the Host clears them.A flag is cleared by writing a ’1’ to the corresponding bit position. Writing a ’0’ has no effect. A hard reset will clear the register. 0= Rx Buffer not updated 1= Rx Buffer updated from new message"]
pub type ND1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - New Data\\[31:0\\]
The register holds the New Data flags of Rx Buffers 0 to 31. The flags are set when the respective Rx Buffer has been updated from a received frame. The flags remain set until the Host clears them.A flag is cleared by writing a ’1’ to the corresponding bit position. Writing a ’0’ has no effect. A hard reset will clear the register. 0= Rx Buffer not updated 1= Rx Buffer updated from new message"]
    #[inline(always)]
    pub fn nd1(&self) -> ND1_R {
        ND1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - New Data\\[31:0\\]
The register holds the New Data flags of Rx Buffers 0 to 31. The flags are set when the respective Rx Buffer has been updated from a received frame. The flags remain set until the Host clears them.A flag is cleared by writing a ’1’ to the corresponding bit position. Writing a ’0’ has no effect. A hard reset will clear the register. 0= Rx Buffer not updated 1= Rx Buffer updated from new message"]
    #[inline(always)]
    #[must_use]
    pub fn nd1(&mut self) -> ND1_W<NDAT1_SPEC> {
        ND1_W::new(self, 0)
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
#[doc = "new data1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ndat1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ndat1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NDAT1_SPEC;
impl crate::RegisterSpec for NDAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ndat1::R`](R) reader structure"]
impl crate::Readable for NDAT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ndat1::W`](W) writer structure"]
impl crate::Writable for NDAT1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NDAT1 to value 0"]
impl crate::Resettable for NDAT1_SPEC {
    const RESET_VALUE: u32 = 0;
}
