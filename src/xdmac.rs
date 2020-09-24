#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global Type Register"]
    pub gtype: GTYPE,
    #[doc = "0x04 - Global Configuration Register"]
    pub gcfg: GCFG,
    #[doc = "0x08 - Global Weighted Arbiter Configuration Register"]
    pub gwac: GWAC,
    #[doc = "0x0c - Global Interrupt Enable Register"]
    pub gie: GIE,
    #[doc = "0x10 - Global Interrupt Disable Register"]
    pub gid: GID,
    #[doc = "0x14 - Global Interrupt Mask Register"]
    pub gim: GIM,
    #[doc = "0x18 - Global Interrupt Status Register"]
    pub gis: GIS,
    #[doc = "0x1c - Global Channel Enable Register"]
    pub ge: GE,
    #[doc = "0x20 - Global Channel Disable Register"]
    pub gd: GD,
    #[doc = "0x24 - Global Channel Status Register"]
    pub gs: GS,
    #[doc = "0x28 - Global Channel Read Suspend Register"]
    pub grs: GRS,
    #[doc = "0x2c - Global Channel Write Suspend Register"]
    pub gws: GWS,
    #[doc = "0x30 - Global Channel Read Write Suspend Register"]
    pub grws: GRWS,
    #[doc = "0x34 - Global Channel Read Write Resume Register"]
    pub grwr: GRWR,
    #[doc = "0x38 - Global Channel Software Request Register"]
    pub gswr: GSWR,
    #[doc = "0x3c - Global Channel Software Request Status Register"]
    pub gsws: GSWS,
    #[doc = "0x40 - Global Channel Software Flush Request Register"]
    pub gswf: GSWF,
    _reserved17: [u8; 12usize],
    #[doc = "0x50 - Channel Interrupt Enable Register (chid = 0)"]
    pub cie0: CIE0,
    #[doc = "0x54 - Channel Interrupt Disable Register (chid = 0)"]
    pub cid0: CID0,
    #[doc = "0x58 - Channel Interrupt Mask Register (chid = 0)"]
    pub cim0: CIM0,
    #[doc = "0x5c - Channel Interrupt Status Register (chid = 0)"]
    pub cis0: CIS0,
    #[doc = "0x60 - Channel Source Address Register (chid = 0)"]
    pub csa0: CSA0,
    #[doc = "0x64 - Channel Destination Address Register (chid = 0)"]
    pub cda0: CDA0,
    #[doc = "0x68 - Channel Next Descriptor Address Register (chid = 0)"]
    pub cnda0: CNDA0,
    #[doc = "0x6c - Channel Next Descriptor Control Register (chid = 0)"]
    pub cndc0: CNDC0,
    #[doc = "0x70 - Channel Microblock Control Register (chid = 0)"]
    pub cubc0: CUBC0,
    #[doc = "0x74 - Channel Block Control Register (chid = 0)"]
    pub cbc0: CBC0,
    #[doc = "0x78 - Channel Configuration Register (chid = 0)"]
    pub cc0: CC0,
    #[doc = "0x7c - Channel Data Stride Memory Set Pattern (chid = 0)"]
    pub cds_msp0: CDS_MSP0,
    #[doc = "0x80 - Channel Source Microblock Stride (chid = 0)"]
    pub csus0: CSUS0,
    #[doc = "0x84 - Channel Destination Microblock Stride (chid = 0)"]
    pub cdus0: CDUS0,
    _reserved31: [u8; 8usize],
    #[doc = "0x90 - Channel Interrupt Enable Register (chid = 1)"]
    pub cie1: CIE1,
    #[doc = "0x94 - Channel Interrupt Disable Register (chid = 1)"]
    pub cid1: CID1,
    #[doc = "0x98 - Channel Interrupt Mask Register (chid = 1)"]
    pub cim1: CIM1,
    #[doc = "0x9c - Channel Interrupt Status Register (chid = 1)"]
    pub cis1: CIS1,
    #[doc = "0xa0 - Channel Source Address Register (chid = 1)"]
    pub csa1: CSA1,
    #[doc = "0xa4 - Channel Destination Address Register (chid = 1)"]
    pub cda1: CDA1,
    #[doc = "0xa8 - Channel Next Descriptor Address Register (chid = 1)"]
    pub cnda1: CNDA1,
    #[doc = "0xac - Channel Next Descriptor Control Register (chid = 1)"]
    pub cndc1: CNDC1,
    #[doc = "0xb0 - Channel Microblock Control Register (chid = 1)"]
    pub cubc1: CUBC1,
    #[doc = "0xb4 - Channel Block Control Register (chid = 1)"]
    pub cbc1: CBC1,
    #[doc = "0xb8 - Channel Configuration Register (chid = 1)"]
    pub cc1: CC1,
    #[doc = "0xbc - Channel Data Stride Memory Set Pattern (chid = 1)"]
    pub cds_msp1: CDS_MSP1,
    #[doc = "0xc0 - Channel Source Microblock Stride (chid = 1)"]
    pub csus1: CSUS1,
    #[doc = "0xc4 - Channel Destination Microblock Stride (chid = 1)"]
    pub cdus1: CDUS1,
    _reserved45: [u8; 8usize],
    #[doc = "0xd0 - Channel Interrupt Enable Register (chid = 2)"]
    pub cie2: CIE2,
    #[doc = "0xd4 - Channel Interrupt Disable Register (chid = 2)"]
    pub cid2: CID2,
    #[doc = "0xd8 - Channel Interrupt Mask Register (chid = 2)"]
    pub cim2: CIM2,
    #[doc = "0xdc - Channel Interrupt Status Register (chid = 2)"]
    pub cis2: CIS2,
    #[doc = "0xe0 - Channel Source Address Register (chid = 2)"]
    pub csa2: CSA2,
    #[doc = "0xe4 - Channel Destination Address Register (chid = 2)"]
    pub cda2: CDA2,
    #[doc = "0xe8 - Channel Next Descriptor Address Register (chid = 2)"]
    pub cnda2: CNDA2,
    #[doc = "0xec - Channel Next Descriptor Control Register (chid = 2)"]
    pub cndc2: CNDC2,
    #[doc = "0xf0 - Channel Microblock Control Register (chid = 2)"]
    pub cubc2: CUBC2,
    #[doc = "0xf4 - Channel Block Control Register (chid = 2)"]
    pub cbc2: CBC2,
    #[doc = "0xf8 - Channel Configuration Register (chid = 2)"]
    pub cc2: CC2,
    #[doc = "0xfc - Channel Data Stride Memory Set Pattern (chid = 2)"]
    pub cds_msp2: CDS_MSP2,
    #[doc = "0x100 - Channel Source Microblock Stride (chid = 2)"]
    pub csus2: CSUS2,
    #[doc = "0x104 - Channel Destination Microblock Stride (chid = 2)"]
    pub cdus2: CDUS2,
    _reserved59: [u8; 8usize],
    #[doc = "0x110 - Channel Interrupt Enable Register (chid = 3)"]
    pub cie3: CIE3,
    #[doc = "0x114 - Channel Interrupt Disable Register (chid = 3)"]
    pub cid3: CID3,
    #[doc = "0x118 - Channel Interrupt Mask Register (chid = 3)"]
    pub cim3: CIM3,
    #[doc = "0x11c - Channel Interrupt Status Register (chid = 3)"]
    pub cis3: CIS3,
    #[doc = "0x120 - Channel Source Address Register (chid = 3)"]
    pub csa3: CSA3,
    #[doc = "0x124 - Channel Destination Address Register (chid = 3)"]
    pub cda3: CDA3,
    #[doc = "0x128 - Channel Next Descriptor Address Register (chid = 3)"]
    pub cnda3: CNDA3,
    #[doc = "0x12c - Channel Next Descriptor Control Register (chid = 3)"]
    pub cndc3: CNDC3,
    #[doc = "0x130 - Channel Microblock Control Register (chid = 3)"]
    pub cubc3: CUBC3,
    #[doc = "0x134 - Channel Block Control Register (chid = 3)"]
    pub cbc3: CBC3,
    #[doc = "0x138 - Channel Configuration Register (chid = 3)"]
    pub cc3: CC3,
    #[doc = "0x13c - Channel Data Stride Memory Set Pattern (chid = 3)"]
    pub cds_msp3: CDS_MSP3,
    #[doc = "0x140 - Channel Source Microblock Stride (chid = 3)"]
    pub csus3: CSUS3,
    #[doc = "0x144 - Channel Destination Microblock Stride (chid = 3)"]
    pub cdus3: CDUS3,
    _reserved73: [u8; 8usize],
    #[doc = "0x150 - Channel Interrupt Enable Register (chid = 4)"]
    pub cie4: CIE4,
    #[doc = "0x154 - Channel Interrupt Disable Register (chid = 4)"]
    pub cid4: CID4,
    #[doc = "0x158 - Channel Interrupt Mask Register (chid = 4)"]
    pub cim4: CIM4,
    #[doc = "0x15c - Channel Interrupt Status Register (chid = 4)"]
    pub cis4: CIS4,
    #[doc = "0x160 - Channel Source Address Register (chid = 4)"]
    pub csa4: CSA4,
    #[doc = "0x164 - Channel Destination Address Register (chid = 4)"]
    pub cda4: CDA4,
    #[doc = "0x168 - Channel Next Descriptor Address Register (chid = 4)"]
    pub cnda4: CNDA4,
    #[doc = "0x16c - Channel Next Descriptor Control Register (chid = 4)"]
    pub cndc4: CNDC4,
    #[doc = "0x170 - Channel Microblock Control Register (chid = 4)"]
    pub cubc4: CUBC4,
    #[doc = "0x174 - Channel Block Control Register (chid = 4)"]
    pub cbc4: CBC4,
    #[doc = "0x178 - Channel Configuration Register (chid = 4)"]
    pub cc4: CC4,
    #[doc = "0x17c - Channel Data Stride Memory Set Pattern (chid = 4)"]
    pub cds_msp4: CDS_MSP4,
    #[doc = "0x180 - Channel Source Microblock Stride (chid = 4)"]
    pub csus4: CSUS4,
    #[doc = "0x184 - Channel Destination Microblock Stride (chid = 4)"]
    pub cdus4: CDUS4,
    _reserved87: [u8; 8usize],
    #[doc = "0x190 - Channel Interrupt Enable Register (chid = 5)"]
    pub cie5: CIE5,
    #[doc = "0x194 - Channel Interrupt Disable Register (chid = 5)"]
    pub cid5: CID5,
    #[doc = "0x198 - Channel Interrupt Mask Register (chid = 5)"]
    pub cim5: CIM5,
    #[doc = "0x19c - Channel Interrupt Status Register (chid = 5)"]
    pub cis5: CIS5,
    #[doc = "0x1a0 - Channel Source Address Register (chid = 5)"]
    pub csa5: CSA5,
    #[doc = "0x1a4 - Channel Destination Address Register (chid = 5)"]
    pub cda5: CDA5,
    #[doc = "0x1a8 - Channel Next Descriptor Address Register (chid = 5)"]
    pub cnda5: CNDA5,
    #[doc = "0x1ac - Channel Next Descriptor Control Register (chid = 5)"]
    pub cndc5: CNDC5,
    #[doc = "0x1b0 - Channel Microblock Control Register (chid = 5)"]
    pub cubc5: CUBC5,
    #[doc = "0x1b4 - Channel Block Control Register (chid = 5)"]
    pub cbc5: CBC5,
    #[doc = "0x1b8 - Channel Configuration Register (chid = 5)"]
    pub cc5: CC5,
    #[doc = "0x1bc - Channel Data Stride Memory Set Pattern (chid = 5)"]
    pub cds_msp5: CDS_MSP5,
    #[doc = "0x1c0 - Channel Source Microblock Stride (chid = 5)"]
    pub csus5: CSUS5,
    #[doc = "0x1c4 - Channel Destination Microblock Stride (chid = 5)"]
    pub cdus5: CDUS5,
    _reserved101: [u8; 8usize],
    #[doc = "0x1d0 - Channel Interrupt Enable Register (chid = 6)"]
    pub cie6: CIE6,
    #[doc = "0x1d4 - Channel Interrupt Disable Register (chid = 6)"]
    pub cid6: CID6,
    #[doc = "0x1d8 - Channel Interrupt Mask Register (chid = 6)"]
    pub cim6: CIM6,
    #[doc = "0x1dc - Channel Interrupt Status Register (chid = 6)"]
    pub cis6: CIS6,
    #[doc = "0x1e0 - Channel Source Address Register (chid = 6)"]
    pub csa6: CSA6,
    #[doc = "0x1e4 - Channel Destination Address Register (chid = 6)"]
    pub cda6: CDA6,
    #[doc = "0x1e8 - Channel Next Descriptor Address Register (chid = 6)"]
    pub cnda6: CNDA6,
    #[doc = "0x1ec - Channel Next Descriptor Control Register (chid = 6)"]
    pub cndc6: CNDC6,
    #[doc = "0x1f0 - Channel Microblock Control Register (chid = 6)"]
    pub cubc6: CUBC6,
    #[doc = "0x1f4 - Channel Block Control Register (chid = 6)"]
    pub cbc6: CBC6,
    #[doc = "0x1f8 - Channel Configuration Register (chid = 6)"]
    pub cc6: CC6,
    #[doc = "0x1fc - Channel Data Stride Memory Set Pattern (chid = 6)"]
    pub cds_msp6: CDS_MSP6,
    #[doc = "0x200 - Channel Source Microblock Stride (chid = 6)"]
    pub csus6: CSUS6,
    #[doc = "0x204 - Channel Destination Microblock Stride (chid = 6)"]
    pub cdus6: CDUS6,
    _reserved115: [u8; 8usize],
    #[doc = "0x210 - Channel Interrupt Enable Register (chid = 7)"]
    pub cie7: CIE7,
    #[doc = "0x214 - Channel Interrupt Disable Register (chid = 7)"]
    pub cid7: CID7,
    #[doc = "0x218 - Channel Interrupt Mask Register (chid = 7)"]
    pub cim7: CIM7,
    #[doc = "0x21c - Channel Interrupt Status Register (chid = 7)"]
    pub cis7: CIS7,
    #[doc = "0x220 - Channel Source Address Register (chid = 7)"]
    pub csa7: CSA7,
    #[doc = "0x224 - Channel Destination Address Register (chid = 7)"]
    pub cda7: CDA7,
    #[doc = "0x228 - Channel Next Descriptor Address Register (chid = 7)"]
    pub cnda7: CNDA7,
    #[doc = "0x22c - Channel Next Descriptor Control Register (chid = 7)"]
    pub cndc7: CNDC7,
    #[doc = "0x230 - Channel Microblock Control Register (chid = 7)"]
    pub cubc7: CUBC7,
    #[doc = "0x234 - Channel Block Control Register (chid = 7)"]
    pub cbc7: CBC7,
    #[doc = "0x238 - Channel Configuration Register (chid = 7)"]
    pub cc7: CC7,
    #[doc = "0x23c - Channel Data Stride Memory Set Pattern (chid = 7)"]
    pub cds_msp7: CDS_MSP7,
    #[doc = "0x240 - Channel Source Microblock Stride (chid = 7)"]
    pub csus7: CSUS7,
    #[doc = "0x244 - Channel Destination Microblock Stride (chid = 7)"]
    pub cdus7: CDUS7,
    _reserved129: [u8; 8usize],
    #[doc = "0x250 - Channel Interrupt Enable Register (chid = 8)"]
    pub cie8: CIE8,
    #[doc = "0x254 - Channel Interrupt Disable Register (chid = 8)"]
    pub cid8: CID8,
    #[doc = "0x258 - Channel Interrupt Mask Register (chid = 8)"]
    pub cim8: CIM8,
    #[doc = "0x25c - Channel Interrupt Status Register (chid = 8)"]
    pub cis8: CIS8,
    #[doc = "0x260 - Channel Source Address Register (chid = 8)"]
    pub csa8: CSA8,
    #[doc = "0x264 - Channel Destination Address Register (chid = 8)"]
    pub cda8: CDA8,
    #[doc = "0x268 - Channel Next Descriptor Address Register (chid = 8)"]
    pub cnda8: CNDA8,
    #[doc = "0x26c - Channel Next Descriptor Control Register (chid = 8)"]
    pub cndc8: CNDC8,
    #[doc = "0x270 - Channel Microblock Control Register (chid = 8)"]
    pub cubc8: CUBC8,
    #[doc = "0x274 - Channel Block Control Register (chid = 8)"]
    pub cbc8: CBC8,
    #[doc = "0x278 - Channel Configuration Register (chid = 8)"]
    pub cc8: CC8,
    #[doc = "0x27c - Channel Data Stride Memory Set Pattern (chid = 8)"]
    pub cds_msp8: CDS_MSP8,
    #[doc = "0x280 - Channel Source Microblock Stride (chid = 8)"]
    pub csus8: CSUS8,
    #[doc = "0x284 - Channel Destination Microblock Stride (chid = 8)"]
    pub cdus8: CDUS8,
    _reserved143: [u8; 8usize],
    #[doc = "0x290 - Channel Interrupt Enable Register (chid = 9)"]
    pub cie9: CIE9,
    #[doc = "0x294 - Channel Interrupt Disable Register (chid = 9)"]
    pub cid9: CID9,
    #[doc = "0x298 - Channel Interrupt Mask Register (chid = 9)"]
    pub cim9: CIM9,
    #[doc = "0x29c - Channel Interrupt Status Register (chid = 9)"]
    pub cis9: CIS9,
    #[doc = "0x2a0 - Channel Source Address Register (chid = 9)"]
    pub csa9: CSA9,
    #[doc = "0x2a4 - Channel Destination Address Register (chid = 9)"]
    pub cda9: CDA9,
    #[doc = "0x2a8 - Channel Next Descriptor Address Register (chid = 9)"]
    pub cnda9: CNDA9,
    #[doc = "0x2ac - Channel Next Descriptor Control Register (chid = 9)"]
    pub cndc9: CNDC9,
    #[doc = "0x2b0 - Channel Microblock Control Register (chid = 9)"]
    pub cubc9: CUBC9,
    #[doc = "0x2b4 - Channel Block Control Register (chid = 9)"]
    pub cbc9: CBC9,
    #[doc = "0x2b8 - Channel Configuration Register (chid = 9)"]
    pub cc9: CC9,
    #[doc = "0x2bc - Channel Data Stride Memory Set Pattern (chid = 9)"]
    pub cds_msp9: CDS_MSP9,
    #[doc = "0x2c0 - Channel Source Microblock Stride (chid = 9)"]
    pub csus9: CSUS9,
    #[doc = "0x2c4 - Channel Destination Microblock Stride (chid = 9)"]
    pub cdus9: CDUS9,
    _reserved157: [u8; 8usize],
    #[doc = "0x2d0 - Channel Interrupt Enable Register (chid = 10)"]
    pub cie10: CIE10,
    #[doc = "0x2d4 - Channel Interrupt Disable Register (chid = 10)"]
    pub cid10: CID10,
    #[doc = "0x2d8 - Channel Interrupt Mask Register (chid = 10)"]
    pub cim10: CIM10,
    #[doc = "0x2dc - Channel Interrupt Status Register (chid = 10)"]
    pub cis10: CIS10,
    #[doc = "0x2e0 - Channel Source Address Register (chid = 10)"]
    pub csa10: CSA10,
    #[doc = "0x2e4 - Channel Destination Address Register (chid = 10)"]
    pub cda10: CDA10,
    #[doc = "0x2e8 - Channel Next Descriptor Address Register (chid = 10)"]
    pub cnda10: CNDA10,
    #[doc = "0x2ec - Channel Next Descriptor Control Register (chid = 10)"]
    pub cndc10: CNDC10,
    #[doc = "0x2f0 - Channel Microblock Control Register (chid = 10)"]
    pub cubc10: CUBC10,
    #[doc = "0x2f4 - Channel Block Control Register (chid = 10)"]
    pub cbc10: CBC10,
    #[doc = "0x2f8 - Channel Configuration Register (chid = 10)"]
    pub cc10: CC10,
    #[doc = "0x2fc - Channel Data Stride Memory Set Pattern (chid = 10)"]
    pub cds_msp10: CDS_MSP10,
    #[doc = "0x300 - Channel Source Microblock Stride (chid = 10)"]
    pub csus10: CSUS10,
    #[doc = "0x304 - Channel Destination Microblock Stride (chid = 10)"]
    pub cdus10: CDUS10,
    _reserved171: [u8; 8usize],
    #[doc = "0x310 - Channel Interrupt Enable Register (chid = 11)"]
    pub cie11: CIE11,
    #[doc = "0x314 - Channel Interrupt Disable Register (chid = 11)"]
    pub cid11: CID11,
    #[doc = "0x318 - Channel Interrupt Mask Register (chid = 11)"]
    pub cim11: CIM11,
    #[doc = "0x31c - Channel Interrupt Status Register (chid = 11)"]
    pub cis11: CIS11,
    #[doc = "0x320 - Channel Source Address Register (chid = 11)"]
    pub csa11: CSA11,
    #[doc = "0x324 - Channel Destination Address Register (chid = 11)"]
    pub cda11: CDA11,
    #[doc = "0x328 - Channel Next Descriptor Address Register (chid = 11)"]
    pub cnda11: CNDA11,
    #[doc = "0x32c - Channel Next Descriptor Control Register (chid = 11)"]
    pub cndc11: CNDC11,
    #[doc = "0x330 - Channel Microblock Control Register (chid = 11)"]
    pub cubc11: CUBC11,
    #[doc = "0x334 - Channel Block Control Register (chid = 11)"]
    pub cbc11: CBC11,
    #[doc = "0x338 - Channel Configuration Register (chid = 11)"]
    pub cc11: CC11,
    #[doc = "0x33c - Channel Data Stride Memory Set Pattern (chid = 11)"]
    pub cds_msp11: CDS_MSP11,
    #[doc = "0x340 - Channel Source Microblock Stride (chid = 11)"]
    pub csus11: CSUS11,
    #[doc = "0x344 - Channel Destination Microblock Stride (chid = 11)"]
    pub cdus11: CDUS11,
    _reserved185: [u8; 8usize],
    #[doc = "0x350 - Channel Interrupt Enable Register (chid = 12)"]
    pub cie12: CIE12,
    #[doc = "0x354 - Channel Interrupt Disable Register (chid = 12)"]
    pub cid12: CID12,
    #[doc = "0x358 - Channel Interrupt Mask Register (chid = 12)"]
    pub cim12: CIM12,
    #[doc = "0x35c - Channel Interrupt Status Register (chid = 12)"]
    pub cis12: CIS12,
    #[doc = "0x360 - Channel Source Address Register (chid = 12)"]
    pub csa12: CSA12,
    #[doc = "0x364 - Channel Destination Address Register (chid = 12)"]
    pub cda12: CDA12,
    #[doc = "0x368 - Channel Next Descriptor Address Register (chid = 12)"]
    pub cnda12: CNDA12,
    #[doc = "0x36c - Channel Next Descriptor Control Register (chid = 12)"]
    pub cndc12: CNDC12,
    #[doc = "0x370 - Channel Microblock Control Register (chid = 12)"]
    pub cubc12: CUBC12,
    #[doc = "0x374 - Channel Block Control Register (chid = 12)"]
    pub cbc12: CBC12,
    #[doc = "0x378 - Channel Configuration Register (chid = 12)"]
    pub cc12: CC12,
    #[doc = "0x37c - Channel Data Stride Memory Set Pattern (chid = 12)"]
    pub cds_msp12: CDS_MSP12,
    #[doc = "0x380 - Channel Source Microblock Stride (chid = 12)"]
    pub csus12: CSUS12,
    #[doc = "0x384 - Channel Destination Microblock Stride (chid = 12)"]
    pub cdus12: CDUS12,
    _reserved199: [u8; 8usize],
    #[doc = "0x390 - Channel Interrupt Enable Register (chid = 13)"]
    pub cie13: CIE13,
    #[doc = "0x394 - Channel Interrupt Disable Register (chid = 13)"]
    pub cid13: CID13,
    #[doc = "0x398 - Channel Interrupt Mask Register (chid = 13)"]
    pub cim13: CIM13,
    #[doc = "0x39c - Channel Interrupt Status Register (chid = 13)"]
    pub cis13: CIS13,
    #[doc = "0x3a0 - Channel Source Address Register (chid = 13)"]
    pub csa13: CSA13,
    #[doc = "0x3a4 - Channel Destination Address Register (chid = 13)"]
    pub cda13: CDA13,
    #[doc = "0x3a8 - Channel Next Descriptor Address Register (chid = 13)"]
    pub cnda13: CNDA13,
    #[doc = "0x3ac - Channel Next Descriptor Control Register (chid = 13)"]
    pub cndc13: CNDC13,
    #[doc = "0x3b0 - Channel Microblock Control Register (chid = 13)"]
    pub cubc13: CUBC13,
    #[doc = "0x3b4 - Channel Block Control Register (chid = 13)"]
    pub cbc13: CBC13,
    #[doc = "0x3b8 - Channel Configuration Register (chid = 13)"]
    pub cc13: CC13,
    #[doc = "0x3bc - Channel Data Stride Memory Set Pattern (chid = 13)"]
    pub cds_msp13: CDS_MSP13,
    #[doc = "0x3c0 - Channel Source Microblock Stride (chid = 13)"]
    pub csus13: CSUS13,
    #[doc = "0x3c4 - Channel Destination Microblock Stride (chid = 13)"]
    pub cdus13: CDUS13,
    _reserved213: [u8; 8usize],
    #[doc = "0x3d0 - Channel Interrupt Enable Register (chid = 14)"]
    pub cie14: CIE14,
    #[doc = "0x3d4 - Channel Interrupt Disable Register (chid = 14)"]
    pub cid14: CID14,
    #[doc = "0x3d8 - Channel Interrupt Mask Register (chid = 14)"]
    pub cim14: CIM14,
    #[doc = "0x3dc - Channel Interrupt Status Register (chid = 14)"]
    pub cis14: CIS14,
    #[doc = "0x3e0 - Channel Source Address Register (chid = 14)"]
    pub csa14: CSA14,
    #[doc = "0x3e4 - Channel Destination Address Register (chid = 14)"]
    pub cda14: CDA14,
    #[doc = "0x3e8 - Channel Next Descriptor Address Register (chid = 14)"]
    pub cnda14: CNDA14,
    #[doc = "0x3ec - Channel Next Descriptor Control Register (chid = 14)"]
    pub cndc14: CNDC14,
    #[doc = "0x3f0 - Channel Microblock Control Register (chid = 14)"]
    pub cubc14: CUBC14,
    #[doc = "0x3f4 - Channel Block Control Register (chid = 14)"]
    pub cbc14: CBC14,
    #[doc = "0x3f8 - Channel Configuration Register (chid = 14)"]
    pub cc14: CC14,
    #[doc = "0x3fc - Channel Data Stride Memory Set Pattern (chid = 14)"]
    pub cds_msp14: CDS_MSP14,
    #[doc = "0x400 - Channel Source Microblock Stride (chid = 14)"]
    pub csus14: CSUS14,
    #[doc = "0x404 - Channel Destination Microblock Stride (chid = 14)"]
    pub cdus14: CDUS14,
    _reserved227: [u8; 8usize],
    #[doc = "0x410 - Channel Interrupt Enable Register (chid = 15)"]
    pub cie15: CIE15,
    #[doc = "0x414 - Channel Interrupt Disable Register (chid = 15)"]
    pub cid15: CID15,
    #[doc = "0x418 - Channel Interrupt Mask Register (chid = 15)"]
    pub cim15: CIM15,
    #[doc = "0x41c - Channel Interrupt Status Register (chid = 15)"]
    pub cis15: CIS15,
    #[doc = "0x420 - Channel Source Address Register (chid = 15)"]
    pub csa15: CSA15,
    #[doc = "0x424 - Channel Destination Address Register (chid = 15)"]
    pub cda15: CDA15,
    #[doc = "0x428 - Channel Next Descriptor Address Register (chid = 15)"]
    pub cnda15: CNDA15,
    #[doc = "0x42c - Channel Next Descriptor Control Register (chid = 15)"]
    pub cndc15: CNDC15,
    #[doc = "0x430 - Channel Microblock Control Register (chid = 15)"]
    pub cubc15: CUBC15,
    #[doc = "0x434 - Channel Block Control Register (chid = 15)"]
    pub cbc15: CBC15,
    #[doc = "0x438 - Channel Configuration Register (chid = 15)"]
    pub cc15: CC15,
    #[doc = "0x43c - Channel Data Stride Memory Set Pattern (chid = 15)"]
    pub cds_msp15: CDS_MSP15,
    #[doc = "0x440 - Channel Source Microblock Stride (chid = 15)"]
    pub csus15: CSUS15,
    #[doc = "0x444 - Channel Destination Microblock Stride (chid = 15)"]
    pub cdus15: CDUS15,
    _reserved241: [u8; 8usize],
    #[doc = "0x450 - Channel Interrupt Enable Register (chid = 16)"]
    pub cie16: CIE16,
    #[doc = "0x454 - Channel Interrupt Disable Register (chid = 16)"]
    pub cid16: CID16,
    #[doc = "0x458 - Channel Interrupt Mask Register (chid = 16)"]
    pub cim16: CIM16,
    #[doc = "0x45c - Channel Interrupt Status Register (chid = 16)"]
    pub cis16: CIS16,
    #[doc = "0x460 - Channel Source Address Register (chid = 16)"]
    pub csa16: CSA16,
    #[doc = "0x464 - Channel Destination Address Register (chid = 16)"]
    pub cda16: CDA16,
    #[doc = "0x468 - Channel Next Descriptor Address Register (chid = 16)"]
    pub cnda16: CNDA16,
    #[doc = "0x46c - Channel Next Descriptor Control Register (chid = 16)"]
    pub cndc16: CNDC16,
    #[doc = "0x470 - Channel Microblock Control Register (chid = 16)"]
    pub cubc16: CUBC16,
    #[doc = "0x474 - Channel Block Control Register (chid = 16)"]
    pub cbc16: CBC16,
    #[doc = "0x478 - Channel Configuration Register (chid = 16)"]
    pub cc16: CC16,
    #[doc = "0x47c - Channel Data Stride Memory Set Pattern (chid = 16)"]
    pub cds_msp16: CDS_MSP16,
    #[doc = "0x480 - Channel Source Microblock Stride (chid = 16)"]
    pub csus16: CSUS16,
    #[doc = "0x484 - Channel Destination Microblock Stride (chid = 16)"]
    pub cdus16: CDUS16,
    _reserved255: [u8; 8usize],
    #[doc = "0x490 - Channel Interrupt Enable Register (chid = 17)"]
    pub cie17: CIE17,
    #[doc = "0x494 - Channel Interrupt Disable Register (chid = 17)"]
    pub cid17: CID17,
    #[doc = "0x498 - Channel Interrupt Mask Register (chid = 17)"]
    pub cim17: CIM17,
    #[doc = "0x49c - Channel Interrupt Status Register (chid = 17)"]
    pub cis17: CIS17,
    #[doc = "0x4a0 - Channel Source Address Register (chid = 17)"]
    pub csa17: CSA17,
    #[doc = "0x4a4 - Channel Destination Address Register (chid = 17)"]
    pub cda17: CDA17,
    #[doc = "0x4a8 - Channel Next Descriptor Address Register (chid = 17)"]
    pub cnda17: CNDA17,
    #[doc = "0x4ac - Channel Next Descriptor Control Register (chid = 17)"]
    pub cndc17: CNDC17,
    #[doc = "0x4b0 - Channel Microblock Control Register (chid = 17)"]
    pub cubc17: CUBC17,
    #[doc = "0x4b4 - Channel Block Control Register (chid = 17)"]
    pub cbc17: CBC17,
    #[doc = "0x4b8 - Channel Configuration Register (chid = 17)"]
    pub cc17: CC17,
    #[doc = "0x4bc - Channel Data Stride Memory Set Pattern (chid = 17)"]
    pub cds_msp17: CDS_MSP17,
    #[doc = "0x4c0 - Channel Source Microblock Stride (chid = 17)"]
    pub csus17: CSUS17,
    #[doc = "0x4c4 - Channel Destination Microblock Stride (chid = 17)"]
    pub cdus17: CDUS17,
    _reserved269: [u8; 8usize],
    #[doc = "0x4d0 - Channel Interrupt Enable Register (chid = 18)"]
    pub cie18: CIE18,
    #[doc = "0x4d4 - Channel Interrupt Disable Register (chid = 18)"]
    pub cid18: CID18,
    #[doc = "0x4d8 - Channel Interrupt Mask Register (chid = 18)"]
    pub cim18: CIM18,
    #[doc = "0x4dc - Channel Interrupt Status Register (chid = 18)"]
    pub cis18: CIS18,
    #[doc = "0x4e0 - Channel Source Address Register (chid = 18)"]
    pub csa18: CSA18,
    #[doc = "0x4e4 - Channel Destination Address Register (chid = 18)"]
    pub cda18: CDA18,
    #[doc = "0x4e8 - Channel Next Descriptor Address Register (chid = 18)"]
    pub cnda18: CNDA18,
    #[doc = "0x4ec - Channel Next Descriptor Control Register (chid = 18)"]
    pub cndc18: CNDC18,
    #[doc = "0x4f0 - Channel Microblock Control Register (chid = 18)"]
    pub cubc18: CUBC18,
    #[doc = "0x4f4 - Channel Block Control Register (chid = 18)"]
    pub cbc18: CBC18,
    #[doc = "0x4f8 - Channel Configuration Register (chid = 18)"]
    pub cc18: CC18,
    #[doc = "0x4fc - Channel Data Stride Memory Set Pattern (chid = 18)"]
    pub cds_msp18: CDS_MSP18,
    #[doc = "0x500 - Channel Source Microblock Stride (chid = 18)"]
    pub csus18: CSUS18,
    #[doc = "0x504 - Channel Destination Microblock Stride (chid = 18)"]
    pub cdus18: CDUS18,
    _reserved283: [u8; 8usize],
    #[doc = "0x510 - Channel Interrupt Enable Register (chid = 19)"]
    pub cie19: CIE19,
    #[doc = "0x514 - Channel Interrupt Disable Register (chid = 19)"]
    pub cid19: CID19,
    #[doc = "0x518 - Channel Interrupt Mask Register (chid = 19)"]
    pub cim19: CIM19,
    #[doc = "0x51c - Channel Interrupt Status Register (chid = 19)"]
    pub cis19: CIS19,
    #[doc = "0x520 - Channel Source Address Register (chid = 19)"]
    pub csa19: CSA19,
    #[doc = "0x524 - Channel Destination Address Register (chid = 19)"]
    pub cda19: CDA19,
    #[doc = "0x528 - Channel Next Descriptor Address Register (chid = 19)"]
    pub cnda19: CNDA19,
    #[doc = "0x52c - Channel Next Descriptor Control Register (chid = 19)"]
    pub cndc19: CNDC19,
    #[doc = "0x530 - Channel Microblock Control Register (chid = 19)"]
    pub cubc19: CUBC19,
    #[doc = "0x534 - Channel Block Control Register (chid = 19)"]
    pub cbc19: CBC19,
    #[doc = "0x538 - Channel Configuration Register (chid = 19)"]
    pub cc19: CC19,
    #[doc = "0x53c - Channel Data Stride Memory Set Pattern (chid = 19)"]
    pub cds_msp19: CDS_MSP19,
    #[doc = "0x540 - Channel Source Microblock Stride (chid = 19)"]
    pub csus19: CSUS19,
    #[doc = "0x544 - Channel Destination Microblock Stride (chid = 19)"]
    pub cdus19: CDUS19,
    _reserved297: [u8; 8usize],
    #[doc = "0x550 - Channel Interrupt Enable Register (chid = 20)"]
    pub cie20: CIE20,
    #[doc = "0x554 - Channel Interrupt Disable Register (chid = 20)"]
    pub cid20: CID20,
    #[doc = "0x558 - Channel Interrupt Mask Register (chid = 20)"]
    pub cim20: CIM20,
    #[doc = "0x55c - Channel Interrupt Status Register (chid = 20)"]
    pub cis20: CIS20,
    #[doc = "0x560 - Channel Source Address Register (chid = 20)"]
    pub csa20: CSA20,
    #[doc = "0x564 - Channel Destination Address Register (chid = 20)"]
    pub cda20: CDA20,
    #[doc = "0x568 - Channel Next Descriptor Address Register (chid = 20)"]
    pub cnda20: CNDA20,
    #[doc = "0x56c - Channel Next Descriptor Control Register (chid = 20)"]
    pub cndc20: CNDC20,
    #[doc = "0x570 - Channel Microblock Control Register (chid = 20)"]
    pub cubc20: CUBC20,
    #[doc = "0x574 - Channel Block Control Register (chid = 20)"]
    pub cbc20: CBC20,
    #[doc = "0x578 - Channel Configuration Register (chid = 20)"]
    pub cc20: CC20,
    #[doc = "0x57c - Channel Data Stride Memory Set Pattern (chid = 20)"]
    pub cds_msp20: CDS_MSP20,
    #[doc = "0x580 - Channel Source Microblock Stride (chid = 20)"]
    pub csus20: CSUS20,
    #[doc = "0x584 - Channel Destination Microblock Stride (chid = 20)"]
    pub cdus20: CDUS20,
    _reserved311: [u8; 8usize],
    #[doc = "0x590 - Channel Interrupt Enable Register (chid = 21)"]
    pub cie21: CIE21,
    #[doc = "0x594 - Channel Interrupt Disable Register (chid = 21)"]
    pub cid21: CID21,
    #[doc = "0x598 - Channel Interrupt Mask Register (chid = 21)"]
    pub cim21: CIM21,
    #[doc = "0x59c - Channel Interrupt Status Register (chid = 21)"]
    pub cis21: CIS21,
    #[doc = "0x5a0 - Channel Source Address Register (chid = 21)"]
    pub csa21: CSA21,
    #[doc = "0x5a4 - Channel Destination Address Register (chid = 21)"]
    pub cda21: CDA21,
    #[doc = "0x5a8 - Channel Next Descriptor Address Register (chid = 21)"]
    pub cnda21: CNDA21,
    #[doc = "0x5ac - Channel Next Descriptor Control Register (chid = 21)"]
    pub cndc21: CNDC21,
    #[doc = "0x5b0 - Channel Microblock Control Register (chid = 21)"]
    pub cubc21: CUBC21,
    #[doc = "0x5b4 - Channel Block Control Register (chid = 21)"]
    pub cbc21: CBC21,
    #[doc = "0x5b8 - Channel Configuration Register (chid = 21)"]
    pub cc21: CC21,
    #[doc = "0x5bc - Channel Data Stride Memory Set Pattern (chid = 21)"]
    pub cds_msp21: CDS_MSP21,
    #[doc = "0x5c0 - Channel Source Microblock Stride (chid = 21)"]
    pub csus21: CSUS21,
    #[doc = "0x5c4 - Channel Destination Microblock Stride (chid = 21)"]
    pub cdus21: CDUS21,
    _reserved325: [u8; 8usize],
    #[doc = "0x5d0 - Channel Interrupt Enable Register (chid = 22)"]
    pub cie22: CIE22,
    #[doc = "0x5d4 - Channel Interrupt Disable Register (chid = 22)"]
    pub cid22: CID22,
    #[doc = "0x5d8 - Channel Interrupt Mask Register (chid = 22)"]
    pub cim22: CIM22,
    #[doc = "0x5dc - Channel Interrupt Status Register (chid = 22)"]
    pub cis22: CIS22,
    #[doc = "0x5e0 - Channel Source Address Register (chid = 22)"]
    pub csa22: CSA22,
    #[doc = "0x5e4 - Channel Destination Address Register (chid = 22)"]
    pub cda22: CDA22,
    #[doc = "0x5e8 - Channel Next Descriptor Address Register (chid = 22)"]
    pub cnda22: CNDA22,
    #[doc = "0x5ec - Channel Next Descriptor Control Register (chid = 22)"]
    pub cndc22: CNDC22,
    #[doc = "0x5f0 - Channel Microblock Control Register (chid = 22)"]
    pub cubc22: CUBC22,
    #[doc = "0x5f4 - Channel Block Control Register (chid = 22)"]
    pub cbc22: CBC22,
    #[doc = "0x5f8 - Channel Configuration Register (chid = 22)"]
    pub cc22: CC22,
    #[doc = "0x5fc - Channel Data Stride Memory Set Pattern (chid = 22)"]
    pub cds_msp22: CDS_MSP22,
    #[doc = "0x600 - Channel Source Microblock Stride (chid = 22)"]
    pub csus22: CSUS22,
    #[doc = "0x604 - Channel Destination Microblock Stride (chid = 22)"]
    pub cdus22: CDUS22,
    _reserved339: [u8; 8usize],
    #[doc = "0x610 - Channel Interrupt Enable Register (chid = 23)"]
    pub cie23: CIE23,
    #[doc = "0x614 - Channel Interrupt Disable Register (chid = 23)"]
    pub cid23: CID23,
    #[doc = "0x618 - Channel Interrupt Mask Register (chid = 23)"]
    pub cim23: CIM23,
    #[doc = "0x61c - Channel Interrupt Status Register (chid = 23)"]
    pub cis23: CIS23,
    #[doc = "0x620 - Channel Source Address Register (chid = 23)"]
    pub csa23: CSA23,
    #[doc = "0x624 - Channel Destination Address Register (chid = 23)"]
    pub cda23: CDA23,
    #[doc = "0x628 - Channel Next Descriptor Address Register (chid = 23)"]
    pub cnda23: CNDA23,
    #[doc = "0x62c - Channel Next Descriptor Control Register (chid = 23)"]
    pub cndc23: CNDC23,
    #[doc = "0x630 - Channel Microblock Control Register (chid = 23)"]
    pub cubc23: CUBC23,
    #[doc = "0x634 - Channel Block Control Register (chid = 23)"]
    pub cbc23: CBC23,
    #[doc = "0x638 - Channel Configuration Register (chid = 23)"]
    pub cc23: CC23,
    #[doc = "0x63c - Channel Data Stride Memory Set Pattern (chid = 23)"]
    pub cds_msp23: CDS_MSP23,
    #[doc = "0x640 - Channel Source Microblock Stride (chid = 23)"]
    pub csus23: CSUS23,
    #[doc = "0x644 - Channel Destination Microblock Stride (chid = 23)"]
    pub cdus23: CDUS23,
}
#[doc = "Global Type Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtype](gtype) module"]
pub type GTYPE = crate::Reg<u32, _GTYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GTYPE;
#[doc = "`read()` method returns [gtype::R](gtype::R) reader structure"]
impl crate::Readable for GTYPE {}
#[doc = "`write(|w| ..)` method takes [gtype::W](gtype::W) writer structure"]
impl crate::Writable for GTYPE {}
#[doc = "Global Type Register"]
pub mod gtype;
#[doc = "Global Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcfg](gcfg) module"]
pub type GCFG = crate::Reg<u32, _GCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCFG;
#[doc = "`read()` method returns [gcfg::R](gcfg::R) reader structure"]
impl crate::Readable for GCFG {}
#[doc = "Global Configuration Register"]
pub mod gcfg;
#[doc = "Global Weighted Arbiter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gwac](gwac) module"]
pub type GWAC = crate::Reg<u32, _GWAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GWAC;
#[doc = "`read()` method returns [gwac::R](gwac::R) reader structure"]
impl crate::Readable for GWAC {}
#[doc = "`write(|w| ..)` method takes [gwac::W](gwac::W) writer structure"]
impl crate::Writable for GWAC {}
#[doc = "Global Weighted Arbiter Configuration Register"]
pub mod gwac;
#[doc = "Global Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gie](gie) module"]
pub type GIE = crate::Reg<u32, _GIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GIE;
#[doc = "`write(|w| ..)` method takes [gie::W](gie::W) writer structure"]
impl crate::Writable for GIE {}
#[doc = "Global Interrupt Enable Register"]
pub mod gie;
#[doc = "Global Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gid](gid) module"]
pub type GID = crate::Reg<u32, _GID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GID;
#[doc = "`write(|w| ..)` method takes [gid::W](gid::W) writer structure"]
impl crate::Writable for GID {}
#[doc = "Global Interrupt Disable Register"]
pub mod gid;
#[doc = "Global Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gim](gim) module"]
pub type GIM = crate::Reg<u32, _GIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GIM;
#[doc = "`read()` method returns [gim::R](gim::R) reader structure"]
impl crate::Readable for GIM {}
#[doc = "Global Interrupt Mask Register"]
pub mod gim;
#[doc = "Global Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gis](gis) module"]
pub type GIS = crate::Reg<u32, _GIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GIS;
#[doc = "`read()` method returns [gis::R](gis::R) reader structure"]
impl crate::Readable for GIS {}
#[doc = "Global Interrupt Status Register"]
pub mod gis;
#[doc = "Global Channel Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ge](ge) module"]
pub type GE = crate::Reg<u32, _GE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GE;
#[doc = "`write(|w| ..)` method takes [ge::W](ge::W) writer structure"]
impl crate::Writable for GE {}
#[doc = "Global Channel Enable Register"]
pub mod ge;
#[doc = "Global Channel Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gd](gd) module"]
pub type GD = crate::Reg<u32, _GD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GD;
#[doc = "`write(|w| ..)` method takes [gd::W](gd::W) writer structure"]
impl crate::Writable for GD {}
#[doc = "Global Channel Disable Register"]
pub mod gd;
#[doc = "Global Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gs](gs) module"]
pub type GS = crate::Reg<u32, _GS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GS;
#[doc = "`read()` method returns [gs::R](gs::R) reader structure"]
impl crate::Readable for GS {}
#[doc = "Global Channel Status Register"]
pub mod gs;
#[doc = "Global Channel Read Suspend Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grs](grs) module"]
pub type GRS = crate::Reg<u32, _GRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRS;
#[doc = "`read()` method returns [grs::R](grs::R) reader structure"]
impl crate::Readable for GRS {}
#[doc = "`write(|w| ..)` method takes [grs::W](grs::W) writer structure"]
impl crate::Writable for GRS {}
#[doc = "Global Channel Read Suspend Register"]
pub mod grs;
#[doc = "Global Channel Write Suspend Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gws](gws) module"]
pub type GWS = crate::Reg<u32, _GWS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GWS;
#[doc = "`read()` method returns [gws::R](gws::R) reader structure"]
impl crate::Readable for GWS {}
#[doc = "`write(|w| ..)` method takes [gws::W](gws::W) writer structure"]
impl crate::Writable for GWS {}
#[doc = "Global Channel Write Suspend Register"]
pub mod gws;
#[doc = "Global Channel Read Write Suspend Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grws](grws) module"]
pub type GRWS = crate::Reg<u32, _GRWS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRWS;
#[doc = "`write(|w| ..)` method takes [grws::W](grws::W) writer structure"]
impl crate::Writable for GRWS {}
#[doc = "Global Channel Read Write Suspend Register"]
pub mod grws;
#[doc = "Global Channel Read Write Resume Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grwr](grwr) module"]
pub type GRWR = crate::Reg<u32, _GRWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRWR;
#[doc = "`write(|w| ..)` method takes [grwr::W](grwr::W) writer structure"]
impl crate::Writable for GRWR {}
#[doc = "Global Channel Read Write Resume Register"]
pub mod grwr;
#[doc = "Global Channel Software Request Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gswr](gswr) module"]
pub type GSWR = crate::Reg<u32, _GSWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GSWR;
#[doc = "`write(|w| ..)` method takes [gswr::W](gswr::W) writer structure"]
impl crate::Writable for GSWR {}
#[doc = "Global Channel Software Request Register"]
pub mod gswr;
#[doc = "Global Channel Software Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gsws](gsws) module"]
pub type GSWS = crate::Reg<u32, _GSWS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GSWS;
#[doc = "`read()` method returns [gsws::R](gsws::R) reader structure"]
impl crate::Readable for GSWS {}
#[doc = "Global Channel Software Request Status Register"]
pub mod gsws;
#[doc = "Global Channel Software Flush Request Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gswf](gswf) module"]
pub type GSWF = crate::Reg<u32, _GSWF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GSWF;
#[doc = "`write(|w| ..)` method takes [gswf::W](gswf::W) writer structure"]
impl crate::Writable for GSWF {}
#[doc = "Global Channel Software Flush Request Register"]
pub mod gswf;
#[doc = "Channel Interrupt Enable Register (chid = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie0](cie0) module"]
pub type CIE0 = crate::Reg<u32, _CIE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE0;
#[doc = "`write(|w| ..)` method takes [cie0::W](cie0::W) writer structure"]
impl crate::Writable for CIE0 {}
#[doc = "Channel Interrupt Enable Register (chid = 0)"]
pub mod cie0;
#[doc = "Channel Interrupt Disable Register (chid = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid0](cid0) module"]
pub type CID0 = crate::Reg<u32, _CID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID0;
#[doc = "`write(|w| ..)` method takes [cid0::W](cid0::W) writer structure"]
impl crate::Writable for CID0 {}
#[doc = "Channel Interrupt Disable Register (chid = 0)"]
pub mod cid0;
#[doc = "Channel Interrupt Mask Register (chid = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim0](cim0) module"]
pub type CIM0 = crate::Reg<u32, _CIM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM0;
#[doc = "`write(|w| ..)` method takes [cim0::W](cim0::W) writer structure"]
impl crate::Writable for CIM0 {}
#[doc = "Channel Interrupt Mask Register (chid = 0)"]
pub mod cim0;
#[doc = "Channel Interrupt Status Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis0](cis0) module"]
pub type CIS0 = crate::Reg<u32, _CIS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS0;
#[doc = "`read()` method returns [cis0::R](cis0::R) reader structure"]
impl crate::Readable for CIS0 {}
#[doc = "Channel Interrupt Status Register (chid = 0)"]
pub mod cis0;
#[doc = "Channel Source Address Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa0](csa0) module"]
pub type CSA0 = crate::Reg<u32, _CSA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA0;
#[doc = "`read()` method returns [csa0::R](csa0::R) reader structure"]
impl crate::Readable for CSA0 {}
#[doc = "`write(|w| ..)` method takes [csa0::W](csa0::W) writer structure"]
impl crate::Writable for CSA0 {}
#[doc = "Channel Source Address Register (chid = 0)"]
pub mod csa0;
#[doc = "Channel Destination Address Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda0](cda0) module"]
pub type CDA0 = crate::Reg<u32, _CDA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA0;
#[doc = "`read()` method returns [cda0::R](cda0::R) reader structure"]
impl crate::Readable for CDA0 {}
#[doc = "`write(|w| ..)` method takes [cda0::W](cda0::W) writer structure"]
impl crate::Writable for CDA0 {}
#[doc = "Channel Destination Address Register (chid = 0)"]
pub mod cda0;
#[doc = "Channel Next Descriptor Address Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda0](cnda0) module"]
pub type CNDA0 = crate::Reg<u32, _CNDA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA0;
#[doc = "`read()` method returns [cnda0::R](cnda0::R) reader structure"]
impl crate::Readable for CNDA0 {}
#[doc = "`write(|w| ..)` method takes [cnda0::W](cnda0::W) writer structure"]
impl crate::Writable for CNDA0 {}
#[doc = "Channel Next Descriptor Address Register (chid = 0)"]
pub mod cnda0;
#[doc = "Channel Next Descriptor Control Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc0](cndc0) module"]
pub type CNDC0 = crate::Reg<u32, _CNDC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC0;
#[doc = "`read()` method returns [cndc0::R](cndc0::R) reader structure"]
impl crate::Readable for CNDC0 {}
#[doc = "`write(|w| ..)` method takes [cndc0::W](cndc0::W) writer structure"]
impl crate::Writable for CNDC0 {}
#[doc = "Channel Next Descriptor Control Register (chid = 0)"]
pub mod cndc0;
#[doc = "Channel Microblock Control Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc0](cubc0) module"]
pub type CUBC0 = crate::Reg<u32, _CUBC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC0;
#[doc = "`read()` method returns [cubc0::R](cubc0::R) reader structure"]
impl crate::Readable for CUBC0 {}
#[doc = "`write(|w| ..)` method takes [cubc0::W](cubc0::W) writer structure"]
impl crate::Writable for CUBC0 {}
#[doc = "Channel Microblock Control Register (chid = 0)"]
pub mod cubc0;
#[doc = "Channel Block Control Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc0](cbc0) module"]
pub type CBC0 = crate::Reg<u32, _CBC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC0;
#[doc = "`read()` method returns [cbc0::R](cbc0::R) reader structure"]
impl crate::Readable for CBC0 {}
#[doc = "`write(|w| ..)` method takes [cbc0::W](cbc0::W) writer structure"]
impl crate::Writable for CBC0 {}
#[doc = "Channel Block Control Register (chid = 0)"]
pub mod cbc0;
#[doc = "Channel Configuration Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc0](cc0) module"]
pub type CC0 = crate::Reg<u32, _CC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC0;
#[doc = "`read()` method returns [cc0::R](cc0::R) reader structure"]
impl crate::Readable for CC0 {}
#[doc = "`write(|w| ..)` method takes [cc0::W](cc0::W) writer structure"]
impl crate::Writable for CC0 {}
#[doc = "Channel Configuration Register (chid = 0)"]
pub mod cc0;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp0](cds_msp0) module"]
pub type CDS_MSP0 = crate::Reg<u32, _CDS_MSP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP0;
#[doc = "`read()` method returns [cds_msp0::R](cds_msp0::R) reader structure"]
impl crate::Readable for CDS_MSP0 {}
#[doc = "`write(|w| ..)` method takes [cds_msp0::W](cds_msp0::W) writer structure"]
impl crate::Writable for CDS_MSP0 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 0)"]
pub mod cds_msp0;
#[doc = "Channel Source Microblock Stride (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus0](csus0) module"]
pub type CSUS0 = crate::Reg<u32, _CSUS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS0;
#[doc = "`read()` method returns [csus0::R](csus0::R) reader structure"]
impl crate::Readable for CSUS0 {}
#[doc = "`write(|w| ..)` method takes [csus0::W](csus0::W) writer structure"]
impl crate::Writable for CSUS0 {}
#[doc = "Channel Source Microblock Stride (chid = 0)"]
pub mod csus0;
#[doc = "Channel Destination Microblock Stride (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus0](cdus0) module"]
pub type CDUS0 = crate::Reg<u32, _CDUS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS0;
#[doc = "`read()` method returns [cdus0::R](cdus0::R) reader structure"]
impl crate::Readable for CDUS0 {}
#[doc = "`write(|w| ..)` method takes [cdus0::W](cdus0::W) writer structure"]
impl crate::Writable for CDUS0 {}
#[doc = "Channel Destination Microblock Stride (chid = 0)"]
pub mod cdus0;
#[doc = "Channel Interrupt Enable Register (chid = 1)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie1](cie1) module"]
pub type CIE1 = crate::Reg<u32, _CIE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE1;
#[doc = "`write(|w| ..)` method takes [cie1::W](cie1::W) writer structure"]
impl crate::Writable for CIE1 {}
#[doc = "Channel Interrupt Enable Register (chid = 1)"]
pub mod cie1;
#[doc = "Channel Interrupt Disable Register (chid = 1)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid1](cid1) module"]
pub type CID1 = crate::Reg<u32, _CID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID1;
#[doc = "`write(|w| ..)` method takes [cid1::W](cid1::W) writer structure"]
impl crate::Writable for CID1 {}
#[doc = "Channel Interrupt Disable Register (chid = 1)"]
pub mod cid1;
#[doc = "Channel Interrupt Mask Register (chid = 1)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim1](cim1) module"]
pub type CIM1 = crate::Reg<u32, _CIM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM1;
#[doc = "`write(|w| ..)` method takes [cim1::W](cim1::W) writer structure"]
impl crate::Writable for CIM1 {}
#[doc = "Channel Interrupt Mask Register (chid = 1)"]
pub mod cim1;
#[doc = "Channel Interrupt Status Register (chid = 1)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis1](cis1) module"]
pub type CIS1 = crate::Reg<u32, _CIS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS1;
#[doc = "`read()` method returns [cis1::R](cis1::R) reader structure"]
impl crate::Readable for CIS1 {}
#[doc = "Channel Interrupt Status Register (chid = 1)"]
pub mod cis1;
#[doc = "Channel Source Address Register (chid = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa1](csa1) module"]
pub type CSA1 = crate::Reg<u32, _CSA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA1;
#[doc = "`read()` method returns [csa1::R](csa1::R) reader structure"]
impl crate::Readable for CSA1 {}
#[doc = "`write(|w| ..)` method takes [csa1::W](csa1::W) writer structure"]
impl crate::Writable for CSA1 {}
#[doc = "Channel Source Address Register (chid = 1)"]
pub mod csa1;
#[doc = "Channel Destination Address Register (chid = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda1](cda1) module"]
pub type CDA1 = crate::Reg<u32, _CDA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA1;
#[doc = "`read()` method returns [cda1::R](cda1::R) reader structure"]
impl crate::Readable for CDA1 {}
#[doc = "`write(|w| ..)` method takes [cda1::W](cda1::W) writer structure"]
impl crate::Writable for CDA1 {}
#[doc = "Channel Destination Address Register (chid = 1)"]
pub mod cda1;
#[doc = "Channel Next Descriptor Address Register (chid = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda1](cnda1) module"]
pub type CNDA1 = crate::Reg<u32, _CNDA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA1;
#[doc = "`read()` method returns [cnda1::R](cnda1::R) reader structure"]
impl crate::Readable for CNDA1 {}
#[doc = "`write(|w| ..)` method takes [cnda1::W](cnda1::W) writer structure"]
impl crate::Writable for CNDA1 {}
#[doc = "Channel Next Descriptor Address Register (chid = 1)"]
pub mod cnda1;
#[doc = "Channel Next Descriptor Control Register (chid = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc1](cndc1) module"]
pub type CNDC1 = crate::Reg<u32, _CNDC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC1;
#[doc = "`read()` method returns [cndc1::R](cndc1::R) reader structure"]
impl crate::Readable for CNDC1 {}
#[doc = "`write(|w| ..)` method takes [cndc1::W](cndc1::W) writer structure"]
impl crate::Writable for CNDC1 {}
#[doc = "Channel Next Descriptor Control Register (chid = 1)"]
pub mod cndc1;
#[doc = "Channel Microblock Control Register (chid = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc1](cubc1) module"]
pub type CUBC1 = crate::Reg<u32, _CUBC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC1;
#[doc = "`read()` method returns [cubc1::R](cubc1::R) reader structure"]
impl crate::Readable for CUBC1 {}
#[doc = "`write(|w| ..)` method takes [cubc1::W](cubc1::W) writer structure"]
impl crate::Writable for CUBC1 {}
#[doc = "Channel Microblock Control Register (chid = 1)"]
pub mod cubc1;
#[doc = "Channel Block Control Register (chid = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc1](cbc1) module"]
pub type CBC1 = crate::Reg<u32, _CBC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC1;
#[doc = "`read()` method returns [cbc1::R](cbc1::R) reader structure"]
impl crate::Readable for CBC1 {}
#[doc = "`write(|w| ..)` method takes [cbc1::W](cbc1::W) writer structure"]
impl crate::Writable for CBC1 {}
#[doc = "Channel Block Control Register (chid = 1)"]
pub mod cbc1;
#[doc = "Channel Configuration Register (chid = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc1](cc1) module"]
pub type CC1 = crate::Reg<u32, _CC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC1;
#[doc = "`read()` method returns [cc1::R](cc1::R) reader structure"]
impl crate::Readable for CC1 {}
#[doc = "`write(|w| ..)` method takes [cc1::W](cc1::W) writer structure"]
impl crate::Writable for CC1 {}
#[doc = "Channel Configuration Register (chid = 1)"]
pub mod cc1;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp1](cds_msp1) module"]
pub type CDS_MSP1 = crate::Reg<u32, _CDS_MSP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP1;
#[doc = "`read()` method returns [cds_msp1::R](cds_msp1::R) reader structure"]
impl crate::Readable for CDS_MSP1 {}
#[doc = "`write(|w| ..)` method takes [cds_msp1::W](cds_msp1::W) writer structure"]
impl crate::Writable for CDS_MSP1 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 1)"]
pub mod cds_msp1;
#[doc = "Channel Source Microblock Stride (chid = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus1](csus1) module"]
pub type CSUS1 = crate::Reg<u32, _CSUS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS1;
#[doc = "`read()` method returns [csus1::R](csus1::R) reader structure"]
impl crate::Readable for CSUS1 {}
#[doc = "`write(|w| ..)` method takes [csus1::W](csus1::W) writer structure"]
impl crate::Writable for CSUS1 {}
#[doc = "Channel Source Microblock Stride (chid = 1)"]
pub mod csus1;
#[doc = "Channel Destination Microblock Stride (chid = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus1](cdus1) module"]
pub type CDUS1 = crate::Reg<u32, _CDUS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS1;
#[doc = "`read()` method returns [cdus1::R](cdus1::R) reader structure"]
impl crate::Readable for CDUS1 {}
#[doc = "`write(|w| ..)` method takes [cdus1::W](cdus1::W) writer structure"]
impl crate::Writable for CDUS1 {}
#[doc = "Channel Destination Microblock Stride (chid = 1)"]
pub mod cdus1;
#[doc = "Channel Interrupt Enable Register (chid = 2)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie2](cie2) module"]
pub type CIE2 = crate::Reg<u32, _CIE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE2;
#[doc = "`write(|w| ..)` method takes [cie2::W](cie2::W) writer structure"]
impl crate::Writable for CIE2 {}
#[doc = "Channel Interrupt Enable Register (chid = 2)"]
pub mod cie2;
#[doc = "Channel Interrupt Disable Register (chid = 2)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid2](cid2) module"]
pub type CID2 = crate::Reg<u32, _CID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID2;
#[doc = "`write(|w| ..)` method takes [cid2::W](cid2::W) writer structure"]
impl crate::Writable for CID2 {}
#[doc = "Channel Interrupt Disable Register (chid = 2)"]
pub mod cid2;
#[doc = "Channel Interrupt Mask Register (chid = 2)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim2](cim2) module"]
pub type CIM2 = crate::Reg<u32, _CIM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM2;
#[doc = "`write(|w| ..)` method takes [cim2::W](cim2::W) writer structure"]
impl crate::Writable for CIM2 {}
#[doc = "Channel Interrupt Mask Register (chid = 2)"]
pub mod cim2;
#[doc = "Channel Interrupt Status Register (chid = 2)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis2](cis2) module"]
pub type CIS2 = crate::Reg<u32, _CIS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS2;
#[doc = "`read()` method returns [cis2::R](cis2::R) reader structure"]
impl crate::Readable for CIS2 {}
#[doc = "Channel Interrupt Status Register (chid = 2)"]
pub mod cis2;
#[doc = "Channel Source Address Register (chid = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa2](csa2) module"]
pub type CSA2 = crate::Reg<u32, _CSA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA2;
#[doc = "`read()` method returns [csa2::R](csa2::R) reader structure"]
impl crate::Readable for CSA2 {}
#[doc = "`write(|w| ..)` method takes [csa2::W](csa2::W) writer structure"]
impl crate::Writable for CSA2 {}
#[doc = "Channel Source Address Register (chid = 2)"]
pub mod csa2;
#[doc = "Channel Destination Address Register (chid = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda2](cda2) module"]
pub type CDA2 = crate::Reg<u32, _CDA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA2;
#[doc = "`read()` method returns [cda2::R](cda2::R) reader structure"]
impl crate::Readable for CDA2 {}
#[doc = "`write(|w| ..)` method takes [cda2::W](cda2::W) writer structure"]
impl crate::Writable for CDA2 {}
#[doc = "Channel Destination Address Register (chid = 2)"]
pub mod cda2;
#[doc = "Channel Next Descriptor Address Register (chid = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda2](cnda2) module"]
pub type CNDA2 = crate::Reg<u32, _CNDA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA2;
#[doc = "`read()` method returns [cnda2::R](cnda2::R) reader structure"]
impl crate::Readable for CNDA2 {}
#[doc = "`write(|w| ..)` method takes [cnda2::W](cnda2::W) writer structure"]
impl crate::Writable for CNDA2 {}
#[doc = "Channel Next Descriptor Address Register (chid = 2)"]
pub mod cnda2;
#[doc = "Channel Next Descriptor Control Register (chid = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc2](cndc2) module"]
pub type CNDC2 = crate::Reg<u32, _CNDC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC2;
#[doc = "`read()` method returns [cndc2::R](cndc2::R) reader structure"]
impl crate::Readable for CNDC2 {}
#[doc = "`write(|w| ..)` method takes [cndc2::W](cndc2::W) writer structure"]
impl crate::Writable for CNDC2 {}
#[doc = "Channel Next Descriptor Control Register (chid = 2)"]
pub mod cndc2;
#[doc = "Channel Microblock Control Register (chid = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc2](cubc2) module"]
pub type CUBC2 = crate::Reg<u32, _CUBC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC2;
#[doc = "`read()` method returns [cubc2::R](cubc2::R) reader structure"]
impl crate::Readable for CUBC2 {}
#[doc = "`write(|w| ..)` method takes [cubc2::W](cubc2::W) writer structure"]
impl crate::Writable for CUBC2 {}
#[doc = "Channel Microblock Control Register (chid = 2)"]
pub mod cubc2;
#[doc = "Channel Block Control Register (chid = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc2](cbc2) module"]
pub type CBC2 = crate::Reg<u32, _CBC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC2;
#[doc = "`read()` method returns [cbc2::R](cbc2::R) reader structure"]
impl crate::Readable for CBC2 {}
#[doc = "`write(|w| ..)` method takes [cbc2::W](cbc2::W) writer structure"]
impl crate::Writable for CBC2 {}
#[doc = "Channel Block Control Register (chid = 2)"]
pub mod cbc2;
#[doc = "Channel Configuration Register (chid = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc2](cc2) module"]
pub type CC2 = crate::Reg<u32, _CC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC2;
#[doc = "`read()` method returns [cc2::R](cc2::R) reader structure"]
impl crate::Readable for CC2 {}
#[doc = "`write(|w| ..)` method takes [cc2::W](cc2::W) writer structure"]
impl crate::Writable for CC2 {}
#[doc = "Channel Configuration Register (chid = 2)"]
pub mod cc2;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp2](cds_msp2) module"]
pub type CDS_MSP2 = crate::Reg<u32, _CDS_MSP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP2;
#[doc = "`read()` method returns [cds_msp2::R](cds_msp2::R) reader structure"]
impl crate::Readable for CDS_MSP2 {}
#[doc = "`write(|w| ..)` method takes [cds_msp2::W](cds_msp2::W) writer structure"]
impl crate::Writable for CDS_MSP2 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 2)"]
pub mod cds_msp2;
#[doc = "Channel Source Microblock Stride (chid = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus2](csus2) module"]
pub type CSUS2 = crate::Reg<u32, _CSUS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS2;
#[doc = "`read()` method returns [csus2::R](csus2::R) reader structure"]
impl crate::Readable for CSUS2 {}
#[doc = "`write(|w| ..)` method takes [csus2::W](csus2::W) writer structure"]
impl crate::Writable for CSUS2 {}
#[doc = "Channel Source Microblock Stride (chid = 2)"]
pub mod csus2;
#[doc = "Channel Destination Microblock Stride (chid = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus2](cdus2) module"]
pub type CDUS2 = crate::Reg<u32, _CDUS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS2;
#[doc = "`read()` method returns [cdus2::R](cdus2::R) reader structure"]
impl crate::Readable for CDUS2 {}
#[doc = "`write(|w| ..)` method takes [cdus2::W](cdus2::W) writer structure"]
impl crate::Writable for CDUS2 {}
#[doc = "Channel Destination Microblock Stride (chid = 2)"]
pub mod cdus2;
#[doc = "Channel Interrupt Enable Register (chid = 3)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie3](cie3) module"]
pub type CIE3 = crate::Reg<u32, _CIE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE3;
#[doc = "`write(|w| ..)` method takes [cie3::W](cie3::W) writer structure"]
impl crate::Writable for CIE3 {}
#[doc = "Channel Interrupt Enable Register (chid = 3)"]
pub mod cie3;
#[doc = "Channel Interrupt Disable Register (chid = 3)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid3](cid3) module"]
pub type CID3 = crate::Reg<u32, _CID3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID3;
#[doc = "`write(|w| ..)` method takes [cid3::W](cid3::W) writer structure"]
impl crate::Writable for CID3 {}
#[doc = "Channel Interrupt Disable Register (chid = 3)"]
pub mod cid3;
#[doc = "Channel Interrupt Mask Register (chid = 3)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim3](cim3) module"]
pub type CIM3 = crate::Reg<u32, _CIM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM3;
#[doc = "`write(|w| ..)` method takes [cim3::W](cim3::W) writer structure"]
impl crate::Writable for CIM3 {}
#[doc = "Channel Interrupt Mask Register (chid = 3)"]
pub mod cim3;
#[doc = "Channel Interrupt Status Register (chid = 3)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis3](cis3) module"]
pub type CIS3 = crate::Reg<u32, _CIS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS3;
#[doc = "`read()` method returns [cis3::R](cis3::R) reader structure"]
impl crate::Readable for CIS3 {}
#[doc = "Channel Interrupt Status Register (chid = 3)"]
pub mod cis3;
#[doc = "Channel Source Address Register (chid = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa3](csa3) module"]
pub type CSA3 = crate::Reg<u32, _CSA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA3;
#[doc = "`read()` method returns [csa3::R](csa3::R) reader structure"]
impl crate::Readable for CSA3 {}
#[doc = "`write(|w| ..)` method takes [csa3::W](csa3::W) writer structure"]
impl crate::Writable for CSA3 {}
#[doc = "Channel Source Address Register (chid = 3)"]
pub mod csa3;
#[doc = "Channel Destination Address Register (chid = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda3](cda3) module"]
pub type CDA3 = crate::Reg<u32, _CDA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA3;
#[doc = "`read()` method returns [cda3::R](cda3::R) reader structure"]
impl crate::Readable for CDA3 {}
#[doc = "`write(|w| ..)` method takes [cda3::W](cda3::W) writer structure"]
impl crate::Writable for CDA3 {}
#[doc = "Channel Destination Address Register (chid = 3)"]
pub mod cda3;
#[doc = "Channel Next Descriptor Address Register (chid = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda3](cnda3) module"]
pub type CNDA3 = crate::Reg<u32, _CNDA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA3;
#[doc = "`read()` method returns [cnda3::R](cnda3::R) reader structure"]
impl crate::Readable for CNDA3 {}
#[doc = "`write(|w| ..)` method takes [cnda3::W](cnda3::W) writer structure"]
impl crate::Writable for CNDA3 {}
#[doc = "Channel Next Descriptor Address Register (chid = 3)"]
pub mod cnda3;
#[doc = "Channel Next Descriptor Control Register (chid = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc3](cndc3) module"]
pub type CNDC3 = crate::Reg<u32, _CNDC3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC3;
#[doc = "`read()` method returns [cndc3::R](cndc3::R) reader structure"]
impl crate::Readable for CNDC3 {}
#[doc = "`write(|w| ..)` method takes [cndc3::W](cndc3::W) writer structure"]
impl crate::Writable for CNDC3 {}
#[doc = "Channel Next Descriptor Control Register (chid = 3)"]
pub mod cndc3;
#[doc = "Channel Microblock Control Register (chid = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc3](cubc3) module"]
pub type CUBC3 = crate::Reg<u32, _CUBC3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC3;
#[doc = "`read()` method returns [cubc3::R](cubc3::R) reader structure"]
impl crate::Readable for CUBC3 {}
#[doc = "`write(|w| ..)` method takes [cubc3::W](cubc3::W) writer structure"]
impl crate::Writable for CUBC3 {}
#[doc = "Channel Microblock Control Register (chid = 3)"]
pub mod cubc3;
#[doc = "Channel Block Control Register (chid = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc3](cbc3) module"]
pub type CBC3 = crate::Reg<u32, _CBC3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC3;
#[doc = "`read()` method returns [cbc3::R](cbc3::R) reader structure"]
impl crate::Readable for CBC3 {}
#[doc = "`write(|w| ..)` method takes [cbc3::W](cbc3::W) writer structure"]
impl crate::Writable for CBC3 {}
#[doc = "Channel Block Control Register (chid = 3)"]
pub mod cbc3;
#[doc = "Channel Configuration Register (chid = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc3](cc3) module"]
pub type CC3 = crate::Reg<u32, _CC3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC3;
#[doc = "`read()` method returns [cc3::R](cc3::R) reader structure"]
impl crate::Readable for CC3 {}
#[doc = "`write(|w| ..)` method takes [cc3::W](cc3::W) writer structure"]
impl crate::Writable for CC3 {}
#[doc = "Channel Configuration Register (chid = 3)"]
pub mod cc3;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp3](cds_msp3) module"]
pub type CDS_MSP3 = crate::Reg<u32, _CDS_MSP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP3;
#[doc = "`read()` method returns [cds_msp3::R](cds_msp3::R) reader structure"]
impl crate::Readable for CDS_MSP3 {}
#[doc = "`write(|w| ..)` method takes [cds_msp3::W](cds_msp3::W) writer structure"]
impl crate::Writable for CDS_MSP3 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 3)"]
pub mod cds_msp3;
#[doc = "Channel Source Microblock Stride (chid = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus3](csus3) module"]
pub type CSUS3 = crate::Reg<u32, _CSUS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS3;
#[doc = "`read()` method returns [csus3::R](csus3::R) reader structure"]
impl crate::Readable for CSUS3 {}
#[doc = "`write(|w| ..)` method takes [csus3::W](csus3::W) writer structure"]
impl crate::Writable for CSUS3 {}
#[doc = "Channel Source Microblock Stride (chid = 3)"]
pub mod csus3;
#[doc = "Channel Destination Microblock Stride (chid = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus3](cdus3) module"]
pub type CDUS3 = crate::Reg<u32, _CDUS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS3;
#[doc = "`read()` method returns [cdus3::R](cdus3::R) reader structure"]
impl crate::Readable for CDUS3 {}
#[doc = "`write(|w| ..)` method takes [cdus3::W](cdus3::W) writer structure"]
impl crate::Writable for CDUS3 {}
#[doc = "Channel Destination Microblock Stride (chid = 3)"]
pub mod cdus3;
#[doc = "Channel Interrupt Enable Register (chid = 4)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie4](cie4) module"]
pub type CIE4 = crate::Reg<u32, _CIE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE4;
#[doc = "`write(|w| ..)` method takes [cie4::W](cie4::W) writer structure"]
impl crate::Writable for CIE4 {}
#[doc = "Channel Interrupt Enable Register (chid = 4)"]
pub mod cie4;
#[doc = "Channel Interrupt Disable Register (chid = 4)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid4](cid4) module"]
pub type CID4 = crate::Reg<u32, _CID4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID4;
#[doc = "`write(|w| ..)` method takes [cid4::W](cid4::W) writer structure"]
impl crate::Writable for CID4 {}
#[doc = "Channel Interrupt Disable Register (chid = 4)"]
pub mod cid4;
#[doc = "Channel Interrupt Mask Register (chid = 4)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim4](cim4) module"]
pub type CIM4 = crate::Reg<u32, _CIM4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM4;
#[doc = "`write(|w| ..)` method takes [cim4::W](cim4::W) writer structure"]
impl crate::Writable for CIM4 {}
#[doc = "Channel Interrupt Mask Register (chid = 4)"]
pub mod cim4;
#[doc = "Channel Interrupt Status Register (chid = 4)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis4](cis4) module"]
pub type CIS4 = crate::Reg<u32, _CIS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS4;
#[doc = "`read()` method returns [cis4::R](cis4::R) reader structure"]
impl crate::Readable for CIS4 {}
#[doc = "Channel Interrupt Status Register (chid = 4)"]
pub mod cis4;
#[doc = "Channel Source Address Register (chid = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa4](csa4) module"]
pub type CSA4 = crate::Reg<u32, _CSA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA4;
#[doc = "`read()` method returns [csa4::R](csa4::R) reader structure"]
impl crate::Readable for CSA4 {}
#[doc = "`write(|w| ..)` method takes [csa4::W](csa4::W) writer structure"]
impl crate::Writable for CSA4 {}
#[doc = "Channel Source Address Register (chid = 4)"]
pub mod csa4;
#[doc = "Channel Destination Address Register (chid = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda4](cda4) module"]
pub type CDA4 = crate::Reg<u32, _CDA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA4;
#[doc = "`read()` method returns [cda4::R](cda4::R) reader structure"]
impl crate::Readable for CDA4 {}
#[doc = "`write(|w| ..)` method takes [cda4::W](cda4::W) writer structure"]
impl crate::Writable for CDA4 {}
#[doc = "Channel Destination Address Register (chid = 4)"]
pub mod cda4;
#[doc = "Channel Next Descriptor Address Register (chid = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda4](cnda4) module"]
pub type CNDA4 = crate::Reg<u32, _CNDA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA4;
#[doc = "`read()` method returns [cnda4::R](cnda4::R) reader structure"]
impl crate::Readable for CNDA4 {}
#[doc = "`write(|w| ..)` method takes [cnda4::W](cnda4::W) writer structure"]
impl crate::Writable for CNDA4 {}
#[doc = "Channel Next Descriptor Address Register (chid = 4)"]
pub mod cnda4;
#[doc = "Channel Next Descriptor Control Register (chid = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc4](cndc4) module"]
pub type CNDC4 = crate::Reg<u32, _CNDC4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC4;
#[doc = "`read()` method returns [cndc4::R](cndc4::R) reader structure"]
impl crate::Readable for CNDC4 {}
#[doc = "`write(|w| ..)` method takes [cndc4::W](cndc4::W) writer structure"]
impl crate::Writable for CNDC4 {}
#[doc = "Channel Next Descriptor Control Register (chid = 4)"]
pub mod cndc4;
#[doc = "Channel Microblock Control Register (chid = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc4](cubc4) module"]
pub type CUBC4 = crate::Reg<u32, _CUBC4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC4;
#[doc = "`read()` method returns [cubc4::R](cubc4::R) reader structure"]
impl crate::Readable for CUBC4 {}
#[doc = "`write(|w| ..)` method takes [cubc4::W](cubc4::W) writer structure"]
impl crate::Writable for CUBC4 {}
#[doc = "Channel Microblock Control Register (chid = 4)"]
pub mod cubc4;
#[doc = "Channel Block Control Register (chid = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc4](cbc4) module"]
pub type CBC4 = crate::Reg<u32, _CBC4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC4;
#[doc = "`read()` method returns [cbc4::R](cbc4::R) reader structure"]
impl crate::Readable for CBC4 {}
#[doc = "`write(|w| ..)` method takes [cbc4::W](cbc4::W) writer structure"]
impl crate::Writable for CBC4 {}
#[doc = "Channel Block Control Register (chid = 4)"]
pub mod cbc4;
#[doc = "Channel Configuration Register (chid = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc4](cc4) module"]
pub type CC4 = crate::Reg<u32, _CC4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC4;
#[doc = "`read()` method returns [cc4::R](cc4::R) reader structure"]
impl crate::Readable for CC4 {}
#[doc = "`write(|w| ..)` method takes [cc4::W](cc4::W) writer structure"]
impl crate::Writable for CC4 {}
#[doc = "Channel Configuration Register (chid = 4)"]
pub mod cc4;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp4](cds_msp4) module"]
pub type CDS_MSP4 = crate::Reg<u32, _CDS_MSP4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP4;
#[doc = "`read()` method returns [cds_msp4::R](cds_msp4::R) reader structure"]
impl crate::Readable for CDS_MSP4 {}
#[doc = "`write(|w| ..)` method takes [cds_msp4::W](cds_msp4::W) writer structure"]
impl crate::Writable for CDS_MSP4 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 4)"]
pub mod cds_msp4;
#[doc = "Channel Source Microblock Stride (chid = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus4](csus4) module"]
pub type CSUS4 = crate::Reg<u32, _CSUS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS4;
#[doc = "`read()` method returns [csus4::R](csus4::R) reader structure"]
impl crate::Readable for CSUS4 {}
#[doc = "`write(|w| ..)` method takes [csus4::W](csus4::W) writer structure"]
impl crate::Writable for CSUS4 {}
#[doc = "Channel Source Microblock Stride (chid = 4)"]
pub mod csus4;
#[doc = "Channel Destination Microblock Stride (chid = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus4](cdus4) module"]
pub type CDUS4 = crate::Reg<u32, _CDUS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS4;
#[doc = "`read()` method returns [cdus4::R](cdus4::R) reader structure"]
impl crate::Readable for CDUS4 {}
#[doc = "`write(|w| ..)` method takes [cdus4::W](cdus4::W) writer structure"]
impl crate::Writable for CDUS4 {}
#[doc = "Channel Destination Microblock Stride (chid = 4)"]
pub mod cdus4;
#[doc = "Channel Interrupt Enable Register (chid = 5)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie5](cie5) module"]
pub type CIE5 = crate::Reg<u32, _CIE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE5;
#[doc = "`write(|w| ..)` method takes [cie5::W](cie5::W) writer structure"]
impl crate::Writable for CIE5 {}
#[doc = "Channel Interrupt Enable Register (chid = 5)"]
pub mod cie5;
#[doc = "Channel Interrupt Disable Register (chid = 5)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid5](cid5) module"]
pub type CID5 = crate::Reg<u32, _CID5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID5;
#[doc = "`write(|w| ..)` method takes [cid5::W](cid5::W) writer structure"]
impl crate::Writable for CID5 {}
#[doc = "Channel Interrupt Disable Register (chid = 5)"]
pub mod cid5;
#[doc = "Channel Interrupt Mask Register (chid = 5)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim5](cim5) module"]
pub type CIM5 = crate::Reg<u32, _CIM5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM5;
#[doc = "`write(|w| ..)` method takes [cim5::W](cim5::W) writer structure"]
impl crate::Writable for CIM5 {}
#[doc = "Channel Interrupt Mask Register (chid = 5)"]
pub mod cim5;
#[doc = "Channel Interrupt Status Register (chid = 5)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis5](cis5) module"]
pub type CIS5 = crate::Reg<u32, _CIS5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS5;
#[doc = "`read()` method returns [cis5::R](cis5::R) reader structure"]
impl crate::Readable for CIS5 {}
#[doc = "Channel Interrupt Status Register (chid = 5)"]
pub mod cis5;
#[doc = "Channel Source Address Register (chid = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa5](csa5) module"]
pub type CSA5 = crate::Reg<u32, _CSA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA5;
#[doc = "`read()` method returns [csa5::R](csa5::R) reader structure"]
impl crate::Readable for CSA5 {}
#[doc = "`write(|w| ..)` method takes [csa5::W](csa5::W) writer structure"]
impl crate::Writable for CSA5 {}
#[doc = "Channel Source Address Register (chid = 5)"]
pub mod csa5;
#[doc = "Channel Destination Address Register (chid = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda5](cda5) module"]
pub type CDA5 = crate::Reg<u32, _CDA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA5;
#[doc = "`read()` method returns [cda5::R](cda5::R) reader structure"]
impl crate::Readable for CDA5 {}
#[doc = "`write(|w| ..)` method takes [cda5::W](cda5::W) writer structure"]
impl crate::Writable for CDA5 {}
#[doc = "Channel Destination Address Register (chid = 5)"]
pub mod cda5;
#[doc = "Channel Next Descriptor Address Register (chid = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda5](cnda5) module"]
pub type CNDA5 = crate::Reg<u32, _CNDA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA5;
#[doc = "`read()` method returns [cnda5::R](cnda5::R) reader structure"]
impl crate::Readable for CNDA5 {}
#[doc = "`write(|w| ..)` method takes [cnda5::W](cnda5::W) writer structure"]
impl crate::Writable for CNDA5 {}
#[doc = "Channel Next Descriptor Address Register (chid = 5)"]
pub mod cnda5;
#[doc = "Channel Next Descriptor Control Register (chid = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc5](cndc5) module"]
pub type CNDC5 = crate::Reg<u32, _CNDC5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC5;
#[doc = "`read()` method returns [cndc5::R](cndc5::R) reader structure"]
impl crate::Readable for CNDC5 {}
#[doc = "`write(|w| ..)` method takes [cndc5::W](cndc5::W) writer structure"]
impl crate::Writable for CNDC5 {}
#[doc = "Channel Next Descriptor Control Register (chid = 5)"]
pub mod cndc5;
#[doc = "Channel Microblock Control Register (chid = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc5](cubc5) module"]
pub type CUBC5 = crate::Reg<u32, _CUBC5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC5;
#[doc = "`read()` method returns [cubc5::R](cubc5::R) reader structure"]
impl crate::Readable for CUBC5 {}
#[doc = "`write(|w| ..)` method takes [cubc5::W](cubc5::W) writer structure"]
impl crate::Writable for CUBC5 {}
#[doc = "Channel Microblock Control Register (chid = 5)"]
pub mod cubc5;
#[doc = "Channel Block Control Register (chid = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc5](cbc5) module"]
pub type CBC5 = crate::Reg<u32, _CBC5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC5;
#[doc = "`read()` method returns [cbc5::R](cbc5::R) reader structure"]
impl crate::Readable for CBC5 {}
#[doc = "`write(|w| ..)` method takes [cbc5::W](cbc5::W) writer structure"]
impl crate::Writable for CBC5 {}
#[doc = "Channel Block Control Register (chid = 5)"]
pub mod cbc5;
#[doc = "Channel Configuration Register (chid = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc5](cc5) module"]
pub type CC5 = crate::Reg<u32, _CC5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC5;
#[doc = "`read()` method returns [cc5::R](cc5::R) reader structure"]
impl crate::Readable for CC5 {}
#[doc = "`write(|w| ..)` method takes [cc5::W](cc5::W) writer structure"]
impl crate::Writable for CC5 {}
#[doc = "Channel Configuration Register (chid = 5)"]
pub mod cc5;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp5](cds_msp5) module"]
pub type CDS_MSP5 = crate::Reg<u32, _CDS_MSP5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP5;
#[doc = "`read()` method returns [cds_msp5::R](cds_msp5::R) reader structure"]
impl crate::Readable for CDS_MSP5 {}
#[doc = "`write(|w| ..)` method takes [cds_msp5::W](cds_msp5::W) writer structure"]
impl crate::Writable for CDS_MSP5 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 5)"]
pub mod cds_msp5;
#[doc = "Channel Source Microblock Stride (chid = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus5](csus5) module"]
pub type CSUS5 = crate::Reg<u32, _CSUS5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS5;
#[doc = "`read()` method returns [csus5::R](csus5::R) reader structure"]
impl crate::Readable for CSUS5 {}
#[doc = "`write(|w| ..)` method takes [csus5::W](csus5::W) writer structure"]
impl crate::Writable for CSUS5 {}
#[doc = "Channel Source Microblock Stride (chid = 5)"]
pub mod csus5;
#[doc = "Channel Destination Microblock Stride (chid = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus5](cdus5) module"]
pub type CDUS5 = crate::Reg<u32, _CDUS5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS5;
#[doc = "`read()` method returns [cdus5::R](cdus5::R) reader structure"]
impl crate::Readable for CDUS5 {}
#[doc = "`write(|w| ..)` method takes [cdus5::W](cdus5::W) writer structure"]
impl crate::Writable for CDUS5 {}
#[doc = "Channel Destination Microblock Stride (chid = 5)"]
pub mod cdus5;
#[doc = "Channel Interrupt Enable Register (chid = 6)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie6](cie6) module"]
pub type CIE6 = crate::Reg<u32, _CIE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE6;
#[doc = "`write(|w| ..)` method takes [cie6::W](cie6::W) writer structure"]
impl crate::Writable for CIE6 {}
#[doc = "Channel Interrupt Enable Register (chid = 6)"]
pub mod cie6;
#[doc = "Channel Interrupt Disable Register (chid = 6)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid6](cid6) module"]
pub type CID6 = crate::Reg<u32, _CID6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID6;
#[doc = "`write(|w| ..)` method takes [cid6::W](cid6::W) writer structure"]
impl crate::Writable for CID6 {}
#[doc = "Channel Interrupt Disable Register (chid = 6)"]
pub mod cid6;
#[doc = "Channel Interrupt Mask Register (chid = 6)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim6](cim6) module"]
pub type CIM6 = crate::Reg<u32, _CIM6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM6;
#[doc = "`write(|w| ..)` method takes [cim6::W](cim6::W) writer structure"]
impl crate::Writable for CIM6 {}
#[doc = "Channel Interrupt Mask Register (chid = 6)"]
pub mod cim6;
#[doc = "Channel Interrupt Status Register (chid = 6)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis6](cis6) module"]
pub type CIS6 = crate::Reg<u32, _CIS6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS6;
#[doc = "`read()` method returns [cis6::R](cis6::R) reader structure"]
impl crate::Readable for CIS6 {}
#[doc = "Channel Interrupt Status Register (chid = 6)"]
pub mod cis6;
#[doc = "Channel Source Address Register (chid = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa6](csa6) module"]
pub type CSA6 = crate::Reg<u32, _CSA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA6;
#[doc = "`read()` method returns [csa6::R](csa6::R) reader structure"]
impl crate::Readable for CSA6 {}
#[doc = "`write(|w| ..)` method takes [csa6::W](csa6::W) writer structure"]
impl crate::Writable for CSA6 {}
#[doc = "Channel Source Address Register (chid = 6)"]
pub mod csa6;
#[doc = "Channel Destination Address Register (chid = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda6](cda6) module"]
pub type CDA6 = crate::Reg<u32, _CDA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA6;
#[doc = "`read()` method returns [cda6::R](cda6::R) reader structure"]
impl crate::Readable for CDA6 {}
#[doc = "`write(|w| ..)` method takes [cda6::W](cda6::W) writer structure"]
impl crate::Writable for CDA6 {}
#[doc = "Channel Destination Address Register (chid = 6)"]
pub mod cda6;
#[doc = "Channel Next Descriptor Address Register (chid = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda6](cnda6) module"]
pub type CNDA6 = crate::Reg<u32, _CNDA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA6;
#[doc = "`read()` method returns [cnda6::R](cnda6::R) reader structure"]
impl crate::Readable for CNDA6 {}
#[doc = "`write(|w| ..)` method takes [cnda6::W](cnda6::W) writer structure"]
impl crate::Writable for CNDA6 {}
#[doc = "Channel Next Descriptor Address Register (chid = 6)"]
pub mod cnda6;
#[doc = "Channel Next Descriptor Control Register (chid = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc6](cndc6) module"]
pub type CNDC6 = crate::Reg<u32, _CNDC6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC6;
#[doc = "`read()` method returns [cndc6::R](cndc6::R) reader structure"]
impl crate::Readable for CNDC6 {}
#[doc = "`write(|w| ..)` method takes [cndc6::W](cndc6::W) writer structure"]
impl crate::Writable for CNDC6 {}
#[doc = "Channel Next Descriptor Control Register (chid = 6)"]
pub mod cndc6;
#[doc = "Channel Microblock Control Register (chid = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc6](cubc6) module"]
pub type CUBC6 = crate::Reg<u32, _CUBC6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC6;
#[doc = "`read()` method returns [cubc6::R](cubc6::R) reader structure"]
impl crate::Readable for CUBC6 {}
#[doc = "`write(|w| ..)` method takes [cubc6::W](cubc6::W) writer structure"]
impl crate::Writable for CUBC6 {}
#[doc = "Channel Microblock Control Register (chid = 6)"]
pub mod cubc6;
#[doc = "Channel Block Control Register (chid = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc6](cbc6) module"]
pub type CBC6 = crate::Reg<u32, _CBC6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC6;
#[doc = "`read()` method returns [cbc6::R](cbc6::R) reader structure"]
impl crate::Readable for CBC6 {}
#[doc = "`write(|w| ..)` method takes [cbc6::W](cbc6::W) writer structure"]
impl crate::Writable for CBC6 {}
#[doc = "Channel Block Control Register (chid = 6)"]
pub mod cbc6;
#[doc = "Channel Configuration Register (chid = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc6](cc6) module"]
pub type CC6 = crate::Reg<u32, _CC6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC6;
#[doc = "`read()` method returns [cc6::R](cc6::R) reader structure"]
impl crate::Readable for CC6 {}
#[doc = "`write(|w| ..)` method takes [cc6::W](cc6::W) writer structure"]
impl crate::Writable for CC6 {}
#[doc = "Channel Configuration Register (chid = 6)"]
pub mod cc6;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp6](cds_msp6) module"]
pub type CDS_MSP6 = crate::Reg<u32, _CDS_MSP6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP6;
#[doc = "`read()` method returns [cds_msp6::R](cds_msp6::R) reader structure"]
impl crate::Readable for CDS_MSP6 {}
#[doc = "`write(|w| ..)` method takes [cds_msp6::W](cds_msp6::W) writer structure"]
impl crate::Writable for CDS_MSP6 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 6)"]
pub mod cds_msp6;
#[doc = "Channel Source Microblock Stride (chid = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus6](csus6) module"]
pub type CSUS6 = crate::Reg<u32, _CSUS6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS6;
#[doc = "`read()` method returns [csus6::R](csus6::R) reader structure"]
impl crate::Readable for CSUS6 {}
#[doc = "`write(|w| ..)` method takes [csus6::W](csus6::W) writer structure"]
impl crate::Writable for CSUS6 {}
#[doc = "Channel Source Microblock Stride (chid = 6)"]
pub mod csus6;
#[doc = "Channel Destination Microblock Stride (chid = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus6](cdus6) module"]
pub type CDUS6 = crate::Reg<u32, _CDUS6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS6;
#[doc = "`read()` method returns [cdus6::R](cdus6::R) reader structure"]
impl crate::Readable for CDUS6 {}
#[doc = "`write(|w| ..)` method takes [cdus6::W](cdus6::W) writer structure"]
impl crate::Writable for CDUS6 {}
#[doc = "Channel Destination Microblock Stride (chid = 6)"]
pub mod cdus6;
#[doc = "Channel Interrupt Enable Register (chid = 7)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie7](cie7) module"]
pub type CIE7 = crate::Reg<u32, _CIE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE7;
#[doc = "`write(|w| ..)` method takes [cie7::W](cie7::W) writer structure"]
impl crate::Writable for CIE7 {}
#[doc = "Channel Interrupt Enable Register (chid = 7)"]
pub mod cie7;
#[doc = "Channel Interrupt Disable Register (chid = 7)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid7](cid7) module"]
pub type CID7 = crate::Reg<u32, _CID7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID7;
#[doc = "`write(|w| ..)` method takes [cid7::W](cid7::W) writer structure"]
impl crate::Writable for CID7 {}
#[doc = "Channel Interrupt Disable Register (chid = 7)"]
pub mod cid7;
#[doc = "Channel Interrupt Mask Register (chid = 7)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim7](cim7) module"]
pub type CIM7 = crate::Reg<u32, _CIM7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM7;
#[doc = "`write(|w| ..)` method takes [cim7::W](cim7::W) writer structure"]
impl crate::Writable for CIM7 {}
#[doc = "Channel Interrupt Mask Register (chid = 7)"]
pub mod cim7;
#[doc = "Channel Interrupt Status Register (chid = 7)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis7](cis7) module"]
pub type CIS7 = crate::Reg<u32, _CIS7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS7;
#[doc = "`read()` method returns [cis7::R](cis7::R) reader structure"]
impl crate::Readable for CIS7 {}
#[doc = "Channel Interrupt Status Register (chid = 7)"]
pub mod cis7;
#[doc = "Channel Source Address Register (chid = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa7](csa7) module"]
pub type CSA7 = crate::Reg<u32, _CSA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA7;
#[doc = "`read()` method returns [csa7::R](csa7::R) reader structure"]
impl crate::Readable for CSA7 {}
#[doc = "`write(|w| ..)` method takes [csa7::W](csa7::W) writer structure"]
impl crate::Writable for CSA7 {}
#[doc = "Channel Source Address Register (chid = 7)"]
pub mod csa7;
#[doc = "Channel Destination Address Register (chid = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda7](cda7) module"]
pub type CDA7 = crate::Reg<u32, _CDA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA7;
#[doc = "`read()` method returns [cda7::R](cda7::R) reader structure"]
impl crate::Readable for CDA7 {}
#[doc = "`write(|w| ..)` method takes [cda7::W](cda7::W) writer structure"]
impl crate::Writable for CDA7 {}
#[doc = "Channel Destination Address Register (chid = 7)"]
pub mod cda7;
#[doc = "Channel Next Descriptor Address Register (chid = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda7](cnda7) module"]
pub type CNDA7 = crate::Reg<u32, _CNDA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA7;
#[doc = "`read()` method returns [cnda7::R](cnda7::R) reader structure"]
impl crate::Readable for CNDA7 {}
#[doc = "`write(|w| ..)` method takes [cnda7::W](cnda7::W) writer structure"]
impl crate::Writable for CNDA7 {}
#[doc = "Channel Next Descriptor Address Register (chid = 7)"]
pub mod cnda7;
#[doc = "Channel Next Descriptor Control Register (chid = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc7](cndc7) module"]
pub type CNDC7 = crate::Reg<u32, _CNDC7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC7;
#[doc = "`read()` method returns [cndc7::R](cndc7::R) reader structure"]
impl crate::Readable for CNDC7 {}
#[doc = "`write(|w| ..)` method takes [cndc7::W](cndc7::W) writer structure"]
impl crate::Writable for CNDC7 {}
#[doc = "Channel Next Descriptor Control Register (chid = 7)"]
pub mod cndc7;
#[doc = "Channel Microblock Control Register (chid = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc7](cubc7) module"]
pub type CUBC7 = crate::Reg<u32, _CUBC7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC7;
#[doc = "`read()` method returns [cubc7::R](cubc7::R) reader structure"]
impl crate::Readable for CUBC7 {}
#[doc = "`write(|w| ..)` method takes [cubc7::W](cubc7::W) writer structure"]
impl crate::Writable for CUBC7 {}
#[doc = "Channel Microblock Control Register (chid = 7)"]
pub mod cubc7;
#[doc = "Channel Block Control Register (chid = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc7](cbc7) module"]
pub type CBC7 = crate::Reg<u32, _CBC7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC7;
#[doc = "`read()` method returns [cbc7::R](cbc7::R) reader structure"]
impl crate::Readable for CBC7 {}
#[doc = "`write(|w| ..)` method takes [cbc7::W](cbc7::W) writer structure"]
impl crate::Writable for CBC7 {}
#[doc = "Channel Block Control Register (chid = 7)"]
pub mod cbc7;
#[doc = "Channel Configuration Register (chid = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc7](cc7) module"]
pub type CC7 = crate::Reg<u32, _CC7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC7;
#[doc = "`read()` method returns [cc7::R](cc7::R) reader structure"]
impl crate::Readable for CC7 {}
#[doc = "`write(|w| ..)` method takes [cc7::W](cc7::W) writer structure"]
impl crate::Writable for CC7 {}
#[doc = "Channel Configuration Register (chid = 7)"]
pub mod cc7;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp7](cds_msp7) module"]
pub type CDS_MSP7 = crate::Reg<u32, _CDS_MSP7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP7;
#[doc = "`read()` method returns [cds_msp7::R](cds_msp7::R) reader structure"]
impl crate::Readable for CDS_MSP7 {}
#[doc = "`write(|w| ..)` method takes [cds_msp7::W](cds_msp7::W) writer structure"]
impl crate::Writable for CDS_MSP7 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 7)"]
pub mod cds_msp7;
#[doc = "Channel Source Microblock Stride (chid = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus7](csus7) module"]
pub type CSUS7 = crate::Reg<u32, _CSUS7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS7;
#[doc = "`read()` method returns [csus7::R](csus7::R) reader structure"]
impl crate::Readable for CSUS7 {}
#[doc = "`write(|w| ..)` method takes [csus7::W](csus7::W) writer structure"]
impl crate::Writable for CSUS7 {}
#[doc = "Channel Source Microblock Stride (chid = 7)"]
pub mod csus7;
#[doc = "Channel Destination Microblock Stride (chid = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus7](cdus7) module"]
pub type CDUS7 = crate::Reg<u32, _CDUS7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS7;
#[doc = "`read()` method returns [cdus7::R](cdus7::R) reader structure"]
impl crate::Readable for CDUS7 {}
#[doc = "`write(|w| ..)` method takes [cdus7::W](cdus7::W) writer structure"]
impl crate::Writable for CDUS7 {}
#[doc = "Channel Destination Microblock Stride (chid = 7)"]
pub mod cdus7;
#[doc = "Channel Interrupt Enable Register (chid = 8)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie8](cie8) module"]
pub type CIE8 = crate::Reg<u32, _CIE8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE8;
#[doc = "`write(|w| ..)` method takes [cie8::W](cie8::W) writer structure"]
impl crate::Writable for CIE8 {}
#[doc = "Channel Interrupt Enable Register (chid = 8)"]
pub mod cie8;
#[doc = "Channel Interrupt Disable Register (chid = 8)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid8](cid8) module"]
pub type CID8 = crate::Reg<u32, _CID8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID8;
#[doc = "`write(|w| ..)` method takes [cid8::W](cid8::W) writer structure"]
impl crate::Writable for CID8 {}
#[doc = "Channel Interrupt Disable Register (chid = 8)"]
pub mod cid8;
#[doc = "Channel Interrupt Mask Register (chid = 8)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim8](cim8) module"]
pub type CIM8 = crate::Reg<u32, _CIM8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM8;
#[doc = "`write(|w| ..)` method takes [cim8::W](cim8::W) writer structure"]
impl crate::Writable for CIM8 {}
#[doc = "Channel Interrupt Mask Register (chid = 8)"]
pub mod cim8;
#[doc = "Channel Interrupt Status Register (chid = 8)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis8](cis8) module"]
pub type CIS8 = crate::Reg<u32, _CIS8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS8;
#[doc = "`read()` method returns [cis8::R](cis8::R) reader structure"]
impl crate::Readable for CIS8 {}
#[doc = "Channel Interrupt Status Register (chid = 8)"]
pub mod cis8;
#[doc = "Channel Source Address Register (chid = 8)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa8](csa8) module"]
pub type CSA8 = crate::Reg<u32, _CSA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA8;
#[doc = "`read()` method returns [csa8::R](csa8::R) reader structure"]
impl crate::Readable for CSA8 {}
#[doc = "`write(|w| ..)` method takes [csa8::W](csa8::W) writer structure"]
impl crate::Writable for CSA8 {}
#[doc = "Channel Source Address Register (chid = 8)"]
pub mod csa8;
#[doc = "Channel Destination Address Register (chid = 8)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda8](cda8) module"]
pub type CDA8 = crate::Reg<u32, _CDA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA8;
#[doc = "`read()` method returns [cda8::R](cda8::R) reader structure"]
impl crate::Readable for CDA8 {}
#[doc = "`write(|w| ..)` method takes [cda8::W](cda8::W) writer structure"]
impl crate::Writable for CDA8 {}
#[doc = "Channel Destination Address Register (chid = 8)"]
pub mod cda8;
#[doc = "Channel Next Descriptor Address Register (chid = 8)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda8](cnda8) module"]
pub type CNDA8 = crate::Reg<u32, _CNDA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA8;
#[doc = "`read()` method returns [cnda8::R](cnda8::R) reader structure"]
impl crate::Readable for CNDA8 {}
#[doc = "`write(|w| ..)` method takes [cnda8::W](cnda8::W) writer structure"]
impl crate::Writable for CNDA8 {}
#[doc = "Channel Next Descriptor Address Register (chid = 8)"]
pub mod cnda8;
#[doc = "Channel Next Descriptor Control Register (chid = 8)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc8](cndc8) module"]
pub type CNDC8 = crate::Reg<u32, _CNDC8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC8;
#[doc = "`read()` method returns [cndc8::R](cndc8::R) reader structure"]
impl crate::Readable for CNDC8 {}
#[doc = "`write(|w| ..)` method takes [cndc8::W](cndc8::W) writer structure"]
impl crate::Writable for CNDC8 {}
#[doc = "Channel Next Descriptor Control Register (chid = 8)"]
pub mod cndc8;
#[doc = "Channel Microblock Control Register (chid = 8)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc8](cubc8) module"]
pub type CUBC8 = crate::Reg<u32, _CUBC8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC8;
#[doc = "`read()` method returns [cubc8::R](cubc8::R) reader structure"]
impl crate::Readable for CUBC8 {}
#[doc = "`write(|w| ..)` method takes [cubc8::W](cubc8::W) writer structure"]
impl crate::Writable for CUBC8 {}
#[doc = "Channel Microblock Control Register (chid = 8)"]
pub mod cubc8;
#[doc = "Channel Block Control Register (chid = 8)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc8](cbc8) module"]
pub type CBC8 = crate::Reg<u32, _CBC8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC8;
#[doc = "`read()` method returns [cbc8::R](cbc8::R) reader structure"]
impl crate::Readable for CBC8 {}
#[doc = "`write(|w| ..)` method takes [cbc8::W](cbc8::W) writer structure"]
impl crate::Writable for CBC8 {}
#[doc = "Channel Block Control Register (chid = 8)"]
pub mod cbc8;
#[doc = "Channel Configuration Register (chid = 8)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc8](cc8) module"]
pub type CC8 = crate::Reg<u32, _CC8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC8;
#[doc = "`read()` method returns [cc8::R](cc8::R) reader structure"]
impl crate::Readable for CC8 {}
#[doc = "`write(|w| ..)` method takes [cc8::W](cc8::W) writer structure"]
impl crate::Writable for CC8 {}
#[doc = "Channel Configuration Register (chid = 8)"]
pub mod cc8;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 8)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp8](cds_msp8) module"]
pub type CDS_MSP8 = crate::Reg<u32, _CDS_MSP8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP8;
#[doc = "`read()` method returns [cds_msp8::R](cds_msp8::R) reader structure"]
impl crate::Readable for CDS_MSP8 {}
#[doc = "`write(|w| ..)` method takes [cds_msp8::W](cds_msp8::W) writer structure"]
impl crate::Writable for CDS_MSP8 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 8)"]
pub mod cds_msp8;
#[doc = "Channel Source Microblock Stride (chid = 8)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus8](csus8) module"]
pub type CSUS8 = crate::Reg<u32, _CSUS8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS8;
#[doc = "`read()` method returns [csus8::R](csus8::R) reader structure"]
impl crate::Readable for CSUS8 {}
#[doc = "`write(|w| ..)` method takes [csus8::W](csus8::W) writer structure"]
impl crate::Writable for CSUS8 {}
#[doc = "Channel Source Microblock Stride (chid = 8)"]
pub mod csus8;
#[doc = "Channel Destination Microblock Stride (chid = 8)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus8](cdus8) module"]
pub type CDUS8 = crate::Reg<u32, _CDUS8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS8;
#[doc = "`read()` method returns [cdus8::R](cdus8::R) reader structure"]
impl crate::Readable for CDUS8 {}
#[doc = "`write(|w| ..)` method takes [cdus8::W](cdus8::W) writer structure"]
impl crate::Writable for CDUS8 {}
#[doc = "Channel Destination Microblock Stride (chid = 8)"]
pub mod cdus8;
#[doc = "Channel Interrupt Enable Register (chid = 9)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie9](cie9) module"]
pub type CIE9 = crate::Reg<u32, _CIE9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE9;
#[doc = "`write(|w| ..)` method takes [cie9::W](cie9::W) writer structure"]
impl crate::Writable for CIE9 {}
#[doc = "Channel Interrupt Enable Register (chid = 9)"]
pub mod cie9;
#[doc = "Channel Interrupt Disable Register (chid = 9)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid9](cid9) module"]
pub type CID9 = crate::Reg<u32, _CID9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID9;
#[doc = "`write(|w| ..)` method takes [cid9::W](cid9::W) writer structure"]
impl crate::Writable for CID9 {}
#[doc = "Channel Interrupt Disable Register (chid = 9)"]
pub mod cid9;
#[doc = "Channel Interrupt Mask Register (chid = 9)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim9](cim9) module"]
pub type CIM9 = crate::Reg<u32, _CIM9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM9;
#[doc = "`write(|w| ..)` method takes [cim9::W](cim9::W) writer structure"]
impl crate::Writable for CIM9 {}
#[doc = "Channel Interrupt Mask Register (chid = 9)"]
pub mod cim9;
#[doc = "Channel Interrupt Status Register (chid = 9)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis9](cis9) module"]
pub type CIS9 = crate::Reg<u32, _CIS9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS9;
#[doc = "`read()` method returns [cis9::R](cis9::R) reader structure"]
impl crate::Readable for CIS9 {}
#[doc = "Channel Interrupt Status Register (chid = 9)"]
pub mod cis9;
#[doc = "Channel Source Address Register (chid = 9)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa9](csa9) module"]
pub type CSA9 = crate::Reg<u32, _CSA9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA9;
#[doc = "`read()` method returns [csa9::R](csa9::R) reader structure"]
impl crate::Readable for CSA9 {}
#[doc = "`write(|w| ..)` method takes [csa9::W](csa9::W) writer structure"]
impl crate::Writable for CSA9 {}
#[doc = "Channel Source Address Register (chid = 9)"]
pub mod csa9;
#[doc = "Channel Destination Address Register (chid = 9)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda9](cda9) module"]
pub type CDA9 = crate::Reg<u32, _CDA9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA9;
#[doc = "`read()` method returns [cda9::R](cda9::R) reader structure"]
impl crate::Readable for CDA9 {}
#[doc = "`write(|w| ..)` method takes [cda9::W](cda9::W) writer structure"]
impl crate::Writable for CDA9 {}
#[doc = "Channel Destination Address Register (chid = 9)"]
pub mod cda9;
#[doc = "Channel Next Descriptor Address Register (chid = 9)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda9](cnda9) module"]
pub type CNDA9 = crate::Reg<u32, _CNDA9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA9;
#[doc = "`read()` method returns [cnda9::R](cnda9::R) reader structure"]
impl crate::Readable for CNDA9 {}
#[doc = "`write(|w| ..)` method takes [cnda9::W](cnda9::W) writer structure"]
impl crate::Writable for CNDA9 {}
#[doc = "Channel Next Descriptor Address Register (chid = 9)"]
pub mod cnda9;
#[doc = "Channel Next Descriptor Control Register (chid = 9)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc9](cndc9) module"]
pub type CNDC9 = crate::Reg<u32, _CNDC9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC9;
#[doc = "`read()` method returns [cndc9::R](cndc9::R) reader structure"]
impl crate::Readable for CNDC9 {}
#[doc = "`write(|w| ..)` method takes [cndc9::W](cndc9::W) writer structure"]
impl crate::Writable for CNDC9 {}
#[doc = "Channel Next Descriptor Control Register (chid = 9)"]
pub mod cndc9;
#[doc = "Channel Microblock Control Register (chid = 9)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc9](cubc9) module"]
pub type CUBC9 = crate::Reg<u32, _CUBC9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC9;
#[doc = "`read()` method returns [cubc9::R](cubc9::R) reader structure"]
impl crate::Readable for CUBC9 {}
#[doc = "`write(|w| ..)` method takes [cubc9::W](cubc9::W) writer structure"]
impl crate::Writable for CUBC9 {}
#[doc = "Channel Microblock Control Register (chid = 9)"]
pub mod cubc9;
#[doc = "Channel Block Control Register (chid = 9)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc9](cbc9) module"]
pub type CBC9 = crate::Reg<u32, _CBC9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC9;
#[doc = "`read()` method returns [cbc9::R](cbc9::R) reader structure"]
impl crate::Readable for CBC9 {}
#[doc = "`write(|w| ..)` method takes [cbc9::W](cbc9::W) writer structure"]
impl crate::Writable for CBC9 {}
#[doc = "Channel Block Control Register (chid = 9)"]
pub mod cbc9;
#[doc = "Channel Configuration Register (chid = 9)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc9](cc9) module"]
pub type CC9 = crate::Reg<u32, _CC9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC9;
#[doc = "`read()` method returns [cc9::R](cc9::R) reader structure"]
impl crate::Readable for CC9 {}
#[doc = "`write(|w| ..)` method takes [cc9::W](cc9::W) writer structure"]
impl crate::Writable for CC9 {}
#[doc = "Channel Configuration Register (chid = 9)"]
pub mod cc9;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 9)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp9](cds_msp9) module"]
pub type CDS_MSP9 = crate::Reg<u32, _CDS_MSP9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP9;
#[doc = "`read()` method returns [cds_msp9::R](cds_msp9::R) reader structure"]
impl crate::Readable for CDS_MSP9 {}
#[doc = "`write(|w| ..)` method takes [cds_msp9::W](cds_msp9::W) writer structure"]
impl crate::Writable for CDS_MSP9 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 9)"]
pub mod cds_msp9;
#[doc = "Channel Source Microblock Stride (chid = 9)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus9](csus9) module"]
pub type CSUS9 = crate::Reg<u32, _CSUS9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS9;
#[doc = "`read()` method returns [csus9::R](csus9::R) reader structure"]
impl crate::Readable for CSUS9 {}
#[doc = "`write(|w| ..)` method takes [csus9::W](csus9::W) writer structure"]
impl crate::Writable for CSUS9 {}
#[doc = "Channel Source Microblock Stride (chid = 9)"]
pub mod csus9;
#[doc = "Channel Destination Microblock Stride (chid = 9)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus9](cdus9) module"]
pub type CDUS9 = crate::Reg<u32, _CDUS9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS9;
#[doc = "`read()` method returns [cdus9::R](cdus9::R) reader structure"]
impl crate::Readable for CDUS9 {}
#[doc = "`write(|w| ..)` method takes [cdus9::W](cdus9::W) writer structure"]
impl crate::Writable for CDUS9 {}
#[doc = "Channel Destination Microblock Stride (chid = 9)"]
pub mod cdus9;
#[doc = "Channel Interrupt Enable Register (chid = 10)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie10](cie10) module"]
pub type CIE10 = crate::Reg<u32, _CIE10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE10;
#[doc = "`write(|w| ..)` method takes [cie10::W](cie10::W) writer structure"]
impl crate::Writable for CIE10 {}
#[doc = "Channel Interrupt Enable Register (chid = 10)"]
pub mod cie10;
#[doc = "Channel Interrupt Disable Register (chid = 10)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid10](cid10) module"]
pub type CID10 = crate::Reg<u32, _CID10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID10;
#[doc = "`write(|w| ..)` method takes [cid10::W](cid10::W) writer structure"]
impl crate::Writable for CID10 {}
#[doc = "Channel Interrupt Disable Register (chid = 10)"]
pub mod cid10;
#[doc = "Channel Interrupt Mask Register (chid = 10)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim10](cim10) module"]
pub type CIM10 = crate::Reg<u32, _CIM10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM10;
#[doc = "`write(|w| ..)` method takes [cim10::W](cim10::W) writer structure"]
impl crate::Writable for CIM10 {}
#[doc = "Channel Interrupt Mask Register (chid = 10)"]
pub mod cim10;
#[doc = "Channel Interrupt Status Register (chid = 10)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis10](cis10) module"]
pub type CIS10 = crate::Reg<u32, _CIS10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS10;
#[doc = "`read()` method returns [cis10::R](cis10::R) reader structure"]
impl crate::Readable for CIS10 {}
#[doc = "Channel Interrupt Status Register (chid = 10)"]
pub mod cis10;
#[doc = "Channel Source Address Register (chid = 10)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa10](csa10) module"]
pub type CSA10 = crate::Reg<u32, _CSA10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA10;
#[doc = "`read()` method returns [csa10::R](csa10::R) reader structure"]
impl crate::Readable for CSA10 {}
#[doc = "`write(|w| ..)` method takes [csa10::W](csa10::W) writer structure"]
impl crate::Writable for CSA10 {}
#[doc = "Channel Source Address Register (chid = 10)"]
pub mod csa10;
#[doc = "Channel Destination Address Register (chid = 10)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda10](cda10) module"]
pub type CDA10 = crate::Reg<u32, _CDA10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA10;
#[doc = "`read()` method returns [cda10::R](cda10::R) reader structure"]
impl crate::Readable for CDA10 {}
#[doc = "`write(|w| ..)` method takes [cda10::W](cda10::W) writer structure"]
impl crate::Writable for CDA10 {}
#[doc = "Channel Destination Address Register (chid = 10)"]
pub mod cda10;
#[doc = "Channel Next Descriptor Address Register (chid = 10)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda10](cnda10) module"]
pub type CNDA10 = crate::Reg<u32, _CNDA10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA10;
#[doc = "`read()` method returns [cnda10::R](cnda10::R) reader structure"]
impl crate::Readable for CNDA10 {}
#[doc = "`write(|w| ..)` method takes [cnda10::W](cnda10::W) writer structure"]
impl crate::Writable for CNDA10 {}
#[doc = "Channel Next Descriptor Address Register (chid = 10)"]
pub mod cnda10;
#[doc = "Channel Next Descriptor Control Register (chid = 10)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc10](cndc10) module"]
pub type CNDC10 = crate::Reg<u32, _CNDC10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC10;
#[doc = "`read()` method returns [cndc10::R](cndc10::R) reader structure"]
impl crate::Readable for CNDC10 {}
#[doc = "`write(|w| ..)` method takes [cndc10::W](cndc10::W) writer structure"]
impl crate::Writable for CNDC10 {}
#[doc = "Channel Next Descriptor Control Register (chid = 10)"]
pub mod cndc10;
#[doc = "Channel Microblock Control Register (chid = 10)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc10](cubc10) module"]
pub type CUBC10 = crate::Reg<u32, _CUBC10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC10;
#[doc = "`read()` method returns [cubc10::R](cubc10::R) reader structure"]
impl crate::Readable for CUBC10 {}
#[doc = "`write(|w| ..)` method takes [cubc10::W](cubc10::W) writer structure"]
impl crate::Writable for CUBC10 {}
#[doc = "Channel Microblock Control Register (chid = 10)"]
pub mod cubc10;
#[doc = "Channel Block Control Register (chid = 10)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc10](cbc10) module"]
pub type CBC10 = crate::Reg<u32, _CBC10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC10;
#[doc = "`read()` method returns [cbc10::R](cbc10::R) reader structure"]
impl crate::Readable for CBC10 {}
#[doc = "`write(|w| ..)` method takes [cbc10::W](cbc10::W) writer structure"]
impl crate::Writable for CBC10 {}
#[doc = "Channel Block Control Register (chid = 10)"]
pub mod cbc10;
#[doc = "Channel Configuration Register (chid = 10)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc10](cc10) module"]
pub type CC10 = crate::Reg<u32, _CC10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC10;
#[doc = "`read()` method returns [cc10::R](cc10::R) reader structure"]
impl crate::Readable for CC10 {}
#[doc = "`write(|w| ..)` method takes [cc10::W](cc10::W) writer structure"]
impl crate::Writable for CC10 {}
#[doc = "Channel Configuration Register (chid = 10)"]
pub mod cc10;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 10)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp10](cds_msp10) module"]
pub type CDS_MSP10 = crate::Reg<u32, _CDS_MSP10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP10;
#[doc = "`read()` method returns [cds_msp10::R](cds_msp10::R) reader structure"]
impl crate::Readable for CDS_MSP10 {}
#[doc = "`write(|w| ..)` method takes [cds_msp10::W](cds_msp10::W) writer structure"]
impl crate::Writable for CDS_MSP10 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 10)"]
pub mod cds_msp10;
#[doc = "Channel Source Microblock Stride (chid = 10)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus10](csus10) module"]
pub type CSUS10 = crate::Reg<u32, _CSUS10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS10;
#[doc = "`read()` method returns [csus10::R](csus10::R) reader structure"]
impl crate::Readable for CSUS10 {}
#[doc = "`write(|w| ..)` method takes [csus10::W](csus10::W) writer structure"]
impl crate::Writable for CSUS10 {}
#[doc = "Channel Source Microblock Stride (chid = 10)"]
pub mod csus10;
#[doc = "Channel Destination Microblock Stride (chid = 10)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus10](cdus10) module"]
pub type CDUS10 = crate::Reg<u32, _CDUS10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS10;
#[doc = "`read()` method returns [cdus10::R](cdus10::R) reader structure"]
impl crate::Readable for CDUS10 {}
#[doc = "`write(|w| ..)` method takes [cdus10::W](cdus10::W) writer structure"]
impl crate::Writable for CDUS10 {}
#[doc = "Channel Destination Microblock Stride (chid = 10)"]
pub mod cdus10;
#[doc = "Channel Interrupt Enable Register (chid = 11)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie11](cie11) module"]
pub type CIE11 = crate::Reg<u32, _CIE11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE11;
#[doc = "`write(|w| ..)` method takes [cie11::W](cie11::W) writer structure"]
impl crate::Writable for CIE11 {}
#[doc = "Channel Interrupt Enable Register (chid = 11)"]
pub mod cie11;
#[doc = "Channel Interrupt Disable Register (chid = 11)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid11](cid11) module"]
pub type CID11 = crate::Reg<u32, _CID11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID11;
#[doc = "`write(|w| ..)` method takes [cid11::W](cid11::W) writer structure"]
impl crate::Writable for CID11 {}
#[doc = "Channel Interrupt Disable Register (chid = 11)"]
pub mod cid11;
#[doc = "Channel Interrupt Mask Register (chid = 11)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim11](cim11) module"]
pub type CIM11 = crate::Reg<u32, _CIM11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM11;
#[doc = "`write(|w| ..)` method takes [cim11::W](cim11::W) writer structure"]
impl crate::Writable for CIM11 {}
#[doc = "Channel Interrupt Mask Register (chid = 11)"]
pub mod cim11;
#[doc = "Channel Interrupt Status Register (chid = 11)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis11](cis11) module"]
pub type CIS11 = crate::Reg<u32, _CIS11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS11;
#[doc = "`read()` method returns [cis11::R](cis11::R) reader structure"]
impl crate::Readable for CIS11 {}
#[doc = "Channel Interrupt Status Register (chid = 11)"]
pub mod cis11;
#[doc = "Channel Source Address Register (chid = 11)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa11](csa11) module"]
pub type CSA11 = crate::Reg<u32, _CSA11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA11;
#[doc = "`read()` method returns [csa11::R](csa11::R) reader structure"]
impl crate::Readable for CSA11 {}
#[doc = "`write(|w| ..)` method takes [csa11::W](csa11::W) writer structure"]
impl crate::Writable for CSA11 {}
#[doc = "Channel Source Address Register (chid = 11)"]
pub mod csa11;
#[doc = "Channel Destination Address Register (chid = 11)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda11](cda11) module"]
pub type CDA11 = crate::Reg<u32, _CDA11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA11;
#[doc = "`read()` method returns [cda11::R](cda11::R) reader structure"]
impl crate::Readable for CDA11 {}
#[doc = "`write(|w| ..)` method takes [cda11::W](cda11::W) writer structure"]
impl crate::Writable for CDA11 {}
#[doc = "Channel Destination Address Register (chid = 11)"]
pub mod cda11;
#[doc = "Channel Next Descriptor Address Register (chid = 11)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda11](cnda11) module"]
pub type CNDA11 = crate::Reg<u32, _CNDA11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA11;
#[doc = "`read()` method returns [cnda11::R](cnda11::R) reader structure"]
impl crate::Readable for CNDA11 {}
#[doc = "`write(|w| ..)` method takes [cnda11::W](cnda11::W) writer structure"]
impl crate::Writable for CNDA11 {}
#[doc = "Channel Next Descriptor Address Register (chid = 11)"]
pub mod cnda11;
#[doc = "Channel Next Descriptor Control Register (chid = 11)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc11](cndc11) module"]
pub type CNDC11 = crate::Reg<u32, _CNDC11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC11;
#[doc = "`read()` method returns [cndc11::R](cndc11::R) reader structure"]
impl crate::Readable for CNDC11 {}
#[doc = "`write(|w| ..)` method takes [cndc11::W](cndc11::W) writer structure"]
impl crate::Writable for CNDC11 {}
#[doc = "Channel Next Descriptor Control Register (chid = 11)"]
pub mod cndc11;
#[doc = "Channel Microblock Control Register (chid = 11)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc11](cubc11) module"]
pub type CUBC11 = crate::Reg<u32, _CUBC11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC11;
#[doc = "`read()` method returns [cubc11::R](cubc11::R) reader structure"]
impl crate::Readable for CUBC11 {}
#[doc = "`write(|w| ..)` method takes [cubc11::W](cubc11::W) writer structure"]
impl crate::Writable for CUBC11 {}
#[doc = "Channel Microblock Control Register (chid = 11)"]
pub mod cubc11;
#[doc = "Channel Block Control Register (chid = 11)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc11](cbc11) module"]
pub type CBC11 = crate::Reg<u32, _CBC11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC11;
#[doc = "`read()` method returns [cbc11::R](cbc11::R) reader structure"]
impl crate::Readable for CBC11 {}
#[doc = "`write(|w| ..)` method takes [cbc11::W](cbc11::W) writer structure"]
impl crate::Writable for CBC11 {}
#[doc = "Channel Block Control Register (chid = 11)"]
pub mod cbc11;
#[doc = "Channel Configuration Register (chid = 11)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc11](cc11) module"]
pub type CC11 = crate::Reg<u32, _CC11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC11;
#[doc = "`read()` method returns [cc11::R](cc11::R) reader structure"]
impl crate::Readable for CC11 {}
#[doc = "`write(|w| ..)` method takes [cc11::W](cc11::W) writer structure"]
impl crate::Writable for CC11 {}
#[doc = "Channel Configuration Register (chid = 11)"]
pub mod cc11;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 11)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp11](cds_msp11) module"]
pub type CDS_MSP11 = crate::Reg<u32, _CDS_MSP11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP11;
#[doc = "`read()` method returns [cds_msp11::R](cds_msp11::R) reader structure"]
impl crate::Readable for CDS_MSP11 {}
#[doc = "`write(|w| ..)` method takes [cds_msp11::W](cds_msp11::W) writer structure"]
impl crate::Writable for CDS_MSP11 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 11)"]
pub mod cds_msp11;
#[doc = "Channel Source Microblock Stride (chid = 11)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus11](csus11) module"]
pub type CSUS11 = crate::Reg<u32, _CSUS11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS11;
#[doc = "`read()` method returns [csus11::R](csus11::R) reader structure"]
impl crate::Readable for CSUS11 {}
#[doc = "`write(|w| ..)` method takes [csus11::W](csus11::W) writer structure"]
impl crate::Writable for CSUS11 {}
#[doc = "Channel Source Microblock Stride (chid = 11)"]
pub mod csus11;
#[doc = "Channel Destination Microblock Stride (chid = 11)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus11](cdus11) module"]
pub type CDUS11 = crate::Reg<u32, _CDUS11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS11;
#[doc = "`read()` method returns [cdus11::R](cdus11::R) reader structure"]
impl crate::Readable for CDUS11 {}
#[doc = "`write(|w| ..)` method takes [cdus11::W](cdus11::W) writer structure"]
impl crate::Writable for CDUS11 {}
#[doc = "Channel Destination Microblock Stride (chid = 11)"]
pub mod cdus11;
#[doc = "Channel Interrupt Enable Register (chid = 12)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie12](cie12) module"]
pub type CIE12 = crate::Reg<u32, _CIE12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE12;
#[doc = "`write(|w| ..)` method takes [cie12::W](cie12::W) writer structure"]
impl crate::Writable for CIE12 {}
#[doc = "Channel Interrupt Enable Register (chid = 12)"]
pub mod cie12;
#[doc = "Channel Interrupt Disable Register (chid = 12)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid12](cid12) module"]
pub type CID12 = crate::Reg<u32, _CID12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID12;
#[doc = "`write(|w| ..)` method takes [cid12::W](cid12::W) writer structure"]
impl crate::Writable for CID12 {}
#[doc = "Channel Interrupt Disable Register (chid = 12)"]
pub mod cid12;
#[doc = "Channel Interrupt Mask Register (chid = 12)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim12](cim12) module"]
pub type CIM12 = crate::Reg<u32, _CIM12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM12;
#[doc = "`write(|w| ..)` method takes [cim12::W](cim12::W) writer structure"]
impl crate::Writable for CIM12 {}
#[doc = "Channel Interrupt Mask Register (chid = 12)"]
pub mod cim12;
#[doc = "Channel Interrupt Status Register (chid = 12)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis12](cis12) module"]
pub type CIS12 = crate::Reg<u32, _CIS12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS12;
#[doc = "`read()` method returns [cis12::R](cis12::R) reader structure"]
impl crate::Readable for CIS12 {}
#[doc = "Channel Interrupt Status Register (chid = 12)"]
pub mod cis12;
#[doc = "Channel Source Address Register (chid = 12)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa12](csa12) module"]
pub type CSA12 = crate::Reg<u32, _CSA12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA12;
#[doc = "`read()` method returns [csa12::R](csa12::R) reader structure"]
impl crate::Readable for CSA12 {}
#[doc = "`write(|w| ..)` method takes [csa12::W](csa12::W) writer structure"]
impl crate::Writable for CSA12 {}
#[doc = "Channel Source Address Register (chid = 12)"]
pub mod csa12;
#[doc = "Channel Destination Address Register (chid = 12)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda12](cda12) module"]
pub type CDA12 = crate::Reg<u32, _CDA12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA12;
#[doc = "`read()` method returns [cda12::R](cda12::R) reader structure"]
impl crate::Readable for CDA12 {}
#[doc = "`write(|w| ..)` method takes [cda12::W](cda12::W) writer structure"]
impl crate::Writable for CDA12 {}
#[doc = "Channel Destination Address Register (chid = 12)"]
pub mod cda12;
#[doc = "Channel Next Descriptor Address Register (chid = 12)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda12](cnda12) module"]
pub type CNDA12 = crate::Reg<u32, _CNDA12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA12;
#[doc = "`read()` method returns [cnda12::R](cnda12::R) reader structure"]
impl crate::Readable for CNDA12 {}
#[doc = "`write(|w| ..)` method takes [cnda12::W](cnda12::W) writer structure"]
impl crate::Writable for CNDA12 {}
#[doc = "Channel Next Descriptor Address Register (chid = 12)"]
pub mod cnda12;
#[doc = "Channel Next Descriptor Control Register (chid = 12)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc12](cndc12) module"]
pub type CNDC12 = crate::Reg<u32, _CNDC12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC12;
#[doc = "`read()` method returns [cndc12::R](cndc12::R) reader structure"]
impl crate::Readable for CNDC12 {}
#[doc = "`write(|w| ..)` method takes [cndc12::W](cndc12::W) writer structure"]
impl crate::Writable for CNDC12 {}
#[doc = "Channel Next Descriptor Control Register (chid = 12)"]
pub mod cndc12;
#[doc = "Channel Microblock Control Register (chid = 12)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc12](cubc12) module"]
pub type CUBC12 = crate::Reg<u32, _CUBC12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC12;
#[doc = "`read()` method returns [cubc12::R](cubc12::R) reader structure"]
impl crate::Readable for CUBC12 {}
#[doc = "`write(|w| ..)` method takes [cubc12::W](cubc12::W) writer structure"]
impl crate::Writable for CUBC12 {}
#[doc = "Channel Microblock Control Register (chid = 12)"]
pub mod cubc12;
#[doc = "Channel Block Control Register (chid = 12)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc12](cbc12) module"]
pub type CBC12 = crate::Reg<u32, _CBC12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC12;
#[doc = "`read()` method returns [cbc12::R](cbc12::R) reader structure"]
impl crate::Readable for CBC12 {}
#[doc = "`write(|w| ..)` method takes [cbc12::W](cbc12::W) writer structure"]
impl crate::Writable for CBC12 {}
#[doc = "Channel Block Control Register (chid = 12)"]
pub mod cbc12;
#[doc = "Channel Configuration Register (chid = 12)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc12](cc12) module"]
pub type CC12 = crate::Reg<u32, _CC12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC12;
#[doc = "`read()` method returns [cc12::R](cc12::R) reader structure"]
impl crate::Readable for CC12 {}
#[doc = "`write(|w| ..)` method takes [cc12::W](cc12::W) writer structure"]
impl crate::Writable for CC12 {}
#[doc = "Channel Configuration Register (chid = 12)"]
pub mod cc12;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 12)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp12](cds_msp12) module"]
pub type CDS_MSP12 = crate::Reg<u32, _CDS_MSP12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP12;
#[doc = "`read()` method returns [cds_msp12::R](cds_msp12::R) reader structure"]
impl crate::Readable for CDS_MSP12 {}
#[doc = "`write(|w| ..)` method takes [cds_msp12::W](cds_msp12::W) writer structure"]
impl crate::Writable for CDS_MSP12 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 12)"]
pub mod cds_msp12;
#[doc = "Channel Source Microblock Stride (chid = 12)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus12](csus12) module"]
pub type CSUS12 = crate::Reg<u32, _CSUS12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS12;
#[doc = "`read()` method returns [csus12::R](csus12::R) reader structure"]
impl crate::Readable for CSUS12 {}
#[doc = "`write(|w| ..)` method takes [csus12::W](csus12::W) writer structure"]
impl crate::Writable for CSUS12 {}
#[doc = "Channel Source Microblock Stride (chid = 12)"]
pub mod csus12;
#[doc = "Channel Destination Microblock Stride (chid = 12)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus12](cdus12) module"]
pub type CDUS12 = crate::Reg<u32, _CDUS12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS12;
#[doc = "`read()` method returns [cdus12::R](cdus12::R) reader structure"]
impl crate::Readable for CDUS12 {}
#[doc = "`write(|w| ..)` method takes [cdus12::W](cdus12::W) writer structure"]
impl crate::Writable for CDUS12 {}
#[doc = "Channel Destination Microblock Stride (chid = 12)"]
pub mod cdus12;
#[doc = "Channel Interrupt Enable Register (chid = 13)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie13](cie13) module"]
pub type CIE13 = crate::Reg<u32, _CIE13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE13;
#[doc = "`write(|w| ..)` method takes [cie13::W](cie13::W) writer structure"]
impl crate::Writable for CIE13 {}
#[doc = "Channel Interrupt Enable Register (chid = 13)"]
pub mod cie13;
#[doc = "Channel Interrupt Disable Register (chid = 13)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid13](cid13) module"]
pub type CID13 = crate::Reg<u32, _CID13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID13;
#[doc = "`write(|w| ..)` method takes [cid13::W](cid13::W) writer structure"]
impl crate::Writable for CID13 {}
#[doc = "Channel Interrupt Disable Register (chid = 13)"]
pub mod cid13;
#[doc = "Channel Interrupt Mask Register (chid = 13)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim13](cim13) module"]
pub type CIM13 = crate::Reg<u32, _CIM13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM13;
#[doc = "`write(|w| ..)` method takes [cim13::W](cim13::W) writer structure"]
impl crate::Writable for CIM13 {}
#[doc = "Channel Interrupt Mask Register (chid = 13)"]
pub mod cim13;
#[doc = "Channel Interrupt Status Register (chid = 13)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis13](cis13) module"]
pub type CIS13 = crate::Reg<u32, _CIS13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS13;
#[doc = "`read()` method returns [cis13::R](cis13::R) reader structure"]
impl crate::Readable for CIS13 {}
#[doc = "Channel Interrupt Status Register (chid = 13)"]
pub mod cis13;
#[doc = "Channel Source Address Register (chid = 13)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa13](csa13) module"]
pub type CSA13 = crate::Reg<u32, _CSA13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA13;
#[doc = "`read()` method returns [csa13::R](csa13::R) reader structure"]
impl crate::Readable for CSA13 {}
#[doc = "`write(|w| ..)` method takes [csa13::W](csa13::W) writer structure"]
impl crate::Writable for CSA13 {}
#[doc = "Channel Source Address Register (chid = 13)"]
pub mod csa13;
#[doc = "Channel Destination Address Register (chid = 13)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda13](cda13) module"]
pub type CDA13 = crate::Reg<u32, _CDA13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA13;
#[doc = "`read()` method returns [cda13::R](cda13::R) reader structure"]
impl crate::Readable for CDA13 {}
#[doc = "`write(|w| ..)` method takes [cda13::W](cda13::W) writer structure"]
impl crate::Writable for CDA13 {}
#[doc = "Channel Destination Address Register (chid = 13)"]
pub mod cda13;
#[doc = "Channel Next Descriptor Address Register (chid = 13)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda13](cnda13) module"]
pub type CNDA13 = crate::Reg<u32, _CNDA13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA13;
#[doc = "`read()` method returns [cnda13::R](cnda13::R) reader structure"]
impl crate::Readable for CNDA13 {}
#[doc = "`write(|w| ..)` method takes [cnda13::W](cnda13::W) writer structure"]
impl crate::Writable for CNDA13 {}
#[doc = "Channel Next Descriptor Address Register (chid = 13)"]
pub mod cnda13;
#[doc = "Channel Next Descriptor Control Register (chid = 13)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc13](cndc13) module"]
pub type CNDC13 = crate::Reg<u32, _CNDC13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC13;
#[doc = "`read()` method returns [cndc13::R](cndc13::R) reader structure"]
impl crate::Readable for CNDC13 {}
#[doc = "`write(|w| ..)` method takes [cndc13::W](cndc13::W) writer structure"]
impl crate::Writable for CNDC13 {}
#[doc = "Channel Next Descriptor Control Register (chid = 13)"]
pub mod cndc13;
#[doc = "Channel Microblock Control Register (chid = 13)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc13](cubc13) module"]
pub type CUBC13 = crate::Reg<u32, _CUBC13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC13;
#[doc = "`read()` method returns [cubc13::R](cubc13::R) reader structure"]
impl crate::Readable for CUBC13 {}
#[doc = "`write(|w| ..)` method takes [cubc13::W](cubc13::W) writer structure"]
impl crate::Writable for CUBC13 {}
#[doc = "Channel Microblock Control Register (chid = 13)"]
pub mod cubc13;
#[doc = "Channel Block Control Register (chid = 13)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc13](cbc13) module"]
pub type CBC13 = crate::Reg<u32, _CBC13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC13;
#[doc = "`read()` method returns [cbc13::R](cbc13::R) reader structure"]
impl crate::Readable for CBC13 {}
#[doc = "`write(|w| ..)` method takes [cbc13::W](cbc13::W) writer structure"]
impl crate::Writable for CBC13 {}
#[doc = "Channel Block Control Register (chid = 13)"]
pub mod cbc13;
#[doc = "Channel Configuration Register (chid = 13)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc13](cc13) module"]
pub type CC13 = crate::Reg<u32, _CC13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC13;
#[doc = "`read()` method returns [cc13::R](cc13::R) reader structure"]
impl crate::Readable for CC13 {}
#[doc = "`write(|w| ..)` method takes [cc13::W](cc13::W) writer structure"]
impl crate::Writable for CC13 {}
#[doc = "Channel Configuration Register (chid = 13)"]
pub mod cc13;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 13)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp13](cds_msp13) module"]
pub type CDS_MSP13 = crate::Reg<u32, _CDS_MSP13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP13;
#[doc = "`read()` method returns [cds_msp13::R](cds_msp13::R) reader structure"]
impl crate::Readable for CDS_MSP13 {}
#[doc = "`write(|w| ..)` method takes [cds_msp13::W](cds_msp13::W) writer structure"]
impl crate::Writable for CDS_MSP13 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 13)"]
pub mod cds_msp13;
#[doc = "Channel Source Microblock Stride (chid = 13)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus13](csus13) module"]
pub type CSUS13 = crate::Reg<u32, _CSUS13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS13;
#[doc = "`read()` method returns [csus13::R](csus13::R) reader structure"]
impl crate::Readable for CSUS13 {}
#[doc = "`write(|w| ..)` method takes [csus13::W](csus13::W) writer structure"]
impl crate::Writable for CSUS13 {}
#[doc = "Channel Source Microblock Stride (chid = 13)"]
pub mod csus13;
#[doc = "Channel Destination Microblock Stride (chid = 13)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus13](cdus13) module"]
pub type CDUS13 = crate::Reg<u32, _CDUS13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS13;
#[doc = "`read()` method returns [cdus13::R](cdus13::R) reader structure"]
impl crate::Readable for CDUS13 {}
#[doc = "`write(|w| ..)` method takes [cdus13::W](cdus13::W) writer structure"]
impl crate::Writable for CDUS13 {}
#[doc = "Channel Destination Microblock Stride (chid = 13)"]
pub mod cdus13;
#[doc = "Channel Interrupt Enable Register (chid = 14)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie14](cie14) module"]
pub type CIE14 = crate::Reg<u32, _CIE14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE14;
#[doc = "`write(|w| ..)` method takes [cie14::W](cie14::W) writer structure"]
impl crate::Writable for CIE14 {}
#[doc = "Channel Interrupt Enable Register (chid = 14)"]
pub mod cie14;
#[doc = "Channel Interrupt Disable Register (chid = 14)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid14](cid14) module"]
pub type CID14 = crate::Reg<u32, _CID14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID14;
#[doc = "`write(|w| ..)` method takes [cid14::W](cid14::W) writer structure"]
impl crate::Writable for CID14 {}
#[doc = "Channel Interrupt Disable Register (chid = 14)"]
pub mod cid14;
#[doc = "Channel Interrupt Mask Register (chid = 14)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim14](cim14) module"]
pub type CIM14 = crate::Reg<u32, _CIM14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM14;
#[doc = "`write(|w| ..)` method takes [cim14::W](cim14::W) writer structure"]
impl crate::Writable for CIM14 {}
#[doc = "Channel Interrupt Mask Register (chid = 14)"]
pub mod cim14;
#[doc = "Channel Interrupt Status Register (chid = 14)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis14](cis14) module"]
pub type CIS14 = crate::Reg<u32, _CIS14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS14;
#[doc = "`read()` method returns [cis14::R](cis14::R) reader structure"]
impl crate::Readable for CIS14 {}
#[doc = "Channel Interrupt Status Register (chid = 14)"]
pub mod cis14;
#[doc = "Channel Source Address Register (chid = 14)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa14](csa14) module"]
pub type CSA14 = crate::Reg<u32, _CSA14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA14;
#[doc = "`read()` method returns [csa14::R](csa14::R) reader structure"]
impl crate::Readable for CSA14 {}
#[doc = "`write(|w| ..)` method takes [csa14::W](csa14::W) writer structure"]
impl crate::Writable for CSA14 {}
#[doc = "Channel Source Address Register (chid = 14)"]
pub mod csa14;
#[doc = "Channel Destination Address Register (chid = 14)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda14](cda14) module"]
pub type CDA14 = crate::Reg<u32, _CDA14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA14;
#[doc = "`read()` method returns [cda14::R](cda14::R) reader structure"]
impl crate::Readable for CDA14 {}
#[doc = "`write(|w| ..)` method takes [cda14::W](cda14::W) writer structure"]
impl crate::Writable for CDA14 {}
#[doc = "Channel Destination Address Register (chid = 14)"]
pub mod cda14;
#[doc = "Channel Next Descriptor Address Register (chid = 14)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda14](cnda14) module"]
pub type CNDA14 = crate::Reg<u32, _CNDA14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA14;
#[doc = "`read()` method returns [cnda14::R](cnda14::R) reader structure"]
impl crate::Readable for CNDA14 {}
#[doc = "`write(|w| ..)` method takes [cnda14::W](cnda14::W) writer structure"]
impl crate::Writable for CNDA14 {}
#[doc = "Channel Next Descriptor Address Register (chid = 14)"]
pub mod cnda14;
#[doc = "Channel Next Descriptor Control Register (chid = 14)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc14](cndc14) module"]
pub type CNDC14 = crate::Reg<u32, _CNDC14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC14;
#[doc = "`read()` method returns [cndc14::R](cndc14::R) reader structure"]
impl crate::Readable for CNDC14 {}
#[doc = "`write(|w| ..)` method takes [cndc14::W](cndc14::W) writer structure"]
impl crate::Writable for CNDC14 {}
#[doc = "Channel Next Descriptor Control Register (chid = 14)"]
pub mod cndc14;
#[doc = "Channel Microblock Control Register (chid = 14)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc14](cubc14) module"]
pub type CUBC14 = crate::Reg<u32, _CUBC14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC14;
#[doc = "`read()` method returns [cubc14::R](cubc14::R) reader structure"]
impl crate::Readable for CUBC14 {}
#[doc = "`write(|w| ..)` method takes [cubc14::W](cubc14::W) writer structure"]
impl crate::Writable for CUBC14 {}
#[doc = "Channel Microblock Control Register (chid = 14)"]
pub mod cubc14;
#[doc = "Channel Block Control Register (chid = 14)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc14](cbc14) module"]
pub type CBC14 = crate::Reg<u32, _CBC14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC14;
#[doc = "`read()` method returns [cbc14::R](cbc14::R) reader structure"]
impl crate::Readable for CBC14 {}
#[doc = "`write(|w| ..)` method takes [cbc14::W](cbc14::W) writer structure"]
impl crate::Writable for CBC14 {}
#[doc = "Channel Block Control Register (chid = 14)"]
pub mod cbc14;
#[doc = "Channel Configuration Register (chid = 14)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc14](cc14) module"]
pub type CC14 = crate::Reg<u32, _CC14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC14;
#[doc = "`read()` method returns [cc14::R](cc14::R) reader structure"]
impl crate::Readable for CC14 {}
#[doc = "`write(|w| ..)` method takes [cc14::W](cc14::W) writer structure"]
impl crate::Writable for CC14 {}
#[doc = "Channel Configuration Register (chid = 14)"]
pub mod cc14;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 14)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp14](cds_msp14) module"]
pub type CDS_MSP14 = crate::Reg<u32, _CDS_MSP14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP14;
#[doc = "`read()` method returns [cds_msp14::R](cds_msp14::R) reader structure"]
impl crate::Readable for CDS_MSP14 {}
#[doc = "`write(|w| ..)` method takes [cds_msp14::W](cds_msp14::W) writer structure"]
impl crate::Writable for CDS_MSP14 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 14)"]
pub mod cds_msp14;
#[doc = "Channel Source Microblock Stride (chid = 14)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus14](csus14) module"]
pub type CSUS14 = crate::Reg<u32, _CSUS14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS14;
#[doc = "`read()` method returns [csus14::R](csus14::R) reader structure"]
impl crate::Readable for CSUS14 {}
#[doc = "`write(|w| ..)` method takes [csus14::W](csus14::W) writer structure"]
impl crate::Writable for CSUS14 {}
#[doc = "Channel Source Microblock Stride (chid = 14)"]
pub mod csus14;
#[doc = "Channel Destination Microblock Stride (chid = 14)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus14](cdus14) module"]
pub type CDUS14 = crate::Reg<u32, _CDUS14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS14;
#[doc = "`read()` method returns [cdus14::R](cdus14::R) reader structure"]
impl crate::Readable for CDUS14 {}
#[doc = "`write(|w| ..)` method takes [cdus14::W](cdus14::W) writer structure"]
impl crate::Writable for CDUS14 {}
#[doc = "Channel Destination Microblock Stride (chid = 14)"]
pub mod cdus14;
#[doc = "Channel Interrupt Enable Register (chid = 15)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie15](cie15) module"]
pub type CIE15 = crate::Reg<u32, _CIE15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE15;
#[doc = "`write(|w| ..)` method takes [cie15::W](cie15::W) writer structure"]
impl crate::Writable for CIE15 {}
#[doc = "Channel Interrupt Enable Register (chid = 15)"]
pub mod cie15;
#[doc = "Channel Interrupt Disable Register (chid = 15)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid15](cid15) module"]
pub type CID15 = crate::Reg<u32, _CID15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID15;
#[doc = "`write(|w| ..)` method takes [cid15::W](cid15::W) writer structure"]
impl crate::Writable for CID15 {}
#[doc = "Channel Interrupt Disable Register (chid = 15)"]
pub mod cid15;
#[doc = "Channel Interrupt Mask Register (chid = 15)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim15](cim15) module"]
pub type CIM15 = crate::Reg<u32, _CIM15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM15;
#[doc = "`write(|w| ..)` method takes [cim15::W](cim15::W) writer structure"]
impl crate::Writable for CIM15 {}
#[doc = "Channel Interrupt Mask Register (chid = 15)"]
pub mod cim15;
#[doc = "Channel Interrupt Status Register (chid = 15)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis15](cis15) module"]
pub type CIS15 = crate::Reg<u32, _CIS15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS15;
#[doc = "`read()` method returns [cis15::R](cis15::R) reader structure"]
impl crate::Readable for CIS15 {}
#[doc = "Channel Interrupt Status Register (chid = 15)"]
pub mod cis15;
#[doc = "Channel Source Address Register (chid = 15)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa15](csa15) module"]
pub type CSA15 = crate::Reg<u32, _CSA15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA15;
#[doc = "`read()` method returns [csa15::R](csa15::R) reader structure"]
impl crate::Readable for CSA15 {}
#[doc = "`write(|w| ..)` method takes [csa15::W](csa15::W) writer structure"]
impl crate::Writable for CSA15 {}
#[doc = "Channel Source Address Register (chid = 15)"]
pub mod csa15;
#[doc = "Channel Destination Address Register (chid = 15)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda15](cda15) module"]
pub type CDA15 = crate::Reg<u32, _CDA15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA15;
#[doc = "`read()` method returns [cda15::R](cda15::R) reader structure"]
impl crate::Readable for CDA15 {}
#[doc = "`write(|w| ..)` method takes [cda15::W](cda15::W) writer structure"]
impl crate::Writable for CDA15 {}
#[doc = "Channel Destination Address Register (chid = 15)"]
pub mod cda15;
#[doc = "Channel Next Descriptor Address Register (chid = 15)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda15](cnda15) module"]
pub type CNDA15 = crate::Reg<u32, _CNDA15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA15;
#[doc = "`read()` method returns [cnda15::R](cnda15::R) reader structure"]
impl crate::Readable for CNDA15 {}
#[doc = "`write(|w| ..)` method takes [cnda15::W](cnda15::W) writer structure"]
impl crate::Writable for CNDA15 {}
#[doc = "Channel Next Descriptor Address Register (chid = 15)"]
pub mod cnda15;
#[doc = "Channel Next Descriptor Control Register (chid = 15)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc15](cndc15) module"]
pub type CNDC15 = crate::Reg<u32, _CNDC15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC15;
#[doc = "`read()` method returns [cndc15::R](cndc15::R) reader structure"]
impl crate::Readable for CNDC15 {}
#[doc = "`write(|w| ..)` method takes [cndc15::W](cndc15::W) writer structure"]
impl crate::Writable for CNDC15 {}
#[doc = "Channel Next Descriptor Control Register (chid = 15)"]
pub mod cndc15;
#[doc = "Channel Microblock Control Register (chid = 15)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc15](cubc15) module"]
pub type CUBC15 = crate::Reg<u32, _CUBC15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC15;
#[doc = "`read()` method returns [cubc15::R](cubc15::R) reader structure"]
impl crate::Readable for CUBC15 {}
#[doc = "`write(|w| ..)` method takes [cubc15::W](cubc15::W) writer structure"]
impl crate::Writable for CUBC15 {}
#[doc = "Channel Microblock Control Register (chid = 15)"]
pub mod cubc15;
#[doc = "Channel Block Control Register (chid = 15)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc15](cbc15) module"]
pub type CBC15 = crate::Reg<u32, _CBC15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC15;
#[doc = "`read()` method returns [cbc15::R](cbc15::R) reader structure"]
impl crate::Readable for CBC15 {}
#[doc = "`write(|w| ..)` method takes [cbc15::W](cbc15::W) writer structure"]
impl crate::Writable for CBC15 {}
#[doc = "Channel Block Control Register (chid = 15)"]
pub mod cbc15;
#[doc = "Channel Configuration Register (chid = 15)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc15](cc15) module"]
pub type CC15 = crate::Reg<u32, _CC15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC15;
#[doc = "`read()` method returns [cc15::R](cc15::R) reader structure"]
impl crate::Readable for CC15 {}
#[doc = "`write(|w| ..)` method takes [cc15::W](cc15::W) writer structure"]
impl crate::Writable for CC15 {}
#[doc = "Channel Configuration Register (chid = 15)"]
pub mod cc15;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 15)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp15](cds_msp15) module"]
pub type CDS_MSP15 = crate::Reg<u32, _CDS_MSP15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP15;
#[doc = "`read()` method returns [cds_msp15::R](cds_msp15::R) reader structure"]
impl crate::Readable for CDS_MSP15 {}
#[doc = "`write(|w| ..)` method takes [cds_msp15::W](cds_msp15::W) writer structure"]
impl crate::Writable for CDS_MSP15 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 15)"]
pub mod cds_msp15;
#[doc = "Channel Source Microblock Stride (chid = 15)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus15](csus15) module"]
pub type CSUS15 = crate::Reg<u32, _CSUS15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS15;
#[doc = "`read()` method returns [csus15::R](csus15::R) reader structure"]
impl crate::Readable for CSUS15 {}
#[doc = "`write(|w| ..)` method takes [csus15::W](csus15::W) writer structure"]
impl crate::Writable for CSUS15 {}
#[doc = "Channel Source Microblock Stride (chid = 15)"]
pub mod csus15;
#[doc = "Channel Destination Microblock Stride (chid = 15)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus15](cdus15) module"]
pub type CDUS15 = crate::Reg<u32, _CDUS15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS15;
#[doc = "`read()` method returns [cdus15::R](cdus15::R) reader structure"]
impl crate::Readable for CDUS15 {}
#[doc = "`write(|w| ..)` method takes [cdus15::W](cdus15::W) writer structure"]
impl crate::Writable for CDUS15 {}
#[doc = "Channel Destination Microblock Stride (chid = 15)"]
pub mod cdus15;
#[doc = "Channel Interrupt Enable Register (chid = 16)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie16](cie16) module"]
pub type CIE16 = crate::Reg<u32, _CIE16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE16;
#[doc = "`write(|w| ..)` method takes [cie16::W](cie16::W) writer structure"]
impl crate::Writable for CIE16 {}
#[doc = "Channel Interrupt Enable Register (chid = 16)"]
pub mod cie16;
#[doc = "Channel Interrupt Disable Register (chid = 16)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid16](cid16) module"]
pub type CID16 = crate::Reg<u32, _CID16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID16;
#[doc = "`write(|w| ..)` method takes [cid16::W](cid16::W) writer structure"]
impl crate::Writable for CID16 {}
#[doc = "Channel Interrupt Disable Register (chid = 16)"]
pub mod cid16;
#[doc = "Channel Interrupt Mask Register (chid = 16)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim16](cim16) module"]
pub type CIM16 = crate::Reg<u32, _CIM16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM16;
#[doc = "`write(|w| ..)` method takes [cim16::W](cim16::W) writer structure"]
impl crate::Writable for CIM16 {}
#[doc = "Channel Interrupt Mask Register (chid = 16)"]
pub mod cim16;
#[doc = "Channel Interrupt Status Register (chid = 16)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis16](cis16) module"]
pub type CIS16 = crate::Reg<u32, _CIS16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS16;
#[doc = "`read()` method returns [cis16::R](cis16::R) reader structure"]
impl crate::Readable for CIS16 {}
#[doc = "Channel Interrupt Status Register (chid = 16)"]
pub mod cis16;
#[doc = "Channel Source Address Register (chid = 16)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa16](csa16) module"]
pub type CSA16 = crate::Reg<u32, _CSA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA16;
#[doc = "`read()` method returns [csa16::R](csa16::R) reader structure"]
impl crate::Readable for CSA16 {}
#[doc = "`write(|w| ..)` method takes [csa16::W](csa16::W) writer structure"]
impl crate::Writable for CSA16 {}
#[doc = "Channel Source Address Register (chid = 16)"]
pub mod csa16;
#[doc = "Channel Destination Address Register (chid = 16)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda16](cda16) module"]
pub type CDA16 = crate::Reg<u32, _CDA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA16;
#[doc = "`read()` method returns [cda16::R](cda16::R) reader structure"]
impl crate::Readable for CDA16 {}
#[doc = "`write(|w| ..)` method takes [cda16::W](cda16::W) writer structure"]
impl crate::Writable for CDA16 {}
#[doc = "Channel Destination Address Register (chid = 16)"]
pub mod cda16;
#[doc = "Channel Next Descriptor Address Register (chid = 16)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda16](cnda16) module"]
pub type CNDA16 = crate::Reg<u32, _CNDA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA16;
#[doc = "`read()` method returns [cnda16::R](cnda16::R) reader structure"]
impl crate::Readable for CNDA16 {}
#[doc = "`write(|w| ..)` method takes [cnda16::W](cnda16::W) writer structure"]
impl crate::Writable for CNDA16 {}
#[doc = "Channel Next Descriptor Address Register (chid = 16)"]
pub mod cnda16;
#[doc = "Channel Next Descriptor Control Register (chid = 16)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc16](cndc16) module"]
pub type CNDC16 = crate::Reg<u32, _CNDC16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC16;
#[doc = "`read()` method returns [cndc16::R](cndc16::R) reader structure"]
impl crate::Readable for CNDC16 {}
#[doc = "`write(|w| ..)` method takes [cndc16::W](cndc16::W) writer structure"]
impl crate::Writable for CNDC16 {}
#[doc = "Channel Next Descriptor Control Register (chid = 16)"]
pub mod cndc16;
#[doc = "Channel Microblock Control Register (chid = 16)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc16](cubc16) module"]
pub type CUBC16 = crate::Reg<u32, _CUBC16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC16;
#[doc = "`read()` method returns [cubc16::R](cubc16::R) reader structure"]
impl crate::Readable for CUBC16 {}
#[doc = "`write(|w| ..)` method takes [cubc16::W](cubc16::W) writer structure"]
impl crate::Writable for CUBC16 {}
#[doc = "Channel Microblock Control Register (chid = 16)"]
pub mod cubc16;
#[doc = "Channel Block Control Register (chid = 16)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc16](cbc16) module"]
pub type CBC16 = crate::Reg<u32, _CBC16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC16;
#[doc = "`read()` method returns [cbc16::R](cbc16::R) reader structure"]
impl crate::Readable for CBC16 {}
#[doc = "`write(|w| ..)` method takes [cbc16::W](cbc16::W) writer structure"]
impl crate::Writable for CBC16 {}
#[doc = "Channel Block Control Register (chid = 16)"]
pub mod cbc16;
#[doc = "Channel Configuration Register (chid = 16)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc16](cc16) module"]
pub type CC16 = crate::Reg<u32, _CC16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC16;
#[doc = "`read()` method returns [cc16::R](cc16::R) reader structure"]
impl crate::Readable for CC16 {}
#[doc = "`write(|w| ..)` method takes [cc16::W](cc16::W) writer structure"]
impl crate::Writable for CC16 {}
#[doc = "Channel Configuration Register (chid = 16)"]
pub mod cc16;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 16)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp16](cds_msp16) module"]
pub type CDS_MSP16 = crate::Reg<u32, _CDS_MSP16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP16;
#[doc = "`read()` method returns [cds_msp16::R](cds_msp16::R) reader structure"]
impl crate::Readable for CDS_MSP16 {}
#[doc = "`write(|w| ..)` method takes [cds_msp16::W](cds_msp16::W) writer structure"]
impl crate::Writable for CDS_MSP16 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 16)"]
pub mod cds_msp16;
#[doc = "Channel Source Microblock Stride (chid = 16)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus16](csus16) module"]
pub type CSUS16 = crate::Reg<u32, _CSUS16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS16;
#[doc = "`read()` method returns [csus16::R](csus16::R) reader structure"]
impl crate::Readable for CSUS16 {}
#[doc = "`write(|w| ..)` method takes [csus16::W](csus16::W) writer structure"]
impl crate::Writable for CSUS16 {}
#[doc = "Channel Source Microblock Stride (chid = 16)"]
pub mod csus16;
#[doc = "Channel Destination Microblock Stride (chid = 16)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus16](cdus16) module"]
pub type CDUS16 = crate::Reg<u32, _CDUS16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS16;
#[doc = "`read()` method returns [cdus16::R](cdus16::R) reader structure"]
impl crate::Readable for CDUS16 {}
#[doc = "`write(|w| ..)` method takes [cdus16::W](cdus16::W) writer structure"]
impl crate::Writable for CDUS16 {}
#[doc = "Channel Destination Microblock Stride (chid = 16)"]
pub mod cdus16;
#[doc = "Channel Interrupt Enable Register (chid = 17)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie17](cie17) module"]
pub type CIE17 = crate::Reg<u32, _CIE17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE17;
#[doc = "`write(|w| ..)` method takes [cie17::W](cie17::W) writer structure"]
impl crate::Writable for CIE17 {}
#[doc = "Channel Interrupt Enable Register (chid = 17)"]
pub mod cie17;
#[doc = "Channel Interrupt Disable Register (chid = 17)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid17](cid17) module"]
pub type CID17 = crate::Reg<u32, _CID17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID17;
#[doc = "`write(|w| ..)` method takes [cid17::W](cid17::W) writer structure"]
impl crate::Writable for CID17 {}
#[doc = "Channel Interrupt Disable Register (chid = 17)"]
pub mod cid17;
#[doc = "Channel Interrupt Mask Register (chid = 17)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim17](cim17) module"]
pub type CIM17 = crate::Reg<u32, _CIM17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM17;
#[doc = "`write(|w| ..)` method takes [cim17::W](cim17::W) writer structure"]
impl crate::Writable for CIM17 {}
#[doc = "Channel Interrupt Mask Register (chid = 17)"]
pub mod cim17;
#[doc = "Channel Interrupt Status Register (chid = 17)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis17](cis17) module"]
pub type CIS17 = crate::Reg<u32, _CIS17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS17;
#[doc = "`read()` method returns [cis17::R](cis17::R) reader structure"]
impl crate::Readable for CIS17 {}
#[doc = "Channel Interrupt Status Register (chid = 17)"]
pub mod cis17;
#[doc = "Channel Source Address Register (chid = 17)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa17](csa17) module"]
pub type CSA17 = crate::Reg<u32, _CSA17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA17;
#[doc = "`read()` method returns [csa17::R](csa17::R) reader structure"]
impl crate::Readable for CSA17 {}
#[doc = "`write(|w| ..)` method takes [csa17::W](csa17::W) writer structure"]
impl crate::Writable for CSA17 {}
#[doc = "Channel Source Address Register (chid = 17)"]
pub mod csa17;
#[doc = "Channel Destination Address Register (chid = 17)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda17](cda17) module"]
pub type CDA17 = crate::Reg<u32, _CDA17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA17;
#[doc = "`read()` method returns [cda17::R](cda17::R) reader structure"]
impl crate::Readable for CDA17 {}
#[doc = "`write(|w| ..)` method takes [cda17::W](cda17::W) writer structure"]
impl crate::Writable for CDA17 {}
#[doc = "Channel Destination Address Register (chid = 17)"]
pub mod cda17;
#[doc = "Channel Next Descriptor Address Register (chid = 17)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda17](cnda17) module"]
pub type CNDA17 = crate::Reg<u32, _CNDA17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA17;
#[doc = "`read()` method returns [cnda17::R](cnda17::R) reader structure"]
impl crate::Readable for CNDA17 {}
#[doc = "`write(|w| ..)` method takes [cnda17::W](cnda17::W) writer structure"]
impl crate::Writable for CNDA17 {}
#[doc = "Channel Next Descriptor Address Register (chid = 17)"]
pub mod cnda17;
#[doc = "Channel Next Descriptor Control Register (chid = 17)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc17](cndc17) module"]
pub type CNDC17 = crate::Reg<u32, _CNDC17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC17;
#[doc = "`read()` method returns [cndc17::R](cndc17::R) reader structure"]
impl crate::Readable for CNDC17 {}
#[doc = "`write(|w| ..)` method takes [cndc17::W](cndc17::W) writer structure"]
impl crate::Writable for CNDC17 {}
#[doc = "Channel Next Descriptor Control Register (chid = 17)"]
pub mod cndc17;
#[doc = "Channel Microblock Control Register (chid = 17)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc17](cubc17) module"]
pub type CUBC17 = crate::Reg<u32, _CUBC17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC17;
#[doc = "`read()` method returns [cubc17::R](cubc17::R) reader structure"]
impl crate::Readable for CUBC17 {}
#[doc = "`write(|w| ..)` method takes [cubc17::W](cubc17::W) writer structure"]
impl crate::Writable for CUBC17 {}
#[doc = "Channel Microblock Control Register (chid = 17)"]
pub mod cubc17;
#[doc = "Channel Block Control Register (chid = 17)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc17](cbc17) module"]
pub type CBC17 = crate::Reg<u32, _CBC17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC17;
#[doc = "`read()` method returns [cbc17::R](cbc17::R) reader structure"]
impl crate::Readable for CBC17 {}
#[doc = "`write(|w| ..)` method takes [cbc17::W](cbc17::W) writer structure"]
impl crate::Writable for CBC17 {}
#[doc = "Channel Block Control Register (chid = 17)"]
pub mod cbc17;
#[doc = "Channel Configuration Register (chid = 17)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc17](cc17) module"]
pub type CC17 = crate::Reg<u32, _CC17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC17;
#[doc = "`read()` method returns [cc17::R](cc17::R) reader structure"]
impl crate::Readable for CC17 {}
#[doc = "`write(|w| ..)` method takes [cc17::W](cc17::W) writer structure"]
impl crate::Writable for CC17 {}
#[doc = "Channel Configuration Register (chid = 17)"]
pub mod cc17;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 17)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp17](cds_msp17) module"]
pub type CDS_MSP17 = crate::Reg<u32, _CDS_MSP17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP17;
#[doc = "`read()` method returns [cds_msp17::R](cds_msp17::R) reader structure"]
impl crate::Readable for CDS_MSP17 {}
#[doc = "`write(|w| ..)` method takes [cds_msp17::W](cds_msp17::W) writer structure"]
impl crate::Writable for CDS_MSP17 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 17)"]
pub mod cds_msp17;
#[doc = "Channel Source Microblock Stride (chid = 17)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus17](csus17) module"]
pub type CSUS17 = crate::Reg<u32, _CSUS17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS17;
#[doc = "`read()` method returns [csus17::R](csus17::R) reader structure"]
impl crate::Readable for CSUS17 {}
#[doc = "`write(|w| ..)` method takes [csus17::W](csus17::W) writer structure"]
impl crate::Writable for CSUS17 {}
#[doc = "Channel Source Microblock Stride (chid = 17)"]
pub mod csus17;
#[doc = "Channel Destination Microblock Stride (chid = 17)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus17](cdus17) module"]
pub type CDUS17 = crate::Reg<u32, _CDUS17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS17;
#[doc = "`read()` method returns [cdus17::R](cdus17::R) reader structure"]
impl crate::Readable for CDUS17 {}
#[doc = "`write(|w| ..)` method takes [cdus17::W](cdus17::W) writer structure"]
impl crate::Writable for CDUS17 {}
#[doc = "Channel Destination Microblock Stride (chid = 17)"]
pub mod cdus17;
#[doc = "Channel Interrupt Enable Register (chid = 18)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie18](cie18) module"]
pub type CIE18 = crate::Reg<u32, _CIE18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE18;
#[doc = "`write(|w| ..)` method takes [cie18::W](cie18::W) writer structure"]
impl crate::Writable for CIE18 {}
#[doc = "Channel Interrupt Enable Register (chid = 18)"]
pub mod cie18;
#[doc = "Channel Interrupt Disable Register (chid = 18)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid18](cid18) module"]
pub type CID18 = crate::Reg<u32, _CID18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID18;
#[doc = "`write(|w| ..)` method takes [cid18::W](cid18::W) writer structure"]
impl crate::Writable for CID18 {}
#[doc = "Channel Interrupt Disable Register (chid = 18)"]
pub mod cid18;
#[doc = "Channel Interrupt Mask Register (chid = 18)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim18](cim18) module"]
pub type CIM18 = crate::Reg<u32, _CIM18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM18;
#[doc = "`write(|w| ..)` method takes [cim18::W](cim18::W) writer structure"]
impl crate::Writable for CIM18 {}
#[doc = "Channel Interrupt Mask Register (chid = 18)"]
pub mod cim18;
#[doc = "Channel Interrupt Status Register (chid = 18)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis18](cis18) module"]
pub type CIS18 = crate::Reg<u32, _CIS18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS18;
#[doc = "`read()` method returns [cis18::R](cis18::R) reader structure"]
impl crate::Readable for CIS18 {}
#[doc = "Channel Interrupt Status Register (chid = 18)"]
pub mod cis18;
#[doc = "Channel Source Address Register (chid = 18)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa18](csa18) module"]
pub type CSA18 = crate::Reg<u32, _CSA18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA18;
#[doc = "`read()` method returns [csa18::R](csa18::R) reader structure"]
impl crate::Readable for CSA18 {}
#[doc = "`write(|w| ..)` method takes [csa18::W](csa18::W) writer structure"]
impl crate::Writable for CSA18 {}
#[doc = "Channel Source Address Register (chid = 18)"]
pub mod csa18;
#[doc = "Channel Destination Address Register (chid = 18)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda18](cda18) module"]
pub type CDA18 = crate::Reg<u32, _CDA18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA18;
#[doc = "`read()` method returns [cda18::R](cda18::R) reader structure"]
impl crate::Readable for CDA18 {}
#[doc = "`write(|w| ..)` method takes [cda18::W](cda18::W) writer structure"]
impl crate::Writable for CDA18 {}
#[doc = "Channel Destination Address Register (chid = 18)"]
pub mod cda18;
#[doc = "Channel Next Descriptor Address Register (chid = 18)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda18](cnda18) module"]
pub type CNDA18 = crate::Reg<u32, _CNDA18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA18;
#[doc = "`read()` method returns [cnda18::R](cnda18::R) reader structure"]
impl crate::Readable for CNDA18 {}
#[doc = "`write(|w| ..)` method takes [cnda18::W](cnda18::W) writer structure"]
impl crate::Writable for CNDA18 {}
#[doc = "Channel Next Descriptor Address Register (chid = 18)"]
pub mod cnda18;
#[doc = "Channel Next Descriptor Control Register (chid = 18)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc18](cndc18) module"]
pub type CNDC18 = crate::Reg<u32, _CNDC18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC18;
#[doc = "`read()` method returns [cndc18::R](cndc18::R) reader structure"]
impl crate::Readable for CNDC18 {}
#[doc = "`write(|w| ..)` method takes [cndc18::W](cndc18::W) writer structure"]
impl crate::Writable for CNDC18 {}
#[doc = "Channel Next Descriptor Control Register (chid = 18)"]
pub mod cndc18;
#[doc = "Channel Microblock Control Register (chid = 18)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc18](cubc18) module"]
pub type CUBC18 = crate::Reg<u32, _CUBC18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC18;
#[doc = "`read()` method returns [cubc18::R](cubc18::R) reader structure"]
impl crate::Readable for CUBC18 {}
#[doc = "`write(|w| ..)` method takes [cubc18::W](cubc18::W) writer structure"]
impl crate::Writable for CUBC18 {}
#[doc = "Channel Microblock Control Register (chid = 18)"]
pub mod cubc18;
#[doc = "Channel Block Control Register (chid = 18)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc18](cbc18) module"]
pub type CBC18 = crate::Reg<u32, _CBC18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC18;
#[doc = "`read()` method returns [cbc18::R](cbc18::R) reader structure"]
impl crate::Readable for CBC18 {}
#[doc = "`write(|w| ..)` method takes [cbc18::W](cbc18::W) writer structure"]
impl crate::Writable for CBC18 {}
#[doc = "Channel Block Control Register (chid = 18)"]
pub mod cbc18;
#[doc = "Channel Configuration Register (chid = 18)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc18](cc18) module"]
pub type CC18 = crate::Reg<u32, _CC18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC18;
#[doc = "`read()` method returns [cc18::R](cc18::R) reader structure"]
impl crate::Readable for CC18 {}
#[doc = "`write(|w| ..)` method takes [cc18::W](cc18::W) writer structure"]
impl crate::Writable for CC18 {}
#[doc = "Channel Configuration Register (chid = 18)"]
pub mod cc18;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 18)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp18](cds_msp18) module"]
pub type CDS_MSP18 = crate::Reg<u32, _CDS_MSP18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP18;
#[doc = "`read()` method returns [cds_msp18::R](cds_msp18::R) reader structure"]
impl crate::Readable for CDS_MSP18 {}
#[doc = "`write(|w| ..)` method takes [cds_msp18::W](cds_msp18::W) writer structure"]
impl crate::Writable for CDS_MSP18 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 18)"]
pub mod cds_msp18;
#[doc = "Channel Source Microblock Stride (chid = 18)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus18](csus18) module"]
pub type CSUS18 = crate::Reg<u32, _CSUS18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS18;
#[doc = "`read()` method returns [csus18::R](csus18::R) reader structure"]
impl crate::Readable for CSUS18 {}
#[doc = "`write(|w| ..)` method takes [csus18::W](csus18::W) writer structure"]
impl crate::Writable for CSUS18 {}
#[doc = "Channel Source Microblock Stride (chid = 18)"]
pub mod csus18;
#[doc = "Channel Destination Microblock Stride (chid = 18)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus18](cdus18) module"]
pub type CDUS18 = crate::Reg<u32, _CDUS18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS18;
#[doc = "`read()` method returns [cdus18::R](cdus18::R) reader structure"]
impl crate::Readable for CDUS18 {}
#[doc = "`write(|w| ..)` method takes [cdus18::W](cdus18::W) writer structure"]
impl crate::Writable for CDUS18 {}
#[doc = "Channel Destination Microblock Stride (chid = 18)"]
pub mod cdus18;
#[doc = "Channel Interrupt Enable Register (chid = 19)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie19](cie19) module"]
pub type CIE19 = crate::Reg<u32, _CIE19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE19;
#[doc = "`write(|w| ..)` method takes [cie19::W](cie19::W) writer structure"]
impl crate::Writable for CIE19 {}
#[doc = "Channel Interrupt Enable Register (chid = 19)"]
pub mod cie19;
#[doc = "Channel Interrupt Disable Register (chid = 19)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid19](cid19) module"]
pub type CID19 = crate::Reg<u32, _CID19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID19;
#[doc = "`write(|w| ..)` method takes [cid19::W](cid19::W) writer structure"]
impl crate::Writable for CID19 {}
#[doc = "Channel Interrupt Disable Register (chid = 19)"]
pub mod cid19;
#[doc = "Channel Interrupt Mask Register (chid = 19)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim19](cim19) module"]
pub type CIM19 = crate::Reg<u32, _CIM19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM19;
#[doc = "`write(|w| ..)` method takes [cim19::W](cim19::W) writer structure"]
impl crate::Writable for CIM19 {}
#[doc = "Channel Interrupt Mask Register (chid = 19)"]
pub mod cim19;
#[doc = "Channel Interrupt Status Register (chid = 19)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis19](cis19) module"]
pub type CIS19 = crate::Reg<u32, _CIS19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS19;
#[doc = "`read()` method returns [cis19::R](cis19::R) reader structure"]
impl crate::Readable for CIS19 {}
#[doc = "Channel Interrupt Status Register (chid = 19)"]
pub mod cis19;
#[doc = "Channel Source Address Register (chid = 19)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa19](csa19) module"]
pub type CSA19 = crate::Reg<u32, _CSA19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA19;
#[doc = "`read()` method returns [csa19::R](csa19::R) reader structure"]
impl crate::Readable for CSA19 {}
#[doc = "`write(|w| ..)` method takes [csa19::W](csa19::W) writer structure"]
impl crate::Writable for CSA19 {}
#[doc = "Channel Source Address Register (chid = 19)"]
pub mod csa19;
#[doc = "Channel Destination Address Register (chid = 19)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda19](cda19) module"]
pub type CDA19 = crate::Reg<u32, _CDA19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA19;
#[doc = "`read()` method returns [cda19::R](cda19::R) reader structure"]
impl crate::Readable for CDA19 {}
#[doc = "`write(|w| ..)` method takes [cda19::W](cda19::W) writer structure"]
impl crate::Writable for CDA19 {}
#[doc = "Channel Destination Address Register (chid = 19)"]
pub mod cda19;
#[doc = "Channel Next Descriptor Address Register (chid = 19)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda19](cnda19) module"]
pub type CNDA19 = crate::Reg<u32, _CNDA19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA19;
#[doc = "`read()` method returns [cnda19::R](cnda19::R) reader structure"]
impl crate::Readable for CNDA19 {}
#[doc = "`write(|w| ..)` method takes [cnda19::W](cnda19::W) writer structure"]
impl crate::Writable for CNDA19 {}
#[doc = "Channel Next Descriptor Address Register (chid = 19)"]
pub mod cnda19;
#[doc = "Channel Next Descriptor Control Register (chid = 19)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc19](cndc19) module"]
pub type CNDC19 = crate::Reg<u32, _CNDC19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC19;
#[doc = "`read()` method returns [cndc19::R](cndc19::R) reader structure"]
impl crate::Readable for CNDC19 {}
#[doc = "`write(|w| ..)` method takes [cndc19::W](cndc19::W) writer structure"]
impl crate::Writable for CNDC19 {}
#[doc = "Channel Next Descriptor Control Register (chid = 19)"]
pub mod cndc19;
#[doc = "Channel Microblock Control Register (chid = 19)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc19](cubc19) module"]
pub type CUBC19 = crate::Reg<u32, _CUBC19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC19;
#[doc = "`read()` method returns [cubc19::R](cubc19::R) reader structure"]
impl crate::Readable for CUBC19 {}
#[doc = "`write(|w| ..)` method takes [cubc19::W](cubc19::W) writer structure"]
impl crate::Writable for CUBC19 {}
#[doc = "Channel Microblock Control Register (chid = 19)"]
pub mod cubc19;
#[doc = "Channel Block Control Register (chid = 19)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc19](cbc19) module"]
pub type CBC19 = crate::Reg<u32, _CBC19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC19;
#[doc = "`read()` method returns [cbc19::R](cbc19::R) reader structure"]
impl crate::Readable for CBC19 {}
#[doc = "`write(|w| ..)` method takes [cbc19::W](cbc19::W) writer structure"]
impl crate::Writable for CBC19 {}
#[doc = "Channel Block Control Register (chid = 19)"]
pub mod cbc19;
#[doc = "Channel Configuration Register (chid = 19)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc19](cc19) module"]
pub type CC19 = crate::Reg<u32, _CC19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC19;
#[doc = "`read()` method returns [cc19::R](cc19::R) reader structure"]
impl crate::Readable for CC19 {}
#[doc = "`write(|w| ..)` method takes [cc19::W](cc19::W) writer structure"]
impl crate::Writable for CC19 {}
#[doc = "Channel Configuration Register (chid = 19)"]
pub mod cc19;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 19)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp19](cds_msp19) module"]
pub type CDS_MSP19 = crate::Reg<u32, _CDS_MSP19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP19;
#[doc = "`read()` method returns [cds_msp19::R](cds_msp19::R) reader structure"]
impl crate::Readable for CDS_MSP19 {}
#[doc = "`write(|w| ..)` method takes [cds_msp19::W](cds_msp19::W) writer structure"]
impl crate::Writable for CDS_MSP19 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 19)"]
pub mod cds_msp19;
#[doc = "Channel Source Microblock Stride (chid = 19)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus19](csus19) module"]
pub type CSUS19 = crate::Reg<u32, _CSUS19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS19;
#[doc = "`read()` method returns [csus19::R](csus19::R) reader structure"]
impl crate::Readable for CSUS19 {}
#[doc = "`write(|w| ..)` method takes [csus19::W](csus19::W) writer structure"]
impl crate::Writable for CSUS19 {}
#[doc = "Channel Source Microblock Stride (chid = 19)"]
pub mod csus19;
#[doc = "Channel Destination Microblock Stride (chid = 19)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus19](cdus19) module"]
pub type CDUS19 = crate::Reg<u32, _CDUS19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS19;
#[doc = "`read()` method returns [cdus19::R](cdus19::R) reader structure"]
impl crate::Readable for CDUS19 {}
#[doc = "`write(|w| ..)` method takes [cdus19::W](cdus19::W) writer structure"]
impl crate::Writable for CDUS19 {}
#[doc = "Channel Destination Microblock Stride (chid = 19)"]
pub mod cdus19;
#[doc = "Channel Interrupt Enable Register (chid = 20)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie20](cie20) module"]
pub type CIE20 = crate::Reg<u32, _CIE20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE20;
#[doc = "`write(|w| ..)` method takes [cie20::W](cie20::W) writer structure"]
impl crate::Writable for CIE20 {}
#[doc = "Channel Interrupt Enable Register (chid = 20)"]
pub mod cie20;
#[doc = "Channel Interrupt Disable Register (chid = 20)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid20](cid20) module"]
pub type CID20 = crate::Reg<u32, _CID20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID20;
#[doc = "`write(|w| ..)` method takes [cid20::W](cid20::W) writer structure"]
impl crate::Writable for CID20 {}
#[doc = "Channel Interrupt Disable Register (chid = 20)"]
pub mod cid20;
#[doc = "Channel Interrupt Mask Register (chid = 20)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim20](cim20) module"]
pub type CIM20 = crate::Reg<u32, _CIM20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM20;
#[doc = "`write(|w| ..)` method takes [cim20::W](cim20::W) writer structure"]
impl crate::Writable for CIM20 {}
#[doc = "Channel Interrupt Mask Register (chid = 20)"]
pub mod cim20;
#[doc = "Channel Interrupt Status Register (chid = 20)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis20](cis20) module"]
pub type CIS20 = crate::Reg<u32, _CIS20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS20;
#[doc = "`read()` method returns [cis20::R](cis20::R) reader structure"]
impl crate::Readable for CIS20 {}
#[doc = "Channel Interrupt Status Register (chid = 20)"]
pub mod cis20;
#[doc = "Channel Source Address Register (chid = 20)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa20](csa20) module"]
pub type CSA20 = crate::Reg<u32, _CSA20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA20;
#[doc = "`read()` method returns [csa20::R](csa20::R) reader structure"]
impl crate::Readable for CSA20 {}
#[doc = "`write(|w| ..)` method takes [csa20::W](csa20::W) writer structure"]
impl crate::Writable for CSA20 {}
#[doc = "Channel Source Address Register (chid = 20)"]
pub mod csa20;
#[doc = "Channel Destination Address Register (chid = 20)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda20](cda20) module"]
pub type CDA20 = crate::Reg<u32, _CDA20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA20;
#[doc = "`read()` method returns [cda20::R](cda20::R) reader structure"]
impl crate::Readable for CDA20 {}
#[doc = "`write(|w| ..)` method takes [cda20::W](cda20::W) writer structure"]
impl crate::Writable for CDA20 {}
#[doc = "Channel Destination Address Register (chid = 20)"]
pub mod cda20;
#[doc = "Channel Next Descriptor Address Register (chid = 20)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda20](cnda20) module"]
pub type CNDA20 = crate::Reg<u32, _CNDA20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA20;
#[doc = "`read()` method returns [cnda20::R](cnda20::R) reader structure"]
impl crate::Readable for CNDA20 {}
#[doc = "`write(|w| ..)` method takes [cnda20::W](cnda20::W) writer structure"]
impl crate::Writable for CNDA20 {}
#[doc = "Channel Next Descriptor Address Register (chid = 20)"]
pub mod cnda20;
#[doc = "Channel Next Descriptor Control Register (chid = 20)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc20](cndc20) module"]
pub type CNDC20 = crate::Reg<u32, _CNDC20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC20;
#[doc = "`read()` method returns [cndc20::R](cndc20::R) reader structure"]
impl crate::Readable for CNDC20 {}
#[doc = "`write(|w| ..)` method takes [cndc20::W](cndc20::W) writer structure"]
impl crate::Writable for CNDC20 {}
#[doc = "Channel Next Descriptor Control Register (chid = 20)"]
pub mod cndc20;
#[doc = "Channel Microblock Control Register (chid = 20)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc20](cubc20) module"]
pub type CUBC20 = crate::Reg<u32, _CUBC20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC20;
#[doc = "`read()` method returns [cubc20::R](cubc20::R) reader structure"]
impl crate::Readable for CUBC20 {}
#[doc = "`write(|w| ..)` method takes [cubc20::W](cubc20::W) writer structure"]
impl crate::Writable for CUBC20 {}
#[doc = "Channel Microblock Control Register (chid = 20)"]
pub mod cubc20;
#[doc = "Channel Block Control Register (chid = 20)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc20](cbc20) module"]
pub type CBC20 = crate::Reg<u32, _CBC20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC20;
#[doc = "`read()` method returns [cbc20::R](cbc20::R) reader structure"]
impl crate::Readable for CBC20 {}
#[doc = "`write(|w| ..)` method takes [cbc20::W](cbc20::W) writer structure"]
impl crate::Writable for CBC20 {}
#[doc = "Channel Block Control Register (chid = 20)"]
pub mod cbc20;
#[doc = "Channel Configuration Register (chid = 20)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc20](cc20) module"]
pub type CC20 = crate::Reg<u32, _CC20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC20;
#[doc = "`read()` method returns [cc20::R](cc20::R) reader structure"]
impl crate::Readable for CC20 {}
#[doc = "`write(|w| ..)` method takes [cc20::W](cc20::W) writer structure"]
impl crate::Writable for CC20 {}
#[doc = "Channel Configuration Register (chid = 20)"]
pub mod cc20;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 20)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp20](cds_msp20) module"]
pub type CDS_MSP20 = crate::Reg<u32, _CDS_MSP20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP20;
#[doc = "`read()` method returns [cds_msp20::R](cds_msp20::R) reader structure"]
impl crate::Readable for CDS_MSP20 {}
#[doc = "`write(|w| ..)` method takes [cds_msp20::W](cds_msp20::W) writer structure"]
impl crate::Writable for CDS_MSP20 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 20)"]
pub mod cds_msp20;
#[doc = "Channel Source Microblock Stride (chid = 20)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus20](csus20) module"]
pub type CSUS20 = crate::Reg<u32, _CSUS20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS20;
#[doc = "`read()` method returns [csus20::R](csus20::R) reader structure"]
impl crate::Readable for CSUS20 {}
#[doc = "`write(|w| ..)` method takes [csus20::W](csus20::W) writer structure"]
impl crate::Writable for CSUS20 {}
#[doc = "Channel Source Microblock Stride (chid = 20)"]
pub mod csus20;
#[doc = "Channel Destination Microblock Stride (chid = 20)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus20](cdus20) module"]
pub type CDUS20 = crate::Reg<u32, _CDUS20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS20;
#[doc = "`read()` method returns [cdus20::R](cdus20::R) reader structure"]
impl crate::Readable for CDUS20 {}
#[doc = "`write(|w| ..)` method takes [cdus20::W](cdus20::W) writer structure"]
impl crate::Writable for CDUS20 {}
#[doc = "Channel Destination Microblock Stride (chid = 20)"]
pub mod cdus20;
#[doc = "Channel Interrupt Enable Register (chid = 21)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie21](cie21) module"]
pub type CIE21 = crate::Reg<u32, _CIE21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE21;
#[doc = "`write(|w| ..)` method takes [cie21::W](cie21::W) writer structure"]
impl crate::Writable for CIE21 {}
#[doc = "Channel Interrupt Enable Register (chid = 21)"]
pub mod cie21;
#[doc = "Channel Interrupt Disable Register (chid = 21)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid21](cid21) module"]
pub type CID21 = crate::Reg<u32, _CID21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID21;
#[doc = "`write(|w| ..)` method takes [cid21::W](cid21::W) writer structure"]
impl crate::Writable for CID21 {}
#[doc = "Channel Interrupt Disable Register (chid = 21)"]
pub mod cid21;
#[doc = "Channel Interrupt Mask Register (chid = 21)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim21](cim21) module"]
pub type CIM21 = crate::Reg<u32, _CIM21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM21;
#[doc = "`write(|w| ..)` method takes [cim21::W](cim21::W) writer structure"]
impl crate::Writable for CIM21 {}
#[doc = "Channel Interrupt Mask Register (chid = 21)"]
pub mod cim21;
#[doc = "Channel Interrupt Status Register (chid = 21)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis21](cis21) module"]
pub type CIS21 = crate::Reg<u32, _CIS21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS21;
#[doc = "`read()` method returns [cis21::R](cis21::R) reader structure"]
impl crate::Readable for CIS21 {}
#[doc = "Channel Interrupt Status Register (chid = 21)"]
pub mod cis21;
#[doc = "Channel Source Address Register (chid = 21)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa21](csa21) module"]
pub type CSA21 = crate::Reg<u32, _CSA21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA21;
#[doc = "`read()` method returns [csa21::R](csa21::R) reader structure"]
impl crate::Readable for CSA21 {}
#[doc = "`write(|w| ..)` method takes [csa21::W](csa21::W) writer structure"]
impl crate::Writable for CSA21 {}
#[doc = "Channel Source Address Register (chid = 21)"]
pub mod csa21;
#[doc = "Channel Destination Address Register (chid = 21)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda21](cda21) module"]
pub type CDA21 = crate::Reg<u32, _CDA21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA21;
#[doc = "`read()` method returns [cda21::R](cda21::R) reader structure"]
impl crate::Readable for CDA21 {}
#[doc = "`write(|w| ..)` method takes [cda21::W](cda21::W) writer structure"]
impl crate::Writable for CDA21 {}
#[doc = "Channel Destination Address Register (chid = 21)"]
pub mod cda21;
#[doc = "Channel Next Descriptor Address Register (chid = 21)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda21](cnda21) module"]
pub type CNDA21 = crate::Reg<u32, _CNDA21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA21;
#[doc = "`read()` method returns [cnda21::R](cnda21::R) reader structure"]
impl crate::Readable for CNDA21 {}
#[doc = "`write(|w| ..)` method takes [cnda21::W](cnda21::W) writer structure"]
impl crate::Writable for CNDA21 {}
#[doc = "Channel Next Descriptor Address Register (chid = 21)"]
pub mod cnda21;
#[doc = "Channel Next Descriptor Control Register (chid = 21)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc21](cndc21) module"]
pub type CNDC21 = crate::Reg<u32, _CNDC21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC21;
#[doc = "`read()` method returns [cndc21::R](cndc21::R) reader structure"]
impl crate::Readable for CNDC21 {}
#[doc = "`write(|w| ..)` method takes [cndc21::W](cndc21::W) writer structure"]
impl crate::Writable for CNDC21 {}
#[doc = "Channel Next Descriptor Control Register (chid = 21)"]
pub mod cndc21;
#[doc = "Channel Microblock Control Register (chid = 21)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc21](cubc21) module"]
pub type CUBC21 = crate::Reg<u32, _CUBC21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC21;
#[doc = "`read()` method returns [cubc21::R](cubc21::R) reader structure"]
impl crate::Readable for CUBC21 {}
#[doc = "`write(|w| ..)` method takes [cubc21::W](cubc21::W) writer structure"]
impl crate::Writable for CUBC21 {}
#[doc = "Channel Microblock Control Register (chid = 21)"]
pub mod cubc21;
#[doc = "Channel Block Control Register (chid = 21)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc21](cbc21) module"]
pub type CBC21 = crate::Reg<u32, _CBC21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC21;
#[doc = "`read()` method returns [cbc21::R](cbc21::R) reader structure"]
impl crate::Readable for CBC21 {}
#[doc = "`write(|w| ..)` method takes [cbc21::W](cbc21::W) writer structure"]
impl crate::Writable for CBC21 {}
#[doc = "Channel Block Control Register (chid = 21)"]
pub mod cbc21;
#[doc = "Channel Configuration Register (chid = 21)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc21](cc21) module"]
pub type CC21 = crate::Reg<u32, _CC21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC21;
#[doc = "`read()` method returns [cc21::R](cc21::R) reader structure"]
impl crate::Readable for CC21 {}
#[doc = "`write(|w| ..)` method takes [cc21::W](cc21::W) writer structure"]
impl crate::Writable for CC21 {}
#[doc = "Channel Configuration Register (chid = 21)"]
pub mod cc21;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 21)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp21](cds_msp21) module"]
pub type CDS_MSP21 = crate::Reg<u32, _CDS_MSP21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP21;
#[doc = "`read()` method returns [cds_msp21::R](cds_msp21::R) reader structure"]
impl crate::Readable for CDS_MSP21 {}
#[doc = "`write(|w| ..)` method takes [cds_msp21::W](cds_msp21::W) writer structure"]
impl crate::Writable for CDS_MSP21 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 21)"]
pub mod cds_msp21;
#[doc = "Channel Source Microblock Stride (chid = 21)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus21](csus21) module"]
pub type CSUS21 = crate::Reg<u32, _CSUS21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS21;
#[doc = "`read()` method returns [csus21::R](csus21::R) reader structure"]
impl crate::Readable for CSUS21 {}
#[doc = "`write(|w| ..)` method takes [csus21::W](csus21::W) writer structure"]
impl crate::Writable for CSUS21 {}
#[doc = "Channel Source Microblock Stride (chid = 21)"]
pub mod csus21;
#[doc = "Channel Destination Microblock Stride (chid = 21)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus21](cdus21) module"]
pub type CDUS21 = crate::Reg<u32, _CDUS21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS21;
#[doc = "`read()` method returns [cdus21::R](cdus21::R) reader structure"]
impl crate::Readable for CDUS21 {}
#[doc = "`write(|w| ..)` method takes [cdus21::W](cdus21::W) writer structure"]
impl crate::Writable for CDUS21 {}
#[doc = "Channel Destination Microblock Stride (chid = 21)"]
pub mod cdus21;
#[doc = "Channel Interrupt Enable Register (chid = 22)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie22](cie22) module"]
pub type CIE22 = crate::Reg<u32, _CIE22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE22;
#[doc = "`write(|w| ..)` method takes [cie22::W](cie22::W) writer structure"]
impl crate::Writable for CIE22 {}
#[doc = "Channel Interrupt Enable Register (chid = 22)"]
pub mod cie22;
#[doc = "Channel Interrupt Disable Register (chid = 22)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid22](cid22) module"]
pub type CID22 = crate::Reg<u32, _CID22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID22;
#[doc = "`write(|w| ..)` method takes [cid22::W](cid22::W) writer structure"]
impl crate::Writable for CID22 {}
#[doc = "Channel Interrupt Disable Register (chid = 22)"]
pub mod cid22;
#[doc = "Channel Interrupt Mask Register (chid = 22)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim22](cim22) module"]
pub type CIM22 = crate::Reg<u32, _CIM22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM22;
#[doc = "`write(|w| ..)` method takes [cim22::W](cim22::W) writer structure"]
impl crate::Writable for CIM22 {}
#[doc = "Channel Interrupt Mask Register (chid = 22)"]
pub mod cim22;
#[doc = "Channel Interrupt Status Register (chid = 22)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis22](cis22) module"]
pub type CIS22 = crate::Reg<u32, _CIS22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS22;
#[doc = "`read()` method returns [cis22::R](cis22::R) reader structure"]
impl crate::Readable for CIS22 {}
#[doc = "Channel Interrupt Status Register (chid = 22)"]
pub mod cis22;
#[doc = "Channel Source Address Register (chid = 22)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa22](csa22) module"]
pub type CSA22 = crate::Reg<u32, _CSA22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA22;
#[doc = "`read()` method returns [csa22::R](csa22::R) reader structure"]
impl crate::Readable for CSA22 {}
#[doc = "`write(|w| ..)` method takes [csa22::W](csa22::W) writer structure"]
impl crate::Writable for CSA22 {}
#[doc = "Channel Source Address Register (chid = 22)"]
pub mod csa22;
#[doc = "Channel Destination Address Register (chid = 22)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda22](cda22) module"]
pub type CDA22 = crate::Reg<u32, _CDA22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA22;
#[doc = "`read()` method returns [cda22::R](cda22::R) reader structure"]
impl crate::Readable for CDA22 {}
#[doc = "`write(|w| ..)` method takes [cda22::W](cda22::W) writer structure"]
impl crate::Writable for CDA22 {}
#[doc = "Channel Destination Address Register (chid = 22)"]
pub mod cda22;
#[doc = "Channel Next Descriptor Address Register (chid = 22)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda22](cnda22) module"]
pub type CNDA22 = crate::Reg<u32, _CNDA22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA22;
#[doc = "`read()` method returns [cnda22::R](cnda22::R) reader structure"]
impl crate::Readable for CNDA22 {}
#[doc = "`write(|w| ..)` method takes [cnda22::W](cnda22::W) writer structure"]
impl crate::Writable for CNDA22 {}
#[doc = "Channel Next Descriptor Address Register (chid = 22)"]
pub mod cnda22;
#[doc = "Channel Next Descriptor Control Register (chid = 22)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc22](cndc22) module"]
pub type CNDC22 = crate::Reg<u32, _CNDC22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC22;
#[doc = "`read()` method returns [cndc22::R](cndc22::R) reader structure"]
impl crate::Readable for CNDC22 {}
#[doc = "`write(|w| ..)` method takes [cndc22::W](cndc22::W) writer structure"]
impl crate::Writable for CNDC22 {}
#[doc = "Channel Next Descriptor Control Register (chid = 22)"]
pub mod cndc22;
#[doc = "Channel Microblock Control Register (chid = 22)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc22](cubc22) module"]
pub type CUBC22 = crate::Reg<u32, _CUBC22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC22;
#[doc = "`read()` method returns [cubc22::R](cubc22::R) reader structure"]
impl crate::Readable for CUBC22 {}
#[doc = "`write(|w| ..)` method takes [cubc22::W](cubc22::W) writer structure"]
impl crate::Writable for CUBC22 {}
#[doc = "Channel Microblock Control Register (chid = 22)"]
pub mod cubc22;
#[doc = "Channel Block Control Register (chid = 22)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc22](cbc22) module"]
pub type CBC22 = crate::Reg<u32, _CBC22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC22;
#[doc = "`read()` method returns [cbc22::R](cbc22::R) reader structure"]
impl crate::Readable for CBC22 {}
#[doc = "`write(|w| ..)` method takes [cbc22::W](cbc22::W) writer structure"]
impl crate::Writable for CBC22 {}
#[doc = "Channel Block Control Register (chid = 22)"]
pub mod cbc22;
#[doc = "Channel Configuration Register (chid = 22)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc22](cc22) module"]
pub type CC22 = crate::Reg<u32, _CC22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC22;
#[doc = "`read()` method returns [cc22::R](cc22::R) reader structure"]
impl crate::Readable for CC22 {}
#[doc = "`write(|w| ..)` method takes [cc22::W](cc22::W) writer structure"]
impl crate::Writable for CC22 {}
#[doc = "Channel Configuration Register (chid = 22)"]
pub mod cc22;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 22)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp22](cds_msp22) module"]
pub type CDS_MSP22 = crate::Reg<u32, _CDS_MSP22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP22;
#[doc = "`read()` method returns [cds_msp22::R](cds_msp22::R) reader structure"]
impl crate::Readable for CDS_MSP22 {}
#[doc = "`write(|w| ..)` method takes [cds_msp22::W](cds_msp22::W) writer structure"]
impl crate::Writable for CDS_MSP22 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 22)"]
pub mod cds_msp22;
#[doc = "Channel Source Microblock Stride (chid = 22)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus22](csus22) module"]
pub type CSUS22 = crate::Reg<u32, _CSUS22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS22;
#[doc = "`read()` method returns [csus22::R](csus22::R) reader structure"]
impl crate::Readable for CSUS22 {}
#[doc = "`write(|w| ..)` method takes [csus22::W](csus22::W) writer structure"]
impl crate::Writable for CSUS22 {}
#[doc = "Channel Source Microblock Stride (chid = 22)"]
pub mod csus22;
#[doc = "Channel Destination Microblock Stride (chid = 22)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus22](cdus22) module"]
pub type CDUS22 = crate::Reg<u32, _CDUS22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS22;
#[doc = "`read()` method returns [cdus22::R](cdus22::R) reader structure"]
impl crate::Readable for CDUS22 {}
#[doc = "`write(|w| ..)` method takes [cdus22::W](cdus22::W) writer structure"]
impl crate::Writable for CDUS22 {}
#[doc = "Channel Destination Microblock Stride (chid = 22)"]
pub mod cdus22;
#[doc = "Channel Interrupt Enable Register (chid = 23)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie23](cie23) module"]
pub type CIE23 = crate::Reg<u32, _CIE23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE23;
#[doc = "`write(|w| ..)` method takes [cie23::W](cie23::W) writer structure"]
impl crate::Writable for CIE23 {}
#[doc = "Channel Interrupt Enable Register (chid = 23)"]
pub mod cie23;
#[doc = "Channel Interrupt Disable Register (chid = 23)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid23](cid23) module"]
pub type CID23 = crate::Reg<u32, _CID23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID23;
#[doc = "`write(|w| ..)` method takes [cid23::W](cid23::W) writer structure"]
impl crate::Writable for CID23 {}
#[doc = "Channel Interrupt Disable Register (chid = 23)"]
pub mod cid23;
#[doc = "Channel Interrupt Mask Register (chid = 23)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim23](cim23) module"]
pub type CIM23 = crate::Reg<u32, _CIM23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM23;
#[doc = "`write(|w| ..)` method takes [cim23::W](cim23::W) writer structure"]
impl crate::Writable for CIM23 {}
#[doc = "Channel Interrupt Mask Register (chid = 23)"]
pub mod cim23;
#[doc = "Channel Interrupt Status Register (chid = 23)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis23](cis23) module"]
pub type CIS23 = crate::Reg<u32, _CIS23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS23;
#[doc = "`read()` method returns [cis23::R](cis23::R) reader structure"]
impl crate::Readable for CIS23 {}
#[doc = "Channel Interrupt Status Register (chid = 23)"]
pub mod cis23;
#[doc = "Channel Source Address Register (chid = 23)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa23](csa23) module"]
pub type CSA23 = crate::Reg<u32, _CSA23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA23;
#[doc = "`read()` method returns [csa23::R](csa23::R) reader structure"]
impl crate::Readable for CSA23 {}
#[doc = "`write(|w| ..)` method takes [csa23::W](csa23::W) writer structure"]
impl crate::Writable for CSA23 {}
#[doc = "Channel Source Address Register (chid = 23)"]
pub mod csa23;
#[doc = "Channel Destination Address Register (chid = 23)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda23](cda23) module"]
pub type CDA23 = crate::Reg<u32, _CDA23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA23;
#[doc = "`read()` method returns [cda23::R](cda23::R) reader structure"]
impl crate::Readable for CDA23 {}
#[doc = "`write(|w| ..)` method takes [cda23::W](cda23::W) writer structure"]
impl crate::Writable for CDA23 {}
#[doc = "Channel Destination Address Register (chid = 23)"]
pub mod cda23;
#[doc = "Channel Next Descriptor Address Register (chid = 23)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda23](cnda23) module"]
pub type CNDA23 = crate::Reg<u32, _CNDA23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA23;
#[doc = "`read()` method returns [cnda23::R](cnda23::R) reader structure"]
impl crate::Readable for CNDA23 {}
#[doc = "`write(|w| ..)` method takes [cnda23::W](cnda23::W) writer structure"]
impl crate::Writable for CNDA23 {}
#[doc = "Channel Next Descriptor Address Register (chid = 23)"]
pub mod cnda23;
#[doc = "Channel Next Descriptor Control Register (chid = 23)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc23](cndc23) module"]
pub type CNDC23 = crate::Reg<u32, _CNDC23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC23;
#[doc = "`read()` method returns [cndc23::R](cndc23::R) reader structure"]
impl crate::Readable for CNDC23 {}
#[doc = "`write(|w| ..)` method takes [cndc23::W](cndc23::W) writer structure"]
impl crate::Writable for CNDC23 {}
#[doc = "Channel Next Descriptor Control Register (chid = 23)"]
pub mod cndc23;
#[doc = "Channel Microblock Control Register (chid = 23)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc23](cubc23) module"]
pub type CUBC23 = crate::Reg<u32, _CUBC23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC23;
#[doc = "`read()` method returns [cubc23::R](cubc23::R) reader structure"]
impl crate::Readable for CUBC23 {}
#[doc = "`write(|w| ..)` method takes [cubc23::W](cubc23::W) writer structure"]
impl crate::Writable for CUBC23 {}
#[doc = "Channel Microblock Control Register (chid = 23)"]
pub mod cubc23;
#[doc = "Channel Block Control Register (chid = 23)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc23](cbc23) module"]
pub type CBC23 = crate::Reg<u32, _CBC23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC23;
#[doc = "`read()` method returns [cbc23::R](cbc23::R) reader structure"]
impl crate::Readable for CBC23 {}
#[doc = "`write(|w| ..)` method takes [cbc23::W](cbc23::W) writer structure"]
impl crate::Writable for CBC23 {}
#[doc = "Channel Block Control Register (chid = 23)"]
pub mod cbc23;
#[doc = "Channel Configuration Register (chid = 23)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc23](cc23) module"]
pub type CC23 = crate::Reg<u32, _CC23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC23;
#[doc = "`read()` method returns [cc23::R](cc23::R) reader structure"]
impl crate::Readable for CC23 {}
#[doc = "`write(|w| ..)` method takes [cc23::W](cc23::W) writer structure"]
impl crate::Writable for CC23 {}
#[doc = "Channel Configuration Register (chid = 23)"]
pub mod cc23;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 23)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp23](cds_msp23) module"]
pub type CDS_MSP23 = crate::Reg<u32, _CDS_MSP23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP23;
#[doc = "`read()` method returns [cds_msp23::R](cds_msp23::R) reader structure"]
impl crate::Readable for CDS_MSP23 {}
#[doc = "`write(|w| ..)` method takes [cds_msp23::W](cds_msp23::W) writer structure"]
impl crate::Writable for CDS_MSP23 {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 23)"]
pub mod cds_msp23;
#[doc = "Channel Source Microblock Stride (chid = 23)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus23](csus23) module"]
pub type CSUS23 = crate::Reg<u32, _CSUS23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS23;
#[doc = "`read()` method returns [csus23::R](csus23::R) reader structure"]
impl crate::Readable for CSUS23 {}
#[doc = "`write(|w| ..)` method takes [csus23::W](csus23::W) writer structure"]
impl crate::Writable for CSUS23 {}
#[doc = "Channel Source Microblock Stride (chid = 23)"]
pub mod csus23;
#[doc = "Channel Destination Microblock Stride (chid = 23)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus23](cdus23) module"]
pub type CDUS23 = crate::Reg<u32, _CDUS23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS23;
#[doc = "`read()` method returns [cdus23::R](cdus23::R) reader structure"]
impl crate::Readable for CDUS23 {}
#[doc = "`write(|w| ..)` method takes [cdus23::W](cdus23::W) writer structure"]
impl crate::Writable for CDUS23 {}
#[doc = "Channel Destination Microblock Stride (chid = 23)"]
pub mod cdus23;
