#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    shadow: [SHADOW; 128],
    shadow_lock: [SHADOW_LOCK; 8],
    _reserved2: [u8; 0x01e0],
    fuse: [FUSE; 128],
    fuse_lock: [FUSE_LOCK; 8],
    _reserved4: [u8; 0x01e0],
    unlock: UNLOCK,
    data: DATA,
    addr: ADDR,
    cmd: CMD,
    _reserved8: [u8; 0x01f0],
    load_req: LOAD_REQ,
    load_comp: LOAD_COMP,
    _reserved10: [u8; 0x18],
    region: [REGION; 4],
    _reserved11: [u8; 0x01d0],
    int_flag: INT_FLAG,
    int_en: INT_EN,
}
impl RegisterBlock {
    #[doc = "0x00..0x200 - no description available"]
    #[inline(always)]
    pub const fn shadow(&self, n: usize) -> &SHADOW {
        &self.shadow[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x200 - no description available"]
    #[inline(always)]
    pub fn shadow_iter(&self) -> impl Iterator<Item = &SHADOW> {
        self.shadow.iter()
    }
    #[doc = "0x00 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow000(&self) -> &SHADOW {
        self.shadow(0)
    }
    #[doc = "0x04 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow001(&self) -> &SHADOW {
        self.shadow(1)
    }
    #[doc = "0x08 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow002(&self) -> &SHADOW {
        self.shadow(2)
    }
    #[doc = "0x0c - no description available"]
    #[inline(always)]
    pub const fn shadowshadow003(&self) -> &SHADOW {
        self.shadow(3)
    }
    #[doc = "0x10 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow004(&self) -> &SHADOW {
        self.shadow(4)
    }
    #[doc = "0x14 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow005(&self) -> &SHADOW {
        self.shadow(5)
    }
    #[doc = "0x18 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow006(&self) -> &SHADOW {
        self.shadow(6)
    }
    #[doc = "0x1c - no description available"]
    #[inline(always)]
    pub const fn shadowshadow007(&self) -> &SHADOW {
        self.shadow(7)
    }
    #[doc = "0x20 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow008(&self) -> &SHADOW {
        self.shadow(8)
    }
    #[doc = "0x24 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow009(&self) -> &SHADOW {
        self.shadow(9)
    }
    #[doc = "0x28 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow010(&self) -> &SHADOW {
        self.shadow(10)
    }
    #[doc = "0x2c - no description available"]
    #[inline(always)]
    pub const fn shadowshadow011(&self) -> &SHADOW {
        self.shadow(11)
    }
    #[doc = "0x30 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow012(&self) -> &SHADOW {
        self.shadow(12)
    }
    #[doc = "0x34 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow013(&self) -> &SHADOW {
        self.shadow(13)
    }
    #[doc = "0x38 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow014(&self) -> &SHADOW {
        self.shadow(14)
    }
    #[doc = "0x3c - no description available"]
    #[inline(always)]
    pub const fn shadowshadow015(&self) -> &SHADOW {
        self.shadow(15)
    }
    #[doc = "0x40 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow016(&self) -> &SHADOW {
        self.shadow(16)
    }
    #[doc = "0x44 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow017(&self) -> &SHADOW {
        self.shadow(17)
    }
    #[doc = "0x48 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow018(&self) -> &SHADOW {
        self.shadow(18)
    }
    #[doc = "0x4c - no description available"]
    #[inline(always)]
    pub const fn shadowshadow019(&self) -> &SHADOW {
        self.shadow(19)
    }
    #[doc = "0x50 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow020(&self) -> &SHADOW {
        self.shadow(20)
    }
    #[doc = "0x54 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow021(&self) -> &SHADOW {
        self.shadow(21)
    }
    #[doc = "0x58 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow022(&self) -> &SHADOW {
        self.shadow(22)
    }
    #[doc = "0x5c - no description available"]
    #[inline(always)]
    pub const fn shadowshadow023(&self) -> &SHADOW {
        self.shadow(23)
    }
    #[doc = "0x60 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow024(&self) -> &SHADOW {
        self.shadow(24)
    }
    #[doc = "0x64 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow025(&self) -> &SHADOW {
        self.shadow(25)
    }
    #[doc = "0x68 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow026(&self) -> &SHADOW {
        self.shadow(26)
    }
    #[doc = "0x6c - no description available"]
    #[inline(always)]
    pub const fn shadowshadow027(&self) -> &SHADOW {
        self.shadow(27)
    }
    #[doc = "0x70 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow028(&self) -> &SHADOW {
        self.shadow(28)
    }
    #[doc = "0x74 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow029(&self) -> &SHADOW {
        self.shadow(29)
    }
    #[doc = "0x78 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow030(&self) -> &SHADOW {
        self.shadow(30)
    }
    #[doc = "0x7c - no description available"]
    #[inline(always)]
    pub const fn shadowshadow031(&self) -> &SHADOW {
        self.shadow(31)
    }
    #[doc = "0x80 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow032(&self) -> &SHADOW {
        self.shadow(32)
    }
    #[doc = "0x84 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow033(&self) -> &SHADOW {
        self.shadow(33)
    }
    #[doc = "0x88 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow034(&self) -> &SHADOW {
        self.shadow(34)
    }
    #[doc = "0x8c - no description available"]
    #[inline(always)]
    pub const fn shadowshadow035(&self) -> &SHADOW {
        self.shadow(35)
    }
    #[doc = "0x90 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow036(&self) -> &SHADOW {
        self.shadow(36)
    }
    #[doc = "0x94 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow037(&self) -> &SHADOW {
        self.shadow(37)
    }
    #[doc = "0x98 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow038(&self) -> &SHADOW {
        self.shadow(38)
    }
    #[doc = "0x9c - no description available"]
    #[inline(always)]
    pub const fn shadowshadow039(&self) -> &SHADOW {
        self.shadow(39)
    }
    #[doc = "0xa0 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow040(&self) -> &SHADOW {
        self.shadow(40)
    }
    #[doc = "0xa4 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow041(&self) -> &SHADOW {
        self.shadow(41)
    }
    #[doc = "0xa8 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow042(&self) -> &SHADOW {
        self.shadow(42)
    }
    #[doc = "0xac - no description available"]
    #[inline(always)]
    pub const fn shadowshadow043(&self) -> &SHADOW {
        self.shadow(43)
    }
    #[doc = "0xb0 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow044(&self) -> &SHADOW {
        self.shadow(44)
    }
    #[doc = "0xb4 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow045(&self) -> &SHADOW {
        self.shadow(45)
    }
    #[doc = "0xb8 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow046(&self) -> &SHADOW {
        self.shadow(46)
    }
    #[doc = "0xbc - no description available"]
    #[inline(always)]
    pub const fn shadowshadow047(&self) -> &SHADOW {
        self.shadow(47)
    }
    #[doc = "0xc0 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow048(&self) -> &SHADOW {
        self.shadow(48)
    }
    #[doc = "0xc4 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow049(&self) -> &SHADOW {
        self.shadow(49)
    }
    #[doc = "0xc8 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow050(&self) -> &SHADOW {
        self.shadow(50)
    }
    #[doc = "0xcc - no description available"]
    #[inline(always)]
    pub const fn shadowshadow051(&self) -> &SHADOW {
        self.shadow(51)
    }
    #[doc = "0xd0 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow052(&self) -> &SHADOW {
        self.shadow(52)
    }
    #[doc = "0xd4 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow053(&self) -> &SHADOW {
        self.shadow(53)
    }
    #[doc = "0xd8 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow054(&self) -> &SHADOW {
        self.shadow(54)
    }
    #[doc = "0xdc - no description available"]
    #[inline(always)]
    pub const fn shadowshadow055(&self) -> &SHADOW {
        self.shadow(55)
    }
    #[doc = "0xe0 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow056(&self) -> &SHADOW {
        self.shadow(56)
    }
    #[doc = "0xe4 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow057(&self) -> &SHADOW {
        self.shadow(57)
    }
    #[doc = "0xe8 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow058(&self) -> &SHADOW {
        self.shadow(58)
    }
    #[doc = "0xec - no description available"]
    #[inline(always)]
    pub const fn shadowshadow059(&self) -> &SHADOW {
        self.shadow(59)
    }
    #[doc = "0xf0 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow060(&self) -> &SHADOW {
        self.shadow(60)
    }
    #[doc = "0xf4 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow061(&self) -> &SHADOW {
        self.shadow(61)
    }
    #[doc = "0xf8 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow062(&self) -> &SHADOW {
        self.shadow(62)
    }
    #[doc = "0xfc - no description available"]
    #[inline(always)]
    pub const fn shadowshadow063(&self) -> &SHADOW {
        self.shadow(63)
    }
    #[doc = "0x100 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow064(&self) -> &SHADOW {
        self.shadow(64)
    }
    #[doc = "0x104 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow065(&self) -> &SHADOW {
        self.shadow(65)
    }
    #[doc = "0x108 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow066(&self) -> &SHADOW {
        self.shadow(66)
    }
    #[doc = "0x10c - no description available"]
    #[inline(always)]
    pub const fn shadowshadow067(&self) -> &SHADOW {
        self.shadow(67)
    }
    #[doc = "0x110 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow068(&self) -> &SHADOW {
        self.shadow(68)
    }
    #[doc = "0x114 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow069(&self) -> &SHADOW {
        self.shadow(69)
    }
    #[doc = "0x118 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow070(&self) -> &SHADOW {
        self.shadow(70)
    }
    #[doc = "0x11c - no description available"]
    #[inline(always)]
    pub const fn shadowshadow071(&self) -> &SHADOW {
        self.shadow(71)
    }
    #[doc = "0x120 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow072(&self) -> &SHADOW {
        self.shadow(72)
    }
    #[doc = "0x124 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow073(&self) -> &SHADOW {
        self.shadow(73)
    }
    #[doc = "0x128 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow074(&self) -> &SHADOW {
        self.shadow(74)
    }
    #[doc = "0x12c - no description available"]
    #[inline(always)]
    pub const fn shadowshadow075(&self) -> &SHADOW {
        self.shadow(75)
    }
    #[doc = "0x130 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow076(&self) -> &SHADOW {
        self.shadow(76)
    }
    #[doc = "0x134 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow077(&self) -> &SHADOW {
        self.shadow(77)
    }
    #[doc = "0x138 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow078(&self) -> &SHADOW {
        self.shadow(78)
    }
    #[doc = "0x13c - no description available"]
    #[inline(always)]
    pub const fn shadowshadow079(&self) -> &SHADOW {
        self.shadow(79)
    }
    #[doc = "0x140 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow080(&self) -> &SHADOW {
        self.shadow(80)
    }
    #[doc = "0x144 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow081(&self) -> &SHADOW {
        self.shadow(81)
    }
    #[doc = "0x148 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow082(&self) -> &SHADOW {
        self.shadow(82)
    }
    #[doc = "0x14c - no description available"]
    #[inline(always)]
    pub const fn shadowshadow083(&self) -> &SHADOW {
        self.shadow(83)
    }
    #[doc = "0x150 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow084(&self) -> &SHADOW {
        self.shadow(84)
    }
    #[doc = "0x154 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow085(&self) -> &SHADOW {
        self.shadow(85)
    }
    #[doc = "0x158 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow086(&self) -> &SHADOW {
        self.shadow(86)
    }
    #[doc = "0x15c - no description available"]
    #[inline(always)]
    pub const fn shadowshadow087(&self) -> &SHADOW {
        self.shadow(87)
    }
    #[doc = "0x160 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow088(&self) -> &SHADOW {
        self.shadow(88)
    }
    #[doc = "0x164 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow089(&self) -> &SHADOW {
        self.shadow(89)
    }
    #[doc = "0x168 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow090(&self) -> &SHADOW {
        self.shadow(90)
    }
    #[doc = "0x16c - no description available"]
    #[inline(always)]
    pub const fn shadowshadow091(&self) -> &SHADOW {
        self.shadow(91)
    }
    #[doc = "0x170 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow092(&self) -> &SHADOW {
        self.shadow(92)
    }
    #[doc = "0x174 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow093(&self) -> &SHADOW {
        self.shadow(93)
    }
    #[doc = "0x178 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow094(&self) -> &SHADOW {
        self.shadow(94)
    }
    #[doc = "0x17c - no description available"]
    #[inline(always)]
    pub const fn shadowshadow095(&self) -> &SHADOW {
        self.shadow(95)
    }
    #[doc = "0x180 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow096(&self) -> &SHADOW {
        self.shadow(96)
    }
    #[doc = "0x184 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow097(&self) -> &SHADOW {
        self.shadow(97)
    }
    #[doc = "0x188 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow098(&self) -> &SHADOW {
        self.shadow(98)
    }
    #[doc = "0x18c - no description available"]
    #[inline(always)]
    pub const fn shadowshadow099(&self) -> &SHADOW {
        self.shadow(99)
    }
    #[doc = "0x190 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow100(&self) -> &SHADOW {
        self.shadow(100)
    }
    #[doc = "0x194 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow101(&self) -> &SHADOW {
        self.shadow(101)
    }
    #[doc = "0x198 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow102(&self) -> &SHADOW {
        self.shadow(102)
    }
    #[doc = "0x19c - no description available"]
    #[inline(always)]
    pub const fn shadowshadow103(&self) -> &SHADOW {
        self.shadow(103)
    }
    #[doc = "0x1a0 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow104(&self) -> &SHADOW {
        self.shadow(104)
    }
    #[doc = "0x1a4 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow105(&self) -> &SHADOW {
        self.shadow(105)
    }
    #[doc = "0x1a8 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow106(&self) -> &SHADOW {
        self.shadow(106)
    }
    #[doc = "0x1ac - no description available"]
    #[inline(always)]
    pub const fn shadowshadow107(&self) -> &SHADOW {
        self.shadow(107)
    }
    #[doc = "0x1b0 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow108(&self) -> &SHADOW {
        self.shadow(108)
    }
    #[doc = "0x1b4 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow109(&self) -> &SHADOW {
        self.shadow(109)
    }
    #[doc = "0x1b8 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow110(&self) -> &SHADOW {
        self.shadow(110)
    }
    #[doc = "0x1bc - no description available"]
    #[inline(always)]
    pub const fn shadowshadow111(&self) -> &SHADOW {
        self.shadow(111)
    }
    #[doc = "0x1c0 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow112(&self) -> &SHADOW {
        self.shadow(112)
    }
    #[doc = "0x1c4 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow113(&self) -> &SHADOW {
        self.shadow(113)
    }
    #[doc = "0x1c8 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow114(&self) -> &SHADOW {
        self.shadow(114)
    }
    #[doc = "0x1cc - no description available"]
    #[inline(always)]
    pub const fn shadowshadow115(&self) -> &SHADOW {
        self.shadow(115)
    }
    #[doc = "0x1d0 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow116(&self) -> &SHADOW {
        self.shadow(116)
    }
    #[doc = "0x1d4 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow117(&self) -> &SHADOW {
        self.shadow(117)
    }
    #[doc = "0x1d8 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow118(&self) -> &SHADOW {
        self.shadow(118)
    }
    #[doc = "0x1dc - no description available"]
    #[inline(always)]
    pub const fn shadowshadow119(&self) -> &SHADOW {
        self.shadow(119)
    }
    #[doc = "0x1e0 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow120(&self) -> &SHADOW {
        self.shadow(120)
    }
    #[doc = "0x1e4 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow121(&self) -> &SHADOW {
        self.shadow(121)
    }
    #[doc = "0x1e8 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow122(&self) -> &SHADOW {
        self.shadow(122)
    }
    #[doc = "0x1ec - no description available"]
    #[inline(always)]
    pub const fn shadowshadow123(&self) -> &SHADOW {
        self.shadow(123)
    }
    #[doc = "0x1f0 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow124(&self) -> &SHADOW {
        self.shadow(124)
    }
    #[doc = "0x1f4 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow125(&self) -> &SHADOW {
        self.shadow(125)
    }
    #[doc = "0x1f8 - no description available"]
    #[inline(always)]
    pub const fn shadowshadow126(&self) -> &SHADOW {
        self.shadow(126)
    }
    #[doc = "0x1fc - no description available"]
    #[inline(always)]
    pub const fn shadowshadow127(&self) -> &SHADOW {
        self.shadow(127)
    }
    #[doc = "0x200..0x220 - no description available"]
    #[inline(always)]
    pub const fn shadow_lock(&self, n: usize) -> &SHADOW_LOCK {
        &self.shadow_lock[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x220 - no description available"]
    #[inline(always)]
    pub fn shadow_lock_iter(&self) -> impl Iterator<Item = &SHADOW_LOCK> {
        self.shadow_lock.iter()
    }
    #[doc = "0x200 - no description available"]
    #[inline(always)]
    pub const fn shadow_locklock00(&self) -> &SHADOW_LOCK {
        self.shadow_lock(0)
    }
    #[doc = "0x204 - no description available"]
    #[inline(always)]
    pub const fn shadow_locklock01(&self) -> &SHADOW_LOCK {
        self.shadow_lock(1)
    }
    #[doc = "0x208 - no description available"]
    #[inline(always)]
    pub const fn shadow_locklock02(&self) -> &SHADOW_LOCK {
        self.shadow_lock(2)
    }
    #[doc = "0x20c - no description available"]
    #[inline(always)]
    pub const fn shadow_locklock03(&self) -> &SHADOW_LOCK {
        self.shadow_lock(3)
    }
    #[doc = "0x210 - no description available"]
    #[inline(always)]
    pub const fn shadow_locklock04(&self) -> &SHADOW_LOCK {
        self.shadow_lock(4)
    }
    #[doc = "0x214 - no description available"]
    #[inline(always)]
    pub const fn shadow_locklock05(&self) -> &SHADOW_LOCK {
        self.shadow_lock(5)
    }
    #[doc = "0x218 - no description available"]
    #[inline(always)]
    pub const fn shadow_locklock06(&self) -> &SHADOW_LOCK {
        self.shadow_lock(6)
    }
    #[doc = "0x21c - no description available"]
    #[inline(always)]
    pub const fn shadow_locklock07(&self) -> &SHADOW_LOCK {
        self.shadow_lock(7)
    }
    #[doc = "0x400..0x600 - no description available"]
    #[inline(always)]
    pub const fn fuse(&self, n: usize) -> &FUSE {
        &self.fuse[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x600 - no description available"]
    #[inline(always)]
    pub fn fuse_iter(&self) -> impl Iterator<Item = &FUSE> {
        self.fuse.iter()
    }
    #[doc = "0x400 - no description available"]
    #[inline(always)]
    pub const fn fusefuse000(&self) -> &FUSE {
        self.fuse(0)
    }
    #[doc = "0x404 - no description available"]
    #[inline(always)]
    pub const fn fusefuse001(&self) -> &FUSE {
        self.fuse(1)
    }
    #[doc = "0x408 - no description available"]
    #[inline(always)]
    pub const fn fusefuse002(&self) -> &FUSE {
        self.fuse(2)
    }
    #[doc = "0x40c - no description available"]
    #[inline(always)]
    pub const fn fusefuse003(&self) -> &FUSE {
        self.fuse(3)
    }
    #[doc = "0x410 - no description available"]
    #[inline(always)]
    pub const fn fusefuse004(&self) -> &FUSE {
        self.fuse(4)
    }
    #[doc = "0x414 - no description available"]
    #[inline(always)]
    pub const fn fusefuse005(&self) -> &FUSE {
        self.fuse(5)
    }
    #[doc = "0x418 - no description available"]
    #[inline(always)]
    pub const fn fusefuse006(&self) -> &FUSE {
        self.fuse(6)
    }
    #[doc = "0x41c - no description available"]
    #[inline(always)]
    pub const fn fusefuse007(&self) -> &FUSE {
        self.fuse(7)
    }
    #[doc = "0x420 - no description available"]
    #[inline(always)]
    pub const fn fusefuse008(&self) -> &FUSE {
        self.fuse(8)
    }
    #[doc = "0x424 - no description available"]
    #[inline(always)]
    pub const fn fusefuse009(&self) -> &FUSE {
        self.fuse(9)
    }
    #[doc = "0x428 - no description available"]
    #[inline(always)]
    pub const fn fusefuse010(&self) -> &FUSE {
        self.fuse(10)
    }
    #[doc = "0x42c - no description available"]
    #[inline(always)]
    pub const fn fusefuse011(&self) -> &FUSE {
        self.fuse(11)
    }
    #[doc = "0x430 - no description available"]
    #[inline(always)]
    pub const fn fusefuse012(&self) -> &FUSE {
        self.fuse(12)
    }
    #[doc = "0x434 - no description available"]
    #[inline(always)]
    pub const fn fusefuse013(&self) -> &FUSE {
        self.fuse(13)
    }
    #[doc = "0x438 - no description available"]
    #[inline(always)]
    pub const fn fusefuse014(&self) -> &FUSE {
        self.fuse(14)
    }
    #[doc = "0x43c - no description available"]
    #[inline(always)]
    pub const fn fusefuse015(&self) -> &FUSE {
        self.fuse(15)
    }
    #[doc = "0x440 - no description available"]
    #[inline(always)]
    pub const fn fusefuse016(&self) -> &FUSE {
        self.fuse(16)
    }
    #[doc = "0x444 - no description available"]
    #[inline(always)]
    pub const fn fusefuse017(&self) -> &FUSE {
        self.fuse(17)
    }
    #[doc = "0x448 - no description available"]
    #[inline(always)]
    pub const fn fusefuse018(&self) -> &FUSE {
        self.fuse(18)
    }
    #[doc = "0x44c - no description available"]
    #[inline(always)]
    pub const fn fusefuse019(&self) -> &FUSE {
        self.fuse(19)
    }
    #[doc = "0x450 - no description available"]
    #[inline(always)]
    pub const fn fusefuse020(&self) -> &FUSE {
        self.fuse(20)
    }
    #[doc = "0x454 - no description available"]
    #[inline(always)]
    pub const fn fusefuse021(&self) -> &FUSE {
        self.fuse(21)
    }
    #[doc = "0x458 - no description available"]
    #[inline(always)]
    pub const fn fusefuse022(&self) -> &FUSE {
        self.fuse(22)
    }
    #[doc = "0x45c - no description available"]
    #[inline(always)]
    pub const fn fusefuse023(&self) -> &FUSE {
        self.fuse(23)
    }
    #[doc = "0x460 - no description available"]
    #[inline(always)]
    pub const fn fusefuse024(&self) -> &FUSE {
        self.fuse(24)
    }
    #[doc = "0x464 - no description available"]
    #[inline(always)]
    pub const fn fusefuse025(&self) -> &FUSE {
        self.fuse(25)
    }
    #[doc = "0x468 - no description available"]
    #[inline(always)]
    pub const fn fusefuse026(&self) -> &FUSE {
        self.fuse(26)
    }
    #[doc = "0x46c - no description available"]
    #[inline(always)]
    pub const fn fusefuse027(&self) -> &FUSE {
        self.fuse(27)
    }
    #[doc = "0x470 - no description available"]
    #[inline(always)]
    pub const fn fusefuse028(&self) -> &FUSE {
        self.fuse(28)
    }
    #[doc = "0x474 - no description available"]
    #[inline(always)]
    pub const fn fusefuse029(&self) -> &FUSE {
        self.fuse(29)
    }
    #[doc = "0x478 - no description available"]
    #[inline(always)]
    pub const fn fusefuse030(&self) -> &FUSE {
        self.fuse(30)
    }
    #[doc = "0x47c - no description available"]
    #[inline(always)]
    pub const fn fusefuse031(&self) -> &FUSE {
        self.fuse(31)
    }
    #[doc = "0x480 - no description available"]
    #[inline(always)]
    pub const fn fusefuse032(&self) -> &FUSE {
        self.fuse(32)
    }
    #[doc = "0x484 - no description available"]
    #[inline(always)]
    pub const fn fusefuse033(&self) -> &FUSE {
        self.fuse(33)
    }
    #[doc = "0x488 - no description available"]
    #[inline(always)]
    pub const fn fusefuse034(&self) -> &FUSE {
        self.fuse(34)
    }
    #[doc = "0x48c - no description available"]
    #[inline(always)]
    pub const fn fusefuse035(&self) -> &FUSE {
        self.fuse(35)
    }
    #[doc = "0x490 - no description available"]
    #[inline(always)]
    pub const fn fusefuse036(&self) -> &FUSE {
        self.fuse(36)
    }
    #[doc = "0x494 - no description available"]
    #[inline(always)]
    pub const fn fusefuse037(&self) -> &FUSE {
        self.fuse(37)
    }
    #[doc = "0x498 - no description available"]
    #[inline(always)]
    pub const fn fusefuse038(&self) -> &FUSE {
        self.fuse(38)
    }
    #[doc = "0x49c - no description available"]
    #[inline(always)]
    pub const fn fusefuse039(&self) -> &FUSE {
        self.fuse(39)
    }
    #[doc = "0x4a0 - no description available"]
    #[inline(always)]
    pub const fn fusefuse040(&self) -> &FUSE {
        self.fuse(40)
    }
    #[doc = "0x4a4 - no description available"]
    #[inline(always)]
    pub const fn fusefuse041(&self) -> &FUSE {
        self.fuse(41)
    }
    #[doc = "0x4a8 - no description available"]
    #[inline(always)]
    pub const fn fusefuse042(&self) -> &FUSE {
        self.fuse(42)
    }
    #[doc = "0x4ac - no description available"]
    #[inline(always)]
    pub const fn fusefuse043(&self) -> &FUSE {
        self.fuse(43)
    }
    #[doc = "0x4b0 - no description available"]
    #[inline(always)]
    pub const fn fusefuse044(&self) -> &FUSE {
        self.fuse(44)
    }
    #[doc = "0x4b4 - no description available"]
    #[inline(always)]
    pub const fn fusefuse045(&self) -> &FUSE {
        self.fuse(45)
    }
    #[doc = "0x4b8 - no description available"]
    #[inline(always)]
    pub const fn fusefuse046(&self) -> &FUSE {
        self.fuse(46)
    }
    #[doc = "0x4bc - no description available"]
    #[inline(always)]
    pub const fn fusefuse047(&self) -> &FUSE {
        self.fuse(47)
    }
    #[doc = "0x4c0 - no description available"]
    #[inline(always)]
    pub const fn fusefuse048(&self) -> &FUSE {
        self.fuse(48)
    }
    #[doc = "0x4c4 - no description available"]
    #[inline(always)]
    pub const fn fusefuse049(&self) -> &FUSE {
        self.fuse(49)
    }
    #[doc = "0x4c8 - no description available"]
    #[inline(always)]
    pub const fn fusefuse050(&self) -> &FUSE {
        self.fuse(50)
    }
    #[doc = "0x4cc - no description available"]
    #[inline(always)]
    pub const fn fusefuse051(&self) -> &FUSE {
        self.fuse(51)
    }
    #[doc = "0x4d0 - no description available"]
    #[inline(always)]
    pub const fn fusefuse052(&self) -> &FUSE {
        self.fuse(52)
    }
    #[doc = "0x4d4 - no description available"]
    #[inline(always)]
    pub const fn fusefuse053(&self) -> &FUSE {
        self.fuse(53)
    }
    #[doc = "0x4d8 - no description available"]
    #[inline(always)]
    pub const fn fusefuse054(&self) -> &FUSE {
        self.fuse(54)
    }
    #[doc = "0x4dc - no description available"]
    #[inline(always)]
    pub const fn fusefuse055(&self) -> &FUSE {
        self.fuse(55)
    }
    #[doc = "0x4e0 - no description available"]
    #[inline(always)]
    pub const fn fusefuse056(&self) -> &FUSE {
        self.fuse(56)
    }
    #[doc = "0x4e4 - no description available"]
    #[inline(always)]
    pub const fn fusefuse057(&self) -> &FUSE {
        self.fuse(57)
    }
    #[doc = "0x4e8 - no description available"]
    #[inline(always)]
    pub const fn fusefuse058(&self) -> &FUSE {
        self.fuse(58)
    }
    #[doc = "0x4ec - no description available"]
    #[inline(always)]
    pub const fn fusefuse059(&self) -> &FUSE {
        self.fuse(59)
    }
    #[doc = "0x4f0 - no description available"]
    #[inline(always)]
    pub const fn fusefuse060(&self) -> &FUSE {
        self.fuse(60)
    }
    #[doc = "0x4f4 - no description available"]
    #[inline(always)]
    pub const fn fusefuse061(&self) -> &FUSE {
        self.fuse(61)
    }
    #[doc = "0x4f8 - no description available"]
    #[inline(always)]
    pub const fn fusefuse062(&self) -> &FUSE {
        self.fuse(62)
    }
    #[doc = "0x4fc - no description available"]
    #[inline(always)]
    pub const fn fusefuse063(&self) -> &FUSE {
        self.fuse(63)
    }
    #[doc = "0x500 - no description available"]
    #[inline(always)]
    pub const fn fusefuse064(&self) -> &FUSE {
        self.fuse(64)
    }
    #[doc = "0x504 - no description available"]
    #[inline(always)]
    pub const fn fusefuse065(&self) -> &FUSE {
        self.fuse(65)
    }
    #[doc = "0x508 - no description available"]
    #[inline(always)]
    pub const fn fusefuse066(&self) -> &FUSE {
        self.fuse(66)
    }
    #[doc = "0x50c - no description available"]
    #[inline(always)]
    pub const fn fusefuse067(&self) -> &FUSE {
        self.fuse(67)
    }
    #[doc = "0x510 - no description available"]
    #[inline(always)]
    pub const fn fusefuse068(&self) -> &FUSE {
        self.fuse(68)
    }
    #[doc = "0x514 - no description available"]
    #[inline(always)]
    pub const fn fusefuse069(&self) -> &FUSE {
        self.fuse(69)
    }
    #[doc = "0x518 - no description available"]
    #[inline(always)]
    pub const fn fusefuse070(&self) -> &FUSE {
        self.fuse(70)
    }
    #[doc = "0x51c - no description available"]
    #[inline(always)]
    pub const fn fusefuse071(&self) -> &FUSE {
        self.fuse(71)
    }
    #[doc = "0x520 - no description available"]
    #[inline(always)]
    pub const fn fusefuse072(&self) -> &FUSE {
        self.fuse(72)
    }
    #[doc = "0x524 - no description available"]
    #[inline(always)]
    pub const fn fusefuse073(&self) -> &FUSE {
        self.fuse(73)
    }
    #[doc = "0x528 - no description available"]
    #[inline(always)]
    pub const fn fusefuse074(&self) -> &FUSE {
        self.fuse(74)
    }
    #[doc = "0x52c - no description available"]
    #[inline(always)]
    pub const fn fusefuse075(&self) -> &FUSE {
        self.fuse(75)
    }
    #[doc = "0x530 - no description available"]
    #[inline(always)]
    pub const fn fusefuse076(&self) -> &FUSE {
        self.fuse(76)
    }
    #[doc = "0x534 - no description available"]
    #[inline(always)]
    pub const fn fusefuse077(&self) -> &FUSE {
        self.fuse(77)
    }
    #[doc = "0x538 - no description available"]
    #[inline(always)]
    pub const fn fusefuse078(&self) -> &FUSE {
        self.fuse(78)
    }
    #[doc = "0x53c - no description available"]
    #[inline(always)]
    pub const fn fusefuse079(&self) -> &FUSE {
        self.fuse(79)
    }
    #[doc = "0x540 - no description available"]
    #[inline(always)]
    pub const fn fusefuse080(&self) -> &FUSE {
        self.fuse(80)
    }
    #[doc = "0x544 - no description available"]
    #[inline(always)]
    pub const fn fusefuse081(&self) -> &FUSE {
        self.fuse(81)
    }
    #[doc = "0x548 - no description available"]
    #[inline(always)]
    pub const fn fusefuse082(&self) -> &FUSE {
        self.fuse(82)
    }
    #[doc = "0x54c - no description available"]
    #[inline(always)]
    pub const fn fusefuse083(&self) -> &FUSE {
        self.fuse(83)
    }
    #[doc = "0x550 - no description available"]
    #[inline(always)]
    pub const fn fusefuse084(&self) -> &FUSE {
        self.fuse(84)
    }
    #[doc = "0x554 - no description available"]
    #[inline(always)]
    pub const fn fusefuse085(&self) -> &FUSE {
        self.fuse(85)
    }
    #[doc = "0x558 - no description available"]
    #[inline(always)]
    pub const fn fusefuse086(&self) -> &FUSE {
        self.fuse(86)
    }
    #[doc = "0x55c - no description available"]
    #[inline(always)]
    pub const fn fusefuse087(&self) -> &FUSE {
        self.fuse(87)
    }
    #[doc = "0x560 - no description available"]
    #[inline(always)]
    pub const fn fusefuse088(&self) -> &FUSE {
        self.fuse(88)
    }
    #[doc = "0x564 - no description available"]
    #[inline(always)]
    pub const fn fusefuse089(&self) -> &FUSE {
        self.fuse(89)
    }
    #[doc = "0x568 - no description available"]
    #[inline(always)]
    pub const fn fusefuse090(&self) -> &FUSE {
        self.fuse(90)
    }
    #[doc = "0x56c - no description available"]
    #[inline(always)]
    pub const fn fusefuse091(&self) -> &FUSE {
        self.fuse(91)
    }
    #[doc = "0x570 - no description available"]
    #[inline(always)]
    pub const fn fusefuse092(&self) -> &FUSE {
        self.fuse(92)
    }
    #[doc = "0x574 - no description available"]
    #[inline(always)]
    pub const fn fusefuse093(&self) -> &FUSE {
        self.fuse(93)
    }
    #[doc = "0x578 - no description available"]
    #[inline(always)]
    pub const fn fusefuse094(&self) -> &FUSE {
        self.fuse(94)
    }
    #[doc = "0x57c - no description available"]
    #[inline(always)]
    pub const fn fusefuse095(&self) -> &FUSE {
        self.fuse(95)
    }
    #[doc = "0x580 - no description available"]
    #[inline(always)]
    pub const fn fusefuse096(&self) -> &FUSE {
        self.fuse(96)
    }
    #[doc = "0x584 - no description available"]
    #[inline(always)]
    pub const fn fusefuse097(&self) -> &FUSE {
        self.fuse(97)
    }
    #[doc = "0x588 - no description available"]
    #[inline(always)]
    pub const fn fusefuse098(&self) -> &FUSE {
        self.fuse(98)
    }
    #[doc = "0x58c - no description available"]
    #[inline(always)]
    pub const fn fusefuse099(&self) -> &FUSE {
        self.fuse(99)
    }
    #[doc = "0x590 - no description available"]
    #[inline(always)]
    pub const fn fusefuse100(&self) -> &FUSE {
        self.fuse(100)
    }
    #[doc = "0x594 - no description available"]
    #[inline(always)]
    pub const fn fusefuse101(&self) -> &FUSE {
        self.fuse(101)
    }
    #[doc = "0x598 - no description available"]
    #[inline(always)]
    pub const fn fusefuse102(&self) -> &FUSE {
        self.fuse(102)
    }
    #[doc = "0x59c - no description available"]
    #[inline(always)]
    pub const fn fusefuse103(&self) -> &FUSE {
        self.fuse(103)
    }
    #[doc = "0x5a0 - no description available"]
    #[inline(always)]
    pub const fn fusefuse104(&self) -> &FUSE {
        self.fuse(104)
    }
    #[doc = "0x5a4 - no description available"]
    #[inline(always)]
    pub const fn fusefuse105(&self) -> &FUSE {
        self.fuse(105)
    }
    #[doc = "0x5a8 - no description available"]
    #[inline(always)]
    pub const fn fusefuse106(&self) -> &FUSE {
        self.fuse(106)
    }
    #[doc = "0x5ac - no description available"]
    #[inline(always)]
    pub const fn fusefuse107(&self) -> &FUSE {
        self.fuse(107)
    }
    #[doc = "0x5b0 - no description available"]
    #[inline(always)]
    pub const fn fusefuse108(&self) -> &FUSE {
        self.fuse(108)
    }
    #[doc = "0x5b4 - no description available"]
    #[inline(always)]
    pub const fn fusefuse109(&self) -> &FUSE {
        self.fuse(109)
    }
    #[doc = "0x5b8 - no description available"]
    #[inline(always)]
    pub const fn fusefuse110(&self) -> &FUSE {
        self.fuse(110)
    }
    #[doc = "0x5bc - no description available"]
    #[inline(always)]
    pub const fn fusefuse111(&self) -> &FUSE {
        self.fuse(111)
    }
    #[doc = "0x5c0 - no description available"]
    #[inline(always)]
    pub const fn fusefuse112(&self) -> &FUSE {
        self.fuse(112)
    }
    #[doc = "0x5c4 - no description available"]
    #[inline(always)]
    pub const fn fusefuse113(&self) -> &FUSE {
        self.fuse(113)
    }
    #[doc = "0x5c8 - no description available"]
    #[inline(always)]
    pub const fn fusefuse114(&self) -> &FUSE {
        self.fuse(114)
    }
    #[doc = "0x5cc - no description available"]
    #[inline(always)]
    pub const fn fusefuse115(&self) -> &FUSE {
        self.fuse(115)
    }
    #[doc = "0x5d0 - no description available"]
    #[inline(always)]
    pub const fn fusefuse116(&self) -> &FUSE {
        self.fuse(116)
    }
    #[doc = "0x5d4 - no description available"]
    #[inline(always)]
    pub const fn fusefuse117(&self) -> &FUSE {
        self.fuse(117)
    }
    #[doc = "0x5d8 - no description available"]
    #[inline(always)]
    pub const fn fusefuse118(&self) -> &FUSE {
        self.fuse(118)
    }
    #[doc = "0x5dc - no description available"]
    #[inline(always)]
    pub const fn fusefuse119(&self) -> &FUSE {
        self.fuse(119)
    }
    #[doc = "0x5e0 - no description available"]
    #[inline(always)]
    pub const fn fusefuse120(&self) -> &FUSE {
        self.fuse(120)
    }
    #[doc = "0x5e4 - no description available"]
    #[inline(always)]
    pub const fn fusefuse121(&self) -> &FUSE {
        self.fuse(121)
    }
    #[doc = "0x5e8 - no description available"]
    #[inline(always)]
    pub const fn fusefuse122(&self) -> &FUSE {
        self.fuse(122)
    }
    #[doc = "0x5ec - no description available"]
    #[inline(always)]
    pub const fn fusefuse123(&self) -> &FUSE {
        self.fuse(123)
    }
    #[doc = "0x5f0 - no description available"]
    #[inline(always)]
    pub const fn fusefuse124(&self) -> &FUSE {
        self.fuse(124)
    }
    #[doc = "0x5f4 - no description available"]
    #[inline(always)]
    pub const fn fusefuse125(&self) -> &FUSE {
        self.fuse(125)
    }
    #[doc = "0x5f8 - no description available"]
    #[inline(always)]
    pub const fn fusefuse126(&self) -> &FUSE {
        self.fuse(126)
    }
    #[doc = "0x5fc - no description available"]
    #[inline(always)]
    pub const fn fusefuse127(&self) -> &FUSE {
        self.fuse(127)
    }
    #[doc = "0x600..0x620 - no description available"]
    #[inline(always)]
    pub const fn fuse_lock(&self, n: usize) -> &FUSE_LOCK {
        &self.fuse_lock[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x600..0x620 - no description available"]
    #[inline(always)]
    pub fn fuse_lock_iter(&self) -> impl Iterator<Item = &FUSE_LOCK> {
        self.fuse_lock.iter()
    }
    #[doc = "0x600 - no description available"]
    #[inline(always)]
    pub const fn fuse_locklock00(&self) -> &FUSE_LOCK {
        self.fuse_lock(0)
    }
    #[doc = "0x604 - no description available"]
    #[inline(always)]
    pub const fn fuse_locklock01(&self) -> &FUSE_LOCK {
        self.fuse_lock(1)
    }
    #[doc = "0x608 - no description available"]
    #[inline(always)]
    pub const fn fuse_locklock02(&self) -> &FUSE_LOCK {
        self.fuse_lock(2)
    }
    #[doc = "0x60c - no description available"]
    #[inline(always)]
    pub const fn fuse_locklock03(&self) -> &FUSE_LOCK {
        self.fuse_lock(3)
    }
    #[doc = "0x610 - no description available"]
    #[inline(always)]
    pub const fn fuse_locklock04(&self) -> &FUSE_LOCK {
        self.fuse_lock(4)
    }
    #[doc = "0x614 - no description available"]
    #[inline(always)]
    pub const fn fuse_locklock05(&self) -> &FUSE_LOCK {
        self.fuse_lock(5)
    }
    #[doc = "0x618 - no description available"]
    #[inline(always)]
    pub const fn fuse_locklock06(&self) -> &FUSE_LOCK {
        self.fuse_lock(6)
    }
    #[doc = "0x61c - no description available"]
    #[inline(always)]
    pub const fn fuse_locklock07(&self) -> &FUSE_LOCK {
        self.fuse_lock(7)
    }
    #[doc = "0x800 - UNLOCK"]
    #[inline(always)]
    pub const fn unlock(&self) -> &UNLOCK {
        &self.unlock
    }
    #[doc = "0x804 - DATA"]
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
    #[doc = "0x808 - ADDR"]
    #[inline(always)]
    pub const fn addr(&self) -> &ADDR {
        &self.addr
    }
    #[doc = "0x80c - CMD"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0xa00 - LOAD Request"]
    #[inline(always)]
    pub const fn load_req(&self) -> &LOAD_REQ {
        &self.load_req
    }
    #[doc = "0xa04 - LOAD complete"]
    #[inline(always)]
    pub const fn load_comp(&self) -> &LOAD_COMP {
        &self.load_comp
    }
    #[doc = "0xa20..0xa30 - no description available"]
    #[inline(always)]
    pub const fn region(&self, n: usize) -> &REGION {
        &self.region[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa20..0xa30 - no description available"]
    #[inline(always)]
    pub fn region_iter(&self) -> impl Iterator<Item = &REGION> {
        self.region.iter()
    }
    #[doc = "0xa20 - no description available"]
    #[inline(always)]
    pub const fn regionload_region0(&self) -> &REGION {
        self.region(0)
    }
    #[doc = "0xa24 - no description available"]
    #[inline(always)]
    pub const fn regionload_region1(&self) -> &REGION {
        self.region(1)
    }
    #[doc = "0xa28 - no description available"]
    #[inline(always)]
    pub const fn regionload_region2(&self) -> &REGION {
        self.region(2)
    }
    #[doc = "0xa2c - no description available"]
    #[inline(always)]
    pub const fn regionload_region3(&self) -> &REGION {
        self.region(3)
    }
    #[doc = "0xc00 - interrupt flag"]
    #[inline(always)]
    pub const fn int_flag(&self) -> &INT_FLAG {
        &self.int_flag
    }
    #[doc = "0xc04 - interrupt enable"]
    #[inline(always)]
    pub const fn int_en(&self) -> &INT_EN {
        &self.int_en
    }
}
#[doc = "SHADOW (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shadow::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shadow::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shadow`]
module"]
pub type SHADOW = crate::Reg<shadow::SHADOW_SPEC>;
#[doc = "no description available"]
pub mod shadow;
#[doc = "SHADOW_LOCK (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shadow_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shadow_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shadow_lock`]
module"]
pub type SHADOW_LOCK = crate::Reg<shadow_lock::SHADOW_LOCK_SPEC>;
#[doc = "no description available"]
pub mod shadow_lock;
#[doc = "FUSE (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fuse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fuse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fuse`]
module"]
pub type FUSE = crate::Reg<fuse::FUSE_SPEC>;
#[doc = "no description available"]
pub mod fuse;
#[doc = "FUSE_LOCK (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fuse_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fuse_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fuse_lock`]
module"]
pub type FUSE_LOCK = crate::Reg<fuse_lock::FUSE_LOCK_SPEC>;
#[doc = "no description available"]
pub mod fuse_lock;
#[doc = "UNLOCK (rw) register accessor: UNLOCK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`unlock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unlock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unlock`]
module"]
pub type UNLOCK = crate::Reg<unlock::UNLOCK_SPEC>;
#[doc = "UNLOCK"]
pub mod unlock;
#[doc = "DATA (rw) register accessor: DATA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "DATA"]
pub mod data;
#[doc = "ADDR (rw) register accessor: ADDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "ADDR"]
pub mod addr;
#[doc = "CMD (rw) register accessor: CMD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "CMD"]
pub mod cmd;
#[doc = "LOAD_REQ (rw) register accessor: LOAD Request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`load_req::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load_req::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@load_req`]
module"]
pub type LOAD_REQ = crate::Reg<load_req::LOAD_REQ_SPEC>;
#[doc = "LOAD Request"]
pub mod load_req;
#[doc = "LOAD_COMP (rw) register accessor: LOAD complete\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`load_comp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load_comp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@load_comp`]
module"]
pub type LOAD_COMP = crate::Reg<load_comp::LOAD_COMP_SPEC>;
#[doc = "LOAD complete"]
pub mod load_comp;
#[doc = "REGION (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region`]
module"]
pub type REGION = crate::Reg<region::REGION_SPEC>;
#[doc = "no description available"]
pub mod region;
#[doc = "INT_FLAG (rw) register accessor: interrupt flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_flag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_flag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_flag`]
module"]
pub type INT_FLAG = crate::Reg<int_flag::INT_FLAG_SPEC>;
#[doc = "interrupt flag"]
pub mod int_flag;
#[doc = "INT_EN (rw) register accessor: interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_en`]
module"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "interrupt enable"]
pub mod int_en;
