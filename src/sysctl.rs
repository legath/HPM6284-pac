#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    resource: [RESOURCE; 166],
    _reserved1: [u8; 0x0568],
    group0: [GROUP0; 4],
    group1: [GROUP1; 4],
    _reserved3: [u8; 0x80],
    affiliate: [AFFILIATE; 2],
    retention: [RETENTION; 2],
    _reserved5: [u8; 0x06c0],
    power: [POWER; 2],
    _reserved6: [u8; 0x03e0],
    reset: [RESET; 3],
    _reserved7: [u8; 0x03d0],
    clock_cpu: [CLOCK_CPU; 1],
    clock: [CLOCK; 39],
    _reserved9: [u8; 0x0360],
    adcclk: [ADCCLK; 3],
    dacclk: [DACCLK; 2],
    _reserved11: [u8; 0x03ec],
    global00: GLOBAL00,
    _reserved12: [u8; 0x03fc],
    monitor: (),
    _reserved13: [u8; 0x0400],
    cpu: (),
}
impl RegisterBlock {
    #[doc = "0x00..0x298 - no description available"]
    #[inline(always)]
    pub const fn resource(&self, n: usize) -> &RESOURCE {
        &self.resource[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x298 - no description available"]
    #[inline(always)]
    pub fn resource_iter(&self) -> impl Iterator<Item = &RESOURCE> {
        self.resource.iter()
    }
    #[doc = "0x00 - no description available"]
    #[inline(always)]
    pub const fn resourcecpu0(&self) -> &RESOURCE {
        self.resource(0)
    }
    #[doc = "0x04 - no description available"]
    #[inline(always)]
    pub const fn resourcecpx0(&self) -> &RESOURCE {
        self.resource(1)
    }
    #[doc = "0x08 - no description available"]
    #[inline(always)]
    pub const fn resourceexe0(&self) -> &RESOURCE {
        self.resource(2)
    }
    #[doc = "0x0c - no description available"]
    #[inline(always)]
    pub const fn resourcewak0(&self) -> &RESOURCE {
        self.resource(3)
    }
    #[doc = "0x10 - no description available"]
    #[inline(always)]
    pub const fn resourcecpu0_per(&self) -> &RESOURCE {
        self.resource(4)
    }
    #[doc = "0x14 - no description available"]
    #[inline(always)]
    pub const fn resourcecpu1(&self) -> &RESOURCE {
        self.resource(5)
    }
    #[doc = "0x18 - no description available"]
    #[inline(always)]
    pub const fn resourcecpx1(&self) -> &RESOURCE {
        self.resource(6)
    }
    #[doc = "0x1c - no description available"]
    #[inline(always)]
    pub const fn resourceexe1(&self) -> &RESOURCE {
        self.resource(7)
    }
    #[doc = "0x20 - no description available"]
    #[inline(always)]
    pub const fn resourcewak1(&self) -> &RESOURCE {
        self.resource(8)
    }
    #[doc = "0x24 - no description available"]
    #[inline(always)]
    pub const fn resourcecpu1_per(&self) -> &RESOURCE {
        self.resource(9)
    }
    #[doc = "0x28 - no description available"]
    #[inline(always)]
    pub const fn resourcelogic0(&self) -> &RESOURCE {
        self.resource(10)
    }
    #[doc = "0x2c - no description available"]
    #[inline(always)]
    pub const fn resourcelogic1(&self) -> &RESOURCE {
        self.resource(11)
    }
    #[doc = "0x30 - no description available"]
    #[inline(always)]
    pub const fn resourcelogic2(&self) -> &RESOURCE {
        self.resource(12)
    }
    #[doc = "0x34 - no description available"]
    #[inline(always)]
    pub const fn resourcelogic3(&self) -> &RESOURCE {
        self.resource(13)
    }
    #[doc = "0x38 - no description available"]
    #[inline(always)]
    pub const fn resourcepmic(&self) -> &RESOURCE {
        self.resource(14)
    }
    #[doc = "0x3c - no description available"]
    #[inline(always)]
    pub const fn resourcepow_cpu0(&self) -> &RESOURCE {
        self.resource(15)
    }
    #[doc = "0x40 - no description available"]
    #[inline(always)]
    pub const fn resourcepow_cpu1(&self) -> &RESOURCE {
        self.resource(16)
    }
    #[doc = "0x44 - no description available"]
    #[inline(always)]
    pub const fn resourcerst_soc(&self) -> &RESOURCE {
        self.resource(17)
    }
    #[doc = "0x48 - no description available"]
    #[inline(always)]
    pub const fn resourcerst_cpu0(&self) -> &RESOURCE {
        self.resource(18)
    }
    #[doc = "0x4c - no description available"]
    #[inline(always)]
    pub const fn resourcerst_cpu1(&self) -> &RESOURCE {
        self.resource(19)
    }
    #[doc = "0x50 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_src_xtal(&self) -> &RESOURCE {
        self.resource(20)
    }
    #[doc = "0x54 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_src_pll0(&self) -> &RESOURCE {
        self.resource(21)
    }
    #[doc = "0x58 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_src_clk0_pll0(&self) -> &RESOURCE {
        self.resource(22)
    }
    #[doc = "0x5c - no description available"]
    #[inline(always)]
    pub const fn resourceclk_src_clk1_pll0(&self) -> &RESOURCE {
        self.resource(23)
    }
    #[doc = "0x60 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_src_clk2_pll0(&self) -> &RESOURCE {
        self.resource(24)
    }
    #[doc = "0x64 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_src_pll1(&self) -> &RESOURCE {
        self.resource(25)
    }
    #[doc = "0x68 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_src_clk0_pll1(&self) -> &RESOURCE {
        self.resource(26)
    }
    #[doc = "0x6c - no description available"]
    #[inline(always)]
    pub const fn resourceclk_src_clk1_pll1(&self) -> &RESOURCE {
        self.resource(27)
    }
    #[doc = "0x70 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_src_pll2(&self) -> &RESOURCE {
        self.resource(28)
    }
    #[doc = "0x74 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_src_clk0_pll2(&self) -> &RESOURCE {
        self.resource(29)
    }
    #[doc = "0x78 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_src_clk1_pll2(&self) -> &RESOURCE {
        self.resource(30)
    }
    #[doc = "0x7c - no description available"]
    #[inline(always)]
    pub const fn resourceclk_src_pll0_ref(&self) -> &RESOURCE {
        self.resource(31)
    }
    #[doc = "0x80 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_src_pll1_ref(&self) -> &RESOURCE {
        self.resource(32)
    }
    #[doc = "0x84 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_src_pll2_ref(&self) -> &RESOURCE {
        self.resource(33)
    }
    #[doc = "0x88 - no description available"]
    #[inline(always)]
    pub const fn resourcembist_soc(&self) -> &RESOURCE {
        self.resource(34)
    }
    #[doc = "0x8c - no description available"]
    #[inline(always)]
    pub const fn resourcembist_cpu0(&self) -> &RESOURCE {
        self.resource(35)
    }
    #[doc = "0x90 - no description available"]
    #[inline(always)]
    pub const fn resourcembist_cpu1(&self) -> &RESOURCE {
        self.resource(36)
    }
    #[doc = "0x94 - no description available"]
    #[inline(always)]
    pub const fn resourcembist_con(&self) -> &RESOURCE {
        self.resource(37)
    }
    #[doc = "0x98 - no description available"]
    #[inline(always)]
    pub const fn resourcedft_start_bus(&self) -> &RESOURCE {
        self.resource(38)
    }
    #[doc = "0x9c - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_cpu0(&self) -> &RESOURCE {
        self.resource(39)
    }
    #[doc = "0xa0 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_mct0(&self) -> &RESOURCE {
        self.resource(40)
    }
    #[doc = "0xa4 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_mct1(&self) -> &RESOURCE {
        self.resource(41)
    }
    #[doc = "0xa8 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_xpi0(&self) -> &RESOURCE {
        self.resource(42)
    }
    #[doc = "0xac - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_tmr0(&self) -> &RESOURCE {
        self.resource(43)
    }
    #[doc = "0xb0 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_tmr1(&self) -> &RESOURCE {
        self.resource(44)
    }
    #[doc = "0xb4 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_tmr2(&self) -> &RESOURCE {
        self.resource(45)
    }
    #[doc = "0xb8 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_tmr3(&self) -> &RESOURCE {
        self.resource(46)
    }
    #[doc = "0xbc - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_urt0(&self) -> &RESOURCE {
        self.resource(47)
    }
    #[doc = "0xc0 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_urt1(&self) -> &RESOURCE {
        self.resource(48)
    }
    #[doc = "0xc4 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_urt2(&self) -> &RESOURCE {
        self.resource(49)
    }
    #[doc = "0xc8 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_urt3(&self) -> &RESOURCE {
        self.resource(50)
    }
    #[doc = "0xcc - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_urt4(&self) -> &RESOURCE {
        self.resource(51)
    }
    #[doc = "0xd0 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_urt5(&self) -> &RESOURCE {
        self.resource(52)
    }
    #[doc = "0xd4 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_urt6(&self) -> &RESOURCE {
        self.resource(53)
    }
    #[doc = "0xd8 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_urt7(&self) -> &RESOURCE {
        self.resource(54)
    }
    #[doc = "0xdc - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_i2c0(&self) -> &RESOURCE {
        self.resource(55)
    }
    #[doc = "0xe0 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_i2c1(&self) -> &RESOURCE {
        self.resource(56)
    }
    #[doc = "0xe4 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_i2c2(&self) -> &RESOURCE {
        self.resource(57)
    }
    #[doc = "0xe8 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_i2c3(&self) -> &RESOURCE {
        self.resource(58)
    }
    #[doc = "0xec - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_spi0(&self) -> &RESOURCE {
        self.resource(59)
    }
    #[doc = "0xf0 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_spi1(&self) -> &RESOURCE {
        self.resource(60)
    }
    #[doc = "0xf4 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_spi2(&self) -> &RESOURCE {
        self.resource(61)
    }
    #[doc = "0xf8 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_spi3(&self) -> &RESOURCE {
        self.resource(62)
    }
    #[doc = "0xfc - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_can0(&self) -> &RESOURCE {
        self.resource(63)
    }
    #[doc = "0x100 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_can1(&self) -> &RESOURCE {
        self.resource(64)
    }
    #[doc = "0x104 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_can2(&self) -> &RESOURCE {
        self.resource(65)
    }
    #[doc = "0x108 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_can3(&self) -> &RESOURCE {
        self.resource(66)
    }
    #[doc = "0x10c - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_ptpc(&self) -> &RESOURCE {
        self.resource(67)
    }
    #[doc = "0x110 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_ana0(&self) -> &RESOURCE {
        self.resource(68)
    }
    #[doc = "0x114 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_ana1(&self) -> &RESOURCE {
        self.resource(69)
    }
    #[doc = "0x118 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_ana2(&self) -> &RESOURCE {
        self.resource(70)
    }
    #[doc = "0x11c - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_ana3(&self) -> &RESOURCE {
        self.resource(71)
    }
    #[doc = "0x120 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_ana4(&self) -> &RESOURCE {
        self.resource(72)
    }
    #[doc = "0x124 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_ref0(&self) -> &RESOURCE {
        self.resource(73)
    }
    #[doc = "0x128 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_ref1(&self) -> &RESOURCE {
        self.resource(74)
    }
    #[doc = "0x12c - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_lin0(&self) -> &RESOURCE {
        self.resource(75)
    }
    #[doc = "0x130 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_lin1(&self) -> &RESOURCE {
        self.resource(76)
    }
    #[doc = "0x134 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_lin2(&self) -> &RESOURCE {
        self.resource(77)
    }
    #[doc = "0x138 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_lin3(&self) -> &RESOURCE {
        self.resource(78)
    }
    #[doc = "0x13c - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_adc0(&self) -> &RESOURCE {
        self.resource(79)
    }
    #[doc = "0x140 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_adc1(&self) -> &RESOURCE {
        self.resource(80)
    }
    #[doc = "0x144 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_adc2(&self) -> &RESOURCE {
        self.resource(81)
    }
    #[doc = "0x148 - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_dac0(&self) -> &RESOURCE {
        self.resource(82)
    }
    #[doc = "0x14c - no description available"]
    #[inline(always)]
    pub const fn resourceclk_top_dac1(&self) -> &RESOURCE {
        self.resource(83)
    }
    #[doc = "0x150 - no description available"]
    #[inline(always)]
    pub const fn resourceusb0_mem(&self) -> &RESOURCE {
        self.resource(84)
    }
    #[doc = "0x154 - no description available"]
    #[inline(always)]
    pub const fn resourcerom0_mem(&self) -> &RESOURCE {
        self.resource(85)
    }
    #[doc = "0x158 - no description available"]
    #[inline(always)]
    pub const fn resourceram0_mem(&self) -> &RESOURCE {
        self.resource(86)
    }
    #[doc = "0x15c - no description available"]
    #[inline(always)]
    pub const fn resourceahbp_mem(&self) -> &RESOURCE {
        self.resource(87)
    }
    #[doc = "0x160 - no description available"]
    #[inline(always)]
    pub const fn resourcexpi0_mem(&self) -> &RESOURCE {
        self.resource(88)
    }
    #[doc = "0x164 - no description available"]
    #[inline(always)]
    pub const fn resourcecan0_mem(&self) -> &RESOURCE {
        self.resource(89)
    }
    #[doc = "0x168 - no description available"]
    #[inline(always)]
    pub const fn resourcecan1_mem(&self) -> &RESOURCE {
        self.resource(90)
    }
    #[doc = "0x16c - no description available"]
    #[inline(always)]
    pub const fn resourcecan2_mem(&self) -> &RESOURCE {
        self.resource(91)
    }
    #[doc = "0x170 - no description available"]
    #[inline(always)]
    pub const fn resourcecan3_mem(&self) -> &RESOURCE {
        self.resource(92)
    }
    #[doc = "0x174 - no description available"]
    #[inline(always)]
    pub const fn resourcesdp0_mem(&self) -> &RESOURCE {
        self.resource(93)
    }
    #[doc = "0x178 - no description available"]
    #[inline(always)]
    pub const fn resourcecpx0_mem(&self) -> &RESOURCE {
        self.resource(94)
    }
    #[doc = "0x17c - no description available"]
    #[inline(always)]
    pub const fn resourcecor0_mem(&self) -> &RESOURCE {
        self.resource(95)
    }
    #[doc = "0x180 - no description available"]
    #[inline(always)]
    pub const fn resourcelmm0_mem(&self) -> &RESOURCE {
        self.resource(96)
    }
    #[doc = "0x184 - no description available"]
    #[inline(always)]
    pub const fn resourcecpx1_mem(&self) -> &RESOURCE {
        self.resource(97)
    }
    #[doc = "0x188 - no description available"]
    #[inline(always)]
    pub const fn resourcecor1_mem(&self) -> &RESOURCE {
        self.resource(98)
    }
    #[doc = "0x18c - no description available"]
    #[inline(always)]
    pub const fn resourcelmm1_mem(&self) -> &RESOURCE {
        self.resource(99)
    }
    #[doc = "0x190 - no description available"]
    #[inline(always)]
    pub const fn resourceahbp(&self) -> &RESOURCE {
        self.resource(100)
    }
    #[doc = "0x194 - no description available"]
    #[inline(always)]
    pub const fn resourceaxis(&self) -> &RESOURCE {
        self.resource(101)
    }
    #[doc = "0x198 - no description available"]
    #[inline(always)]
    pub const fn resourceaxic(&self) -> &RESOURCE {
        self.resource(102)
    }
    #[doc = "0x19c - no description available"]
    #[inline(always)]
    pub const fn resourcelmm0(&self) -> &RESOURCE {
        self.resource(103)
    }
    #[doc = "0x1a0 - no description available"]
    #[inline(always)]
    pub const fn resourcemct0(&self) -> &RESOURCE {
        self.resource(104)
    }
    #[doc = "0x1a4 - no description available"]
    #[inline(always)]
    pub const fn resourcelmm1(&self) -> &RESOURCE {
        self.resource(105)
    }
    #[doc = "0x1a8 - no description available"]
    #[inline(always)]
    pub const fn resourcemct1(&self) -> &RESOURCE {
        self.resource(106)
    }
    #[doc = "0x1ac - no description available"]
    #[inline(always)]
    pub const fn resourcerom0(&self) -> &RESOURCE {
        self.resource(107)
    }
    #[doc = "0x1b0 - no description available"]
    #[inline(always)]
    pub const fn resourceram0(&self) -> &RESOURCE {
        self.resource(108)
    }
    #[doc = "0x1b4 - no description available"]
    #[inline(always)]
    pub const fn resourcei2c0(&self) -> &RESOURCE {
        self.resource(109)
    }
    #[doc = "0x1b8 - no description available"]
    #[inline(always)]
    pub const fn resourcei2c1(&self) -> &RESOURCE {
        self.resource(110)
    }
    #[doc = "0x1bc - no description available"]
    #[inline(always)]
    pub const fn resourcei2c2(&self) -> &RESOURCE {
        self.resource(111)
    }
    #[doc = "0x1c0 - no description available"]
    #[inline(always)]
    pub const fn resourcei2c3(&self) -> &RESOURCE {
        self.resource(112)
    }
    #[doc = "0x1c4 - no description available"]
    #[inline(always)]
    pub const fn resourcetmr0(&self) -> &RESOURCE {
        self.resource(113)
    }
    #[doc = "0x1c8 - no description available"]
    #[inline(always)]
    pub const fn resourcetmr1(&self) -> &RESOURCE {
        self.resource(114)
    }
    #[doc = "0x1cc - no description available"]
    #[inline(always)]
    pub const fn resourcetmr2(&self) -> &RESOURCE {
        self.resource(115)
    }
    #[doc = "0x1d0 - no description available"]
    #[inline(always)]
    pub const fn resourcetmr3(&self) -> &RESOURCE {
        self.resource(116)
    }
    #[doc = "0x1d4 - no description available"]
    #[inline(always)]
    pub const fn resourcegpio(&self) -> &RESOURCE {
        self.resource(117)
    }
    #[doc = "0x1d8 - no description available"]
    #[inline(always)]
    pub const fn resourceadc0(&self) -> &RESOURCE {
        self.resource(118)
    }
    #[doc = "0x1dc - no description available"]
    #[inline(always)]
    pub const fn resourceadc1(&self) -> &RESOURCE {
        self.resource(119)
    }
    #[doc = "0x1e0 - no description available"]
    #[inline(always)]
    pub const fn resourceadc2(&self) -> &RESOURCE {
        self.resource(120)
    }
    #[doc = "0x1e4 - no description available"]
    #[inline(always)]
    pub const fn resourcedac0(&self) -> &RESOURCE {
        self.resource(121)
    }
    #[doc = "0x1e8 - no description available"]
    #[inline(always)]
    pub const fn resourcedac1(&self) -> &RESOURCE {
        self.resource(122)
    }
    #[doc = "0x1ec - no description available"]
    #[inline(always)]
    pub const fn resourceacmp(&self) -> &RESOURCE {
        self.resource(123)
    }
    #[doc = "0x1f0 - no description available"]
    #[inline(always)]
    pub const fn resourcespi0(&self) -> &RESOURCE {
        self.resource(124)
    }
    #[doc = "0x1f4 - no description available"]
    #[inline(always)]
    pub const fn resourcespi1(&self) -> &RESOURCE {
        self.resource(125)
    }
    #[doc = "0x1f8 - no description available"]
    #[inline(always)]
    pub const fn resourcespi2(&self) -> &RESOURCE {
        self.resource(126)
    }
    #[doc = "0x1fc - no description available"]
    #[inline(always)]
    pub const fn resourcespi3(&self) -> &RESOURCE {
        self.resource(127)
    }
    #[doc = "0x200 - no description available"]
    #[inline(always)]
    pub const fn resourcesdm0(&self) -> &RESOURCE {
        self.resource(128)
    }
    #[doc = "0x204 - no description available"]
    #[inline(always)]
    pub const fn resourceurt0(&self) -> &RESOURCE {
        self.resource(129)
    }
    #[doc = "0x208 - no description available"]
    #[inline(always)]
    pub const fn resourceurt1(&self) -> &RESOURCE {
        self.resource(130)
    }
    #[doc = "0x20c - no description available"]
    #[inline(always)]
    pub const fn resourceurt2(&self) -> &RESOURCE {
        self.resource(131)
    }
    #[doc = "0x210 - no description available"]
    #[inline(always)]
    pub const fn resourceurt3(&self) -> &RESOURCE {
        self.resource(132)
    }
    #[doc = "0x214 - no description available"]
    #[inline(always)]
    pub const fn resourceurt4(&self) -> &RESOURCE {
        self.resource(133)
    }
    #[doc = "0x218 - no description available"]
    #[inline(always)]
    pub const fn resourceurt5(&self) -> &RESOURCE {
        self.resource(134)
    }
    #[doc = "0x21c - no description available"]
    #[inline(always)]
    pub const fn resourceurt6(&self) -> &RESOURCE {
        self.resource(135)
    }
    #[doc = "0x220 - no description available"]
    #[inline(always)]
    pub const fn resourceurt7(&self) -> &RESOURCE {
        self.resource(136)
    }
    #[doc = "0x224 - no description available"]
    #[inline(always)]
    pub const fn resourcelin0(&self) -> &RESOURCE {
        self.resource(137)
    }
    #[doc = "0x228 - no description available"]
    #[inline(always)]
    pub const fn resourcelin1(&self) -> &RESOURCE {
        self.resource(138)
    }
    #[doc = "0x22c - no description available"]
    #[inline(always)]
    pub const fn resourcelin2(&self) -> &RESOURCE {
        self.resource(139)
    }
    #[doc = "0x230 - no description available"]
    #[inline(always)]
    pub const fn resourcelin3(&self) -> &RESOURCE {
        self.resource(140)
    }
    #[doc = "0x234 - no description available"]
    #[inline(always)]
    pub const fn resourceptpc(&self) -> &RESOURCE {
        self.resource(141)
    }
    #[doc = "0x238 - no description available"]
    #[inline(always)]
    pub const fn resourcecan0(&self) -> &RESOURCE {
        self.resource(142)
    }
    #[doc = "0x23c - no description available"]
    #[inline(always)]
    pub const fn resourcecan1(&self) -> &RESOURCE {
        self.resource(143)
    }
    #[doc = "0x240 - no description available"]
    #[inline(always)]
    pub const fn resourcecan2(&self) -> &RESOURCE {
        self.resource(144)
    }
    #[doc = "0x244 - no description available"]
    #[inline(always)]
    pub const fn resourcecan3(&self) -> &RESOURCE {
        self.resource(145)
    }
    #[doc = "0x248 - no description available"]
    #[inline(always)]
    pub const fn resourcewdg0(&self) -> &RESOURCE {
        self.resource(146)
    }
    #[doc = "0x24c - no description available"]
    #[inline(always)]
    pub const fn resourcewdg1(&self) -> &RESOURCE {
        self.resource(147)
    }
    #[doc = "0x250 - no description available"]
    #[inline(always)]
    pub const fn resourcembx0(&self) -> &RESOURCE {
        self.resource(148)
    }
    #[doc = "0x254 - no description available"]
    #[inline(always)]
    pub const fn resourcembx1(&self) -> &RESOURCE {
        self.resource(149)
    }
    #[doc = "0x258 - no description available"]
    #[inline(always)]
    pub const fn resourcecrc0(&self) -> &RESOURCE {
        self.resource(150)
    }
    #[doc = "0x25c - no description available"]
    #[inline(always)]
    pub const fn resourcemot0(&self) -> &RESOURCE {
        self.resource(151)
    }
    #[doc = "0x260 - no description available"]
    #[inline(always)]
    pub const fn resourcemot1(&self) -> &RESOURCE {
        self.resource(152)
    }
    #[doc = "0x264 - no description available"]
    #[inline(always)]
    pub const fn resourcemot2(&self) -> &RESOURCE {
        self.resource(153)
    }
    #[doc = "0x268 - no description available"]
    #[inline(always)]
    pub const fn resourcemot3(&self) -> &RESOURCE {
        self.resource(154)
    }
    #[doc = "0x26c - no description available"]
    #[inline(always)]
    pub const fn resourcemsyn(&self) -> &RESOURCE {
        self.resource(155)
    }
    #[doc = "0x270 - no description available"]
    #[inline(always)]
    pub const fn resourcexpi0(&self) -> &RESOURCE {
        self.resource(156)
    }
    #[doc = "0x274 - no description available"]
    #[inline(always)]
    pub const fn resourcehdma(&self) -> &RESOURCE {
        self.resource(157)
    }
    #[doc = "0x278 - no description available"]
    #[inline(always)]
    pub const fn resourcexdma(&self) -> &RESOURCE {
        self.resource(158)
    }
    #[doc = "0x27c - no description available"]
    #[inline(always)]
    pub const fn resourcekman(&self) -> &RESOURCE {
        self.resource(159)
    }
    #[doc = "0x280 - no description available"]
    #[inline(always)]
    pub const fn resourcesdp0(&self) -> &RESOURCE {
        self.resource(160)
    }
    #[doc = "0x284 - no description available"]
    #[inline(always)]
    pub const fn resourcerng0(&self) -> &RESOURCE {
        self.resource(161)
    }
    #[doc = "0x288 - no description available"]
    #[inline(always)]
    pub const fn resourcetsns(&self) -> &RESOURCE {
        self.resource(162)
    }
    #[doc = "0x28c - no description available"]
    #[inline(always)]
    pub const fn resourceusb0(&self) -> &RESOURCE {
        self.resource(163)
    }
    #[doc = "0x290 - no description available"]
    #[inline(always)]
    pub const fn resourceref0(&self) -> &RESOURCE {
        self.resource(164)
    }
    #[doc = "0x294 - no description available"]
    #[inline(always)]
    pub const fn resourceref1(&self) -> &RESOURCE {
        self.resource(165)
    }
    #[doc = "0x800..0x840 - no description available"]
    #[inline(always)]
    pub const fn group0(&self, n: usize) -> &GROUP0 {
        &self.group0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x840 - no description available"]
    #[inline(always)]
    pub fn group0_iter(&self) -> impl Iterator<Item = &GROUP0> {
        self.group0.iter()
    }
    #[doc = "0x800..0x810 - no description available"]
    #[inline(always)]
    pub const fn group0link0(&self) -> &GROUP0 {
        self.group0(0)
    }
    #[doc = "0x810..0x820 - no description available"]
    #[inline(always)]
    pub const fn group0link1(&self) -> &GROUP0 {
        self.group0(1)
    }
    #[doc = "0x820..0x830 - no description available"]
    #[inline(always)]
    pub const fn group0link2(&self) -> &GROUP0 {
        self.group0(2)
    }
    #[doc = "0x830..0x840 - no description available"]
    #[inline(always)]
    pub const fn group0link3(&self) -> &GROUP0 {
        self.group0(3)
    }
    #[doc = "0x840..0x880 - no description available"]
    #[inline(always)]
    pub const fn group1(&self, n: usize) -> &GROUP1 {
        &self.group1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x840..0x880 - no description available"]
    #[inline(always)]
    pub fn group1_iter(&self) -> impl Iterator<Item = &GROUP1> {
        self.group1.iter()
    }
    #[doc = "0x840..0x850 - no description available"]
    #[inline(always)]
    pub const fn group1link0(&self) -> &GROUP1 {
        self.group1(0)
    }
    #[doc = "0x850..0x860 - no description available"]
    #[inline(always)]
    pub const fn group1link1(&self) -> &GROUP1 {
        self.group1(1)
    }
    #[doc = "0x860..0x870 - no description available"]
    #[inline(always)]
    pub const fn group1link2(&self) -> &GROUP1 {
        self.group1(2)
    }
    #[doc = "0x870..0x880 - no description available"]
    #[inline(always)]
    pub const fn group1link3(&self) -> &GROUP1 {
        self.group1(3)
    }
    #[doc = "0x900..0x920 - no description available"]
    #[inline(always)]
    pub const fn affiliate(&self, n: usize) -> &AFFILIATE {
        &self.affiliate[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x900..0x920 - no description available"]
    #[inline(always)]
    pub fn affiliate_iter(&self) -> impl Iterator<Item = &AFFILIATE> {
        self.affiliate.iter()
    }
    #[doc = "0x900..0x910 - no description available"]
    #[inline(always)]
    pub const fn affiliatecpu0(&self) -> &AFFILIATE {
        self.affiliate(0)
    }
    #[doc = "0x910..0x920 - no description available"]
    #[inline(always)]
    pub const fn affiliatecpu1(&self) -> &AFFILIATE {
        self.affiliate(1)
    }
    #[doc = "0x920..0x940 - no description available"]
    #[inline(always)]
    pub const fn retention(&self, n: usize) -> &RETENTION {
        &self.retention[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x920..0x940 - no description available"]
    #[inline(always)]
    pub fn retention_iter(&self) -> impl Iterator<Item = &RETENTION> {
        self.retention.iter()
    }
    #[doc = "0x920..0x930 - no description available"]
    #[inline(always)]
    pub const fn retentioncpu0(&self) -> &RETENTION {
        self.retention(0)
    }
    #[doc = "0x930..0x940 - no description available"]
    #[inline(always)]
    pub const fn retentioncpu1(&self) -> &RETENTION {
        self.retention(1)
    }
    #[doc = "0x1000..0x1020 - no description available"]
    #[inline(always)]
    pub const fn power(&self, n: usize) -> &POWER {
        &self.power[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x1020 - no description available"]
    #[inline(always)]
    pub fn power_iter(&self) -> impl Iterator<Item = &POWER> {
        self.power.iter()
    }
    #[doc = "0x1000..0x1010 - no description available"]
    #[inline(always)]
    pub const fn powercpu0(&self) -> &POWER {
        self.power(0)
    }
    #[doc = "0x1010..0x1020 - no description available"]
    #[inline(always)]
    pub const fn powercpu1(&self) -> &POWER {
        self.power(1)
    }
    #[doc = "0x1400..0x1430 - no description available"]
    #[inline(always)]
    pub const fn reset(&self, n: usize) -> &RESET {
        &self.reset[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1400..0x1430 - no description available"]
    #[inline(always)]
    pub fn reset_iter(&self) -> impl Iterator<Item = &RESET> {
        self.reset.iter()
    }
    #[doc = "0x1400..0x1410 - no description available"]
    #[inline(always)]
    pub const fn resetsoc(&self) -> &RESET {
        self.reset(0)
    }
    #[doc = "0x1410..0x1420 - no description available"]
    #[inline(always)]
    pub const fn resetcpu0(&self) -> &RESET {
        self.reset(1)
    }
    #[doc = "0x1420..0x1430 - no description available"]
    #[inline(always)]
    pub const fn resetcpu1(&self) -> &RESET {
        self.reset(2)
    }
    #[doc = "0x1800 - no description available"]
    #[inline(always)]
    pub const fn clock_cpu(&self, n: usize) -> &CLOCK_CPU {
        &self.clock_cpu[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1800 - no description available"]
    #[inline(always)]
    pub fn clock_cpu_iter(&self) -> impl Iterator<Item = &CLOCK_CPU> {
        self.clock_cpu.iter()
    }
    #[doc = "0x1800 - no description available"]
    #[inline(always)]
    pub const fn clock_cpuclk_top_cpu0(&self) -> &CLOCK_CPU {
        self.clock_cpu(0)
    }
    #[doc = "0x1804..0x18a0 - no description available"]
    #[inline(always)]
    pub const fn clock(&self, n: usize) -> &CLOCK {
        &self.clock[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1804..0x18a0 - no description available"]
    #[inline(always)]
    pub fn clock_iter(&self) -> impl Iterator<Item = &CLOCK> {
        self.clock.iter()
    }
    #[doc = "0x1804 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_mct0(&self) -> &CLOCK {
        self.clock(0)
    }
    #[doc = "0x1808 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_mct1(&self) -> &CLOCK {
        self.clock(1)
    }
    #[doc = "0x180c - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_xpi0(&self) -> &CLOCK {
        self.clock(2)
    }
    #[doc = "0x1810 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_tmr0(&self) -> &CLOCK {
        self.clock(3)
    }
    #[doc = "0x1814 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_tmr1(&self) -> &CLOCK {
        self.clock(4)
    }
    #[doc = "0x1818 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_tmr2(&self) -> &CLOCK {
        self.clock(5)
    }
    #[doc = "0x181c - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_tmr3(&self) -> &CLOCK {
        self.clock(6)
    }
    #[doc = "0x1820 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_urt0(&self) -> &CLOCK {
        self.clock(7)
    }
    #[doc = "0x1824 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_urt1(&self) -> &CLOCK {
        self.clock(8)
    }
    #[doc = "0x1828 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_urt2(&self) -> &CLOCK {
        self.clock(9)
    }
    #[doc = "0x182c - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_urt3(&self) -> &CLOCK {
        self.clock(10)
    }
    #[doc = "0x1830 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_urt4(&self) -> &CLOCK {
        self.clock(11)
    }
    #[doc = "0x1834 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_urt5(&self) -> &CLOCK {
        self.clock(12)
    }
    #[doc = "0x1838 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_urt6(&self) -> &CLOCK {
        self.clock(13)
    }
    #[doc = "0x183c - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_urt7(&self) -> &CLOCK {
        self.clock(14)
    }
    #[doc = "0x1840 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_i2c0(&self) -> &CLOCK {
        self.clock(15)
    }
    #[doc = "0x1844 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_i2c1(&self) -> &CLOCK {
        self.clock(16)
    }
    #[doc = "0x1848 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_i2c2(&self) -> &CLOCK {
        self.clock(17)
    }
    #[doc = "0x184c - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_i2c3(&self) -> &CLOCK {
        self.clock(18)
    }
    #[doc = "0x1850 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_spi0(&self) -> &CLOCK {
        self.clock(19)
    }
    #[doc = "0x1854 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_spi1(&self) -> &CLOCK {
        self.clock(20)
    }
    #[doc = "0x1858 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_spi2(&self) -> &CLOCK {
        self.clock(21)
    }
    #[doc = "0x185c - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_spi3(&self) -> &CLOCK {
        self.clock(22)
    }
    #[doc = "0x1860 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_can0(&self) -> &CLOCK {
        self.clock(23)
    }
    #[doc = "0x1864 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_can1(&self) -> &CLOCK {
        self.clock(24)
    }
    #[doc = "0x1868 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_can2(&self) -> &CLOCK {
        self.clock(25)
    }
    #[doc = "0x186c - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_can3(&self) -> &CLOCK {
        self.clock(26)
    }
    #[doc = "0x1870 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_ptpc(&self) -> &CLOCK {
        self.clock(27)
    }
    #[doc = "0x1874 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_ana0(&self) -> &CLOCK {
        self.clock(28)
    }
    #[doc = "0x1878 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_ana1(&self) -> &CLOCK {
        self.clock(29)
    }
    #[doc = "0x187c - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_ana2(&self) -> &CLOCK {
        self.clock(30)
    }
    #[doc = "0x1880 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_ana3(&self) -> &CLOCK {
        self.clock(31)
    }
    #[doc = "0x1884 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_ana4(&self) -> &CLOCK {
        self.clock(32)
    }
    #[doc = "0x1888 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_ref0(&self) -> &CLOCK {
        self.clock(33)
    }
    #[doc = "0x188c - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_ref1(&self) -> &CLOCK {
        self.clock(34)
    }
    #[doc = "0x1890 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_lin0(&self) -> &CLOCK {
        self.clock(35)
    }
    #[doc = "0x1894 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_lin1(&self) -> &CLOCK {
        self.clock(36)
    }
    #[doc = "0x1898 - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_lin2(&self) -> &CLOCK {
        self.clock(37)
    }
    #[doc = "0x189c - no description available"]
    #[inline(always)]
    pub const fn clockclk_top_lin3(&self) -> &CLOCK {
        self.clock(38)
    }
    #[doc = "0x1c00..0x1c0c - no description available"]
    #[inline(always)]
    pub const fn adcclk(&self, n: usize) -> &ADCCLK {
        &self.adcclk[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c00..0x1c0c - no description available"]
    #[inline(always)]
    pub fn adcclk_iter(&self) -> impl Iterator<Item = &ADCCLK> {
        self.adcclk.iter()
    }
    #[doc = "0x1c00 - no description available"]
    #[inline(always)]
    pub const fn adcclkclk_top_adc0(&self) -> &ADCCLK {
        self.adcclk(0)
    }
    #[doc = "0x1c04 - no description available"]
    #[inline(always)]
    pub const fn adcclkclk_top_adc1(&self) -> &ADCCLK {
        self.adcclk(1)
    }
    #[doc = "0x1c08 - no description available"]
    #[inline(always)]
    pub const fn adcclkclk_top_adc2(&self) -> &ADCCLK {
        self.adcclk(2)
    }
    #[doc = "0x1c0c..0x1c14 - no description available"]
    #[inline(always)]
    pub const fn dacclk(&self, n: usize) -> &DACCLK {
        &self.dacclk[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c0c..0x1c14 - no description available"]
    #[inline(always)]
    pub fn dacclk_iter(&self) -> impl Iterator<Item = &DACCLK> {
        self.dacclk.iter()
    }
    #[doc = "0x1c0c - no description available"]
    #[inline(always)]
    pub const fn dacclkclk_top_dac0(&self) -> &DACCLK {
        self.dacclk(0)
    }
    #[doc = "0x1c10 - no description available"]
    #[inline(always)]
    pub const fn dacclkclk_top_dac1(&self) -> &DACCLK {
        self.dacclk(1)
    }
    #[doc = "0x2000 - Clock senario"]
    #[inline(always)]
    pub const fn global00(&self) -> &GLOBAL00 {
        &self.global00
    }
    #[doc = "0x2400..0x2440 - no description available"]
    #[inline(always)]
    pub const fn monitor(&self, n: usize) -> &MONITOR {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(9216)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2400..0x2440 - no description available"]
    #[inline(always)]
    pub fn monitor_iter(&self) -> impl Iterator<Item = &MONITOR> {
        (0..4).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(9216)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x2400..0x2410 - no description available"]
    #[inline(always)]
    pub const fn monitorslice0(&self) -> &MONITOR {
        self.monitor(0)
    }
    #[doc = "0x2420..0x2430 - no description available"]
    #[inline(always)]
    pub const fn monitorslice1(&self) -> &MONITOR {
        self.monitor(1)
    }
    #[doc = "0x2440..0x2450 - no description available"]
    #[inline(always)]
    pub const fn monitorslice2(&self) -> &MONITOR {
        self.monitor(2)
    }
    #[doc = "0x2460..0x2470 - no description available"]
    #[inline(always)]
    pub const fn monitorslice3(&self) -> &MONITOR {
        self.monitor(3)
    }
    #[doc = "0x2800..0x2920 - no description available"]
    #[inline(always)]
    pub const fn cpu(&self, n: usize) -> &CPU {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(10240)
                .add(1024 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2800..0x2920 - no description available"]
    #[inline(always)]
    pub fn cpu_iter(&self) -> impl Iterator<Item = &CPU> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(10240)
                .add(1024 * n)
                .cast()
        })
    }
    #[doc = "0x2800..0x2890 - no description available"]
    #[inline(always)]
    pub const fn cpucpu0(&self) -> &CPU {
        self.cpu(0)
    }
    #[doc = "0x2c00..0x2c90 - no description available"]
    #[inline(always)]
    pub const fn cpucpu1(&self) -> &CPU {
        self.cpu(1)
    }
}
#[doc = "RESOURCE (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resource::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resource::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resource`]
module"]
pub type RESOURCE = crate::Reg<resource::RESOURCE_SPEC>;
#[doc = "no description available"]
pub mod resource;
#[doc = "no description available"]
pub use self::group0::GROUP0;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod group0;
#[doc = "no description available"]
pub use self::group1::GROUP1;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod group1;
#[doc = "no description available"]
pub use self::affiliate::AFFILIATE;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod affiliate;
#[doc = "no description available"]
pub use self::retention::RETENTION;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod retention;
#[doc = "no description available"]
pub use self::power::POWER;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod power;
#[doc = "no description available"]
pub use self::reset::RESET;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod reset;
#[doc = "CLOCK_CPU (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_cpu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_cpu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_cpu`]
module"]
pub type CLOCK_CPU = crate::Reg<clock_cpu::CLOCK_CPU_SPEC>;
#[doc = "no description available"]
pub mod clock_cpu;
#[doc = "CLOCK (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`]
module"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "no description available"]
pub mod clock;
#[doc = "ADCCLK (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcclk`]
module"]
pub type ADCCLK = crate::Reg<adcclk::ADCCLK_SPEC>;
#[doc = "no description available"]
pub mod adcclk;
#[doc = "DACCLK (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacclk`]
module"]
pub type DACCLK = crate::Reg<dacclk::DACCLK_SPEC>;
#[doc = "no description available"]
pub mod dacclk;
#[doc = "global00 (rw) register accessor: Clock senario\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`global00::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`global00::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@global00`]
module"]
pub type GLOBAL00 = crate::Reg<global00::GLOBAL00_SPEC>;
#[doc = "Clock senario"]
pub mod global00;
#[doc = "no description available"]
pub use self::monitor::MONITOR;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod monitor;
#[doc = "no description available"]
pub use self::cpu::CPU;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod cpu;
