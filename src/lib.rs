// LNGCNV VERSION 1.6.0-ALPHA.3 / MIT LICENSE © 2022 PIOTR BAJDEK

// LIBRARY

use std::io::Write;
use std::fs::OpenOptions;

// ENGLISH: IPA

pub fn ipaaueng(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str1dot = original_text.to_owned() + "."; // mark word ending
   let str2 = &str1dot.to_lowercase();
   let str3 = &str2.replace("i'd", "ɐːd");
   let str4 = &str3.replace("i’d", "ɐːd");
   let str5 = &str4.replace("i'll", "ɑːl");
   let str6 = &str5.replace("i’ll", "ɑːl");
   let str7 = &str6.replace("i'm", "ɐ̃ːm");
   let str8 = &str7.replace("i’m", "ɐ̃ːm");
   let str9 = &str8.replace("i've", "ɐːv");
   let str10 = &str9.replace("i’ve", "ɐːv");
   let str11 = &str10.replace("'", "");
   let str12 = &str11.replace("’", "");
   let str13 = &str12.replace("etc.", "e̞tˈs̺ɛ̝t͡ʃɹə.");
   let str14 = &str13.replace("answ", "ɑːns");
   let str15 = &str14.replace("ballet", "bɐle̞ɪ̝");
   let str16 = &str15.replace("caramel", "ˈkʰæɹməɫ");
   let str17 = &str16.replace("challeng", "t͡ʃæl̥ɪ̝nd͡ʒ");
   let str18 = &str17.replace("christmas", "krismas");
   let str19 = &str18.replace("debris", "dɪbɹiː");
   let str20 = &str19.replace("entrepreneur", "ˌɒnt̠ɹ̥əpɹəˈnɜː");
   let str21 = &str20.replace("garage", "ˈɡɐɹəd͡ʒ");
   let str22 = &str21.replace("listen", "lɪ̝sᵊn");
   let str23 = &str22.replace("often", "ɒfᵊn");
   let str24 = &str23.replace("perhaps", "pe̞ːhæps");
   let str25 = &str24.replace("comm", "kʰɒmm");
   let str26 = &str25.replace("litt", "lɪ̝tt");
   let str27 = &str26.replace("alumi", "ælʲʉmɪ̝");
   let str28 = &str27.replace("nium", "nʲəm");
   let str29 = &str28.replace("subtl", "sɐtl");
   let str30 = &str29.replace("debt", "det");
   let str31 = &str30.replace("wedn", "wɛn");
   let str32 = &str31.replace("yed", "yd̥");
   let str33 = &str32.replace("bb", "b");
   let str34 = &str33.replace("cce", "kse");
   let str35 = &str34.replace("cci", "ksɪ̝");
   let str36 = &str35.replace("musc", "mɐsc");
   let str37 = &str36.replace("psycho", "sɐɪ̝ko");
   let str38 = &str37.replace("sca", "s.ca");
   let str39 = &str38.replace("sc", "s");
   let str40 = &str39.replace("cen", "sen");
   let str41 = &str40.replace("dd", "d̥");
   let str42 = &str41.replace("choir", "kwɐɪ̝ə");
   let str43 = &str42.replace("ghost", "gɞ̜ʉ̟st");
   let str44 = &str43.replace("gh", "g");
   let str45 = &str44.replace("gue", "ge");
   let str46 = &str45.replace("kn", "n");
   let str47 = &str46.replace("qu", "kw");
   let str48 = &str47.replace("rh", "ɹ");
   let str49 = &str48.replace("ttem", "tʰem");
   let str50 = &str49.replace("tten", "tʰen");
   let str51 = &str50.replace("wr", "ɹ");
   let str52 = &str51.replace("wha", "wɒ");
   let str53 = &str52.replace("dess", "dɪ̝ss");
   let str54 = &str53.replace("sse", "ze");
   let str55 = &str54.replace("pad", "pæd");
   let str56 = &str55.replace("ve read", "ve ɹe̞ːd");
   let str57 = &str56.replace("has read", "has ɹe̞ːd");
   let str58 = &str57.replace("nt read", "nt ɹe̞ːd");
   let str59 = &str58.replace("ered", "əd̥");
   let str60 = &str59.replace("bare", "be̞ː");
   let str61 = &str60.replace("bari", "be̞ːɹi");
   let str62 = &str61.replace("care", "kʰe̞ː");
   let str63 = &str62.replace("cari", "kʰe̞ːɹi");
   let str64 = &str63.replace("dare", "de̞ː");
   let str65 = &str64.replace("dari", "de̞ːɹi");
   let str66 = &str65.replace("mare", "me̞ː");
   let str67 = &str66.replace("mari", "me̞ːɹi");
   let str68 = &str67.replace("share", "ʃe̞ː");
   let str69 = &str68.replace("shari", "ʃe̞ːɹi");
   let str70 = &str69.replace("tare", "te̞ː");
   let str71 = &str70.replace("tari", "te̞ːɹi");
   let str72 = &str71.replace("ware", "we̞ː");
   let str73 = &str72.replace("esta", "ɪ̝sta");
   let str74 = &str73.replace("est ", "ɪ̝st ");
   let str75 = &str74.replace("est.", "ɪ̝st.");
   let str76 = &str75.replace("est,", "ɪ̝st,");
   let str77 = &str76.replace("est?", "ɪ̝st?");
   let str78 = &str77.replace("est!", "ɪ̝st!");
   let str79 = &str78.replace("est;", "ɪ̝st;");
   let str80 = &str79.replace("est-", "ɪ̝st-");
   let str81 = &str80.replace("est)", "ɪ̝st)");
   let str82 = &str81.replace("bɪ̝st", "be̞st");
   let str83 = &str82.replace("rɪ̝st", "re̞st");
   let str84 = &str83.replace("uɪ̝st", "e̞st");
   let str85 = &str84.replace("wɪ̝st", "we̞st");
   let str86 = &str85.replace("sword", "s̺ɞːd̥");
   let str87 = &str86.replace("ord", "ɜːd");
   let str88 = &str87.replace("orld", "ɜːld");
   let str89 = &str88.replace("ork", "ɜːk");
   let str90 = &str89.replace("orth", "oːθ");
   let str91 = &str90.replace("ore̞st", "oɹe̞st");
   let str92 = &str91.replace("ore", "oː");
   let str93 = &str92.replace("or", "oː");
   let str94 = &str93.replace("air", "eː");
   let str95 = &str94.replace("there", "ðeː");
   let str96 = &str95.replace("were", "wɜ̝ː");
   let str97 = &str96.replace("irr", "ɜ̝ːɹ");
   let str98 = &str97.replace("circ", "sɜ̝ːc");
   let str99 = &str98.replace("ir", "ɜ̝ː");
   let str100 = &str99.replace("isl", "ɐɪ̝l");
   let str101 = &str100.replace("rious", "ɹiəs");
   let str102 = &str101.replace("cious", "ʃiəs");
   let str103 = &str102.replace("tious", "ʃiəs");
   let str104 = &str103.replace("ious", "iəs");
   let str105 = &str104.replace("bour", "bə");
   let str106 = &str105.replace("iour", "j|ə");
   let str107 = &str106.replace("nour", "nə");
   let str108 = &str107.replace("olour", "ɔ̝lə");
   let str109 = &str108.replace("youre", "j|ʉə");
   let str110 = &str109.replace("your", "j|ʉː");
   let str111 = &str110.replace("ourse", "oːs̺");
   let str112 = &str111.replace("our", "æɔə");
   let str113 = &str112.replace("easure", "e̞ːʒə");
   let str114 = &str113.replace("eisure", "ɜʒə");
   let str115 = &str114.replace("sure", "ʃʲɔˑ");
   let str116 = &str115.replace("cture", "ct͡ʃə");
   let str117 = &str116.replace("ure", "ʲʉːə");
   let str118 = &str117.replace("ounce", "æɔ̞̃nce");
   let str119 = &str118.replace("ounci", "æɔ̞̃ns̺i");
   let str120 = &str119.replace("ound", "æɔ̞̃nd̥");
   let str121 = &str120.replace("rs", "ɹz");
   let str122 = &str121.replace("ceipt", "ceit");
   let str123 = &str122.replace("eir", "iˑ");
   let str124 = &str123.replace("ei", "iˑ");
   let str125 = &str124.replace("ow", "ɞ̜ʉ̟");
   let str126 = &str125.replace("wea", "we̞ˑ");
   let str127 = &str126.replace("eady", "e̞dy");
   let str128 = &str127.replace("eason", "ᵊiːs̬ᵊn");
   let str129 = &str128.replace("ear ", "ɪ̝ə ");
   let str130 = &str129.replace("ear", "e̞ː");
   let str131 = &str130.replace("eali", "ᵊiːəli");
   let str132 = &str131.replace("ea", "ᵊiː");
   let str133 = &str132.replace("aunt", "ɑːnt");
   let str134 = &str133.replace("daughter", "dɒtə");
   let str135 = &str134.replace("au", "ɔ̝");
   let str136 = &str135.replace("archi", "ɐːkʰi");
   let str137 = &str136.replace("are", "ɑ");
   let str138 = &str137.replace("arm", "õːm");   
   let str139 = &str138.replace("arn", "õːn");   
   let str140 = &str139.replace("arr", "əɹ");
   let str141 = &str140.replace("are ", "ɑː");
   let str142 = &str141.replace("eyre", "e̞ː");
   let str143 = &str142.replace("re ", "ə ");
   let str144 = &str143.replace(" ar", " əɹ");
   let str145 = &str144.replace("liar", "lɐɪ̝ə");
   let str146 = &str145.replace("ar a", "ɐːɹ a");
   let str147 = &str146.replace("ar e", "ɐːɹ e");
   let str148 = &str147.replace("ar o", "ɐːɹ o");
   let str149 = &str148.replace("ar", "ɐː");
   let str150 = &str149.replace("ust", "ɐs̺t");
   let str151 = &str150.replace("tri", "t͡ʃɹɪ̝");
   let str152 = &str151.replace("str", "sṯɹ̥");
   let str153 = &str152.replace("spr", "spɹ");
   let str154 = &str153.replace("pr", "pɹ̥");
   let str155 = &str154.replace("outh", "œɔθ");
   let str156 = &str155.replace("out", "æɔt");
   let str157 = &str156.replace("obi", "ɞ̜ʉ̟bi");
   let str158 = &str157.replace("ick", "ɪ̝ck");
   let str159 = &str158.replace("eel", "iːᵊɫ");
   let str160 = &str159.replace("ee", "ᵊiː");
   let str161 = &str160.replace("cially", "ʃl̥ᵊiː");
   let str162 = &str161.replace("dia", "dɐɪ̝");
   let str163 = &str162.replace("ia", "iːə");
   let str164 = &str163.replace("ile", "ɑɪ̝ɫe");
   let str165 = &str164.replace("ance", "ænse");
   let str166 = &str165.replace("anci", "ænsi");
   let str167 = &str166.replace("astle", "æsᵊl");
   let str168 = &str167.replace("stle", "sᵊl");
   let str169 = &str168.replace("trast", "træst");
   let str170 = &str169.replace("and", "æ̃n");
   let str171 = &str170.replace("vel", "vɫ");
   let str172 = &str171.replace("behav", "bɪ̝hæi̞v");
   let str173 = &str172.replace("hav", "hæv");
   let str174 = &str173.replace("ave", "æi̞v");
   let str175 = &str174.replace("avi", "æi̞vi");
   let str176 = &str175.replace("ame", "æĩ̞m");
   let str177 = &str176.replace("ami", "æĩ̞mi");
   let str178 = &str177.replace("ache", "æi̞k̥ʰ");
   let str179 = &str178.replace("ake", "æi̞k̥e");
   let str180 = &str179.replace("aki", "æi̞k̥i");
   let str181 = &str180.replace("ade", "æi̞d̥");
   let str182 = &str181.replace("adi", "æi̞d̥i");
   let str183 = &str182.replace("ase", "æi̞se");
   let str184 = &str183.replace("asi", "æi̞si");
   let str185 = &str184.replace("ice", "æi̞s̺");
   let str186 = &str185.replace("pɹ̥ove", "pɹ̥ʉːve");
   let str187 = &str186.replace("pɹ̥ovid", "pɹ̥əvid");
   let str188 = &str187.replace("pɹ̥ovi", "pɹ̥ʉːvi");
   let str189 = &str188.replace("ide", "æi̞d̥e");
   let str190 = &str189.replace("idi", "æi̞d̥i");
   let str191 = &str190.replace("water", "wɒðə");
   let str192 = &str191.replace("ate", "æi̞te");
   let str193 = &str192.replace("ati", "æi̞ti");
   let str194 = &str193.replace("ssage", "sɪ̝d͡ʒ");
   let str195 = &str194.replace("uage", "wɪ̝d͡ʒ");
   let str196 = &str195.replace("age", "æi̞d͡ʒe");
   let str197 = &str196.replace("agi", "æi̞d͡ʒi");
   let str198 = &str197.replace("ale", "æi̞le");
   let str199 = &str198.replace("ike", "æi̞ke");
   let str200 = &str199.replace("iki", "æi̞ki");
   let str201 = &str200.replace("ime", "æi̞me");
   let str202 = &str201.replace("machine", "məʃin");
   let str203 = &str202.replace("ine", "æi̞ne");
   let str204 = &str203.replace("ina", "æi̞na");
   let str205 = &str204.replace("ini", "æi̞ni");
   let str206 = &str205.replace("ite", "æi̞te");
   let str207 = &str206.replace("iti", "æi̞ti");
   let str208 = &str207.replace("ita", "æi̞ta");
   let str209 = &str208.replace("æi̞tc", "ite̞c");
   let str210 = &str209.replace("ise", "æi̞s̺e");
   let str211 = &str210.replace("isi", "æi̞s̺i");
   let str212 = &str211.replace("ize", "æi̞s̺e");
   let str213 = &str212.replace("izi", "æi̞s̺i");
   let str214 = &str213.replace("ype", "æi̞pe");
   let str215 = &str214.replace("ypi", "æi̞pi");
   let str216 = &str215.replace("ay", "æi̞");
   let str217 = &str216.replace("ey", "æi̞");
   let str218 = &str217.replace("aw", "ɔː");
   let str219 = &str218.replace("aste", "æi̞s̺t");
   let str220 = &str219.replace("asthma", "æsmə");
   let str221 = &str220.replace("ast", "ɐːs̺t");
   let str222 = &str221.replace("etable", "tbɫ");
   let str223 = &str222.replace("ortable", "tbɫ");
   let str224 = &str223.replace("shal", "ʃʲæl");
   let str225 = &str224.replace("gra", "ɡɹ̥æ");
   let str226 = &str225.replace("sylla", "sɪ̝lə");
   let str227 = &str226.replace("able", "æi̞bɫ");
   let str228 = &str227.replace("cæi̞", "kʰæi̞");
   let str229 = &str228.replace("cæĩ̞", "kʰæĩ̞");
   let str230 = &str229.replace("tæi̞", "tʰæi̞");
   let str231 = &str230.replace("tæĩ̞", "tʰæĩ̞");
   let str232 = &str231.replace("imme", "ə̃mᵊiː");
   let str233 = &str232.replace("obli", "əblɑe");
   let str234 = &str233.replace("ob", "əb");
   let str235 = &str234.replace("oubl", "ɐbl");
   let str236 = &str235.replace("ble ", "bɫ ");
   let str237 = &str236.replace("ble.", "bɫ.");
   let str238 = &str237.replace("ble,", "bɫ,");
   let str239 = &str238.replace("ble;", "bɫ;");
   let str240 = &str239.replace("ble:", "bɫ:");
   let str241 = &str240.replace("ble?", "bɫ?");
   let str242 = &str241.replace("ble!", "bɫ!");
   let str243 = &str242.replace("ble-", "bɫ-");
   let str244 = &str243.replace("ble)", "bɫ)");
   let str245 = &str244.replace("igh", "ɐɪ̝");
   let str246 = &str245.replace("die", "d̥ɐɪ̝");
   let str247 = &str246.replace("lie", "lɐɪ̝");
   let str248 = &str247.replace("tie", "tɐɪ̝");
   let str249 = &str248.replace("pie", "pɐɪ̝");
   let str250 = &str249.replace("iet", "ɐɪ̝ət");
   let str251 = &str250.replace("by", "bɐɪ̝");
   let str252 = &str251.replace("my", "mɐɪ̝");
   let str253 = &str252.replace("cry", "kɹ̥ɐɪ̝");
   let str254 = &str253.replace("dry", "dɹ̥ɐɪ̝");
   let str255 = &str254.replace("fly", "flɐɪ̝");
   let str256 = &str255.replace("shy", "ʃɐɪ̝");
   let str257 = &str256.replace("sky", "s̺kɐɪ̝");
   let str258 = &str257.replace("try", "trɐɪ̝");
   let str259 = &str258.replace("why", "wɐɪ̝");
   let str260 = &str259.replace("iche", "iːʃ");
   let str261 = &str260.replace("uel", "ʲʉːᵊl");
   let str262 = &str261.replace("oa", "əʉ");
   let str263 = &str262.replace("cəʉ", "kʰəʉ");
   let str264 = &str263.replace("əʉl", "ɔoɫ");
   let str265 = &str264.replace("ole", "ɔoɫ");
   let str266 = &str265.replace("ool", "uːɫ");
   let str267 = &str266.replace("oll", "ɔɫ");
   let str268 = &str267.replace("old", "ɔoʊd");
   let str269 = &str268.replace("dont", "dɞ̜ʉ̟nt̚ ");
   let str270 = &str269.replace("ond", "ənd̥");
   let str271 = &str270.replace("oes", "ɐs");
   let str272 = &str271.replace("cous", "kʰɐs");
   let str273 = &str272.replace("touch", "tɐt͡ʃ");
   let str274 = &str273.replace("ouch", "æɔt͡ʃ");
   let str275 = &str274.replace("oubt", "æɔt");
   let str276 = &str275.replace("ought", "ɔt");
   let str277 = &str276.replace("ough", "ɐ̟f̟");
   let str278 = &str277.replace("could", "kʰʊd");
   let str279 = &str278.replace("ouse", "æɔse");
   let str280 = &str279.replace("ou", "ʉ");
   let str281 = &str280.replace("ooe", "ʉˑ");
   let str282 = &str281.replace("ood", "ɐd");
   let str283 = &str282.replace("oo", "ʊ");
   let str284 = &str283.replace("ose", "ɘʉs̺");
   let str285 = &str284.replace("use", "ʲʉːz");
   let str286 = &str285.replace("usi", "ʲʉːzi");
   let str287 = &str286.replace("actu", "ækʃʉ");
   let str288 = &str287.replace("act", "ækt");
   let str289 = &str288.replace("alf", "ɐːf");
   let str290 = &str289.replace("alk", "ɒːk");
   let str291 = &str290.replace("alm", "ɐːm");
   let str292 = &str291.replace("eith", "iːð");
   let str293 = &str292.replace("urth", "e̞ːð");
   let str294 = &str293.replace("ura", "ʲʉra");
   let str295 = &str294.replace("ewe", "ʲʉːe");
   let str296 = &str295.replace("ew", "ʲʉː");
   let str297 = &str296.replace("une", "ʲʉ̃ːn");
   let str298 = &str297.replace("uni", "ʲʉ̃ːni");
   let str299 = &str298.replace("tʲʉ̃ːn", "t͡ʃʉ̃ːn");
   let str300 = &str299.replace("dʲʉ̃ːn", "d͡ʒʉ̃ːn");
   let str301 = &str300.replace("ume", "ʲʉ̃ːm");
   let str302 = &str301.replace("umi", "ʲʉ̃ːmi");
   let str303 = &str302.replace("assʲʉ̃ːm", "əʃʉ̃ːm");
   let str304 = &str303.replace("upi", "ʲʉːpi");
   let str305 = &str304.replace("ude", "ʲʉde");
   let str306 = &str305.replace("ute", "ʲʉːte");
   let str307 = &str306.replace("uti", "ʲʉːti");
   let str308 = &str307.replace("move", "mʉ̟v");
   let str309 = &str308.replace("movi", "mʉ̟vi");
   let str310 = &str309.replace("love", "lʌ̹ve");
   let str311 = &str310.replace("lovi", "lʌ̹vi");
   let str312 = &str311.replace("to dove", "to dɞ̜ʉ̟v");
   let str313 = &str312.replace("dove", "dʌ̹v");
   let str314 = &str313.replace("ove", "ɞ̜ʉ̟ve");
   let str315 = &str314.replace("uck", "ɐk");
   let str316 = &str315.replace("some", "s̺ʌ̹m");
   let str317 = &str316.replace("ome", "ɞ̜ʉ̟m");
   let str318 = &str317.replace("opef", "ɞ̜ʉ̟pf");
   let str319 = &str318.replace("opel", "ɞ̜ʉ̟pl");
   let str320 = &str319.replace("ope", "ɞ̜ʉ̟pe");
   let str321 = &str320.replace("opi", "ɞ̜ʉ̟pi");
   let str322 = &str321.replace("ode", "ɞ̜ʉ̟d̥e");
   let str323 = &str322.replace("ote", "ɞ̜ʉ̟t");
   let str324 = &str323.replace("oti", "ɞ̜ʉ̟ti");
   let str325 = &str324.replace("oto", "ɞ̜ʉ̟to");
   let str326 = &str325.replace("uild", "ɪ̝ld");
   let str327 = &str326.replace("uilt", "ɪ̝lt");
   let str328 = &str327.replace("uin", "uːin");
   let str329 = &str328.replace("cuit", "kʰɪ̝t");
   let str330 = &str329.replace("guit", "gɪ̝t");
   let str331 = &str330.replace("uit", "uːt");
   let str332 = &str331.replace("off", "ɔ̝f");
   let str333 = &str332.replace("of", "ɔ̝v");
   let str334 = &str333.replace("sson", "s̺ᵊn");
   let str335 = &str334.replace("con", "kə̃n");
   let str336 = &str335.replace("con", "kə̃n");
   let str337 = &str336.replace("gone", "gɔ̃n");
   let str338 = &str337.replace("phon", "fɔ̃n");
   let str339 = &str338.replace("ton", "tɔ̃n");
   let str340 = &str339.replace("zon", "zɔ̃n");
   let str341 = &str340.replace("one", "wɐ̃n");
   let str342 = &str341.replace("engl", "ɪ̝ngl");
   let str343 = &str342.replace("ng", "ŋ");
   let str344 = &str343.replace("aŋ", "æŋ");
   let str345 = &str344.replace("tiŋ", "ɾɪ̝̃ŋ");
   let str346 = &str345.replace("iŋ", "ɪ̝̃ŋ");
   let str347 = &str346.replace("oŋ", "ɔ̝̃ŋ");
   let str348 = &str347.replace("ent ", "ənt ");
   let str349 = &str348.replace("ent.", "ənt.");
   let str350 = &str349.replace("ent,", "ənt,");
   let str351 = &str350.replace("ent;", "ənt;");
   let str352 = &str351.replace("ent?", "ənt?");
   let str353 = &str352.replace("ent!", "ənt!");
   let str354 = &str353.replace("ent-", "ənt-");
   let str355 = &str354.replace("ent)", "ənt)");
   let str356 = &str355.replace("ent:", "ənt:");
   let str357 = &str356.replace("ene", "iːn");
   let str358 = &str357.replace("en", "ẽ̞n̪");
   let str359 = &str358.replace("in", "ɪ̝̃n");
   let str360 = &str359.replace("im", "ɪ̝̃m");
   let str361 = &str360.replace("un", "ɐ̃n");
   let str362 = &str361.replace("up", "ɐp");
   let str363 = &str362.replace("but ", "bət ");
   let str364 = &str363.replace("but,", "bət,");
   let str365 = &str364.replace("ak", "æk");
   let str366 = &str365.replace("ik", "ɪ̝k");
   let str367 = &str366.replace("id", "ɪ̝d");
   let str368 = &str367.replace("this", "ðɪ̝s̺");
   let str369 = &str368.replace("is ", "ɪ̝z̥ ");
   let str370 = &str369.replace("is.", "ɪ̝z̥.");
   let str371 = &str370.replace("is,", "ɪ̝z̥,");
   let str372 = &str371.replace("is", "ɪ̝s");
   let str373 = &str372.replace("it ", "ɪ̝t ");
   let str374 = &str373.replace("it.", "ɪ̝t.");
   let str375 = &str374.replace("it,", "ɪ̝t,");
   let str376 = &str375.replace("it;", "ɪ̝t;");
   let str377 = &str376.replace("it?", "ɪ̝t?");
   let str378 = &str377.replace("it!", "ɪ̝t!");
   let str379 = &str378.replace("it)", "ɪ̝t)");
   let str380 = &str379.replace("it-", "ɪ̝t-");
   let str381 = &str380.replace("it:", "ɪ̝t:");
   let str382 = &str381.replace("ẽ̞n̪th", "ẽn̪θ");
   let str383 = &str382.replace("othes", "ɞ̜ʉ̟ðz");
   let str384 = &str383.replace("oth", "ɐð");
   let str385 = &str384.replace("uth", "ʉθ");
   let str386 = &str385.replace("ythm", "ɪðᵊm");
   let str387 = &str386.replace("yme", "ɐɪ̝me");
   let str388 = &str387.replace("orth", "oːθ");
   let str389 = &str388.replace("that", "ðət̪");
   let str390 = &str389.replace("than", "ðə̃n̪");
   let str391 = &str390.replace("tha", "ðæ");
   let str392 = &str391.replace("thẽn̪", "ðẽn̪");
   let str393 = &str392.replace("these", "ðiːz");
   let str394 = &str393.replace("the ", "ðə ");
   let str395 = &str394.replace("thi", "θɪ̝");
   let str396 = &str395.replace("th", "ð");
   let str397 = &str396.replace("two", "tʰʉː");
   let str398 = &str397.replace("pt", "p̚t");
   let str399 = &str398.replace("spu", "s̺pʲʉː");
   let str400 = &str399.replace("pu", "pʰu");
   let str401 = &str400.replace("pʊ", "pʰʊ");
   let str402 = &str401.replace("tʊ", "tʰʊ");
   let str403 = &str402.replace("dge", "d͡ʒe");
   let str404 = &str403.replace("cant", "k̟ɑːnt");
   let str405 = &str404.replace("can", "k̟ʰæ̃ːn");
   let str406 = &str405.replace("ken", "k̟ʰɛ̃ːn");
   let str407 = &str406.replace("who", "hʉː");
   let str408 = &str407.replace("wh", "w");
   let str409 = &str408.replace("we ", "wi̞ː ");
   let str410 = &str409.replace("we.", "wi̞ː.");
   let str411 = &str410.replace("we,", "wi̞ː,");
   let str412 = &str411.replace("we;", "wi̞ː;");
   let str413 = &str412.replace("we:", "wi̞ː:");
   let str414 = &str413.replace("we?", "wi̞ː?");
   let str415 = &str414.replace("we!", "wi̞ː!");
   let str416 = &str415.replace("we-", "wi̞ː-");
   let str417 = &str416.replace("we)", "wi̞ː)");
   let str418 = &str417.replace("she ", "ʃi̞ː ");
   let str419 = &str418.replace("she.", "ʃi̞ː.");
   let str420 = &str419.replace("she,", "ʃi̞ː,");
   let str421 = &str420.replace("she;", "ʃi̞ː;");
   let str422 = &str421.replace("she:", "ʃi̞ː:");
   let str423 = &str422.replace("she?", "ʃi̞ː?");
   let str424 = &str423.replace("she!", "ʃi̞ː!");
   let str425 = &str424.replace("she-", "ʃi̞ː-");
   let str426 = &str425.replace("she)", "ʃi̞ː)");
   let str427 = &str426.replace("shes ", "ʃi̞ːz ");
   let str428 = &str427.replace("he ", "hi̞ː ");
   let str429 = &str428.replace("he.", "hi̞ː.");
   let str430 = &str429.replace("he,", "hi̞ː,");
   let str431 = &str430.replace("he;", "hi̞ː;");
   let str432 = &str431.replace("he:", "hi̞ː:");
   let str433 = &str432.replace("he?", "hi̞ː?");
   let str434 = &str433.replace("he!", "hi̞ː!");
   let str435 = &str434.replace("he-", "hi̞ː-");
   let str436 = &str435.replace("he)", "hi̞ː)");
   let str437 = &str436.replace("hes ", "hi̞ːz ");
   let str438 = &str437.replace("sh", "ʃ");
   let str439 = &str438.replace("sto", "stɔ");
   let str440 = &str439.replace("nf", "ɱf");
   let str441 = &str440.replace("oɱ", "ə̃ɱ");
   let str442 = &str441.replace("om", "ə̃m");
   let str443 = &str442.replace("bad", "bæd");
   let str444 = &str443.replace("hab", "hæb");
   let str445 = &str444.replace("had", "hæd");
   let str446 = &str445.replace("suk", "sək");
   let str447 = &str446.replace("urg", "ɜːg");
   let str448 = &str447.replace("app", "æp");
   let str449 = &str448.replace("bit", "bət");
   let str450 = &str449.replace("rot", "ɹət");
   let str451 = &str450.replace("ot", "ɔ̝t");
   let str452 = &str451.replace("ert", "e̞ˑt");
   let str453 = &str452.replace("ere ", "i̞ə ");
   let str454 = &str453.replace("ere.", "i̞ə.");
   let str455 = &str454.replace("ere,", "i̞ə,");
   let str456 = &str455.replace("ere;", "i̞ə;");
   let str457 = &str456.replace("ere?", "i̞ə?");
   let str458 = &str457.replace("ere!", "i̞ə!");
   let str459 = &str458.replace("ere-", "i̞ə-");
   let str460 = &str459.replace("ere)", "i̞ə)");
   let str461 = &str460.replace("ere:", "i̞ə:");
   let str462 = &str461.replace("mber", "mer");
   let str463 = &str462.replace("er a", "əɹ a");
   let str464 = &str463.replace("er e", "əɹ e");
   let str465 = &str464.replace("er o", "əɹ o");
   let str466 = &str465.replace("er ", "ə ");
   let str467 = &str466.replace("er.", "ə.");
   let str468 = &str467.replace("er,", "ə,");
   let str469 = &str468.replace("er;", "ə;");
   let str470 = &str469.replace("er:", "ə:");
   let str471 = &str470.replace("er?", "ə?");
   let str472 = &str471.replace("er!", "ə!");
   let str473 = &str472.replace("er-", "ə-");
   let str474 = &str473.replace("er)", "ə)");
   let str475 = &str474.replace("ses", "zəz̥");
   let str476 = &str475.replace("sas", "zəz̥");
   let str477 = &str476.replace("as ", "əz̥ ");
   let str478 = &str477.replace("as.", "əz̥.");
   let str479 = &str478.replace("as,", "əz̥,");
   let str480 = &str479.replace("as;", "əz̥;");
   let str481 = &str480.replace("as?", "əz̥?");
   let str482 = &str481.replace("as!", "əz̥!");
   let str483 = &str482.replace("as-", "əz̥-");
   let str484 = &str483.replace("as)", "əz̥)");
   let str485 = &str484.replace("as:", "əz̥:");
   let str486 = &str485.replace("ped ", "p̚t ");
   let str487 = &str486.replace("ped.", "p̚t.");
   let str488 = &str487.replace("ped,", "p̚t,");
   let str489 = &str488.replace("ped;", "p̚t;");
   let str490 = &str489.replace("ped?", "p̚t?");
   let str491 = &str490.replace("ped!", "p̚t!");
   let str492 = &str491.replace("ped-", "p̚t-");
   let str493 = &str492.replace("ped)", "p̚t)");
   let str494 = &str493.replace("ped:", "p̚t:");
   let str495 = &str494.replace("ged", "d͡ʒ");
   let str496 = &str495.replace("ked", "kt");
   let str497 = &str496.replace("k̥ed", "kt");
   let str498 = &str497.replace("ed ", "əd̥ ");
   let str499 = &str498.replace("ed.", "əd̥.");
   let str500 = &str499.replace("ed,", "əd̥,");
   let str501 = &str500.replace("ed;", "əd̥;");
   let str502 = &str501.replace("ed?", "əd̥?");
   let str503 = &str502.replace("ed!", "əd̥!");
   let str504 = &str503.replace("ed-", "əd̥-");
   let str505 = &str504.replace("ed)", "əd̥)");
   let str506 = &str505.replace("ed:", "əd̥:");
   let str507 = &str506.replace("bəd̥", "be̞d̥");
   let str508 = &str507.replace("bad̥", "bæd̥");
   let str509 = &str508.replace("be ", "bɪ̝ ");
   let str510 = &str509.replace("be.", "bɪ̝.");
   let str511 = &str510.replace("be,", "bɪ̝,");
   let str512 = &str511.replace("be;", "bɪ̝;");
   let str513 = &str512.replace("be?", "bɪ̝?");
   let str514 = &str513.replace("be!", "bɪ̝!");
   let str515 = &str514.replace("be-", "bɪ̝-");
   let str516 = &str515.replace("be)", "bɪ̝)");
   let str517 = &str516.replace("be:", "bɪ̝:");
   let str518 = &str517.replace(" be", " bɪ̝");
   let str519 = &str518.replace(" me ", " mɪ̝ ");
   let str520 = &str519.replace(" me.", " mɪ̝.");
   let str521 = &str520.replace(" me,", " mɪ̝,");
   let str522 = &str521.replace(" me;", " mɪ̝;");
   let str523 = &str522.replace(" me?", " mɪ̝?");
   let str524 = &str523.replace(" me!", " mɪ̝!");
   let str525 = &str524.replace(" me-", " mɪ̝-");
   let str526 = &str525.replace(" me)", " mɪ̝)");
   let str527 = &str526.replace(" me:", " mɪ̝:");
   let str528 = &str527.replace("nal", "nᵊl");
   let str529 = &str528.replace("ral", "ɹᵊl");
   let str530 = &str529.replace(" all ", " ɒl ");
   let str531 = &str530.replace(" all", " əl");
   let str532 = &str531.replace(" al", " ɒl");
   let str533 = &str532.replace(" re", " ɹɪ̝");
   let str534 = &str533.replace("ation", "æi̞ʃŋ");
   let str535 = &str534.replace("tion", "ʃŋ");
   let str536 = &str535.replace("sion", "ʃŋ");
   let str537 = &str536.replace("zion", "ʒᵊŋ");
   let str538 = &str537.replace("cky", "kiː");
   let str539 = &str538.replace("ctly", "t͡ʃl̥ᵊiː");
   let str540 = &str539.replace("tely", "ʔl̥ᵊiː");
   let str541 = &str540.replace("llery", "l̥ɹᵊiː");
   let str542 = &str541.replace("ly", "l̥ᵊiː");
   let str543 = &str542.replace("ical", "ɪ̝kᵊl");
   let str544 = &str543.replace("ic ", "ɪ̝kʰ ");
   let str545 = &str544.replace("ic.", "ɪ̝kʰ.");
   let str546 = &str545.replace("ic,", "ɪ̝kʰ,");
   let str547 = &str546.replace("ic!", "ɪ̝k!ʰ");
   let str548 = &str547.replace("ic?", "ɪ̝kʰ?");
   let str549 = &str548.replace("ic-", "ɪ̝kʰ-");
   let str550 = &str549.replace("ic)", "ɪ̝kʰ)");
   let str551 = &str550.replace("ic;", "ɪ̝kʰ;");
   let str552 = &str551.replace("ic:", "ɪ̝kʰ:");
   let str553 = &str552.replace("ce ", "s̺ ");
   let str554 = &str553.replace("ce.", "s̺.");
   let str555 = &str554.replace("ce,", "s̺,");
   let str556 = &str555.replace("ce;", "s̺;");
   let str557 = &str556.replace("ce:", "s̺:");
   let str558 = &str557.replace("ce?", "s̺?");
   let str559 = &str558.replace("ce!", "s̺!");
   let str560 = &str559.replace("ce-", "s̺-");
   let str561 = &str560.replace("ce)", "s̺)");
   let str562 = &str561.replace("ive ", "ɪ̝v ");
   let str563 = &str562.replace("ive.", "ɪ̝v.");
   let str564 = &str563.replace("ive,", "ɪ̝v,");
   let str565 = &str564.replace("ive;", "ɪ̝v;");
   let str566 = &str565.replace("ive:", "ɪ̝v:");
   let str567 = &str566.replace("ive?", "ɪ̝v?");
   let str568 = &str567.replace("ive!", "ɪ̝v!");
   let str569 = &str568.replace("ive-", "ɪ̝v-");
   let str570 = &str569.replace("ive)", "ɪ̝v)");
   let str571 = &str570.replace("e ", " ");
   let str572 = &str571.replace("e.", ".");
   let str573 = &str572.replace("e,", ",");
   let str574 = &str573.replace("e;", ";");
   let str575 = &str574.replace("e:", ":");
   let str576 = &str575.replace("e?", "?");
   let str577 = &str576.replace("e!", "!");
   let str578 = &str577.replace("e-", "-");
   let str579 = &str578.replace("e)", ")");
   let str580 = &str579.replace(" do ", " dʊ ");
   let str581 = &str580.replace(" do.", " dʊ.");
   let str582 = &str581.replace(" do,", " dʊ,");
   let str583 = &str582.replace(" do;", " dʊ;");
   let str584 = &str583.replace(" do:", " dʊ:");
   let str585 = &str584.replace(" do?", " dʊ?");
   let str586 = &str585.replace(" do!", " dʊ!");
   let str587 = &str586.replace(" do-", " dʊ-");
   let str588 = &str587.replace(" do)", " dʊ)");
   let str589 = &str588.replace("to", "tə");
   let str590 = &str589.replace("o ", "əʉ ");
   let str591 = &str590.replace("o.", "əʉ.");
   let str592 = &str591.replace("o,", "əʉ,");
   let str593 = &str592.replace("o;", "əʉ;");
   let str594 = &str593.replace("o:", "əʉ:");
   let str595 = &str594.replace("o?", "əʉ?");
   let str596 = &str595.replace("o!", "əʉ!");
   let str597 = &str596.replace("o-", "əʉ-");
   let str598 = &str597.replace("o)", "əʉ)");
   let str599 = &str598.replace("cy ", "s̺iː ");
   let str600 = &str599.replace("cy.", "s̺iː.");
   let str601 = &str600.replace("cy,", "s̺iː,");
   let str602 = &str601.replace("cy;", "s̺iː;");
   let str603 = &str602.replace("cy:", "s̺iː:");
   let str604 = &str603.replace("cy?", "s̺iː?");
   let str605 = &str604.replace("cy!", "s̺iː!");
   let str606 = &str605.replace("cy-", "s̺iː-");
   let str607 = &str606.replace("cy)", "s̺iː)");
   let str608 = &str607.replace("logy", "ləd͡ʒiː");
   let str609 = &str608.replace("y ", "iː ");
   let str610 = &str609.replace("y.", "iː.");
   let str611 = &str610.replace("y,", "iː,");
   let str612 = &str611.replace("y?", "iː?");
   let str613 = &str612.replace("y!", "iː!");
   let str614 = &str613.replace("y)", "iː)");
   let str615 = &str614.replace("y:", "iː:");
   let str616 = &str615.replace("y-", "iː-");
   let str617 = &str616.replace("ci", "si");
   let str618 = &str617.replace("diff", "dɪ̝f");
   let str619 = &str618.replace("mach", "məkʰ");
   let str620 = &str619.replace("tch", "t͡ʃ");
   let str621 = &str620.replace("ch ", "t͡ʃ ");
   let str622 = &str621.replace("ch.", "t͡ʃ.");
   let str623 = &str622.replace("ch,", "t͡ʃ,");
   let str624 = &str623.replace("ch;", "t͡ʃ;");
   let str625 = &str624.replace("ch:", "t͡ʃ:");
   let str626 = &str625.replace("ch?", "t͡ʃ?");
   let str627 = &str626.replace("ch!", "t͡ʃ!");
   let str628 = &str627.replace("ch-", "t͡ʃ-");
   let str629 = &str628.replace("ch)", "t͡ʃ)");
   let str630 = &str629.replace("chæ", "t͡ʃæ");
   let str631 = &str630.replace("chʊ", "t͡ʃʊ");
   let str632 = &str631.replace("ch", "k");
   let str633 = &str632.replace("cc", "k");
   let str634 = &str633.replace("ck", "k");
   let str635 = &str634.replace("c", "k");
   let str636 = &str635.replace("kæ", "kʰæ");
   let str637 = &str636.replace("kl", "kl̥");
   let str638 = &str637.replace("gr", "ɡɹ̥");
   let str639 = &str638.replace("tr", "ṯɹ̥");
   let str640 = &str639.replace("ut", "ɐt");
   let str641 = &str640.replace("ud", "ɐd");
   let str642 = &str641.replace("ʊk", "ʊkʰ");
   let str643 = &str642.replace("æk", "ækʰ");
   let str644 = &str643.replace("r", "ɹ");
   let str645 = &str644.replace("ck", "k");
   let str646 = &str645.replace("dɹ", "d͡ʒɹ̥");
   let str647 = &str646.replace("dst", "d.st");
   let str648 = &str647.replace("ds", "d͡z");
   let str649 = &str648.replace("d̥s", "d͡z");
   let str650 = &str649.replace("ns ", "nz̥ ");
   let str651 = &str650.replace("ns.", "nz̥.");
   let str652 = &str651.replace("ns,", "nz̥,");
   let str653 = &str652.replace("ns;", "nz̥;");
   let str654 = &str653.replace("ns:", "nz̥:");
   let str655 = &str654.replace("ns?", "nz̥?");
   let str656 = &str655.replace("ns!", "nz̥!");
   let str657 = &str656.replace("ns-", "nz̥-");
   let str658 = &str657.replace("ns)", "nz̥)");
   let str659 = &str658.replace("ŋs", "ŋs̬");
   let str660 = &str659.replace("æi̞s", "æi̞z̥");
   let str661 = &str660.replace("x", "ks");
   let str662 = &str661.replace("ss", "s̺");
   let str663 = &str662.replace("s", "s̺");
   let str664 = &str663.replace("i ", "ɐɪ̝ ");
   let str665 = &str664.replace("ph", "f");
   let str666 = &str665.replace("ff", "f");
   let str667 = &str666.replace("f", "f̟");
   let str668 = &str667.replace("pp", "p");
   let str669 = &str668.replace("pf", "f");
   let str670 = &str669.replace("mm", "m");
   let str671 = &str670.replace("mb ", "m ");
   let str672 = &str671.replace("mb.", "m.");
   let str673 = &str672.replace("mb,", "m,");
   let str674 = &str673.replace("mb;", "m;");
   let str675 = &str674.replace("mb:", "m:");
   let str676 = &str675.replace("mb?", "m?");
   let str677 = &str676.replace("mb!", "m!");
   let str678 = &str677.replace("mb-", "m-");
   let str679 = &str678.replace("mb)", "m)");
   let str680 = &str679.replace("damn", "dæm");
   let str681 = &str680.replace("amn ", "æm ");
   let str682 = &str681.replace("amn.", "æm.");
   let str683 = &str682.replace("amn,", "æm,");
   let str684 = &str683.replace("amn;", "æm;");
   let str685 = &str684.replace("amn:", "æm:");
   let str686 = &str685.replace("amn?", "æm?");
   let str687 = &str686.replace("amn!", "æm!");
   let str688 = &str687.replace("amn-", "æm-");
   let str689 = &str688.replace("amn)", "æm)");
   let str690 = &str689.replace("umn", "ᵊm");
   let str691 = &str690.replace("ymn", "i̞m");
   let str692 = &str691.replace("nn", "n");
   let str693 = &str692.replace("s̺nt", "zᵊnt̚");
   let str694 = &str693.replace("nt ", "nt̚  ");
   let str695 = &str694.replace("nt.", "nt̚ .");
   let str696 = &str695.replace("nt,", "nt̚ ,");
   let str697 = &str696.replace("nt;", "nt̚ ;");
   let str698 = &str697.replace("nt:", "nt̚ :");
   let str699 = &str698.replace("nt?", "nt̚ ?");
   let str700 = &str699.replace("nt!", "nt̚ !");
   let str701 = &str700.replace("nt-", "nt̚ -");
   let str702 = &str701.replace("nt)", "nt̚ )");
   let str703 = &str702.replace("j", "d͡ʒ");
   let str704 = &str703.replace("d͡ʒ|", "j");
   let str705 = &str704.replace("y", "j");
   let str706 = &str705.replace("ai", "eɪ̝");
   let str707 = &str706.replace("a", "ə");
   let str708 = &str707.replace("e-", "iː");
   let str709 = &str708.replace("e", "e̞");
   let str710 = &str709.replace("ət", "ət̪");
   let str711 = &str710.replace("tt", "ð");
   let str712 = &str711.replace("rr", "ɹ");
   let str713 = &str712.replace("r", "ɹ");
   let str714 = &str713.replace("ll", "l");
   let str715 = &str714.replace("l ", "ɫ ");
   let str716 = &str715.replace("l.", "ɫ.");
   let str717 = &str716.replace("l,", "ɫ,");
   let str718 = &str717.replace("l;", "ɫ;");
   let str719 = &str718.replace("l?", "ɫ?");
   let str720 = &str719.replace("l!", "ɫ!");
   let str721 = &str720.replace("l-", "ɫ-");
   let str722 = &str721.replace("l)", "ɫ)");
   let str723 = &str722.replace("l:", "ɫ:");
   let str724 = &str723.replace("all", "ɒl");
   let str725 = &str724.replace(" al", " əl");
   let str726 = &str725.replace("ʉ̟ld", "ʉ̟d̥");
   let str727 = &str726.replace("ld", "l̥d");
   let str728 = &str727.replace("t̪ ð", "t̪̚ ð");
   let str729 = &str728.replace("d ð", "d̪̚ ð");
   let str730 = &str729.replace("d ", "d̥ ");
   let str731 = &str730.replace("d.", "d̥.");
   let str732 = &str731.replace("d,", "d̥,");
   let str733 = &str732.replace("d;", "d̥;");
   let str734 = &str733.replace("d:", "d̥:");
   let str735 = &str734.replace("d?", "d̥?");
   let str736 = &str735.replace("d!", "d̥!");
   let str737 = &str736.replace("d-", "d̥-");
   let str738 = &str737.replace("d)", "d̥)");
   let str739 = &str738.replace("ðə ə", "ðiː ə");
   let str740 = &str739.replace("ðə ɐ", "ðiː ɐ");
   let str741 = &str740.replace("ɪ̝ ɪ̝", "ɪ̝");
   let str742 = &str741.replace("n m", "m m");
   let str743 = &str742.replace("n n", " n");
   let str744 = &str743.replace("g", "ɡ");
   let str745 = &str744.replace(",", " ∣");
   let str746 = &str745.replace(";", " ∥");
   let str747 = &str746.replace(":", " ∣");
   let str748 = &str747.replace(". ", " ∥ ");
   let str749 = &str748.replace(".", "");
   let str750 = &str749.replace("! ", " ∥ ");
   let str751 = &str750.replace("!", "");
   let str752 = &str751.replace("? ", " ∥ ");
   let str753 = &str752.replace("?", "");
   let str754 = &str753.replace("(", "∣ ");
   let str755 = &str754.replace(")", " ∣");
   let str756 = &str755.replace(" - ", " ∣ ");
   let str757 = &str756.replace(" – ", " ∣ ");
   let result = &str757.replace("--", " ∣ ");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("AUSTRALIAN ENGLISH: [EXPERIMENTAL, STILL IN ALPHA STAGE]".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("AUSTRALIAN ENGLISH: [EXPERIMENTAL, STILL IN ALPHA STAGE]".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Australian English: {}", red.to_owned() + "[experimental, still in alpha stage]" + reset);
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// ENGLISH: ORTHOGRAPHY

pub fn ortuseng(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str1space = original_text.to_owned() + " "; // mark word ending
   let str2 = &str1space.replace("celling", "celing");
   let str3 = &str2.replace("delling", "deling");
   let str4 = &str3.replace("velling", "veling");
   let str5 = &str4.replace("bour", "bor");
   let str6 = &str5.replace("iour", "ior");
   let str7 = &str6.replace("nour", "nor");
   let str8 = &str7.replace("olour", "olor");
   let str9 = &str8.replace("centre", "center");
   let str10 = &str9.replace("metre", "meter");
   let str11 = &str10.replace("theatre", "theater");
   let str12 = &str11.replace("fence", "fense");
   let str13 = &str12.replace("lise", "lize");
   let str14 = &str13.replace("lyse", "lyze");
   let str15 = &str14.replace("sation", "zation");
   let str16 = &str15.replace("aestetic", "estetic");
   let str17 = &str16.replace("Aestetic", "Estetic");
   let str18 = &str17.replace("faec", "fec");
   let str19 = &str18.replace("Faec", "Fec");
   let str20 = &str19.replace("palaeo", "paleo");
   let str21 = &str20.replace("Palaeo", "Paleo");
   let str22 = &str21.replace("manoeuvre", "maneuver");
   let str23 = &str22.replace("aluminium", "aluminum");
   let str24 = &str23.replace("Aluminium", "Aluminum");
   let str25 = &str24.replace(" axe ", " ax ");
   let str26 = &str25.replace(" axe,", " ax,");
   let str27 = &str26.replace(" axe.", " ax.");
   let str28 = &str27.replace("cheque", "check");
   let str29 = &str28.replace("cumquat", "kumquat");
   let str30 = &str29.replace("fulfil", "fulfill");
   let str31 = &str30.replace("fulfilll", "fulfill");
   let str32 = &str31.replace("Fulfil", "Fulfill");
   let str33 = &str32.replace("Fulfilll", "Fulfill");
   let str34 = &str33.replace("grey", "gray");
   let str35 = &str34.replace("Grey", "Gray");
   let str36 = &str35.replace("kerb", "curb");
   let str37 = &str36.replace("moustache", "mustache");
   let str38 = &str37.replace("mum ", "mom ");
   let str39 = &str38.replace("mum,", "mom,");
   let str40 = &str39.replace("mum.", "mom.");
   let str41 = &str40.replace("Mum,", "Mom,");
   let str42 = &str41.replace("programme", "program");
   let str43 = &str42.replace("Programme", "Program");
   let str44 = &str43.replace("sceptical", "skeptical");
   let str45 = &str44.replace("Sceptical", "Skeptical");
   let str46 = &str45.replace("CELLING", "CELING");
   let str47 = &str46.replace("DELLING", "DELING");
   let str48 = &str47.replace("VELLING", "VELING");
   let str49 = &str48.replace("BOUR", "BOR");
   let str50 = &str49.replace("IOUR", "IOR");
   let str51 = &str50.replace("NOUR", "NOR");
   let str52 = &str51.replace("OLOUR", "OLOR");
   let str53 = &str52.replace("CENTRE", "CENTER");
   let str54 = &str53.replace("METRE", "METER");
   let str55 = &str54.replace("THEATRE", "THEATER");
   let str56 = &str55.replace("FENCE", "FENSE");
   let str57 = &str56.replace("LISE", "LIZE");
   let str58 = &str57.replace("LYSE", "LYZE");
   let str59 = &str58.replace("SATION", "ZATION");
   let str60 = &str59.replace("AESTETIC", "ESTETIC");
   let str61 = &str60.replace("FAEC", "FEC");
   let str62 = &str61.replace("PALAEO", "PALEO");
   let str63 = &str62.replace("MANOEUVRE", "MANEUVER");
   let str64 = &str63.replace("ALUMINIUM", "ALUMINUM");
   let str65 = &str64.replace(" AXE ", " AX ");
   let str66 = &str65.replace(" AXE,", " AX,");
   let str67 = &str66.replace(" AXE.", " AX.");
   let str68 = &str67.replace("CHEQUE", "CHECK");
   let str69 = &str68.replace("CUMQUAT", "KUMQUAT");
   let str70 = &str69.replace("FULFIL", "FULFILL");
   let str71 = &str70.replace("FULFILLL", "FULFILL");
   let str72 = &str71.replace("GREY", "GRAY");
   let str73 = &str72.replace("KERB", "CURB");
   let str74 = &str73.replace("MOUSTACHE", "MUSTACHE");
   let str75 = &str74.replace("MUM ", "MOM ");
   let str76 = &str75.replace("MUM,", "MOM,");
   let str77 = &str76.replace("MUM.", "MOM.");
   let str78 = &str77.replace("PROGRAMME", "PROGRAM");
   let str79 = &str78.replace("SCEPTICAL", "SKEPTICAL");
   let str80 = &str79.replace("liquorice", "licorice");
   let str81 = &str80.replace("LIQUORICE", "LICORICE");
   let str82 = &str81.replace("tyre", "tire");
   let str83 = &str82.replace("TYRE", "TIRE");
   let str84 = &str83.replace("wellery", "welry");
   let result = &str84.replace("WELLERY", "WELRY");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("AMERICAN ENGLISH:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("AMERICAN ENGLISH:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("American English:");
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// LATIN: IPA

pub fn ipalat(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str1dot = original_text.to_owned() + "."; // mark word ending
   let str2 = &str1dot.to_lowercase();
   let str3 = &str2.replace("ꟾ", "ī");
   let str4 = &str3.replace("g", "ɡ");
   let str5 = &str4.replace("v", "u");
   let str6 = &str5.replace("uu", "vu");
   let str7 = &str6.replace("ui", "vi");
   let str8 = &str7.replace("ue", "ve");
   let str9 = &str8.replace("ua", "va");
   let str10 = &str9.replace("uo", "vo");
   let str11 = &str10.replace("uū", "vū");
   let str12 = &str11.replace("uī", "vī");
   let str13 = &str12.replace("uē", "vē");
   let str14 = &str13.replace("uā", "vā");
   let str15 = &str14.replace("uō", "vō");
   let str16 = &str15.replace("bs", "ps");
   let str17 = &str16.replace("bt", "pt");
   let str18 = &str17.replace("s", "s̺");
   let str19 = &str18.replace("s̺b", "s̬b");
   let str20 = &str19.replace("s̺d", "s̬d");
   let str21 = &str20.replace("s̺ɡ", "s̬ɡ");
   let str22 = &str21.replace("s̺m", "s̬m");
   let str23 = &str22.replace("s̺n", "s̬n");
   let str24 = &str23.replace("nd", "md");
   let str25 = &str24.replace("n d", "m d");
   let str26 = &str25.replace("np", "mp");
   let str27 = &str26.replace("n p", "m p");
   let str28 = &str27.replace("inf", "ĩːf");
   let str29 = &str28.replace("nf", "ɱf");
   let str30 = &str29.replace("n f", "ɱ f");
   let str31 = &str30.replace("nm", "mm");
   let str32 = &str31.replace("n m", "m m");
   let str33 = &str32.replace("nq", "ŋq");
   let str34 = &str33.replace("n q", "ŋ q");
   let str35 = &str34.replace("nc", "ŋc");
   let str36 = &str35.replace("n c", "ŋ c");
   let str37 = &str36.replace("l", "ɫ");
   let str38 = &str37.replace("ɫɫ", "ll");
   let str39 = &str38.replace("ɫi", "li");
   let str40 = &str39.replace("rh", "r̥");
   let str41 = &str40.replace("v", "w");
   let str42 = &str41.replace("x", "k͡s̺");
   let str43 = &str42.replace("ɡn", "ŋn");
   let str44 = &str43.replace("ë", "ɛ");
   let str45 = &str44.replace("ü", "u̞");
   let str46 = &str45.replace("ĕr", "ær");
   let str47 = &str46.replace("ie", "īe");
   let str48 = &str47.replace("cume", "kʉme");
   let str49 = &str48.replace("ptim", "ptɨm");
   let str50 = &str49.replace("crim", "krɨm");
   let str51 = &str50.replace("c", "k");
   let str52 = &str51.replace("q", "k");
   let str53 = &str52.replace("um ", "ũː ");
   let str54 = &str53.replace("um,", "ũː,");
   let str55 = &str54.replace("um.", "ũː.");
   let str56 = &str55.replace("um?", "ũː?");
   let str57 = &str56.replace("um!", "ũː!");
   let str58 = &str57.replace("um·", "ũː·");
   let str59 = &str58.replace("em ", "ɛ̝̃ː ");
   let str60 = &str59.replace("em,", "ɛ̝̃ː,");
   let str61 = &str60.replace("em;", "ɛ̝̃ː;");
   let str62 = &str61.replace("em.", "ɛ̝̃ː.");
   let str63 = &str62.replace("em?", "ɛ̝̃ː?");
   let str64 = &str63.replace("em!", "ɛ̝̃ː!");
   let str65 = &str64.replace("em·", "ɛ̝̃ː·");
   let str66 = &str65.replace("ens", "ɛ̝̃ːs̺");
   let str67 = &str66.replace("ons", "ɔ̝̃ːs̺");
   let str68 = &str67.replace("er", "ær");
   let str69 = &str68.replace("ia", "ja");
   let str70 = &str69.replace("ii", "ji");
   let str71 = &str70.replace("io", "jo");
   let str72 = &str71.replace("iu", "ju");
   let str73 = &str72.replace("iā", "jā");
   let str74 = &str73.replace("iī", "jī");
   let str75 = &str74.replace("iō", "jō");
   let str76 = &str75.replace("iū", "jū");
   let str77 = &str76.replace("ea", "ia");
   let str78 = &str77.replace("ae", "ɐɛ̯");
   let str79 = &str78.replace("oe", "ɔɛ̯");
   let str80 = &str79.replace("i", "i̞");
   let str81 = &str80.replace("o", "ɔ");
   let str82 = &str81.replace("u", "u̞");
   let str83 = &str82.replace("y", "ʏ");
   let str84 = &str83.replace("a", "ɐ");
   let str85 = &str84.replace("e", "ɛ");
   let str86 = &str85.replace("ā", "ɐː");
   let str87 = &str86.replace("á", "ɐː");
   let str88 = &str87.replace("ē", "ɛ̝ː");
   let str89 = &str88.replace("é", "ɛ̝ː");
   let str90 = &str89.replace("ī", "iː");
   let str91 = &str90.replace("í", "iː");
   let str92 = &str91.replace("ō", "ɔ̝ː");
   let str93 = &str92.replace("ó", "ɔ̝ː");
   let str94 = &str93.replace("ū", "uː");
   let str95 = &str94.replace("ú", "uː");
   let str96 = &str95.replace("ȳ", "yː");
   let str97 = &str96.replace("ý", "yː");
   let str98 = &str97.replace("ă", "ɐ");
   let str99 = &str98.replace("ĕ", "ɛ");
   let str100 = &str99.replace("ĭ", "i̞");
   let str101 = &str100.replace("ŏ", "ɔ");
   let str102 = &str101.replace("ŭ", "u̞");
   let str103 = &str102.replace("y̆", "ʏ");
   let str104 = &str103.replace("ũ̞ː", "ũː");
   let str105 = &str104.replace("ɡw", "ɡʷ");
   let str106 = &str105.replace("kw", "kʷ");
   let str107 = &str106.replace("kh", "kʰ");
   let str108 = &str107.replace("ph", "pʰ");
   let str109 = &str108.replace("th", "tʰ");
   let str110 = &str109.replace(",", " ∣");
   let str111 = &str110.replace(";", " ∥");
   let str112 = &str111.replace(":", " ∣");
   let str113 = &str112.replace(". ", " ∥ ");
   let str114 = &str113.replace(".", "");
   let str115 = &str114.replace("! ", " ∥ ");
   let str116 = &str115.replace("!", "");
   let str117 = &str116.replace("? ", " ∥ ");
   let str118 = &str117.replace("?", "");
   let str119 = &str118.replace("(", "∣ ");
   let str120 = &str119.replace(")", " ∣");
   let str121 = &str120.replace("·", " ");
   let str122 = &str121.replace(" - ", " ∣ ");
   let str123 = &str122.replace(" – ", " ∣ ");
   let result = &str123.replace("--", " ∣ ");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("CLASSICAL LATIN:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("CLASSICAL LATIN:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Classical Latin:");
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// LATIN: ORTHOGRAPHY

pub fn ortlat(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str2 = original_text.replace("a", "A");
   let str3 = &str2.replace("á", "Á");
   let str4 = &str3.replace("ă", "A");
   let str5 = &str4.replace("ā", "Á");
   let str6 = &str5.replace("b", "B");
   let str7 = &str6.replace("c", "C");
   let str8 = &str7.replace("d", "D");
   let str9 = &str8.replace("e", "E");
   let str10 = &str9.replace("ĕ", "E");
   let str11 = &str10.replace("é", "É");
   let str12 = &str11.replace("ē", "É");
   let str13 = &str12.replace("ë", "E");
   let str14 = &str13.replace("f", "F");
   let str15 = &str14.replace("g", "G");
   let str16 = &str15.replace("h", "H");
   let str17 = &str16.replace("i", "I");
   let str18 = &str17.replace("ĭ", "I");
   let str19 = &str18.replace("Ī", "ꟾ");
   let str20 = &str19.replace("ī", "ꟾ");
   let str21 = &str20.replace("í", "ꟾ");
   let str22 = &str21.replace("Í", "ꟾ");
   let str23 = &str22.replace("j", "I");
   let str24 = &str23.replace("k", "K");
   let str25 = &str24.replace("l", "L");
   let str26 = &str25.replace("m", "M");
   let str27 = &str26.replace("n", "N");
   let str28 = &str27.replace("o", "O");
   let str29 = &str28.replace("ŏ", "O");
   let str30 = &str29.replace("ō", "Ó");
   let str31 = &str30.replace("p", "P");
   let str32 = &str31.replace("q", "Q");
   let str33 = &str32.replace("r", "R");
   let str34 = &str33.replace("s", "S");
   let str35 = &str34.replace("t", "T");
   let str36 = &str35.replace("u", "V");
   let str37 = &str36.replace("v", "V");
   let str38 = &str37.replace("ú", "Ú");
   let str39 = &str38.replace("ŭ", "V");
   let str40 = &str39.replace("ū", "Ú");
   let str41 = &str40.replace("ü", "V");
   let str42 = &str41.replace("w", "VV");
   let str43 = &str42.replace("x", "X");
   let str44 = &str43.replace("y", "Y");
   let str45 = &str44.replace("y̆", "Y");
   let str46 = &str45.replace("ȳ", "Ý");
   let str47 = &str46.replace("ý", "Ý");
   let str48 = &str47.replace("z", "Z");
   let str49 = &str48.replace("Ă", "A");
   let str50 = &str49.replace("Ā", "Á");
   let str51 = &str50.replace("Ĕ", "E");
   let str52 = &str51.replace("Ë", "E");
   let str53 = &str52.replace("Ĭ", "I");
   let str54 = &str53.replace("Ī", "ꟾ");
   let str55 = &str54.replace("Í", "ꟾ");
   let str56 = &str55.replace("J", "I");
   let str57 = &str56.replace("Ŏ", "O");
   let str58 = &str57.replace("Ō", "Ó");
   let str59 = &str58.replace("Ŭ", "V");
   let str60 = &str59.replace("Ū", "Ú");
   let str61 = &str60.replace("Ü", "V");
   let str62 = &str61.replace("Y̆", "Y");
   let str63 = &str62.replace("Ȳ", "Ý");
   let str64 = &str63.replace(" - ", "·");
   let str65 = &str64.replace(" – ", "·");
   let str66 = &str65.replace("--", "·");
   let str67 = &str66.replace(", ", "·");
   let str68 = &str67.replace("; ", "·");
   let str69 = &str68.replace(": ", "·");
   let str70 = &str69.replace(". ", "·");
   let str71 = &str70.replace("! ", "·");
   let str72 = &str71.replace("? ", "·");
   let str73 = &str72.replace(".", "");
   let str74 = &str73.replace("!", "");
   let str75 = &str74.replace("?", "");
   let result = &str75.replace(" ", "·");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("CLASSICAL LATIN:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("CLASSICAL LATIN:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Classical Latin:");
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// POLISH: IPA

pub fn ipapol(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str2 = original_text.to_lowercase();
   let str3 = &str2.replace("dż", "ɖ͡ʐ");
   let str4 = &str3.replace("dzi", "d͡ʑi");
   let str5 = &str4.replace("dź", "d͡ʑ");
   let str6 = &str5.replace("rdz", "ɾd̪͡z̪");
   let str7 = &str6.replace("rd", "ɾd̪");
   let str8 = &str7.replace("dz", "d͡z");
   let str9 = &str8.replace("ód", "ód̥");
   let str10 = &str9.replace("dcz", "d̥cz");
   let str11 = &str10.replace("dt", "d̥t");
   let str12 = &str11.replace("dk", "d̥k");
   let str13 = &str12.replace("t", "t̪");
   let str14 = &str13.replace("st̪rz", "ʂʈ͡ʂ");
   let str15 = &str14.replace("t̪rz", "ṯʃ");
   let str16 = &str15.replace("prz", "pʂ");
   let str17 = &str16.replace("t̪r", "ṯɾ̥");
   let str18 = &str17.replace("rt̪", "ɾ̥ṯ");
   let str19 = &str18.replace("t̪m", "ṯm̥");
   let str20 = &str19.replace("izn", "is̬n̥");
   let str21 = &str20.replace("zn", "zn̥");
   let str22 = &str21.replace("ans", "an̥s");
   let str23 = &str22.replace("ącz", "ąṉcz");
   let str24 = &str23.replace("sz", "ʃ");
   let str25 = &str24.replace("śln", "śl̥n");
   let str26 = &str25.replace("ślm", "śl̥m");
   let str27 = &str26.replace("śń", "śɲ̥");
   let str28 = &str27.replace("ń", "ɲ");
   let str29 = &str28.replace("ni", "ɲi");
   let str30 = &str29.replace("mc", "m̥c");
   let str31 = &str30.replace("nc", "n̥c");
   let str32 = &str31.replace("ch", "χ");
   let str33 = &str32.replace("h", "x");
   let str34 = &str33.replace("ś", "ɕ");
   let str35 = &str34.replace("s", "s̻");
   let str36 = &str35.replace("cz", "ʈ͡ʂ");
   let str37 = &str36.replace("ci", "t͡ɕi");
   let str38 = &str37.replace("c", "t͡s");
   let str39 = &str38.replace("ć", "t͡ɕ");
   let str40 = &str39.replace("χrz", "χʂ");   
   let str41 = &str40.replace("rz", "ʒ");
   let str42 = &str41.replace("r", "ɾ");
   let str43 = &str42.replace("ż", "ʒ");
   let str44 = &str43.replace("ź", "ʑ");
   let str45 = &str44.replace("ng", "ŋɡ");
   let str46 = &str45.replace("nk", "ŋk");
   let str47 = &str46.replace("g", "ɡ");
   let str48 = &str47.replace("wk", "v̥k");
   let str49 = &str48.replace("w", "v");
   let str50 = &str49.replace("bł", "b̥w̥");
   let str51 = &str50.replace("ł", "w");
   let str52 = &str51.replace("im", "i̞ɱ");
   let str53 = &str52.replace("in", "i̞ŋ");
   let str54 = &str53.replace("ia", "i̞a");
   let str55 = &str54.replace("io", "i̞o");
   let str56 = &str55.replace("ie", "i̞e");
   let str57 = &str56.replace("iu", "i̞ü");
   let str58 = &str57.replace("ju", "jü");
   let str59 = &str58.replace("i", "i̞");
   let str60 = &str59.replace("aʈ͡ʂ", "ɐʈ͡ʂ");
   let str61 = &str60.replace("ʈ͡ʂa", "ʈ͡ʂɐ");
   let str62 = &str61.replace("at͡s", "ɐt͡s");
   let str63 = &str62.replace("t͡sa", "t͡sɐ");
   let str64 = &str63.replace("aʃ", "ɐʃ");
   let str65 = &str64.replace("ʃa", "ʃɐ");
   let str66 = &str65.replace("s̻a", "s̻ɐ");
   let str67 = &str66.replace("as̻", "ɐs̻");
   let str68 = &str67.replace("am", "aɱ");
   let str69 = &str68.replace("an", "aŋ");
   let str70 = &str69.replace("a", "ä");
   let str71 = &str70.replace("eʈ͡ʂ", "ɛ̝̈ʈ͡ʂ");
   let str72 = &str71.replace("ʈ͡ʂe", "ʈ͡ʂɛ̝̈");
   let str73 = &str72.replace("et͡s", "ɛ̝̈t͡s");
   let str74 = &str73.replace("t͡se", "t͡sɛ̝̈");
   let str75 = &str74.replace("eʃ", "ɛ̝̈ʃ");
   let str76 = &str75.replace("ʃe", "ʃɛ̝̈");
   let str77 = &str76.replace("s̻e", "s̻ɛ̝̈");
   let str78 = &str77.replace("es̻", "ɛ̝̈s̻");
   let str79 = &str78.replace("em", "ɛ̝ɱ");
   let str80 = &str79.replace("en", "ɛ̝ŋ");
   let str81 = &str80.replace("e", "ɛ̝");
   let str82 = &str81.replace("ę", "ɛ̝̃");
   let str83 = &str82.replace("oʈ͡ʂ", "ɔ̝̈ʈ͡ʂ");
   let str84 = &str83.replace("ʈ͡ʂo", "ʈ͡ʂɔ̝̈");
   let str85 = &str84.replace("ot͡s", "ɔ̝̈t͡s");
   let str86 = &str85.replace("t͡so", "t͡sɔ̝̈");
   let str87 = &str86.replace("oʃ", "ɔ̝̈ʃ");
   let str88 = &str87.replace("ʃo", "ʃɔ̝̈");
   let str89 = &str88.replace("s̻o", "s̻ɔ̝̈");
   let str90 = &str89.replace("os̻", "ɔ̝̈s̻");
   let str91 = &str90.replace("om", "ɔ̝ɱ");
   let str92 = &str91.replace("on", "ɔ̝ŋ");
   let str93 = &str92.replace("o", "ɔ̝");
   let str94 = &str93.replace("ą", "ɔ̝̃");
   let str95 = &str94.replace("ó", "u");
   let str96 = &str95.replace("um", "u̞ɱ");
   let str97 = &str96.replace("un", "u̞ŋ");
   let str98 = &str97.replace("u", "u̞");
   let str99 = &str98.replace("yʈ͡ʂ", "ɘʈ͡ʂ");
   let str100 = &str99.replace("ʈ͡ʂy", "ʈ͡ʂɘ");
   let str101 = &str100.replace("yt͡s", "ɘt͡s");
   let str102 = &str101.replace("t͡sy", "t͡sɘ");
   let str103 = &str102.replace("yʃ", "ɘʃ");
   let str104 = &str103.replace("ʃy", "ʃɘ");
   let str105 = &str104.replace("s̻y", "s̻ɘ");
   let str106 = &str105.replace("ys̻", "ɘs̻");
   let str107 = &str106.replace("ym", "ɘ̟ɱ");
   let str108 = &str107.replace("yn", "ɘ̟ŋ");
   let str109 = &str108.replace("y", "ɘ̟");
   let str110 = &str109.replace(",", " ∣");
   let str111 = &str110.replace(";", " ∥");
   let str112 = &str111.replace(":", " ∣");
   let str113 = &str112.replace(". ", " ∥ ");
   let str114 = &str113.replace(".", "");
   let str115 = &str114.replace("! ", " ∥ ");
   let str116 = &str115.replace("!", "");
   let str117 = &str116.replace("? ", " ∥ ");
   let str118 = &str117.replace("?", "");
   let str119 = &str118.replace("(", "∣ ");
   let str120 = &str119.replace(")", " ∣");
   let str121 = &str120.replace(" - ", " ∣ ");
   let str122 = &str121.replace(" – ", " ∣ ");
   let result = &str122.replace("--", " ∣ ");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("POLISH (CZĘSTOCHOWA):".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("POLISH (CZĘSTOCHOWA):".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Polish (Częstochowa):");
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// AYACUCHO QUECHUA: IPA

pub fn ipaque(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str2 = original_text.to_lowercase();
   let str3 = &str2.replace("sh", "ch");
   let str4 = &str3.replace("chh", "ch");
   let str5 = &str4.replace("kh", "k");
   let str6 = &str5.replace("th", "t");
   let str7 = &str6.replace("ph", "p");
   let str8 = &str7.replace("qh", "q");
   let str9 = &str8.replace("j", "q");
   let str10 = &str9.replace("'", "");
   let str11 = &str10.replace("’", "");
   let str12 = &str11.replace("qu", "qo");
   let str13 = &str12.replace("uq", "oq");
   let str14 = &str13.replace("qi", "qe");
   let str15 = &str14.replace("iq", "eq");
   let str16 = &str15.replace("an", "aŋ");
   let str17 = &str16.replace("ma", "mæ");
   let str18 = &str17.replace("mis", "mɪ̝s̻");
   let str19 = &str18.replace("mich", "mɪ̝ch");
   let str20 = &str19.replace("cha", "chä");
   let str21 = &str20.replace("ch", "ʧ");
   let str22 = &str21.replace("g", "ɡ");
   let str23 = &str22.replace("ka", "kæ̞");
   let str24 = &str23.replace("da", "dæ");
   let str25 = &str24.replace("d", "d̥");
   let str26 = &str25.replace("lla", "llæ");
   let str27 = &str26.replace("ll", "ʎ");
   let str28 = &str27.replace("ñ", "ɲ");
   let str29 = &str28.replace("pa", "pä");
   let str30 = &str29.replace("f", "f̟");
   let str31 = &str30.replace("q", "χ");
   let str32 = &str31.replace("r", "ɾ");
   let str33 = &str32.replace("ɾχ", "ɾ̥χ");
   let str34 = &str33.replace("sa", "s̻ä");
   let str35 = &str34.replace("s", "s̻");
   let str36 = &str35.replace("ci", "s̻i");
   let str37 = &str36.replace("ce", "s̻e");
   let str38 = &str37.replace("z", "s̻");
   let str39 = &str38.replace("t", "t̪");
   let str40 = &str39.replace("wa", "wæ");   
   let str41 = &str40.replace("ya", "yæ");
   let str42 = &str41.replace("y", "j");
   let str43 = &str42.replace("o", "ʊ̞");
   let str44 = &str43.replace("e", "ɪ̞");
   let str45 = &str44.replace("u", "u̞");
   let str46 = &str45.replace("i", "i̞");
   let str47 = &str46.replace(",", " ∣");
   let str48 = &str47.replace(";", " ∥");
   let str49 = &str48.replace(":", " ∣");
   let str50 = &str49.replace(". ", " ∥ ");
   let str51 = &str50.replace(".", "");
   let str52 = &str51.replace("! ", " ∥ ");
   let str53 = &str52.replace("!", "");
   let str54 = &str53.replace("¡", "");
   let str55 = &str54.replace("? ", " ∥ ");
   let str56 = &str55.replace("?", "");
   let str57 = &str56.replace("¿", "");
   let str58 = &str57.replace("(", "∣ ");
   let str59 = &str58.replace(")", " ∣");
   let str60 = &str59.replace(" - ", " ∣ ");
   let str61 = &str60.replace(" – ", " ∣ ");
   let result = &str61.replace("--", " ∣ ");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("AYACUCHO QUECHUA (WANTA):".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("AYACUCHO QUECHUA (WANTA):".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Ayacucho Quechua (Wanta):");
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// AYACUCHO QUECHUA: DIALECT

pub fn quelct(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str1space = original_text.to_owned() + " "; // mark word ending
   let str2 = &str1space.replace("o", "e");
   let str3 = &str2.replace("e", "i");
   let str4 = &str3.replace("chh", "ch");
   let str5 = &str4.replace("kh", "k");
   let str6 = &str5.replace("th", "t");
   let str7 = &str6.replace("ph", "p");
   let str8 = &str7.replace("qh", "q");
   let str9 = &str8.replace("j", "q");
   let str10 = &str9.replace("'", "");
   let str11 = &str10.replace("’", "");
   let str12 = &str11.replace("san", "chkan");
   let str13 = &str12.replace("shan", "chkan");
   let str14 = &str13.replace("shyan", "chkan");
   let str15 = &str14.replace("shwan", "swan");
   let str16 = &str15.replace("sh", "ch");
   let str17 = &str16.replace(" aska ", " achka ");
   let str18 = &str17.replace("qti", "pti");
   let str19 = &str18.replace("lqu", "llqu");
   let str20 = &str19.replace("lllqu", "llqu");
   let str21 = &str20.replace("ulqi", "ullqi");
   let str22 = &str21.replace("an ", "am ");
   let str23 = &str22.replace("an.", "am.");
   let str24 = &str23.replace("an,", "am,");
   let str25 = &str24.replace("an;", "am;");
   let str26 = &str25.replace("an:", "am:");
   let str27 = &str26.replace("an!", "am!");
   let str28 = &str27.replace("an?", "am?");
   let str29 = &str28.replace("chkam", "chkan");
   let str30 = &str29.replace("hanam", "hanan");
   let str31 = &str30.replace("Hanam", "Hanan");
   let str32 = &str31.replace("kanam", "kanan");
   let str33 = &str32.replace("Kanam", "Kanan");
   let str34 = &str33.replace(" kunam", " kunan");
   let str35 = &str34.replace("Kunam", "Kunan");
   let str36 = &str35.replace(" ñam ", " ñan ");
   let str37 = &str36.replace(" yam ", " ñan ");
   let str38 = &str37.replace("mam ", "man ");
   let str39 = &str38.replace("mam.", "man.");
   let str40 = &str39.replace("mam,", "man,");
   let str41 = &str40.replace("mam;", "man;");
   let str42 = &str41.replace("mam:", "man:");
   let str43 = &str42.replace("mam!", "man!");
   let str44 = &str43.replace("mam?", "man?");
   let str45 = &str44.replace("munam", "munan");
   let str46 = &str45.replace("swam", "swan");
   let str47 = &str46.replace("wam ", "wan ");
   let str48 = &str47.replace("wam.", "wan.");
   let str49 = &str48.replace("wam,", "wan,");
   let str50 = &str49.replace("wam;", "wan;");
   let str51 = &str50.replace("wam:", "wan:");
   let str52 = &str51.replace("wam!", "wan!");
   let str53 = &str52.replace("wam?", "wan?");
   let str54 = &str53.replace("un ", "um ");
   let str55 = &str54.replace("un.", "um.");
   let str56 = &str55.replace("un,", "um,");
   let str57 = &str56.replace("un;", "um;");
   let str58 = &str57.replace("un:", "um:");
   let str59 = &str58.replace("un!", "um!");
   let str60 = &str59.replace("un?", "um?");
   let str61 = &str60.replace("hatum", "hatun");
   let str62 = &str61.replace("Hatum", "Hatun");
   let str63 = &str62.replace("sum ", "sun ");
   let str64 = &str63.replace("sum.", "sun.");
   let str65 = &str64.replace("sum,", "sun,");
   let str66 = &str65.replace("sum;", "sun;");
   let str67 = &str66.replace("sum:", "sun:");
   let str68 = &str67.replace("sum!", "sun!");
   let str69 = &str68.replace("sum?", "sun?");
   let str70 = &str69.replace("squm", "squn");
   let str71 = &str70.replace(" ukum ", " ukun ");
   let str72 = &str71.replace("in ", "im ");
   let str73 = &str72.replace("in.", "im.");
   let str74 = &str73.replace("in,", "im,");
   let str75 = &str74.replace("in;", "im;");
   let str76 = &str75.replace("in:", "im:");
   let str77 = &str76.replace("in!", "im!");
   let str78 = &str77.replace("in?", "im?");
   let str79 = &str78.replace(" nim ", " nin ");
   let str80 = &str79.replace("allim", "allin");
   let str81 = &str80.replace("Allim", "Allin");
   let str82 = &str81.replace("knim", "knin");
   let str83 = &str82.replace("nnim", "nnin");
   let str84 = &str83.replace("qnim", "qnin");
   let str85 = &str84.replace("ptim", "ptin");
   let str86 = &str85.replace("stim", "stin");
   let str87 = &str86.replace("ntim ", "ntin ");
   let str88 = &str87.replace("ntim.", "ntin.");
   let str89 = &str88.replace("ntim,", "ntin,");
   let str90 = &str89.replace("ntim;", "ntin;");
   let str91 = &str90.replace("ntim:", "ntin:");
   let str92 = &str91.replace("ntim?", "ntin?");
   let str93 = &str92.replace("ntim!", "ntin!");
   let str94 = &str93.replace("urim ", "urin ");
   let str95 = &str94.replace("urim.", "urin.");
   let str96 = &str95.replace("urim,", "urin,");
   let str97 = &str96.replace("urim;", "urin;");
   let str98 = &str97.replace("urim:", "urin:");
   let str99 = &str98.replace("urim?", "urin?");
   let str100 = &str99.replace("urim!", "urin!");
   let str101 = &str100.replace("Urim", "Urin");
   let str102 = &str101.replace("ynim", "ynin");
   let str103 = &str102.replace(" chanim", " chanin");
   let str104 = &str103.replace("llimpim", "llimpin");
   let str105 = &str104.replace(" sapim ", " sapin ");
   let str106 = &str105.replace(" rapim ", " rapin ");
   let str107 = &str106.replace("q simim", "pa simin");
   let str108 = &str107.replace("pa simim", "pa simin");
   let str109 = &str108.replace("q sutim", "pa sutin");
   let str110 = &str109.replace("pa sutim", "pa sutin");
   let str111 = &str110.replace("churim", "churin");
   let str112 = &str111.replace("q churin", "pa churin");
   let str113 = &str112.replace("turim", "turin");
   let str114 = &str113.replace("q turin", "pa turin");
   let str115 = &str114.replace("aqarim ", "aqarin ");
   let str116 = &str115.replace("wakim", "wakin");
   let str117 = &str116.replace("Wakim", "Wakin");
   let str118 = &str117.replace("nuqa", "ñuqa");
   let str119 = &str118.replace("ñuqaq", "ñuqapa");
   let str120 = &str119.replace("Nuqa", "Ñuqa");
   let str121 = &str120.replace("Ñuqaq", "Ñuqapa");
   let str122 = &str121.replace("nchis", "nchik");
   let str123 = &str122.replace("nchiq", "nchik");
   let str124 = &str123.replace("nkichis", "nkichik");
   let str125 = &str124.replace("nkichiq", "nkichik");
   let str126 = &str125.replace("ykichis", "ykichik");
   let str127 = &str126.replace(" qanchik", " qanchis");
   let str128 = &str127.replace("qankuna", "qamkuna");
   let str129 = &str128.replace("Qankuna", "Qamkuna");
   let str130 = &str129.replace(" qanqa ", " qamqa ");
   let str131 = &str130.replace("Qanqa", "Qampa");
   let str132 = &str131.replace(" qanpa ", " qampa ");
   let str133 = &str132.replace("Qanpa", "Qampa");
   let str134 = &str133.replace("qanpaq", "qampaq");
   let str135 = &str134.replace("Qanpaq", "Qampaq");
   let str136 = &str135.replace(" qanwan", " qamwan");
   let str137 = &str136.replace("Qanwan", "Qamwan");
   let str138 = &str137.replace("Qan ", "Qam ");
   let str139 = &str138.replace("personayki", "qam");
   let str140 = &str139.replace("Personayki", "Qam");
   let str141 = &str140.replace("llanka", "llamka");
   let str142 = &str141.replace("Llanka", "Llamka");
   let str143 = &str142.replace(" uk ", " huk ");
   let str144 = &str143.replace("kinsa", "kimsa");
   let str145 = &str144.replace("Kinsa", "Kimsa");
   let str146 = &str145.replace("pisqa", "pichqa");
   let str147 = &str146.replace("Pisqa", "Pichqa");
   let str148 = &str147.replace(" ukya", " upya");
   let str149 = &str148.replace("Ukya", "Upya");
   let str150 = &str149.replace(" uqya", " upya");
   let str151 = &str150.replace("Uqya", "Upya");
   let str152 = &str151.replace("llillanmi", "llinllam");
   let str153 = &str152.replace("llillanchu", "llinllachu");
   let str154 = &str153.replace(" unu ", " yaku ");
   let str155 = &str154.replace(" unuta ", " yakuta ");
   let str156 = &str155.replace("haykaq", "haykapi");
   let str157 = &str156.replace("Haykaq", "Haykapi");
   let str158 = &str157.replace("maykaq", "haykapi");
   let str159 = &str158.replace("Maykaq", "Haykapi");
   let str160 = &str159.replace("mayka", "hayka");
   let str161 = &str160.replace("Mayka", "Hayka");
   let str162 = &str161.replace("mashqa", "hayka");
   let str163 = &str162.replace("Mashqa", "Hayka");
   let str164 = &str163.replace("punchay", "punchaw");
   let str165 = &str164.replace("ñaqa", "yaqa");
   let str166 = &str165.replace("Ñaqa", "Yaqa");
   let str167 = &str166.replace(" qaya ", " paqarin ");
   let str168 = &str167.replace(" qayan ", " paqarinmi ");
   let str169 = &str168.replace("Qaya ", "Paqarin ");
   let str170 = &str169.replace("Qayan ", "Paqarinmi ");
   let str171 = &str170.replace("Chh", "Ch");
   let str172 = &str171.replace("Kh", "K");
   let str173 = &str172.replace("Th", "T");
   let str174 = &str173.replace("Ph", "P");
   let str175 = &str174.replace("Qh", "Q");
   let str176 = &str175.replace("chum ", "chun ");
   let str177 = &str176.replace("chum.", "chun.");
   let str178 = &str177.replace("chum,", "chun,");
   let str179 = &str178.replace("Uchun ", "Uchum ");
   let str180 = &str179.replace("q parla", "pa parla");
   let str181 = &str180.replace("q rima", "pa rima");
   let result = &str181.replace("q simi", "pa simi");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("AYACUCHO QUECHUA:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("AYACUCHO QUECHUA:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Ayacucho Quechua:");
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// QUECHUA: ORTHOGRAPHY TRIVOCALIC

pub fn ortquetri(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str2 = original_text.replace("o", "u");
   let str3 = &str2.replace("O", "U");
   let str4 = &str3.replace("e", "i");
   let result = &str4.replace("E", "I");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("TRIVOCALIC:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("TRIVOCALIC:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Trivocalic:");
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// QUECHUA: ORTHOGRAPHY PENTAVOCALIC

pub fn ortquepen(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str2 = original_text.replace("qu", "qo");
   let str3 = &str2.replace("Qu", "Qo");
   let str4 = &str3.replace("QU", "QO");
   let str5 = &str4.replace("qi", "qe");
   let str6 = &str5.replace("Qi", "Qe");
   let str7 = &str6.replace("QI", "QE");
   let str8 = &str7.replace("qhu", "qho");
   let str9 = &str8.replace("Qhu", "Qho");
   let str10 = &str9.replace("QHU", "QHO");
   let str11 = &str10.replace("qhi", "qhe");
   let str12 = &str11.replace("Qhi", "Qhe");
   let str13 = &str12.replace("QHI", "QHE");
   let str14 = &str13.replace("ko", "ku");
   let str15 = &str14.replace("Ko", "Ku");
   let str16 = &str15.replace("KO", "KU");
   let str17 = &str16.replace("ke", "ki");
   let str18 = &str17.replace("Ke", "Ki");
   let str19 = &str18.replace("KE", "KI");
   let str20 = &str19.replace("kho", "khu");
   let str21 = &str20.replace("Kho", "Khu");
   let str22 = &str21.replace("KHO", "KHU");
   let str23 = &str22.replace("khe", "khi");
   let str24 = &str23.replace("Khe", "Khi");
   let str25 = &str24.replace("KHE", "KHI");
   let str26 = &str25.replace("q'u", "q'o");
   let str27 = &str26.replace("Q'u", "Q'o");
   let str28 = &str27.replace("Q'U", "Q'O");
   let str29 = &str28.replace("q'i", "q'e");
   let str30 = &str29.replace("Q'i", "Q'e");
   let str31 = &str30.replace("Q'I", "Q'E");
   let str32 = &str31.replace("q’u", "q’o");
   let str33 = &str32.replace("Q’u", "Q’o");
   let str34 = &str33.replace("Q’U", "Q’O");
   let str35 = &str34.replace("q’i", "q’e");
   let str36 = &str35.replace("Q’i", "Q’e");
   let str37 = &str36.replace("Q’I", "Q’E");
   let str38 = &str37.replace("uq", "oq");
   let str39 = &str38.replace("Uq", "Oq");
   let str40 = &str39.replace("UQ", "OQ");
   let str41 = &str40.replace("iq", "eq");
   let str42 = &str41.replace("Iq", "Eq");
   let result = &str42.replace("IQ", "EQ");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("PENTAVOCALIC:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("PENTAVOCALIC:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Pentavocalic:");
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// SANTA CRUZ DE LA SIERRA: IPA

pub fn spabosantacruz(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str1dot = original_text.to_owned() + "."; // mark word ending
   let str2dot = ".".to_owned() + &str1dot;
   let str3low = &str2dot.to_lowercase();
   let str1sgn = str3low.replace(";", ",");
   let str2sgn = str1sgn.replace(":", ",");
   let str3sgn = str2sgn.replace("!", ".");
   let str4sgn = str3sgn.replace("?", ".");
   let str1acc = &str4sgn.replace("", "");
   let str1 = str1acc.replace ("", "");
   let result = &str1.replace("", "");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("SANTA CRUZ DE LA SIERRA, BO: [NOT YET IMPLEMENTED]".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("SANTA CRUZ DE LA SIERRA, BO: [NOT YET IMPLEMENTED]".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Santa Cruz de la Sierra, BO:{}", red.to_owned() + " [not yet implemented]" + reset);
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// BOGOTÁ: IPA

pub fn spacobogota(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str1dot = original_text.to_owned() + "."; // mark word ending
   let str2dot = ".".to_owned() + &str1dot;
   let str3low = &str2dot.to_lowercase();
   let str1sgn = str3low.replace(";", ",");
   let str2sgn = str1sgn.replace(":", ",");
   let str3sgn = str2sgn.replace("!", ".");
   let str4sgn = str3sgn.replace("?", ".");
   let str1acc = &str4sgn.replace("", "");
   let str1 = str1acc.replace ("", "");
   let result = &str1.replace("", "");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("BOGOTÁ, CO: [NOT YET IMPLEMENTED]".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("BOGOTÁ, CO: [NOT YET IMPLEMENTED]".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Bogotá, CO:{}", red.to_owned() + " [not yet implemented]" + reset);
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// LETICIA: IPA

pub fn spacoleticia(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str1dot = original_text.to_owned() + "."; // mark word ending
   let str2dot = ".".to_owned() + &str1dot;
   let str3low = &str2dot.to_lowercase();
   let str1sgn = str3low.replace(";", ",");
   let str2sgn = str1sgn.replace(":", ",");
   let str3sgn = str2sgn.replace("!", ".");
   let str4sgn = str3sgn.replace("?", ".");
   let str1acc = &str4sgn.replace("", "");
   let str1 = str1acc.replace ("", "");
   let result = &str1.replace("", "");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("LETICIA, CO: [NOT YET IMPLEMENTED]".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("LETICIA, CO: [NOT YET IMPLEMENTED]".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Leticia, CO:{}", red.to_owned() + " [not yet implemented]" + reset);
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// MEDELLÍN: IPA

pub fn spacomedellin(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str1dot = original_text.to_owned() + "."; // mark word ending
   let str2dot = ".".to_owned() + &str1dot; // word beginning
   let str3low = &str2dot.to_lowercase();
   let str1sgn = str3low.replace(";", "."); // simplification
   let str2sgn = str1sgn.replace(":", ",");
   let str3sgn = str2sgn.replace("!", ".");
   let str4sgn = str3sgn.replace("?", ".");
   let str5sgn = str4sgn.replace("¡", "."); // word beginning
   let str6sgn = str5sgn.replace("¿", "."); // word beginning
   let str7sgn = str6sgn.replace(")", ",");
   let str8sgn = &str7sgn.replace("(", "∣ .");
   let str1acc = &str8sgn.replace("ar ", "ár "); // word stress
   let str2acc = &str1acc.replace("ar.", "ár.");
   let str3acc = &str2acc.replace("ar,", "ár,");
   let str4acc = &str3acc.replace("arme ", "árme ");
   let str5acc = &str4acc.replace("arme.", "árme.");
   let str6acc = &str5acc.replace("arme,", "árme,");
   let str7acc = &str6acc.replace("arte ", "árte ");
   let str8acc = &str7acc.replace("arte.", "árte.");
   let str9acc = &str8acc.replace("arte,", "árte,");
   let str10acc = &str9acc.replace("arse ", "árse ");
   let str11acc = &str10acc.replace("arse.", "árse.");
   let str12acc = &str11acc.replace("arse,", "árse,");
   let str13acc = &str12acc.replace("iendo", "iéndo");
   let str14acc = &str13acc.replace("ando ", "ándo ");
   let str15acc = &str14acc.replace("ando.", "ándo.");
   let str16acc = &str15acc.replace("ando,", "ándo,");
   let str17acc = &str16acc.replace("aba ", "ába ");
   let str18acc = &str17acc.replace("aba.", "ába.");
   let str19acc = &str18acc.replace("aba,", "ába,");
   let str20acc = &str19acc.replace("aban ", "ában ");
   let str21acc = &str20acc.replace("aban.", "ában.");
   let str22acc = &str21acc.replace("aban,", "ában,");
   let str23acc = &str22acc.replace("abas ", "ábas ");
   let str24acc = &str23acc.replace("abas.", "ábas.");
   let str25acc = &str24acc.replace("abas,", "ábas,");
   let str26acc = &str25acc.replace("acha ", "ácha ");
   let str27acc = &str26acc.replace("acha.", "ácha.");
   let str28acc = &str27acc.replace("acha,", "ácha,");
   let str29acc = &str28acc.replace("acho", "ácho");
   let str30acc = &str29acc.replace("ada ", "áda ");
   let str31acc = &str30acc.replace("ada.", "áda.");
   let str32acc = &str31acc.replace("ada,", "áda,");
   let str33acc = &str32acc.replace("adas", "ádas");
   let str34acc = &str33acc.replace("ado ", "ádo ");
   let str35acc = &str34acc.replace("ado.", "ádo.");
   let str36acc = &str35acc.replace("ado,", "ádo,");
   let str37acc = &str36acc.replace("ados", "ádos");
   let str38acc = &str37acc.replace("ante ", "ánte ");
   let str39acc = &str38acc.replace("ante.", "ánte.");
   let str40acc = &str39acc.replace("ante,", "ánte,");
   let str41acc = &str40acc.replace("antes", "ántes");
   let str42acc = &str41acc.replace("ente ", "énte ");
   let str43acc = &str42acc.replace("ente.", "énte.");
   let str44acc = &str43acc.replace("ente,", "énte,");
   let str45acc = &str44acc.replace("entes", "éntes");
   let str46acc = &str45acc.replace("aste ", "áste ");
   let str47acc = &str46acc.replace("aste.", "áste.");
   let str48acc = &str47acc.replace("aste,", "áste,");
   let str49acc = &str48acc.replace("ad ", "ád ");
   let str50acc = &str49acc.replace("ad.", "ád.");
   let str51acc = &str50acc.replace("ad,", "ád,");
   let str52acc = &str51acc.replace("al ", "ál ");
   let str53acc = &str52acc.replace("al.", "ál.");
   let str54acc = &str53acc.replace("al,", "ál,");
   let str55acc = &str54acc.replace("ambre", "ámbre");
   let str56acc = &str55acc.replace("ano ", "áno ");
   let str57acc = &str56acc.replace("ano.", "áno.");
   let str58acc = &str57acc.replace("ano,", "áno,");
   let str59acc = &str58acc.replace("anos ", "ános ");
   let str60acc = &str59acc.replace("anos.", "ános.");
   let str61acc = &str60acc.replace("anos,", "ános,");
   let str62acc = &str61acc.replace("ano", "áno");
   let str63acc = &str62acc.replace("éáno", "éano");
   let str64acc = &str63acc.replace("átáno", "átano");
   let str65acc = &str64acc.replace("ara ", "ára ");
   let str66acc = &str65acc.replace("ara.", "ára.");
   let str67acc = &str66acc.replace("ara,", "ára,");
   let str68acc = &str67acc.replace("aras ", "áras ");
   let str69acc = &str68acc.replace("aras.", "áras.");
   let str70acc = &str69acc.replace("aras,", "áras,");
   let str71acc = &str70acc.replace("aria", "ária");
   let str72acc = &str71acc.replace("ario", "ário");
   let str73acc = &str72acc.replace("aron ", "áron ");
   let str74acc = &str73acc.replace("aron.", "áron.");
   let str75acc = &str74acc.replace("aron,", "áron,");
   let str76acc = &str75acc.replace("ato ", "áto ");
   let str77acc = &str76acc.replace("ato.", "áto.");
   let str78acc = &str77acc.replace("ato,", "áto,");
   let str79acc = &str78acc.replace("atos ", "átos ");
   let str80acc = &str79acc.replace("atos.", "átos.");
   let str81acc = &str80acc.replace("atos,", "átos,");
   let str82acc = &str81acc.replace("enso ", "énso ");
   let str83acc = &str82acc.replace("enso.", "énso.");
   let str84acc = &str83acc.replace("enso,", "énso,");
   let str85acc = &str84acc.replace("ensos", "énsos");
   let str86acc = &str85acc.replace("ento ", "énto ");
   let str87acc = &str86acc.replace("ento.", "énto.");
   let str88acc = &str87acc.replace("ento,", "énto,");
   let str89acc = &str88acc.replace("entos ", "éntos ");
   let str90acc = &str89acc.replace("entos.", "éntos.");
   let str91acc = &str90acc.replace("entos,", "éntos,");
   let str92acc = &str91acc.replace("er ", "ér ");
   let str93acc = &str92acc.replace("er.", "er.");
   let str94acc = &str93acc.replace("er,", "ér,");
   let str95acc = &str94acc.replace("étér", "éter");
   let str96acc = &str95acc.replace("erme ", "érme ");
   let str97acc = &str96acc.replace("erme.", "érme.");
   let str98acc = &str97acc.replace("erme,", "érme,");
   let str99acc = &str98acc.replace("erte ", "érte ");
   let str100acc = &str99acc.replace("erte.", "érte.");
   let str101acc = &str100acc.replace("erte,", "érte,");
   let str102acc = &str101acc.replace("erse ", "érse ");
   let str103acc = &str102acc.replace("erse.", "érse.");
   let str104acc = &str103acc.replace("erse,", "érse,");
   let str105acc = &str104acc.replace("ero ", "éro ");
   let str106acc = &str105acc.replace("ero.", "éro.");
   let str107acc = &str106acc.replace("ero,", "éro,");
   let str108acc = &str107acc.replace("eros ", "éros ");
   let str109acc = &str108acc.replace("eros.", "éros.");
   let str110acc = &str109acc.replace("eros,", "éros,");
   let str111acc = &str110acc.replace("eron", "éron");
   let str112acc = &str111acc.replace("esca ", "ésca ");
   let str113acc = &str112acc.replace("esca.", "ésca.");
   let str114acc = &str113acc.replace("esca,", "ésca,");
   let str115acc = &str114acc.replace("esco ", "ésco ");
   let str116acc = &str115acc.replace("esco.", "ésco.");
   let str117acc = &str116acc.replace("esco,", "ésco,");
   let str118acc = &str117acc.replace("esco,", "ésco,");
   let str119acc = &str118acc.replace("ella", "élla");
   let str120acc = &str119acc.replace("ello", "éllo");
   let str121acc = &str120acc.replace("odo ", "ódo ");
   let str122acc = &str121acc.replace("odo.", "ódo.");
   let str123acc = &str122acc.replace("odo,", "ódo,");
   let str124acc = &str123acc.replace("odos ", "ódos ");
   let str125acc = &str124acc.replace("odos.", "ódos.");
   let str126acc = &str125acc.replace("odos,", "ódos,");
   let str127acc = &str126acc.replace("ómódo", "ómodo");
   let str128acc = &str127acc.replace("omo ", "ómo ");
   let str129acc = &str128acc.replace("omo.", "ómo.");
   let str130acc = &str129acc.replace("omo,", "ómo,");
   let str131acc = &str130acc.replace("omos ", "ómos ");
   let str132acc = &str131acc.replace("omos.", "ómos.");
   let str133acc = &str132acc.replace("omos,", "ómos,");
   let str134acc = &str133acc.replace("ones ", "ónes ");
   let str135acc = &str134acc.replace("ones.", "ónes.");
   let str136acc = &str135acc.replace("ones,", "ónes,");
   let str137acc = &str136acc.replace("or ", "ór ");
   let str138acc = &str137acc.replace("or.", "ór.");   
   let str139acc = &str138acc.replace("or,", "ór,");   
   let str140acc = &str139acc.replace("ores", "óres");
   let str141acc = &str140acc.replace("oso", "óso");
   let str142acc = &str141acc.replace("osos", "ósos");
   let str143acc = &str142acc.replace("ósot", "osót");
   let str144acc = &str143acc.replace("osa ", "ósa ");
   let str145acc = &str144acc.replace("osa.", "ósa.");
   let str146acc = &str145acc.replace("osa,", "ósa,");
   let str147acc = &str146acc.replace("osas", "ósas");
   let str148acc = &str147acc.replace("otro", "ótro");
   let str149acc = &str148acc.replace("uela", "uéla");
   let str150acc = &str149acc.replace("uelo", "uéla");
   let str151acc = &str150acc.replace("eña ", "éña ");   
   let str152acc = &str151acc.replace("eña.", "éña.");
   let str153acc = &str152acc.replace("eña,", "éña,");
   let str154acc = &str153acc.replace("eñas ", "éñas ");
   let str155acc = &str154acc.replace("eñas.", "éñas.");
   let str156acc = &str155acc.replace("eñas,", "éñas,");
   let str157acc = &str156acc.replace("eño ", "éño ");
   let str158acc = &str157acc.replace("eño.", "éño.");
   let str159acc = &str158acc.replace("eño,", "éño,");
   let str160acc = &str159acc.replace("eños ", "éños ");
   let str161acc = &str160acc.replace("eños.", "éños.");
   let str162acc = &str161acc.replace("eños,", "éños,");
   let str163acc = &str162acc.replace("ozc", "ózc");
   let str1wrd = str163acc.replace ("méxic", "méjic"); // words
   let str1 = str1wrd.replace ("ch", "ʈ͡ʂʲ"); // main
   let str2 = &str1.replace("ce", "se");
   let str3 = &str2.replace("cé", "sé");
   let str4 = &str3.replace("ci", "ʒi");
   let str5 = &str4.replace("cí", "ʒí");
   let str6 = &str5.replace("c", "k");
   let str7 = &str6.replace("que", "ke");
   let str8 = &str7.replace("qué", "ké");
   let str9 = &str8.replace("qui", "ki");
   let str10 = &str9.replace("q", "k");
   let str11 = &str10.replace("ké", "kʰe̞");
   let str12 = &str11.replace("ke", "kʰe̞");
   let str13 = &str12.replace("kó", "kʰɔ̝");
   let str14 = &str13.replace("ko", "kʰɔ̝");
   let str15 = &str14.replace("x", "ks");
   let str16 = &str15.replace("h", "");
   let str17 = &str16.replace("j", "h");
   let str18 = &str17.replace("ge", "he");
   let str19 = &str18.replace("gé", "hé");
   let str20 = &str19.replace("gi", "hi");
   let str21 = &str20.replace("r", "ɾ");
   let str22 = &str21.replace("subɾ", "subr");
   let str23 = &str22.replace("abɾ", "abr");
   let str24 = &str23.replace("nɾ", "nr");
   let str25 = &str24.replace("lɾ", "lr");
   let str26 = &str25.replace("sɾ", "sr");
   let str27 = &str26.replace(" ɾ", " r");
   let str28 = &str27.replace(".ɾ", ".r");
   let str29 = &str28.replace("ɾɾ", "r");
   let str30 = &str29.replace("oɾ", "oɹ");
   let str31 = &str30.replace("ór", "óɹ");
   let str32 = &str31.replace("y ", "i ");
   let str33 = &str32.replace("y.", "i.");
   let str34 = &str33.replace("y,", "i,");
   let str35 = &str34.replace("y", "ɟ͡ʝ");
   let str36 = &str35.replace("lle", "d͡ʑe");
   let str37 = &str36.replace("ll", "ɟ͡ʝ");
   let str38 = &str37.replace("lʈ͡ʂʲ", "lʲʈ͡ʂʲ");
   let str39 = &str38.replace("lt", "l̪t");
   let str40 = &str39.replace("b", "β");
   let str41 = &str40.replace(" β", " b");
   let str42 = &str41.replace(".β", ".b");
   let str43 = &str42.replace("mβ", "ɱb̪");
   let str44 = &str43.replace("lβ", "lb̪");
   let str45 = &str44.replace("l b", "l b̪");
   let str46 = &str45.replace("v", "β̞");
   let str47 = &str46.replace("fg", "vg");
   let str48 = &str47.replace("fn", "vn");
   let str49 = &str48.replace("gue", "ge");
   let str50 = &str49.replace("gué", "gé");
   let str51 = &str50.replace("gui", "gi");
   let str52 = &str51.replace("guí", "gí");
   let str53 = &str52.replace("g", "ɣ");
   let str54 = &str53.replace("lɣ", "lɡ");
   let str55 = &str54.replace("nɣ", "nɡ");
   let str56 = &str55.replace("ád ", "á ");
   let str57 = &str56.replace("ád.", "á.");
   let str58 = &str57.replace("ád,", "á,");
   let str59 = &str58.replace("id ", "i ");
   let str60 = &str59.replace("id.", "i.");
   let str61 = &str60.replace("id,", "i,");
   let str62 = &str61.replace("d", "ð̞");
   let str63 = &str62.replace(" ð̞", " ð");
   let str64 = &str63.replace(".ð̞", ".ð");
   let str65 = &str64.replace("nð̞", "nð");
   let str66 = &str65.replace("lð̞", "ld");
   let str67 = &str66.replace("ð̞a ", "ʔa ");
   let str68 = &str67.replace("ð̞a.", "ʔa.");
   let str69 = &str68.replace("ð̞a,", "ʔa,");
   let str70 = &str69.replace("ð̞o ", "ʔo ");
   let str71 = &str70.replace("ð̞o.", "ʔo.");
   let str72 = &str71.replace("ð̞o,", "ʔo,");
   let str73 = &str72.replace("z", "s");
   let str74 = &str73.replace("s", "s̺");
   let str75 = &str74.replace("s̺ ", "s̬ ");
   let str76 = &str75.replace("s̺.", "s̬.");
   let str77 = &str76.replace("s̺,", "s̬,");
   let str78 = &str77.replace("es̬ ", "ez̺ ");
   let str79 = &str78.replace("es̬.", "ez̺.");
   let str80 = &str79.replace("es̬,", "ez̺,");
   let str81 = &str80.replace("s̺ɣ", "s̬ɣ");
   let str82 = &str81.replace("s̺k", "s̬k");
   let str83 = &str82.replace("s̺d", "s̬ð̞");
   let str84 = &str83.replace("as̺a", "as̬a");
   let str85 = &str84.replace("as̺o", "as̬o");
   let str86 = &str85.replace("as̺e", "as̬e");
   let str87 = &str86.replace("as̺i", "as̬i");
   let str88 = &str87.replace("as̺u", "as̬u");
   let str89 = &str88.replace("as̺á", "as̬á");
   let str90 = &str89.replace("as̺ó", "as̬ó");
   let str91 = &str90.replace("as̺é", "as̬é");
   let str92 = &str91.replace("as̺í", "as̬í");
   let str93 = &str92.replace("as̺ú", "as̬ú");
   let str94 = &str93.replace("ás̺a", "ás̬a");
   let str95 = &str94.replace("ás̺o", "ás̬o");
   let str96 = &str95.replace("ás̺e", "ás̬e");
   let str97 = &str96.replace("ás̺i", "ás̬i");
   let str98 = &str97.replace("ás̺u", "ás̬u");
   let str99 = &str98.replace("os̺a", "os̬a");
   let str100 = &str99.replace("os̺o", "os̬o");
   let str101 = &str100.replace("os̺e", "os̬e");
   let str102 = &str101.replace("os̺i", "os̬i");
   let str103 = &str102.replace("os̺u", "os̬u");
   let str104 = &str103.replace("os̺á", "os̬á");
   let str105 = &str104.replace("os̺ó", "os̬ó");
   let str106 = &str105.replace("os̺é", "os̬é");
   let str107 = &str106.replace("os̺í", "os̬í");
   let str108 = &str107.replace("os̺ú", "os̬ú");
   let str109 = &str108.replace("ós̺a", "ós̬a");
   let str110 = &str109.replace("ós̺o", "ós̬o");
   let str111 = &str110.replace("ós̺e", "ós̬e");
   let str112 = &str111.replace("ós̺i", "ós̬i");
   let str113 = &str112.replace("ós̺u", "ós̬u");
   let str114 = &str113.replace("es̺a", "es̬a");
   let str115 = &str114.replace("es̺o", "es̬o");
   let str116 = &str115.replace("es̺e", "es̬e");
   let str117 = &str116.replace("es̺i", "es̬i");
   let str118 = &str117.replace("es̺u", "es̬u");
   let str119 = &str118.replace("es̺á", "es̬á");
   let str120 = &str119.replace("es̺ó", "es̬ó");
   let str121 = &str120.replace("es̺é", "es̬é");
   let str122 = &str121.replace("es̺í", "es̬í");
   let str123 = &str122.replace("es̺ú", "es̬ú");
   let str124 = &str123.replace("és̺a", "és̬a");
   let str125 = &str124.replace("és̺o", "és̬o");
   let str126 = &str125.replace("és̺e", "és̬e");
   let str127 = &str126.replace("és̺i", "és̬i");
   let str128 = &str127.replace("és̺u", "és̬u");
   let str129 = &str128.replace("is̺a", "is̬a");
   let str130 = &str129.replace("is̺o", "is̬o");
   let str131 = &str130.replace("is̺e", "is̬e");
   let str132 = &str131.replace("is̺i", "is̬i");
   let str133 = &str132.replace("is̺u", "is̬u");
   let str134 = &str133.replace("is̺á", "is̬á");
   let str135 = &str134.replace("is̺ó", "is̬ó");
   let str136 = &str135.replace("is̺é", "is̬é");
   let str137 = &str136.replace("is̺í", "is̬í");
   let str138 = &str137.replace("is̺ú", "is̬ú");
   let str139 = &str138.replace("ís̺a", "ís̬a");
   let str140 = &str139.replace("ís̺o", "ís̬o");
   let str141 = &str140.replace("ís̺e", "ís̬e");
   let str142 = &str141.replace("ís̺i", "ís̬i");
   let str143 = &str142.replace("ís̺u", "ís̬u");
   let str144 = &str143.replace("us̺a", "us̬a");
   let str145 = &str144.replace("us̺o", "us̬o");
   let str146 = &str145.replace("us̺e", "us̬e");
   let str147 = &str146.replace("us̺i", "us̬i");
   let str148 = &str147.replace("us̺u", "us̬u");
   let str149 = &str148.replace("us̺á", "us̬á");
   let str150 = &str149.replace("us̺ó", "us̬ó");
   let str151 = &str150.replace("us̺é", "us̬é");
   let str152 = &str151.replace("us̺í", "us̬í");
   let str153 = &str152.replace("us̺ú", "us̬ú");
   let str154 = &str153.replace("ús̺a", "ús̬a");
   let str155 = &str154.replace("ús̺o", "ús̬o");
   let str156 = &str155.replace("ús̺e", "ús̬e");
   let str157 = &str156.replace("ús̺i", "ús̬i");
   let str158 = &str157.replace("ús̺u", "ús̬u");
   let str159 = &str158.replace("ks̺s̺e", "ks̺e");
   let str160 = &str159.replace("ks̺s̺i", "ks̺i");
   let str161 = &str160.replace("mn", "nn");
   let str162 = &str161.replace("nʈ͡ʂʲ", "nʲʈ͡ʂʲ");
   let str163 = &str162.replace("t", "t̪");
   let str164 = &str163.replace("nt̪", "n̪t̪");
   let str165 = &str164.replace("nð", "n̪ð");
   let str166 = &str165.replace("nk", "ŋk");
   let str167 = &str166.replace("nɣ̞", "ŋɣ̞");
   let str168 = &str167.replace("n ", "ŋ ");
   let str169 = &str168.replace("n.", "ŋ.");
   let str170 = &str169.replace("n,", "ŋ,");
   let str171 = &str170.replace("nm", "m̚m");
   let str172 = &str171.replace("n m", "m m");
   let str173 = &str172.replace("nβ", "mβ");
   let str174 = &str173.replace("n β", "m β");
   let str175 = &str174.replace("np", "mp");
   let str176 = &str175.replace("n p", "m p");
   let str177 = &str176.replace("nf", "mf");
   let str178 = &str177.replace("n f", "m f");
   let str179 = &str178.replace("ñ", "ɲ");
   let str180 = &str179.replace("f", "ɸ");
   let str181 = &str180.replace("í.", "i.");
   let str182 = &str181.replace("ia", "ja");
   let str183 = &str182.replace("ie", "je");
   let str184 = &str183.replace("io", "jo");
   let str185 = &str184.replace("iu", "ju");
   let str186 = &str185.replace("iá", "já");
   let str187 = &str186.replace("ié", "jé");
   let str188 = &str187.replace("ió", "jó");
   let str189 = &str188.replace("iú", "jú");
   let str190 = &str189.replace("ea", "ia");
   let str191 = &str190.replace("eá", "iá");
   let str192 = &str191.replace("ká", "kɑ");
   let str193 = &str192.replace("ka", "kɑ");
   let str194 = &str193.replace("á", "ɐ̞");
   let str195 = &str194.replace("a", "ɐ");
   let str196 = &str195.replace("ʝɐ̞", "ɟ͡ʝɐ̞");
   let str197 = &str196.replace("ó", "o̞");
   let str198 = &str197.replace("o ", "ʊ ");
   let str199 = &str198.replace("o.", "ʊ.");
   let str200 = &str199.replace("o,", "ʊ,");
   let str201 = &str200.replace("os̬ ", "ʊs̬ ");
   let str202 = &str201.replace("os̬.", "ʊs̬.");
   let str203 = &str202.replace("os̬,", "ʊs̬,");
   let str204 = &str203.replace("lʊ ", "lo ");
   let str205 = &str204.replace("lʊ.", "lo.");
   let str206 = &str205.replace("lʊ,", "lo,");
   let str207 = &str206.replace("lʊs̬ ", "los̬ ");
   let str208 = &str207.replace("lʊs̬.", "los̬.");
   let str209 = &str208.replace("lʊs̬,", "los̬,");
   let str210 = &str209.replace("ɾʊ ", "ɾo ");
   let str211 = &str210.replace("ɾʊ.", "ɾo.");
   let str212 = &str211.replace("ɾʊ,", "ɾo,");
   let str213 = &str212.replace("ɾʊs̬ ", "ɾos̬ ");
   let str214 = &str213.replace("ɾʊs̬.", "ɾos̬.");
   let str215 = &str214.replace("ɾʊs̬,", "ɾos̬,");
   let str216 = &str215.replace("ɡʊ ", "ɡo̞ ");
   let str217 = &str216.replace("ɡʊ.", "ɡo̞.");
   let str218 = &str217.replace("ɡʊ,", "ɡo̞,");
   let str219 = &str218.replace("ɡʊs̬ ", "ɡo̞s̬ ");
   let str220 = &str219.replace("ɡʊs̬.", "ɡo̞s̬.");
   let str221 = &str220.replace("ɡʊs̬,", "ɡo̞s̬,");
   let str222 = &str221.replace("ɣʊ ", "ɣo̞ ");
   let str223 = &str222.replace("ɣʊ.", "ɣo̞.");
   let str224 = &str223.replace("ɣʊ,", "ɣo̞,");
   let str225 = &str224.replace("ɣʊs̬ ", "ɣo̞s̬ ");
   let str226 = &str225.replace("ɣʊs̬.", "ɣo̞s̬.");
   let str227 = &str226.replace("ɣʊs̬,", "ɣo̞s̬,");
   let str228 = &str227.replace("mʊ ", "mo̞ ");
   let str229 = &str228.replace("mʊ.", "mo̞.");
   let str230 = &str229.replace("mʊ,", "mo̞,");
   let str231 = &str230.replace("mʊs̬ ", "mo̞s̬ ");
   let str232 = &str231.replace("mʊs̬.", "mo̞s̬.");
   let str233 = &str232.replace("mʊs̬,", "mo̞s̬,");
   let str234 = &str233.replace("nʊ ", "no̞ ");
   let str235 = &str234.replace("nʊ.", "no̞.");
   let str236 = &str235.replace("nʊ,", "no̞,");
   let str237 = &str236.replace("nʊs̬ ", "no̞s̬ ");
   let str238 = &str237.replace("nʊs̬.", "no̞s̬.");
   let str239 = &str238.replace("nʊs̬,", "no̞s̬,");
   let str240 = &str239.replace("ɲʊ ", "ɲo̞ ");
   let str241 = &str240.replace("ɲʊ.", "ɲo̞.");
   let str242 = &str241.replace("ɲʊ,", "ɲo̞,");
   let str243 = &str242.replace("ɲʊs̬ ", "ɲo̞s̬ ");
   let str244 = &str243.replace("ɲʊs̬.", "ɲo̞s̬.");
   let str245 = &str244.replace("ɲʊs̬,", "ɲo̞s̬,");
   let str246 = &str245.replace("é", "e̞");
   let str247 = &str246.replace("í", "i");
   let str248 = &str247.replace("ú", "u");
   let str249 = &str248.replace("ɟ͡ʝʊ", "ɟ͡ʝö");
   let str1nas = &str249.replace("o̞n", "õ̞n"); // nasalizations
   let str2nas = &str1nas.replace("o̞m ", "õ̞m ");
   let str3nas = &str2nas.replace("ɐ̞m̚m", "ɐ̞̃m̚m");
   let str4nas = &str3nas.replace("ɔ̝m̚m", "ɔ̝̃m̚m");
   let str5nas = &str4nas.replace("em̚m", "ẽm̚m");
   let str6nas = &str5nas.replace("im̚m", "ĩm̚m");
   let str7nas = &str6nas.replace("um̚m", "ũm̚m");
   let str8nas = &str7nas.replace("ɐ̞nn", "ɐ̞̃nn");
   let str9nas = &str8nas.replace("onn", "õnn");
   let str10nas = &str9nas.replace("enn", "ẽnn");
   let str11nas = &str10nas.replace("inn", "ĩnn");
   let str12nas = &str11nas.replace("unn", "ũnn");
   let str13nas = &str12nas.replace("ɐ̞ɱ", "ɐ̞̃ɱ");
   let str14nas = &str13nas.replace("oɱ", "õ̞ɱ");
   let str15nas = &str14nas.replace("eɱ", "ẽɱ");
   let str16nas = &str15nas.replace("ɐɱ", "ɐ̃ɱ");
   let str17nas = &str16nas.replace("ɑɱ", "ɑ̃ɱ");
   let str18nas = &str17nas.replace("o̞ɱ", "õ̞ɱ");
   let str19nas = &str18nas.replace("ɔ̝ɱ", "ɔ̝̃ɱ");
   let str20nas = &str19nas.replace("e̞ɱ", "ẽ̞ɱ");
   let str21nas = &str20nas.replace("iɱ", "ĩɱ");
   let str22nas = &str21nas.replace("uɱ", "ũɱ");
   let str23nas = &str22nas.replace("ɐ̞ŋ", "ɐ̞̃ŋ");
   let str24nas = &str23nas.replace("oŋ", "õŋ");
   let str25nas = &str24nas.replace("eŋ", "ẽŋ");
   let str26nas = &str25nas.replace("ɐŋ", "ɐ̃ŋ");
   let str27nas = &str26nas.replace("ɑŋ", "ɑ̃ŋ");
   let str28nas = &str27nas.replace("o̞ŋ", "õ̞ŋ");
   let str29nas = &str28nas.replace("ɔ̝ŋ", "ɔ̝̃ŋ");
   let str30nas = &str29nas.replace("e̞ŋ", "ẽ̞ŋ");
   let str31nas = &str30nas.replace("iŋ", "ĩŋ");
   let str32nas = &str31nas.replace("uŋ", "ũŋ");
   let str33nas = &str32nas.replace("ɐ̞n̪", "ɐ̞̃n̪");
   let str34nas = &str33nas.replace("ɑn̪", "ɑ̃n̪");
   let str35nas = &str34nas.replace("on̪", "õn̪");
   let str36nas = &str35nas.replace("en̪", "ẽn̪");
   let str37nas = &str36nas.replace("ɐn̪", "ɐ̃n̪");
   let str38nas = &str37nas.replace("o̞n̪", "õ̞n̪");
   let str39nas = &str38nas.replace("ɔ̝n̪", "ɔ̝̃n̪");
   let str40nas = &str39nas.replace("e̞n̪", "ẽ̞n̪");
   let str41nas = &str40nas.replace("in̪", "ĩn̪");
   let str42nas = &str41nas.replace("un̪", "ũn̪");
   let str43nas = &str42nas.replace("ɐnʲ", "ɐ̃nʲ");
   let str44nas = &str43nas.replace("ɐ̞nʲ", "ɐ̞̃nʲ");
   let str45nas = &str44nas.replace("ɑnʲ", "ɑ̃nʲ");
   let str46nas = &str45nas.replace("onʲ", "õnʲ");
   let str47nas = &str46nas.replace("o̞nʲ", "õ̞nʲ");
   let str48nas = &str47nas.replace("ɔ̝nʲ", "ɔ̝̃nʲ");
   let str49nas = &str48nas.replace("enʲ", "ẽnʲ");
   let str50nas = &str49nas.replace("e̞nʲ", "ẽ̞nʲ");
   let str51nas = &str50nas.replace("inʲ", "ĩnʲ");
   let str52nas = &str51nas.replace("unʲ", "ũnʲ");
   let str1pnc = &str52nas.replace(",", " ∣"); // space
   let str2pnc = &str1pnc.replace(". ", " ∥ ");
   let str3pnc = &str2pnc.replace(".", "");
   let str4pnc = &str3pnc.replace(" - ", " ∣ ");
   let str5pnc = &str4pnc.replace(" – ", " ∣ ");
   let str6pnc = &str5pnc.replace("--", " ∣ ");
   let result = &str6pnc.replace("∣ ∥", "∥");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("MEDELLÍN, CO:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("MEDELLÍN, CO:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Medellín, CO:");
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// SANTA MARTA: IPA

pub fn spacosantamarta(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str1dot = original_text.to_owned() + "."; // mark word ending
   let str2dot = ".".to_owned() + &str1dot;
   let str3low = &str2dot.to_lowercase();
   let str1sgn = str3low.replace(";", ",");
   let str2sgn = str1sgn.replace(":", ",");
   let str3sgn = str2sgn.replace("!", ".");
   let str4sgn = str3sgn.replace("?", ".");
   let str1acc = &str4sgn.replace("", "");
   let str1 = str1acc.replace ("", "");
   let result = &str1.replace("", "");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("SANTA MARTA, CO: [NOT YET IMPLEMENTED]".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("SANTA MARTA, CO: [NOT YET IMPLEMENTED]".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Santa Marta, CO:{}", red.to_owned() + " [not yet implemented]" + reset);
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// CÁDIZ: IPA

pub fn spaescadiz(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str1dot = original_text.to_owned() + "."; // mark word ending
   let str2dot = ".".to_owned() + &str1dot;
   let str3low = &str2dot.to_lowercase();
   let str1sgn = str3low.replace(";", ",");
   let str2sgn = str1sgn.replace(":", ",");
   let str3sgn = str2sgn.replace("!", ".");
   let str4sgn = str3sgn.replace("?", ".");
   let str1acc = &str4sgn.replace("", "");
   let str1 = str1acc.replace ("", "");
   let result = &str1.replace("", "");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("CÁDIZ, ES: [NOT YET IMPLEMENTED]".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("CÁDIZ, ES: [NOT YET IMPLEMENTED]".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Cádiz, ES:{}", red.to_owned() + " [not yet implemented]" + reset);
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// MADRID: IPA

pub fn spaesmadrid(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str1dot = original_text.to_owned() + "."; // mark word ending
   let str2dot = ".".to_owned() + &str1dot; // word beginning
   let str3low = &str2dot.to_lowercase();
   let str1sgn = str3low.replace(";", "."); // simplification
   let str2sgn = str1sgn.replace(":", ",");
   let str3sgn = str2sgn.replace("!", ".");
   let str4sgn = str3sgn.replace("?", ".");
   let str5sgn = str4sgn.replace("¡", "."); // word beginning
   let str6sgn = str5sgn.replace("¿", "."); // word beginning
   let str7sgn = str6sgn.replace(")", ",");
   let str8sgn = &str7sgn.replace("(", "∣ .");
   let str1acc = &str8sgn.replace("ar ", "ár "); // word stress
   let str2acc = &str1acc.replace("ar.", "ár.");
   let str3acc = &str2acc.replace("ar,", "ár,");
   let str4acc = &str3acc.replace("arme ", "árme ");
   let str5acc = &str4acc.replace("arme.", "árme.");
   let str6acc = &str5acc.replace("arme,", "árme,");
   let str7acc = &str6acc.replace("arte ", "árte ");
   let str8acc = &str7acc.replace("arte.", "árte.");
   let str9acc = &str8acc.replace("arte,", "árte,");
   let str10acc = &str9acc.replace("arse ", "árse ");
   let str11acc = &str10acc.replace("arse.", "árse.");
   let str12acc = &str11acc.replace("arse,", "árse,");
   let str13acc = &str12acc.replace("iendo", "iéndo");
   let str14acc = &str13acc.replace("ando ", "ándo ");
   let str15acc = &str14acc.replace("ando.", "ándo.");
   let str16acc = &str15acc.replace("ando,", "ándo,");
   let str17acc = &str16acc.replace("aba ", "ába ");
   let str18acc = &str17acc.replace("aba.", "ába.");
   let str19acc = &str18acc.replace("aba,", "ába,");
   let str20acc = &str19acc.replace("aban ", "ában ");
   let str21acc = &str20acc.replace("aban.", "ában.");
   let str22acc = &str21acc.replace("aban,", "ában,");
   let str23acc = &str22acc.replace("abas ", "ábas ");
   let str24acc = &str23acc.replace("abas.", "ábas.");
   let str25acc = &str24acc.replace("abas,", "ábas,");
   let str26acc = &str25acc.replace("acha ", "ácha ");
   let str27acc = &str26acc.replace("acha.", "ácha.");
   let str28acc = &str27acc.replace("acha,", "ácha,");
   let str29acc = &str28acc.replace("acho", "ácho");
   let str30acc = &str29acc.replace("ada ", "áda ");
   let str31acc = &str30acc.replace("ada.", "áda.");
   let str32acc = &str31acc.replace("ada,", "áda,");
   let str33acc = &str32acc.replace("adas", "ádas");
   let str34acc = &str33acc.replace("ado ", "ádo ");
   let str35acc = &str34acc.replace("ado.", "ádo.");
   let str36acc = &str35acc.replace("ado,", "ádo,");
   let str37acc = &str36acc.replace("ados", "ádos");
   let str38acc = &str37acc.replace("ante ", "ánte ");
   let str39acc = &str38acc.replace("ante.", "ánte.");
   let str40acc = &str39acc.replace("ante,", "ánte,");
   let str41acc = &str40acc.replace("antes", "ántes");
   let str42acc = &str41acc.replace("ente ", "énte ");
   let str43acc = &str42acc.replace("ente.", "énte.");
   let str44acc = &str43acc.replace("ente,", "énte,");
   let str45acc = &str44acc.replace("entes", "éntes");
   let str46acc = &str45acc.replace("aste ", "áste ");
   let str47acc = &str46acc.replace("aste.", "áste.");
   let str48acc = &str47acc.replace("aste,", "áste,");
   let str49acc = &str48acc.replace("ad ", "ád ");
   let str50acc = &str49acc.replace("ad.", "ád.");
   let str51acc = &str50acc.replace("ad,", "ád,");
   let str52acc = &str51acc.replace("al ", "ál ");
   let str53acc = &str52acc.replace("al.", "ál.");
   let str54acc = &str53acc.replace("al,", "ál,");
   let str55acc = &str54acc.replace("ambre", "ámbre");
   let str56acc = &str55acc.replace("ano ", "áno ");
   let str57acc = &str56acc.replace("ano.", "áno.");
   let str58acc = &str57acc.replace("ano,", "áno,");
   let str59acc = &str58acc.replace("anos ", "ános ");
   let str60acc = &str59acc.replace("anos.", "ános.");
   let str61acc = &str60acc.replace("anos,", "ános,");
   let str62acc = &str61acc.replace("ano", "áno");
   let str63acc = &str62acc.replace("éáno", "éano");
   let str64acc = &str63acc.replace("átáno", "átano");
   let str65acc = &str64acc.replace("ara ", "ára ");
   let str66acc = &str65acc.replace("ara.", "ára.");
   let str67acc = &str66acc.replace("ara,", "ára,");
   let str68acc = &str67acc.replace("aras ", "áras ");
   let str69acc = &str68acc.replace("aras.", "áras.");
   let str70acc = &str69acc.replace("aras,", "áras,");
   let str71acc = &str70acc.replace("aria", "ária");
   let str72acc = &str71acc.replace("ario", "ário");
   let str73acc = &str72acc.replace("aron ", "áron ");
   let str74acc = &str73acc.replace("aron.", "áron.");
   let str75acc = &str74acc.replace("aron,", "áron,");
   let str76acc = &str75acc.replace("ato ", "áto ");
   let str77acc = &str76acc.replace("ato.", "áto.");
   let str78acc = &str77acc.replace("ato,", "áto,");
   let str79acc = &str78acc.replace("atos ", "átos ");
   let str80acc = &str79acc.replace("atos.", "átos.");
   let str81acc = &str80acc.replace("atos,", "átos,");
   let str82acc = &str81acc.replace("enso ", "énso ");
   let str83acc = &str82acc.replace("enso.", "énso.");
   let str84acc = &str83acc.replace("enso,", "énso,");
   let str85acc = &str84acc.replace("ensos", "énsos");
   let str86acc = &str85acc.replace("ento ", "énto ");
   let str87acc = &str86acc.replace("ento.", "énto.");
   let str88acc = &str87acc.replace("ento,", "énto,");
   let str89acc = &str88acc.replace("entos ", "éntos ");
   let str90acc = &str89acc.replace("entos.", "éntos.");
   let str91acc = &str90acc.replace("entos,", "éntos,");
   let str92acc = &str91acc.replace("er ", "ér ");
   let str93acc = &str92acc.replace("er.", "er.");
   let str94acc = &str93acc.replace("er,", "ér,");
   let str95acc = &str94acc.replace("étér", "éter");
   let str96acc = &str95acc.replace("erme ", "érme ");
   let str97acc = &str96acc.replace("erme.", "érme.");
   let str98acc = &str97acc.replace("erme,", "érme,");
   let str99acc = &str98acc.replace("erte ", "érte ");
   let str100acc = &str99acc.replace("erte.", "érte.");
   let str101acc = &str100acc.replace("erte,", "érte,");
   let str102acc = &str101acc.replace("erse ", "érse ");
   let str103acc = &str102acc.replace("erse.", "érse.");
   let str104acc = &str103acc.replace("erse,", "érse,");
   let str105acc = &str104acc.replace("ero ", "éro ");
   let str106acc = &str105acc.replace("ero.", "éro.");
   let str107acc = &str106acc.replace("ero,", "éro,");
   let str108acc = &str107acc.replace("eros ", "éros ");
   let str109acc = &str108acc.replace("eros.", "éros.");
   let str110acc = &str109acc.replace("eros,", "éros,");
   let str111acc = &str110acc.replace("eron", "éron");
   let str112acc = &str111acc.replace("esca ", "ésca ");
   let str113acc = &str112acc.replace("esca.", "ésca.");
   let str114acc = &str113acc.replace("esca,", "ésca,");
   let str115acc = &str114acc.replace("esco ", "ésco ");
   let str116acc = &str115acc.replace("esco.", "ésco.");
   let str117acc = &str116acc.replace("esco,", "ésco,");
   let str118acc = &str117acc.replace("esco,", "ésco,");
   let str119acc = &str118acc.replace("ella", "élla");
   let str120acc = &str119acc.replace("ello", "éllo");
   let str121acc = &str120acc.replace("odo ", "ódo ");
   let str122acc = &str121acc.replace("odo.", "ódo.");
   let str123acc = &str122acc.replace("odo,", "ódo,");
   let str124acc = &str123acc.replace("odos ", "ódos ");
   let str125acc = &str124acc.replace("odos.", "ódos.");
   let str126acc = &str125acc.replace("odos,", "ódos,");
   let str127acc = &str126acc.replace("ómódo", "ómodo");
   let str128acc = &str127acc.replace("omo ", "ómo ");
   let str129acc = &str128acc.replace("omo.", "ómo.");
   let str130acc = &str129acc.replace("omo,", "ómo,");
   let str131acc = &str130acc.replace("omos ", "ómos ");
   let str132acc = &str131acc.replace("omos.", "ómos.");
   let str133acc = &str132acc.replace("omos,", "ómos,");
   let str134acc = &str133acc.replace("ones ", "ónes ");
   let str135acc = &str134acc.replace("ones.", "ónes.");
   let str136acc = &str135acc.replace("ones,", "ónes,");
   let str137acc = &str136acc.replace("or ", "ór ");
   let str138acc = &str137acc.replace("or.", "ór.");   
   let str139acc = &str138acc.replace("or,", "ór,");   
   let str140acc = &str139acc.replace("ores", "óres");
   let str141acc = &str140acc.replace("oso", "óso");
   let str142acc = &str141acc.replace("osos", "ósos");
   let str143acc = &str142acc.replace("ósot", "osót");
   let str144acc = &str143acc.replace("osa ", "ósa ");
   let str145acc = &str144acc.replace("osa.", "ósa.");
   let str146acc = &str145acc.replace("osa,", "ósa,");
   let str147acc = &str146acc.replace("osas", "ósas");
   let str148acc = &str147acc.replace("otro", "ótro");
   let str149acc = &str148acc.replace("uela", "uéla");
   let str150acc = &str149acc.replace("uelo", "uéla");
   let str151acc = &str150acc.replace("eña ", "éña ");   
   let str152acc = &str151acc.replace("eña.", "éña.");
   let str153acc = &str152acc.replace("eña,", "éña,");
   let str154acc = &str153acc.replace("eñas ", "éñas ");
   let str155acc = &str154acc.replace("eñas.", "éñas.");
   let str156acc = &str155acc.replace("eñas,", "éñas,");
   let str157acc = &str156acc.replace("eño ", "éño ");
   let str158acc = &str157acc.replace("eño.", "éño.");
   let str159acc = &str158acc.replace("eño,", "éño,");
   let str160acc = &str159acc.replace("eños ", "éños ");
   let str161acc = &str160acc.replace("eños.", "éños.");
   let str162acc = &str161acc.replace("eños,", "éños,");
   let str163acc = &str162acc.replace("ozc", "ózc");
   let str1wrd = str163acc.replace ("méxic", "méjic"); // words
   let str1 = str1wrd.replace ("ch", "t͡ʃ"); // main
   let str2 = &str1.replace("ce", "θe");
   let str3 = &str2.replace("cé", "θé");
   let str4 = &str3.replace("ci", "θi");
   let str5 = &str4.replace("cí", "θí");
   let str6 = &str5.replace("c", "k");
   let str7 = &str6.replace("que", "ke");
   let str8 = &str7.replace("qué", "ké");
   let str9 = &str8.replace("qui", "ki");
   let str10 = &str9.replace("q", "k");
   let str11 = &str10.replace("x", "ks");
   let str12 = &str11.replace("j", "x");
   let str13 = &str12.replace("ge", "xe");
   let str14 = &str13.replace("gé", "xé");
   let str15 = &str14.replace("gi", "χi");
   let str16 = &str15.replace("xi", "χi");
   let str17 = &str16.replace("ix", "iχ");
   let str18 = &str17.replace("xu", "χu");
   let str19 = &str18.replace("ux", "uχ");
   let str20 = &str19.replace("r", "ɾ");
   let str21 = &str20.replace("subɾ", "subr");
   let str22 = &str21.replace("abɾ", "abr");
   let str23 = &str22.replace("nɾ", "nr");
   let str24 = &str23.replace("lɾ", "lr");
   let str25 = &str24.replace("sɾ", "sr");
   let str26 = &str25.replace(" ɾ", " r");
   let str27 = &str26.replace(".ɾ", ".r");
   let str28 = &str27.replace("ɾɾ", "r");
   let str29 = &str28.replace("oɾ", "oɹ");
   let str30 = &str29.replace("ór", "óɹ");
   let str31 = &str30.replace("y ", "i ");
   let str32 = &str31.replace("y.", "i.");
   let str33 = &str32.replace("y,", "i,");
   let str34 = &str33.replace("y", "ʝ");
   let str35 = &str34.replace("ll", "j");
   let str36 = &str35.replace("ld", "l̪d");
   let str37 = &str36.replace("lt͡ʃ", "lʲt͡ʃ");
   let str38 = &str37.replace("lt", "l̪t");
   let str39 = &str38.replace("lz", "l̟z");
   let str40 = &str39.replace("lk", "ʟk");
   let str41 = &str40.replace("lg", "ʟg");
   let str42 = &str41.replace("lx", "ʟx");
   let str43 = &str42.replace("b", "β̞");
   let str44 = &str43.replace(" β̞", " β");
   let str45 = &str44.replace(".β̞", ".β");
   let str46 = &str45.replace("mβ̞", "mβ");
   let str47 = &str46.replace("v", "β̞");
   let str48 = &str47.replace("fg", "vg");
   let str49 = &str48.replace("fn", "vn");
   let str50 = &str49.replace("gue", "ge");
   let str51 = &str50.replace("gué", "gé");
   let str52 = &str51.replace("gui", "gi");
   let str53 = &str52.replace("guí", "gí");
   let str54 = &str53.replace("g", "ɣ̞");
   let str55 = &str54.replace("nɣ̞", "nɣ");
   let str56 = &str55.replace("ád ", "á ");
   let str57 = &str56.replace("ád.", "á.");
   let str58 = &str57.replace("ád,", "á,");
   let str59 = &str58.replace("id ", "iθ ");
   let str60 = &str59.replace("id.", "iθ.");
   let str61 = &str60.replace("id,", "iθ,");
   let str62 = &str61.replace("d", "ð̞");
   let str63 = &str62.replace(" ð̞", " ð");
   let str64 = &str63.replace(".ð̞", ".ð");
   let str65 = &str64.replace("nð̞", "nð");
   let str66 = &str65.replace("l̪ð̞", "l̪ð");
   let str67 = &str66.replace("s", "s̺");
   let str68 = &str67.replace("s̺ ", "s̬ ");
   let str69 = &str68.replace("s̺.", "s̬.");
   let str70 = &str69.replace("s̺,", "s̬,");
   let str71 = &str70.replace("s̺ɣ̞", "s̬ɣ̞");
   let str72 = &str71.replace("s̺k", "xk");
   let str73 = &str72.replace("zɣ̞", "ðɣ̞");
   let str74 = &str73.replace("z", "θ");
   let str75 = &str74.replace("s̺θ", "s̺");
   let str76 = &str75.replace("ks̺θ", "ks̺");
   let str77 = &str76.replace("mn", "nn");
   let str78 = &str77.replace("nt͡ʃ", "nʲt͡ʃ");
   let str79 = &str78.replace("t", "t̪");
   let str80 = &str79.replace("nt̪", "n̪t̪");
   let str81 = &str80.replace("nð", "n̪ð");
   let str82 = &str81.replace("nk", "ŋk");
   let str83 = &str82.replace("nɣ̞", "ŋɣ̞");
   let str84 = &str83.replace("nx", "ŋx");
   let str85 = &str84.replace("nθ", "n̟θ");
   let str86 = &str85.replace("nχ", "ɴχ");
   let str87 = &str86.replace("nm", "m̚m");
   let str88 = &str87.replace("n m", "m m");
   let str89 = &str88.replace("nβ", "mβ");
   let str90 = &str89.replace("n β", "m β");
   let str91 = &str90.replace("np", "mp");
   let str92 = &str91.replace("n p", "m p");
   let str93 = &str92.replace("nf", "ɱf");
   let str94 = &str93.replace("n f", "ɱ f");
   let str95 = &str94.replace("ñ", "ɲ");
   let str96 = &str95.replace("h", "");
   let str97 = &str96.replace("í.", "i.");
   let str98 = &str97.replace("ia", "ja");
   let str99 = &str98.replace("ie", "je");
   let str100 = &str99.replace("io", "jo");
   let str101 = &str100.replace("iu", "ju");
   let str102 = &str101.replace("iá", "já");
   let str103 = &str102.replace("ié", "jé");
   let str104 = &str103.replace("ió", "jó");
   let str105 = &str104.replace("iú", "jú");
   let str106 = &str105.replace("á", "ɐ̞");
   let str107 = &str106.replace("a", "ɐ");
   let str108 = &str107.replace("ʝɐ̞", "ɟ͡ʝɐ̞");
   let str109 = &str108.replace("ó", "ɔ");
   let str110 = &str109.replace("o", "ɔ̝");
   let str111 = &str110.replace("é", "ɛ");
   let str112 = &str111.replace("e", "ɛ̝");
   let str113 = &str112.replace("í", "i");
   let str114 = &str113.replace("ú", "u");
   let str1nas = &str114.replace("ɔn", "ɔ̃n"); // nasalizations
   let str2nas = &str1nas.replace("ɔm ", "ɔ̃m ");
   let str3nas = &str2nas.replace("mu", "mũ");
   let str4nas = &str3nas.replace("ɐ̞m̚m", "ɐ̞̃m̚m");
   let str5nas = &str4nas.replace("ɔ̝m̚m", "ɔ̝̃m̚m");
   let str6nas = &str5nas.replace("ɛ̝m̚m", "ɛ̝̃m̚m");
   let str7nas = &str6nas.replace("im̚m", "ĩm̚m");
   let str8nas = &str7nas.replace("um̚m", "ũm̚m");
   let str9nas = &str8nas.replace("ɐ̞nn", "ɐ̞̃nn");
   let str10nas = &str9nas.replace("ɔ̝nn", "ɔ̝̃nn");
   let str11nas = &str10nas.replace("ɛ̝nn", "ɛ̝̃nn");
   let str12nas = &str11nas.replace("inn", "ĩnn");
   let str13nas = &str12nas.replace("unn", "ũnn");
   let str14nas = &str13nas.replace("ɐ̞ɱ", "ɐ̞̃ɱ");
   let str15nas = &str14nas.replace("ɔ̝ɱ", "ɔ̝̃ɱ");
   let str16nas = &str15nas.replace("ɛ̝ɱ", "ɛ̝̃ɱ");
   let str17nas = &str16nas.replace("ɐɱ", "ɐ̃ɱ");
   let str18nas = &str17nas.replace("ɔɱ", "ɔ̃ɱ");
   let str19nas = &str18nas.replace("ɛɱ", "ɛ̃ɱ");
   let str20nas = &str19nas.replace("iɱ", "ĩɱ");
   let str21nas = &str20nas.replace("uɱ", "ũɱ");
   let str22nas = &str21nas.replace("ɐ̞ŋ", "ɐ̞̃ŋ");
   let str23nas = &str22nas.replace("ɔ̝ŋ", "ɔ̝̃ŋ");
   let str24nas = &str23nas.replace("ɛ̝ŋ", "ɛ̝̃ŋ");
   let str25nas = &str24nas.replace("ɐŋ", "ɐ̃ŋ");
   let str26nas = &str25nas.replace("ɔŋ", "ɔ̃ŋ");
   let str27nas = &str26nas.replace("ɛŋ", "ɛ̃ŋ");
   let str28nas = &str27nas.replace("iŋ", "ĩŋ");
   let str29nas = &str28nas.replace("uŋ", "ũŋ");
   let str30nas = &str29nas.replace("ɐ̞n̪", "ɐ̞̃n̪");
   let str31nas = &str30nas.replace("ɔ̝n̪", "ɔ̝̃n̪");
   let str32nas = &str31nas.replace("ɛ̝n̪", "ɛ̝̃n̪");
   let str33nas = &str32nas.replace("ɐn̪", "ɐ̃n̪");
   let str34nas = &str33nas.replace("ɔn̪", "ɔ̃n̪");
   let str35nas = &str34nas.replace("ɛn̪", "ɛ̃n̪");
   let str36nas = &str35nas.replace("in̪", "ĩn̪");
   let str37nas = &str36nas.replace("un̪", "ũn̪");
   let str38nas = &str37nas.replace("ɐ̞nʲ", "ɐ̞̃nʲ");
   let str39nas = &str38nas.replace("ɔ̝nʲ", "ɔ̝̃nʲ");
   let str40nas = &str39nas.replace("ɛ̝nʲ", "ɛ̝̃nʲ");
   let str41nas = &str40nas.replace("ɐnʲ", "ɐ̃nʲ");
   let str42nas = &str41nas.replace("ɔnʲ", "ɔ̃nʲ");
   let str43nas = &str42nas.replace("ɛnʲ", "ɛ̃nʲ");
   let str44nas = &str43nas.replace("inʲ", "ĩnʲ");
   let str45nas = &str44nas.replace("unʲ", "ũnʲ");
   let str46nas = &str45nas.replace("ɐ̞n̟", "ɐ̞̃n̟");
   let str47nas = &str46nas.replace("ɔ̝n̟", "ɔ̝̃n̟");
   let str48nas = &str47nas.replace("ɛ̝n̟", "ɛ̝̃n̟");
   let str49nas = &str48nas.replace("ɐn̟", "ɐ̃n̟");
   let str50nas = &str49nas.replace("ɔn̟", "ɔ̃n̟");
   let str51nas = &str50nas.replace("ɛn̟", "ɛ̃n̟");
   let str52nas = &str51nas.replace("in̟", "ĩn̟");
   let str53nas = &str52nas.replace("un̟", "ũn̟");
   let str54nas = &str53nas.replace("ɐ̞ɴ", "ɐ̞̃ɴ");
   let str55nas = &str54nas.replace("ɔ̝ɴ", "ɔ̝̃ɴ");
   let str56nas = &str55nas.replace("ɛ̝ɴ", "ɛ̝̃ɴ");
   let str57nas = &str56nas.replace("ɐɴ", "ɐ̃ɴ");
   let str58nas = &str57nas.replace("ɔɴ", "ɔ̃ɴ");
   let str59nas = &str58nas.replace("ɛɴ", "ɛ̃ɴ");
   let str60nas = &str59nas.replace("iɴ", "ĩɴ");
   let str61nas = &str60nas.replace("uɴ", "ũɴ");
   let str1pnc = &str61nas.replace(",", " ∣"); // space
   let str2pnc = &str1pnc.replace(". ", " ∥ ");
   let str3pnc = &str2pnc.replace(".", "");
   let str4pnc = &str3pnc.replace(" - ", " ∣ ");
   let str5pnc = &str4pnc.replace(" – ", " ∣ ");
   let str6pnc = &str5pnc.replace("--", " ∣ ");
   let result = &str6pnc.replace("∣ ∥", "∥");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("MADRID, ES:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("MADRID, ES:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Madrid, ES:");
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// CIUDAD DE MÉXICO: IPA

pub fn spamxciudaddemexico(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str1dot = original_text.to_owned() + "."; // mark word ending
   let str2dot = ".".to_owned() + &str1dot;
   let str3low = &str2dot.to_lowercase();
   let str1sgn = str3low.replace(";", ",");
   let str2sgn = str1sgn.replace(":", ",");
   let str3sgn = str2sgn.replace("!", ".");
   let str4sgn = str3sgn.replace("?", ".");
   let str1acc = &str4sgn.replace("", "");
   let str1 = str1acc.replace ("", "");
   let result = &str1.replace("", "");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("CIUDAD DE MÉXICO, MX: [NOT YET IMPLEMENTED]".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("CIUDAD DE MÉXICO, MX: [NOT YET IMPLEMENTED]".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Ciudad de México, MX:{}", red.to_owned() + " [not yet implemented]" + reset);
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// UMARIAÇU: IPA

pub fn tcabrumariacu(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str2 = original_text.to_lowercase();
   let str3 = &str2.replace("1", "₁");
   let str4 = &str3.replace("2", "₂");
   let str5 = &str4.replace("3", "₃");
   let str6 = &str5.replace("4", "₄");
   let str7 = &str6.replace("5", "₅");
   let str8 = &str7.replace("6", "₆");
   let str9 = &str8.replace("x", "ʔ");
   let str10 = &str9.replace("'", "ʔ");
   let str11 = &str10.replace("’", "ʔ");
   let str12 = &str11.replace("b", "b̥");
   let str13 = &str12.replace("cue", "kue");
   let str14 = &str13.replace("cui", "kui");
   let str15 = &str14.replace("c", "k");
   let str16 = &str15.replace("tkh", "ch");
   let str17 = &str16.replace("kh", "ch");
   let str18 = &str17.replace("ch", "ʧ");
   let str19 = &str18.replace("d", "d");
   let str20 = &str19.replace("ng", "ŋ");
   let str21 = &str20.replace("g", "ɡ̥");
   let str22 = &str21.replace("n", "n̪");
   let str23 = &str22.replace("nh", "ñ");
   let str24 = &str23.replace("ñ", "ɲ");
   let str25 = &str24.replace("que", "ke");
   let str26 = &str25.replace("qui", "ki");
   let str27 = &str26.replace("q", "k");
   let str28 = &str27.replace("r", "ɾ");
   let str29 = &str28.replace("t", "t̪");
   let str30 = &str29.replace("y", "d͡ʒ");
   let str31 = &str30.replace("j", "d͡ʒ");
   let str32 = &str31.replace("f", "ɸ");
   let str33 = &str32.replace("z", "s");
   let str34 = &str33.replace("ç", "s");
   let str35 = &str34.replace("s", "s̺");
   let str36 = &str35.replace("v", "ɰ");
   let str37 = &str36.replace("ɡ", "ɡ̥");
   let str38 = &str37.replace("ã", "ɑ̃");
   let str39 = &str38.replace("ã", "ɑ̃");
   let str40 = &str39.replace("õ", "ɔ̝̃");
   let str41 = &str40.replace("õ", "ɔ̝̃");
   let str42 = &str41.replace("ẽ", "ɛ̝̃");
   let str43 = &str42.replace("ẽ", "ɛ̝̃");
   let str44 = &str43.replace("ũ", "ʊ̃");
   let str45 = &str44.replace("ũ", "ʊ̃");
   let str46 = &str45.replace("ü̃", "ɯ̃");
   let str47 = &str46.replace("a̰", "ʌ̰");
   let str48 = &str47.replace("a̱", "ʌ̰");
   let str49 = &str48.replace("a̠", "ʌ̰");
   let str50 = &str49.replace("o̰", "ɔ̰");
   let str51 = &str50.replace("o̱", "ɔ̰");
   let str52 = &str51.replace("o̠", "ɔ̰");
   let str53 = &str52.replace("ḛ", "ɛ̰");
   let str54 = &str53.replace("e̱", "ɛ̰");
   let str55 = &str54.replace("e̠", "ɛ̰");
   let str56 = &str55.replace("ḭ", "ḭ");
   let str57 = &str56.replace("i̱", "ḭ");
   let str58 = &str57.replace("i̠", "ḭ");
   let str59 = &str58.replace("ṵ", "ʊ̰");
   let str60 = &str59.replace("u̱", "ʊ̰");
   let str61 = &str60.replace("u̠", "ʊ̰");
   let str62 = &str61.replace("ṵ̈", "ɯ̰");
   let str63 = &str62.replace("ü̱", "ɯ̰");
   let str64 = &str63.replace("ü̠", "ɯ̰");
   let str65 = &str64.replace("á", "a");
   let str66 = &str65.replace("é", "ɛ̝");
   let str67 = &str66.replace("í", "i");
   let str68 = &str67.replace("ó", "o");
   let str69 = &str68.replace("ú", "ʊ");
   let str70 = &str69.replace("ǘ", "ü");
   let str71 = &str70.replace("à", "a");
   let str72 = &str71.replace("è", "ɛ̝");
   let str73 = &str72.replace("ì", "i");
   let str74 = &str73.replace("ò", "o");
   let str75 = &str74.replace("ù", "ʊ");
   let str76 = &str75.replace("ǜ", "ü");
   let str77 = &str76.replace("w", "ɰ");
   let str78 = &str77.replace("d͡ʒae", "d͡ʒæi");
   let str79 = &str78.replace("a", "ɑ̈");
   let str80 = &str79.replace("e", "ɛ̝");
   let str81 = &str80.replace("o", "ɔ̝");
   let str82 = &str81.replace("u", "ʊ");
   let str83 = &str82.replace("ü", "ɯ̈");
   let str84 = &str83.replace("ɑ̈ɑ̃", "ɑ̃ː");
   let str85 = &str84.replace("ɑ̈ɑ̈", "ɑ̈ː");
   let str86 = &str85.replace(". ", " ∥ ");
   let str87 = &str86.replace(".", "");
   let str88 = &str87.replace(",", " ∣");
   let str89 = &str88.replace(";", " ∥");
   let str90 = &str89.replace(":", " ∣");
   let str91 = &str90.replace("? ", " ∥ ");
   let str92 = &str91.replace("?", "");   
   let str93 = &str92.replace("¿", "");
   let str94 = &str93.replace("! ", " ∥ ");
   let str95 = &str94.replace("!", "");
   let str96 = &str95.replace("¡", "");
   let str97 = &str96.replace(" - ", " ∣ ");
   let str98 = &str97.replace(" – ", " ∣ ");
   let str99 = &str98.replace("--", " ∣ ");
   let str100 = &str99.replace("(", "∣ ");
   let result = &str100.replace(")", " ∣");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("UMARIAÇU, BR:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("UMARIAÇU, BR:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Umariaçu, BR:");
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// VILA BETÂNIA: IPA

pub fn tcabrvilabetania(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str2 = original_text.to_lowercase();
   let str3 = &str2.replace("1", "₁");
   let str4 = &str3.replace("2", "₂");
   let str5 = &str4.replace("3", "₃");
   let str6 = &str5.replace("4", "₄");
   let str7 = &str6.replace("5", "₅");
   let str8 = &str7.replace("6", "₆");
   let str9 = &str8.replace("x", "ʔ");
   let str10 = &str9.replace("'", "ʔ");
   let str11 = &str10.replace("’", "ʔ");
   let str12 = &str11.replace("b", "b̥");
   let str13 = &str12.replace("cue", "kue");
   let str14 = &str13.replace("cui", "kui");
   let str15 = &str14.replace("c", "k");
   let str16 = &str15.replace("tkh", "ch");
   let str17 = &str16.replace("kh", "ch");
   let str18 = &str17.replace("ch", "ʧ");
   let str19 = &str18.replace("d", "d");
   let str20 = &str19.replace("ng", "ŋ");
   let str21 = &str20.replace("g", "ɡ");
   let str22 = &str21.replace("n", "n̪");
   let str23 = &str22.replace("nh", "ñ");
   let str24 = &str23.replace("ñ", "ɲ");
   let str25 = &str24.replace("que", "ke");
   let str26 = &str25.replace("qui", "ki");
   let str27 = &str26.replace("q", "k");
   let str28 = &str27.replace("r", "ɾ");
   let str29 = &str28.replace("t", "t̪");
   let str30 = &str29.replace("y", "d͡ʒ");
   let str31 = &str30.replace("j", "d͡ʒ");
   let str32 = &str31.replace("f", "ɸ");
   let str33 = &str32.replace("z", "s");
   let str34 = &str33.replace("ç", "s");
   let str35 = &str34.replace("s", "s̺");
   let str36 = &str35.replace("v", "ɰ");
   let str37 = &str36.replace("ɡ", "ɡ̥");
   let str38 = &str37.replace("ã", "ɐ̝̃");
   let str39 = &str38.replace("ã", "ɐ̝̃");
   let str40 = &str39.replace("õ", "ɒ̃");
   let str41 = &str40.replace("õ", "ɒ̃");
   let str42 = &str41.replace("ẽ", "ɶ̃");
   let str43 = &str42.replace("ẽ", "ɶ̃");
   let str44 = &str43.replace("ũ", "ũ");
   let str45 = &str44.replace("ũ", "ũ");
   let str46 = &str45.replace("ü̃", "ɤ̝̃");
   let str47 = &str46.replace("a̰", "ʌ̰̈");
   let str48 = &str47.replace("a̱", "ʌ̰̈");
   let str49 = &str48.replace("a̠", "ʌ̰̈");
   let str50 = &str49.replace("o̰", "ɒ̰");
   let str51 = &str50.replace("o̱", "ɒ̰");
   let str52 = &str51.replace("o̠", "ɒ̰");
   let str53 = &str52.replace("ḛ", "ɶ̰");
   let str54 = &str53.replace("e̱", "ɶ̰");
   let str55 = &str54.replace("e̠", "ɶ̰");
   let str56 = &str55.replace("ḭ", "ḭ");
   let str57 = &str56.replace("i̱", "ḭ");
   let str58 = &str57.replace("i̠", "ḭ");
   let str59 = &str58.replace("ṵ", "ṵ");
   let str60 = &str59.replace("u̱", "ṵ");
   let str61 = &str60.replace("u̠", "ṵ");
   let str62 = &str61.replace("ṵ̈", "ɤ̰");
   let str63 = &str62.replace("ü̱", "ɤ̰");
   let str64 = &str63.replace("ü̠", "ɤ̰");
   let str65 = &str64.replace("á", "a");
   let str66 = &str65.replace("é", "e");
   let str67 = &str66.replace("í", "i");
   let str68 = &str67.replace("ó", "o");
   let str69 = &str68.replace("ú", "u");
   let str70 = &str69.replace("ǘ", "ü");
   let str71 = &str70.replace("à", "a");
   let str72 = &str71.replace("è", "e");
   let str73 = &str72.replace("ì", "i");
   let str73a = &str73.replace("i", "i̞");
   let str74 = &str73a.replace("ò", "o");
   let str75 = &str74.replace("ù", "u");
   let str76 = &str75.replace("ǜ", "ü");
   let str77 = &str76.replace("w", "ɰ");
   let str78 = &str77.replace("d͡ʒae", "d͡ʒɶi̞");
   let str79 = &str78.replace("a", "ɐ̝");
   let str80 = &str79.replace("e", "ɶ");
   let str81 = &str80.replace("o", "ɒ");
   let str82 = &str81.replace("u", "ü");
   let str83 = &str82.replace("ü̃", "ũ");
   let str84 = &str83.replace("ü", "ɤ̝");
   let str85 = &str84.replace("ɐ̝ɐ̝̃", "ɐ̝̃ː");
   let str86 = &str85.replace("ɐ̝ɐ̝", "ɐ̝ː");
   let str87 = &str86.replace(". ", " ∥ ");
   let str88 = &str87.replace(".", "");
   let str89 = &str88.replace(",", " ∣");
   let str90 = &str89.replace(";", " ∥");
   let str91 = &str90.replace(":", " ∣");
   let str92 = &str91.replace("? ", " ∥ ");
   let str93 = &str92.replace("?", "");   
   let str94 = &str93.replace("¿", "");
   let str95 = &str94.replace("! ", " ∥ ");
   let str96 = &str95.replace("!", "");
   let str97 = &str96.replace("¡", "");
   let str98 = &str97.replace(" - ", " ∣ ");
   let str99 = &str98.replace(" – ", " ∣ ");
   let str100 = &str99.replace("--", " ∣ ");
   let str101 = &str100.replace("(", "∣ ");
   let result = &str101.replace(")", " ∣");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("VILA BETÂNIA, BR:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("VILA BETÂNIA, BR:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Vila Betânia, BR:");
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// NAZARETH: IPA

pub fn tcaconazareth(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str2 = original_text.to_lowercase();
   let str3 = &str2.replace("1", "₁");
   let str4 = &str3.replace("2", "₂");
   let str5 = &str4.replace("3", "₃");
   let str6 = &str5.replace("4", "₄");
   let str7 = &str6.replace("5", "₅");
   let str8 = &str7.replace("6", "₆");
   let str9 = &str8.replace("x", "ʔ");
   let str10 = &str9.replace("'", "ʔ");
   let str11 = &str10.replace("’", "ʔ");
   let str12 = &str11.replace("b", "b̥");
   let str13 = &str12.replace("w", "ɰ");
   let str14 = &str13.replace("v", "ɰ");
   let str15 = &str14.replace("cue", "kue");
   let str16 = &str15.replace("cui", "kui");
   let str17 = &str16.replace("c", "k");
   let str18 = &str17.replace("tkh", "ch");
   let str19 = &str18.replace("kh", "ch");
   let str20 = &str19.replace("ch", "ʧ");
   let str21 = &str20.replace("d", "d");
   let str22 = &str21.replace("ng", "ŋ");
   let str23 = &str22.replace("g", "ɡ");
   let str24 = &str23.replace("n", "n̪");
   let str25 = &str24.replace("nh", "ñ");
   let str26 = &str25.replace("ñ", "ɲ");
   let str27 = &str26.replace("que", "ke");
   let str28 = &str27.replace("qui", "ki");
   let str29 = &str28.replace("q", "k");
   let str30 = &str29.replace("r", "ɾ");
   let str31 = &str30.replace("t", "t̪");
   let str32 = &str31.replace("y", "d͡ʒ");
   let str33 = &str32.replace("j", "d͡ʒ");
   let str34 = &str33.replace("f", "ɸ");
   let str35 = &str34.replace("z", "s");
   let str36 = &str35.replace("ç", "s");
   let str37 = &str36.replace("s", "s̺");
   let str38 = &str37.replace("ã", "ɐ̃");
   let str39 = &str38.replace("ã", "ɐ̃");
   let str40 = &str39.replace("ũ", "ʉ̃");
   let str41 = &str40.replace("ũ", "ʉ̃");
   let str42 = &str41.replace("ü̃", "ɨ̃");
   let str43 = &str42.replace("a̰", "ʌ̰");
   let str44 = &str43.replace("a̱", "ʌ̰");
   let str45 = &str44.replace("a̠", "ʌ̰");
   let str46 = &str45.replace("o̰", "o̰");
   let str47 = &str46.replace("o̱", "o̰");
   let str48 = &str47.replace("o̠", "o̰");
   let str49 = &str48.replace("ḛ", "ḛ");
   let str50 = &str49.replace("e̱", "ḛ");
   let str51 = &str50.replace("e̠", "ḛ");
   let str52 = &str51.replace("ḭ", "ḭ");
   let str53 = &str52.replace("i̱", "ḭ");
   let str54 = &str53.replace("i̠", "ḭ");
   let str55 = &str54.replace("u̱", "ṵ");
   let str56 = &str55.replace("u̠", "ṵ");
   let str57 = &str56.replace("ṵ̈", "ɨ̰");
   let str58 = &str57.replace("ü̱", "ɨ̰");
   let str59 = &str58.replace("ü̠", "ɨ̰");
   let str60 = &str59.replace("á", "a");
   let str61 = &str60.replace("é", "e");
   let str62 = &str61.replace("í", "i");
   let str63 = &str62.replace("ó", "o");
   let str64 = &str63.replace("ú", "ʉ̱");
   let str65 = &str64.replace("ǘ", "ü");
   let str66 = &str65.replace("à", "a");
   let str67 = &str66.replace("è", "e");
   let str68 = &str67.replace("ì", "i");
   let str69 = &str68.replace("ò", "o");
   let str70 = &str69.replace("ù", "ʉ̱");
   let str71 = &str70.replace("ǜ", "ü");
   let str72 = &str71.replace("d͡ʒae", "d͡ʒæː");
   let str73 = &str72.replace("a", "ɐ");
   let str74 = &str73.replace("u", "ʉ̱");
   let str75 = &str74.replace("ü", "ɨ");
   let str76 = &str75.replace("ʉ̱̰", "ṵ");
   let str77 = &str76.replace("ɐɐ̃", "ɐ̃ː");
   let str78 = &str77.replace("ɐɐ", "ɐː");
   let str79 = &str78.replace(". ", " ∥ ");
   let str80 = &str79.replace(".", "");
   let str81 = &str80.replace(",", " ∣");
   let str82 = &str81.replace(";", " ∥");
   let str83 = &str82.replace(":", " ∣");
   let str84 = &str83.replace("? ", " ∥ ");
   let str85 = &str84.replace("?", "");   
   let str86 = &str85.replace("¿", "");
   let str87 = &str86.replace("! ", " ∥ ");
   let str88 = &str87.replace("!", "");
   let str89 = &str88.replace("¡", "");
   let str90 = &str89.replace(" - ", " ∣ ");
   let str91 = &str90.replace(" – ", " ∣ ");
   let str92 = &str91.replace("--", " ∣ ");
   let str93 = &str92.replace("(", "∣ ");
   let result = &str93.replace(")", " ∣");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("NAZARETH, CO:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("NAZARETH, CO:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Nazareth, CO:");
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// RIO COTUHÉ: IPA

pub fn tcacoriocotuhe(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str2 = original_text.to_lowercase();
   let str3 = &str2.replace("1", "₁");
   let str4 = &str3.replace("2", "₂");
   let str5 = &str4.replace("3", "₃");
   let str6 = &str5.replace("4", "₄");
   let str7 = &str6.replace("5", "₅");
   let str8 = &str7.replace("6", "₆");
   let str9 = &str8.replace("x", "ʔ");
   let str10 = &str9.replace("'", "ʔ");
   let str11 = &str10.replace("’", "ʔ");
   let str12 = &str11.replace("b", "b̥");
   let str13 = &str12.replace("cue", "kue");
   let str14 = &str13.replace("cui", "kui");
   let str15 = &str14.replace("c", "k");
   let str16 = &str15.replace("tkh", "ch");
   let str17 = &str16.replace("kh", "ch");
   let str18 = &str17.replace("ch", "ʧ");
   let str19 = &str18.replace("d", "d");
   let str20 = &str19.replace("ng", "ŋ");
   let str21 = &str20.replace("g", "ɡ");
   let str22 = &str21.replace("n", "n̪");
   let str23 = &str22.replace("nh", "ñ");
   let str24 = &str23.replace("ñ", "ɲ");
   let str25 = &str24.replace("que", "ke");
   let str26 = &str25.replace("qui", "ki");
   let str27 = &str26.replace("q", "k");
   let str28 = &str27.replace("r", "ɾ");
   let str29 = &str28.replace("t", "t̪");
   let str30 = &str29.replace("y", "d͡ʒ");
   let str31 = &str30.replace("j", "d͡ʒ");
   let str32 = &str31.replace("f", "ɸ");
   let str33 = &str32.replace("z", "s");
   let str34 = &str33.replace("ç", "s");
   let str35 = &str34.replace("s", "s̺");
   let str36 = &str35.replace("v", "ɰ");
   let str37 = &str36.replace("k", "k̬");
   let str38 = &str37.replace("ã", "ɐ̃");
   let str39 = &str38.replace("ã", "ɐ̃");
   let str40 = &str39.replace("õ", "ɒ̃");
   let str41 = &str40.replace("õ", "ɒ̃");
   let str42 = &str41.replace("ẽ", "ɶ̃");
   let str43 = &str42.replace("ẽ", "ɶ̃");
   let str44 = &str43.replace("ũ", "ʊ̞̃");
   let str45 = &str44.replace("ũ", "ʊ̞̃");
   let str46 = &str45.replace("ü̃", "ə̃");
   let str47 = &str46.replace("a̰", "ʌ̰");
   let str48 = &str47.replace("a̱", "ʌ̰");
   let str49 = &str48.replace("a̠", "ʌ̰");
   let str50 = &str49.replace("o̰", "ɒ̰");
   let str51 = &str50.replace("o̱", "ɒ̰");
   let str52 = &str51.replace("o̠", "ɒ̰");
   let str53 = &str52.replace("ḛ", "ɶ̰");
   let str54 = &str53.replace("e̱", "ɶ̰");
   let str55 = &str54.replace("e̠", "ɶ̰");
   let str56 = &str55.replace("ḭ", "ḭ");
   let str57 = &str56.replace("i̱", "ḭ");
   let str58 = &str57.replace("i̠", "ḭ");
   let str59 = &str58.replace("ṵ̈", "ɘ̰");
   let str60 = &str59.replace("ü̱", "ɘ̰");
   let str61 = &str60.replace("ü̠", "ɘ̰");
   let str62 = &str61.replace("á", "a");
   let str63 = &str62.replace("é", "e");
   let str64 = &str63.replace("í", "i");
   let str65 = &str64.replace("ó", "o");
   let str66 = &str65.replace("ú", "u");
   let str67 = &str66.replace("ǘ", "ü");
   let str68 = &str67.replace("à", "a");
   let str69 = &str68.replace("è", "e");
   let str70 = &str69.replace("ì", "i");
   let str71 = &str70.replace("ò", "o");
   let str72 = &str71.replace("ù", "u");
   let str73 = &str72.replace("ǜ", "ü");
   let str74 = &str73.replace("w", "ɰ");
   let str75 = &str74.replace("ɡü", "ɡüː");
   let str76 = &str75.replace("k̬ü", "k̬üː");
   let str77 = &str76.replace("ɾü", "ɾüː");
   let str78 = &str77.replace("a", "ɐ̝");
   let str79 = &str78.replace("e", "ɶ");
   let str80 = &str79.replace("i", "i̞");
   let str81 = &str80.replace("o", "ɒ");
   let str82 = &str81.replace("ṵ", "o̰");
   let str83 = &str82.replace("u̱", "o̰");
   let str84 = &str83.replace("u̠", "o̰");
   let str85 = &str84.replace("u", "ö");
   let str86 = &str85.replace("ü", "ɘ");
   let str87 = &str86.replace("ɐ̝ɐ̝", "ɐ̝ː");
   let str88 = &str87.replace(". ", " ∥ ");
   let str89 = &str88.replace(".", "");
   let str90 = &str89.replace(",", " ∣");
   let str91 = &str90.replace(";", " ∥");
   let str92 = &str91.replace(":", " ∣");
   let str93 = &str92.replace("? ", " ∥ ");
   let str94 = &str93.replace("?", "");   
   let str95 = &str94.replace("¿", "");
   let str96 = &str95.replace("! ", " ∥ ");
   let str97 = &str96.replace("!", "");
   let str98 = &str97.replace("¡", "");
   let str99 = &str98.replace(" - ", " ∣ ");
   let str100 = &str99.replace(" – ", " ∣ ");
   let str101 = &str100.replace("--", " ∣ ");
   let str102 = &str101.replace("(", "∣ ");
   let result = &str102.replace(")", " ∣");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("RÍO COTUHÉ, CO:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("RÍO COTUHÉ, CO:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Río Cotuhé, CO:");
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// CUSHILLOCOCHA: IPA

pub fn tcapecushillococha(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str2 = original_text.to_lowercase();
   let str3 = &str2.replace("1", "₁");
   let str4 = &str3.replace("2", "₂");
   let str5 = &str4.replace("3", "₃");
   let str6 = &str5.replace("4", "₄");
   let str7 = &str6.replace("5", "₅");
   let str8 = &str7.replace("6", "₆");
   let str9 = &str8.replace("x", "ʔ");
   let str10 = &str9.replace("'", "ʔ");
   let str11 = &str10.replace("’", "ʔ");
   let str12 = &str11.replace("b", "b̥");
   let str13 = &str12.replace("cue", "kue");
   let str14 = &str13.replace("cui", "kui");
   let str15 = &str14.replace("c", "k");
   let str16 = &str15.replace("tkh", "ch");
   let str17 = &str16.replace("kh", "ch");
   let str18 = &str17.replace("ch", "ʧ");
   let str19 = &str18.replace("d", "d");
   let str20 = &str19.replace("ng", "ŋ");
   let str21 = &str20.replace("g", "ɡ");
   let str22 = &str21.replace("n", "n̪");
   let str23 = &str22.replace("nh", "ñ");
   let str24 = &str23.replace("ñ", "ɲ");
   let str25 = &str24.replace("que", "ke");
   let str26 = &str25.replace("qui", "ki");
   let str27 = &str26.replace("q", "k");
   let str28 = &str27.replace("r", "ɾ");
   let str29 = &str28.replace("t", "t̪");
   let str30 = &str29.replace("y", "d͡ʒ");
   let str31 = &str30.replace("j", "d͡ʒ");
   let str32 = &str31.replace("f", "ɸ");
   let str33 = &str32.replace("z", "s");
   let str34 = &str33.replace("ç", "s");
   let str35 = &str34.replace("s", "s̺");
   let str36 = &str35.replace("v", "ɰ");
   let str37 = &str36.replace("ɡ", "ɡ̥");
   let str38 = &str37.replace("ã", "ã̹");
   let str39 = &str38.replace("ã", "ã̹");
   let str40 = &str39.replace("õ", "ɔ̃");
   let str41 = &str40.replace("õ", "ɔ̃");
   let str42 = &str41.replace("ẽ", "ɛ̃");
   let str43 = &str42.replace("ẽ", "ɛ̃");
   let str44 = &str43.replace("ũ", "ʊ̃");
   let str45 = &str44.replace("ũ", "ʊ̃");
   let str46 = &str45.replace("ü̃", "ɨ̃");
   let str47 = &str46.replace("a̰", "ʌ̰");
   let str48 = &str47.replace("a̱", "ʌ̰");
   let str49 = &str48.replace("a̠", "ʌ̰");
   let str50 = &str49.replace("o̰", "ɔ̰");
   let str51 = &str50.replace("o̱", "ɔ̰");
   let str52 = &str51.replace("o̠", "ɔ̰");
   let str53 = &str52.replace("ḛ", "ɛ̰");
   let str54 = &str53.replace("e̱", "ɛ̰");
   let str55 = &str54.replace("e̠", "ɛ̰");
   let str56 = &str55.replace("ḭ", "ḭ");
   let str57 = &str56.replace("i̱", "ḭ");
   let str58 = &str57.replace("i̠", "ḭ");
   let str59 = &str58.replace("ṵ", "ʊ̰");
   let str60 = &str59.replace("u̱", "ʊ̰");
   let str61 = &str60.replace("u̠", "ʊ̰");
   let str62 = &str61.replace("ṵ̈", "ɨ̰");
   let str63 = &str62.replace("ü̱", "ɨ̰");
   let str64 = &str63.replace("ü̠", "ɨ̰");
   let str65 = &str64.replace("á", "a");
   let str66 = &str65.replace("é", "e");
   let str67 = &str66.replace("í", "i");
   let str68 = &str67.replace("ó", "o");
   let str69 = &str68.replace("ú", "u");
   let str70 = &str69.replace("ǘ", "ü");
   let str71 = &str70.replace("à", "a");
   let str72 = &str71.replace("è", "e");
   let str73 = &str72.replace("ì", "i");
   let str74 = &str73.replace("ò", "o");
   let str75 = &str74.replace("ù", "u");
   let str76 = &str75.replace("ǜ", "ü");
   let str77 = &str76.replace("w", "ɰ");
   let str78 = &str77.replace("d͡ʒae", "d͡ʒɛi");
   let str79 = &str78.replace("a", "a̹");
   let str80 = &str79.replace("e", "ɛ");
   let str81 = &str80.replace("o", "ɔ");
   let str82 = &str81.replace("u", "ʊ");
   let str83 = &str82.replace("ü", "ɨ̞");
   let str84 = &str83.replace("a̹ã̹", "ã̹ː");
   let str85 = &str84.replace("a̹a̹", "a̹ː");
   let str86 = &str85.replace(". ", " ∥ ");
   let str87 = &str86.replace(".", "");
   let str88 = &str87.replace(",", " ∣");
   let str89 = &str88.replace(";", " ∥");
   let str90 = &str89.replace(":", " ∣");
   let str91 = &str90.replace("? ", " ∥ ");
   let str92 = &str91.replace("?", "");   
   let str93 = &str92.replace("¿", "");
   let str94 = &str93.replace("! ", " ∥ ");
   let str95 = &str94.replace("!", "");
   let str96 = &str95.replace("¡", "");
   let str97 = &str96.replace(" - ", " ∣ ");
   let str98 = &str97.replace(" – ", " ∣ ");
   let str99 = &str98.replace("--", " ∣ ");
   let str100 = &str99.replace("(", "∣ ");
   let result = &str100.replace(")", " ∣");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("CUSHILLOCOCHA, PE:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("CUSHILLOCOCHA, PE:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Cushillococha, PE:");
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// TIKUNA ORTHOGRAPHY: BRAZIL

pub fn orttcabr(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str2 = original_text.replace("y", "j");
   let str3 = &str2.replace("Y", "J");
   let str4 = &str3.replace("ch", "tch");
   let str5 = &str4.replace("Ch", "Tch");
   let str6 = &str5.replace("CH", "TCH");
   let str7 = &str6.replace("ttch", "tch");
   let str8 = &str7.replace("Ttch", "Tch");
   let str9 = &str8.replace("TTCH", "TCH");
   let str10 = &str9.replace("ñ", "nh");
   let str11 = &str10.replace("Ñ", "NH");
   let str12 = &str11.replace("ka", "ca");
   let str13 = &str12.replace("Ka", "Ca");
   let str14 = &str13.replace("KA", "CA");
   let str15 = &str14.replace("ka̱", "ca");
   let str16 = &str15.replace("Ka̱", "Ca");
   let str17 = &str16.replace("KA̱", "CA");
   let str18 = &str17.replace("ká", "cá");
   let str19 = &str18.replace("Ká", "Cá");
   let str20 = &str19.replace("KÁ", "CÁ");
   let str21 = &str20.replace("ko", "co");
   let str22 = &str21.replace("Ko", "Co");
   let str23 = &str22.replace("KO", "CO");
   let str24 = &str23.replace("kõ", "cõ");
   let str25 = &str24.replace("Kõ", "Cõ");
   let str26 = &str25.replace("KÕ", "CÕ");
   let str27 = &str26.replace("kó", "có");
   let str28 = &str27.replace("Kó", "Có");
   let str29 = &str28.replace("KÓ", "CÓ");
   let str30 = &str29.replace("ku", "cu");
   let str31 = &str30.replace("Ku", "Cu");
   let str32 = &str31.replace("KU", "CU");
   let str33 = &str32.replace("ku̱", "cu’");
   let str34 = &str33.replace("Ku̱", "Cu’");
   let str35 = &str34.replace("KU̱", "CU’");
   let str36 = &str35.replace("kú", "cú");
   let str37 = &str36.replace("Kú", "Cú");
   let str38 = &str37.replace("KÚ", "CÚ");
   let str39 = &str38.replace("kü", "cü");
   let str40 = &str39.replace("Kü", "Cü");
   let str41 = &str40.replace("KÜ", "CÜ");
   let str42 = &str41.replace("kü’", "cü’");
   let str43 = &str42.replace("Kü’", "Cü’");
   let str44 = &str43.replace("KÜ’", "CÜ’");
   let str45 = &str44.replace("cua", "qua");
   let str46 = &str45.replace("Cua", "Qua");
   let str47 = &str46.replace("CUA", "QUA");
   let str48 = &str47.replace("cua̱", "qua’");
   let str49 = &str48.replace("Cua̱", "Qua’");
   let str50 = &str49.replace("CUA̱", "QUA’");
   let str51 = &str50.replace("kua", "qua");
   let str52 = &str51.replace("Kua", "Qua");
   let str53 = &str52.replace("KUA", "QUA");
   let str54 = &str53.replace("kua̱", "qua’");
   let str55 = &str54.replace("Kua̱", "Qua’");
   let str56 = &str55.replace("KUA̱", "QUA’");
   let str57 = &str56.replace("ki", "qui");
   let str58 = &str57.replace("Ki", "Qui");
   let str59 = &str58.replace("KI", "QUI");
   let str60 = &str59.replace("ke", "que");
   let str61 = &str60.replace("Ke", "Que");
   let str62 = &str61.replace("KE", "QUE");
   let str63 = &str62.replace("ax", "a’");
   let str64 = &str63.replace("ãx", "ã’");
   let str65 = &str64.replace("a̱x", "a’");
   let str66 = &str65.replace("ã̱x", "ã’");
   let str67 = &str66.replace("áx", "á’");
   let str68 = &str67.replace("ex", "e’");
   let str69 = &str68.replace("ẽx", "ẽ’");
   let str70 = &str69.replace("e̱x", "e’");
   let str71 = &str70.replace("ẽ̱x", "ẽ’");
   let str72 = &str71.replace("éx", "é’");
   let str73 = &str72.replace("ix", "i’");
   let str74 = &str73.replace("ĩx", "ĩ’");
   let str75 = &str74.replace("íx", "í’");
   let str76 = &str75.replace("ox", "o’");
   let str77 = &str76.replace("õx", "õ’");
   let str78 = &str77.replace("o̱x", "o’");
   let str79 = &str78.replace("õ̱x", "õ’");
   let str80 = &str79.replace("óx", "ó’");
   let str81 = &str80.replace("ux", "u’");
   let str82 = &str81.replace("ũx", "ũ’");
   let str83 = &str82.replace("u̱x", "u’");
   let str84 = &str83.replace("úx", "ú’");
   let str85 = &str84.replace("üx", "ü’");
   let str86 = &str85.replace("ü̃x", "ü̃’");
   let str87 = &str86.replace("ü̱x", "ü’");
   let str88 = &str87.replace("ü̱̃x", "ü̃’");
   let str89 = &str88.replace("ǘx", "ǘ’");
   let str90 = &str89.replace("AX", "A’");
   let str91 = &str90.replace("ÃX", "Ã’");
   let str92 = &str91.replace("A̱X", "A’");
   let str93 = &str92.replace("Ã̱X", "Ã’");
   let str94 = &str93.replace("ÁX", "Á’");
   let str95 = &str94.replace("EX", "E’");
   let str96 = &str95.replace("ẼX", "Ẽ’");
   let str97 = &str96.replace("E̱X", "E’");
   let str98 = &str97.replace("Ẽ̱X", "Ẽ’");
   let str99 = &str98.replace("ÉX", "É’");
   let str100 = &str99.replace("IX", "I’");
   let str101 = &str100.replace("ĨX", "Ĩ’");
   let str102 = &str101.replace("I̱X", "I’");
   let str103 = &str102.replace("Ĩ̱X", "Ĩ’");
   let str104 = &str103.replace("ÍX", "Í’");
   let str105 = &str104.replace("OX", "O’");
   let str106 = &str105.replace("ÕX", "Õ’");
   let str107 = &str106.replace("O̱X", "O’");
   let str108 = &str107.replace("Õ̱X", "Õ’");
   let str109 = &str108.replace("ÓX", "Ó’");
   let str110 = &str109.replace("UX", "U’");
   let str111 = &str110.replace("ŨX", "Ũ’");
   let str112 = &str111.replace("U̱X", "U’");
   let str113 = &str112.replace("Ũ̱X", "Ũ’");
   let str114 = &str113.replace("ÚX", "Ú’");
   let str115 = &str114.replace("ÜX", "Ü’");
   let str116 = &str115.replace("Ü̃X", "Ü̃’");
   let str117 = &str116.replace("Ü̱X", "Ü’");
   let str118 = &str117.replace("Ü̱̃X", "Ü̃’");
   let str119 = &str118.replace("ǗX", "Ǘ’");
   let str120 = &str119.replace("Ax", "A’");
   let str121 = &str120.replace("Ãx", "Ã’");
   let str122 = &str121.replace("A̱x", "A’");
   let str123 = &str122.replace("Ã̱x", "Ã’");
   let str124 = &str123.replace("Áx", "Á’");
   let str125 = &str124.replace("Ex", "E’");
   let str126 = &str125.replace("Ẽx", "Ẽ’");
   let str127 = &str126.replace("E̱x", "E’");
   let str128 = &str127.replace("Ẽ̱x", "Ẽ’");
   let str129 = &str128.replace("Éx", "É’");
   let str130 = &str129.replace("Ix", "I’");
   let str131 = &str130.replace("Ĩx", "Ĩ’");
   let str132 = &str131.replace("Íx", "Í’");
   let str133 = &str132.replace("Ox", "O’");
   let str134 = &str133.replace("Õx", "Õ’");
   let str135 = &str134.replace("O̱x", "O’");
   let str136 = &str135.replace("Õ̱x", "Õ’");
   let str137 = &str136.replace("Óx", "Ó’");
   let str138 = &str137.replace("Ux", "U’");
   let str139 = &str138.replace("Ũx", "Ũ’");
   let str140 = &str139.replace("U̱x", "U’");
   let str141 = &str140.replace("Ũ̱x", "Ũ’");
   let str142 = &str141.replace("Úx", "Ú’");
   let str143 = &str142.replace("Üx", "Ü’");
   let str144 = &str143.replace("Ü̃x", "Ü̃’");
   let str145 = &str144.replace("Ü̱x", "Ü’");
   let str146 = &str145.replace("Ü̱̃x", "Ü̃’");
   let result = &str146.replace("Ǘx", "Ǘ’");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("BRAZIL:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("BRAZIL:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Brazil:");
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// TIKUNA ORTHOGRAPHY: COLOMBIA

pub fn orttcaco(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str2 = original_text.replace("j", "y");
   let str3 = &str2.replace("J", "Y");
   let str4 = &str3.replace("tch", "ch");
   let str5 = &str4.replace("Tch", "Ch");
   let str6 = &str5.replace("TCH", "CH");
   let str7 = &str6.replace("nh", "ñ");
   let str8 = &str7.replace("Nh", "Ñ");
   let str9 = &str8.replace("NH", "Ñ");
   let str10 = &str9.replace("c", "k");
   let str11 = &str10.replace("C", "K");
   let str12 = &str11.replace("kh", "ch");
   let str13 = &str12.replace("Kh", "Ch");
   let str14 = &str13.replace("KH", "CH");
   let str15 = &str14.replace("qui", "ki");
   let str16 = &str15.replace("Qui", "Ki");
   let str17 = &str16.replace("QUI", "KI");
   let str18 = &str17.replace("que", "ke");
   let str19 = &str18.replace("Que", "Ke");
   let str20 = &str19.replace("QUE", "KE");
   let str21 = &str20.replace("q", "k");
   let str22 = &str21.replace("Q", "K");
   let str23 = &str22.replace("x", "");
   let str24 = &str23.replace("X", "");
   let str25 = &str24.replace("'", "");
   let result = &str25.replace("’", "");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("COLOMBIA:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("COLOMBIA:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Colombia:");
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// TIKUNA ORTHOGRAPHY: PERU ILV

pub fn orttcapeilv(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str2 = original_text.replace("j", "y");
   let str3 = &str2.replace("J", "Y");
   let str4 = &str3.replace("tch", "ch");
   let str5 = &str4.replace("Tch", "Ch");
   let str6 = &str5.replace("TCH", "CH");
   let str7 = &str6.replace("nh", "ñ");
   let str8 = &str7.replace("Nh", "Ñ");
   let str9 = &str8.replace("NH", "Ñ");
   let str10 = &str9.replace("q", "c");
   let str11 = &str10.replace("Q", "C");
   let str12 = &str11.replace("à", "a");
   let str13 = &str12.replace("ò", "o");
   let str14 = &str13.replace("è", "e");
   let str15 = &str14.replace("ì", "i");
   let str16 = &str15.replace("ù", "u");
   let str17 = &str16.replace("ǜ", "ü");
   let str18 = &str17.replace("À", "A");
   let str19 = &str18.replace("Ò", "O");
   let str20 = &str19.replace("È", "E");
   let str21 = &str20.replace("Ì", "I");
   let str22 = &str21.replace("Ù", "U");
   let str23 = &str22.replace("Ǜ", "Ü");
   let str24 = &str23.replace("ki", "qui");
   let str25 = &str24.replace("Ki", "Qui");
   let str26 = &str25.replace("KI", "QUI");
   let str27 = &str26.replace("ke", "que");
   let str28 = &str27.replace("Ke", "Que");
   let str29 = &str28.replace("KE", "QUE");
   let str30 = &str29.replace("kí", "quí");
   let str31 = &str30.replace("Kí", "Quí");
   let str32 = &str31.replace("KÍ", "QUÍ");
   let str33 = &str32.replace("ké", "qué");
   let str34 = &str33.replace("Ké", "Qué");
   let str35 = &str34.replace("KÉ", "QUÉ");
   let str36 = &str35.replace("k", "c");
   let str37 = &str36.replace("K", "C");
   let str38 = &str37.replace("a'", "ax");
   let str39 = &str38.replace("ã'", "ãx");
   let str40 = &str39.replace("a̱'", "a̱x");
   let str41 = &str40.replace("ã̱'", "ã̱x");
   let str42 = &str41.replace("á'", "áx");
   let str43 = &str42.replace("e'", "ex");
   let str44 = &str43.replace("ẽ'", "ẽx");
   let str45 = &str44.replace("e̱'", "e̱x");
   let str46 = &str45.replace("ẽ̱'", "ẽ̱x");
   let str47 = &str46.replace("é'", "éx");
   let str48 = &str47.replace("i'", "ix");
   let str49 = &str48.replace("ĩ'", "ĩx");
   let str50 = &str49.replace("i̱'", "i̱x");
   let str51 = &str50.replace("ĩ̱'", "ĩ̱x");
   let str52 = &str51.replace("í'", "íx");
   let str53 = &str52.replace("o'", "ox");
   let str54 = &str53.replace("õ'", "õx");
   let str55 = &str54.replace("o̱'", "o̱x");
   let str56 = &str55.replace("õ̱'", "õ̱x");
   let str57 = &str56.replace("ó'", "óx");
   let str58 = &str57.replace("u'", "ux");
   let str59 = &str58.replace("ũ'", "ũx");
   let str60 = &str59.replace("u̱'", "u̱x");
   let str61 = &str60.replace("ũ̱'", "ũ̱x");
   let str62 = &str61.replace("ú'", "úx");
   let str63 = &str62.replace("ü'", "üx");
   let str64 = &str63.replace("ü̃'", "ü̃x");
   let str65 = &str64.replace("ü̱'", "ü̱x");
   let str66 = &str65.replace("ü̱̃'", "ü̱̃x");
   let str67 = &str66.replace("ǘ'", "ǘx");
   let str68 = &str67.replace("'", "x");
   let str69 = &str68.replace("a’", "ax");
   let str70 = &str69.replace("ã’", "ãx");
   let str71 = &str70.replace("a̱’", "a̱x");
   let str72 = &str71.replace("ã̱’", "ã̱x");
   let str73 = &str72.replace("á’", "áx");
   let str74 = &str73.replace("e’", "ex");
   let str75 = &str74.replace("ẽ’", "ẽx");
   let str76 = &str75.replace("e̱’", "e̱x");
   let str77 = &str76.replace("ẽ̱’", "ẽ̱x");
   let str78 = &str77.replace("é’", "éx");
   let str79 = &str78.replace("i’", "ix");
   let str80 = &str79.replace("ĩ’", "ĩx");
   let str81 = &str80.replace("i̱’", "i̱x");
   let str82 = &str81.replace("ĩ̱’", "ĩ̱x");
   let str83 = &str82.replace("í’", "íx");
   let str84 = &str83.replace("o’", "ox");
   let str85 = &str84.replace("õ’", "õx");
   let str86 = &str85.replace("o̱’", "o̱x");
   let str87 = &str86.replace("õ̱’", "õ̱x");
   let str88 = &str87.replace("ó’", "óx");
   let str89 = &str88.replace("u’", "ux");
   let str90 = &str89.replace("ũ’", "ũx");
   let str91 = &str90.replace("u̱’", "u̱x");
   let str92 = &str91.replace("ũ̱’", "ũ̱x");
   let str93 = &str92.replace("ú’", "úx");
   let str94 = &str93.replace("ü’", "üx");
   let str95 = &str94.replace("ü̃’", "ü̃x");
   let str96 = &str95.replace("ü̱’", "ü̱x");
   let str97 = &str96.replace("ü̱̃’", "ü̱̃x");
   let str98 = &str97.replace("ǘ’", "ǘx");
   let result = &str98.replace("’", "x");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("PERU (ILV):".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("PERU (ILV):".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Peru (ILV):");
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}

//   ++++++++++   ++++++++++   ++++++++++

// TIKUNA ORTHOGRAPHY: PERU FORMABIAP

pub fn orttcapeformabiap(original_text: &str, usefile: &str, outputfile: &str) {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";

   let str2 = original_text.replace("j", "y");
   let str3 = &str2.replace("J", "Y");
   let str4 = &str3.replace("tch", "ch");
   let str5 = &str4.replace("Tch", "Ch");
   let str6 = &str5.replace("TCH", "CH");
   let str7 = &str6.replace("nh", "ñ");
   let str8 = &str7.replace("Nh", "Ñ");
   let str9 = &str8.replace("NH", "Ñ");
   let str10 = &str9.replace("à", "a");
   let str11 = &str10.replace("ò", "o");
   let str12 = &str11.replace("è", "e");
   let str13 = &str12.replace("ì", "i");
   let str14 = &str13.replace("ù", "u");
   let str15 = &str14.replace("ǜ", "ü");
   let str16 = &str15.replace("À", "A");
   let str17 = &str16.replace("Ò", "O");
   let str18 = &str17.replace("È", "E");
   let str19 = &str18.replace("Ì", "I");
   let str20 = &str19.replace("Ù", "U");
   let str21 = &str20.replace("Ǜ", "Ü");
   let str22 = &str21.replace("á", "a");
   let str23 = &str22.replace("ó", "o");
   let str24 = &str23.replace("é", "e");
   let str25 = &str24.replace("í", "i");
   let str26 = &str25.replace("ú", "u");
   let str27 = &str26.replace("ǘ", "ü");
   let str28 = &str27.replace("Á", "A");
   let str29 = &str28.replace("É", "E");
   let str30 = &str29.replace("Í", "I");
   let str31 = &str30.replace("Ó", "O");
   let str32 = &str31.replace("Ú", "U");
   let str33 = &str32.replace("Ǘ", "Ü");
   let str34 = &str33.replace("c", "k");
   let str35 = &str34.replace("C", "K");
   let str36 = &str35.replace("kh", "ch");
   let str37 = &str36.replace("Kh", "Ch");
   let str38 = &str37.replace("KH", "CH");
   let str39 = &str38.replace("qui", "ki");
   let str40 = &str39.replace("Qui", "Ki");
   let str41 = &str40.replace("QUI", "KI");
   let str42 = &str41.replace("que", "ke");
   let str43 = &str42.replace("Que", "Ke");
   let str44 = &str43.replace("QUE", "KE");
   let str45 = &str44.replace("q", "k");
   let str46 = &str45.replace("Q", "K");
   let str47 = &str46.replace("a'", "ax");
   let str48 = &str47.replace("ã'", "ãx");
   let str49 = &str48.replace("a̱'", "ax");
   let str50 = &str49.replace("ã̱'", "ãx");
   let str51 = &str50.replace("e'", "ex");
   let str52 = &str51.replace("ẽ'", "ẽx");
   let str53 = &str52.replace("e̱'", "ex");
   let str54 = &str53.replace("ẽ̱'", "ẽx");
   let str55 = &str54.replace("i'", "ix");
   let str56 = &str55.replace("ĩ'", "ĩx");
   let str57 = &str56.replace("i̱'", "ix");
   let str58 = &str57.replace("ĩ̱'", "ĩx");
   let str59 = &str58.replace("o'", "ox");
   let str60 = &str59.replace("õ'", "õx");
   let str61 = &str60.replace("o̱'", "ox");
   let str62 = &str61.replace("õ̱'", "õx");
   let str63 = &str62.replace("u'", "ux");
   let str64 = &str63.replace("ũ'", "ũx");
   let str65 = &str64.replace("u̱'", "ux");
   let str66 = &str65.replace("ũ̱'", "ũx");
   let str67 = &str66.replace("ü'", "üx");
   let str68 = &str67.replace("ü̃'", "ü̃x");
   let str69 = &str68.replace("ü̱'", "üx");
   let str70 = &str69.replace("ü̱̃'", "ü̃x");
   let str71 = &str70.replace("A̱", "AX");
   let str72 = &str71.replace("Ã̱", "ÃX");
   let str73 = &str72.replace("E̱", "EX");
   let str74 = &str73.replace("Ẽ̱", "ẼX");
   let str75 = &str74.replace("I̱", "IX");
   let str76 = &str75.replace("Ĩ̱", "ĨX");
   let str77 = &str76.replace("O̱", "OX");
   let str78 = &str77.replace("Õ̱", "ÕX");
   let str79 = &str78.replace("U̱", "UX");
   let str80 = &str79.replace("Ũ̱", "ŨX");
   let str81 = &str80.replace("Ü̱", "ÜX");
   let str82 = &str81.replace("Ü̱̃", "Ü̃X");
   let str83 = &str82.replace("'", "x");
   let str84 = &str83.replace("a’", "ax");
   let str85 = &str84.replace("ã’", "ãx");
   let str86 = &str85.replace("a̱’", "ax");
   let str87 = &str86.replace("ã̱’", "ãx");
   let str88 = &str87.replace("e’", "ex");
   let str89 = &str88.replace("ẽ’", "ẽx");
   let str90 = &str89.replace("e̱’", "ex");
   let str91 = &str90.replace("ẽ̱’", "ẽx");
   let str92 = &str91.replace("i’", "ix");
   let str93 = &str92.replace("ĩ’", "ĩx");
   let str94 = &str93.replace("i̱’", "ix");
   let str95 = &str94.replace("ĩ̱’", "ĩx");
   let str96 = &str95.replace("o’", "ox");
   let str97 = &str96.replace("õ’", "õx");
   let str98 = &str97.replace("o̱’", "ox");
   let str99 = &str98.replace("õ̱’", "õx");
   let str100 = &str99.replace("u’", "ux");
   let str101 = &str100.replace("ũ’", "ũx");
   let str102 = &str101.replace("u̱’", "ux");
   let str103 = &str102.replace("ũ̱’", "ũx");
   let str104 = &str103.replace("ü’", "üx");
   let str105 = &str104.replace("ü̃’", "ü̃x");
   let str106 = &str105.replace("ü̱’", "üx");
   let str107 = &str106.replace("ü̱̃’", "ü̃x");
   let result = &str107.replace("’", "x");

   if usefile == "new" {
   let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
   file.write_all("PERU (FORMABIAP):".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));   
   }
   if usefile == "old" {
   let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
   file.write("PERU (FORMABIAP):".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
   }
   if usefile == "terminal" {
   println!("");
   println!("Peru (FORMABIAP):");
   println!("");
   print!("{}", yellow);
   println!("{}", result);
   print!("{}", reset);
   }
}
