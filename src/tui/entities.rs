use phf::phf_set;

static ENTITY_SET: phf::Set<&'static str> = phf_set! {
    "\u{0009}",
    "\u{000A}",
    "\u{0021}",
    "\u{0022}",
    "\u{0023}",
    "\u{0024}",
    "\u{0025}",
    "\u{0026}",
    "\u{0027}",
    "\u{0028}",
    "\u{0029}",
    "\u{002A}",
    "\u{002B}",
    "\u{002C}",
    "\u{002E}",
    "\u{002F}",
    "\u{003A}",
    "\u{003B}",
    "\u{003C}",
    "\u{003C}\u{20d2}",
    "\u{003D}",
    "\u{003D}\u{20e5}",
    "\u{003E}",
    "\u{003E}\u{20d2}",
    "\u{003F}",
    "\u{0040}",
    "\u{005B}",
    "\u{005C}",
    "\u{005D}",
    "\u{005E}",
    "\u{005F}",
    "\u{0060}",
    "\u{0066}j",
    "\u{007B}",
    "\u{007C}",
    "\u{007D}",
    "\u{00A0}",
    "\u{00A1}",
    "\u{00A2}",
    "\u{00A3}",
    "\u{00A4}",
    "\u{00A5}",
    "\u{00A6}",
    "\u{00A7}",
    "\u{00A8}",
    "\u{00A9}",
    "\u{00AA}",
    "\u{00AB}",
    "\u{00AC}",
    "\u{00AD}",
    "\u{00AE}",
    "\u{00AF}",
    "\u{00B0}",
    "\u{00B1}",
    "\u{00B2}",
    "\u{00B3}",
    "\u{00B4}",
    "\u{00B5}",
    "\u{00B6}",
    "\u{00B7}",
    "\u{00B8}",
    "\u{00B9}",
    "\u{00BA}",
    "\u{00BB}",
    "\u{00BC}",
    "\u{00BD}",
    "\u{00BE}",
    "\u{00BF}",
    "\u{00C0}",
    "\u{00C1}",
    "\u{00C2}",
    "\u{00C3}",
    "\u{00C4}",
    "\u{00C5}",
    "\u{00C6}",
    "\u{00C7}",
    "\u{00C8}",
    "\u{00C9}",
    "\u{00CA}",
    "\u{00CB}",
    "\u{00CC}",
    "\u{00CD}",
    "\u{00CE}",
    "\u{00CF}",
    "\u{00D0}",
    "\u{00D1}",
    "\u{00D2}",
    "\u{00D3}",
    "\u{00D4}",
    "\u{00D5}",
    "\u{00D6}",
    "\u{00D7}",
    "\u{00D8}",
    "\u{00D9}",
    "\u{00DA}",
    "\u{00DB}",
    "\u{00DC}",
    "\u{00DD}",
    "\u{00DE}",
    "\u{00DF}",
    "\u{00E0}",
    "\u{00E1}",
    "\u{00E2}",
    "\u{00E3}",
    "\u{00E4}",
    "\u{00E5}",
    "\u{00E6}",
    "\u{00E7}",
    "\u{00E8}",
    "\u{00E9}",
    "\u{00EA}",
    "\u{00EB}",
    "\u{00EC}",
    "\u{00ED}",
    "\u{00EE}",
    "\u{00EF}",
    "\u{00F0}",
    "\u{00F1}",
    "\u{00F2}",
    "\u{00F3}",
    "\u{00F4}",
    "\u{00F5}",
    "\u{00F6}",
    "\u{00F7}",
    "\u{00F8}",
    "\u{00F9}",
    "\u{00FA}",
    "\u{00FB}",
    "\u{00FC}",
    "\u{00FD}",
    "\u{00FE}",
    "\u{00FF}",
    "\u{0100}",
    "\u{0101}",
    "\u{0102}",
    "\u{0103}",
    "\u{0104}",
    "\u{0105}",
    "\u{0106}",
    "\u{0107}",
    "\u{0108}",
    "\u{0109}",
    "\u{010A}",
    "\u{010B}",
    "\u{010C}",
    "\u{010D}",
    "\u{010E}",
    "\u{010F}",
    "\u{0110}",
    "\u{0111}",
    "\u{0112}",
    "\u{0113}",
    "\u{0116}",
    "\u{0117}",
    "\u{0118}",
    "\u{0119}",
    "\u{011A}",
    "\u{011B}",
    "\u{011C}",
    "\u{011D}",
    "\u{011E}",
    "\u{011F}",
    "\u{0120}",
    "\u{0121}",
    "\u{0122}",
    "\u{0124}",
    "\u{0125}",
    "\u{0126}",
    "\u{0127}",
    "\u{0128}",
    "\u{0129}",
    "\u{012A}",
    "\u{012B}",
    "\u{012E}",
    "\u{012F}",
    "\u{0130}",
    "\u{0131}",
    "\u{0132}",
    "\u{0133}",
    "\u{0134}",
    "\u{0135}",
    "\u{0136}",
    "\u{0137}",
    "\u{0138}",
    "\u{0139}",
    "\u{013A}",
    "\u{013B}",
    "\u{013C}",
    "\u{013D}",
    "\u{013E}",
    "\u{013F}",
    "\u{0140}",
    "\u{0141}",
    "\u{0142}",
    "\u{0143}",
    "\u{0144}",
    "\u{0145}",
    "\u{0146}",
    "\u{0147}",
    "\u{0148}",
    "\u{0149}",
    "\u{014A}",
    "\u{014B}",
    "\u{014C}",
    "\u{014D}",
    "\u{0150}",
    "\u{0151}",
    "\u{0152}",
    "\u{0153}",
    "\u{0154}",
    "\u{0155}",
    "\u{0156}",
    "\u{0157}",
    "\u{0158}",
    "\u{0159}",
    "\u{015A}",
    "\u{015B}",
    "\u{015C}",
    "\u{015D}",
    "\u{015E}",
    "\u{015F}",
    "\u{0160}",
    "\u{0161}",
    "\u{0162}",
    "\u{0163}",
    "\u{0164}",
    "\u{0165}",
    "\u{0166}",
    "\u{0167}",
    "\u{0168}",
    "\u{0169}",
    "\u{016A}",
    "\u{016B}",
    "\u{016C}",
    "\u{016D}",
    "\u{016E}",
    "\u{016F}",
    "\u{0170}",
    "\u{0171}",
    "\u{0172}",
    "\u{0173}",
    "\u{0174}",
    "\u{0175}",
    "\u{0176}",
    "\u{0177}",
    "\u{0178}",
    "\u{0179}",
    "\u{017A}",
    "\u{017B}",
    "\u{017C}",
    "\u{017D}",
    "\u{017E}",
    "\u{0192}",
    "\u{01B5}",
    "\u{01F5}",
    "\u{0237}",
    "\u{02C6}",
    "\u{02C7}",
    "\u{02D8}",
    "\u{02D9}",
    "\u{02DA}",
    "\u{02DB}",
    "\u{02DC}",
    "\u{02DD}",
    "\u{0311}",
    "\u{0391}",
    "\u{0392}",
    "\u{0393}",
    "\u{0394}",
    "\u{0395}",
    "\u{0396}",
    "\u{0397}",
    "\u{0398}",
    "\u{0399}",
    "\u{039A}",
    "\u{039B}",
    "\u{039C}",
    "\u{039D}",
    "\u{039E}",
    "\u{039F}",
    "\u{03A0}",
    "\u{03A1}",
    "\u{03A3}",
    "\u{03A4}",
    "\u{03A5}",
    "\u{03A6}",
    "\u{03A7}",
    "\u{03A8}",
    "\u{03A9}",
    "\u{03B1}",
    "\u{03B2}",
    "\u{03B3}",
    "\u{03B4}",
    "\u{03B5}",
    "\u{03B6}",
    "\u{03B7}",
    "\u{03B8}",
    "\u{03B9}",
    "\u{03BA}",
    "\u{03BB}",
    "\u{03BC}",
    "\u{03BD}",
    "\u{03BE}",
    "\u{03BF}",
    "\u{03C0}",
    "\u{03C1}",
    "\u{03C2}",
    "\u{03C3}",
    "\u{03C4}",
    "\u{03C5}",
    "\u{03C6}",
    "\u{03C7}",
    "\u{03C8}",
    "\u{03C9}",
    "\u{03D1}",
    "\u{03D2}",
    "\u{03D5}",
    "\u{03D6}",
    "\u{03DC}",
    "\u{03DD}",
    "\u{03F0}",
    "\u{03F1}",
    "\u{03F5}",
    "\u{03F6}",
    "\u{0401}",
    "\u{0402}",
    "\u{0403}",
    "\u{0404}",
    "\u{0405}",
    "\u{0406}",
    "\u{0407}",
    "\u{0408}",
    "\u{0409}",
    "\u{040A}",
    "\u{040B}",
    "\u{040C}",
    "\u{040E}",
    "\u{040F}",
    "\u{0410}",
    "\u{0411}",
    "\u{0412}",
    "\u{0413}",
    "\u{0414}",
    "\u{0415}",
    "\u{0416}",
    "\u{0417}",
    "\u{0418}",
    "\u{0419}",
    "\u{041A}",
    "\u{041B}",
    "\u{041C}",
    "\u{041D}",
    "\u{041E}",
    "\u{041F}",
    "\u{0420}",
    "\u{0421}",
    "\u{0422}",
    "\u{0423}",
    "\u{0424}",
    "\u{0425}",
    "\u{0426}",
    "\u{0427}",
    "\u{0428}",
    "\u{0429}",
    "\u{042A}",
    "\u{042B}",
    "\u{042C}",
    "\u{042D}",
    "\u{042E}",
    "\u{042F}",
    "\u{0430}",
    "\u{0431}",
    "\u{0432}",
    "\u{0433}",
    "\u{0434}",
    "\u{0435}",
    "\u{0436}",
    "\u{0437}",
    "\u{0438}",
    "\u{0439}",
    "\u{043A}",
    "\u{043B}",
    "\u{043C}",
    "\u{043D}",
    "\u{043E}",
    "\u{043F}",
    "\u{0440}",
    "\u{0441}",
    "\u{0442}",
    "\u{0443}",
    "\u{0444}",
    "\u{0445}",
    "\u{0446}",
    "\u{0447}",
    "\u{0448}",
    "\u{0449}",
    "\u{044A}",
    "\u{044B}",
    "\u{044C}",
    "\u{044D}",
    "\u{044E}",
    "\u{044F}",
    "\u{0451}",
    "\u{0452}",
    "\u{0453}",
    "\u{0454}",
    "\u{0455}",
    "\u{0456}",
    "\u{0457}",
    "\u{0458}",
    "\u{0459}",
    "\u{045A}",
    "\u{045B}",
    "\u{045C}",
    "\u{045E}",
    "\u{045F}",
    "\u{1D49C}",
    "\u{1D49E}",
    "\u{1D49F}",
    "\u{1D4A2}",
    "\u{1D4A5}",
    "\u{1D4A6}",
    "\u{1D4A9}",
    "\u{1D4AA}",
    "\u{1D4AB}",
    "\u{1D4AC}",
    "\u{1D4AE}",
    "\u{1D4AF}",
    "\u{1D4B0}",
    "\u{1D4B1}",
    "\u{1D4B2}",
    "\u{1D4B3}",
    "\u{1D4B4}",
    "\u{1D4B5}",
    "\u{1D4B6}",
    "\u{1D4B7}",
    "\u{1D4B8}",
    "\u{1D4B9}",
    "\u{1D4BB}",
    "\u{1D4BD}",
    "\u{1D4BE}",
    "\u{1D4BF}",
    "\u{1D4C0}",
    "\u{1D4C1}",
    "\u{1D4C2}",
    "\u{1D4C3}",
    "\u{1D4C5}",
    "\u{1D4C6}",
    "\u{1D4C7}",
    "\u{1D4C8}",
    "\u{1D4C9}",
    "\u{1D4CA}",
    "\u{1D4CB}",
    "\u{1D4CC}",
    "\u{1D4CD}",
    "\u{1D4CE}",
    "\u{1D4CF}",
    "\u{1D504}",
    "\u{1D505}",
    "\u{1D507}",
    "\u{1D508}",
    "\u{1D509}",
    "\u{1D50A}",
    "\u{1D50D}",
    "\u{1D50E}",
    "\u{1D50F}",
    "\u{1D510}",
    "\u{1D511}",
    "\u{1D512}",
    "\u{1D513}",
    "\u{1D514}",
    "\u{1D516}",
    "\u{1D517}",
    "\u{1D518}",
    "\u{1D519}",
    "\u{1D51A}",
    "\u{1D51B}",
    "\u{1D51C}",
    "\u{1D51E}",
    "\u{1D51F}",
    "\u{1D520}",
    "\u{1D521}",
    "\u{1D522}",
    "\u{1D523}",
    "\u{1D524}",
    "\u{1D525}",
    "\u{1D526}",
    "\u{1D527}",
    "\u{1D528}",
    "\u{1D529}",
    "\u{1D52A}",
    "\u{1D52B}",
    "\u{1D52C}",
    "\u{1D52D}",
    "\u{1D52E}",
    "\u{1D52F}",
    "\u{1D530}",
    "\u{1D531}",
    "\u{1D532}",
    "\u{1D533}",
    "\u{1D534}",
    "\u{1D535}",
    "\u{1D536}",
    "\u{1D537}",
    "\u{1D538}",
    "\u{1D539}",
    "\u{1D53B}",
    "\u{1D53C}",
    "\u{1D53D}",
    "\u{1D53E}",
    "\u{1D540}",
    "\u{1D541}",
    "\u{1D542}",
    "\u{1D543}",
    "\u{1D544}",
    "\u{1D546}",
    "\u{1D54A}",
    "\u{1D54B}",
    "\u{1D54C}",
    "\u{1D54D}",
    "\u{1D54E}",
    "\u{1D54F}",
    "\u{1D550}",
    "\u{1D552}",
    "\u{1D553}",
    "\u{1D554}",
    "\u{1D555}",
    "\u{1D556}",
    "\u{1D557}",
    "\u{1D558}",
    "\u{1D559}",
    "\u{1D55A}",
    "\u{1D55B}",
    "\u{1D55C}",
    "\u{1D55D}",
    "\u{1D55E}",
    "\u{1D55F}",
    "\u{1D560}",
    "\u{1D561}",
    "\u{1D562}",
    "\u{1D563}",
    "\u{1D564}",
    "\u{1D565}",
    "\u{1D566}",
    "\u{1D567}",
    "\u{1D568}",
    "\u{1D569}",
    "\u{1D56A}",
    "\u{1D56B}",
    "\u{2002}",
    "\u{2003}",
    "\u{2004}",
    "\u{2005}",
    "\u{2007}",
    "\u{2008}",
    "\u{2009}",
    "\u{200A}",
    "\u{200B}",
    "\u{200C}",
    "\u{200D}",
    "\u{200E}",
    "\u{200F}",
    "\u{2010}",
    "\u{2013}",
    "\u{2014}",
    "\u{2015}",
    "\u{2016}",
    "\u{2018}",
    "\u{2019}",
    "\u{201A}",
    "\u{201C}",
    "\u{201D}",
    "\u{201E}",
    "\u{2020}",
    "\u{2021}",
    "\u{2022}",
    "\u{2025}",
    "\u{2026}",
    "\u{2030}",
    "\u{2031}",
    "\u{2032}",
    "\u{2033}",
    "\u{2034}",
    "\u{2035}",
    "\u{2039}",
    "\u{203A}",
    "\u{203E}",
    "\u{2041}",
    "\u{2043}",
    "\u{2044}",
    "\u{204F}",
    "\u{2057}",
    "\u{205F}",
    "\u{205F}\u{200a}",
    "\u{2060}",
    "\u{2061}",
    "\u{2062}",
    "\u{2063}",
    "\u{20AC}",
    "\u{20DB}",
    "\u{20DC}",
    "\u{2102}",
    "\u{2105}",
    "\u{210A}",
    "\u{210B}",
    "\u{210C}",
    "\u{210D}",
    "\u{210E}",
    "\u{210F}",
    "\u{2110}",
    "\u{2111}",
    "\u{2112}",
    "\u{2113}",
    "\u{2115}",
    "\u{2116}",
    "\u{2117}",
    "\u{2118}",
    "\u{2119}",
    "\u{211A}",
    "\u{211B}",
    "\u{211C}",
    "\u{211D}",
    "\u{211E}",
    "\u{2122}",
    "\u{2124}",
    "\u{2127}",
    "\u{2128}",
    "\u{2129}",
    "\u{212C}",
    "\u{212D}",
    "\u{212F}",
    "\u{2130}",
    "\u{2131}",
    "\u{2133}",
    "\u{2134}",
    "\u{2135}",
    "\u{2136}",
    "\u{2137}",
    "\u{2138}",
    "\u{2145}",
    "\u{2146}",
    "\u{2147}",
    "\u{2148}",
    "\u{2153}",
    "\u{2154}",
    "\u{2155}",
    "\u{2156}",
    "\u{2157}",
    "\u{2158}",
    "\u{2159}",
    "\u{215A}",
    "\u{215B}",
    "\u{215C}",
    "\u{215D}",
    "\u{215E}",
    "\u{2190}",
    "\u{2191}",
    "\u{2192}",
    "\u{2193}",
    "\u{2194}",
    "\u{2195}",
    "\u{2196}",
    "\u{2197}",
    "\u{2198}",
    "\u{2199}",
    "\u{219A}",
    "\u{219B}",
    "\u{219D}",
    "\u{219D}\u{338}",
    "\u{219E}",
    "\u{219F}",
    "\u{21A0}",
    "\u{21A1}",
    "\u{21A2}",
    "\u{21A3}",
    "\u{21A4}",
    "\u{21A5}",
    "\u{21A6}",
    "\u{21A7}",
    "\u{21A9}",
    "\u{21AA}",
    "\u{21AB}",
    "\u{21AC}",
    "\u{21AD}",
    "\u{21AE}",
    "\u{21B0}",
    "\u{21B1}",
    "\u{21B2}",
    "\u{21B3}",
    "\u{21B5}",
    "\u{21B6}",
    "\u{21B7}",
    "\u{21BA}",
    "\u{21BB}",
    "\u{21BC}",
    "\u{21BD}",
    "\u{21BE}",
    "\u{21BF}",
    "\u{21C0}",
    "\u{21C1}",
    "\u{21C2}",
    "\u{21C3}",
    "\u{21C4}",
    "\u{21C5}",
    "\u{21C6}",
    "\u{21C7}",
    "\u{21C8}",
    "\u{21C9}",
    "\u{21CA}",
    "\u{21CB}",
    "\u{21CC}",
    "\u{21CD}",
    "\u{21CE}",
    "\u{21CF}",
    "\u{21D0}",
    "\u{21D1}",
    "\u{21D2}",
    "\u{21D3}",
    "\u{21D4}",
    "\u{21D5}",
    "\u{21D6}",
    "\u{21D7}",
    "\u{21D8}",
    "\u{21D9}",
    "\u{21DA}",
    "\u{21DB}",
    "\u{21DD}",
    "\u{21E4}",
    "\u{21E5}",
    "\u{21F5}",
    "\u{21FD}",
    "\u{21FE}",
    "\u{21FF}",
    "\u{2200}",
    "\u{2201}",
    "\u{2202}",
    "\u{2202}\u{338}",
    "\u{2203}",
    "\u{2204}",
    "\u{2205}",
    "\u{2207}",
    "\u{2208}",
    "\u{2209}",
    "\u{220B}",
    "\u{220C}",
    "\u{220F}",
    "\u{2210}",
    "\u{2211}",
    "\u{2212}",
    "\u{2213}",
    "\u{2214}",
    "\u{2216}",
    "\u{2217}",
    "\u{2218}",
    "\u{221A}",
    "\u{221D}",
    "\u{221E}",
    "\u{221F}",
    "\u{2220}",
    "\u{2220}\u{20d2}",
    "\u{2221}",
    "\u{2222}",
    "\u{2223}",
    "\u{2224}",
    "\u{2225}",
    "\u{2226}",
    "\u{2227}",
    "\u{2228}",
    "\u{2229}",
    "\u{2229}\u{fe00}",
    "\u{222A}",
    "\u{222A}\u{fe00}",
    "\u{222B}",
    "\u{222C}",
    "\u{222D}",
    "\u{222E}",
    "\u{222F}",
    "\u{2230}",
    "\u{2231}",
    "\u{2232}",
    "\u{2233}",
    "\u{2234}",
    "\u{2235}",
    "\u{2236}",
    "\u{2237}",
    "\u{2238}",
    "\u{223A}",
    "\u{223B}",
    "\u{223C}",
    "\u{223C}\u{20d2}",
    "\u{223D}",
    "\u{223D}\u{331}",
    "\u{223E}",
    "\u{223E}\u{333}",
    "\u{223F}",
    "\u{2240}",
    "\u{2241}",
    "\u{2242}",
    "\u{2242}\u{338}",
    "\u{2243}",
    "\u{2244}",
    "\u{2245}",
    "\u{2246}",
    "\u{2247}",
    "\u{2248}",
    "\u{2249}",
    "\u{224A}",
    "\u{224B}",
    "\u{224B}\u{338}",
    "\u{224C}",
    "\u{224D}",
    "\u{224D}\u{20d2}",
    "\u{224E}",
    "\u{224E}\u{338}",
    "\u{224F}",
    "\u{224F}\u{338}",
    "\u{2250}",
    "\u{2250}\u{338}",
    "\u{2251}",
    "\u{2252}",
    "\u{2253}",
    "\u{2254}",
    "\u{2255}",
    "\u{2256}",
    "\u{2257}",
    "\u{2259}",
    "\u{225A}",
    "\u{225C}",
    "\u{225F}",
    "\u{2260}",
    "\u{2261}",
    "\u{2261}\u{20e5}",
    "\u{2262}",
    "\u{2264}",
    "\u{2264}\u{20d2}",
    "\u{2265}",
    "\u{2265}\u{20d2}",
    "\u{2266}",
    "\u{2266}\u{338}",
    "\u{2267}",
    "\u{2267}\u{338}",
    "\u{2268}",
    "\u{2268}\u{fe00}",
    "\u{2269}",
    "\u{2269}\u{fe00}",
    "\u{226A}",
    "\u{226A}\u{338}",
    "\u{226A}\u{20d2}",
    "\u{226B}",
    "\u{226B}\u{338}",
    "\u{226B}\u{20d2}",
    "\u{226C}",
    "\u{226D}",
    "\u{226E}",
    "\u{226F}",
    "\u{2270}",
    "\u{2271}",
    "\u{2272}",
    "\u{2273}",
    "\u{2274}",
    "\u{2275}",
    "\u{2276}",
    "\u{2277}",
    "\u{2278}",
    "\u{2279}",
    "\u{227A}",
    "\u{227B}",
    "\u{227C}",
    "\u{227D}",
    "\u{227E}",
    "\u{227F}",
    "\u{227F}\u{338}",
    "\u{2280}",
    "\u{2281}",
    "\u{2282}",
    "\u{2282}\u{20d2}",
    "\u{2283}",
    "\u{2283}\u{20d2}",
    "\u{2284}",
    "\u{2285}",
    "\u{2286}",
    "\u{2287}",
    "\u{2288}",
    "\u{2289}",
    "\u{228A}",
    "\u{228A}\u{fe00}",
    "\u{228B}",
    "\u{228B}\u{fe00}",
    "\u{228D}",
    "\u{228E}",
    "\u{228F}",
    "\u{228F}\u{338}",
    "\u{2290}",
    "\u{2290}\u{338}",
    "\u{2291}",
    "\u{2292}",
    "\u{2293}",
    "\u{2293}\u{fe00}",
    "\u{2294}",
    "\u{2294}\u{fe00}",
    "\u{2295}",
    "\u{2296}",
    "\u{2297}",
    "\u{2298}",
    "\u{2299}",
    "\u{229A}",
    "\u{229B}",
    "\u{229D}",
    "\u{229E}",
    "\u{229F}",
    "\u{22A0}",
    "\u{22A1}",
    "\u{22A2}",
    "\u{22A3}",
    "\u{22A4}",
    "\u{22A5}",
    "\u{22A7}",
    "\u{22A8}",
    "\u{22A9}",
    "\u{22AA}",
    "\u{22AB}",
    "\u{22AC}",
    "\u{22AD}",
    "\u{22AE}",
    "\u{22AF}",
    "\u{22B0}",
    "\u{22B2}",
    "\u{22B3}",
    "\u{22B4}",
    "\u{22B4}\u{20d2}",
    "\u{22B5}",
    "\u{22B5}\u{20d2}",
    "\u{22B6}",
    "\u{22B7}",
    "\u{22B8}",
    "\u{22B9}",
    "\u{22BA}",
    "\u{22BB}",
    "\u{22BD}",
    "\u{22BE}",
    "\u{22BF}",
    "\u{22C0}",
    "\u{22C1}",
    "\u{22C2}",
    "\u{22C3}",
    "\u{22C4}",
    "\u{22C5}",
    "\u{22C6}",
    "\u{22C7}",
    "\u{22C8}",
    "\u{22C9}",
    "\u{22CA}",
    "\u{22CB}",
    "\u{22CC}",
    "\u{22CD}",
    "\u{22CE}",
    "\u{22CF}",
    "\u{22D0}",
    "\u{22D1}",
    "\u{22D2}",
    "\u{22D3}",
    "\u{22D4}",
    "\u{22D5}",
    "\u{22D6}",
    "\u{22D7}",
    "\u{22D8}",
    "\u{22D8}\u{338}",
    "\u{22D9}",
    "\u{22D9}\u{338}",
    "\u{22DA}",
    "\u{22DA}\u{fe00}",
    "\u{22DB}",
    "\u{22DB}\u{fe00}",
    "\u{22DE}",
    "\u{22DF}",
    "\u{22E0}",
    "\u{22E1}",
    "\u{22E2}",
    "\u{22E3}",
    "\u{22E6}",
    "\u{22E7}",
    "\u{22E8}",
    "\u{22E9}",
    "\u{22EA}",
    "\u{22EB}",
    "\u{22EC}",
    "\u{22ED}",
    "\u{22EE}",
    "\u{22EF}",
    "\u{22F0}",
    "\u{22F1}",
    "\u{22F2}",
    "\u{22F3}",
    "\u{22F4}",
    "\u{22F5}",
    "\u{22F5}\u{338}",
    "\u{22F6}",
    "\u{22F7}",
    "\u{22F9}",
    "\u{22F9}\u{338}",
    "\u{22FA}",
    "\u{22FB}",
    "\u{22FC}",
    "\u{22FD}",
    "\u{22FE}",
    "\u{2305}",
    "\u{2306}",
    "\u{2308}",
    "\u{2309}",
    "\u{230A}",
    "\u{230B}",
    "\u{230C}",
    "\u{230D}",
    "\u{230E}",
    "\u{230F}",
    "\u{2310}",
    "\u{2312}",
    "\u{2313}",
    "\u{2315}",
    "\u{2316}",
    "\u{231C}",
    "\u{231D}",
    "\u{231E}",
    "\u{231F}",
    "\u{2322}",
    "\u{2323}",
    "\u{232D}",
    "\u{232E}",
    "\u{2336}",
    "\u{233D}",
    "\u{233F}",
    "\u{237C}",
    "\u{23B0}",
    "\u{23B1}",
    "\u{23B4}",
    "\u{23B5}",
    "\u{23B6}",
    "\u{23DC}",
    "\u{23DD}",
    "\u{23DE}",
    "\u{23DF}",
    "\u{23E2}",
    "\u{23E7}",
    "\u{2423}",
    "\u{24C8}",
    "\u{2500}",
    "\u{2502}",
    "\u{250C}",
    "\u{2510}",
    "\u{2514}",
    "\u{2518}",
    "\u{251C}",
    "\u{2524}",
    "\u{252C}",
    "\u{2534}",
    "\u{253C}",
    "\u{2550}",
    "\u{2551}",
    "\u{2552}",
    "\u{2553}",
    "\u{2554}",
    "\u{2555}",
    "\u{2556}",
    "\u{2557}",
    "\u{2558}",
    "\u{2559}",
    "\u{255A}",
    "\u{255B}",
    "\u{255C}",
    "\u{255D}",
    "\u{255E}",
    "\u{255F}",
    "\u{2560}",
    "\u{2561}",
    "\u{2562}",
    "\u{2563}",
    "\u{2564}",
    "\u{2565}",
    "\u{2566}",
    "\u{2567}",
    "\u{2568}",
    "\u{2569}",
    "\u{256A}",
    "\u{256B}",
    "\u{256C}",
    "\u{2580}",
    "\u{2584}",
    "\u{2588}",
    "\u{2591}",
    "\u{2592}",
    "\u{2593}",
    "\u{25A1}",
    "\u{25AA}",
    "\u{25AB}",
    "\u{25AD}",
    "\u{25AE}",
    "\u{25B1}",
    "\u{25B3}",
    "\u{25B4}",
    "\u{25B5}",
    "\u{25B8}",
    "\u{25B9}",
    "\u{25BD}",
    "\u{25BE}",
    "\u{25BF}",
    "\u{25C2}",
    "\u{25C3}",
    "\u{25CA}",
    "\u{25CB}",
    "\u{25EC}",
    "\u{25EF}",
    "\u{25F8}",
    "\u{25F9}",
    "\u{25FA}",
    "\u{25FB}",
    "\u{25FC}",
    "\u{2605}",
    "\u{2606}",
    "\u{260E}",
    "\u{2640}",
    "\u{2642}",
    "\u{2660}",
    "\u{2663}",
    "\u{2665}",
    "\u{2666}",
    "\u{266A}",
    "\u{266D}",
    "\u{266E}",
    "\u{266F}",
    "\u{2713}",
    "\u{2717}",
    "\u{2720}",
    "\u{2736}",
    "\u{2758}",
    "\u{2772}",
    "\u{2773}",
    "\u{27C8}",
    "\u{27C9}",
    "\u{27E6}",
    "\u{27E7}",
    "\u{27E8}",
    "\u{27E9}",
    "\u{27EA}",
    "\u{27EB}",
    "\u{27EC}",
    "\u{27ED}",
    "\u{27F5}",
    "\u{27F6}",
    "\u{27F7}",
    "\u{27F8}",
    "\u{27F9}",
    "\u{27FA}",
    "\u{27FC}",
    "\u{27FF}",
    "\u{2902}",
    "\u{2903}",
    "\u{2904}",
    "\u{2905}",
    "\u{290C}",
    "\u{290D}",
    "\u{290E}",
    "\u{290F}",
    "\u{2910}",
    "\u{2911}",
    "\u{2912}",
    "\u{2913}",
    "\u{2916}",
    "\u{2919}",
    "\u{291A}",
    "\u{291B}",
    "\u{291C}",
    "\u{291D}",
    "\u{291E}",
    "\u{291F}",
    "\u{2920}",
    "\u{2923}",
    "\u{2924}",
    "\u{2925}",
    "\u{2926}",
    "\u{2927}",
    "\u{2928}",
    "\u{2929}",
    "\u{292A}",
    "\u{2933}",
    "\u{2933}\u{338}",
    "\u{2935}",
    "\u{2936}",
    "\u{2937}",
    "\u{2938}",
    "\u{2939}",
    "\u{293C}",
    "\u{293D}",
    "\u{2945}",
    "\u{2948}",
    "\u{2949}",
    "\u{294A}",
    "\u{294B}",
    "\u{294E}",
    "\u{294F}",
    "\u{2950}",
    "\u{2951}",
    "\u{2952}",
    "\u{2953}",
    "\u{2954}",
    "\u{2955}",
    "\u{2956}",
    "\u{2957}",
    "\u{2958}",
    "\u{2959}",
    "\u{295A}",
    "\u{295B}",
    "\u{295C}",
    "\u{295D}",
    "\u{295E}",
    "\u{295F}",
    "\u{2960}",
    "\u{2961}",
    "\u{2962}",
    "\u{2963}",
    "\u{2964}",
    "\u{2965}",
    "\u{2966}",
    "\u{2967}",
    "\u{2968}",
    "\u{2969}",
    "\u{296A}",
    "\u{296B}",
    "\u{296C}",
    "\u{296D}",
    "\u{296E}",
    "\u{296F}",
    "\u{2970}",
    "\u{2971}",
    "\u{2972}",
    "\u{2973}",
    "\u{2974}",
    "\u{2975}",
    "\u{2976}",
    "\u{2978}",
    "\u{2979}",
    "\u{297B}",
    "\u{297C}",
    "\u{297D}",
    "\u{297E}",
    "\u{297F}",
    "\u{2985}",
    "\u{2986}",
    "\u{298B}",
    "\u{298C}",
    "\u{298D}",
    "\u{298E}",
    "\u{298F}",
    "\u{2990}",
    "\u{2991}",
    "\u{2992}",
    "\u{2993}",
    "\u{2994}",
    "\u{2995}",
    "\u{2996}",
    "\u{299A}",
    "\u{299C}",
    "\u{299D}",
    "\u{29A4}",
    "\u{29A5}",
    "\u{29A6}",
    "\u{29A7}",
    "\u{29A8}",
    "\u{29A9}",
    "\u{29AA}",
    "\u{29AB}",
    "\u{29AC}",
    "\u{29AD}",
    "\u{29AE}",
    "\u{29AF}",
    "\u{29B0}",
    "\u{29B1}",
    "\u{29B2}",
    "\u{29B3}",
    "\u{29B4}",
    "\u{29B5}",
    "\u{29B6}",
    "\u{29B7}",
    "\u{29B9}",
    "\u{29BB}",
    "\u{29BC}",
    "\u{29BE}",
    "\u{29BF}",
    "\u{29C0}",
    "\u{29C1}",
    "\u{29C2}",
    "\u{29C3}",
    "\u{29C4}",
    "\u{29C5}",
    "\u{29C9}",
    "\u{29CD}",
    "\u{29CE}",
    "\u{29CF}",
    "\u{29CF}\u{338}",
    "\u{29D0}",
    "\u{29D0}\u{338}",
    "\u{29DC}",
    "\u{29DD}",
    "\u{29DE}",
    "\u{29E3}",
    "\u{29E4}",
    "\u{29E5}",
    "\u{29EB}",
    "\u{29F4}",
    "\u{29F6}",
    "\u{2A00}",
    "\u{2A01}",
    "\u{2A02}",
    "\u{2A04}",
    "\u{2A06}",
    "\u{2A0C}",
    "\u{2A0D}",
    "\u{2A10}",
    "\u{2A11}",
    "\u{2A12}",
    "\u{2A13}",
    "\u{2A14}",
    "\u{2A15}",
    "\u{2A16}",
    "\u{2A17}",
    "\u{2A22}",
    "\u{2A23}",
    "\u{2A24}",
    "\u{2A25}",
    "\u{2A26}",
    "\u{2A27}",
    "\u{2A29}",
    "\u{2A2A}",
    "\u{2A2D}",
    "\u{2A2E}",
    "\u{2A2F}",
    "\u{2A30}",
    "\u{2A31}",
    "\u{2A33}",
    "\u{2A34}",
    "\u{2A35}",
    "\u{2A36}",
    "\u{2A37}",
    "\u{2A38}",
    "\u{2A39}",
    "\u{2A3A}",
    "\u{2A3B}",
    "\u{2A3C}",
    "\u{2A3F}",
    "\u{2A40}",
    "\u{2A42}",
    "\u{2A43}",
    "\u{2A44}",
    "\u{2A45}",
    "\u{2A46}",
    "\u{2A47}",
    "\u{2A48}",
    "\u{2A49}",
    "\u{2A4A}",
    "\u{2A4B}",
    "\u{2A4C}",
    "\u{2A4D}",
    "\u{2A50}",
    "\u{2A53}",
    "\u{2A54}",
    "\u{2A55}",
    "\u{2A56}",
    "\u{2A57}",
    "\u{2A58}",
    "\u{2A5A}",
    "\u{2A5B}",
    "\u{2A5C}",
    "\u{2A5D}",
    "\u{2A5F}",
    "\u{2A66}",
    "\u{2A6A}",
    "\u{2A6D}",
    "\u{2A6D}\u{338}",
    "\u{2A6E}",
    "\u{2A6F}",
    "\u{2A70}",
    "\u{2A70}\u{338}",
    "\u{2A71}",
    "\u{2A72}",
    "\u{2A73}",
    "\u{2A74}",
    "\u{2A75}",
    "\u{2A77}",
    "\u{2A78}",
    "\u{2A79}",
    "\u{2A7A}",
    "\u{2A7B}",
    "\u{2A7C}",
    "\u{2A7D}",
    "\u{2A7D}\u{338}",
    "\u{2A7E}",
    "\u{2A7E}\u{338}",
    "\u{2A7F}",
    "\u{2A80}",
    "\u{2A81}",
    "\u{2A82}",
    "\u{2A83}",
    "\u{2A84}",
    "\u{2A85}",
    "\u{2A86}",
    "\u{2A87}",
    "\u{2A88}",
    "\u{2A89}",
    "\u{2A8A}",
    "\u{2A8B}",
    "\u{2A8C}",
    "\u{2A8D}",
    "\u{2A8E}",
    "\u{2A8F}",
    "\u{2A90}",
    "\u{2A91}",
    "\u{2A92}",
    "\u{2A93}",
    "\u{2A94}",
    "\u{2A95}",
    "\u{2A96}",
    "\u{2A97}",
    "\u{2A98}",
    "\u{2A99}",
    "\u{2A9A}",
    "\u{2A9D}",
    "\u{2A9E}",
    "\u{2A9F}",
    "\u{2AA0}",
    "\u{2AA1}",
    "\u{2AA1}\u{338}",
    "\u{2AA2}",
    "\u{2AA2}\u{338}",
    "\u{2AA4}",
    "\u{2AA5}",
    "\u{2AA6}",
    "\u{2AA7}",
    "\u{2AA8}",
    "\u{2AA9}",
    "\u{2AAA}",
    "\u{2AAB}",
    "\u{2AAC}",
    "\u{2AAC}\u{fe00}",
    "\u{2AAD}",
    "\u{2AAD}\u{fe00}",
    "\u{2AAE}",
    "\u{2AAF}",
    "\u{2AAF}\u{338}",
    "\u{2AB0}",
    "\u{2AB0}\u{338}",
    "\u{2AB3}",
    "\u{2AB4}",
    "\u{2AB5}",
    "\u{2AB6}",
    "\u{2AB7}",
    "\u{2AB8}",
    "\u{2AB9}",
    "\u{2ABA}",
    "\u{2ABB}",
    "\u{2ABC}",
    "\u{2ABD}",
    "\u{2ABE}",
    "\u{2ABF}",
    "\u{2AC0}",
    "\u{2AC1}",
    "\u{2AC2}",
    "\u{2AC3}",
    "\u{2AC4}",
    "\u{2AC5}",
    "\u{2AC5}\u{338}",
    "\u{2AC6}",
    "\u{2AC6}\u{338}",
    "\u{2AC7}",
    "\u{2AC8}",
    "\u{2ACB}",
    "\u{2ACB}\u{fe00}",
    "\u{2ACC}",
    "\u{2ACC}\u{fe00}",
    "\u{2ACF}",
    "\u{2AD0}",
    "\u{2AD1}",
    "\u{2AD2}",
    "\u{2AD3}",
    "\u{2AD4}",
    "\u{2AD5}",
    "\u{2AD6}",
    "\u{2AD7}",
    "\u{2AD8}",
    "\u{2AD9}",
    "\u{2ADA}",
    "\u{2ADB}",
    "\u{2AE4}",
    "\u{2AE6}",
    "\u{2AE7}",
    "\u{2AE8}",
    "\u{2AE9}",
    "\u{2AEB}",
    "\u{2AEC}",
    "\u{2AED}",
    "\u{2AEE}",
    "\u{2AEF}",
    "\u{2AF0}",
    "\u{2AF1}",
    "\u{2AF2}",
    "\u{2AF3}",
    "\u{2AFD}",
    "\u{2AFD}\u{20e5}",
    "\u{FB00}",
    "\u{FB01}",
    "\u{FB02}",
    "\u{FB03}",
    "\u{FB04}",
};
// Taken from pulldown_cmark for consistency
// Hack, yes, but the real question is why cursive styled spans need to keep
// their original input; this doesn't make sense in the context of a markdown
// parser which changes the input...
pub fn is_entity(s: &str) -> bool {
    ENTITY_SET.contains(s)
}