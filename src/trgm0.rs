#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    filtcfg: [FILTCFG; 20],
    _reserved1: [u8; 0xb0],
    trgocfg: [TRGOCFG; 67],
    _reserved2: [u8; 0xf4],
    dmacfg: [DMACFG; 4],
    _reserved3: [u8; 0xf0],
    gcr: GCR,
}
impl RegisterBlock {
    #[doc = "0x00..0x50 - no description available"]
    #[inline(always)]
    pub const fn filtcfg(&self, n: usize) -> &FILTCFG {
        &self.filtcfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x50 - no description available"]
    #[inline(always)]
    pub fn filtcfg_iter(&self) -> impl Iterator<Item = &FILTCFG> {
        self.filtcfg.iter()
    }
    #[doc = "0x00 - no description available"]
    #[inline(always)]
    pub const fn filtcfgpwm_in0(&self) -> &FILTCFG {
        self.filtcfg(0)
    }
    #[doc = "0x04 - no description available"]
    #[inline(always)]
    pub const fn filtcfgpwm_in1(&self) -> &FILTCFG {
        self.filtcfg(1)
    }
    #[doc = "0x08 - no description available"]
    #[inline(always)]
    pub const fn filtcfgpwm_in2(&self) -> &FILTCFG {
        self.filtcfg(2)
    }
    #[doc = "0x0c - no description available"]
    #[inline(always)]
    pub const fn filtcfgpwm_in3(&self) -> &FILTCFG {
        self.filtcfg(3)
    }
    #[doc = "0x10 - no description available"]
    #[inline(always)]
    pub const fn filtcfgpwm_in4(&self) -> &FILTCFG {
        self.filtcfg(4)
    }
    #[doc = "0x14 - no description available"]
    #[inline(always)]
    pub const fn filtcfgpwm_in5(&self) -> &FILTCFG {
        self.filtcfg(5)
    }
    #[doc = "0x18 - no description available"]
    #[inline(always)]
    pub const fn filtcfgpwm_in6(&self) -> &FILTCFG {
        self.filtcfg(6)
    }
    #[doc = "0x1c - no description available"]
    #[inline(always)]
    pub const fn filtcfgpwm_in7(&self) -> &FILTCFG {
        self.filtcfg(7)
    }
    #[doc = "0x20 - no description available"]
    #[inline(always)]
    pub const fn filtcfgtrgm_in0(&self) -> &FILTCFG {
        self.filtcfg(8)
    }
    #[doc = "0x24 - no description available"]
    #[inline(always)]
    pub const fn filtcfgtrgm_in1(&self) -> &FILTCFG {
        self.filtcfg(9)
    }
    #[doc = "0x28 - no description available"]
    #[inline(always)]
    pub const fn filtcfgtrgm_in2(&self) -> &FILTCFG {
        self.filtcfg(10)
    }
    #[doc = "0x2c - no description available"]
    #[inline(always)]
    pub const fn filtcfgtrgm_in3(&self) -> &FILTCFG {
        self.filtcfg(11)
    }
    #[doc = "0x30 - no description available"]
    #[inline(always)]
    pub const fn filtcfgtrgm_in4(&self) -> &FILTCFG {
        self.filtcfg(12)
    }
    #[doc = "0x34 - no description available"]
    #[inline(always)]
    pub const fn filtcfgtrgm_in5(&self) -> &FILTCFG {
        self.filtcfg(13)
    }
    #[doc = "0x38 - no description available"]
    #[inline(always)]
    pub const fn filtcfgtrgm_in6(&self) -> &FILTCFG {
        self.filtcfg(14)
    }
    #[doc = "0x3c - no description available"]
    #[inline(always)]
    pub const fn filtcfgtrgm_in7(&self) -> &FILTCFG {
        self.filtcfg(15)
    }
    #[doc = "0x40 - no description available"]
    #[inline(always)]
    pub const fn filtcfgtrgm_in8(&self) -> &FILTCFG {
        self.filtcfg(16)
    }
    #[doc = "0x44 - no description available"]
    #[inline(always)]
    pub const fn filtcfgtrgm_in9(&self) -> &FILTCFG {
        self.filtcfg(17)
    }
    #[doc = "0x48 - no description available"]
    #[inline(always)]
    pub const fn filtcfgtrgm_in10(&self) -> &FILTCFG {
        self.filtcfg(18)
    }
    #[doc = "0x4c - no description available"]
    #[inline(always)]
    pub const fn filtcfgtrgm_in11(&self) -> &FILTCFG {
        self.filtcfg(19)
    }
    #[doc = "0x100..0x20c - no description available"]
    #[inline(always)]
    pub const fn trgocfg(&self, n: usize) -> &TRGOCFG {
        &self.trgocfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x20c - no description available"]
    #[inline(always)]
    pub fn trgocfg_iter(&self) -> impl Iterator<Item = &TRGOCFG> {
        self.trgocfg.iter()
    }
    #[doc = "0x100 - no description available"]
    #[inline(always)]
    pub const fn trgocfgtrgm_out0(&self) -> &TRGOCFG {
        self.trgocfg(0)
    }
    #[doc = "0x104 - no description available"]
    #[inline(always)]
    pub const fn trgocfgtrgm_out1(&self) -> &TRGOCFG {
        self.trgocfg(1)
    }
    #[doc = "0x108 - no description available"]
    #[inline(always)]
    pub const fn trgocfgtrgm_out2(&self) -> &TRGOCFG {
        self.trgocfg(2)
    }
    #[doc = "0x10c - no description available"]
    #[inline(always)]
    pub const fn trgocfgtrgm_out3(&self) -> &TRGOCFG {
        self.trgocfg(3)
    }
    #[doc = "0x110 - no description available"]
    #[inline(always)]
    pub const fn trgocfgtrgm_out4(&self) -> &TRGOCFG {
        self.trgocfg(4)
    }
    #[doc = "0x114 - no description available"]
    #[inline(always)]
    pub const fn trgocfgtrgm_out5(&self) -> &TRGOCFG {
        self.trgocfg(5)
    }
    #[doc = "0x118 - no description available"]
    #[inline(always)]
    pub const fn trgocfgtrgm_out6(&self) -> &TRGOCFG {
        self.trgocfg(6)
    }
    #[doc = "0x11c - no description available"]
    #[inline(always)]
    pub const fn trgocfgtrgm_out7(&self) -> &TRGOCFG {
        self.trgocfg(7)
    }
    #[doc = "0x120 - no description available"]
    #[inline(always)]
    pub const fn trgocfgtrgm_out8(&self) -> &TRGOCFG {
        self.trgocfg(8)
    }
    #[doc = "0x124 - no description available"]
    #[inline(always)]
    pub const fn trgocfgtrgm_out9(&self) -> &TRGOCFG {
        self.trgocfg(9)
    }
    #[doc = "0x128 - no description available"]
    #[inline(always)]
    pub const fn trgocfgtrgm_out10(&self) -> &TRGOCFG {
        self.trgocfg(10)
    }
    #[doc = "0x12c - no description available"]
    #[inline(always)]
    pub const fn trgocfgtrgm_out11(&self) -> &TRGOCFG {
        self.trgocfg(11)
    }
    #[doc = "0x130 - no description available"]
    #[inline(always)]
    pub const fn trgocfgtrgm_outx0(&self) -> &TRGOCFG {
        self.trgocfg(12)
    }
    #[doc = "0x134 - no description available"]
    #[inline(always)]
    pub const fn trgocfgtrgm_outx1(&self) -> &TRGOCFG {
        self.trgocfg(13)
    }
    #[doc = "0x138 - no description available"]
    #[inline(always)]
    pub const fn trgocfgpwm_synci(&self) -> &TRGOCFG {
        self.trgocfg(14)
    }
    #[doc = "0x13c - no description available"]
    #[inline(always)]
    pub const fn trgocfgpwm_frci(&self) -> &TRGOCFG {
        self.trgocfg(15)
    }
    #[doc = "0x140 - no description available"]
    #[inline(always)]
    pub const fn trgocfgpwm_frcsynci(&self) -> &TRGOCFG {
        self.trgocfg(16)
    }
    #[doc = "0x144 - no description available"]
    #[inline(always)]
    pub const fn trgocfgpwm_shrldsynci(&self) -> &TRGOCFG {
        self.trgocfg(17)
    }
    #[doc = "0x148 - no description available"]
    #[inline(always)]
    pub const fn trgocfgpwm_faulti0(&self) -> &TRGOCFG {
        self.trgocfg(18)
    }
    #[doc = "0x14c - no description available"]
    #[inline(always)]
    pub const fn trgocfgpwm_faulti1(&self) -> &TRGOCFG {
        self.trgocfg(19)
    }
    #[doc = "0x150 - no description available"]
    #[inline(always)]
    pub const fn trgocfgpwm_faulti2(&self) -> &TRGOCFG {
        self.trgocfg(20)
    }
    #[doc = "0x154 - no description available"]
    #[inline(always)]
    pub const fn trgocfgpwm_faulti3(&self) -> &TRGOCFG {
        self.trgocfg(21)
    }
    #[doc = "0x158 - no description available"]
    #[inline(always)]
    pub const fn trgocfgpwm_in8(&self) -> &TRGOCFG {
        self.trgocfg(22)
    }
    #[doc = "0x15c - no description available"]
    #[inline(always)]
    pub const fn trgocfgpwm_in9(&self) -> &TRGOCFG {
        self.trgocfg(23)
    }
    #[doc = "0x160 - no description available"]
    #[inline(always)]
    pub const fn trgocfgpwm_in10(&self) -> &TRGOCFG {
        self.trgocfg(24)
    }
    #[doc = "0x164 - no description available"]
    #[inline(always)]
    pub const fn trgocfgpwm_in11(&self) -> &TRGOCFG {
        self.trgocfg(25)
    }
    #[doc = "0x168 - no description available"]
    #[inline(always)]
    pub const fn trgocfgpwm_in12(&self) -> &TRGOCFG {
        self.trgocfg(26)
    }
    #[doc = "0x16c - no description available"]
    #[inline(always)]
    pub const fn trgocfgpwm_in13(&self) -> &TRGOCFG {
        self.trgocfg(27)
    }
    #[doc = "0x170 - no description available"]
    #[inline(always)]
    pub const fn trgocfgpwm_in14(&self) -> &TRGOCFG {
        self.trgocfg(28)
    }
    #[doc = "0x174 - no description available"]
    #[inline(always)]
    pub const fn trgocfgpwm_in15(&self) -> &TRGOCFG {
        self.trgocfg(29)
    }
    #[doc = "0x178 - no description available"]
    #[inline(always)]
    pub const fn trgocfgpla_in0(&self) -> &TRGOCFG {
        self.trgocfg(30)
    }
    #[doc = "0x17c - no description available"]
    #[inline(always)]
    pub const fn trgocfgpla_in1(&self) -> &TRGOCFG {
        self.trgocfg(31)
    }
    #[doc = "0x180 - no description available"]
    #[inline(always)]
    pub const fn trgocfgpla_in2(&self) -> &TRGOCFG {
        self.trgocfg(32)
    }
    #[doc = "0x184 - no description available"]
    #[inline(always)]
    pub const fn trgocfgpla_in3(&self) -> &TRGOCFG {
        self.trgocfg(33)
    }
    #[doc = "0x188 - no description available"]
    #[inline(always)]
    pub const fn trgocfgpla_in4(&self) -> &TRGOCFG {
        self.trgocfg(34)
    }
    #[doc = "0x18c - no description available"]
    #[inline(always)]
    pub const fn trgocfgpla_in5(&self) -> &TRGOCFG {
        self.trgocfg(35)
    }
    #[doc = "0x190 - no description available"]
    #[inline(always)]
    pub const fn trgocfgpla_in6(&self) -> &TRGOCFG {
        self.trgocfg(36)
    }
    #[doc = "0x194 - no description available"]
    #[inline(always)]
    pub const fn trgocfgpla_in7(&self) -> &TRGOCFG {
        self.trgocfg(37)
    }
    #[doc = "0x198 - no description available"]
    #[inline(always)]
    pub const fn trgocfgqei_a(&self) -> &TRGOCFG {
        self.trgocfg(38)
    }
    #[doc = "0x19c - no description available"]
    #[inline(always)]
    pub const fn trgocfgqei_b(&self) -> &TRGOCFG {
        self.trgocfg(39)
    }
    #[doc = "0x1a0 - no description available"]
    #[inline(always)]
    pub const fn trgocfgqei_z(&self) -> &TRGOCFG {
        self.trgocfg(40)
    }
    #[doc = "0x1a4 - no description available"]
    #[inline(always)]
    pub const fn trgocfgqei_h(&self) -> &TRGOCFG {
        self.trgocfg(41)
    }
    #[doc = "0x1a8 - no description available"]
    #[inline(always)]
    pub const fn trgocfgqei_pause(&self) -> &TRGOCFG {
        self.trgocfg(42)
    }
    #[doc = "0x1ac - no description available"]
    #[inline(always)]
    pub const fn trgocfgqei_snapi(&self) -> &TRGOCFG {
        self.trgocfg(43)
    }
    #[doc = "0x1b0 - no description available"]
    #[inline(always)]
    pub const fn trgocfghall_u(&self) -> &TRGOCFG {
        self.trgocfg(44)
    }
    #[doc = "0x1b4 - no description available"]
    #[inline(always)]
    pub const fn trgocfghall_v(&self) -> &TRGOCFG {
        self.trgocfg(45)
    }
    #[doc = "0x1b8 - no description available"]
    #[inline(always)]
    pub const fn trgocfghall_w(&self) -> &TRGOCFG {
        self.trgocfg(46)
    }
    #[doc = "0x1bc - no description available"]
    #[inline(always)]
    pub const fn trgocfghall_snapi(&self) -> &TRGOCFG {
        self.trgocfg(47)
    }
    #[doc = "0x1c0 - no description available"]
    #[inline(always)]
    pub const fn trgocfgadc0_strgi(&self) -> &TRGOCFG {
        self.trgocfg(48)
    }
    #[doc = "0x1c4 - no description available"]
    #[inline(always)]
    pub const fn trgocfgadc1_strgi(&self) -> &TRGOCFG {
        self.trgocfg(49)
    }
    #[doc = "0x1c8 - no description available"]
    #[inline(always)]
    pub const fn trgocfgadc2_strgi(&self) -> &TRGOCFG {
        self.trgocfg(50)
    }
    #[doc = "0x1cc - no description available"]
    #[inline(always)]
    pub const fn trgocfgadcx_ptrgi0a(&self) -> &TRGOCFG {
        self.trgocfg(51)
    }
    #[doc = "0x1d0 - no description available"]
    #[inline(always)]
    pub const fn trgocfgadcx_ptrgi0b(&self) -> &TRGOCFG {
        self.trgocfg(52)
    }
    #[doc = "0x1d4 - no description available"]
    #[inline(always)]
    pub const fn trgocfgadcx_ptrgi0c(&self) -> &TRGOCFG {
        self.trgocfg(53)
    }
    #[doc = "0x1d8 - no description available"]
    #[inline(always)]
    pub const fn trgocfggptmra_synci(&self) -> &TRGOCFG {
        self.trgocfg(54)
    }
    #[doc = "0x1dc - no description available"]
    #[inline(always)]
    pub const fn trgocfggptmra_in2(&self) -> &TRGOCFG {
        self.trgocfg(55)
    }
    #[doc = "0x1e0 - no description available"]
    #[inline(always)]
    pub const fn trgocfggptmra_in3(&self) -> &TRGOCFG {
        self.trgocfg(56)
    }
    #[doc = "0x1e4 - no description available"]
    #[inline(always)]
    pub const fn trgocfgdac_buf_trig(&self) -> &TRGOCFG {
        self.trgocfg(57)
    }
    #[doc = "0x1e8 - no description available"]
    #[inline(always)]
    pub const fn trgocfgdac0_step_trig(&self) -> &TRGOCFG {
        self.trgocfg(58)
    }
    #[doc = "0x1ec - no description available"]
    #[inline(always)]
    pub const fn trgocfgdac1_step_trig(&self) -> &TRGOCFG {
        self.trgocfg(59)
    }
    #[doc = "0x1f0 - no description available"]
    #[inline(always)]
    pub const fn trgocfgcmpx_win(&self) -> &TRGOCFG {
        self.trgocfg(60)
    }
    #[doc = "0x1f4 - no description available"]
    #[inline(always)]
    pub const fn trgocfgcan_ptpc0_cap(&self) -> &TRGOCFG {
        self.trgocfg(61)
    }
    #[doc = "0x1f8 - no description available"]
    #[inline(always)]
    pub const fn trgocfgcan_ptpc1_cap(&self) -> &TRGOCFG {
        self.trgocfg(62)
    }
    #[doc = "0x1fc - no description available"]
    #[inline(always)]
    pub const fn trgocfgsdfm_evt0(&self) -> &TRGOCFG {
        self.trgocfg(63)
    }
    #[doc = "0x200 - no description available"]
    #[inline(always)]
    pub const fn trgocfgsdfm_evt1(&self) -> &TRGOCFG {
        self.trgocfg(64)
    }
    #[doc = "0x204 - no description available"]
    #[inline(always)]
    pub const fn trgocfgsdfm_evt2(&self) -> &TRGOCFG {
        self.trgocfg(65)
    }
    #[doc = "0x208 - no description available"]
    #[inline(always)]
    pub const fn trgocfgsdfm_evt3(&self) -> &TRGOCFG {
        self.trgocfg(66)
    }
    #[doc = "0x300..0x310 - no description available"]
    #[inline(always)]
    pub const fn dmacfg(&self, n: usize) -> &DMACFG {
        &self.dmacfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x300..0x310 - no description available"]
    #[inline(always)]
    pub fn dmacfg_iter(&self) -> impl Iterator<Item = &DMACFG> {
        self.dmacfg.iter()
    }
    #[doc = "0x400 - General Control Register"]
    #[inline(always)]
    pub const fn gcr(&self) -> &GCR {
        &self.gcr
    }
}
#[doc = "FILTCFG (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filtcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filtcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filtcfg`]
module"]
pub type FILTCFG = crate::Reg<filtcfg::FILTCFG_SPEC>;
#[doc = "no description available"]
pub mod filtcfg;
#[doc = "TRGOCFG (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trgocfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trgocfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trgocfg`]
module"]
pub type TRGOCFG = crate::Reg<trgocfg::TRGOCFG_SPEC>;
#[doc = "no description available"]
pub mod trgocfg;
#[doc = "DMACFG (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacfg`]
module"]
pub type DMACFG = crate::Reg<dmacfg::DMACFG_SPEC>;
#[doc = "no description available"]
pub mod dmacfg;
#[doc = "GCR (rw) register accessor: General Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcr`]
module"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "General Control Register"]
pub mod gcr;
