// LNGCNV VERSION 1.9.2 / MIT LICENSE / COPYRIGHT © 2022–2024 PIOTR BAJDEK

// MODULE MODENG_AU

// CLIPPY LINTS

#![deny(clippy::no_effect_replace)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::too_many_lines, clippy::unicode_not_nfc)]
#![allow(clippy::string_lit_as_bytes)]

// IMPORTS

use std::fs::OpenOptions;
use std::io::Write;

// SIMPLIFY INTERPUNCTION

fn engpncbeg(lowercase: &str) -> String {
    let pncbeg = &lowercase
        .replace([';', '!', '?'], ".")
        .replace(").", ".")
        .replace("),", ",")
        .replace([')', ':'], ",")
        .replace("--", " – ")
        .replace('(', "∣ .")
        .replace(' ', "# #")
        .replace('.', "#.#")
        .replace(',', "#,#");
    pncbeg.to_string()
}

// REMOVE INTERPUNCTION

fn engpncend(strmod: &str) -> String {
    let result = &strmod
        .replace("# #", " ")
        .replace("#.#", ".")
        .replace("#,#", ",")
        .replace(',', " ∣")
        .replace(". ", " ∥ ")
        .replace('.', "")
        .replace(" - ", " ∣ ")
        .replace(" – ", " ∣ ")
        .replace("∣ ∣", "∣")
        .replace("∣ ∥", "∥");
    result.to_string()
}

// CANBERRA: IPA

