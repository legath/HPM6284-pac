#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    softmkey: [SOFTMKEY; 8],
    softpkey: [SOFTPKEY; 8],
    sec_key_ctl: SEC_KEY_CTL,
    nsc_key_ctl: NSC_KEY_CTL,
    rng: RNG,
    read_control: READ_CONTROL,
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - no description available"]
    #[inline(always)]
    pub const fn softmkey(&self, n: usize) -> &SOFTMKEY {
        &self.softmkey[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - no description available"]
    #[inline(always)]
    pub fn softmkey_iter(&self) -> impl Iterator<Item = &SOFTMKEY> {
        self.softmkey.iter()
    }
    #[doc = "0x00 - no description available"]
    #[inline(always)]
    pub const fn softmkeysfk0(&self) -> &SOFTMKEY {
        self.softmkey(0)
    }
    #[doc = "0x04 - no description available"]
    #[inline(always)]
    pub const fn softmkeysfk1(&self) -> &SOFTMKEY {
        self.softmkey(1)
    }
    #[doc = "0x08 - no description available"]
    #[inline(always)]
    pub const fn softmkeysfk2(&self) -> &SOFTMKEY {
        self.softmkey(2)
    }
    #[doc = "0x0c - no description available"]
    #[inline(always)]
    pub const fn softmkeysfk3(&self) -> &SOFTMKEY {
        self.softmkey(3)
    }
    #[doc = "0x10 - no description available"]
    #[inline(always)]
    pub const fn softmkeysfk4(&self) -> &SOFTMKEY {
        self.softmkey(4)
    }
    #[doc = "0x14 - no description available"]
    #[inline(always)]
    pub const fn softmkeysfk5(&self) -> &SOFTMKEY {
        self.softmkey(5)
    }
    #[doc = "0x18 - no description available"]
    #[inline(always)]
    pub const fn softmkeysfk6(&self) -> &SOFTMKEY {
        self.softmkey(6)
    }
    #[doc = "0x1c - no description available"]
    #[inline(always)]
    pub const fn softmkeysfk7(&self) -> &SOFTMKEY {
        self.softmkey(7)
    }
    #[doc = "0x20..0x40 - no description available"]
    #[inline(always)]
    pub const fn softpkey(&self, n: usize) -> &SOFTPKEY {
        &self.softpkey[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x40 - no description available"]
    #[inline(always)]
    pub fn softpkey_iter(&self) -> impl Iterator<Item = &SOFTPKEY> {
        self.softpkey.iter()
    }
    #[doc = "0x20 - no description available"]
    #[inline(always)]
    pub const fn softpkeyspk0(&self) -> &SOFTPKEY {
        self.softpkey(0)
    }
    #[doc = "0x24 - no description available"]
    #[inline(always)]
    pub const fn softpkeyspk1(&self) -> &SOFTPKEY {
        self.softpkey(1)
    }
    #[doc = "0x28 - no description available"]
    #[inline(always)]
    pub const fn softpkeyspk2(&self) -> &SOFTPKEY {
        self.softpkey(2)
    }
    #[doc = "0x2c - no description available"]
    #[inline(always)]
    pub const fn softpkeyspk3(&self) -> &SOFTPKEY {
        self.softpkey(3)
    }
    #[doc = "0x30 - no description available"]
    #[inline(always)]
    pub const fn softpkeyspk4(&self) -> &SOFTPKEY {
        self.softpkey(4)
    }
    #[doc = "0x34 - no description available"]
    #[inline(always)]
    pub const fn softpkeyspk5(&self) -> &SOFTPKEY {
        self.softpkey(5)
    }
    #[doc = "0x38 - no description available"]
    #[inline(always)]
    pub const fn softpkeyspk6(&self) -> &SOFTPKEY {
        self.softpkey(6)
    }
    #[doc = "0x3c - no description available"]
    #[inline(always)]
    pub const fn softpkeyspk7(&self) -> &SOFTPKEY {
        self.softpkey(7)
    }
    #[doc = "0x40 - secure key generation"]
    #[inline(always)]
    pub const fn sec_key_ctl(&self) -> &SEC_KEY_CTL {
        &self.sec_key_ctl
    }
    #[doc = "0x44 - non-secure key generation"]
    #[inline(always)]
    pub const fn nsc_key_ctl(&self) -> &NSC_KEY_CTL {
        &self.nsc_key_ctl
    }
    #[doc = "0x48 - Random number interface behavior"]
    #[inline(always)]
    pub const fn rng(&self) -> &RNG {
        &self.rng
    }
    #[doc = "0x4c - key read out control"]
    #[inline(always)]
    pub const fn read_control(&self) -> &READ_CONTROL {
        &self.read_control
    }
}
#[doc = "SOFTMKEY (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softmkey::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softmkey::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softmkey`]
module"]
pub type SOFTMKEY = crate::Reg<softmkey::SOFTMKEY_SPEC>;
#[doc = "no description available"]
pub mod softmkey;
#[doc = "SOFTPKEY (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softpkey::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softpkey::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softpkey`]
module"]
pub type SOFTPKEY = crate::Reg<softpkey::SOFTPKEY_SPEC>;
#[doc = "no description available"]
pub mod softpkey;
#[doc = "SEC_KEY_CTL (rw) register accessor: secure key generation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sec_key_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sec_key_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_key_ctl`]
module"]
pub type SEC_KEY_CTL = crate::Reg<sec_key_ctl::SEC_KEY_CTL_SPEC>;
#[doc = "secure key generation"]
pub mod sec_key_ctl;
#[doc = "NSC_KEY_CTL (rw) register accessor: non-secure key generation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nsc_key_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nsc_key_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsc_key_ctl`]
module"]
pub type NSC_KEY_CTL = crate::Reg<nsc_key_ctl::NSC_KEY_CTL_SPEC>;
#[doc = "non-secure key generation"]
pub mod nsc_key_ctl;
#[doc = "RNG (rw) register accessor: Random number interface behavior\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rng::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng`]
module"]
pub type RNG = crate::Reg<rng::RNG_SPEC>;
#[doc = "Random number interface behavior"]
pub mod rng;
#[doc = "READ_CONTROL (rw) register accessor: key read out control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`read_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`read_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@read_control`]
module"]
pub type READ_CONTROL = crate::Reg<read_control::READ_CONTROL_SPEC>;
#[doc = "key read out control"]
pub mod read_control;
