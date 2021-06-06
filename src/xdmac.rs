#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global Type Register"]
    pub gtype: crate::Reg<gtype::GTYPE_SPEC>,
    #[doc = "0x04 - Global Configuration Register"]
    pub gcfg: crate::Reg<gcfg::GCFG_SPEC>,
    #[doc = "0x08 - Global Weighted Arbiter Configuration Register"]
    pub gwac: crate::Reg<gwac::GWAC_SPEC>,
    #[doc = "0x0c - Global Interrupt Enable Register"]
    pub gie: crate::Reg<gie::GIE_SPEC>,
    #[doc = "0x10 - Global Interrupt Disable Register"]
    pub gid: crate::Reg<gid::GID_SPEC>,
    #[doc = "0x14 - Global Interrupt Mask Register"]
    pub gim: crate::Reg<gim::GIM_SPEC>,
    #[doc = "0x18 - Global Interrupt Status Register"]
    pub gis: crate::Reg<gis::GIS_SPEC>,
    #[doc = "0x1c - Global Channel Enable Register"]
    pub ge: crate::Reg<ge::GE_SPEC>,
    #[doc = "0x20 - Global Channel Disable Register"]
    pub gd: crate::Reg<gd::GD_SPEC>,
    #[doc = "0x24 - Global Channel Status Register"]
    pub gs: crate::Reg<gs::GS_SPEC>,
    #[doc = "0x28 - Global Channel Read Suspend Register"]
    pub grs: crate::Reg<grs::GRS_SPEC>,
    #[doc = "0x2c - Global Channel Write Suspend Register"]
    pub gws: crate::Reg<gws::GWS_SPEC>,
    #[doc = "0x30 - Global Channel Read Write Suspend Register"]
    pub grws: crate::Reg<grws::GRWS_SPEC>,
    #[doc = "0x34 - Global Channel Read Write Resume Register"]
    pub grwr: crate::Reg<grwr::GRWR_SPEC>,
    #[doc = "0x38 - Global Channel Software Request Register"]
    pub gswr: crate::Reg<gswr::GSWR_SPEC>,
    #[doc = "0x3c - Global Channel Software Request Status Register"]
    pub gsws: crate::Reg<gsws::GSWS_SPEC>,
    #[doc = "0x40 - Global Channel Software Flush Request Register"]
    pub gswf: crate::Reg<gswf::GSWF_SPEC>,
    _reserved17: [u8; 12usize],
    #[doc = "0x50 - Channel Interrupt Enable Register (chid = 0)"]
    pub cie0: crate::Reg<cie0::CIE0_SPEC>,
    #[doc = "0x54 - Channel Interrupt Disable Register (chid = 0)"]
    pub cid0: crate::Reg<cid0::CID0_SPEC>,
    #[doc = "0x58 - Channel Interrupt Mask Register (chid = 0)"]
    pub cim0: crate::Reg<cim0::CIM0_SPEC>,
    #[doc = "0x5c - Channel Interrupt Status Register (chid = 0)"]
    pub cis0: crate::Reg<cis0::CIS0_SPEC>,
    #[doc = "0x60 - Channel Source Address Register (chid = 0)"]
    pub csa0: crate::Reg<csa0::CSA0_SPEC>,
    #[doc = "0x64 - Channel Destination Address Register (chid = 0)"]
    pub cda0: crate::Reg<cda0::CDA0_SPEC>,
    #[doc = "0x68 - Channel Next Descriptor Address Register (chid = 0)"]
    pub cnda0: crate::Reg<cnda0::CNDA0_SPEC>,
    #[doc = "0x6c - Channel Next Descriptor Control Register (chid = 0)"]
    pub cndc0: crate::Reg<cndc0::CNDC0_SPEC>,
    #[doc = "0x70 - Channel Microblock Control Register (chid = 0)"]
    pub cubc0: crate::Reg<cubc0::CUBC0_SPEC>,
    #[doc = "0x74 - Channel Block Control Register (chid = 0)"]
    pub cbc0: crate::Reg<cbc0::CBC0_SPEC>,
    #[doc = "0x78 - Channel Configuration Register (chid = 0)"]
    pub cc0: crate::Reg<cc0::CC0_SPEC>,
    #[doc = "0x7c - Channel Data Stride Memory Set Pattern (chid = 0)"]
    pub cds_msp0: crate::Reg<cds_msp0::CDS_MSP0_SPEC>,
    #[doc = "0x80 - Channel Source Microblock Stride (chid = 0)"]
    pub csus0: crate::Reg<csus0::CSUS0_SPEC>,
    #[doc = "0x84 - Channel Destination Microblock Stride (chid = 0)"]
    pub cdus0: crate::Reg<cdus0::CDUS0_SPEC>,
    _reserved31: [u8; 8usize],
    #[doc = "0x90 - Channel Interrupt Enable Register (chid = 1)"]
    pub cie1: crate::Reg<cie1::CIE1_SPEC>,
    #[doc = "0x94 - Channel Interrupt Disable Register (chid = 1)"]
    pub cid1: crate::Reg<cid1::CID1_SPEC>,
    #[doc = "0x98 - Channel Interrupt Mask Register (chid = 1)"]
    pub cim1: crate::Reg<cim1::CIM1_SPEC>,
    #[doc = "0x9c - Channel Interrupt Status Register (chid = 1)"]
    pub cis1: crate::Reg<cis1::CIS1_SPEC>,
    #[doc = "0xa0 - Channel Source Address Register (chid = 1)"]
    pub csa1: crate::Reg<csa1::CSA1_SPEC>,
    #[doc = "0xa4 - Channel Destination Address Register (chid = 1)"]
    pub cda1: crate::Reg<cda1::CDA1_SPEC>,
    #[doc = "0xa8 - Channel Next Descriptor Address Register (chid = 1)"]
    pub cnda1: crate::Reg<cnda1::CNDA1_SPEC>,
    #[doc = "0xac - Channel Next Descriptor Control Register (chid = 1)"]
    pub cndc1: crate::Reg<cndc1::CNDC1_SPEC>,
    #[doc = "0xb0 - Channel Microblock Control Register (chid = 1)"]
    pub cubc1: crate::Reg<cubc1::CUBC1_SPEC>,
    #[doc = "0xb4 - Channel Block Control Register (chid = 1)"]
    pub cbc1: crate::Reg<cbc1::CBC1_SPEC>,
    #[doc = "0xb8 - Channel Configuration Register (chid = 1)"]
    pub cc1: crate::Reg<cc1::CC1_SPEC>,
    #[doc = "0xbc - Channel Data Stride Memory Set Pattern (chid = 1)"]
    pub cds_msp1: crate::Reg<cds_msp1::CDS_MSP1_SPEC>,
    #[doc = "0xc0 - Channel Source Microblock Stride (chid = 1)"]
    pub csus1: crate::Reg<csus1::CSUS1_SPEC>,
    #[doc = "0xc4 - Channel Destination Microblock Stride (chid = 1)"]
    pub cdus1: crate::Reg<cdus1::CDUS1_SPEC>,
    _reserved45: [u8; 8usize],
    #[doc = "0xd0 - Channel Interrupt Enable Register (chid = 2)"]
    pub cie2: crate::Reg<cie2::CIE2_SPEC>,
    #[doc = "0xd4 - Channel Interrupt Disable Register (chid = 2)"]
    pub cid2: crate::Reg<cid2::CID2_SPEC>,
    #[doc = "0xd8 - Channel Interrupt Mask Register (chid = 2)"]
    pub cim2: crate::Reg<cim2::CIM2_SPEC>,
    #[doc = "0xdc - Channel Interrupt Status Register (chid = 2)"]
    pub cis2: crate::Reg<cis2::CIS2_SPEC>,
    #[doc = "0xe0 - Channel Source Address Register (chid = 2)"]
    pub csa2: crate::Reg<csa2::CSA2_SPEC>,
    #[doc = "0xe4 - Channel Destination Address Register (chid = 2)"]
    pub cda2: crate::Reg<cda2::CDA2_SPEC>,
    #[doc = "0xe8 - Channel Next Descriptor Address Register (chid = 2)"]
    pub cnda2: crate::Reg<cnda2::CNDA2_SPEC>,
    #[doc = "0xec - Channel Next Descriptor Control Register (chid = 2)"]
    pub cndc2: crate::Reg<cndc2::CNDC2_SPEC>,
    #[doc = "0xf0 - Channel Microblock Control Register (chid = 2)"]
    pub cubc2: crate::Reg<cubc2::CUBC2_SPEC>,
    #[doc = "0xf4 - Channel Block Control Register (chid = 2)"]
    pub cbc2: crate::Reg<cbc2::CBC2_SPEC>,
    #[doc = "0xf8 - Channel Configuration Register (chid = 2)"]
    pub cc2: crate::Reg<cc2::CC2_SPEC>,
    #[doc = "0xfc - Channel Data Stride Memory Set Pattern (chid = 2)"]
    pub cds_msp2: crate::Reg<cds_msp2::CDS_MSP2_SPEC>,
    #[doc = "0x100 - Channel Source Microblock Stride (chid = 2)"]
    pub csus2: crate::Reg<csus2::CSUS2_SPEC>,
    #[doc = "0x104 - Channel Destination Microblock Stride (chid = 2)"]
    pub cdus2: crate::Reg<cdus2::CDUS2_SPEC>,
    _reserved59: [u8; 8usize],
    #[doc = "0x110 - Channel Interrupt Enable Register (chid = 3)"]
    pub cie3: crate::Reg<cie3::CIE3_SPEC>,
    #[doc = "0x114 - Channel Interrupt Disable Register (chid = 3)"]
    pub cid3: crate::Reg<cid3::CID3_SPEC>,
    #[doc = "0x118 - Channel Interrupt Mask Register (chid = 3)"]
    pub cim3: crate::Reg<cim3::CIM3_SPEC>,
    #[doc = "0x11c - Channel Interrupt Status Register (chid = 3)"]
    pub cis3: crate::Reg<cis3::CIS3_SPEC>,
    #[doc = "0x120 - Channel Source Address Register (chid = 3)"]
    pub csa3: crate::Reg<csa3::CSA3_SPEC>,
    #[doc = "0x124 - Channel Destination Address Register (chid = 3)"]
    pub cda3: crate::Reg<cda3::CDA3_SPEC>,
    #[doc = "0x128 - Channel Next Descriptor Address Register (chid = 3)"]
    pub cnda3: crate::Reg<cnda3::CNDA3_SPEC>,
    #[doc = "0x12c - Channel Next Descriptor Control Register (chid = 3)"]
    pub cndc3: crate::Reg<cndc3::CNDC3_SPEC>,
    #[doc = "0x130 - Channel Microblock Control Register (chid = 3)"]
    pub cubc3: crate::Reg<cubc3::CUBC3_SPEC>,
    #[doc = "0x134 - Channel Block Control Register (chid = 3)"]
    pub cbc3: crate::Reg<cbc3::CBC3_SPEC>,
    #[doc = "0x138 - Channel Configuration Register (chid = 3)"]
    pub cc3: crate::Reg<cc3::CC3_SPEC>,
    #[doc = "0x13c - Channel Data Stride Memory Set Pattern (chid = 3)"]
    pub cds_msp3: crate::Reg<cds_msp3::CDS_MSP3_SPEC>,
    #[doc = "0x140 - Channel Source Microblock Stride (chid = 3)"]
    pub csus3: crate::Reg<csus3::CSUS3_SPEC>,
    #[doc = "0x144 - Channel Destination Microblock Stride (chid = 3)"]
    pub cdus3: crate::Reg<cdus3::CDUS3_SPEC>,
    _reserved73: [u8; 8usize],
    #[doc = "0x150 - Channel Interrupt Enable Register (chid = 4)"]
    pub cie4: crate::Reg<cie4::CIE4_SPEC>,
    #[doc = "0x154 - Channel Interrupt Disable Register (chid = 4)"]
    pub cid4: crate::Reg<cid4::CID4_SPEC>,
    #[doc = "0x158 - Channel Interrupt Mask Register (chid = 4)"]
    pub cim4: crate::Reg<cim4::CIM4_SPEC>,
    #[doc = "0x15c - Channel Interrupt Status Register (chid = 4)"]
    pub cis4: crate::Reg<cis4::CIS4_SPEC>,
    #[doc = "0x160 - Channel Source Address Register (chid = 4)"]
    pub csa4: crate::Reg<csa4::CSA4_SPEC>,
    #[doc = "0x164 - Channel Destination Address Register (chid = 4)"]
    pub cda4: crate::Reg<cda4::CDA4_SPEC>,
    #[doc = "0x168 - Channel Next Descriptor Address Register (chid = 4)"]
    pub cnda4: crate::Reg<cnda4::CNDA4_SPEC>,
    #[doc = "0x16c - Channel Next Descriptor Control Register (chid = 4)"]
    pub cndc4: crate::Reg<cndc4::CNDC4_SPEC>,
    #[doc = "0x170 - Channel Microblock Control Register (chid = 4)"]
    pub cubc4: crate::Reg<cubc4::CUBC4_SPEC>,
    #[doc = "0x174 - Channel Block Control Register (chid = 4)"]
    pub cbc4: crate::Reg<cbc4::CBC4_SPEC>,
    #[doc = "0x178 - Channel Configuration Register (chid = 4)"]
    pub cc4: crate::Reg<cc4::CC4_SPEC>,
    #[doc = "0x17c - Channel Data Stride Memory Set Pattern (chid = 4)"]
    pub cds_msp4: crate::Reg<cds_msp4::CDS_MSP4_SPEC>,
    #[doc = "0x180 - Channel Source Microblock Stride (chid = 4)"]
    pub csus4: crate::Reg<csus4::CSUS4_SPEC>,
    #[doc = "0x184 - Channel Destination Microblock Stride (chid = 4)"]
    pub cdus4: crate::Reg<cdus4::CDUS4_SPEC>,
    _reserved87: [u8; 8usize],
    #[doc = "0x190 - Channel Interrupt Enable Register (chid = 5)"]
    pub cie5: crate::Reg<cie5::CIE5_SPEC>,
    #[doc = "0x194 - Channel Interrupt Disable Register (chid = 5)"]
    pub cid5: crate::Reg<cid5::CID5_SPEC>,
    #[doc = "0x198 - Channel Interrupt Mask Register (chid = 5)"]
    pub cim5: crate::Reg<cim5::CIM5_SPEC>,
    #[doc = "0x19c - Channel Interrupt Status Register (chid = 5)"]
    pub cis5: crate::Reg<cis5::CIS5_SPEC>,
    #[doc = "0x1a0 - Channel Source Address Register (chid = 5)"]
    pub csa5: crate::Reg<csa5::CSA5_SPEC>,
    #[doc = "0x1a4 - Channel Destination Address Register (chid = 5)"]
    pub cda5: crate::Reg<cda5::CDA5_SPEC>,
    #[doc = "0x1a8 - Channel Next Descriptor Address Register (chid = 5)"]
    pub cnda5: crate::Reg<cnda5::CNDA5_SPEC>,
    #[doc = "0x1ac - Channel Next Descriptor Control Register (chid = 5)"]
    pub cndc5: crate::Reg<cndc5::CNDC5_SPEC>,
    #[doc = "0x1b0 - Channel Microblock Control Register (chid = 5)"]
    pub cubc5: crate::Reg<cubc5::CUBC5_SPEC>,
    #[doc = "0x1b4 - Channel Block Control Register (chid = 5)"]
    pub cbc5: crate::Reg<cbc5::CBC5_SPEC>,
    #[doc = "0x1b8 - Channel Configuration Register (chid = 5)"]
    pub cc5: crate::Reg<cc5::CC5_SPEC>,
    #[doc = "0x1bc - Channel Data Stride Memory Set Pattern (chid = 5)"]
    pub cds_msp5: crate::Reg<cds_msp5::CDS_MSP5_SPEC>,
    #[doc = "0x1c0 - Channel Source Microblock Stride (chid = 5)"]
    pub csus5: crate::Reg<csus5::CSUS5_SPEC>,
    #[doc = "0x1c4 - Channel Destination Microblock Stride (chid = 5)"]
    pub cdus5: crate::Reg<cdus5::CDUS5_SPEC>,
    _reserved101: [u8; 8usize],
    #[doc = "0x1d0 - Channel Interrupt Enable Register (chid = 6)"]
    pub cie6: crate::Reg<cie6::CIE6_SPEC>,
    #[doc = "0x1d4 - Channel Interrupt Disable Register (chid = 6)"]
    pub cid6: crate::Reg<cid6::CID6_SPEC>,
    #[doc = "0x1d8 - Channel Interrupt Mask Register (chid = 6)"]
    pub cim6: crate::Reg<cim6::CIM6_SPEC>,
    #[doc = "0x1dc - Channel Interrupt Status Register (chid = 6)"]
    pub cis6: crate::Reg<cis6::CIS6_SPEC>,
    #[doc = "0x1e0 - Channel Source Address Register (chid = 6)"]
    pub csa6: crate::Reg<csa6::CSA6_SPEC>,
    #[doc = "0x1e4 - Channel Destination Address Register (chid = 6)"]
    pub cda6: crate::Reg<cda6::CDA6_SPEC>,
    #[doc = "0x1e8 - Channel Next Descriptor Address Register (chid = 6)"]
    pub cnda6: crate::Reg<cnda6::CNDA6_SPEC>,
    #[doc = "0x1ec - Channel Next Descriptor Control Register (chid = 6)"]
    pub cndc6: crate::Reg<cndc6::CNDC6_SPEC>,
    #[doc = "0x1f0 - Channel Microblock Control Register (chid = 6)"]
    pub cubc6: crate::Reg<cubc6::CUBC6_SPEC>,
    #[doc = "0x1f4 - Channel Block Control Register (chid = 6)"]
    pub cbc6: crate::Reg<cbc6::CBC6_SPEC>,
    #[doc = "0x1f8 - Channel Configuration Register (chid = 6)"]
    pub cc6: crate::Reg<cc6::CC6_SPEC>,
    #[doc = "0x1fc - Channel Data Stride Memory Set Pattern (chid = 6)"]
    pub cds_msp6: crate::Reg<cds_msp6::CDS_MSP6_SPEC>,
    #[doc = "0x200 - Channel Source Microblock Stride (chid = 6)"]
    pub csus6: crate::Reg<csus6::CSUS6_SPEC>,
    #[doc = "0x204 - Channel Destination Microblock Stride (chid = 6)"]
    pub cdus6: crate::Reg<cdus6::CDUS6_SPEC>,
    _reserved115: [u8; 8usize],
    #[doc = "0x210 - Channel Interrupt Enable Register (chid = 7)"]
    pub cie7: crate::Reg<cie7::CIE7_SPEC>,
    #[doc = "0x214 - Channel Interrupt Disable Register (chid = 7)"]
    pub cid7: crate::Reg<cid7::CID7_SPEC>,
    #[doc = "0x218 - Channel Interrupt Mask Register (chid = 7)"]
    pub cim7: crate::Reg<cim7::CIM7_SPEC>,
    #[doc = "0x21c - Channel Interrupt Status Register (chid = 7)"]
    pub cis7: crate::Reg<cis7::CIS7_SPEC>,
    #[doc = "0x220 - Channel Source Address Register (chid = 7)"]
    pub csa7: crate::Reg<csa7::CSA7_SPEC>,
    #[doc = "0x224 - Channel Destination Address Register (chid = 7)"]
    pub cda7: crate::Reg<cda7::CDA7_SPEC>,
    #[doc = "0x228 - Channel Next Descriptor Address Register (chid = 7)"]
    pub cnda7: crate::Reg<cnda7::CNDA7_SPEC>,
    #[doc = "0x22c - Channel Next Descriptor Control Register (chid = 7)"]
    pub cndc7: crate::Reg<cndc7::CNDC7_SPEC>,
    #[doc = "0x230 - Channel Microblock Control Register (chid = 7)"]
    pub cubc7: crate::Reg<cubc7::CUBC7_SPEC>,
    #[doc = "0x234 - Channel Block Control Register (chid = 7)"]
    pub cbc7: crate::Reg<cbc7::CBC7_SPEC>,
    #[doc = "0x238 - Channel Configuration Register (chid = 7)"]
    pub cc7: crate::Reg<cc7::CC7_SPEC>,
    #[doc = "0x23c - Channel Data Stride Memory Set Pattern (chid = 7)"]
    pub cds_msp7: crate::Reg<cds_msp7::CDS_MSP7_SPEC>,
    #[doc = "0x240 - Channel Source Microblock Stride (chid = 7)"]
    pub csus7: crate::Reg<csus7::CSUS7_SPEC>,
    #[doc = "0x244 - Channel Destination Microblock Stride (chid = 7)"]
    pub cdus7: crate::Reg<cdus7::CDUS7_SPEC>,
    _reserved129: [u8; 8usize],
    #[doc = "0x250 - Channel Interrupt Enable Register (chid = 8)"]
    pub cie8: crate::Reg<cie8::CIE8_SPEC>,
    #[doc = "0x254 - Channel Interrupt Disable Register (chid = 8)"]
    pub cid8: crate::Reg<cid8::CID8_SPEC>,
    #[doc = "0x258 - Channel Interrupt Mask Register (chid = 8)"]
    pub cim8: crate::Reg<cim8::CIM8_SPEC>,
    #[doc = "0x25c - Channel Interrupt Status Register (chid = 8)"]
    pub cis8: crate::Reg<cis8::CIS8_SPEC>,
    #[doc = "0x260 - Channel Source Address Register (chid = 8)"]
    pub csa8: crate::Reg<csa8::CSA8_SPEC>,
    #[doc = "0x264 - Channel Destination Address Register (chid = 8)"]
    pub cda8: crate::Reg<cda8::CDA8_SPEC>,
    #[doc = "0x268 - Channel Next Descriptor Address Register (chid = 8)"]
    pub cnda8: crate::Reg<cnda8::CNDA8_SPEC>,
    #[doc = "0x26c - Channel Next Descriptor Control Register (chid = 8)"]
    pub cndc8: crate::Reg<cndc8::CNDC8_SPEC>,
    #[doc = "0x270 - Channel Microblock Control Register (chid = 8)"]
    pub cubc8: crate::Reg<cubc8::CUBC8_SPEC>,
    #[doc = "0x274 - Channel Block Control Register (chid = 8)"]
    pub cbc8: crate::Reg<cbc8::CBC8_SPEC>,
    #[doc = "0x278 - Channel Configuration Register (chid = 8)"]
    pub cc8: crate::Reg<cc8::CC8_SPEC>,
    #[doc = "0x27c - Channel Data Stride Memory Set Pattern (chid = 8)"]
    pub cds_msp8: crate::Reg<cds_msp8::CDS_MSP8_SPEC>,
    #[doc = "0x280 - Channel Source Microblock Stride (chid = 8)"]
    pub csus8: crate::Reg<csus8::CSUS8_SPEC>,
    #[doc = "0x284 - Channel Destination Microblock Stride (chid = 8)"]
    pub cdus8: crate::Reg<cdus8::CDUS8_SPEC>,
    _reserved143: [u8; 8usize],
    #[doc = "0x290 - Channel Interrupt Enable Register (chid = 9)"]
    pub cie9: crate::Reg<cie9::CIE9_SPEC>,
    #[doc = "0x294 - Channel Interrupt Disable Register (chid = 9)"]
    pub cid9: crate::Reg<cid9::CID9_SPEC>,
    #[doc = "0x298 - Channel Interrupt Mask Register (chid = 9)"]
    pub cim9: crate::Reg<cim9::CIM9_SPEC>,
    #[doc = "0x29c - Channel Interrupt Status Register (chid = 9)"]
    pub cis9: crate::Reg<cis9::CIS9_SPEC>,
    #[doc = "0x2a0 - Channel Source Address Register (chid = 9)"]
    pub csa9: crate::Reg<csa9::CSA9_SPEC>,
    #[doc = "0x2a4 - Channel Destination Address Register (chid = 9)"]
    pub cda9: crate::Reg<cda9::CDA9_SPEC>,
    #[doc = "0x2a8 - Channel Next Descriptor Address Register (chid = 9)"]
    pub cnda9: crate::Reg<cnda9::CNDA9_SPEC>,
    #[doc = "0x2ac - Channel Next Descriptor Control Register (chid = 9)"]
    pub cndc9: crate::Reg<cndc9::CNDC9_SPEC>,
    #[doc = "0x2b0 - Channel Microblock Control Register (chid = 9)"]
    pub cubc9: crate::Reg<cubc9::CUBC9_SPEC>,
    #[doc = "0x2b4 - Channel Block Control Register (chid = 9)"]
    pub cbc9: crate::Reg<cbc9::CBC9_SPEC>,
    #[doc = "0x2b8 - Channel Configuration Register (chid = 9)"]
    pub cc9: crate::Reg<cc9::CC9_SPEC>,
    #[doc = "0x2bc - Channel Data Stride Memory Set Pattern (chid = 9)"]
    pub cds_msp9: crate::Reg<cds_msp9::CDS_MSP9_SPEC>,
    #[doc = "0x2c0 - Channel Source Microblock Stride (chid = 9)"]
    pub csus9: crate::Reg<csus9::CSUS9_SPEC>,
    #[doc = "0x2c4 - Channel Destination Microblock Stride (chid = 9)"]
    pub cdus9: crate::Reg<cdus9::CDUS9_SPEC>,
    _reserved157: [u8; 8usize],
    #[doc = "0x2d0 - Channel Interrupt Enable Register (chid = 10)"]
    pub cie10: crate::Reg<cie10::CIE10_SPEC>,
    #[doc = "0x2d4 - Channel Interrupt Disable Register (chid = 10)"]
    pub cid10: crate::Reg<cid10::CID10_SPEC>,
    #[doc = "0x2d8 - Channel Interrupt Mask Register (chid = 10)"]
    pub cim10: crate::Reg<cim10::CIM10_SPEC>,
    #[doc = "0x2dc - Channel Interrupt Status Register (chid = 10)"]
    pub cis10: crate::Reg<cis10::CIS10_SPEC>,
    #[doc = "0x2e0 - Channel Source Address Register (chid = 10)"]
    pub csa10: crate::Reg<csa10::CSA10_SPEC>,
    #[doc = "0x2e4 - Channel Destination Address Register (chid = 10)"]
    pub cda10: crate::Reg<cda10::CDA10_SPEC>,
    #[doc = "0x2e8 - Channel Next Descriptor Address Register (chid = 10)"]
    pub cnda10: crate::Reg<cnda10::CNDA10_SPEC>,
    #[doc = "0x2ec - Channel Next Descriptor Control Register (chid = 10)"]
    pub cndc10: crate::Reg<cndc10::CNDC10_SPEC>,
    #[doc = "0x2f0 - Channel Microblock Control Register (chid = 10)"]
    pub cubc10: crate::Reg<cubc10::CUBC10_SPEC>,
    #[doc = "0x2f4 - Channel Block Control Register (chid = 10)"]
    pub cbc10: crate::Reg<cbc10::CBC10_SPEC>,
    #[doc = "0x2f8 - Channel Configuration Register (chid = 10)"]
    pub cc10: crate::Reg<cc10::CC10_SPEC>,
    #[doc = "0x2fc - Channel Data Stride Memory Set Pattern (chid = 10)"]
    pub cds_msp10: crate::Reg<cds_msp10::CDS_MSP10_SPEC>,
    #[doc = "0x300 - Channel Source Microblock Stride (chid = 10)"]
    pub csus10: crate::Reg<csus10::CSUS10_SPEC>,
    #[doc = "0x304 - Channel Destination Microblock Stride (chid = 10)"]
    pub cdus10: crate::Reg<cdus10::CDUS10_SPEC>,
    _reserved171: [u8; 8usize],
    #[doc = "0x310 - Channel Interrupt Enable Register (chid = 11)"]
    pub cie11: crate::Reg<cie11::CIE11_SPEC>,
    #[doc = "0x314 - Channel Interrupt Disable Register (chid = 11)"]
    pub cid11: crate::Reg<cid11::CID11_SPEC>,
    #[doc = "0x318 - Channel Interrupt Mask Register (chid = 11)"]
    pub cim11: crate::Reg<cim11::CIM11_SPEC>,
    #[doc = "0x31c - Channel Interrupt Status Register (chid = 11)"]
    pub cis11: crate::Reg<cis11::CIS11_SPEC>,
    #[doc = "0x320 - Channel Source Address Register (chid = 11)"]
    pub csa11: crate::Reg<csa11::CSA11_SPEC>,
    #[doc = "0x324 - Channel Destination Address Register (chid = 11)"]
    pub cda11: crate::Reg<cda11::CDA11_SPEC>,
    #[doc = "0x328 - Channel Next Descriptor Address Register (chid = 11)"]
    pub cnda11: crate::Reg<cnda11::CNDA11_SPEC>,
    #[doc = "0x32c - Channel Next Descriptor Control Register (chid = 11)"]
    pub cndc11: crate::Reg<cndc11::CNDC11_SPEC>,
    #[doc = "0x330 - Channel Microblock Control Register (chid = 11)"]
    pub cubc11: crate::Reg<cubc11::CUBC11_SPEC>,
    #[doc = "0x334 - Channel Block Control Register (chid = 11)"]
    pub cbc11: crate::Reg<cbc11::CBC11_SPEC>,
    #[doc = "0x338 - Channel Configuration Register (chid = 11)"]
    pub cc11: crate::Reg<cc11::CC11_SPEC>,
    #[doc = "0x33c - Channel Data Stride Memory Set Pattern (chid = 11)"]
    pub cds_msp11: crate::Reg<cds_msp11::CDS_MSP11_SPEC>,
    #[doc = "0x340 - Channel Source Microblock Stride (chid = 11)"]
    pub csus11: crate::Reg<csus11::CSUS11_SPEC>,
    #[doc = "0x344 - Channel Destination Microblock Stride (chid = 11)"]
    pub cdus11: crate::Reg<cdus11::CDUS11_SPEC>,
    _reserved185: [u8; 8usize],
    #[doc = "0x350 - Channel Interrupt Enable Register (chid = 12)"]
    pub cie12: crate::Reg<cie12::CIE12_SPEC>,
    #[doc = "0x354 - Channel Interrupt Disable Register (chid = 12)"]
    pub cid12: crate::Reg<cid12::CID12_SPEC>,
    #[doc = "0x358 - Channel Interrupt Mask Register (chid = 12)"]
    pub cim12: crate::Reg<cim12::CIM12_SPEC>,
    #[doc = "0x35c - Channel Interrupt Status Register (chid = 12)"]
    pub cis12: crate::Reg<cis12::CIS12_SPEC>,
    #[doc = "0x360 - Channel Source Address Register (chid = 12)"]
    pub csa12: crate::Reg<csa12::CSA12_SPEC>,
    #[doc = "0x364 - Channel Destination Address Register (chid = 12)"]
    pub cda12: crate::Reg<cda12::CDA12_SPEC>,
    #[doc = "0x368 - Channel Next Descriptor Address Register (chid = 12)"]
    pub cnda12: crate::Reg<cnda12::CNDA12_SPEC>,
    #[doc = "0x36c - Channel Next Descriptor Control Register (chid = 12)"]
    pub cndc12: crate::Reg<cndc12::CNDC12_SPEC>,
    #[doc = "0x370 - Channel Microblock Control Register (chid = 12)"]
    pub cubc12: crate::Reg<cubc12::CUBC12_SPEC>,
    #[doc = "0x374 - Channel Block Control Register (chid = 12)"]
    pub cbc12: crate::Reg<cbc12::CBC12_SPEC>,
    #[doc = "0x378 - Channel Configuration Register (chid = 12)"]
    pub cc12: crate::Reg<cc12::CC12_SPEC>,
    #[doc = "0x37c - Channel Data Stride Memory Set Pattern (chid = 12)"]
    pub cds_msp12: crate::Reg<cds_msp12::CDS_MSP12_SPEC>,
    #[doc = "0x380 - Channel Source Microblock Stride (chid = 12)"]
    pub csus12: crate::Reg<csus12::CSUS12_SPEC>,
    #[doc = "0x384 - Channel Destination Microblock Stride (chid = 12)"]
    pub cdus12: crate::Reg<cdus12::CDUS12_SPEC>,
    _reserved199: [u8; 8usize],
    #[doc = "0x390 - Channel Interrupt Enable Register (chid = 13)"]
    pub cie13: crate::Reg<cie13::CIE13_SPEC>,
    #[doc = "0x394 - Channel Interrupt Disable Register (chid = 13)"]
    pub cid13: crate::Reg<cid13::CID13_SPEC>,
    #[doc = "0x398 - Channel Interrupt Mask Register (chid = 13)"]
    pub cim13: crate::Reg<cim13::CIM13_SPEC>,
    #[doc = "0x39c - Channel Interrupt Status Register (chid = 13)"]
    pub cis13: crate::Reg<cis13::CIS13_SPEC>,
    #[doc = "0x3a0 - Channel Source Address Register (chid = 13)"]
    pub csa13: crate::Reg<csa13::CSA13_SPEC>,
    #[doc = "0x3a4 - Channel Destination Address Register (chid = 13)"]
    pub cda13: crate::Reg<cda13::CDA13_SPEC>,
    #[doc = "0x3a8 - Channel Next Descriptor Address Register (chid = 13)"]
    pub cnda13: crate::Reg<cnda13::CNDA13_SPEC>,
    #[doc = "0x3ac - Channel Next Descriptor Control Register (chid = 13)"]
    pub cndc13: crate::Reg<cndc13::CNDC13_SPEC>,
    #[doc = "0x3b0 - Channel Microblock Control Register (chid = 13)"]
    pub cubc13: crate::Reg<cubc13::CUBC13_SPEC>,
    #[doc = "0x3b4 - Channel Block Control Register (chid = 13)"]
    pub cbc13: crate::Reg<cbc13::CBC13_SPEC>,
    #[doc = "0x3b8 - Channel Configuration Register (chid = 13)"]
    pub cc13: crate::Reg<cc13::CC13_SPEC>,
    #[doc = "0x3bc - Channel Data Stride Memory Set Pattern (chid = 13)"]
    pub cds_msp13: crate::Reg<cds_msp13::CDS_MSP13_SPEC>,
    #[doc = "0x3c0 - Channel Source Microblock Stride (chid = 13)"]
    pub csus13: crate::Reg<csus13::CSUS13_SPEC>,
    #[doc = "0x3c4 - Channel Destination Microblock Stride (chid = 13)"]
    pub cdus13: crate::Reg<cdus13::CDUS13_SPEC>,
    _reserved213: [u8; 8usize],
    #[doc = "0x3d0 - Channel Interrupt Enable Register (chid = 14)"]
    pub cie14: crate::Reg<cie14::CIE14_SPEC>,
    #[doc = "0x3d4 - Channel Interrupt Disable Register (chid = 14)"]
    pub cid14: crate::Reg<cid14::CID14_SPEC>,
    #[doc = "0x3d8 - Channel Interrupt Mask Register (chid = 14)"]
    pub cim14: crate::Reg<cim14::CIM14_SPEC>,
    #[doc = "0x3dc - Channel Interrupt Status Register (chid = 14)"]
    pub cis14: crate::Reg<cis14::CIS14_SPEC>,
    #[doc = "0x3e0 - Channel Source Address Register (chid = 14)"]
    pub csa14: crate::Reg<csa14::CSA14_SPEC>,
    #[doc = "0x3e4 - Channel Destination Address Register (chid = 14)"]
    pub cda14: crate::Reg<cda14::CDA14_SPEC>,
    #[doc = "0x3e8 - Channel Next Descriptor Address Register (chid = 14)"]
    pub cnda14: crate::Reg<cnda14::CNDA14_SPEC>,
    #[doc = "0x3ec - Channel Next Descriptor Control Register (chid = 14)"]
    pub cndc14: crate::Reg<cndc14::CNDC14_SPEC>,
    #[doc = "0x3f0 - Channel Microblock Control Register (chid = 14)"]
    pub cubc14: crate::Reg<cubc14::CUBC14_SPEC>,
    #[doc = "0x3f4 - Channel Block Control Register (chid = 14)"]
    pub cbc14: crate::Reg<cbc14::CBC14_SPEC>,
    #[doc = "0x3f8 - Channel Configuration Register (chid = 14)"]
    pub cc14: crate::Reg<cc14::CC14_SPEC>,
    #[doc = "0x3fc - Channel Data Stride Memory Set Pattern (chid = 14)"]
    pub cds_msp14: crate::Reg<cds_msp14::CDS_MSP14_SPEC>,
    #[doc = "0x400 - Channel Source Microblock Stride (chid = 14)"]
    pub csus14: crate::Reg<csus14::CSUS14_SPEC>,
    #[doc = "0x404 - Channel Destination Microblock Stride (chid = 14)"]
    pub cdus14: crate::Reg<cdus14::CDUS14_SPEC>,
    _reserved227: [u8; 8usize],
    #[doc = "0x410 - Channel Interrupt Enable Register (chid = 15)"]
    pub cie15: crate::Reg<cie15::CIE15_SPEC>,
    #[doc = "0x414 - Channel Interrupt Disable Register (chid = 15)"]
    pub cid15: crate::Reg<cid15::CID15_SPEC>,
    #[doc = "0x418 - Channel Interrupt Mask Register (chid = 15)"]
    pub cim15: crate::Reg<cim15::CIM15_SPEC>,
    #[doc = "0x41c - Channel Interrupt Status Register (chid = 15)"]
    pub cis15: crate::Reg<cis15::CIS15_SPEC>,
    #[doc = "0x420 - Channel Source Address Register (chid = 15)"]
    pub csa15: crate::Reg<csa15::CSA15_SPEC>,
    #[doc = "0x424 - Channel Destination Address Register (chid = 15)"]
    pub cda15: crate::Reg<cda15::CDA15_SPEC>,
    #[doc = "0x428 - Channel Next Descriptor Address Register (chid = 15)"]
    pub cnda15: crate::Reg<cnda15::CNDA15_SPEC>,
    #[doc = "0x42c - Channel Next Descriptor Control Register (chid = 15)"]
    pub cndc15: crate::Reg<cndc15::CNDC15_SPEC>,
    #[doc = "0x430 - Channel Microblock Control Register (chid = 15)"]
    pub cubc15: crate::Reg<cubc15::CUBC15_SPEC>,
    #[doc = "0x434 - Channel Block Control Register (chid = 15)"]
    pub cbc15: crate::Reg<cbc15::CBC15_SPEC>,
    #[doc = "0x438 - Channel Configuration Register (chid = 15)"]
    pub cc15: crate::Reg<cc15::CC15_SPEC>,
    #[doc = "0x43c - Channel Data Stride Memory Set Pattern (chid = 15)"]
    pub cds_msp15: crate::Reg<cds_msp15::CDS_MSP15_SPEC>,
    #[doc = "0x440 - Channel Source Microblock Stride (chid = 15)"]
    pub csus15: crate::Reg<csus15::CSUS15_SPEC>,
    #[doc = "0x444 - Channel Destination Microblock Stride (chid = 15)"]
    pub cdus15: crate::Reg<cdus15::CDUS15_SPEC>,
    _reserved241: [u8; 8usize],
    #[doc = "0x450 - Channel Interrupt Enable Register (chid = 16)"]
    pub cie16: crate::Reg<cie16::CIE16_SPEC>,
    #[doc = "0x454 - Channel Interrupt Disable Register (chid = 16)"]
    pub cid16: crate::Reg<cid16::CID16_SPEC>,
    #[doc = "0x458 - Channel Interrupt Mask Register (chid = 16)"]
    pub cim16: crate::Reg<cim16::CIM16_SPEC>,
    #[doc = "0x45c - Channel Interrupt Status Register (chid = 16)"]
    pub cis16: crate::Reg<cis16::CIS16_SPEC>,
    #[doc = "0x460 - Channel Source Address Register (chid = 16)"]
    pub csa16: crate::Reg<csa16::CSA16_SPEC>,
    #[doc = "0x464 - Channel Destination Address Register (chid = 16)"]
    pub cda16: crate::Reg<cda16::CDA16_SPEC>,
    #[doc = "0x468 - Channel Next Descriptor Address Register (chid = 16)"]
    pub cnda16: crate::Reg<cnda16::CNDA16_SPEC>,
    #[doc = "0x46c - Channel Next Descriptor Control Register (chid = 16)"]
    pub cndc16: crate::Reg<cndc16::CNDC16_SPEC>,
    #[doc = "0x470 - Channel Microblock Control Register (chid = 16)"]
    pub cubc16: crate::Reg<cubc16::CUBC16_SPEC>,
    #[doc = "0x474 - Channel Block Control Register (chid = 16)"]
    pub cbc16: crate::Reg<cbc16::CBC16_SPEC>,
    #[doc = "0x478 - Channel Configuration Register (chid = 16)"]
    pub cc16: crate::Reg<cc16::CC16_SPEC>,
    #[doc = "0x47c - Channel Data Stride Memory Set Pattern (chid = 16)"]
    pub cds_msp16: crate::Reg<cds_msp16::CDS_MSP16_SPEC>,
    #[doc = "0x480 - Channel Source Microblock Stride (chid = 16)"]
    pub csus16: crate::Reg<csus16::CSUS16_SPEC>,
    #[doc = "0x484 - Channel Destination Microblock Stride (chid = 16)"]
    pub cdus16: crate::Reg<cdus16::CDUS16_SPEC>,
    _reserved255: [u8; 8usize],
    #[doc = "0x490 - Channel Interrupt Enable Register (chid = 17)"]
    pub cie17: crate::Reg<cie17::CIE17_SPEC>,
    #[doc = "0x494 - Channel Interrupt Disable Register (chid = 17)"]
    pub cid17: crate::Reg<cid17::CID17_SPEC>,
    #[doc = "0x498 - Channel Interrupt Mask Register (chid = 17)"]
    pub cim17: crate::Reg<cim17::CIM17_SPEC>,
    #[doc = "0x49c - Channel Interrupt Status Register (chid = 17)"]
    pub cis17: crate::Reg<cis17::CIS17_SPEC>,
    #[doc = "0x4a0 - Channel Source Address Register (chid = 17)"]
    pub csa17: crate::Reg<csa17::CSA17_SPEC>,
    #[doc = "0x4a4 - Channel Destination Address Register (chid = 17)"]
    pub cda17: crate::Reg<cda17::CDA17_SPEC>,
    #[doc = "0x4a8 - Channel Next Descriptor Address Register (chid = 17)"]
    pub cnda17: crate::Reg<cnda17::CNDA17_SPEC>,
    #[doc = "0x4ac - Channel Next Descriptor Control Register (chid = 17)"]
    pub cndc17: crate::Reg<cndc17::CNDC17_SPEC>,
    #[doc = "0x4b0 - Channel Microblock Control Register (chid = 17)"]
    pub cubc17: crate::Reg<cubc17::CUBC17_SPEC>,
    #[doc = "0x4b4 - Channel Block Control Register (chid = 17)"]
    pub cbc17: crate::Reg<cbc17::CBC17_SPEC>,
    #[doc = "0x4b8 - Channel Configuration Register (chid = 17)"]
    pub cc17: crate::Reg<cc17::CC17_SPEC>,
    #[doc = "0x4bc - Channel Data Stride Memory Set Pattern (chid = 17)"]
    pub cds_msp17: crate::Reg<cds_msp17::CDS_MSP17_SPEC>,
    #[doc = "0x4c0 - Channel Source Microblock Stride (chid = 17)"]
    pub csus17: crate::Reg<csus17::CSUS17_SPEC>,
    #[doc = "0x4c4 - Channel Destination Microblock Stride (chid = 17)"]
    pub cdus17: crate::Reg<cdus17::CDUS17_SPEC>,
    _reserved269: [u8; 8usize],
    #[doc = "0x4d0 - Channel Interrupt Enable Register (chid = 18)"]
    pub cie18: crate::Reg<cie18::CIE18_SPEC>,
    #[doc = "0x4d4 - Channel Interrupt Disable Register (chid = 18)"]
    pub cid18: crate::Reg<cid18::CID18_SPEC>,
    #[doc = "0x4d8 - Channel Interrupt Mask Register (chid = 18)"]
    pub cim18: crate::Reg<cim18::CIM18_SPEC>,
    #[doc = "0x4dc - Channel Interrupt Status Register (chid = 18)"]
    pub cis18: crate::Reg<cis18::CIS18_SPEC>,
    #[doc = "0x4e0 - Channel Source Address Register (chid = 18)"]
    pub csa18: crate::Reg<csa18::CSA18_SPEC>,
    #[doc = "0x4e4 - Channel Destination Address Register (chid = 18)"]
    pub cda18: crate::Reg<cda18::CDA18_SPEC>,
    #[doc = "0x4e8 - Channel Next Descriptor Address Register (chid = 18)"]
    pub cnda18: crate::Reg<cnda18::CNDA18_SPEC>,
    #[doc = "0x4ec - Channel Next Descriptor Control Register (chid = 18)"]
    pub cndc18: crate::Reg<cndc18::CNDC18_SPEC>,
    #[doc = "0x4f0 - Channel Microblock Control Register (chid = 18)"]
    pub cubc18: crate::Reg<cubc18::CUBC18_SPEC>,
    #[doc = "0x4f4 - Channel Block Control Register (chid = 18)"]
    pub cbc18: crate::Reg<cbc18::CBC18_SPEC>,
    #[doc = "0x4f8 - Channel Configuration Register (chid = 18)"]
    pub cc18: crate::Reg<cc18::CC18_SPEC>,
    #[doc = "0x4fc - Channel Data Stride Memory Set Pattern (chid = 18)"]
    pub cds_msp18: crate::Reg<cds_msp18::CDS_MSP18_SPEC>,
    #[doc = "0x500 - Channel Source Microblock Stride (chid = 18)"]
    pub csus18: crate::Reg<csus18::CSUS18_SPEC>,
    #[doc = "0x504 - Channel Destination Microblock Stride (chid = 18)"]
    pub cdus18: crate::Reg<cdus18::CDUS18_SPEC>,
    _reserved283: [u8; 8usize],
    #[doc = "0x510 - Channel Interrupt Enable Register (chid = 19)"]
    pub cie19: crate::Reg<cie19::CIE19_SPEC>,
    #[doc = "0x514 - Channel Interrupt Disable Register (chid = 19)"]
    pub cid19: crate::Reg<cid19::CID19_SPEC>,
    #[doc = "0x518 - Channel Interrupt Mask Register (chid = 19)"]
    pub cim19: crate::Reg<cim19::CIM19_SPEC>,
    #[doc = "0x51c - Channel Interrupt Status Register (chid = 19)"]
    pub cis19: crate::Reg<cis19::CIS19_SPEC>,
    #[doc = "0x520 - Channel Source Address Register (chid = 19)"]
    pub csa19: crate::Reg<csa19::CSA19_SPEC>,
    #[doc = "0x524 - Channel Destination Address Register (chid = 19)"]
    pub cda19: crate::Reg<cda19::CDA19_SPEC>,
    #[doc = "0x528 - Channel Next Descriptor Address Register (chid = 19)"]
    pub cnda19: crate::Reg<cnda19::CNDA19_SPEC>,
    #[doc = "0x52c - Channel Next Descriptor Control Register (chid = 19)"]
    pub cndc19: crate::Reg<cndc19::CNDC19_SPEC>,
    #[doc = "0x530 - Channel Microblock Control Register (chid = 19)"]
    pub cubc19: crate::Reg<cubc19::CUBC19_SPEC>,
    #[doc = "0x534 - Channel Block Control Register (chid = 19)"]
    pub cbc19: crate::Reg<cbc19::CBC19_SPEC>,
    #[doc = "0x538 - Channel Configuration Register (chid = 19)"]
    pub cc19: crate::Reg<cc19::CC19_SPEC>,
    #[doc = "0x53c - Channel Data Stride Memory Set Pattern (chid = 19)"]
    pub cds_msp19: crate::Reg<cds_msp19::CDS_MSP19_SPEC>,
    #[doc = "0x540 - Channel Source Microblock Stride (chid = 19)"]
    pub csus19: crate::Reg<csus19::CSUS19_SPEC>,
    #[doc = "0x544 - Channel Destination Microblock Stride (chid = 19)"]
    pub cdus19: crate::Reg<cdus19::CDUS19_SPEC>,
    _reserved297: [u8; 8usize],
    #[doc = "0x550 - Channel Interrupt Enable Register (chid = 20)"]
    pub cie20: crate::Reg<cie20::CIE20_SPEC>,
    #[doc = "0x554 - Channel Interrupt Disable Register (chid = 20)"]
    pub cid20: crate::Reg<cid20::CID20_SPEC>,
    #[doc = "0x558 - Channel Interrupt Mask Register (chid = 20)"]
    pub cim20: crate::Reg<cim20::CIM20_SPEC>,
    #[doc = "0x55c - Channel Interrupt Status Register (chid = 20)"]
    pub cis20: crate::Reg<cis20::CIS20_SPEC>,
    #[doc = "0x560 - Channel Source Address Register (chid = 20)"]
    pub csa20: crate::Reg<csa20::CSA20_SPEC>,
    #[doc = "0x564 - Channel Destination Address Register (chid = 20)"]
    pub cda20: crate::Reg<cda20::CDA20_SPEC>,
    #[doc = "0x568 - Channel Next Descriptor Address Register (chid = 20)"]
    pub cnda20: crate::Reg<cnda20::CNDA20_SPEC>,
    #[doc = "0x56c - Channel Next Descriptor Control Register (chid = 20)"]
    pub cndc20: crate::Reg<cndc20::CNDC20_SPEC>,
    #[doc = "0x570 - Channel Microblock Control Register (chid = 20)"]
    pub cubc20: crate::Reg<cubc20::CUBC20_SPEC>,
    #[doc = "0x574 - Channel Block Control Register (chid = 20)"]
    pub cbc20: crate::Reg<cbc20::CBC20_SPEC>,
    #[doc = "0x578 - Channel Configuration Register (chid = 20)"]
    pub cc20: crate::Reg<cc20::CC20_SPEC>,
    #[doc = "0x57c - Channel Data Stride Memory Set Pattern (chid = 20)"]
    pub cds_msp20: crate::Reg<cds_msp20::CDS_MSP20_SPEC>,
    #[doc = "0x580 - Channel Source Microblock Stride (chid = 20)"]
    pub csus20: crate::Reg<csus20::CSUS20_SPEC>,
    #[doc = "0x584 - Channel Destination Microblock Stride (chid = 20)"]
    pub cdus20: crate::Reg<cdus20::CDUS20_SPEC>,
    _reserved311: [u8; 8usize],
    #[doc = "0x590 - Channel Interrupt Enable Register (chid = 21)"]
    pub cie21: crate::Reg<cie21::CIE21_SPEC>,
    #[doc = "0x594 - Channel Interrupt Disable Register (chid = 21)"]
    pub cid21: crate::Reg<cid21::CID21_SPEC>,
    #[doc = "0x598 - Channel Interrupt Mask Register (chid = 21)"]
    pub cim21: crate::Reg<cim21::CIM21_SPEC>,
    #[doc = "0x59c - Channel Interrupt Status Register (chid = 21)"]
    pub cis21: crate::Reg<cis21::CIS21_SPEC>,
    #[doc = "0x5a0 - Channel Source Address Register (chid = 21)"]
    pub csa21: crate::Reg<csa21::CSA21_SPEC>,
    #[doc = "0x5a4 - Channel Destination Address Register (chid = 21)"]
    pub cda21: crate::Reg<cda21::CDA21_SPEC>,
    #[doc = "0x5a8 - Channel Next Descriptor Address Register (chid = 21)"]
    pub cnda21: crate::Reg<cnda21::CNDA21_SPEC>,
    #[doc = "0x5ac - Channel Next Descriptor Control Register (chid = 21)"]
    pub cndc21: crate::Reg<cndc21::CNDC21_SPEC>,
    #[doc = "0x5b0 - Channel Microblock Control Register (chid = 21)"]
    pub cubc21: crate::Reg<cubc21::CUBC21_SPEC>,
    #[doc = "0x5b4 - Channel Block Control Register (chid = 21)"]
    pub cbc21: crate::Reg<cbc21::CBC21_SPEC>,
    #[doc = "0x5b8 - Channel Configuration Register (chid = 21)"]
    pub cc21: crate::Reg<cc21::CC21_SPEC>,
    #[doc = "0x5bc - Channel Data Stride Memory Set Pattern (chid = 21)"]
    pub cds_msp21: crate::Reg<cds_msp21::CDS_MSP21_SPEC>,
    #[doc = "0x5c0 - Channel Source Microblock Stride (chid = 21)"]
    pub csus21: crate::Reg<csus21::CSUS21_SPEC>,
    #[doc = "0x5c4 - Channel Destination Microblock Stride (chid = 21)"]
    pub cdus21: crate::Reg<cdus21::CDUS21_SPEC>,
    _reserved325: [u8; 8usize],
    #[doc = "0x5d0 - Channel Interrupt Enable Register (chid = 22)"]
    pub cie22: crate::Reg<cie22::CIE22_SPEC>,
    #[doc = "0x5d4 - Channel Interrupt Disable Register (chid = 22)"]
    pub cid22: crate::Reg<cid22::CID22_SPEC>,
    #[doc = "0x5d8 - Channel Interrupt Mask Register (chid = 22)"]
    pub cim22: crate::Reg<cim22::CIM22_SPEC>,
    #[doc = "0x5dc - Channel Interrupt Status Register (chid = 22)"]
    pub cis22: crate::Reg<cis22::CIS22_SPEC>,
    #[doc = "0x5e0 - Channel Source Address Register (chid = 22)"]
    pub csa22: crate::Reg<csa22::CSA22_SPEC>,
    #[doc = "0x5e4 - Channel Destination Address Register (chid = 22)"]
    pub cda22: crate::Reg<cda22::CDA22_SPEC>,
    #[doc = "0x5e8 - Channel Next Descriptor Address Register (chid = 22)"]
    pub cnda22: crate::Reg<cnda22::CNDA22_SPEC>,
    #[doc = "0x5ec - Channel Next Descriptor Control Register (chid = 22)"]
    pub cndc22: crate::Reg<cndc22::CNDC22_SPEC>,
    #[doc = "0x5f0 - Channel Microblock Control Register (chid = 22)"]
    pub cubc22: crate::Reg<cubc22::CUBC22_SPEC>,
    #[doc = "0x5f4 - Channel Block Control Register (chid = 22)"]
    pub cbc22: crate::Reg<cbc22::CBC22_SPEC>,
    #[doc = "0x5f8 - Channel Configuration Register (chid = 22)"]
    pub cc22: crate::Reg<cc22::CC22_SPEC>,
    #[doc = "0x5fc - Channel Data Stride Memory Set Pattern (chid = 22)"]
    pub cds_msp22: crate::Reg<cds_msp22::CDS_MSP22_SPEC>,
    #[doc = "0x600 - Channel Source Microblock Stride (chid = 22)"]
    pub csus22: crate::Reg<csus22::CSUS22_SPEC>,
    #[doc = "0x604 - Channel Destination Microblock Stride (chid = 22)"]
    pub cdus22: crate::Reg<cdus22::CDUS22_SPEC>,
    _reserved339: [u8; 8usize],
    #[doc = "0x610 - Channel Interrupt Enable Register (chid = 23)"]
    pub cie23: crate::Reg<cie23::CIE23_SPEC>,
    #[doc = "0x614 - Channel Interrupt Disable Register (chid = 23)"]
    pub cid23: crate::Reg<cid23::CID23_SPEC>,
    #[doc = "0x618 - Channel Interrupt Mask Register (chid = 23)"]
    pub cim23: crate::Reg<cim23::CIM23_SPEC>,
    #[doc = "0x61c - Channel Interrupt Status Register (chid = 23)"]
    pub cis23: crate::Reg<cis23::CIS23_SPEC>,
    #[doc = "0x620 - Channel Source Address Register (chid = 23)"]
    pub csa23: crate::Reg<csa23::CSA23_SPEC>,
    #[doc = "0x624 - Channel Destination Address Register (chid = 23)"]
    pub cda23: crate::Reg<cda23::CDA23_SPEC>,
    #[doc = "0x628 - Channel Next Descriptor Address Register (chid = 23)"]
    pub cnda23: crate::Reg<cnda23::CNDA23_SPEC>,
    #[doc = "0x62c - Channel Next Descriptor Control Register (chid = 23)"]
    pub cndc23: crate::Reg<cndc23::CNDC23_SPEC>,
    #[doc = "0x630 - Channel Microblock Control Register (chid = 23)"]
    pub cubc23: crate::Reg<cubc23::CUBC23_SPEC>,
    #[doc = "0x634 - Channel Block Control Register (chid = 23)"]
    pub cbc23: crate::Reg<cbc23::CBC23_SPEC>,
    #[doc = "0x638 - Channel Configuration Register (chid = 23)"]
    pub cc23: crate::Reg<cc23::CC23_SPEC>,
    #[doc = "0x63c - Channel Data Stride Memory Set Pattern (chid = 23)"]
    pub cds_msp23: crate::Reg<cds_msp23::CDS_MSP23_SPEC>,
    #[doc = "0x640 - Channel Source Microblock Stride (chid = 23)"]
    pub csus23: crate::Reg<csus23::CSUS23_SPEC>,
    #[doc = "0x644 - Channel Destination Microblock Stride (chid = 23)"]
    pub cdus23: crate::Reg<cdus23::CDUS23_SPEC>,
}
#[doc = "GTYPE register accessor: an alias for `Reg<GTYPE_SPEC>`"]
pub type GTYPE = crate::Reg<gtype::GTYPE_SPEC>;
#[doc = "Global Type Register"]
pub mod gtype;
#[doc = "GCFG register accessor: an alias for `Reg<GCFG_SPEC>`"]
pub type GCFG = crate::Reg<gcfg::GCFG_SPEC>;
#[doc = "Global Configuration Register"]
pub mod gcfg;
#[doc = "GWAC register accessor: an alias for `Reg<GWAC_SPEC>`"]
pub type GWAC = crate::Reg<gwac::GWAC_SPEC>;
#[doc = "Global Weighted Arbiter Configuration Register"]
pub mod gwac;
#[doc = "GIE register accessor: an alias for `Reg<GIE_SPEC>`"]
pub type GIE = crate::Reg<gie::GIE_SPEC>;
#[doc = "Global Interrupt Enable Register"]
pub mod gie;
#[doc = "GID register accessor: an alias for `Reg<GID_SPEC>`"]
pub type GID = crate::Reg<gid::GID_SPEC>;
#[doc = "Global Interrupt Disable Register"]
pub mod gid;
#[doc = "GIM register accessor: an alias for `Reg<GIM_SPEC>`"]
pub type GIM = crate::Reg<gim::GIM_SPEC>;
#[doc = "Global Interrupt Mask Register"]
pub mod gim;
#[doc = "GIS register accessor: an alias for `Reg<GIS_SPEC>`"]
pub type GIS = crate::Reg<gis::GIS_SPEC>;
#[doc = "Global Interrupt Status Register"]
pub mod gis;
#[doc = "GE register accessor: an alias for `Reg<GE_SPEC>`"]
pub type GE = crate::Reg<ge::GE_SPEC>;
#[doc = "Global Channel Enable Register"]
pub mod ge;
#[doc = "GD register accessor: an alias for `Reg<GD_SPEC>`"]
pub type GD = crate::Reg<gd::GD_SPEC>;
#[doc = "Global Channel Disable Register"]
pub mod gd;
#[doc = "GS register accessor: an alias for `Reg<GS_SPEC>`"]
pub type GS = crate::Reg<gs::GS_SPEC>;
#[doc = "Global Channel Status Register"]
pub mod gs;
#[doc = "GRS register accessor: an alias for `Reg<GRS_SPEC>`"]
pub type GRS = crate::Reg<grs::GRS_SPEC>;
#[doc = "Global Channel Read Suspend Register"]
pub mod grs;
#[doc = "GWS register accessor: an alias for `Reg<GWS_SPEC>`"]
pub type GWS = crate::Reg<gws::GWS_SPEC>;
#[doc = "Global Channel Write Suspend Register"]
pub mod gws;
#[doc = "GRWS register accessor: an alias for `Reg<GRWS_SPEC>`"]
pub type GRWS = crate::Reg<grws::GRWS_SPEC>;
#[doc = "Global Channel Read Write Suspend Register"]
pub mod grws;
#[doc = "GRWR register accessor: an alias for `Reg<GRWR_SPEC>`"]
pub type GRWR = crate::Reg<grwr::GRWR_SPEC>;
#[doc = "Global Channel Read Write Resume Register"]
pub mod grwr;
#[doc = "GSWR register accessor: an alias for `Reg<GSWR_SPEC>`"]
pub type GSWR = crate::Reg<gswr::GSWR_SPEC>;
#[doc = "Global Channel Software Request Register"]
pub mod gswr;
#[doc = "GSWS register accessor: an alias for `Reg<GSWS_SPEC>`"]
pub type GSWS = crate::Reg<gsws::GSWS_SPEC>;
#[doc = "Global Channel Software Request Status Register"]
pub mod gsws;
#[doc = "GSWF register accessor: an alias for `Reg<GSWF_SPEC>`"]
pub type GSWF = crate::Reg<gswf::GSWF_SPEC>;
#[doc = "Global Channel Software Flush Request Register"]
pub mod gswf;
#[doc = "CIE0 register accessor: an alias for `Reg<CIE0_SPEC>`"]
pub type CIE0 = crate::Reg<cie0::CIE0_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 0)"]
pub mod cie0;
#[doc = "CID0 register accessor: an alias for `Reg<CID0_SPEC>`"]
pub type CID0 = crate::Reg<cid0::CID0_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 0)"]
pub mod cid0;
#[doc = "CIM0 register accessor: an alias for `Reg<CIM0_SPEC>`"]
pub type CIM0 = crate::Reg<cim0::CIM0_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 0)"]
pub mod cim0;
#[doc = "CIS0 register accessor: an alias for `Reg<CIS0_SPEC>`"]
pub type CIS0 = crate::Reg<cis0::CIS0_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 0)"]
pub mod cis0;
#[doc = "CSA0 register accessor: an alias for `Reg<CSA0_SPEC>`"]
pub type CSA0 = crate::Reg<csa0::CSA0_SPEC>;
#[doc = "Channel Source Address Register (chid = 0)"]
pub mod csa0;
#[doc = "CDA0 register accessor: an alias for `Reg<CDA0_SPEC>`"]
pub type CDA0 = crate::Reg<cda0::CDA0_SPEC>;
#[doc = "Channel Destination Address Register (chid = 0)"]
pub mod cda0;
#[doc = "CNDA0 register accessor: an alias for `Reg<CNDA0_SPEC>`"]
pub type CNDA0 = crate::Reg<cnda0::CNDA0_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 0)"]
pub mod cnda0;
#[doc = "CNDC0 register accessor: an alias for `Reg<CNDC0_SPEC>`"]
pub type CNDC0 = crate::Reg<cndc0::CNDC0_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 0)"]
pub mod cndc0;
#[doc = "CUBC0 register accessor: an alias for `Reg<CUBC0_SPEC>`"]
pub type CUBC0 = crate::Reg<cubc0::CUBC0_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 0)"]
pub mod cubc0;
#[doc = "CBC0 register accessor: an alias for `Reg<CBC0_SPEC>`"]
pub type CBC0 = crate::Reg<cbc0::CBC0_SPEC>;
#[doc = "Channel Block Control Register (chid = 0)"]
pub mod cbc0;
#[doc = "CC0 register accessor: an alias for `Reg<CC0_SPEC>`"]
pub type CC0 = crate::Reg<cc0::CC0_SPEC>;
#[doc = "Channel Configuration Register (chid = 0)"]
pub mod cc0;
#[doc = "CDS_MSP0 register accessor: an alias for `Reg<CDS_MSP0_SPEC>`"]
pub type CDS_MSP0 = crate::Reg<cds_msp0::CDS_MSP0_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 0)"]
pub mod cds_msp0;
#[doc = "CSUS0 register accessor: an alias for `Reg<CSUS0_SPEC>`"]
pub type CSUS0 = crate::Reg<csus0::CSUS0_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 0)"]
pub mod csus0;
#[doc = "CDUS0 register accessor: an alias for `Reg<CDUS0_SPEC>`"]
pub type CDUS0 = crate::Reg<cdus0::CDUS0_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 0)"]
pub mod cdus0;
#[doc = "CIE1 register accessor: an alias for `Reg<CIE1_SPEC>`"]
pub type CIE1 = crate::Reg<cie1::CIE1_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 1)"]
pub mod cie1;
#[doc = "CID1 register accessor: an alias for `Reg<CID1_SPEC>`"]
pub type CID1 = crate::Reg<cid1::CID1_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 1)"]
pub mod cid1;
#[doc = "CIM1 register accessor: an alias for `Reg<CIM1_SPEC>`"]
pub type CIM1 = crate::Reg<cim1::CIM1_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 1)"]
pub mod cim1;
#[doc = "CIS1 register accessor: an alias for `Reg<CIS1_SPEC>`"]
pub type CIS1 = crate::Reg<cis1::CIS1_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 1)"]
pub mod cis1;
#[doc = "CSA1 register accessor: an alias for `Reg<CSA1_SPEC>`"]
pub type CSA1 = crate::Reg<csa1::CSA1_SPEC>;
#[doc = "Channel Source Address Register (chid = 1)"]
pub mod csa1;
#[doc = "CDA1 register accessor: an alias for `Reg<CDA1_SPEC>`"]
pub type CDA1 = crate::Reg<cda1::CDA1_SPEC>;
#[doc = "Channel Destination Address Register (chid = 1)"]
pub mod cda1;
#[doc = "CNDA1 register accessor: an alias for `Reg<CNDA1_SPEC>`"]
pub type CNDA1 = crate::Reg<cnda1::CNDA1_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 1)"]
pub mod cnda1;
#[doc = "CNDC1 register accessor: an alias for `Reg<CNDC1_SPEC>`"]
pub type CNDC1 = crate::Reg<cndc1::CNDC1_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 1)"]
pub mod cndc1;
#[doc = "CUBC1 register accessor: an alias for `Reg<CUBC1_SPEC>`"]
pub type CUBC1 = crate::Reg<cubc1::CUBC1_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 1)"]
pub mod cubc1;
#[doc = "CBC1 register accessor: an alias for `Reg<CBC1_SPEC>`"]
pub type CBC1 = crate::Reg<cbc1::CBC1_SPEC>;
#[doc = "Channel Block Control Register (chid = 1)"]
pub mod cbc1;
#[doc = "CC1 register accessor: an alias for `Reg<CC1_SPEC>`"]
pub type CC1 = crate::Reg<cc1::CC1_SPEC>;
#[doc = "Channel Configuration Register (chid = 1)"]
pub mod cc1;
#[doc = "CDS_MSP1 register accessor: an alias for `Reg<CDS_MSP1_SPEC>`"]
pub type CDS_MSP1 = crate::Reg<cds_msp1::CDS_MSP1_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 1)"]
pub mod cds_msp1;
#[doc = "CSUS1 register accessor: an alias for `Reg<CSUS1_SPEC>`"]
pub type CSUS1 = crate::Reg<csus1::CSUS1_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 1)"]
pub mod csus1;
#[doc = "CDUS1 register accessor: an alias for `Reg<CDUS1_SPEC>`"]
pub type CDUS1 = crate::Reg<cdus1::CDUS1_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 1)"]
pub mod cdus1;
#[doc = "CIE2 register accessor: an alias for `Reg<CIE2_SPEC>`"]
pub type CIE2 = crate::Reg<cie2::CIE2_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 2)"]
pub mod cie2;
#[doc = "CID2 register accessor: an alias for `Reg<CID2_SPEC>`"]
pub type CID2 = crate::Reg<cid2::CID2_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 2)"]
pub mod cid2;
#[doc = "CIM2 register accessor: an alias for `Reg<CIM2_SPEC>`"]
pub type CIM2 = crate::Reg<cim2::CIM2_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 2)"]
pub mod cim2;
#[doc = "CIS2 register accessor: an alias for `Reg<CIS2_SPEC>`"]
pub type CIS2 = crate::Reg<cis2::CIS2_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 2)"]
pub mod cis2;
#[doc = "CSA2 register accessor: an alias for `Reg<CSA2_SPEC>`"]
pub type CSA2 = crate::Reg<csa2::CSA2_SPEC>;
#[doc = "Channel Source Address Register (chid = 2)"]
pub mod csa2;
#[doc = "CDA2 register accessor: an alias for `Reg<CDA2_SPEC>`"]
pub type CDA2 = crate::Reg<cda2::CDA2_SPEC>;
#[doc = "Channel Destination Address Register (chid = 2)"]
pub mod cda2;
#[doc = "CNDA2 register accessor: an alias for `Reg<CNDA2_SPEC>`"]
pub type CNDA2 = crate::Reg<cnda2::CNDA2_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 2)"]
pub mod cnda2;
#[doc = "CNDC2 register accessor: an alias for `Reg<CNDC2_SPEC>`"]
pub type CNDC2 = crate::Reg<cndc2::CNDC2_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 2)"]
pub mod cndc2;
#[doc = "CUBC2 register accessor: an alias for `Reg<CUBC2_SPEC>`"]
pub type CUBC2 = crate::Reg<cubc2::CUBC2_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 2)"]
pub mod cubc2;
#[doc = "CBC2 register accessor: an alias for `Reg<CBC2_SPEC>`"]
pub type CBC2 = crate::Reg<cbc2::CBC2_SPEC>;
#[doc = "Channel Block Control Register (chid = 2)"]
pub mod cbc2;
#[doc = "CC2 register accessor: an alias for `Reg<CC2_SPEC>`"]
pub type CC2 = crate::Reg<cc2::CC2_SPEC>;
#[doc = "Channel Configuration Register (chid = 2)"]
pub mod cc2;
#[doc = "CDS_MSP2 register accessor: an alias for `Reg<CDS_MSP2_SPEC>`"]
pub type CDS_MSP2 = crate::Reg<cds_msp2::CDS_MSP2_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 2)"]
pub mod cds_msp2;
#[doc = "CSUS2 register accessor: an alias for `Reg<CSUS2_SPEC>`"]
pub type CSUS2 = crate::Reg<csus2::CSUS2_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 2)"]
pub mod csus2;
#[doc = "CDUS2 register accessor: an alias for `Reg<CDUS2_SPEC>`"]
pub type CDUS2 = crate::Reg<cdus2::CDUS2_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 2)"]
pub mod cdus2;
#[doc = "CIE3 register accessor: an alias for `Reg<CIE3_SPEC>`"]
pub type CIE3 = crate::Reg<cie3::CIE3_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 3)"]
pub mod cie3;
#[doc = "CID3 register accessor: an alias for `Reg<CID3_SPEC>`"]
pub type CID3 = crate::Reg<cid3::CID3_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 3)"]
pub mod cid3;
#[doc = "CIM3 register accessor: an alias for `Reg<CIM3_SPEC>`"]
pub type CIM3 = crate::Reg<cim3::CIM3_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 3)"]
pub mod cim3;
#[doc = "CIS3 register accessor: an alias for `Reg<CIS3_SPEC>`"]
pub type CIS3 = crate::Reg<cis3::CIS3_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 3)"]
pub mod cis3;
#[doc = "CSA3 register accessor: an alias for `Reg<CSA3_SPEC>`"]
pub type CSA3 = crate::Reg<csa3::CSA3_SPEC>;
#[doc = "Channel Source Address Register (chid = 3)"]
pub mod csa3;
#[doc = "CDA3 register accessor: an alias for `Reg<CDA3_SPEC>`"]
pub type CDA3 = crate::Reg<cda3::CDA3_SPEC>;
#[doc = "Channel Destination Address Register (chid = 3)"]
pub mod cda3;
#[doc = "CNDA3 register accessor: an alias for `Reg<CNDA3_SPEC>`"]
pub type CNDA3 = crate::Reg<cnda3::CNDA3_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 3)"]
pub mod cnda3;
#[doc = "CNDC3 register accessor: an alias for `Reg<CNDC3_SPEC>`"]
pub type CNDC3 = crate::Reg<cndc3::CNDC3_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 3)"]
pub mod cndc3;
#[doc = "CUBC3 register accessor: an alias for `Reg<CUBC3_SPEC>`"]
pub type CUBC3 = crate::Reg<cubc3::CUBC3_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 3)"]
pub mod cubc3;
#[doc = "CBC3 register accessor: an alias for `Reg<CBC3_SPEC>`"]
pub type CBC3 = crate::Reg<cbc3::CBC3_SPEC>;
#[doc = "Channel Block Control Register (chid = 3)"]
pub mod cbc3;
#[doc = "CC3 register accessor: an alias for `Reg<CC3_SPEC>`"]
pub type CC3 = crate::Reg<cc3::CC3_SPEC>;
#[doc = "Channel Configuration Register (chid = 3)"]
pub mod cc3;
#[doc = "CDS_MSP3 register accessor: an alias for `Reg<CDS_MSP3_SPEC>`"]
pub type CDS_MSP3 = crate::Reg<cds_msp3::CDS_MSP3_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 3)"]
pub mod cds_msp3;
#[doc = "CSUS3 register accessor: an alias for `Reg<CSUS3_SPEC>`"]
pub type CSUS3 = crate::Reg<csus3::CSUS3_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 3)"]
pub mod csus3;
#[doc = "CDUS3 register accessor: an alias for `Reg<CDUS3_SPEC>`"]
pub type CDUS3 = crate::Reg<cdus3::CDUS3_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 3)"]
pub mod cdus3;
#[doc = "CIE4 register accessor: an alias for `Reg<CIE4_SPEC>`"]
pub type CIE4 = crate::Reg<cie4::CIE4_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 4)"]
pub mod cie4;
#[doc = "CID4 register accessor: an alias for `Reg<CID4_SPEC>`"]
pub type CID4 = crate::Reg<cid4::CID4_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 4)"]
pub mod cid4;
#[doc = "CIM4 register accessor: an alias for `Reg<CIM4_SPEC>`"]
pub type CIM4 = crate::Reg<cim4::CIM4_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 4)"]
pub mod cim4;
#[doc = "CIS4 register accessor: an alias for `Reg<CIS4_SPEC>`"]
pub type CIS4 = crate::Reg<cis4::CIS4_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 4)"]
pub mod cis4;
#[doc = "CSA4 register accessor: an alias for `Reg<CSA4_SPEC>`"]
pub type CSA4 = crate::Reg<csa4::CSA4_SPEC>;
#[doc = "Channel Source Address Register (chid = 4)"]
pub mod csa4;
#[doc = "CDA4 register accessor: an alias for `Reg<CDA4_SPEC>`"]
pub type CDA4 = crate::Reg<cda4::CDA4_SPEC>;
#[doc = "Channel Destination Address Register (chid = 4)"]
pub mod cda4;
#[doc = "CNDA4 register accessor: an alias for `Reg<CNDA4_SPEC>`"]
pub type CNDA4 = crate::Reg<cnda4::CNDA4_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 4)"]
pub mod cnda4;
#[doc = "CNDC4 register accessor: an alias for `Reg<CNDC4_SPEC>`"]
pub type CNDC4 = crate::Reg<cndc4::CNDC4_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 4)"]
pub mod cndc4;
#[doc = "CUBC4 register accessor: an alias for `Reg<CUBC4_SPEC>`"]
pub type CUBC4 = crate::Reg<cubc4::CUBC4_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 4)"]
pub mod cubc4;
#[doc = "CBC4 register accessor: an alias for `Reg<CBC4_SPEC>`"]
pub type CBC4 = crate::Reg<cbc4::CBC4_SPEC>;
#[doc = "Channel Block Control Register (chid = 4)"]
pub mod cbc4;
#[doc = "CC4 register accessor: an alias for `Reg<CC4_SPEC>`"]
pub type CC4 = crate::Reg<cc4::CC4_SPEC>;
#[doc = "Channel Configuration Register (chid = 4)"]
pub mod cc4;
#[doc = "CDS_MSP4 register accessor: an alias for `Reg<CDS_MSP4_SPEC>`"]
pub type CDS_MSP4 = crate::Reg<cds_msp4::CDS_MSP4_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 4)"]
pub mod cds_msp4;
#[doc = "CSUS4 register accessor: an alias for `Reg<CSUS4_SPEC>`"]
pub type CSUS4 = crate::Reg<csus4::CSUS4_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 4)"]
pub mod csus4;
#[doc = "CDUS4 register accessor: an alias for `Reg<CDUS4_SPEC>`"]
pub type CDUS4 = crate::Reg<cdus4::CDUS4_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 4)"]
pub mod cdus4;
#[doc = "CIE5 register accessor: an alias for `Reg<CIE5_SPEC>`"]
pub type CIE5 = crate::Reg<cie5::CIE5_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 5)"]
pub mod cie5;
#[doc = "CID5 register accessor: an alias for `Reg<CID5_SPEC>`"]
pub type CID5 = crate::Reg<cid5::CID5_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 5)"]
pub mod cid5;
#[doc = "CIM5 register accessor: an alias for `Reg<CIM5_SPEC>`"]
pub type CIM5 = crate::Reg<cim5::CIM5_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 5)"]
pub mod cim5;
#[doc = "CIS5 register accessor: an alias for `Reg<CIS5_SPEC>`"]
pub type CIS5 = crate::Reg<cis5::CIS5_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 5)"]
pub mod cis5;
#[doc = "CSA5 register accessor: an alias for `Reg<CSA5_SPEC>`"]
pub type CSA5 = crate::Reg<csa5::CSA5_SPEC>;
#[doc = "Channel Source Address Register (chid = 5)"]
pub mod csa5;
#[doc = "CDA5 register accessor: an alias for `Reg<CDA5_SPEC>`"]
pub type CDA5 = crate::Reg<cda5::CDA5_SPEC>;
#[doc = "Channel Destination Address Register (chid = 5)"]
pub mod cda5;
#[doc = "CNDA5 register accessor: an alias for `Reg<CNDA5_SPEC>`"]
pub type CNDA5 = crate::Reg<cnda5::CNDA5_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 5)"]
pub mod cnda5;
#[doc = "CNDC5 register accessor: an alias for `Reg<CNDC5_SPEC>`"]
pub type CNDC5 = crate::Reg<cndc5::CNDC5_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 5)"]
pub mod cndc5;
#[doc = "CUBC5 register accessor: an alias for `Reg<CUBC5_SPEC>`"]
pub type CUBC5 = crate::Reg<cubc5::CUBC5_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 5)"]
pub mod cubc5;
#[doc = "CBC5 register accessor: an alias for `Reg<CBC5_SPEC>`"]
pub type CBC5 = crate::Reg<cbc5::CBC5_SPEC>;
#[doc = "Channel Block Control Register (chid = 5)"]
pub mod cbc5;
#[doc = "CC5 register accessor: an alias for `Reg<CC5_SPEC>`"]
pub type CC5 = crate::Reg<cc5::CC5_SPEC>;
#[doc = "Channel Configuration Register (chid = 5)"]
pub mod cc5;
#[doc = "CDS_MSP5 register accessor: an alias for `Reg<CDS_MSP5_SPEC>`"]
pub type CDS_MSP5 = crate::Reg<cds_msp5::CDS_MSP5_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 5)"]
pub mod cds_msp5;
#[doc = "CSUS5 register accessor: an alias for `Reg<CSUS5_SPEC>`"]
pub type CSUS5 = crate::Reg<csus5::CSUS5_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 5)"]
pub mod csus5;
#[doc = "CDUS5 register accessor: an alias for `Reg<CDUS5_SPEC>`"]
pub type CDUS5 = crate::Reg<cdus5::CDUS5_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 5)"]
pub mod cdus5;
#[doc = "CIE6 register accessor: an alias for `Reg<CIE6_SPEC>`"]
pub type CIE6 = crate::Reg<cie6::CIE6_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 6)"]
pub mod cie6;
#[doc = "CID6 register accessor: an alias for `Reg<CID6_SPEC>`"]
pub type CID6 = crate::Reg<cid6::CID6_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 6)"]
pub mod cid6;
#[doc = "CIM6 register accessor: an alias for `Reg<CIM6_SPEC>`"]
pub type CIM6 = crate::Reg<cim6::CIM6_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 6)"]
pub mod cim6;
#[doc = "CIS6 register accessor: an alias for `Reg<CIS6_SPEC>`"]
pub type CIS6 = crate::Reg<cis6::CIS6_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 6)"]
pub mod cis6;
#[doc = "CSA6 register accessor: an alias for `Reg<CSA6_SPEC>`"]
pub type CSA6 = crate::Reg<csa6::CSA6_SPEC>;
#[doc = "Channel Source Address Register (chid = 6)"]
pub mod csa6;
#[doc = "CDA6 register accessor: an alias for `Reg<CDA6_SPEC>`"]
pub type CDA6 = crate::Reg<cda6::CDA6_SPEC>;
#[doc = "Channel Destination Address Register (chid = 6)"]
pub mod cda6;
#[doc = "CNDA6 register accessor: an alias for `Reg<CNDA6_SPEC>`"]
pub type CNDA6 = crate::Reg<cnda6::CNDA6_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 6)"]
pub mod cnda6;
#[doc = "CNDC6 register accessor: an alias for `Reg<CNDC6_SPEC>`"]
pub type CNDC6 = crate::Reg<cndc6::CNDC6_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 6)"]
pub mod cndc6;
#[doc = "CUBC6 register accessor: an alias for `Reg<CUBC6_SPEC>`"]
pub type CUBC6 = crate::Reg<cubc6::CUBC6_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 6)"]
pub mod cubc6;
#[doc = "CBC6 register accessor: an alias for `Reg<CBC6_SPEC>`"]
pub type CBC6 = crate::Reg<cbc6::CBC6_SPEC>;
#[doc = "Channel Block Control Register (chid = 6)"]
pub mod cbc6;
#[doc = "CC6 register accessor: an alias for `Reg<CC6_SPEC>`"]
pub type CC6 = crate::Reg<cc6::CC6_SPEC>;
#[doc = "Channel Configuration Register (chid = 6)"]
pub mod cc6;
#[doc = "CDS_MSP6 register accessor: an alias for `Reg<CDS_MSP6_SPEC>`"]
pub type CDS_MSP6 = crate::Reg<cds_msp6::CDS_MSP6_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 6)"]
pub mod cds_msp6;
#[doc = "CSUS6 register accessor: an alias for `Reg<CSUS6_SPEC>`"]
pub type CSUS6 = crate::Reg<csus6::CSUS6_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 6)"]
pub mod csus6;
#[doc = "CDUS6 register accessor: an alias for `Reg<CDUS6_SPEC>`"]
pub type CDUS6 = crate::Reg<cdus6::CDUS6_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 6)"]
pub mod cdus6;
#[doc = "CIE7 register accessor: an alias for `Reg<CIE7_SPEC>`"]
pub type CIE7 = crate::Reg<cie7::CIE7_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 7)"]
pub mod cie7;
#[doc = "CID7 register accessor: an alias for `Reg<CID7_SPEC>`"]
pub type CID7 = crate::Reg<cid7::CID7_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 7)"]
pub mod cid7;
#[doc = "CIM7 register accessor: an alias for `Reg<CIM7_SPEC>`"]
pub type CIM7 = crate::Reg<cim7::CIM7_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 7)"]
pub mod cim7;
#[doc = "CIS7 register accessor: an alias for `Reg<CIS7_SPEC>`"]
pub type CIS7 = crate::Reg<cis7::CIS7_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 7)"]
pub mod cis7;
#[doc = "CSA7 register accessor: an alias for `Reg<CSA7_SPEC>`"]
pub type CSA7 = crate::Reg<csa7::CSA7_SPEC>;
#[doc = "Channel Source Address Register (chid = 7)"]
pub mod csa7;
#[doc = "CDA7 register accessor: an alias for `Reg<CDA7_SPEC>`"]
pub type CDA7 = crate::Reg<cda7::CDA7_SPEC>;
#[doc = "Channel Destination Address Register (chid = 7)"]
pub mod cda7;
#[doc = "CNDA7 register accessor: an alias for `Reg<CNDA7_SPEC>`"]
pub type CNDA7 = crate::Reg<cnda7::CNDA7_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 7)"]
pub mod cnda7;
#[doc = "CNDC7 register accessor: an alias for `Reg<CNDC7_SPEC>`"]
pub type CNDC7 = crate::Reg<cndc7::CNDC7_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 7)"]
pub mod cndc7;
#[doc = "CUBC7 register accessor: an alias for `Reg<CUBC7_SPEC>`"]
pub type CUBC7 = crate::Reg<cubc7::CUBC7_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 7)"]
pub mod cubc7;
#[doc = "CBC7 register accessor: an alias for `Reg<CBC7_SPEC>`"]
pub type CBC7 = crate::Reg<cbc7::CBC7_SPEC>;
#[doc = "Channel Block Control Register (chid = 7)"]
pub mod cbc7;
#[doc = "CC7 register accessor: an alias for `Reg<CC7_SPEC>`"]
pub type CC7 = crate::Reg<cc7::CC7_SPEC>;
#[doc = "Channel Configuration Register (chid = 7)"]
pub mod cc7;
#[doc = "CDS_MSP7 register accessor: an alias for `Reg<CDS_MSP7_SPEC>`"]
pub type CDS_MSP7 = crate::Reg<cds_msp7::CDS_MSP7_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 7)"]
pub mod cds_msp7;
#[doc = "CSUS7 register accessor: an alias for `Reg<CSUS7_SPEC>`"]
pub type CSUS7 = crate::Reg<csus7::CSUS7_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 7)"]
pub mod csus7;
#[doc = "CDUS7 register accessor: an alias for `Reg<CDUS7_SPEC>`"]
pub type CDUS7 = crate::Reg<cdus7::CDUS7_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 7)"]
pub mod cdus7;
#[doc = "CIE8 register accessor: an alias for `Reg<CIE8_SPEC>`"]
pub type CIE8 = crate::Reg<cie8::CIE8_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 8)"]
pub mod cie8;
#[doc = "CID8 register accessor: an alias for `Reg<CID8_SPEC>`"]
pub type CID8 = crate::Reg<cid8::CID8_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 8)"]
pub mod cid8;
#[doc = "CIM8 register accessor: an alias for `Reg<CIM8_SPEC>`"]
pub type CIM8 = crate::Reg<cim8::CIM8_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 8)"]
pub mod cim8;
#[doc = "CIS8 register accessor: an alias for `Reg<CIS8_SPEC>`"]
pub type CIS8 = crate::Reg<cis8::CIS8_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 8)"]
pub mod cis8;
#[doc = "CSA8 register accessor: an alias for `Reg<CSA8_SPEC>`"]
pub type CSA8 = crate::Reg<csa8::CSA8_SPEC>;
#[doc = "Channel Source Address Register (chid = 8)"]
pub mod csa8;
#[doc = "CDA8 register accessor: an alias for `Reg<CDA8_SPEC>`"]
pub type CDA8 = crate::Reg<cda8::CDA8_SPEC>;
#[doc = "Channel Destination Address Register (chid = 8)"]
pub mod cda8;
#[doc = "CNDA8 register accessor: an alias for `Reg<CNDA8_SPEC>`"]
pub type CNDA8 = crate::Reg<cnda8::CNDA8_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 8)"]
pub mod cnda8;
#[doc = "CNDC8 register accessor: an alias for `Reg<CNDC8_SPEC>`"]
pub type CNDC8 = crate::Reg<cndc8::CNDC8_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 8)"]
pub mod cndc8;
#[doc = "CUBC8 register accessor: an alias for `Reg<CUBC8_SPEC>`"]
pub type CUBC8 = crate::Reg<cubc8::CUBC8_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 8)"]
pub mod cubc8;
#[doc = "CBC8 register accessor: an alias for `Reg<CBC8_SPEC>`"]
pub type CBC8 = crate::Reg<cbc8::CBC8_SPEC>;
#[doc = "Channel Block Control Register (chid = 8)"]
pub mod cbc8;
#[doc = "CC8 register accessor: an alias for `Reg<CC8_SPEC>`"]
pub type CC8 = crate::Reg<cc8::CC8_SPEC>;
#[doc = "Channel Configuration Register (chid = 8)"]
pub mod cc8;
#[doc = "CDS_MSP8 register accessor: an alias for `Reg<CDS_MSP8_SPEC>`"]
pub type CDS_MSP8 = crate::Reg<cds_msp8::CDS_MSP8_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 8)"]
pub mod cds_msp8;
#[doc = "CSUS8 register accessor: an alias for `Reg<CSUS8_SPEC>`"]
pub type CSUS8 = crate::Reg<csus8::CSUS8_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 8)"]
pub mod csus8;
#[doc = "CDUS8 register accessor: an alias for `Reg<CDUS8_SPEC>`"]
pub type CDUS8 = crate::Reg<cdus8::CDUS8_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 8)"]
pub mod cdus8;
#[doc = "CIE9 register accessor: an alias for `Reg<CIE9_SPEC>`"]
pub type CIE9 = crate::Reg<cie9::CIE9_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 9)"]
pub mod cie9;
#[doc = "CID9 register accessor: an alias for `Reg<CID9_SPEC>`"]
pub type CID9 = crate::Reg<cid9::CID9_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 9)"]
pub mod cid9;
#[doc = "CIM9 register accessor: an alias for `Reg<CIM9_SPEC>`"]
pub type CIM9 = crate::Reg<cim9::CIM9_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 9)"]
pub mod cim9;
#[doc = "CIS9 register accessor: an alias for `Reg<CIS9_SPEC>`"]
pub type CIS9 = crate::Reg<cis9::CIS9_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 9)"]
pub mod cis9;
#[doc = "CSA9 register accessor: an alias for `Reg<CSA9_SPEC>`"]
pub type CSA9 = crate::Reg<csa9::CSA9_SPEC>;
#[doc = "Channel Source Address Register (chid = 9)"]
pub mod csa9;
#[doc = "CDA9 register accessor: an alias for `Reg<CDA9_SPEC>`"]
pub type CDA9 = crate::Reg<cda9::CDA9_SPEC>;
#[doc = "Channel Destination Address Register (chid = 9)"]
pub mod cda9;
#[doc = "CNDA9 register accessor: an alias for `Reg<CNDA9_SPEC>`"]
pub type CNDA9 = crate::Reg<cnda9::CNDA9_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 9)"]
pub mod cnda9;
#[doc = "CNDC9 register accessor: an alias for `Reg<CNDC9_SPEC>`"]
pub type CNDC9 = crate::Reg<cndc9::CNDC9_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 9)"]
pub mod cndc9;
#[doc = "CUBC9 register accessor: an alias for `Reg<CUBC9_SPEC>`"]
pub type CUBC9 = crate::Reg<cubc9::CUBC9_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 9)"]
pub mod cubc9;
#[doc = "CBC9 register accessor: an alias for `Reg<CBC9_SPEC>`"]
pub type CBC9 = crate::Reg<cbc9::CBC9_SPEC>;
#[doc = "Channel Block Control Register (chid = 9)"]
pub mod cbc9;
#[doc = "CC9 register accessor: an alias for `Reg<CC9_SPEC>`"]
pub type CC9 = crate::Reg<cc9::CC9_SPEC>;
#[doc = "Channel Configuration Register (chid = 9)"]
pub mod cc9;
#[doc = "CDS_MSP9 register accessor: an alias for `Reg<CDS_MSP9_SPEC>`"]
pub type CDS_MSP9 = crate::Reg<cds_msp9::CDS_MSP9_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 9)"]
pub mod cds_msp9;
#[doc = "CSUS9 register accessor: an alias for `Reg<CSUS9_SPEC>`"]
pub type CSUS9 = crate::Reg<csus9::CSUS9_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 9)"]
pub mod csus9;
#[doc = "CDUS9 register accessor: an alias for `Reg<CDUS9_SPEC>`"]
pub type CDUS9 = crate::Reg<cdus9::CDUS9_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 9)"]
pub mod cdus9;
#[doc = "CIE10 register accessor: an alias for `Reg<CIE10_SPEC>`"]
pub type CIE10 = crate::Reg<cie10::CIE10_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 10)"]
pub mod cie10;
#[doc = "CID10 register accessor: an alias for `Reg<CID10_SPEC>`"]
pub type CID10 = crate::Reg<cid10::CID10_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 10)"]
pub mod cid10;
#[doc = "CIM10 register accessor: an alias for `Reg<CIM10_SPEC>`"]
pub type CIM10 = crate::Reg<cim10::CIM10_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 10)"]
pub mod cim10;
#[doc = "CIS10 register accessor: an alias for `Reg<CIS10_SPEC>`"]
pub type CIS10 = crate::Reg<cis10::CIS10_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 10)"]
pub mod cis10;
#[doc = "CSA10 register accessor: an alias for `Reg<CSA10_SPEC>`"]
pub type CSA10 = crate::Reg<csa10::CSA10_SPEC>;
#[doc = "Channel Source Address Register (chid = 10)"]
pub mod csa10;
#[doc = "CDA10 register accessor: an alias for `Reg<CDA10_SPEC>`"]
pub type CDA10 = crate::Reg<cda10::CDA10_SPEC>;
#[doc = "Channel Destination Address Register (chid = 10)"]
pub mod cda10;
#[doc = "CNDA10 register accessor: an alias for `Reg<CNDA10_SPEC>`"]
pub type CNDA10 = crate::Reg<cnda10::CNDA10_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 10)"]
pub mod cnda10;
#[doc = "CNDC10 register accessor: an alias for `Reg<CNDC10_SPEC>`"]
pub type CNDC10 = crate::Reg<cndc10::CNDC10_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 10)"]
pub mod cndc10;
#[doc = "CUBC10 register accessor: an alias for `Reg<CUBC10_SPEC>`"]
pub type CUBC10 = crate::Reg<cubc10::CUBC10_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 10)"]
pub mod cubc10;
#[doc = "CBC10 register accessor: an alias for `Reg<CBC10_SPEC>`"]
pub type CBC10 = crate::Reg<cbc10::CBC10_SPEC>;
#[doc = "Channel Block Control Register (chid = 10)"]
pub mod cbc10;
#[doc = "CC10 register accessor: an alias for `Reg<CC10_SPEC>`"]
pub type CC10 = crate::Reg<cc10::CC10_SPEC>;
#[doc = "Channel Configuration Register (chid = 10)"]
pub mod cc10;
#[doc = "CDS_MSP10 register accessor: an alias for `Reg<CDS_MSP10_SPEC>`"]
pub type CDS_MSP10 = crate::Reg<cds_msp10::CDS_MSP10_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 10)"]
pub mod cds_msp10;
#[doc = "CSUS10 register accessor: an alias for `Reg<CSUS10_SPEC>`"]
pub type CSUS10 = crate::Reg<csus10::CSUS10_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 10)"]
pub mod csus10;
#[doc = "CDUS10 register accessor: an alias for `Reg<CDUS10_SPEC>`"]
pub type CDUS10 = crate::Reg<cdus10::CDUS10_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 10)"]
pub mod cdus10;
#[doc = "CIE11 register accessor: an alias for `Reg<CIE11_SPEC>`"]
pub type CIE11 = crate::Reg<cie11::CIE11_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 11)"]
pub mod cie11;
#[doc = "CID11 register accessor: an alias for `Reg<CID11_SPEC>`"]
pub type CID11 = crate::Reg<cid11::CID11_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 11)"]
pub mod cid11;
#[doc = "CIM11 register accessor: an alias for `Reg<CIM11_SPEC>`"]
pub type CIM11 = crate::Reg<cim11::CIM11_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 11)"]
pub mod cim11;
#[doc = "CIS11 register accessor: an alias for `Reg<CIS11_SPEC>`"]
pub type CIS11 = crate::Reg<cis11::CIS11_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 11)"]
pub mod cis11;
#[doc = "CSA11 register accessor: an alias for `Reg<CSA11_SPEC>`"]
pub type CSA11 = crate::Reg<csa11::CSA11_SPEC>;
#[doc = "Channel Source Address Register (chid = 11)"]
pub mod csa11;
#[doc = "CDA11 register accessor: an alias for `Reg<CDA11_SPEC>`"]
pub type CDA11 = crate::Reg<cda11::CDA11_SPEC>;
#[doc = "Channel Destination Address Register (chid = 11)"]
pub mod cda11;
#[doc = "CNDA11 register accessor: an alias for `Reg<CNDA11_SPEC>`"]
pub type CNDA11 = crate::Reg<cnda11::CNDA11_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 11)"]
pub mod cnda11;
#[doc = "CNDC11 register accessor: an alias for `Reg<CNDC11_SPEC>`"]
pub type CNDC11 = crate::Reg<cndc11::CNDC11_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 11)"]
pub mod cndc11;
#[doc = "CUBC11 register accessor: an alias for `Reg<CUBC11_SPEC>`"]
pub type CUBC11 = crate::Reg<cubc11::CUBC11_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 11)"]
pub mod cubc11;
#[doc = "CBC11 register accessor: an alias for `Reg<CBC11_SPEC>`"]
pub type CBC11 = crate::Reg<cbc11::CBC11_SPEC>;
#[doc = "Channel Block Control Register (chid = 11)"]
pub mod cbc11;
#[doc = "CC11 register accessor: an alias for `Reg<CC11_SPEC>`"]
pub type CC11 = crate::Reg<cc11::CC11_SPEC>;
#[doc = "Channel Configuration Register (chid = 11)"]
pub mod cc11;
#[doc = "CDS_MSP11 register accessor: an alias for `Reg<CDS_MSP11_SPEC>`"]
pub type CDS_MSP11 = crate::Reg<cds_msp11::CDS_MSP11_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 11)"]
pub mod cds_msp11;
#[doc = "CSUS11 register accessor: an alias for `Reg<CSUS11_SPEC>`"]
pub type CSUS11 = crate::Reg<csus11::CSUS11_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 11)"]
pub mod csus11;
#[doc = "CDUS11 register accessor: an alias for `Reg<CDUS11_SPEC>`"]
pub type CDUS11 = crate::Reg<cdus11::CDUS11_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 11)"]
pub mod cdus11;
#[doc = "CIE12 register accessor: an alias for `Reg<CIE12_SPEC>`"]
pub type CIE12 = crate::Reg<cie12::CIE12_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 12)"]
pub mod cie12;
#[doc = "CID12 register accessor: an alias for `Reg<CID12_SPEC>`"]
pub type CID12 = crate::Reg<cid12::CID12_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 12)"]
pub mod cid12;
#[doc = "CIM12 register accessor: an alias for `Reg<CIM12_SPEC>`"]
pub type CIM12 = crate::Reg<cim12::CIM12_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 12)"]
pub mod cim12;
#[doc = "CIS12 register accessor: an alias for `Reg<CIS12_SPEC>`"]
pub type CIS12 = crate::Reg<cis12::CIS12_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 12)"]
pub mod cis12;
#[doc = "CSA12 register accessor: an alias for `Reg<CSA12_SPEC>`"]
pub type CSA12 = crate::Reg<csa12::CSA12_SPEC>;
#[doc = "Channel Source Address Register (chid = 12)"]
pub mod csa12;
#[doc = "CDA12 register accessor: an alias for `Reg<CDA12_SPEC>`"]
pub type CDA12 = crate::Reg<cda12::CDA12_SPEC>;
#[doc = "Channel Destination Address Register (chid = 12)"]
pub mod cda12;
#[doc = "CNDA12 register accessor: an alias for `Reg<CNDA12_SPEC>`"]
pub type CNDA12 = crate::Reg<cnda12::CNDA12_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 12)"]
pub mod cnda12;
#[doc = "CNDC12 register accessor: an alias for `Reg<CNDC12_SPEC>`"]
pub type CNDC12 = crate::Reg<cndc12::CNDC12_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 12)"]
pub mod cndc12;
#[doc = "CUBC12 register accessor: an alias for `Reg<CUBC12_SPEC>`"]
pub type CUBC12 = crate::Reg<cubc12::CUBC12_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 12)"]
pub mod cubc12;
#[doc = "CBC12 register accessor: an alias for `Reg<CBC12_SPEC>`"]
pub type CBC12 = crate::Reg<cbc12::CBC12_SPEC>;
#[doc = "Channel Block Control Register (chid = 12)"]
pub mod cbc12;
#[doc = "CC12 register accessor: an alias for `Reg<CC12_SPEC>`"]
pub type CC12 = crate::Reg<cc12::CC12_SPEC>;
#[doc = "Channel Configuration Register (chid = 12)"]
pub mod cc12;
#[doc = "CDS_MSP12 register accessor: an alias for `Reg<CDS_MSP12_SPEC>`"]
pub type CDS_MSP12 = crate::Reg<cds_msp12::CDS_MSP12_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 12)"]
pub mod cds_msp12;
#[doc = "CSUS12 register accessor: an alias for `Reg<CSUS12_SPEC>`"]
pub type CSUS12 = crate::Reg<csus12::CSUS12_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 12)"]
pub mod csus12;
#[doc = "CDUS12 register accessor: an alias for `Reg<CDUS12_SPEC>`"]
pub type CDUS12 = crate::Reg<cdus12::CDUS12_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 12)"]
pub mod cdus12;
#[doc = "CIE13 register accessor: an alias for `Reg<CIE13_SPEC>`"]
pub type CIE13 = crate::Reg<cie13::CIE13_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 13)"]
pub mod cie13;
#[doc = "CID13 register accessor: an alias for `Reg<CID13_SPEC>`"]
pub type CID13 = crate::Reg<cid13::CID13_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 13)"]
pub mod cid13;
#[doc = "CIM13 register accessor: an alias for `Reg<CIM13_SPEC>`"]
pub type CIM13 = crate::Reg<cim13::CIM13_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 13)"]
pub mod cim13;
#[doc = "CIS13 register accessor: an alias for `Reg<CIS13_SPEC>`"]
pub type CIS13 = crate::Reg<cis13::CIS13_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 13)"]
pub mod cis13;
#[doc = "CSA13 register accessor: an alias for `Reg<CSA13_SPEC>`"]
pub type CSA13 = crate::Reg<csa13::CSA13_SPEC>;
#[doc = "Channel Source Address Register (chid = 13)"]
pub mod csa13;
#[doc = "CDA13 register accessor: an alias for `Reg<CDA13_SPEC>`"]
pub type CDA13 = crate::Reg<cda13::CDA13_SPEC>;
#[doc = "Channel Destination Address Register (chid = 13)"]
pub mod cda13;
#[doc = "CNDA13 register accessor: an alias for `Reg<CNDA13_SPEC>`"]
pub type CNDA13 = crate::Reg<cnda13::CNDA13_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 13)"]
pub mod cnda13;
#[doc = "CNDC13 register accessor: an alias for `Reg<CNDC13_SPEC>`"]
pub type CNDC13 = crate::Reg<cndc13::CNDC13_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 13)"]
pub mod cndc13;
#[doc = "CUBC13 register accessor: an alias for `Reg<CUBC13_SPEC>`"]
pub type CUBC13 = crate::Reg<cubc13::CUBC13_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 13)"]
pub mod cubc13;
#[doc = "CBC13 register accessor: an alias for `Reg<CBC13_SPEC>`"]
pub type CBC13 = crate::Reg<cbc13::CBC13_SPEC>;
#[doc = "Channel Block Control Register (chid = 13)"]
pub mod cbc13;
#[doc = "CC13 register accessor: an alias for `Reg<CC13_SPEC>`"]
pub type CC13 = crate::Reg<cc13::CC13_SPEC>;
#[doc = "Channel Configuration Register (chid = 13)"]
pub mod cc13;
#[doc = "CDS_MSP13 register accessor: an alias for `Reg<CDS_MSP13_SPEC>`"]
pub type CDS_MSP13 = crate::Reg<cds_msp13::CDS_MSP13_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 13)"]
pub mod cds_msp13;
#[doc = "CSUS13 register accessor: an alias for `Reg<CSUS13_SPEC>`"]
pub type CSUS13 = crate::Reg<csus13::CSUS13_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 13)"]
pub mod csus13;
#[doc = "CDUS13 register accessor: an alias for `Reg<CDUS13_SPEC>`"]
pub type CDUS13 = crate::Reg<cdus13::CDUS13_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 13)"]
pub mod cdus13;
#[doc = "CIE14 register accessor: an alias for `Reg<CIE14_SPEC>`"]
pub type CIE14 = crate::Reg<cie14::CIE14_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 14)"]
pub mod cie14;
#[doc = "CID14 register accessor: an alias for `Reg<CID14_SPEC>`"]
pub type CID14 = crate::Reg<cid14::CID14_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 14)"]
pub mod cid14;
#[doc = "CIM14 register accessor: an alias for `Reg<CIM14_SPEC>`"]
pub type CIM14 = crate::Reg<cim14::CIM14_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 14)"]
pub mod cim14;
#[doc = "CIS14 register accessor: an alias for `Reg<CIS14_SPEC>`"]
pub type CIS14 = crate::Reg<cis14::CIS14_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 14)"]
pub mod cis14;
#[doc = "CSA14 register accessor: an alias for `Reg<CSA14_SPEC>`"]
pub type CSA14 = crate::Reg<csa14::CSA14_SPEC>;
#[doc = "Channel Source Address Register (chid = 14)"]
pub mod csa14;
#[doc = "CDA14 register accessor: an alias for `Reg<CDA14_SPEC>`"]
pub type CDA14 = crate::Reg<cda14::CDA14_SPEC>;
#[doc = "Channel Destination Address Register (chid = 14)"]
pub mod cda14;
#[doc = "CNDA14 register accessor: an alias for `Reg<CNDA14_SPEC>`"]
pub type CNDA14 = crate::Reg<cnda14::CNDA14_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 14)"]
pub mod cnda14;
#[doc = "CNDC14 register accessor: an alias for `Reg<CNDC14_SPEC>`"]
pub type CNDC14 = crate::Reg<cndc14::CNDC14_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 14)"]
pub mod cndc14;
#[doc = "CUBC14 register accessor: an alias for `Reg<CUBC14_SPEC>`"]
pub type CUBC14 = crate::Reg<cubc14::CUBC14_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 14)"]
pub mod cubc14;
#[doc = "CBC14 register accessor: an alias for `Reg<CBC14_SPEC>`"]
pub type CBC14 = crate::Reg<cbc14::CBC14_SPEC>;
#[doc = "Channel Block Control Register (chid = 14)"]
pub mod cbc14;
#[doc = "CC14 register accessor: an alias for `Reg<CC14_SPEC>`"]
pub type CC14 = crate::Reg<cc14::CC14_SPEC>;
#[doc = "Channel Configuration Register (chid = 14)"]
pub mod cc14;
#[doc = "CDS_MSP14 register accessor: an alias for `Reg<CDS_MSP14_SPEC>`"]
pub type CDS_MSP14 = crate::Reg<cds_msp14::CDS_MSP14_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 14)"]
pub mod cds_msp14;
#[doc = "CSUS14 register accessor: an alias for `Reg<CSUS14_SPEC>`"]
pub type CSUS14 = crate::Reg<csus14::CSUS14_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 14)"]
pub mod csus14;
#[doc = "CDUS14 register accessor: an alias for `Reg<CDUS14_SPEC>`"]
pub type CDUS14 = crate::Reg<cdus14::CDUS14_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 14)"]
pub mod cdus14;
#[doc = "CIE15 register accessor: an alias for `Reg<CIE15_SPEC>`"]
pub type CIE15 = crate::Reg<cie15::CIE15_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 15)"]
pub mod cie15;
#[doc = "CID15 register accessor: an alias for `Reg<CID15_SPEC>`"]
pub type CID15 = crate::Reg<cid15::CID15_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 15)"]
pub mod cid15;
#[doc = "CIM15 register accessor: an alias for `Reg<CIM15_SPEC>`"]
pub type CIM15 = crate::Reg<cim15::CIM15_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 15)"]
pub mod cim15;
#[doc = "CIS15 register accessor: an alias for `Reg<CIS15_SPEC>`"]
pub type CIS15 = crate::Reg<cis15::CIS15_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 15)"]
pub mod cis15;
#[doc = "CSA15 register accessor: an alias for `Reg<CSA15_SPEC>`"]
pub type CSA15 = crate::Reg<csa15::CSA15_SPEC>;
#[doc = "Channel Source Address Register (chid = 15)"]
pub mod csa15;
#[doc = "CDA15 register accessor: an alias for `Reg<CDA15_SPEC>`"]
pub type CDA15 = crate::Reg<cda15::CDA15_SPEC>;
#[doc = "Channel Destination Address Register (chid = 15)"]
pub mod cda15;
#[doc = "CNDA15 register accessor: an alias for `Reg<CNDA15_SPEC>`"]
pub type CNDA15 = crate::Reg<cnda15::CNDA15_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 15)"]
pub mod cnda15;
#[doc = "CNDC15 register accessor: an alias for `Reg<CNDC15_SPEC>`"]
pub type CNDC15 = crate::Reg<cndc15::CNDC15_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 15)"]
pub mod cndc15;
#[doc = "CUBC15 register accessor: an alias for `Reg<CUBC15_SPEC>`"]
pub type CUBC15 = crate::Reg<cubc15::CUBC15_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 15)"]
pub mod cubc15;
#[doc = "CBC15 register accessor: an alias for `Reg<CBC15_SPEC>`"]
pub type CBC15 = crate::Reg<cbc15::CBC15_SPEC>;
#[doc = "Channel Block Control Register (chid = 15)"]
pub mod cbc15;
#[doc = "CC15 register accessor: an alias for `Reg<CC15_SPEC>`"]
pub type CC15 = crate::Reg<cc15::CC15_SPEC>;
#[doc = "Channel Configuration Register (chid = 15)"]
pub mod cc15;
#[doc = "CDS_MSP15 register accessor: an alias for `Reg<CDS_MSP15_SPEC>`"]
pub type CDS_MSP15 = crate::Reg<cds_msp15::CDS_MSP15_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 15)"]
pub mod cds_msp15;
#[doc = "CSUS15 register accessor: an alias for `Reg<CSUS15_SPEC>`"]
pub type CSUS15 = crate::Reg<csus15::CSUS15_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 15)"]
pub mod csus15;
#[doc = "CDUS15 register accessor: an alias for `Reg<CDUS15_SPEC>`"]
pub type CDUS15 = crate::Reg<cdus15::CDUS15_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 15)"]
pub mod cdus15;
#[doc = "CIE16 register accessor: an alias for `Reg<CIE16_SPEC>`"]
pub type CIE16 = crate::Reg<cie16::CIE16_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 16)"]
pub mod cie16;
#[doc = "CID16 register accessor: an alias for `Reg<CID16_SPEC>`"]
pub type CID16 = crate::Reg<cid16::CID16_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 16)"]
pub mod cid16;
#[doc = "CIM16 register accessor: an alias for `Reg<CIM16_SPEC>`"]
pub type CIM16 = crate::Reg<cim16::CIM16_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 16)"]
pub mod cim16;
#[doc = "CIS16 register accessor: an alias for `Reg<CIS16_SPEC>`"]
pub type CIS16 = crate::Reg<cis16::CIS16_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 16)"]
pub mod cis16;
#[doc = "CSA16 register accessor: an alias for `Reg<CSA16_SPEC>`"]
pub type CSA16 = crate::Reg<csa16::CSA16_SPEC>;
#[doc = "Channel Source Address Register (chid = 16)"]
pub mod csa16;
#[doc = "CDA16 register accessor: an alias for `Reg<CDA16_SPEC>`"]
pub type CDA16 = crate::Reg<cda16::CDA16_SPEC>;
#[doc = "Channel Destination Address Register (chid = 16)"]
pub mod cda16;
#[doc = "CNDA16 register accessor: an alias for `Reg<CNDA16_SPEC>`"]
pub type CNDA16 = crate::Reg<cnda16::CNDA16_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 16)"]
pub mod cnda16;
#[doc = "CNDC16 register accessor: an alias for `Reg<CNDC16_SPEC>`"]
pub type CNDC16 = crate::Reg<cndc16::CNDC16_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 16)"]
pub mod cndc16;
#[doc = "CUBC16 register accessor: an alias for `Reg<CUBC16_SPEC>`"]
pub type CUBC16 = crate::Reg<cubc16::CUBC16_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 16)"]
pub mod cubc16;
#[doc = "CBC16 register accessor: an alias for `Reg<CBC16_SPEC>`"]
pub type CBC16 = crate::Reg<cbc16::CBC16_SPEC>;
#[doc = "Channel Block Control Register (chid = 16)"]
pub mod cbc16;
#[doc = "CC16 register accessor: an alias for `Reg<CC16_SPEC>`"]
pub type CC16 = crate::Reg<cc16::CC16_SPEC>;
#[doc = "Channel Configuration Register (chid = 16)"]
pub mod cc16;
#[doc = "CDS_MSP16 register accessor: an alias for `Reg<CDS_MSP16_SPEC>`"]
pub type CDS_MSP16 = crate::Reg<cds_msp16::CDS_MSP16_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 16)"]
pub mod cds_msp16;
#[doc = "CSUS16 register accessor: an alias for `Reg<CSUS16_SPEC>`"]
pub type CSUS16 = crate::Reg<csus16::CSUS16_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 16)"]
pub mod csus16;
#[doc = "CDUS16 register accessor: an alias for `Reg<CDUS16_SPEC>`"]
pub type CDUS16 = crate::Reg<cdus16::CDUS16_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 16)"]
pub mod cdus16;
#[doc = "CIE17 register accessor: an alias for `Reg<CIE17_SPEC>`"]
pub type CIE17 = crate::Reg<cie17::CIE17_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 17)"]
pub mod cie17;
#[doc = "CID17 register accessor: an alias for `Reg<CID17_SPEC>`"]
pub type CID17 = crate::Reg<cid17::CID17_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 17)"]
pub mod cid17;
#[doc = "CIM17 register accessor: an alias for `Reg<CIM17_SPEC>`"]
pub type CIM17 = crate::Reg<cim17::CIM17_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 17)"]
pub mod cim17;
#[doc = "CIS17 register accessor: an alias for `Reg<CIS17_SPEC>`"]
pub type CIS17 = crate::Reg<cis17::CIS17_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 17)"]
pub mod cis17;
#[doc = "CSA17 register accessor: an alias for `Reg<CSA17_SPEC>`"]
pub type CSA17 = crate::Reg<csa17::CSA17_SPEC>;
#[doc = "Channel Source Address Register (chid = 17)"]
pub mod csa17;
#[doc = "CDA17 register accessor: an alias for `Reg<CDA17_SPEC>`"]
pub type CDA17 = crate::Reg<cda17::CDA17_SPEC>;
#[doc = "Channel Destination Address Register (chid = 17)"]
pub mod cda17;
#[doc = "CNDA17 register accessor: an alias for `Reg<CNDA17_SPEC>`"]
pub type CNDA17 = crate::Reg<cnda17::CNDA17_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 17)"]
pub mod cnda17;
#[doc = "CNDC17 register accessor: an alias for `Reg<CNDC17_SPEC>`"]
pub type CNDC17 = crate::Reg<cndc17::CNDC17_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 17)"]
pub mod cndc17;
#[doc = "CUBC17 register accessor: an alias for `Reg<CUBC17_SPEC>`"]
pub type CUBC17 = crate::Reg<cubc17::CUBC17_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 17)"]
pub mod cubc17;
#[doc = "CBC17 register accessor: an alias for `Reg<CBC17_SPEC>`"]
pub type CBC17 = crate::Reg<cbc17::CBC17_SPEC>;
#[doc = "Channel Block Control Register (chid = 17)"]
pub mod cbc17;
#[doc = "CC17 register accessor: an alias for `Reg<CC17_SPEC>`"]
pub type CC17 = crate::Reg<cc17::CC17_SPEC>;
#[doc = "Channel Configuration Register (chid = 17)"]
pub mod cc17;
#[doc = "CDS_MSP17 register accessor: an alias for `Reg<CDS_MSP17_SPEC>`"]
pub type CDS_MSP17 = crate::Reg<cds_msp17::CDS_MSP17_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 17)"]
pub mod cds_msp17;
#[doc = "CSUS17 register accessor: an alias for `Reg<CSUS17_SPEC>`"]
pub type CSUS17 = crate::Reg<csus17::CSUS17_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 17)"]
pub mod csus17;
#[doc = "CDUS17 register accessor: an alias for `Reg<CDUS17_SPEC>`"]
pub type CDUS17 = crate::Reg<cdus17::CDUS17_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 17)"]
pub mod cdus17;
#[doc = "CIE18 register accessor: an alias for `Reg<CIE18_SPEC>`"]
pub type CIE18 = crate::Reg<cie18::CIE18_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 18)"]
pub mod cie18;
#[doc = "CID18 register accessor: an alias for `Reg<CID18_SPEC>`"]
pub type CID18 = crate::Reg<cid18::CID18_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 18)"]
pub mod cid18;
#[doc = "CIM18 register accessor: an alias for `Reg<CIM18_SPEC>`"]
pub type CIM18 = crate::Reg<cim18::CIM18_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 18)"]
pub mod cim18;
#[doc = "CIS18 register accessor: an alias for `Reg<CIS18_SPEC>`"]
pub type CIS18 = crate::Reg<cis18::CIS18_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 18)"]
pub mod cis18;
#[doc = "CSA18 register accessor: an alias for `Reg<CSA18_SPEC>`"]
pub type CSA18 = crate::Reg<csa18::CSA18_SPEC>;
#[doc = "Channel Source Address Register (chid = 18)"]
pub mod csa18;
#[doc = "CDA18 register accessor: an alias for `Reg<CDA18_SPEC>`"]
pub type CDA18 = crate::Reg<cda18::CDA18_SPEC>;
#[doc = "Channel Destination Address Register (chid = 18)"]
pub mod cda18;
#[doc = "CNDA18 register accessor: an alias for `Reg<CNDA18_SPEC>`"]
pub type CNDA18 = crate::Reg<cnda18::CNDA18_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 18)"]
pub mod cnda18;
#[doc = "CNDC18 register accessor: an alias for `Reg<CNDC18_SPEC>`"]
pub type CNDC18 = crate::Reg<cndc18::CNDC18_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 18)"]
pub mod cndc18;
#[doc = "CUBC18 register accessor: an alias for `Reg<CUBC18_SPEC>`"]
pub type CUBC18 = crate::Reg<cubc18::CUBC18_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 18)"]
pub mod cubc18;
#[doc = "CBC18 register accessor: an alias for `Reg<CBC18_SPEC>`"]
pub type CBC18 = crate::Reg<cbc18::CBC18_SPEC>;
#[doc = "Channel Block Control Register (chid = 18)"]
pub mod cbc18;
#[doc = "CC18 register accessor: an alias for `Reg<CC18_SPEC>`"]
pub type CC18 = crate::Reg<cc18::CC18_SPEC>;
#[doc = "Channel Configuration Register (chid = 18)"]
pub mod cc18;
#[doc = "CDS_MSP18 register accessor: an alias for `Reg<CDS_MSP18_SPEC>`"]
pub type CDS_MSP18 = crate::Reg<cds_msp18::CDS_MSP18_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 18)"]
pub mod cds_msp18;
#[doc = "CSUS18 register accessor: an alias for `Reg<CSUS18_SPEC>`"]
pub type CSUS18 = crate::Reg<csus18::CSUS18_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 18)"]
pub mod csus18;
#[doc = "CDUS18 register accessor: an alias for `Reg<CDUS18_SPEC>`"]
pub type CDUS18 = crate::Reg<cdus18::CDUS18_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 18)"]
pub mod cdus18;
#[doc = "CIE19 register accessor: an alias for `Reg<CIE19_SPEC>`"]
pub type CIE19 = crate::Reg<cie19::CIE19_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 19)"]
pub mod cie19;
#[doc = "CID19 register accessor: an alias for `Reg<CID19_SPEC>`"]
pub type CID19 = crate::Reg<cid19::CID19_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 19)"]
pub mod cid19;
#[doc = "CIM19 register accessor: an alias for `Reg<CIM19_SPEC>`"]
pub type CIM19 = crate::Reg<cim19::CIM19_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 19)"]
pub mod cim19;
#[doc = "CIS19 register accessor: an alias for `Reg<CIS19_SPEC>`"]
pub type CIS19 = crate::Reg<cis19::CIS19_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 19)"]
pub mod cis19;
#[doc = "CSA19 register accessor: an alias for `Reg<CSA19_SPEC>`"]
pub type CSA19 = crate::Reg<csa19::CSA19_SPEC>;
#[doc = "Channel Source Address Register (chid = 19)"]
pub mod csa19;
#[doc = "CDA19 register accessor: an alias for `Reg<CDA19_SPEC>`"]
pub type CDA19 = crate::Reg<cda19::CDA19_SPEC>;
#[doc = "Channel Destination Address Register (chid = 19)"]
pub mod cda19;
#[doc = "CNDA19 register accessor: an alias for `Reg<CNDA19_SPEC>`"]
pub type CNDA19 = crate::Reg<cnda19::CNDA19_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 19)"]
pub mod cnda19;
#[doc = "CNDC19 register accessor: an alias for `Reg<CNDC19_SPEC>`"]
pub type CNDC19 = crate::Reg<cndc19::CNDC19_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 19)"]
pub mod cndc19;
#[doc = "CUBC19 register accessor: an alias for `Reg<CUBC19_SPEC>`"]
pub type CUBC19 = crate::Reg<cubc19::CUBC19_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 19)"]
pub mod cubc19;
#[doc = "CBC19 register accessor: an alias for `Reg<CBC19_SPEC>`"]
pub type CBC19 = crate::Reg<cbc19::CBC19_SPEC>;
#[doc = "Channel Block Control Register (chid = 19)"]
pub mod cbc19;
#[doc = "CC19 register accessor: an alias for `Reg<CC19_SPEC>`"]
pub type CC19 = crate::Reg<cc19::CC19_SPEC>;
#[doc = "Channel Configuration Register (chid = 19)"]
pub mod cc19;
#[doc = "CDS_MSP19 register accessor: an alias for `Reg<CDS_MSP19_SPEC>`"]
pub type CDS_MSP19 = crate::Reg<cds_msp19::CDS_MSP19_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 19)"]
pub mod cds_msp19;
#[doc = "CSUS19 register accessor: an alias for `Reg<CSUS19_SPEC>`"]
pub type CSUS19 = crate::Reg<csus19::CSUS19_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 19)"]
pub mod csus19;
#[doc = "CDUS19 register accessor: an alias for `Reg<CDUS19_SPEC>`"]
pub type CDUS19 = crate::Reg<cdus19::CDUS19_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 19)"]
pub mod cdus19;
#[doc = "CIE20 register accessor: an alias for `Reg<CIE20_SPEC>`"]
pub type CIE20 = crate::Reg<cie20::CIE20_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 20)"]
pub mod cie20;
#[doc = "CID20 register accessor: an alias for `Reg<CID20_SPEC>`"]
pub type CID20 = crate::Reg<cid20::CID20_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 20)"]
pub mod cid20;
#[doc = "CIM20 register accessor: an alias for `Reg<CIM20_SPEC>`"]
pub type CIM20 = crate::Reg<cim20::CIM20_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 20)"]
pub mod cim20;
#[doc = "CIS20 register accessor: an alias for `Reg<CIS20_SPEC>`"]
pub type CIS20 = crate::Reg<cis20::CIS20_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 20)"]
pub mod cis20;
#[doc = "CSA20 register accessor: an alias for `Reg<CSA20_SPEC>`"]
pub type CSA20 = crate::Reg<csa20::CSA20_SPEC>;
#[doc = "Channel Source Address Register (chid = 20)"]
pub mod csa20;
#[doc = "CDA20 register accessor: an alias for `Reg<CDA20_SPEC>`"]
pub type CDA20 = crate::Reg<cda20::CDA20_SPEC>;
#[doc = "Channel Destination Address Register (chid = 20)"]
pub mod cda20;
#[doc = "CNDA20 register accessor: an alias for `Reg<CNDA20_SPEC>`"]
pub type CNDA20 = crate::Reg<cnda20::CNDA20_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 20)"]
pub mod cnda20;
#[doc = "CNDC20 register accessor: an alias for `Reg<CNDC20_SPEC>`"]
pub type CNDC20 = crate::Reg<cndc20::CNDC20_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 20)"]
pub mod cndc20;
#[doc = "CUBC20 register accessor: an alias for `Reg<CUBC20_SPEC>`"]
pub type CUBC20 = crate::Reg<cubc20::CUBC20_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 20)"]
pub mod cubc20;
#[doc = "CBC20 register accessor: an alias for `Reg<CBC20_SPEC>`"]
pub type CBC20 = crate::Reg<cbc20::CBC20_SPEC>;
#[doc = "Channel Block Control Register (chid = 20)"]
pub mod cbc20;
#[doc = "CC20 register accessor: an alias for `Reg<CC20_SPEC>`"]
pub type CC20 = crate::Reg<cc20::CC20_SPEC>;
#[doc = "Channel Configuration Register (chid = 20)"]
pub mod cc20;
#[doc = "CDS_MSP20 register accessor: an alias for `Reg<CDS_MSP20_SPEC>`"]
pub type CDS_MSP20 = crate::Reg<cds_msp20::CDS_MSP20_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 20)"]
pub mod cds_msp20;
#[doc = "CSUS20 register accessor: an alias for `Reg<CSUS20_SPEC>`"]
pub type CSUS20 = crate::Reg<csus20::CSUS20_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 20)"]
pub mod csus20;
#[doc = "CDUS20 register accessor: an alias for `Reg<CDUS20_SPEC>`"]
pub type CDUS20 = crate::Reg<cdus20::CDUS20_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 20)"]
pub mod cdus20;
#[doc = "CIE21 register accessor: an alias for `Reg<CIE21_SPEC>`"]
pub type CIE21 = crate::Reg<cie21::CIE21_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 21)"]
pub mod cie21;
#[doc = "CID21 register accessor: an alias for `Reg<CID21_SPEC>`"]
pub type CID21 = crate::Reg<cid21::CID21_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 21)"]
pub mod cid21;
#[doc = "CIM21 register accessor: an alias for `Reg<CIM21_SPEC>`"]
pub type CIM21 = crate::Reg<cim21::CIM21_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 21)"]
pub mod cim21;
#[doc = "CIS21 register accessor: an alias for `Reg<CIS21_SPEC>`"]
pub type CIS21 = crate::Reg<cis21::CIS21_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 21)"]
pub mod cis21;
#[doc = "CSA21 register accessor: an alias for `Reg<CSA21_SPEC>`"]
pub type CSA21 = crate::Reg<csa21::CSA21_SPEC>;
#[doc = "Channel Source Address Register (chid = 21)"]
pub mod csa21;
#[doc = "CDA21 register accessor: an alias for `Reg<CDA21_SPEC>`"]
pub type CDA21 = crate::Reg<cda21::CDA21_SPEC>;
#[doc = "Channel Destination Address Register (chid = 21)"]
pub mod cda21;
#[doc = "CNDA21 register accessor: an alias for `Reg<CNDA21_SPEC>`"]
pub type CNDA21 = crate::Reg<cnda21::CNDA21_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 21)"]
pub mod cnda21;
#[doc = "CNDC21 register accessor: an alias for `Reg<CNDC21_SPEC>`"]
pub type CNDC21 = crate::Reg<cndc21::CNDC21_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 21)"]
pub mod cndc21;
#[doc = "CUBC21 register accessor: an alias for `Reg<CUBC21_SPEC>`"]
pub type CUBC21 = crate::Reg<cubc21::CUBC21_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 21)"]
pub mod cubc21;
#[doc = "CBC21 register accessor: an alias for `Reg<CBC21_SPEC>`"]
pub type CBC21 = crate::Reg<cbc21::CBC21_SPEC>;
#[doc = "Channel Block Control Register (chid = 21)"]
pub mod cbc21;
#[doc = "CC21 register accessor: an alias for `Reg<CC21_SPEC>`"]
pub type CC21 = crate::Reg<cc21::CC21_SPEC>;
#[doc = "Channel Configuration Register (chid = 21)"]
pub mod cc21;
#[doc = "CDS_MSP21 register accessor: an alias for `Reg<CDS_MSP21_SPEC>`"]
pub type CDS_MSP21 = crate::Reg<cds_msp21::CDS_MSP21_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 21)"]
pub mod cds_msp21;
#[doc = "CSUS21 register accessor: an alias for `Reg<CSUS21_SPEC>`"]
pub type CSUS21 = crate::Reg<csus21::CSUS21_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 21)"]
pub mod csus21;
#[doc = "CDUS21 register accessor: an alias for `Reg<CDUS21_SPEC>`"]
pub type CDUS21 = crate::Reg<cdus21::CDUS21_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 21)"]
pub mod cdus21;
#[doc = "CIE22 register accessor: an alias for `Reg<CIE22_SPEC>`"]
pub type CIE22 = crate::Reg<cie22::CIE22_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 22)"]
pub mod cie22;
#[doc = "CID22 register accessor: an alias for `Reg<CID22_SPEC>`"]
pub type CID22 = crate::Reg<cid22::CID22_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 22)"]
pub mod cid22;
#[doc = "CIM22 register accessor: an alias for `Reg<CIM22_SPEC>`"]
pub type CIM22 = crate::Reg<cim22::CIM22_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 22)"]
pub mod cim22;
#[doc = "CIS22 register accessor: an alias for `Reg<CIS22_SPEC>`"]
pub type CIS22 = crate::Reg<cis22::CIS22_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 22)"]
pub mod cis22;
#[doc = "CSA22 register accessor: an alias for `Reg<CSA22_SPEC>`"]
pub type CSA22 = crate::Reg<csa22::CSA22_SPEC>;
#[doc = "Channel Source Address Register (chid = 22)"]
pub mod csa22;
#[doc = "CDA22 register accessor: an alias for `Reg<CDA22_SPEC>`"]
pub type CDA22 = crate::Reg<cda22::CDA22_SPEC>;
#[doc = "Channel Destination Address Register (chid = 22)"]
pub mod cda22;
#[doc = "CNDA22 register accessor: an alias for `Reg<CNDA22_SPEC>`"]
pub type CNDA22 = crate::Reg<cnda22::CNDA22_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 22)"]
pub mod cnda22;
#[doc = "CNDC22 register accessor: an alias for `Reg<CNDC22_SPEC>`"]
pub type CNDC22 = crate::Reg<cndc22::CNDC22_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 22)"]
pub mod cndc22;
#[doc = "CUBC22 register accessor: an alias for `Reg<CUBC22_SPEC>`"]
pub type CUBC22 = crate::Reg<cubc22::CUBC22_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 22)"]
pub mod cubc22;
#[doc = "CBC22 register accessor: an alias for `Reg<CBC22_SPEC>`"]
pub type CBC22 = crate::Reg<cbc22::CBC22_SPEC>;
#[doc = "Channel Block Control Register (chid = 22)"]
pub mod cbc22;
#[doc = "CC22 register accessor: an alias for `Reg<CC22_SPEC>`"]
pub type CC22 = crate::Reg<cc22::CC22_SPEC>;
#[doc = "Channel Configuration Register (chid = 22)"]
pub mod cc22;
#[doc = "CDS_MSP22 register accessor: an alias for `Reg<CDS_MSP22_SPEC>`"]
pub type CDS_MSP22 = crate::Reg<cds_msp22::CDS_MSP22_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 22)"]
pub mod cds_msp22;
#[doc = "CSUS22 register accessor: an alias for `Reg<CSUS22_SPEC>`"]
pub type CSUS22 = crate::Reg<csus22::CSUS22_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 22)"]
pub mod csus22;
#[doc = "CDUS22 register accessor: an alias for `Reg<CDUS22_SPEC>`"]
pub type CDUS22 = crate::Reg<cdus22::CDUS22_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 22)"]
pub mod cdus22;
#[doc = "CIE23 register accessor: an alias for `Reg<CIE23_SPEC>`"]
pub type CIE23 = crate::Reg<cie23::CIE23_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 23)"]
pub mod cie23;
#[doc = "CID23 register accessor: an alias for `Reg<CID23_SPEC>`"]
pub type CID23 = crate::Reg<cid23::CID23_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 23)"]
pub mod cid23;
#[doc = "CIM23 register accessor: an alias for `Reg<CIM23_SPEC>`"]
pub type CIM23 = crate::Reg<cim23::CIM23_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 23)"]
pub mod cim23;
#[doc = "CIS23 register accessor: an alias for `Reg<CIS23_SPEC>`"]
pub type CIS23 = crate::Reg<cis23::CIS23_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 23)"]
pub mod cis23;
#[doc = "CSA23 register accessor: an alias for `Reg<CSA23_SPEC>`"]
pub type CSA23 = crate::Reg<csa23::CSA23_SPEC>;
#[doc = "Channel Source Address Register (chid = 23)"]
pub mod csa23;
#[doc = "CDA23 register accessor: an alias for `Reg<CDA23_SPEC>`"]
pub type CDA23 = crate::Reg<cda23::CDA23_SPEC>;
#[doc = "Channel Destination Address Register (chid = 23)"]
pub mod cda23;
#[doc = "CNDA23 register accessor: an alias for `Reg<CNDA23_SPEC>`"]
pub type CNDA23 = crate::Reg<cnda23::CNDA23_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 23)"]
pub mod cnda23;
#[doc = "CNDC23 register accessor: an alias for `Reg<CNDC23_SPEC>`"]
pub type CNDC23 = crate::Reg<cndc23::CNDC23_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 23)"]
pub mod cndc23;
#[doc = "CUBC23 register accessor: an alias for `Reg<CUBC23_SPEC>`"]
pub type CUBC23 = crate::Reg<cubc23::CUBC23_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 23)"]
pub mod cubc23;
#[doc = "CBC23 register accessor: an alias for `Reg<CBC23_SPEC>`"]
pub type CBC23 = crate::Reg<cbc23::CBC23_SPEC>;
#[doc = "Channel Block Control Register (chid = 23)"]
pub mod cbc23;
#[doc = "CC23 register accessor: an alias for `Reg<CC23_SPEC>`"]
pub type CC23 = crate::Reg<cc23::CC23_SPEC>;
#[doc = "Channel Configuration Register (chid = 23)"]
pub mod cc23;
#[doc = "CDS_MSP23 register accessor: an alias for `Reg<CDS_MSP23_SPEC>`"]
pub type CDS_MSP23 = crate::Reg<cds_msp23::CDS_MSP23_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 23)"]
pub mod cds_msp23;
#[doc = "CSUS23 register accessor: an alias for `Reg<CSUS23_SPEC>`"]
pub type CSUS23 = crate::Reg<csus23::CSUS23_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 23)"]
pub mod csus23;
#[doc = "CDUS23 register accessor: an alias for `Reg<CDUS23_SPEC>`"]
pub type CDUS23 = crate::Reg<cdus23::CDUS23_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 23)"]
pub mod cdus23;
