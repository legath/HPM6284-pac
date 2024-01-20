#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    feature: FEATURE,
    priority: [PRIORITY; 127],
    _reserved2: [u8; 0x0e00],
    pending: [PENDING; 4],
    _reserved3: [u8; 0x70],
    trigger: [TRIGGER; 4],
    _reserved4: [u8; 0x70],
    number: NUMBER,
    info: INFO,
    _reserved6: [u8; 0x0ef8],
    targetint: (),
    _reserved7: [u8; 0x001f_e000],
    targetconfig: (),
}
impl RegisterBlock {
    #[doc = "0x00 - Feature enable register"]
    #[inline(always)]
    pub const fn feature(&self) -> &FEATURE {
        &self.feature
    }
    #[doc = "0x04..0x200 - no description available"]
    #[inline(always)]
    pub const fn priority(&self, n: usize) -> &PRIORITY {
        &self.priority[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x200 - no description available"]
    #[inline(always)]
    pub fn priority_iter(&self) -> impl Iterator<Item = &PRIORITY> {
        self.priority.iter()
    }
    #[doc = "0x04 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority1(&self) -> &PRIORITY {
        self.priority(0)
    }
    #[doc = "0x08 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority2(&self) -> &PRIORITY {
        self.priority(1)
    }
    #[doc = "0x0c - no description available"]
    #[inline(always)]
    pub const fn prioritypriority3(&self) -> &PRIORITY {
        self.priority(2)
    }
    #[doc = "0x10 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority4(&self) -> &PRIORITY {
        self.priority(3)
    }
    #[doc = "0x14 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority5(&self) -> &PRIORITY {
        self.priority(4)
    }
    #[doc = "0x18 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority6(&self) -> &PRIORITY {
        self.priority(5)
    }
    #[doc = "0x1c - no description available"]
    #[inline(always)]
    pub const fn prioritypriority7(&self) -> &PRIORITY {
        self.priority(6)
    }
    #[doc = "0x20 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority8(&self) -> &PRIORITY {
        self.priority(7)
    }
    #[doc = "0x24 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority9(&self) -> &PRIORITY {
        self.priority(8)
    }
    #[doc = "0x28 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority10(&self) -> &PRIORITY {
        self.priority(9)
    }
    #[doc = "0x2c - no description available"]
    #[inline(always)]
    pub const fn prioritypriority11(&self) -> &PRIORITY {
        self.priority(10)
    }
    #[doc = "0x30 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority12(&self) -> &PRIORITY {
        self.priority(11)
    }
    #[doc = "0x34 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority13(&self) -> &PRIORITY {
        self.priority(12)
    }
    #[doc = "0x38 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority14(&self) -> &PRIORITY {
        self.priority(13)
    }
    #[doc = "0x3c - no description available"]
    #[inline(always)]
    pub const fn prioritypriority15(&self) -> &PRIORITY {
        self.priority(14)
    }
    #[doc = "0x40 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority16(&self) -> &PRIORITY {
        self.priority(15)
    }
    #[doc = "0x44 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority17(&self) -> &PRIORITY {
        self.priority(16)
    }
    #[doc = "0x48 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority18(&self) -> &PRIORITY {
        self.priority(17)
    }
    #[doc = "0x4c - no description available"]
    #[inline(always)]
    pub const fn prioritypriority19(&self) -> &PRIORITY {
        self.priority(18)
    }
    #[doc = "0x50 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority20(&self) -> &PRIORITY {
        self.priority(19)
    }
    #[doc = "0x54 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority21(&self) -> &PRIORITY {
        self.priority(20)
    }
    #[doc = "0x58 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority22(&self) -> &PRIORITY {
        self.priority(21)
    }
    #[doc = "0x5c - no description available"]
    #[inline(always)]
    pub const fn prioritypriority23(&self) -> &PRIORITY {
        self.priority(22)
    }
    #[doc = "0x60 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority24(&self) -> &PRIORITY {
        self.priority(23)
    }
    #[doc = "0x64 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority25(&self) -> &PRIORITY {
        self.priority(24)
    }
    #[doc = "0x68 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority26(&self) -> &PRIORITY {
        self.priority(25)
    }
    #[doc = "0x6c - no description available"]
    #[inline(always)]
    pub const fn prioritypriority27(&self) -> &PRIORITY {
        self.priority(26)
    }
    #[doc = "0x70 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority28(&self) -> &PRIORITY {
        self.priority(27)
    }
    #[doc = "0x74 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority29(&self) -> &PRIORITY {
        self.priority(28)
    }
    #[doc = "0x78 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority30(&self) -> &PRIORITY {
        self.priority(29)
    }
    #[doc = "0x7c - no description available"]
    #[inline(always)]
    pub const fn prioritypriority31(&self) -> &PRIORITY {
        self.priority(30)
    }
    #[doc = "0x80 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority32(&self) -> &PRIORITY {
        self.priority(31)
    }
    #[doc = "0x84 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority33(&self) -> &PRIORITY {
        self.priority(32)
    }
    #[doc = "0x88 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority34(&self) -> &PRIORITY {
        self.priority(33)
    }
    #[doc = "0x8c - no description available"]
    #[inline(always)]
    pub const fn prioritypriority35(&self) -> &PRIORITY {
        self.priority(34)
    }
    #[doc = "0x90 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority36(&self) -> &PRIORITY {
        self.priority(35)
    }
    #[doc = "0x94 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority37(&self) -> &PRIORITY {
        self.priority(36)
    }
    #[doc = "0x98 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority38(&self) -> &PRIORITY {
        self.priority(37)
    }
    #[doc = "0x9c - no description available"]
    #[inline(always)]
    pub const fn prioritypriority39(&self) -> &PRIORITY {
        self.priority(38)
    }
    #[doc = "0xa0 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority40(&self) -> &PRIORITY {
        self.priority(39)
    }
    #[doc = "0xa4 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority41(&self) -> &PRIORITY {
        self.priority(40)
    }
    #[doc = "0xa8 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority42(&self) -> &PRIORITY {
        self.priority(41)
    }
    #[doc = "0xac - no description available"]
    #[inline(always)]
    pub const fn prioritypriority43(&self) -> &PRIORITY {
        self.priority(42)
    }
    #[doc = "0xb0 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority44(&self) -> &PRIORITY {
        self.priority(43)
    }
    #[doc = "0xb4 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority45(&self) -> &PRIORITY {
        self.priority(44)
    }
    #[doc = "0xb8 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority46(&self) -> &PRIORITY {
        self.priority(45)
    }
    #[doc = "0xbc - no description available"]
    #[inline(always)]
    pub const fn prioritypriority47(&self) -> &PRIORITY {
        self.priority(46)
    }
    #[doc = "0xc0 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority48(&self) -> &PRIORITY {
        self.priority(47)
    }
    #[doc = "0xc4 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority49(&self) -> &PRIORITY {
        self.priority(48)
    }
    #[doc = "0xc8 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority50(&self) -> &PRIORITY {
        self.priority(49)
    }
    #[doc = "0xcc - no description available"]
    #[inline(always)]
    pub const fn prioritypriority51(&self) -> &PRIORITY {
        self.priority(50)
    }
    #[doc = "0xd0 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority52(&self) -> &PRIORITY {
        self.priority(51)
    }
    #[doc = "0xd4 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority53(&self) -> &PRIORITY {
        self.priority(52)
    }
    #[doc = "0xd8 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority54(&self) -> &PRIORITY {
        self.priority(53)
    }
    #[doc = "0xdc - no description available"]
    #[inline(always)]
    pub const fn prioritypriority55(&self) -> &PRIORITY {
        self.priority(54)
    }
    #[doc = "0xe0 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority56(&self) -> &PRIORITY {
        self.priority(55)
    }
    #[doc = "0xe4 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority57(&self) -> &PRIORITY {
        self.priority(56)
    }
    #[doc = "0xe8 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority58(&self) -> &PRIORITY {
        self.priority(57)
    }
    #[doc = "0xec - no description available"]
    #[inline(always)]
    pub const fn prioritypriority59(&self) -> &PRIORITY {
        self.priority(58)
    }
    #[doc = "0xf0 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority60(&self) -> &PRIORITY {
        self.priority(59)
    }
    #[doc = "0xf4 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority61(&self) -> &PRIORITY {
        self.priority(60)
    }
    #[doc = "0xf8 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority62(&self) -> &PRIORITY {
        self.priority(61)
    }
    #[doc = "0xfc - no description available"]
    #[inline(always)]
    pub const fn prioritypriority63(&self) -> &PRIORITY {
        self.priority(62)
    }
    #[doc = "0x100 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority64(&self) -> &PRIORITY {
        self.priority(63)
    }
    #[doc = "0x104 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority65(&self) -> &PRIORITY {
        self.priority(64)
    }
    #[doc = "0x108 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority66(&self) -> &PRIORITY {
        self.priority(65)
    }
    #[doc = "0x10c - no description available"]
    #[inline(always)]
    pub const fn prioritypriority67(&self) -> &PRIORITY {
        self.priority(66)
    }
    #[doc = "0x110 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority68(&self) -> &PRIORITY {
        self.priority(67)
    }
    #[doc = "0x114 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority69(&self) -> &PRIORITY {
        self.priority(68)
    }
    #[doc = "0x118 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority70(&self) -> &PRIORITY {
        self.priority(69)
    }
    #[doc = "0x11c - no description available"]
    #[inline(always)]
    pub const fn prioritypriority71(&self) -> &PRIORITY {
        self.priority(70)
    }
    #[doc = "0x120 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority72(&self) -> &PRIORITY {
        self.priority(71)
    }
    #[doc = "0x124 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority73(&self) -> &PRIORITY {
        self.priority(72)
    }
    #[doc = "0x128 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority74(&self) -> &PRIORITY {
        self.priority(73)
    }
    #[doc = "0x12c - no description available"]
    #[inline(always)]
    pub const fn prioritypriority75(&self) -> &PRIORITY {
        self.priority(74)
    }
    #[doc = "0x130 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority76(&self) -> &PRIORITY {
        self.priority(75)
    }
    #[doc = "0x134 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority77(&self) -> &PRIORITY {
        self.priority(76)
    }
    #[doc = "0x138 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority78(&self) -> &PRIORITY {
        self.priority(77)
    }
    #[doc = "0x13c - no description available"]
    #[inline(always)]
    pub const fn prioritypriority79(&self) -> &PRIORITY {
        self.priority(78)
    }
    #[doc = "0x140 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority80(&self) -> &PRIORITY {
        self.priority(79)
    }
    #[doc = "0x144 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority81(&self) -> &PRIORITY {
        self.priority(80)
    }
    #[doc = "0x148 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority82(&self) -> &PRIORITY {
        self.priority(81)
    }
    #[doc = "0x14c - no description available"]
    #[inline(always)]
    pub const fn prioritypriority83(&self) -> &PRIORITY {
        self.priority(82)
    }
    #[doc = "0x150 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority84(&self) -> &PRIORITY {
        self.priority(83)
    }
    #[doc = "0x154 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority85(&self) -> &PRIORITY {
        self.priority(84)
    }
    #[doc = "0x158 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority86(&self) -> &PRIORITY {
        self.priority(85)
    }
    #[doc = "0x15c - no description available"]
    #[inline(always)]
    pub const fn prioritypriority87(&self) -> &PRIORITY {
        self.priority(86)
    }
    #[doc = "0x160 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority88(&self) -> &PRIORITY {
        self.priority(87)
    }
    #[doc = "0x164 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority89(&self) -> &PRIORITY {
        self.priority(88)
    }
    #[doc = "0x168 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority90(&self) -> &PRIORITY {
        self.priority(89)
    }
    #[doc = "0x16c - no description available"]
    #[inline(always)]
    pub const fn prioritypriority91(&self) -> &PRIORITY {
        self.priority(90)
    }
    #[doc = "0x170 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority92(&self) -> &PRIORITY {
        self.priority(91)
    }
    #[doc = "0x174 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority93(&self) -> &PRIORITY {
        self.priority(92)
    }
    #[doc = "0x178 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority94(&self) -> &PRIORITY {
        self.priority(93)
    }
    #[doc = "0x17c - no description available"]
    #[inline(always)]
    pub const fn prioritypriority95(&self) -> &PRIORITY {
        self.priority(94)
    }
    #[doc = "0x180 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority96(&self) -> &PRIORITY {
        self.priority(95)
    }
    #[doc = "0x184 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority97(&self) -> &PRIORITY {
        self.priority(96)
    }
    #[doc = "0x188 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority98(&self) -> &PRIORITY {
        self.priority(97)
    }
    #[doc = "0x18c - no description available"]
    #[inline(always)]
    pub const fn prioritypriority99(&self) -> &PRIORITY {
        self.priority(98)
    }
    #[doc = "0x190 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority100(&self) -> &PRIORITY {
        self.priority(99)
    }
    #[doc = "0x194 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority101(&self) -> &PRIORITY {
        self.priority(100)
    }
    #[doc = "0x198 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority102(&self) -> &PRIORITY {
        self.priority(101)
    }
    #[doc = "0x19c - no description available"]
    #[inline(always)]
    pub const fn prioritypriority103(&self) -> &PRIORITY {
        self.priority(102)
    }
    #[doc = "0x1a0 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority104(&self) -> &PRIORITY {
        self.priority(103)
    }
    #[doc = "0x1a4 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority105(&self) -> &PRIORITY {
        self.priority(104)
    }
    #[doc = "0x1a8 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority106(&self) -> &PRIORITY {
        self.priority(105)
    }
    #[doc = "0x1ac - no description available"]
    #[inline(always)]
    pub const fn prioritypriority107(&self) -> &PRIORITY {
        self.priority(106)
    }
    #[doc = "0x1b0 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority108(&self) -> &PRIORITY {
        self.priority(107)
    }
    #[doc = "0x1b4 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority109(&self) -> &PRIORITY {
        self.priority(108)
    }
    #[doc = "0x1b8 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority110(&self) -> &PRIORITY {
        self.priority(109)
    }
    #[doc = "0x1bc - no description available"]
    #[inline(always)]
    pub const fn prioritypriority111(&self) -> &PRIORITY {
        self.priority(110)
    }
    #[doc = "0x1c0 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority112(&self) -> &PRIORITY {
        self.priority(111)
    }
    #[doc = "0x1c4 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority113(&self) -> &PRIORITY {
        self.priority(112)
    }
    #[doc = "0x1c8 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority114(&self) -> &PRIORITY {
        self.priority(113)
    }
    #[doc = "0x1cc - no description available"]
    #[inline(always)]
    pub const fn prioritypriority115(&self) -> &PRIORITY {
        self.priority(114)
    }
    #[doc = "0x1d0 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority116(&self) -> &PRIORITY {
        self.priority(115)
    }
    #[doc = "0x1d4 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority117(&self) -> &PRIORITY {
        self.priority(116)
    }
    #[doc = "0x1d8 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority118(&self) -> &PRIORITY {
        self.priority(117)
    }
    #[doc = "0x1dc - no description available"]
    #[inline(always)]
    pub const fn prioritypriority119(&self) -> &PRIORITY {
        self.priority(118)
    }
    #[doc = "0x1e0 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority120(&self) -> &PRIORITY {
        self.priority(119)
    }
    #[doc = "0x1e4 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority121(&self) -> &PRIORITY {
        self.priority(120)
    }
    #[doc = "0x1e8 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority122(&self) -> &PRIORITY {
        self.priority(121)
    }
    #[doc = "0x1ec - no description available"]
    #[inline(always)]
    pub const fn prioritypriority123(&self) -> &PRIORITY {
        self.priority(122)
    }
    #[doc = "0x1f0 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority124(&self) -> &PRIORITY {
        self.priority(123)
    }
    #[doc = "0x1f4 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority125(&self) -> &PRIORITY {
        self.priority(124)
    }
    #[doc = "0x1f8 - no description available"]
    #[inline(always)]
    pub const fn prioritypriority126(&self) -> &PRIORITY {
        self.priority(125)
    }
    #[doc = "0x1fc - no description available"]
    #[inline(always)]
    pub const fn prioritypriority127(&self) -> &PRIORITY {
        self.priority(126)
    }
    #[doc = "0x1000..0x1010 - no description available"]
    #[inline(always)]
    pub const fn pending(&self, n: usize) -> &PENDING {
        &self.pending[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x1010 - no description available"]
    #[inline(always)]
    pub fn pending_iter(&self) -> impl Iterator<Item = &PENDING> {
        self.pending.iter()
    }
    #[doc = "0x1000 - no description available"]
    #[inline(always)]
    pub const fn pendingpending0(&self) -> &PENDING {
        self.pending(0)
    }
    #[doc = "0x1004 - no description available"]
    #[inline(always)]
    pub const fn pendingpending1(&self) -> &PENDING {
        self.pending(1)
    }
    #[doc = "0x1008 - no description available"]
    #[inline(always)]
    pub const fn pendingpending2(&self) -> &PENDING {
        self.pending(2)
    }
    #[doc = "0x100c - no description available"]
    #[inline(always)]
    pub const fn pendingpending3(&self) -> &PENDING {
        self.pending(3)
    }
    #[doc = "0x1080..0x1090 - no description available"]
    #[inline(always)]
    pub const fn trigger(&self, n: usize) -> &TRIGGER {
        &self.trigger[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1080..0x1090 - no description available"]
    #[inline(always)]
    pub fn trigger_iter(&self) -> impl Iterator<Item = &TRIGGER> {
        self.trigger.iter()
    }
    #[doc = "0x1080 - no description available"]
    #[inline(always)]
    pub const fn triggertrigger0(&self) -> &TRIGGER {
        self.trigger(0)
    }
    #[doc = "0x1084 - no description available"]
    #[inline(always)]
    pub const fn triggertrigger1(&self) -> &TRIGGER {
        self.trigger(1)
    }
    #[doc = "0x1088 - no description available"]
    #[inline(always)]
    pub const fn triggertrigger2(&self) -> &TRIGGER {
        self.trigger(2)
    }
    #[doc = "0x108c - no description available"]
    #[inline(always)]
    pub const fn triggertrigger3(&self) -> &TRIGGER {
        self.trigger(3)
    }
    #[doc = "0x1100 - Number of supported interrupt sources and targets"]
    #[inline(always)]
    pub const fn number(&self) -> &NUMBER {
        &self.number
    }
    #[doc = "0x1104 - Version and the maximum priority"]
    #[inline(always)]
    pub const fn info(&self) -> &INFO {
        &self.info
    }
    #[doc = "0x2000..0x2020 - no description available"]
    #[inline(always)]
    pub const fn targetint(&self, n: usize) -> &TARGETINT {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(8192)
                .add(128 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2000..0x2020 - no description available"]
    #[inline(always)]
    pub fn targetint_iter(&self) -> impl Iterator<Item = &TARGETINT> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(8192)
                .add(128 * n)
                .cast()
        })
    }
    #[doc = "0x2000..0x2010 - no description available"]
    #[inline(always)]
    pub const fn targetinttarget0(&self) -> &TARGETINT {
        self.targetint(0)
    }
    #[doc = "0x2080..0x2090 - no description available"]
    #[inline(always)]
    pub const fn targetinttarget1(&self) -> &TARGETINT {
        self.targetint(1)
    }
    #[doc = "0x200000..0x200808 - no description available"]
    #[inline(always)]
    pub const fn targetconfig(&self, n: usize) -> &TARGETCONFIG {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(2097152)
                .add(4096 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200000..0x200808 - no description available"]
    #[inline(always)]
    pub fn targetconfig_iter(&self) -> impl Iterator<Item = &TARGETCONFIG> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(2097152)
                .add(4096 * n)
                .cast()
        })
    }
    #[doc = "0x200000..0x200404 - no description available"]
    #[inline(always)]
    pub const fn targetconfigtarget0(&self) -> &TARGETCONFIG {
        self.targetconfig(0)
    }
    #[doc = "0x201000..0x201404 - no description available"]
    #[inline(always)]
    pub const fn targetconfigtarget1(&self) -> &TARGETCONFIG {
        self.targetconfig(1)
    }
}
#[doc = "feature (rw) register accessor: Feature enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`feature::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`feature::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feature`]
module"]
pub type FEATURE = crate::Reg<feature::FEATURE_SPEC>;
#[doc = "Feature enable register"]
pub mod feature;
#[doc = "PRIORITY (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority`]
module"]
pub type PRIORITY = crate::Reg<priority::PRIORITY_SPEC>;
#[doc = "no description available"]
pub mod priority;
#[doc = "PENDING (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pending::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pending::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pending`]
module"]
pub type PENDING = crate::Reg<pending::PENDING_SPEC>;
#[doc = "no description available"]
pub mod pending;
#[doc = "TRIGGER (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trigger::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigger::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigger`]
module"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "no description available"]
pub mod trigger;
#[doc = "NUMBER (rw) register accessor: Number of supported interrupt sources and targets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`number::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`number::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@number`]
module"]
pub type NUMBER = crate::Reg<number::NUMBER_SPEC>;
#[doc = "Number of supported interrupt sources and targets"]
pub mod number;
#[doc = "INFO (rw) register accessor: Version and the maximum priority\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@info`]
module"]
pub type INFO = crate::Reg<info::INFO_SPEC>;
#[doc = "Version and the maximum priority"]
pub mod info;
#[doc = "no description available"]
pub use self::targetint::TARGETINT;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod targetint;
#[doc = "no description available"]
pub use self::targetconfig::TARGETCONFIG;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod targetconfig;
