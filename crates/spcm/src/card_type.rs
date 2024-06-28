use num_enum::TryFromPrimitive;

#[derive(Debug, TryFromPrimitive)]
#[repr(i32)]
pub enum CardType {
    // Copy pasted from bindgen-generated source
    // Case converted by `perl -pe 's/([A-Z])([A-Z]+)/$1 . lc($2)/ge'`
    // TODO: can't we automate it?  Related: https://github.com/rust-lang/rust-bindgen/issues/2174
    Pcideviceid = 0,
    Eval = 16,
    Rsdlga = 20,
    Gmg = 24,
    Van8 = 32,
    Vac = 40,
    Pciautoinstall = 255,
    Dap116 = 256,
    Pad82 = 512,
    Pad82a = 528,
    Pad82b = 544,
    Pci212 = 768,
    Pad1232a = 1024,
    Pad1232b = 1040,
    Pad1232c = 1056,
    Pad1616a = 1280,
    Pad1616b = 1296,
    Pad1616c = 1312,
    Pad1616d = 1328,
    Pad52 = 1536,
    Pad242 = 1792,
    Pck400 = 2048,
    Pad164_2M = 2304,
    Pad164_5M = 2320,
    Pci208 = 4096,
    Cpci208 = 4097,
    Pci412 = 4352,
    Pcidio32 = 4608,
    Pci248 = 4864,
    Padco = 5120,
    Trs582 = 5376,
    Pci258 = 5632,
    // Omitting `mask` and `series`, as it seems to be overlapping
    // Seriesmask = 16711680,
    // Versionmask = 65535,
    // Familymask = 65280,
    // Typemask = 255,
    // Speedmask = 240,
    // Chmask = 15,
    // Miseries = 0,
    // Mcseries = 65536,
    // Mxseries = 131072,
    // M2Iseries = 196608,
    // M2Iexpseries = 262144,
    // M3Iseries = 327680,
    // M3Iexpseries = 393216,
    // M4Iexpseries = 458752,
    // M4Xexpseries = 524288,
    // M2Pexpseries = 589824,
    // M5Iexpseries = 655360,
    Mi2020 = 8224,
    Mi2021 = 8225,
    Mi2025 = 8229,
    Mi2030 = 8240,
    Mi2031 = 8241,
    M2I2020 = 204832,
    M2I2021 = 204833,
    M2I2025 = 204837,
    M2I2030 = 204848,
    M2I2031 = 204849,
    M2I2020Exp = 270368,
    M2I2021Exp = 270369,
    M2I2025Exp = 270373,
    M2I2030Exp = 270384,
    M2I2031Exp = 270385,
    Mc2020 = 73760,
    Mc2021 = 73761,
    Mc2025 = 73765,
    Mc2030 = 73776,
    Mc2031 = 73777,
    Mx2020 = 139296,
    Mx2025 = 139301,
    Mx2030 = 139312,
    M3I2120 = 336160,
    M3I2122 = 336162,
    M3I2130 = 336176,
    M3I2132 = 336178,
    M3I2120Exp = 401696,
    M3I2122Exp = 401698,
    M3I2130Exp = 401712,
    M3I2132Exp = 401714,
    M4I22XxX8 = 467456,
    M4I2210X8 = 467472,
    M4I2211X8 = 467473,
    M4I2212X8 = 467474,
    M4I2220X8 = 467488,
    M4I2221X8 = 467489,
    M4I2223X8 = 467491,
    M4I2230X8 = 467504,
    M4I2233X8 = 467507,
    M4I2234X8 = 467508,
    M4I2280X8 = 467584,
    M4I2281X8 = 467585,
    M4I2283X8 = 467587,
    M4I2290X8 = 467600,
    M4I2293X8 = 467603,
    M4I2294X8 = 467604,
    M4X22XxX4 = 532992,
    M4X2210X4 = 533008,
    M4X2211X4 = 533009,
    M4X2212X4 = 533010,
    M4X2220X4 = 533024,
    M4X2221X4 = 533025,
    M4X2223X4 = 533027,
    M4X2230X4 = 533040,
    M4X2233X4 = 533043,
    M4X2234X4 = 533044,
    M4I23XxX8 = 467712,
    M4I2320X8 = 467744,
    M4I2321X8 = 467745,
    M4I2323X8 = 467747,
    M4I2330X8 = 467760,
    M4I2333X8 = 467763,
    M4I2334X8 = 467764,
    Mi3010 = 12304,
    Mi3011 = 12305,
    Mi3012 = 12306,
    Mi3013 = 12307,
    Mi3014 = 12308,
    Mi3015 = 12309,
    Mi3016 = 12310,
    Mi3020 = 12320,
    Mi3021 = 12321,
    Mi3022 = 12322,
    Mi3023 = 12323,
    Mi3024 = 12324,
    Mi3025 = 12325,
    Mi3026 = 12326,
    Mi3027 = 12327,
    Mi3031 = 12337,
    Mi3033 = 12339,
    M2I3010 = 208912,
    M2I3011 = 208913,
    M2I3012 = 208914,
    M2I3013 = 208915,
    M2I3014 = 208916,
    M2I3015 = 208917,
    M2I3016 = 208918,
    M2I3020 = 208928,
    M2I3021 = 208929,
    M2I3022 = 208930,
    M2I3023 = 208931,
    M2I3024 = 208932,
    M2I3025 = 208933,
    M2I3026 = 208934,
    M2I3027 = 208935,
    M2I3031 = 208945,
    M2I3033 = 208947,
    M2I3010Exp = 274448,
    M2I3011Exp = 274449,
    M2I3012Exp = 274450,
    M2I3013Exp = 274451,
    M2I3014Exp = 274452,
    M2I3015Exp = 274453,
    M2I3016Exp = 274454,
    M2I3020Exp = 274464,
    M2I3021Exp = 274465,
    M2I3022Exp = 274466,
    M2I3023Exp = 274467,
    M2I3024Exp = 274468,
    M2I3025Exp = 274469,
    M2I3026Exp = 274470,
    M2I3027Exp = 274471,
    M2I3031Exp = 274481,
    M2I3033Exp = 274483,
    Mc3010 = 77840,
    Mc3011 = 77841,
    Mc3012 = 77842,
    Mc3013 = 77843,
    Mc3014 = 77844,
    Mc3015 = 77845,
    Mc3016 = 77846,
    Mc3020 = 77856,
    Mc3021 = 77857,
    Mc3022 = 77858,
    Mc3023 = 77859,
    Mc3024 = 77860,
    Mc3025 = 77861,
    Mc3026 = 77862,
    Mc3027 = 77863,
    Mc3031 = 77873,
    Mc3033 = 77875,
    Mx3010 = 143376,
    Mx3011 = 143377,
    Mx3012 = 143378,
    Mx3020 = 143392,
    Mx3021 = 143393,
    Mx3022 = 143394,
    Mx3031 = 143409,
    Mi3110 = 12560,
    Mi3111 = 12561,
    Mi3112 = 12562,
    Mi3120 = 12576,
    Mi3121 = 12577,
    Mi3122 = 12578,
    Mi3130 = 12592,
    Mi3131 = 12593,
    Mi3132 = 12594,
    Mi3140 = 12608,
    M2I3110 = 209168,
    M2I3111 = 209169,
    M2I3112 = 209170,
    M2I3120 = 209184,
    M2I3121 = 209185,
    M2I3122 = 209186,
    M2I3130 = 209200,
    M2I3131 = 209201,
    M2I3132 = 209202,
    M2I3110Exp = 274704,
    M2I3111Exp = 274705,
    M2I3112Exp = 274706,
    M2I3120Exp = 274720,
    M2I3121Exp = 274721,
    M2I3122Exp = 274722,
    M2I3130Exp = 274736,
    M2I3131Exp = 274737,
    M2I3132Exp = 274738,
    Mc3110 = 78096,
    Mc3111 = 78097,
    Mc3112 = 78098,
    Mc3120 = 78112,
    Mc3121 = 78113,
    Mc3122 = 78114,
    Mc3130 = 78128,
    Mc3131 = 78129,
    Mc3132 = 78130,
    Mx3110 = 143632,
    Mx3111 = 143633,
    Mx3120 = 143648,
    Mx3121 = 143649,
    Mx3130 = 143664,
    Mx3131 = 143665,
    M3I3220 = 340512,
    M3I3221 = 340513,
    M3I3240 = 340544,
    M3I3242 = 340546,
    M3I3220Exp = 406048,
    M3I3221Exp = 406049,
    M3I3240Exp = 406080,
    M3I3242Exp = 406082,
    M5I33XxX16 = 668416,
    M5I3321X16 = 668449,
    M5I3330X16 = 668464,
    M5I3337X16 = 668471,
    M5I3350X16 = 668496,
    M5I3357X16 = 668503,
    M5I3360X16 = 668512,
    M5I3367X16 = 668519,
    Mi4020 = 16416,
    Mi4021 = 16417,
    Mi4022 = 16418,
    Mi4030 = 16432,
    Mi4031 = 16433,
    Mi4032 = 16434,
    M2I4020 = 213024,
    M2I4021 = 213025,
    M2I4022 = 213026,
    M2I4028 = 213032,
    M2I4030 = 213040,
    M2I4031 = 213041,
    M2I4032 = 213042,
    M2I4038 = 213048,
    M2I4020Exp = 278560,
    M2I4021Exp = 278561,
    M2I4022Exp = 278562,
    M2I4028Exp = 278568,
    M2I4030Exp = 278576,
    M2I4031Exp = 278577,
    M2I4032Exp = 278578,
    M2I4038Exp = 278584,
    Mc4020 = 81952,
    Mc4021 = 81953,
    Mc4022 = 81954,
    Mc4030 = 81968,
    Mc4031 = 81969,
    Mc4032 = 81970,
    Mx4020 = 147488,
    Mx4021 = 147489,
    Mx4030 = 147504,
    Mx4031 = 147505,
    M3I4110 = 344336,
    M3I4111 = 344337,
    M3I4120 = 344352,
    M3I4121 = 344353,
    M3I4140 = 344384,
    M3I4142 = 344386,
    M3I4110Exp = 409872,
    M3I4111Exp = 409873,
    M3I4120Exp = 409888,
    M3I4121Exp = 409889,
    M3I4140Exp = 409920,
    M3I4142Exp = 409922,
    M4I44XxX8 = 476160,
    M4I4410X8 = 476176,
    M4I4411X8 = 476177,
    M4I4420X8 = 476192,
    M4I4421X8 = 476193,
    M4I4450X8 = 476240,
    M4I4451X8 = 476241,
    M4I4470X8 = 476272,
    M4I4471X8 = 476273,
    M4I4480X8 = 476288,
    M4I4481X8 = 476289,
    M4X44XxX4 = 541696,
    M4X4410X4 = 541712,
    M4X4411X4 = 541713,
    M4X4420X4 = 541728,
    M4X4421X4 = 541729,
    M4X4450X4 = 541776,
    M4X4451X4 = 541777,
    M4X4470X4 = 541808,
    M4X4471X4 = 541809,
    M4X4480X4 = 541824,
    M4X4481X4 = 541825,
    Mi4520 = 17696,
    Mi4521 = 17697,
    Mi4530 = 17712,
    Mi4531 = 17713,
    Mi4540 = 17728,
    Mi4541 = 17729,
    M2I4520 = 214304,
    M2I4521 = 214305,
    M2I4530 = 214320,
    M2I4531 = 214321,
    M2I4540 = 214336,
    M2I4541 = 214337,
    Mc4520 = 83232,
    Mc4521 = 83233,
    Mc4530 = 83248,
    Mc4531 = 83249,
    Mc4540 = 83264,
    Mc4541 = 83265,
    Mx4520 = 148768,
    Mx4530 = 148784,
    Mx4540 = 148800,
    Mi4620 = 17952,
    Mi4621 = 17953,
    Mi4622 = 17954,
    Mi4630 = 17968,
    Mi4631 = 17969,
    Mi4632 = 17970,
    Mi4640 = 17984,
    Mi4641 = 17985,
    Mi4642 = 17986,
    Mi4650 = 18000,
    Mi4651 = 18001,
    Mi4652 = 18002,
    M2I4620 = 214560,
    M2I4621 = 214561,
    M2I4622 = 214562,
    M2I4630 = 214576,
    M2I4631 = 214577,
    M2I4632 = 214578,
    M2I4640 = 214592,
    M2I4641 = 214593,
    M2I4642 = 214594,
    M2I4650 = 214608,
    M2I4651 = 214609,
    M2I4652 = 214610,
    M2I4620Exp = 280096,
    M2I4621Exp = 280097,
    M2I4622Exp = 280098,
    M2I4630Exp = 280112,
    M2I4631Exp = 280113,
    M2I4632Exp = 280114,
    M2I4640Exp = 280128,
    M2I4641Exp = 280129,
    M2I4642Exp = 280130,
    M2I4650Exp = 280144,
    M2I4651Exp = 280145,
    M2I4652Exp = 280146,
    Mc4620 = 83488,
    Mc4621 = 83489,
    Mc4622 = 83490,
    Mc4630 = 83504,
    Mc4631 = 83505,
    Mc4632 = 83506,
    Mc4640 = 83520,
    Mc4641 = 83521,
    Mc4642 = 83522,
    Mc4650 = 83536,
    Mc4651 = 83537,
    Mc4652 = 83538,
    Mx4620 = 149024,
    Mx4621 = 149025,
    Mx4630 = 149040,
    Mx4631 = 149041,
    Mx4640 = 149056,
    Mx4641 = 149057,
    Mx4650 = 149072,
    Mx4651 = 149073,
    Mi4710 = 18192,
    Mi4711 = 18193,
    Mi4720 = 18208,
    Mi4721 = 18209,
    Mi4730 = 18224,
    Mi4731 = 18225,
    Mi4740 = 18240,
    Mi4741 = 18241,
    M2I4710 = 214800,
    M2I4711 = 214801,
    M2I4720 = 214816,
    M2I4721 = 214817,
    M2I4730 = 214832,
    M2I4731 = 214833,
    M2I4740 = 214848,
    M2I4741 = 214849,
    M2I4710Exp = 280336,
    M2I4711Exp = 280337,
    M2I4720Exp = 280352,
    M2I4721Exp = 280353,
    M2I4730Exp = 280368,
    M2I4731Exp = 280369,
    M2I4740Exp = 280384,
    M2I4741Exp = 280385,
    Mc4710 = 83728,
    Mc4711 = 83729,
    Mc4720 = 83744,
    Mc4721 = 83745,
    Mc4730 = 83760,
    Mc4731 = 83761,
    Mx4710 = 149264,
    Mx4720 = 149280,
    Mx4730 = 149296,
    M3I4830 = 346160,
    M3I4831 = 346161,
    M3I4840 = 346176,
    M3I4841 = 346177,
    M3I4860 = 346208,
    M3I4861 = 346209,
    M3I4830Exp = 411696,
    M3I4831Exp = 411697,
    M3I4840Exp = 411712,
    M3I4841Exp = 411713,
    M3I4860Exp = 411744,
    M3I4861Exp = 411745,
    Mi4911 = 18705,
    Mi4912 = 18706,
    Mi4931 = 18737,
    Mi4932 = 18738,
    Mi4960 = 18784,
    Mi4961 = 18785,
    Mi4963 = 18787,
    Mi4964 = 18788,
    Mc4911 = 84241,
    Mc4912 = 84242,
    Mc4931 = 84273,
    Mc4932 = 84274,
    Mc4960 = 84320,
    Mc4961 = 84321,
    Mc4963 = 84323,
    Mc4964 = 84324,
    Mx4911 = 149777,
    Mx4931 = 149809,
    Mx4960 = 149856,
    Mx4963 = 149859,
    M2I4911 = 215313,
    M2I4912 = 215314,
    M2I4931 = 215345,
    M2I4932 = 215346,
    M2I4960 = 215392,
    M2I4961 = 215393,
    M2I4963 = 215395,
    M2I4964 = 215396,
    M2I4911Exp = 280849,
    M2I4912Exp = 280850,
    M2I4931Exp = 280881,
    M2I4932Exp = 280882,
    M2I4960Exp = 280928,
    M2I4961Exp = 280929,
    M2I4963Exp = 280931,
    M2I4964Exp = 280932,
    M2P59XxX4 = 612608,
    M2P5911X4 = 612625,
    M2P5912X4 = 612626,
    M2P5913X4 = 612627,
    M2P5916X4 = 612630,
    M2P5920X4 = 612640,
    M2P5921X4 = 612641,
    M2P5922X4 = 612642,
    M2P5923X4 = 612643,
    M2P5926X4 = 612646,
    M2P5930X4 = 612656,
    M2P5931X4 = 612657,
    M2P5932X4 = 612658,
    M2P5933X4 = 612659,
    M2P5936X4 = 612662,
    M2P5940X4 = 612672,
    M2P5941X4 = 612673,
    M2P5942X4 = 612674,
    M2P5943X4 = 612675,
    M2P5946X4 = 612678,
    M2P5960X4 = 612704,
    M2P5961X4 = 612705,
    M2P5962X4 = 612706,
    M2P5966X4 = 612710,
    M2P5968X4 = 612712,
    Mi6010 = 24592,
    Mi6011 = 24593,
    Mi6012 = 24594,
    Mi6021 = 24609,
    Mi6022 = 24610,
    Mi6030 = 24624,
    Mi6031 = 24625,
    Mi6033 = 24627,
    Mi6034 = 24628,
    M2I6010 = 221200,
    M2I6011 = 221201,
    M2I6012 = 221202,
    M2I6021 = 221217,
    M2I6022 = 221218,
    M2I6030 = 221232,
    M2I6031 = 221233,
    M2I6033 = 221235,
    M2I6034 = 221236,
    M2I6010Exp = 286736,
    M2I6011Exp = 286737,
    M2I6012Exp = 286738,
    M2I6021Exp = 286753,
    M2I6022Exp = 286754,
    M2I6030Exp = 286768,
    M2I6031Exp = 286769,
    M2I6033Exp = 286771,
    M2I6034Exp = 286772,
    Mc6010 = 90128,
    Mc6011 = 90129,
    Mc6012 = 90130,
    Mc6021 = 90145,
    Mc6022 = 90146,
    Mc6030 = 90160,
    Mc6031 = 90161,
    Mc6033 = 90163,
    Mc6034 = 90164,
    Mx6010 = 155664,
    Mx6011 = 155665,
    Mx6021 = 155681,
    Mx6030 = 155696,
    Mx6033 = 155699,
    Mi6105 = 24837,
    Mi6110 = 24848,
    Mi6111 = 24849,
    M2I6105 = 221445,
    M2I6110 = 221456,
    M2I6111 = 221457,
    M2I6105Exp = 286981,
    M2I6110Exp = 286992,
    M2I6111Exp = 286993,
    Mc6110 = 90384,
    Mc6111 = 90385,
    Mx6110 = 155920,
    M2P65XxX4 = 615680,
    M2P6522X4 = 615714,
    M2P6523X4 = 615715,
    M2P6530X4 = 615728,
    M2P6531X4 = 615729,
    M2P6532X4 = 615730,
    M2P6536X4 = 615734,
    M2P6533X4 = 615731,
    M2P6540X4 = 615744,
    M2P6541X4 = 615745,
    M2P6546X4 = 615750,
    M2P6560X4 = 615776,
    M2P6561X4 = 615777,
    M2P6562X4 = 615778,
    M2P6566X4 = 615782,
    M2P6568X4 = 615784,
    M2P6570X4 = 615792,
    M2P6571X4 = 615793,
    M2P6576X4 = 615798,
    M4I66XxX8 = 484864,
    M4I6620X8 = 484896,
    M4I6621X8 = 484897,
    M4I6622X8 = 484898,
    M4I6630X8 = 484912,
    M4I6631X8 = 484913,
    M4X66XxX4 = 550400,
    M4X6620X4 = 550432,
    M4X6621X4 = 550433,
    M4X6622X4 = 550434,
    M4X6630X4 = 550448,
    M4X6631X4 = 550449,
    Mi7005 = 28677,
    Mi7010 = 28688,
    Mi7011 = 28689,
    Mi7020 = 28704,
    Mi7021 = 28705,
    M2I7005 = 225285,
    M2I7010 = 225296,
    M2I7011 = 225297,
    M2I7020 = 225312,
    M2I7021 = 225313,
    M2I7005Exp = 290821,
    M2I7010Exp = 290832,
    M2I7011Exp = 290833,
    M2I7020Exp = 290848,
    M2I7021Exp = 290849,
    Mc7005 = 94213,
    Mc7010 = 94224,
    Mc7011 = 94225,
    Mc7020 = 94240,
    Mc7021 = 94241,
    Mx7005 = 159749,
    Mx7010 = 159760,
    Mx7011 = 159761,
    Mi7210 = 29200,
    Mi7211 = 29201,
    Mi7220 = 29216,
    Mi7221 = 29217,
    M2I7210 = 225808,
    M2I7211 = 225809,
    M2I7220 = 225824,
    M2I7221 = 225825,
    M2I7210Exp = 291344,
    M2I7211Exp = 291345,
    M2I7220Exp = 291360,
    M2I7221Exp = 291361,
    Mc7210 = 94736,
    Mc7211 = 94737,
    Mc7220 = 94752,
    Mc7221 = 94753,
    Mx7210 = 160272,
    Mx7220 = 160288,
    M2P75XxX4 = 619776,
    M2P7515X4 = 619797,
    M4I77XxX8 = 489216,
    M4I7710X8 = 489232,
    M4I7720X8 = 489248,
    M4I7730X8 = 489264,
    M4I7725X8 = 489253,
    M4I7735X8 = 489269,
    M4X77XxX4 = 554752,
    M4X7710X4 = 554768,
    M4X7720X4 = 554784,
    M4X7730X4 = 554800,
    M4X7725X4 = 554789,
    M4X7735X4 = 554805,
    Mx9010 = 167952,
}
