#[doc = "Register `NSC_KEY_CTL` reader"]
pub type R = crate::R<NSC_KEY_CTL_SPEC>;
#[doc = "Register `NSC_KEY_CTL` writer"]
pub type W = crate::W<NSC_KEY_CTL_SPEC>;
#[doc = "Field `KEY_SEL` reader - non-secure symmtric key synthesize setting, key is a XOR of followings bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected"]
pub type KEY_SEL_R = crate::FieldReader;
#[doc = "Field `KEY_SEL` writer - non-secure symmtric key synthesize setting, key is a XOR of followings bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected"]
pub type KEY_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FMK_SEL` reader - fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use origin value in fuse symmetric key"]
pub type FMK_SEL_R = crate::BitReader;
#[doc = "Field `FMK_SEL` writer - fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use origin value in fuse symmetric key"]
pub type FMK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZMK_SEL` reader - batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key"]
pub type ZMK_SEL_R = crate::BitReader;
#[doc = "Field `ZMK_SEL` writer - batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key"]
pub type ZMK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMK_SEL` reader - software symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key"]
pub type SMK_SEL_R = crate::BitReader;
#[doc = "Field `SMK_SEL` writer - software symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key"]
pub type SMK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SK_VAL` reader - session key valid 0: session key is all 0's and not usable 1: session key is valid"]
pub type SK_VAL_R = crate::BitReader;
#[doc = "Field `LOCK_NSC_CTL` reader - block non-secure state key setting being changed"]
pub type LOCK_NSC_CTL_R = crate::BitReader;
#[doc = "Field `LOCK_NSC_CTL` writer - block non-secure state key setting being changed"]
pub type LOCK_NSC_CTL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - non-secure symmtric key synthesize setting, key is a XOR of followings bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected"]
    #[inline(always)]
    pub fn key_sel(&self) -> KEY_SEL_R {
        KEY_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use origin value in fuse symmetric key"]
    #[inline(always)]
    pub fn fmk_sel(&self) -> FMK_SEL_R {
        FMK_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key"]
    #[inline(always)]
    pub fn zmk_sel(&self) -> ZMK_SEL_R {
        ZMK_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - software symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key"]
    #[inline(always)]
    pub fn smk_sel(&self) -> SMK_SEL_R {
        SMK_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - session key valid 0: session key is all 0's and not usable 1: session key is valid"]
    #[inline(always)]
    pub fn sk_val(&self) -> SK_VAL_R {
        SK_VAL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - block non-secure state key setting being changed"]
    #[inline(always)]
    pub fn lock_nsc_ctl(&self) -> LOCK_NSC_CTL_R {
        LOCK_NSC_CTL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - non-secure symmtric key synthesize setting, key is a XOR of followings bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected"]
    #[inline(always)]
    #[must_use]
    pub fn key_sel(&mut self) -> KEY_SEL_W<NSC_KEY_CTL_SPEC> {
        KEY_SEL_W::new(self, 0)
    }
    #[doc = "Bit 4 - fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use origin value in fuse symmetric key"]
    #[inline(always)]
    #[must_use]
    pub fn fmk_sel(&mut self) -> FMK_SEL_W<NSC_KEY_CTL_SPEC> {
        FMK_SEL_W::new(self, 4)
    }
    #[doc = "Bit 8 - batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key"]
    #[inline(always)]
    #[must_use]
    pub fn zmk_sel(&mut self) -> ZMK_SEL_W<NSC_KEY_CTL_SPEC> {
        ZMK_SEL_W::new(self, 8)
    }
    #[doc = "Bit 12 - software symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key"]
    #[inline(always)]
    #[must_use]
    pub fn smk_sel(&mut self) -> SMK_SEL_W<NSC_KEY_CTL_SPEC> {
        SMK_SEL_W::new(self, 12)
    }
    #[doc = "Bit 31 - block non-secure state key setting being changed"]
    #[inline(always)]
    #[must_use]
    pub fn lock_nsc_ctl(&mut self) -> LOCK_NSC_CTL_W<NSC_KEY_CTL_SPEC> {
        LOCK_NSC_CTL_W::new(self, 31)
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
#[doc = "non-secure key generation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nsc_key_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nsc_key_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NSC_KEY_CTL_SPEC;
impl crate::RegisterSpec for NSC_KEY_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nsc_key_ctl::R`](R) reader structure"]
impl crate::Readable for NSC_KEY_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nsc_key_ctl::W`](W) writer structure"]
impl crate::Writable for NSC_KEY_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NSC_KEY_CTL to value 0"]
impl crate::Resettable for NSC_KEY_CTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