pub fn engaucanberra(original_text: &str, usefile: &str, outputfile: &str, reset: &str, red: &str, cyan: &str, yellow: &str) {
    let dotend = original_text.to_owned() + ".";
    let dotbeg = ".".to_owned() + &dotend;
    let lowercase = &dotbeg.to_lowercase();
    let pncbeg = engpncbeg(lowercase);

    let strmod = &pncbeg
        .replace("i'", "ɑ̝ë")
        .replace("i’", "ɑ̝ë")
        .replace("y's", "is")
        .replace("y’s", "is")
        .replace(['\'', '’'], "")
        .replace("#etc#", "#^etsæ̈t͡ʃɹə#")
        .replace("eah", "æ̈ə")
        .replace("ballet", "bɐ̠lei^")
        .replace("caramel", "kʰɶ̜ɹᵊməɫ")
        .replace("christmas", "krismas")
        .replace("debris", "dibɹᵊiː")
        .replace("entrepreneur", "ɔ̈nt̠ɹ̥əpɹənɘ̹ː")
        .replace("garage", "ɡɶ̜ɹəd͡ʒ")
        .replace("listen", "lisᵊn")
        .replace("often", "ɔ̈fᵊn")
        .replace("perhaps", "peːhɶ̜ps")
        .replace("video", "vidɞ̜ʉ̞")
        .replace("comm", "kʰɔ̈mm")
        .replace("modu", "mɔ̈du")
        .replace("#conf", "#kʰɔ̃nf")
        .replace("#con", "#kʰə̃n")
        .replace("con#", "kə̃n#")
        .replace("dule", "d͡ʒʊ̈le")
        .replace("dula", "d͡ʒʊ̈la")
        .replace("ionaire", "io̝ːnə")
        .replace("national", "nɶ̜tional")
        .replace("itation", "ə^tation")
        .replace("verage", "vrəd͡ʒe")
        .replace("ier#", "ʲə#")
        .replace("pass", "pɶ̜ss")
        .replace("alumi", "ɶ̜lʲʉ̟ːmi")
        .replace("nium", "nʲəm")
        .replace("eum#", "iˑəm#")
        .replace("subtl", "sɐ̠tl")
        .replace("debt", "det")
        .replace("wednes", "we^nz̥")
        .replace("print", "pr^int")
        .replace("yed", "yd̥")
        .replace("bb", "b")
        .replace("cce", "kse")
        .replace("cci", "ksi")
        .replace("iced#", "i^sed#")
        .replace("musc", "mɐ̠sc")
        .replace("ward", "^wo̝ːd")
        .replace("ok", "ɞ̜ʉ̞k")
        .replace("ochu", "ɞ̜ʉ̞chu")
        .replace("chure", "ʃʲə")
        .replace("patri", "pɶ̜tri")
        .replace("parallel", "pɶ̜rəle^l")
        .replace("ership", "əship")
        .replace("cally", "kl̥ᵊiː")
        .replace("ally#", "ᵊl̥ᵊiː#")
        .replace("#all#", "#ɔ̈ːl#")
        .replace("#all", "#əl")
        .replace("all", "ɔ̈ːl")
        .replace("#ali", "#a^li")
        .replace("#alo", "#a^lo")
        .replace("#al", "#ɔ̈l")
        .replace("vase", "vɐ̠ːze")
        .replace("#atmo", "#ɶ̜tmo")
        .replace("#atomic", "#a^tɔ̈mic")
        .replace("#atom", "#ɶ̜tom")
        .replace("#edu", "#ed͡ʒʊ̈")
        .replace("dead", "d^e^d")
        .replace("#dedi", "#d^edi")
        .replace("#defini", "#d^efi^ni")
        .replace("#dehy", "#dᵊiːhɑ̝ë")
        .replace("#dens", "#d^ens")
        .replace("#dent", "#d^ent")
        .replace("#demo", "#d^emo")
        .replace("#dex", "#d^ex")
        .replace("#de", "#di^")
        .replace("dynas", "dənəs")
        .replace("dyna", "dɑ̝ënə")
        .replace("natura", "nɶ̜tura")
        .replace("natur", "næ̈ɪtur")
        .replace("phys", "fɘz̥")
        .replace("psychol", "sɑ̝ëkɔ̈l")
        .replace("psycho", "sɑ̝ëkɞ̜ʉ̞")
        .replace("hydro", "hɑ̝ëd^ɹ̥ə")
        .replace("hypoth", "hɑ̝ëpɔ̈th")
        .replace("hypo", "hɑ̝ëpɞ̜ʉ̞")
        .replace("#hy", "#hɑ̝ë")
        .replace("#nob", "#nɞ̜ʉ̞b")
        .replace("secr", "sikr")
        .replace("scr", "skr")
        .replace("isce", "isə")
        .replace("sce", "se")
        .replace("#cel", "#se^l")
        .replace("#cer", "#ser")
        .replace("#cyg", "#si^g")
        .replace("#cyl", "#si^l")
        .replace("#cym", "#si^m")
        .replace("#cyn", "#si^n")
        .replace("#cys", "#si^s")
        .replace("#cyz", "#si^z")
        .replace("#cyre", "#sɑ̝ëre")
        .replace("#cyr", "#si^r")
        .replace("#cyw", "#ki^w")
        .replace("#cy", "#sɑ̝ë")
        .replace("cen", "sen")
        .replace("dd", "d̥")
        .replace("choir", "kwɑ̝ëə")
        .replace("lost", "lɔ̈st")
        .replace("ost", "ɞ̜ʉ̞st")
        .replace("rgue", "rg^ue")
        .replace("gue", "ge")
        .replace("gui", "gi")
        .replace("#co", "#kʰo")
        .replace("que#", "k#")
        .replace("qu", "kw")
        .replace("rh", "ɹ")
        .replace("#awa", "#əwa")
        .replace("#att", "#a^tt")
        .replace("att", "ɶ̜tt")
        .replace("#a^tt", "#att")
        .replace("ttem", "tʰem")
        .replace("tten", "tʰen")
        .replace("wr", "ɹ")
        .replace("wha", "wɔ̈")
        .replace("dess", "diss")
        .replace("pad", "pɶ̜d")
        .replace("ve# #read", "ve# #ɹeːd")
        .replace("has# #read", "has# #ɹeːd")
        .replace("nt# #read", "nt# #ɹeːd")
        .replace("ered", "əd̥")
        .replace("intere", "inte^re")
        .replace("inter", "intə")
        .replace("paedo", "pedɞ̜ʉ̞")
        .replace("#prefe", "#pr^i^fe")
        .replace("#prefi", "#pr^ifi")
        .replace("#prepa", "#pr^ipa")
        .replace("#prett", "#pr^itt")
        .replace("#prev", "#pr^iv")
        .replace("aric#", "ɶ̜ric#")
        .replace("bare", "beə")
        .replace("bari", "beːɹ̥i")
        .replace("care", "kʰeə")
        .replace("cari", "kʰeːɹ̥i")
        .replace("dare", "deə")
        .replace("dari", "deːɹ̥i")
        .replace("mare", "meə")
        .replace("mari", "meːɹ̥i")
        .replace("pare#", "peə#")
        .replace("rare#", "r^eə#")
        .replace("share", "ʃeə")
        .replace("shari", "ʃeːɹ̥i")
        .replace("tare", "teə")
        .replace("tari", "teːɹ̥i")
        .replace("ware#", "weə#")
        .replace("esta", "ista")
        .replace("ven#", "vᵊn#")
        .replace("azy#", "æ̈ɪzy#")
        .replace("aziest#", "æ̈ɪziest#")
        .replace("oken#", "ɞ̜ʉ̞kən#")
        .replace("ecision", "əsəʒᵊn")
        .replace("ident", "i^dənt")
        .replace("eral", "ərəl")
        .replace("sual", "ʒʉ̟əl")
        .replace("answ", "ɶ̜ns")
        .replace("ans", "ɶ̜ns")
        .replace("sword", "sɘ̹ːd̥")
        .replace("orda", "ɔ̈ːda")
        .replace("orde", "ɔ̈ːde")
        .replace("ordi", "ɔ̈ːdi")
        .replace("ord", "ɘ̹ːd")
        .replace("orld", "ɘ̹ːld")
        .replace("ork", "ɘ̹ːk")
        .replace("orth", "o̝ːθ")
        .replace("orest", "o̝ːɹest")
        .replace("fore", "fɔ̈ː")
        .replace("ore# #a", "o̝ːʳ# #a")
        .replace("ore# #e", "o̝ːʳ# #e")
        .replace("ore# #o", "o̝ːʳ# #o")
        .replace("ore# #ɔ̈", "o̝ːʳ# #ɔ̈")
        .replace("ore", "o̝ː")
        .replace("worse", "wɘ̹ːse")
        .replace("orse", "o̝ːse")
        .replace("orce", "o̝ːce")
        .replace("glory", "glɔ̈ːry")
        .replace("#story", "#stɔ̈ːry")
        .replace("#stori", "#stɔ̈ːri")
        .replace("ory", "ᵊry")
        .replace("#ori", "#əɹi")
        .replace("ori", "ɔ̈ɹi")
        .replace("air", "eːə")
        .replace("there", "ðeː")
        .replace("where", "wheː")
        .replace("were", "weː")
        .replace("ere#", "^iə#")
        .replace("circ", "sɘ̹ːc")
        .replace("ira", "ɑ̝ëra")
        .replace("iro", "ɑ̝ëro")
        .replace("ire", "ɑ̝ëə")
        .replace("irr", "iɹ")
        .replace("irth", "irθ")
        .replace("ir", "ɘ̹ː")
        .replace("isl", "ɑ̝ël")
        .replace("iend", "end")
        .replace("rious", "ɹiəs")
        .replace("cious", "ʃiəs")
        .replace("tious", "ʃiəs")
        .replace("amous", "æ̞̃ɪmous")
        .replace("eous#", "iəs#")
        .replace("ous#", "əs#")
        .replace("ous", "æ̞ɞ̠s")
        .replace("bour", "bə")
        .replace("iour", "j|ə")
        .replace("nour", "nə")
        .replace("olour", "ɔ̈lə")
        .replace("tour", "tʊ̈ə")
        .replace("youre", "j|ʉ̟ə")
        .replace("your", "j|ʉ̟ə")
        .replace("ource", "o̝ːse")
        .replace("ourse", "o̝ːse")
        .replace("four", "fɔ̈ː")
        .replace("fut", "fʲʉ̟ːt")
        .replace("our", "æ̞ɞ̠ə")
        .replace("anal", "anɶ̜l")
        .replace("lysis", "ləsis")
        .replace("thesis", "θəsis")
        .replace("easure", "eːʒə")
        .replace("eisure", "ɜʒə")
        .replace("sure", "ʃʲɔ̈ː")
        .replace("mature", "mat͡ʃʲʉ̟ːə")
        .replace("ture", "t͡ʃə")
        .replace("ureau", "ʲʉ̟ːrɞ̜ʉ̞")
        .replace("ure", "ʲʉ̟ːə")
        .replace("eaux#", "ɞ̜ʉ̞z̥#")
        .replace("eau#", "ɞ̜ʉ̞#")
        .replace("eau", "ʲʉ̟ː")
        .replace("uss", "ɐ̠ss")
        .replace("hurr", "hɐ̠ɹ")
        .replace("flurr", "flɐ̠ɹ")
        .replace("urr", "ɘ̹ːɹ")
        .replace("#uri", "#j|ʉ̟ːɹə")
        .replace("uri", "ə̟ɹi")
        .replace("urth", "eːð")
        .replace("ura", "ʲʉ̟ːra")
        .replace("ur", "ɘ̹ː")
        .replace("#chara", "#kʰɶ̜ra")
        .replace("#cha", "#t͡ʃa")
        .replace("#chi", "#t͡ʃi")
        .replace("ceipt", "ceit")
        .replace("cept", "sept")
        .replace("#own", "#ɞ̜ʉ̞n")
        .replace("own", "æɞ̃n")
        .replace("owl", "æ̃ɞ̠l")
        .replace("#əlow", "#əlæ̞ɞ̠")
        .replace("#kʰow", "#kʰæ̞ɞ̠")
        .replace("#how", "#hæ̞ɞ̠")
        .replace("#wow", "#wæ̞ɞ̠")
        .replace("ehow#", "ehæ̞ɞ̠#")
        .replace("yhow#", "yhæ̞ɞ̠#")
        .replace("#now", "#næ̞ɞ̠")
        .replace("#flower", "#flæ̞ɞ̠er")
        .replace("#power", "#pæ̞ɞ̠er")
        .replace("#shower", "#shæ̞ɞ̠er")
        .replace("#towe", "#tæ̞ɞ̠e")
        .replace("kn", "n")
        .replace("ow", "ɞ̜ʉ̞")
        .replace("wea", "weˑ")
        .replace("head", "he^d")
        .replace("eady", "edy")
        .replace("eace", "ᵊiːse")
        .replace("eas", "ᵊiːz̥")
        .replace("eason", "ᵊiːz̥ᵊn")
        .replace("ears#", "^iəʳz̥#")
        .replace("ear#", "^iə#")
        .replace("eart", "ɐ̠ːt")
        .replace("star", "stɐ̠ːʳ")
        .replace("art", "ɐ̠ːt")
        .replace("earl", "eːʳl")
        .replace("ear", "eː")
        .replace("eali", "ᵊiːəli")
        .replace("who#", "hʉ̟ː#")
        .replace("whom#", "hʉ̟ːm#")
        .replace("whod#", "hʉ̟ːd#")
        .replace("who", "ho")
        .replace("wh", "w")
        .replace("aunt", "ɐ̠ːnt")
        .replace("daughter", "dɔ̈tə")
        .replace("augh", "ɐ̠ːf")
        .replace("ause", "ɔ̈z̥")
        .replace("auth", "ɔ̈ːθ")
        .replace("au", "ɔ̈")
        .replace("any", "æ̃nᵊiː^")
        .replace("arm", "õ̝ːm")
        .replace("arn", "õ̝ːn")
        .replace("br", "bɹ̥")
        .replace("are# #a", "ɐ̠ːʳ# #a")
        .replace("are# #e", "ɐ̠ːʳ# #e")
        .replace("are# #o", "ɐ̠ːʳ# #o")
        .replace("are#", "ɐ̠ː#")
        .replace("eyre", "eː")
        .replace("re#", "ə#")
        .replace("wond", "wɐ̠nd")
        .replace("barb", "bɐ̠ːb")
        .replace("archi", "ɐ̠ːkʰə")
        .replace("arch", "ɐ̠ːk")
        .replace("arge", "ɐ̠ːʳd͡ʒe")
        .replace("#arg", "#ɐ̠ːʳg")
        .replace("arr", "ɶ̜ɹ")
        .replace("#ɶ̜ɹ", "#aɹ")
        .replace("#ar", "#əɹ")
        .replace("car#", "cɐ̠ː#")
        .replace("cars#", "cɐ̠ːʳz̥#")
        .replace("#liar", "#lɑ̝ëə")
        .replace("ara", "ɐ̠ːɹa")
        .replace("ar# #a", "ɐ̠ːʳ# #a")
        .replace("ar# #e", "ɐ̠ːʳ# #e")
        .replace("ar# #o", "ɐ̠ːʳ# #o")
        .replace("ari", "ɐ̠ːɹi")
        .replace("aro", "əro")
        .replace("sona", "sɞ̜ʉ̞na")
        .replace("war#", "wɔ̈#")
        .replace("iar#", "ʲə#")
        .replace("lar#", "lə̞#")
        .replace("ar#", "ɐ̠ː#")
        .replace("ata", "ɐ̠ːta")
        .replace("ato", "ɐ̠ːto")
        .replace("ust", "ɐ̠st")
        .replace("thought", "θought")
        .replace("outh", "æ̞ɞ̠θ")
        .replace("out", "æ̞ɞ̠t")
        .replace("obi", "ɞ̜ʉ̞bi")
        .replace("ock", "ɔ̈ck")
        .replace("uke", "ʲʉ̟ːk")
        .replace("ude", "ʲʉ̟ːd")
        .replace("uga", "ʲʉ̟ːga")
        .replace("uge", "ʲʉ̟ːd͡ʒ")
        .replace("ee", "ᵊiː")
        .replace("ield", "ᵊiːld")
        .replace("aeces", "ᵊiːsᵊiːz̥")
        .replace("eces", "ᵊiːsᵊiːz̥")
        .replace("aeo", "iɔ̈")
        .replace("#ever", "#e^ver")
        .replace("e^very", "e^vᵊry")
        .replace("#evid", "#e^vi^d")
        .replace("#evi^d", "#e^vi^d")
        .replace("#ev", "#iv")
        .replace("cially", "ʃl̥ᵊiː")
        .replace("enly", "ənl̥ᵊiː")
        .replace("ship", "ʃəp")
        .replace("ian", "^iˑən")
        .replace("dia", "dɑ̝ëə")
        .replace("gian", "d͡ʒɑ̝ëən")
        .replace("alian", "æ̈ɪlᵊiˑən")
        .replace("migra", "mɑ̝ëgra")
        .replace("sia#", "ʒʲə#")
        .replace("ia", "iːə")
        .replace("ail", "æ̈ɪɫ")
        .replace("ile", "ɑ̝ëɫe")
        .replace("dance", "dæ̃ːnse")
        .replace("danci", "dæ̃ːnsi")
        .replace("ance", "ənse")
        .replace("anci", "ənsi")
        .replace("ence", "ənse")
        .replace("ience", "iənse")
        .replace("#ask", "#ɶ̜sk")
        .replace("astle", "ɶ̜sᵊl")
        .replace("stle", "sᵊl")
        .replace("trast", "trɶ̜st")
        .replace("and", "æ̃ː^nd")
        .replace("vel#", "vᵊɫ#")
        .replace("simi", "simᵊ")
        .replace("ain", "æ̈ɪn")
        .replace("behav", "bihæ̈ɪv")
        .replace("hav", "hɶ̜v")
        .replace("ave", "æ̈ɪv")
        .replace("avi", "æ̈ɪvi")
        .replace("ame", "æ̞̃ɪm")
        .replace("amin", "amən")
        .replace("ami", "æ̞̃ɪmi")
        .replace("cache", "cæ̈ɪʃ")
        .replace("ache", "æ̈ɪk̥ʰ")
        .replace("ake", "æ̈ɪk̥e")
        .replace("eaki", "æ̈ɪk̥i")
        .replace("aki", "æ̈ɪk̥i")
        .replace("ace", "æ̈ɪs")
        .replace("aci", "æ̈ɪsi")
        .replace("ade", "æ̈ɪd̥")
        .replace("adi", "æ̈ɪd̥i")
        .replace("ange", "æ̈ɪnge")
        .replace("angi", "æ̈ɪngi")
        .replace("ape", "æ̈ɪpe")
        .replace("ase", "æ̈ɪse")
        .replace("basil", "bɶ̜zəl")
        .replace("asi", "æ̈ɪsi")
        .replace("maze", "mæ̃ëze")
        .replace("mazi", "mæ̃ëzi")
        .replace("lla", "læ̈ɪ")
        .replace("nice", "nɑ̝ëse")
        .replace("prove", "prʉ̟ːve")
        .replace("provid", "prəvid")
        .replace("provi", "prʉ̟ːvi")
        .replace("ibe", "ɑ̝ëbe")
        .replace("ide", "ɑ̝ëd̥e")
        .replace("idi", "ɑ̝ëd̥i")
        .replace("ɑ̝ëd̥ea", "ɑ̝ëd̥ᵊiːa")
        .replace("grea", "græ̈ɪ")
        .replace("ɹ̥eak", "ɹ̥æ̈ɪk")
        .replace("steak", "stæ̈ɪk")
        .replace("ea", "ᵊiː")
        .replace("eon", "iːon")
        .replace("water", "wɔ̈ðə")
        .replace("man", "mɶ̜n")
        .replace("shad", "ʃɶ̜d")
        .replace("ckage", "ckid͡ʒe")
        .replace("nage", "nəd͡ʒe")
        .replace("tage", "təd͡ʒe")
        .replace("ssage", "sid͡ʒe")
        .replace("uage", "wid͡ʒe")
        .replace("age", "æ̈ɪd͡ʒe")
        .replace("magi", "mɶ̜d͡ʒi")
        .replace("agi", "æ̈ɪd͡ʒi")
        .replace("ale", "æ̈ɪle")
        .replace("#ico", "#ɑ̝ëko")
        .replace("ike", "ɑ̝ëke")
        .replace("iki", "ɑ̝ëki")
        .replace("ife", "ɑ̝ëf")
        .replace("ime", "ɑ̝ëme")
        .replace("machine", "məʃin")
        .replace("fine", "fɑ̝ëne")
        .replace("line", "lɑ̝ëne")
        .replace("mine", "mɑ̝ëne")
        .replace("nine", "nɑ̝ëne")
        .replace("shine", "shɑ̝ëne")
        .replace("iny", "ɑ̝ëny")
        .replace("ini", "ɑ̝ëni")
        .replace("ison", "isᵊn")
        .replace("itely", "ʔl̥ᵊiː")
        .replace("cite", "sɑ̝ëte")
        .replace("ite", "ɑ̝ëte")
        .replace("una", "ʲʉ̟̃ːna")
        .replace("nate", "nə^te")
        .replace("vate", "və^te")
        .replace("ate", "æ̈ɪte")
        .replace("atic", "ɶ̜tic")
        .replace("ati", "æ̈ɪti")
        .replace("ɹiti", "ɹɑ̝ëti")
        .replace("ita", "ɑ̝ëta")
        .replace("ɑ̝ëtc", "itec")
        .replace("oise", "o̝ɪz̥")
        .replace("oing", "o^ing")
        .replace("oi", "o̝ɪ")
        .replace("aise", "æ̈ɪz̥e")
        .replace("ise", "ɑ̝ëse")
        .replace("risi", "ɹɑ̝ësi")
        .replace("iso", "ɑ̝ësə")
        .replace("iza", "ɑ̝ësa")
        .replace("ize", "ɑ̝ëse")
        .replace("izi", "ɑ̝ësi")
        .replace("pri", "prɑ̝ë")
        .replace("rive", "rɑ̝ëve")
        .replace("rivi", "rɑ̝ëvi")
        .replace("ype", "ɑ̝ëpe")
        .replace("ypi", "ɑ̝ëpi")
        .replace("ybe", "ybᵊiː")
        .replace("aw", "ɔ̈ː")
        .replace("hey", "hay")
        .replace("ay", "æ̈ɪ^")
        .replace("they", "thæ̈ɪ^")
        .replace("rey#", "ræ̈ɪ^#")
        .replace("ɹ̥ey#", "ɹ̥æ̈ɪ^#")
        .replace("ey#", "y#")
        .replace("ey", "æ̈ɪ^")
        .replace("aste", "æ̈ɪst")
        .replace("asthma", "ɶ̜smə")
        .replace("ast", "ɐ̠ːst")
        .replace("etable", "tbɫ")
        .replace("ortable", "tbɫ")
        .replace("shal", "ʃʲɶ̜l")
        .replace("cat#", "cɶ̜t#")
        .replace("cats#", "cɶ̜ts#")
        .replace("flat", "flɶ̜t")
        .replace("lass", "lɶ̜ss")
        .replace("am", "æ̃ːm")
        .replace("sylla", "silə")
        .replace("berra", "brra")
        .replace("able", "æ̈ɪbɫ")
        .replace("cæ̈ɪ", "kʰæ̈ɪ")
        .replace("tæ̈ɪ", "tʰæ̈ɪ")
        .replace("imme", "ə̃mᵊiː")
        .replace("wash", "wɔ̈sh")
        .replace("was", "wɔ̈z̥")
        .replace("ash", "ɶ̜sh")
        .replace("obe", "ɞ̜ʉ̞be")
        .replace("obli", "əblɑ̝ë")
        .replace("prog", "prɞ̜ʉ̞g")
        .replace("prob", "prɔ̈b")
        .replace("ob", "əb")
        .replace("oubl", "ɐ̠bl")
        .replace("ble#", "bɫ#")
        .replace("eigh", "æ̈ɪ^")
        .replace("igh", "ɑ̝ë^")
        .replace("die", "d̥ɑ̝ë^")
        .replace("lie", "lɑ̝ë^")
        .replace("tie", "tɑ̝ë^")
        .replace("piece", "pᵊiːse")
        .replace("pie", "pɑ̝ë^")
        .replace("iet", "ɑ̝ëət")
        .replace("bio", "bɑ̝ëo")
        .replace("by", "bɑ̝ë^")
        .replace("my", "mɑ̝ë^")
        .replace("micro", "mɑ̝ëkrɞ̜ʉ̞")
        .replace("ried#", "ryed#")
        .replace("cry", "kɹ̥ɑ̝ë^")
        .replace("dry", "d̠͡zɹ̥ɑ̝ë^")
        .replace("fly", "flɑ̝ë^")
        .replace("shy", "ʃɑ̝ë^")
        .replace("sky", "skɑ̝ë^")
        .replace("#ign", "#i^gn")
        .replace("ign", "ɑ̝ën")
        .replace("#ind", "#i^nd")
        .replace("ind", "ɑ̝ënd")
        .replace("#i^nd", "#ind")
        .replace("etry", "etrᵊiː")
        .replace("ltry", "ltrᵊiː")
        .replace("ntry", "ntrᵊiː")
        .replace("stry", "strᵊiː")
        .replace("try", "trɑ̝ë^")
        .replace("why", "wɑ̝ë^")
        .replace("tri", "t͡ʃɹi")
        .replace("str", "sṯɹ̥")
        .replace("iche", "iːʃ")
        .replace("uel", "ʲʉ̟ːᵊl")
        .replace("ue", "ʲʉ̟ː")
        .replace("oa", "ɞ̜ʉ̞")
        .replace("ɞ̜ʉ̞l", "ɔ̈oɫ")
        .replace("ole", "ɔ̈oɫ")
        .replace("ool", "üːɫ")
        .replace("ull", "ʊ̈ɫ")
        .replace("oll", "ɔ̈ɫ")
        .replace("old", "ɔ̈od")
        .replace("ont#", "ɞ̜ʉ̞nt#")
        .replace("ond", "ənd̥")
        .replace("go#", "gɞ̜ʉ̞#")
        .replace("go^", "gɞ̜ʉ̞^")
        .replace("goes#", "gɞ̜ʉ̞z̥#")
        .replace("shoe", "shʉ̟ː")
        .replace("toe", "tɞ̜ʉ̞")
        .replace("oes", "ɐ̠s")
        .replace("cous", "kʰɐ̠s")
        .replace("touch", "tɐ̠t͡ʃ")
        .replace("oung", "ɐ̠ng")
        .replace("ouch", "æ̞ɞ̠t͡ʃ")
        .replace("oud", "æ̞ɞ̠d")
        .replace("oubt", "æ̞ɞ̠t")
        .replace("ought", "ɔ̈t")
        .replace("ogh", "ɞ̜ʉ̞g")
        .replace("though", "thɞ̜ʉ̞")
        .replace("ough", "ɐ̠f")
        .replace("ug", "ɐ̠g")
        .replace("eno", "ino")
        .replace("mould", "mɞ̜ʉ̞ld")
        .replace("ould", "ʊ̈d")
        .replace("ountr", "ɐ̠ntr")
        .replace("oun", "æɞ̃n")
        .replace("ou", "ʉ̟ː")
        .replace("ooe", "ʉ̟ː")
        .replace("lood", "lɐ̠d")
        .replace("oor#", "ɔ̈ː#")
        .replace("oor", "ɔ̈ːr")
        .replace("oo", "ʊ̈")
        .replace("ose", "ɞ̜ʉ̞se")
        .replace("ocus", "ɞ̜ʉ̞kəs")
        .replace("use", "ʲʉ̟ːz")
        .replace("#uʒ", "#ʲʉ̟ːʒ")
        .replace("#us", "#ʲʉ̟ːs")
        .replace("#busi", "#bɘsi")
        .replace("usi", "ʲʉ̟ːzi")
        .replace("actu", "ɶ̜kʃʉ̟ː")
        .replace("act", "ɶ̜kt")
        .replace("alf", "ɐ̠ːf")
        .replace("alk", "ɔ̈ːk")
        .replace("alm", "ɐ̞̃ːm")
        .replace("geo", "d͡ʒiɔ̈")
        .replace("gen#", "d͡ʒən#")
        .replace("gen", "d͡ʒen")
        .replace("gic", "d͡ʒic")
        .replace("eith", "ɑ̝ëð")
        .replace("ewe", "ʲʉ̟ːe")
        .replace("ew", "ʲʉ̟ː")
        .replace("une", "ʲʉ̟̃ːne")
        .replace("uni", "ʲʉ̟̃ːni")
        .replace("tʲʉ̟̃ːn", "t͡ʃʉ̟̃ːn")
        .replace("dʲʉ̟̃ːn", "d͡ʒʉ̟̃ːn")
        .replace("ume", "ʲʉ̃ːm")
        .replace("umi", "ʲʉ̃ːmi")
        .replace("assʲʉ̃ːm", "əʃʉ̃ːm")
        .replace("upi", "ʲʉ̟ːpi")
        .replace("syd", "sid")
        .replace("ute", "ʲʉ̟ːte")
        .replace("uti", "ʲʉ̟ːti")
        .replace("cove", "cɐ̠ve")
        .replace("kʰove", "kʰɐ̠ve")
        .replace("move", "mʉ̟ːv")
        .replace("movi", "mʉ̟ːvi")
        .replace("love", "lɔ̜ve")
        .replace("lovi", "lɔ̜vi")
        .replace("#kʰome", "#kʰɔ̜me")
        .replace("#kʰomi", "#kʰɔ̜mi")
        .replace("to# #dove", "to# #dɞ̜ʉ̞v")
        .replace("dove", "dɔ̜v")
        .replace("ove", "ɞ̜ʉ̞ve")
        .replace("uck", "ɐ̠k")
        .replace("some", "sɐ̠m")
        .replace("ome", "ɞ̜ʉ̞m")
        .replace("omo", "ɞ̜ʉ̞mɞ̜ʉ̞")
        .replace("opef", "ɞ̜ʉ̞pf")
        .replace("opel", "ɞ̜ʉ̞pl")
        .replace("ope", "ɞ̜ʉ̞pe")
        .replace("opi", "ɞ̜ʉ̞pi")
        .replace("ode", "ɞ̜ʉ̞d̥e")
        .replace("ote", "ɞ̜ʉ̞t")
        .replace("oti", "ɞ̜ʉ̞ti")
        .replace("oto", "ɞ̜ʉ̞to")
        .replace("ony", "ɞ̜ʉ̞ny")
        .replace("onic", "ɔ̈nɘkʰ")
        .replace("uild", "i^ld")
        .replace("ildren", "il̥^d̠͡zɹ̥ən")
        .replace("dr", "d̠͡zɹ̥")
        .replace("ult", "ɐ̠ɫt^")
        .replace("ild", "ɑ̝ëld")
        .replace("uilt", "ilt")
        .replace("nuin", "nʲʉ̟ːin")
        .replace("uin", "ʉ̟ːin")
        .replace("cuit", "kʰit")
        .replace("guit", "git")
        .replace("uit", "ʉ̟ːt")
        .replace("off", "ɔ̈f")
        .replace("of", "ɔ̈v")
        .replace("uff", "ɐ̠f")
        .replace("uy", "ɑ̝ë")
        .replace("sson", "sᵊn")
        .replace("gone", "gɔ̃n")
        .replace("pon", "pɔ̃n")
        .replace("phon", "fɔ̃n")
        .replace("bone", "bɞ̜ʉ̞ne")
        .replace("lone", "lɞ̜ʉ̞ne")
        .replace("tone", "tɞ̜ʉ̞ne")
        .replace("tona", "tɞ̜ʉ̞na")
        .replace("tonge", "tɐ̃ŋ")
        .replace("#to", "#tə^")
        .replace("ton", "tɔ̃n")
        .replace("zone", "zɞ̜ʉ̞ne")
        .replace("#zon", "#zɞ̜ʉ̞n")
        .replace("zon", "zɔ̃n")
        .replace("once", "wɐ̃ns")
        .replace("done", "dɐ̃n")
        .replace("none", "nɐ̃n")
        .replace("one", "wɐ̃n")
        .replace("#only", "#ɞ̜ʉ̞nly")
        .replace("#on", "#ɔ̈n")
        .replace("engl", "ingl")
        .replace("ng#", "ŋ#")
        .replace("finge", "fing^e")
        .replace("linge", "ling^e")
        .replace("ringe", "ring^e")
        .replace("singe", "sing^e")
        .replace("onge", "ong^e")
        .replace("ng", "ŋg")
        .replace("æ̈ɪŋgiŋ", "æ̈ɪŋd͡ʒiŋ")
        .replace("aŋgel", "æ̈ɪŋd͡ʒel")
        .replace("ŋge", "ŋd͡ʒe")
        .replace("ŋgiŋ", "ŋiŋ")
        .replace("igi", "id͡ʒi")
        .replace("sono", "sɔ̈nə")
        .replace("sonn", "sɔ̈nn")
        .replace("son", "sɐ̃n")
        .replace("aŋ", "ɶ̜̃ŋ")
        .replace("i^iŋ", "iŋ")
        .replace("iŋ", "ĩŋ")
        .replace("oŋ", "ɔ̃ŋ")
        .replace("atus", "æ̈ɪtəs")
        .replace("cement", "smənt")
        .replace("lement", "ləmənt")
        .replace("ement#", "mənt#")
        .replace("#men", "#me^n")
        .replace("ment#", "mənt#")
        .replace("ements#", "mənts#")
        .replace("ments#", "mənts#")
        .replace("ener", "ᵊnə")
        .replace("ene", "iːn")
        .replace("en", "en̪")
        .replace("in", "ĩn")
        .replace("im", "ĩm")
        .replace("un", "ɐ̃n")
        .replace("uŋ", "ɐ̃ŋ")
        .replace("up", "ɐ̠p")
        .replace("ub", "ɐ̠b")
        .replace("ute", "ʲʉ̟ːte")
        .replace("uto", "ʲʉ̟ːto")
        .replace("put", "pʊ̈t")
        .replace("uth", "ʉθ")
        .replace("month", "mɐ̠nθ")
        .replace("bath#", "bɐ̠ːθ#")
        .replace("#path", "#pɐ̠ːθ")
        .replace("path#", "pɶ̜θ#")
        .replace("bab#", "bɶ̜b#")
        .replace("uct", "ɐ̠ct")
        .replace("ut", "ɐ̠t")
        .replace("ud", "ɐ̠d")
        .replace("but#", "bət#")
        .replace("ak", "ɶ̜k")
        .replace("ack", "ɶ̜k")
        .replace("ai", "æ̈ɪ^")
        .replace("ik", "ikʰ")
        .replace("this", "ðis^")
        .replace("is#", "iz̥#")
        .replace("ẽ̞n̪th", "ẽn̪θ")
        .replace("othes", "ɞ̜ʉ̞ðz")
        .replace("oth", "ɐ̠ð")
        .replace("ythm", "iðᵊm")
        .replace("yme", "ɑ̝ëme")
        .replace("gh", "g")
        .replace("than", "ðə̃n̪")
        .replace("thẽn̪", "ðẽn̪")
        .replace("these", "ðiːz")
        .replace("the#", "ðə#")
        .replace("thi", "θi")
        .replace("th", "ð")
        .replace("two", "tʰʉ̟ː")
        .replace("pt", "p̚t")
        .replace("spu", "spʲʉ̟ː")
        .replace("pu", "pʰu")
        .replace("pʊ̈", "pʰʊ̈")
        .replace("tʊ̈", "tʰʊ̈")
        .replace("ief", "ᵊiːf")
        .replace("dge", "d͡ʒe")
        .replace("cant", "kʰɐ̞̃ːnt")
        .replace("rant", "^rənt")
        .replace("want", "wɔ̃nt")
        .replace("#ant", "#æ̃ː^nt")
        .replace("can", "kʰæ̃ːn")
        .replace("ken", "kʰẽn")
        .replace("ank", "æ̃ːnk")
        .replace("fan", "fæ̃ːn")
        .replace("an#", "æ̃ːn#")
        .replace("#æ̃ːn", "#an")
        .replace("oolish", "üːlish")
        .replace("olish", "ɞ̜ʉ̞lish")
        .replace("ish", "ɘʃ")
        .replace("lush", "lɐ̠ʃ")
        .replace("rush", "rɐ̠ʃ")
        .replace("ush", "ʊ̈ʃ")
        .replace("she#", "ʃᵊiː#")
        .replace("shes#", "ʃᵊiːz#")
        .replace("sh", "ʃ")
        .replace("we#", "wᵊiː#")
        .replace("he#", "hᵊiː#")
        .replace("hes#", "hᵊiːz#")
        .replace("#hiz̥", "#h^iz̥")
        .replace("#hic", "#h^ic")
        .replace("#hil", "#h^il")
        .replace("#hĩm", "#h^ĩm")
        .replace("#hĩndu", "#h^ĩndu")
        .replace("#hĩŋd͡ʒ", "#h^ĩŋd͡ʒ")
        .replace("#hip", "#h^ip")
        .replace("#hit", "#h^it")
        .replace("#his", "#h^is")
        .replace("#hi", "#hɑ̝ë^")
        .replace("sto", "stɔ̈")
        .replace("nf", "ɱf")
        .replace("from", "frɔ̃m")
        .replace("oɱ", "ə̃ɱ")
        .replace("om", "ə̃m")
        .replace("bad", "bɶ̜d̥")
        .replace("hab", "hɶ̜b")
        .replace("had", "hɶ̜d")
        .replace("gla", "ɡlɶ̜")
        .replace("gra", "ɡɹ̥ɶ̜")
        .replace("suk", "sək")
        .replace("app", "ɶ̜p")
        .replace("ap", "ɶ̜p")
        .replace("#it#", "#ə^t̚#")
        .replace("rot", "ɹət")
        .replace("sys", "səs")
        .replace("ot", "ɔ̈t")
        .replace("ert", "eːt")
        .replace("er# #a", "əɹ# #a")
        .replace("er# #e", "əɹ# #e")
        .replace("er# #o", "əɹ# #o")
        .replace("ers#", "əʳz̥#")
        .replace("rs#", "ʳz̥#")
        .replace("er#", "ə#")
        .replace("erl", "əl")
        .replace("em#", "əm#")
        .replace("elɑ̝ëness#", "linəz̥#")
        .replace("sĩness#", "z̥nəz̥#")
        .replace("ness#", "nəz̥#")
        .replace("ussy", "ʊ̈sy")
        .replace("usy", "ɘz̥y")
        .replace("est#", "əst#")
        .replace("bəst", "b^est")
        .replace("rəst", "r^est")
        .replace("uəst", "est")
        .replace("wəst", "west")
        .replace("ess", "ez̥")
        .replace("oss", "ɔ̈z̥")
        .replace("ss", "s")
        .replace("ses", "zəz̥")
        .replace("zes", "zəz̥")
        .replace("yes#", "yez̥#")
        .replace("es#", "z̥#")
        .replace("əs#", "əz̥#")
        .replace("sas", "zəz̥")
        .replace("as#", "ɶ̜z̥#")
        .replace("ks#", "kz̥#")
        .replace("#bed", "#b^e^d")
        .replace("ped#", "p̚t#")
        .replace("d͡ʒed", "d͡ʒd̚")
        .replace("d͡ʒd", "d͡ʒd̚")
        .replace("ked", "kt")
        .replace("k̥ed", "kt")
        .replace("ed#", "ᵊd̥#")
        .replace("bəd̥", "bed̥")
        .replace("bod", "bɔ̈d")
        .replace("god", "gɔ̈d")
        .replace("dop", "dɔ̈p")
        .replace("#be#", "#bᵊiː#")
        .replace("#bet", "#b^et")
        .replace("#bev", "#b^ev")
        .replace("#be", "#bi")
        .replace("#me#", "#mi^#")
        .replace("t͡ʃa", "t͡ʃɶ̜")
        .replace("nal", "nᵊl")
        .replace("ral", "ɹᵊl")
        .replace("#re", "#ɹi")
        .replace("uism#", "əizəm#")
        .replace("ation", "æ̈ɪʃŋ")
        .replace("ition", "ətion")
        .replace("tion", "ʃŋ")
        .replace("ision", "iʒᵊn")
        .replace("sion", "ʃŋ")
        .replace("zion", "ʒᵊŋ")
        .replace("cky", "kiː")
        .replace("ctly", "t͡ʃl̥ᵊiː")
        .replace("tely", "ʔl̥ᵊiː")
        .replace("llery", "l̥ɹᵊiː")
        .replace("lyr", "li̠r")
        .replace("#ly", "#lɑ̝ë^")
        .replace("ely", "l̥ᵊiː")
        .replace("ily", "l̥ᵊiː")
        .replace("ly", "l̥ᵊiː")
        .replace("fy", "fɑ̝ë")
        .replace("el#", "əɫ#")
        .replace("els#", "əɫz̥#")
        .replace("ll", "l")
        .replace("ls#", "ɫz̥#")
        .replace("ical", "ikᵊl")
        .replace("əmic#", "ɶ̜məkʰ#")
        .replace("ic#", "əkʰ#")
        .replace("ce#", "s#")
        .replace("#ive#", "#ɑ̝ëv#")
        .replace("five", "fɑ̝ëv")
        .replace("live", "lɑ̝ëv")
        .replace("vive", "vɑ̝ëv")
        .replace("e#", "#")
        .replace("#do#", "#dʊ̈#")
        .replace("#do^", "#dʊ̈^")
        .replace("#to#", "#tə#")
        .replace("o#", "ɞ̜ʉ̞#")
        .replace("os#", "ɞ̜ʉ̞z̥#")
        .replace("θor", "θə")
        .replace("#or#", "#ɔ̈ː#")
        .replace("or#", "ə#")
        .replace("or", "ɔ̈ː")
        .replace("to", "tə")
        .replace("cy#", "sᵊiː#")
        .replace("logy", "ləd͡ʒᵊiː")
        .replace("uoy", "oy")
        .replace("oy", "o̝ɪ")
        .replace("o̝ɪs", "o̝ɪz̥")
        .replace("y#", "ᵊiː#")
        .replace("yr", "ᵊiːr")
        .replace("yw", "ᵊiːw")
        .replace("e-", "iː")
        .replace("#eco", "#iːkɞ̜ʉ̞")
        .replace("ci", "si")
        .replace("caf", "cɶ̜f")
        .replace("eful", "fʊ̈ɫ")
        .replace("ful", "fʊ̈ɫ")
        .replace("diff", "dif")
        .replace("uard", "ɐ̠ːd")
        .replace("ard", "ɐ̠ːd")
        .replace("fé", "fɶi")
        .replace("mach", "məkʰ")
        .replace("nnel", "nnᵊl")
        .replace("watch", "wɔ̈t͡ʃ")
        .replace("atch", "ɶ̜t͡ʃ")
        .replace("tch", "t͡ʃ")
        .replace("ch#", "t͡ʃ#")
        .replace("chɶ̜", "t͡ʃɶ̜")
        .replace("chʊ̈", "t͡ʃʊ̈")
        .replace("chec", "t͡ʃec")
        .replace("cho", "t͡ʃo")
        .replace("ch", "k")
        .replace("cc", "k")
        .replace("ck#", "kʰ#")
        .replace("ck", "k")
        .replace('c', "k")
        .replace("kɶ̜", "kʰɶ̜")
        .replace("kl", "kl̥")
        .replace("fr", "fɹ̥")
        .replace("gr", "ɡɹ̥")
        .replace("tr", "ṯɹ̥")
        .replace("um", "ɐ̃m")
        .replace("pʰut", "pʰʊ̈t")
        .replace("ʊ̈k", "ʊ̈kʰ")
        .replace("ɶ̜k", "ɶ̜kʰ")
        .replace("dɹ", "d͡ʒɹ̥")
        .replace("dst", "d.st")
        .replace("ds", "d̠͡z")
        .replace("d̥s", "d̠͡z")
        .replace("ns#", "nz̥#")
        .replace("ns#", "nz̥#")
        .replace("ps#", "pz̥#")
        .replace("ts#", "tz̥#")
        .replace("ys#", "yz̥#")
        .replace("ŋs", "ŋs̬")
        .replace("ë^s#", "ëz̥#")
        .replace("ës#", "ëz̥#")
        .replace("erv", "ɘ̹ːʳv")
        .replace("exp", "iksp")
        .replace("exs", "eks")
        .replace("ext", "ekst")
        .replace("ex", "ɘ̟gz")
        .replace('x', "gz")
        .replace('s', "s̻")
        .replace("s̻̬", "s̬")
        .replace("i#", "ɑ̝ë#")
        .replace("ph", "f")
        .replace("ff", "f")
        .replace("pp", "p")
        .replace("pf", "f")
        .replace("oh#", "ɞ̜ʉ̞#")
        .replace("h#", "#")
        .replace("mm", "m")
        .replace("nb", "mb")
        .replace("mb#", "m#")
        .replace("ɶ̜n", "æ̃ːn")
        .replace("ɶ̜̃n", "æ̃ːn")
        .replace("umn", "ᵊm")
        .replace("ymn", "im")
        .replace("mn#", "m#")
        .replace("nn", "n")
        .replace("s̻nt", "zᵊnt")
        .replace('j', "d͡ʒ")
        .replace("d͡ʒ|", "j")
        .replace("#ʲʉ̟ː", "#jʉ̟ː")
        .replace("#ʲʉ̟̃ː", "#jʉ̟̃ː")
        .replace("at# #an", "aɹ# #an")
        .replace('y', "j")
        .replace(['a', 'o'], "ə")
        .replace("ɔ̈ə", "ɔ̈o")
        .replace("ə̝ɪ", "o̝ɪ")
        .replace("ə̝ː", "o̝ː")
        .replace('e', "e̽")
        .replace("e̽̈", "ë")
        .replace("e̽̃", "ẽ")
        .replace("ft", "ft̪")
        .replace("ət", "ət̪")
        .replace("ət̪#.#", "ɶ̜t̪#.#")
        .replace("ət̪#,#", "ɶ̜t̪#,#")
        .replace("t#", "t̚#")
        .replace("t̚# #j", "t̠# #j")
        .replace("tt", "ð")
        .replace("l#", "ɫ#")
        .replace("üːɫ#", "üːɫ̚#")
        .replace("ʊ̈ɫ#", "ʊ̈ɫ̚#")
        .replace("oɫ#", "oɫ̚#")
        .replace("pr", "pɹ̥")
        .replace("s̻pɹ̥", "s̻pɹ")
        .replace("rr", "ɹ")
        .replace('r', "ɹ")
        .replace("e̽ɹ", "e̽ɹ̥")
        .replace("e̽t̚# #æ̞", "e̽ɹ̪# #æ̞")
        .replace("nk", "ŋk")
        .replace("v#", "ʋ#")
        .replace("ɑ̝e̽ɫ", "ɑ̝e̽ᵊɫ")
        .replace("iːɫ", "iːᵊɫ")
        .replace("d# #j", "d̠͡z# #j")
        .replace("ʉ̟ːld", "ʉ̟ːd̥")
        .replace("ld", "l̥d")
        .replace("lw", "l̥w")
        .replace("lm", "ɫm")
        .replace("d# #ð", "d̪̚# #ð")
        .replace("d#", "d̥#")
        .replace("d̥d̥#", "dəd̥#")
        .replace("nd̥# #ɑ̝", "n# #ɑ̝")
        .replace("ðə# #ə", "ðiː# #ə")
        .replace("ðə# #ɐ̠", "ðiː# #ɐ̠")
        .replace("ðə# #e̽", "ðiː# #e̽")
        .replace("ðə# #j", "ðiː# #j")
        .replace("e̽ː# #i", "e̽ːʳ# #i")
        .replace("is̻", "^i̠s̻")
        .replace("is̬", "^i̠s̬")
        .replace("iz̥", "^i̠z̥")
        .replace("iɫ", "ɪɫ")
        .replace("ë^e̽z̥#", "ëz̥#")
        .replace("#i", "#ᵊiː")
        .replace("n# #m", "m# #m")
        .replace("n# #n", "# #n")
        .replace("iː̃", "ĩː")
        .replace("ʰʰ", "ʰ")
        .replace("o̝ːs̻e̽", "o̝ːs̻")
        .replace("gg", "g")
        .replace('g', "ɡ")
        .replace('^', "");

    let result = engpncend(strmod);

    if usefile == "new" {
        let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
        file.write_all("CANBERRA, ACT, AU:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "old" {
        let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
        file.write_all("CANBERRA, ACT, AU:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("{}", cyan.to_owned() + "Canberra" + reset + ", " + cyan + "ACT" + reset + ", " + cyan + "AU" + reset + ":");
        println!();
        print!("{yellow}");
        println!("{result}");
        print!("{reset}");
    }
}
