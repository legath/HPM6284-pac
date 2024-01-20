#[doc = "Register `READ_CONTROL` reader"]
pub type R = crate::R<READ_CONTROL_SPEC>;
#[doc = "Register `READ_CONTROL` writer"]
pub type W = crate::W<READ_CONTROL_SPEC>;
#[doc = "Field `BLOCK_SMK_READ` reader - symmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out"]
pub type BLOCK_SMK_READ_R = crate::BitReader;
#[doc = "Field `BLOCK_SMK_READ` writer - symmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out"]
pub type BLOCK_SMK_READ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCK_PK_READ` reader - asymmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out"]
pub type BLOCK_PK_READ_R = crate::BitReader;
#[doc = "Field `BLOCK_PK_READ` writer - asymmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out"]
pub type BLOCK_PK_READ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - symmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out"]
    #[inline(always)]
    pub fn block_smk_read(&self) -> BLOCK_SMK_READ_R {
        BLOCK_SMK_READ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - asymmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out"]
    #[inline(always)]
    pub fn block_pk_read(&self) -> BLOCK_PK_READ_R {
        BLOCK_PK_READ_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - symmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out"]
    #[inline(always)]
    #[must_use]
    pub fn block_smk_read(&mut self) -> BLOCK_SMK_READ_W<READ_CONTROL_SPEC> {
        BLOCK_SMK_READ_W::new(self, 0)
    }
    #[doc = "Bit 16 - asymmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out"]
    #[inline(always)]
    #[must_use]
    pub fn block_pk_read(&mut self) -> BLOCK_PK_READ_W<READ_CONTROL_SPEC> {
        BLOCK_PK_READ_W::new(self, 16)
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
#[doc = "key read out control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`read_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`read_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct READ_CONTROL_SPEC;
impl crate::RegisterSpec for READ_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`read_control::R`](R) reader structure"]
impl crate::Readable for READ_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`read_control::W`](W) writer structure"]
impl crate::Writable for READ_CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets READ_CONTROL to value 0"]
impl crate::Resettable for READ_CONTROL_SPEC {
    const RESET_VALUE: u32 = 0;
}
