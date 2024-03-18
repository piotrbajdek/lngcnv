// LNGCNV VERSION 1.9.0 / MIT LICENSE / COPYRIGHT © 2022–2024 PIOTR BAJDEK

// MODULE MODENG_US

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

// DALLAS: IPA

pub fn engusdallas(original_text: &str, usefile: &str, outputfile: &str, reset: &str, red: &str, cyan: &str, yellow: &str) {
    let dotend = original_text.to_owned() + ".";
    let dotbeg = ".".to_owned() + &dotend;
    let lowercase = &dotbeg.to_lowercase();
    let pncbeg = engpncbeg(lowercase);

    let strmod = &pncbeg
        .replace("i'", "æ̈ː")
        .replace("i’", "æ̈ː")
        .replace(['\'', '’'], "")
        .replace("#etc#", "#^etsæ̈t͡ʃɹə#")
        .replace("eah", "æ̈ə")
        .replace("ballet", "bɐ̠lei^")
        .replace("caramel", "ˈkʰæɹməɫ")
        .replace("christmas", "krismas")
        .replace("debris", "dibɹᵊiː")
        .replace("entrepreneur", "ɔ̈nt̠ɹ̥əpɹənɘ̹ː")
        .replace("garage", "ˈɡɐ̠ɹəd͡ʒ")
        .replace("listen", "lisᵊn")
        .replace("often", "ɔ̈fᵊn")
        .replace("perhaps", "peːhæps")
        .replace("video", "vidɔʊ")
        .replace("comm", "kʰɔ̈mm")
        .replace("modu", "mɔ̈du")
        .replace("#conf", "#kʰɔ̃nf")
        .replace("#con", "#kʰə̃n")
        .replace("con#", "kə̃n#")
        .replace("dule", "d͡ʒʊ̈le")
        .replace("dula", "d͡ʒʊ̈la")
        .replace("ionaire", "io̝ːnə")
        .replace("national", "nætional")
        .replace("itation", "ə^tation")
        .replace("verage", "vrəd͡ʒe")
        .replace("ier#", "ʲə#")
        .replace("pass", "pæss")
        .replace("alumi", "ælʲʊ̈ːmi")
        .replace("nium", "nʲəm")
        .replace("eum#", "iˑəm#")
        .replace("subtl", "sɜ̹tl")
        .replace("debt", "det")
        .replace("wednes", "we^nz̥")
        .replace("print", "pr^int")
        .replace("yed", "yd̥")
        .replace("bb", "b")
        .replace("cce", "kse")
        .replace("cci", "ksi")
        .replace("iced#", "i^sed#")
        .replace("musc", "mɜ̹sc")
        .replace("ward", "^wo̝ːd")
        .replace("ok", "ɔʊk")
        .replace("ochu", "ɔʊchu")
        .replace("chure", "ʃʲə")
        .replace("patri", "pætri")
        .replace("parallel", "pærəle^l")
        .replace("ership", "əship")
        .replace("cally", "kl̥ᵊiː")
        .replace("ally#", "ᵊl̥ᵊiː#")
        .replace("#all#", "#ɔ̈ːl#")
        .replace("#all", "#əl")
        .replace("all", "ɔ̈ːl")
        .replace("#ali", "#a^li")
        .replace("#alo", "#a^lo")
        .replace("#al", "#ɔ̈l")
        .replace("#atmo", "#ætmo")
        .replace("#atomic", "#a^tɔ̈mic")
        .replace("#atom", "#ætom")
        .replace("#edu", "#ed͡ʒʊ̈")
        .replace("dead", "d^e^d")
        .replace("#dedi", "#d^edi")
        .replace("#defini", "#d^efi^ni")
        .replace("#dehy", "#dᵊiːhæ̈ː")
        .replace("#dens", "#d^ens")
        .replace("#dent", "#d^ent")
        .replace("#demo", "#d^emo")
        .replace("#dex", "#d^ex")
        .replace("#de", "#di^")
        .replace("natura", "nætura")
        .replace("natur", "ne̞ɪtur")
        .replace("phys", "fɘz̥")
        .replace("psychol", "sæ̈ëkɔ̈l")
        .replace("psycho", "sæ̈ëkɔʊ")
        .replace("hydro", "hæ̈ːd^ɹ̥ə")
        .replace("hypoth", "hæ̈ëpɔ̈th")
        .replace("hypo", "hæ̈ëpɔʊ")
        .replace("#hy", "#hæ̈ː")
        .replace("secr", "sikr")
        .replace("scr", "skr")
        .replace("isce", "isə")
        .replace("sce", "se")
        .replace("#cel", "#se^l")
        .replace("#cer", "#ser")
        .replace("#cyg", "#sɪg")
        .replace("#cyl", "#sɪl")
        .replace("#cym", "#sɪm")
        .replace("#cyn", "#sɪn")
        .replace("#cys", "#sɪs")
        .replace("#cyz", "#sɪz")
        .replace("#cyre", "#sæ̈ëre")
        .replace("#cyr", "#sɪr")
        .replace("#cyw", "#kɪw")
        .replace("#cy", "#sæ̈ë")
        .replace("cen", "sen")
        .replace("dd", "d̥")
        .replace("choir", "kwæ̈ːə")
        .replace("lost", "lɔ̈st")
        .replace("ost", "ɔʊst")
        .replace("rgue", "rg^ue")
        .replace("gue", "ge")
        .replace("gui", "gi")
        .replace("#co", "#kʰo")
        .replace("que#", "k#")
        .replace("qu", "kw")
        .replace("rh", "ɹ")
        .replace("#awa", "#əwa")
        .replace("#att", "#a^tt")
        .replace("att", "ætt")
        .replace("#a^tt", "#att")
        .replace("ttem", "tʰem")
        .replace("tten", "tʰen")
        .replace("wr", "ɹ")
        .replace("wha", "wɔ̈")
        .replace("dess", "diss")
        .replace("pad", "pæd")
        .replace("ve# #read", "ve# #ɹeːd")
        .replace("has# #read", "has# #ɹeːd")
        .replace("nt# #read", "nt# #ɹeːd")
        .replace("ered", "əd̥")
        .replace("intere", "inte^re")
        .replace("inter", "intə")
        .replace("paedo", "pedɔʊ")
        .replace("#prefe", "#pr^i^fe")
        .replace("#prefi", "#pr^ifi")
        .replace("#prepa", "#pr^ipa")
        .replace("#prett", "#pr^itt")
        .replace("#prev", "#pr^iv")
        .replace("aric#", "æric#")
        .replace("bare", "beə")
        .replace("bari", "beːɹ̥i")
        .replace("care", "kʰeə")
        .replace("cari", "kʰeːɹ̥i")
        .replace("dare", "deə")
        .replace("dari", "deːɹ̥i")
        .replace("mare", "meə")
        .replace("mari", "meːɹ̥i")
        .replace("pare#", "peə#")
        .replace("share", "ʃeə")
        .replace("shari", "ʃeːɹ̥i")
        .replace("tare", "teə")
        .replace("tari", "teːɹ̥i")
        .replace("ware#", "weə#")
        .replace("esta", "ista")
        .replace("ven#", "vᵊn#")
        .replace("azy#", "e̞ɪzy#")
        .replace("aziest#", "e̞ɪziest#")
        .replace("oken#", "ɔʊkən#")
        .replace("ecision", "əsəʒᵊn")
        .replace("ident", "i^dənt")
        .replace("eral", "ərəl")
        .replace("sual", "ʒɵ̠→əl")
        .replace("answ", "æns")
        .replace("ans", "æns")
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
        .replace("ira", "æ̈ːra")
        .replace("iro", "æ̈ːro")
        .replace("ire", "æ̈ːə")
        .replace("irr", "iɹ")
        .replace("irth", "irθ")
        .replace("ir", "ɘ̹ː")
        .replace("isl", "æ̈ːl")
        .replace("iend", "end")
        .replace("rious", "ɹiəs")
        .replace("cious", "ʃiəs")
        .replace("tious", "ʃiəs")
        .replace("amous", "æ̞̃ɪmous")
        .replace("eous#", "iəs#")
        .replace("ous#", "əs#")
        .replace("ous", "æɔ̈s")
        .replace("bour", "bə")
        .replace("iour", "j|ə")
        .replace("nour", "nə")
        .replace("olour", "ɔ̈lə")
        .replace("tour", "tʊ̈ə")
        .replace("youre", "j|ɵ̠→ə")
        .replace("your", "j|ɵ̠→ə")
        .replace("ource", "o̝ːse")
        .replace("ourse", "o̝ːse")
        .replace("four", "fɔ̈ː")
        .replace("fut", "fʲʊ̈ːt")
        .replace("our", "æɔ̈ə")
        .replace("anal", "anæl")
        .replace("lysis", "ləsis")
        .replace("thesis", "θəsis")
        .replace("easure", "eːʒə")
        .replace("eisure", "ɜʒə")
        .replace("sure", "ʃʲɔ̈ː")
        .replace("mature", "mat͡ʃʲʊ̈ːə")
        .replace("ture", "t͡ʃə")
        .replace("ureau", "ʲʊ̈ːrɔʊ")
        .replace("ure", "ʲʊ̈ːə")
        .replace("eaux#", "ɔʊz̥#")
        .replace("eau#", "ɔʊ#")
        .replace("eau", "ʲʊ̈ː")
        .replace("uss", "ɜ̹ss")
        .replace("hurr", "hɜ̹ɹ")
        .replace("flurr", "flɜ̹ɹ")
        .replace("urr", "ɘ̹ːɹ")
        .replace("#uri", "#j|ʊ̈ːɹə")
        .replace("uri", "ə̟ɹi")
        .replace("urth", "eːð")
        .replace("ura", "ʲʊ̈ːra")
        .replace("ur", "ɘ̹ː")
        .replace("#chara", "#kʰæra")
        .replace("#cha", "#t͡ʃa")
        .replace("#chi", "#t͡ʃi")
        .replace("ceipt", "ceit")
        .replace("cept", "sept")
        .replace("#own", "#ɔʊn")
        .replace("own", "ɛ̞ɔ̽n")
        .replace("owl", "æ̃ɞ̠l")
        .replace("#əlow", "#əlæɔ̈")
        .replace("#kʰow", "#kʰæɔ̈")
        .replace("#how", "#hæɔ̈")
        .replace("#wow", "#wæɔ̈")
        .replace("ehow#", "ehæɔ̈#")
        .replace("yhow#", "yhæɔ̈#")
        .replace("#now", "#næɔ̈")
        .replace("#flower", "#flæɔ̈er")
        .replace("#power", "#pæɔ̈er")
        .replace("#shower", "#shæɔ̈er")
        .replace("#towe", "#tæɔ̈e")
        .replace("kn", "n")
        .replace("ow", "ɔʊ")
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
        .replace("who#", "hʊ̈ː#")
        .replace("whom#", "hʊ̈ːm#")
        .replace("whod#", "hʊ̈ːd#")
        .replace("who", "ho")
        .replace("wh", "w")
        .replace("aunt", "ɐ̠ːnt")
        .replace("daughter", "dɔ̈tə")
        .replace("augh", "æːf")
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
        .replace("wond", "wɜ̹nd")
        .replace("barb", "bɐ̠ːb")
        .replace("archi", "ɐ̠ːkʰə")
        .replace("arch", "ɐ̠ːk")
        .replace("arge", "ɐ̠ːʳd͡ʒe")
        .replace("#arg", "#ɐ̠ːʳg")
        .replace("arr", "æɹ")
        .replace("#æɹ", "#aɹ")
        .replace("#ar", "#əɹ")
        .replace("car#", "cɐ̠ː#")
        .replace("cars#", "cɐ̠ːʳz̥#")
        .replace("#liar", "#læ̈ːə")
        .replace("ara", "ɐ̠ːɹa")
        .replace("ar# #a", "ɐ̠ːʳ# #a")
        .replace("ar# #e", "ɐ̠ːʳ# #e")
        .replace("ar# #o", "ɐ̠ːʳ# #o")
        .replace("ari", "ɐ̠ːɹi")
        .replace("aro", "əro")
        .replace("war#", "wɔ̈#")
        .replace("iar#", "ʲə#")
        .replace("lar#", "lə̞#")
        .replace("ar#", "ɐ̠ː#")
        .replace("ata", "ɐ̠ːta")
        .replace("ato", "ɐ̠ːto")
        .replace("ust", "ɜ̹st")
        .replace("thought", "θought")
        .replace("outh", "æɔ̈θ")
        .replace("out", "æɔ̈t")
        .replace("obi", "ɔʊbi")
        .replace("ock", "ɔ̈ck")
        .replace("uke", "ʲʊ̈ːk")
        .replace("ude", "ʲʊ̈ːd")
        .replace("uga", "ʲʊ̈ːga")
        .replace("uge", "ʲʊ̈ːd͡ʒ")
        .replace("ee", "ᵊiː")
        .replace("ield", "ᵊiːld")
        .replace("aeces", "ᵊiːsᵊiːz̥")
        .replace("eces", "ᵊiːsᵊiːz̥")
        .replace("aeo", "iɔ̈")
        .replace("#ever", "#e^ver")
        .replace("e^very", "e^vry")
        .replace("#evid", "#e^vi^d")
        .replace("#evi^d", "#e^vi^d")
        .replace("#ev", "#iv")
        .replace("cially", "ʃl̥ᵊiː")
        .replace("enly", "ənl̥ᵊiː")
        .replace("ship", "ʃəp")
        .replace("ian", "^iˑən")
        .replace("dia", "dæ̈ːə")
        .replace("gian", "d͡ʒæ̈ːən")
        .replace("alian", "e̞ɪlᵊiˑən")
        .replace("migra", "mæ̈ːra")
        .replace("sia#", "ʒə#")
        .replace("ia", "iːə")
        .replace("ail", "e̞ɪɫ")
        .replace("ile", "æ̈ːɫe")
        .replace("dance", "dɛ̞ːnse")
        .replace("danci", "dɛ̞ːnsi")
        .replace("ance", "ənse")
        .replace("anci", "ənsi")
        .replace("ence", "ənse")
        .replace("ience", "iənse")
        .replace("astle", "æsᵊl")
        .replace("stle", "sᵊl")
        .replace("trast", "træst")
        .replace("and", "ɛ̞ː^nd")
        .replace("vel#", "vᵊɫ#")
        .replace("simi", "simᵊ")
        .replace("ain", "e̞ɪn")
        .replace("behav", "bihe̞ɪv")
        .replace("hav", "hæv")
        .replace("ave", "e̞ɪv")
        .replace("avi", "e̞ɪvi")
        .replace("ame", "æ̞̃ɪm")
        .replace("amin", "amən")
        .replace("ami", "æ̞̃ɪmi")
        .replace("ache", "e̞ɪk̥ʰ")
        .replace("ake", "e̞ɪk̥e")
        .replace("eaki", "e̞ɪk̥i")
        .replace("aki", "e̞ɪk̥i")
        .replace("ace", "e̞ɪs")
        .replace("aci", "e̞ɪsi")
        .replace("ade", "e̞ɪd̥")
        .replace("adi", "e̞ɪd̥i")
        .replace("ange", "e̞ɪnge")
        .replace("angi", "e̞ɪngi")
        .replace("ape", "e̞ɪpe")
        .replace("vase", "vɐ̠ːse")
        .replace("ase", "e̞ɪse")
        .replace("asi", "e̞ɪsi")
        .replace("maze", "mæ̃ëze")
        .replace("mazi", "mæ̃ëzi")
        .replace("lla", "le̞ɪ")
        .replace("nice", "næ̈ëse")
        .replace("prove", "prʊ̈ːve")
        .replace("provid", "prəvid")
        .replace("provi", "prʊ̈ːvi")
        .replace("ibe", "æ̈ːbe")
        .replace("ide", "æ̈ːd̥e")
        .replace("idi", "æ̈ːd̥i")
        .replace("æ̈ːd̥ea", "æ̈ːd̥ᵊiːa")
        .replace("grea", "gre̞ɪ")
        .replace("ɹ̥eak", "ɹ̥e̞ɪk")
        .replace("steak", "ste̞ɪk")
        .replace("ea", "ᵊiː")
        .replace("eon", "iːɔ̜n")
        .replace("water", "wɔ̈ðə")
        .replace("man", "mæn")
        .replace("shad", "ʃæd")
        .replace("ckage", "ckid͡ʒe")
        .replace("nage", "nəd͡ʒe")
        .replace("tage", "təd͡ʒe")
        .replace("ssage", "sid͡ʒe")
        .replace("uage", "wid͡ʒe")
        .replace("age", "e̞ɪd͡ʒe")
        .replace("magi", "mæd͡ʒi")
        .replace("agi", "e̞ɪd͡ʒi")
        .replace("ale", "e̞ɪle")
        .replace("#ico", "#æ̈ëko")
        .replace("ike", "æ̈ëke")
        .replace("iki", "æ̈ëki")
        .replace("ife", "æ̈ëf")
        .replace("ime", "æ̈ːme")
        .replace("machine", "məʃin")
        .replace("fine", "fæ̈ːne")
        .replace("line", "læ̈ːne")
        .replace("mine", "mæ̈ːne")
        .replace("nine", "næ̈ːne")
        .replace("shine", "shæ̈ːne")
        .replace("iny", "æ̈ːny")
        .replace("ini", "æ̈ːni")
        .replace("ison", "isᵊn")
        .replace("itely", "ʔl̥ᵊiː")
        .replace("cite", "sæ̈ëte")
        .replace("ite", "æ̈ëte")
        .replace("una", "ʲʊ̈ːna")
        .replace("nate", "nə^te")
        .replace("vate", "və^te")
        .replace("ate", "e̞ɪte")
        .replace("atic", "ætic")
        .replace("ati", "e̞ɪti")
        .replace("ɹiti", "ɹæ̈ëti")
        .replace("ita", "æ̈ëta")
        .replace("æ̈ëtc", "itec")
        .replace("oise", "o̝ɪz̥")
        .replace("oing", "o^ing")
        .replace("oi", "o̝ɪ")
        .replace("aise", "e̞ɪz̥e")
        .replace("ise", "æ̈ëse")
        .replace("risi", "ɹæ̈ësi")
        .replace("iso", "æ̈ësə")
        .replace("iza", "æ̈ësa")
        .replace("ize", "æ̈ëse")
        .replace("izi", "æ̈ësi")
        .replace("pri", "præ̈ː")
        .replace("rive", "ræ̈ːve")
        .replace("rivi", "ræ̈ːvi")
        .replace("ype", "æ̈ëpe")
        .replace("ypi", "æ̈ëpi")
        .replace("ybe", "ybᵊiː")
        .replace("aw", "ɔ̈ː")
        .replace("ay", "e̞ɪ^")
        .replace("they", "the̞ɪ^")
        .replace("rey#", "re̞ɪ^#")
        .replace("ɹ̥ey#", "ɹ̥e̞ɪ^#")
        .replace("ey#", "y#")
        .replace("ey", "e̞ɪ^")
        .replace("aste", "e̞ɪst")
        .replace("asthma", "æsmə")
        .replace("ast", "ɐ̠ːst")
        .replace("etable", "tbɫ")
        .replace("ortable", "tbɫ")
        .replace("shal", "ʃʲæl")
        .replace("cat#", "cæt#")
        .replace("cats#", "cæts#")
        .replace("flat", "flæt")
        .replace("lass", "læss")
        .replace("am", "æ̃ːm")
        .replace("sylla", "silə")
        .replace("berra", "brra")
        .replace("able", "e̞ɪbɫ")
        .replace("ce̞ɪ", "kʰe̞ɪ")
        .replace("te̞ɪ", "tʰe̞ɪ")
        .replace("imme", "ə̃mᵊiː")
        .replace("wash", "wɔ̈sh")
        .replace("was", "wɔ̈z̥")
        .replace("ash", "æsh")
        .replace("obe", "ɔʊbe")
        .replace("obli", "əblæ̈ː")
        .replace("prog", "prɔʊg")
        .replace("prob", "prɔ̈b")
        .replace("ob", "əb")
        .replace("oubl", "ɜ̹bl")
        .replace("ble#", "bɫ#")
        .replace("eigh", "e̞ɪ^")
        .replace("ight", "æ̈ë^t")
        .replace("igh", "æ̈ː^")
        .replace("die", "d̥æ̈ː^")
        .replace("lie", "læ̈ː^")
        .replace("tie", "tæ̈ː^")
        .replace("piece", "pᵊiːse")
        .replace("pie", "pæ̈ː^")
        .replace("iet", "æ̈ëət")
        .replace("bio", "bæ̈ëo")
        .replace("by", "bæ̈ː^")
        .replace("my", "mæ̈ː^")
        .replace("micro", "mæ̈ëkrɔʊ")
        .replace("ried#", "ryed#")
        .replace("cry", "kɹ̥æ̈ː^")
        .replace("dry", "d̠͡zɹ̥æ̈ː^")
        .replace("fly", "flæ̈ː^")
        .replace("shy", "ʃæ̈ː^")
        .replace("sky", "skæ̈ː^")
        .replace("#ign", "#i^gn")
        .replace("ign", "æ̈ːn")
        .replace("#ind", "#i^nd")
        .replace("ind", "æ̈ːnd")
        .replace("#i^nd", "#ind")
        .replace("etry", "etrᵊiː")
        .replace("ltry", "ltrᵊiː")
        .replace("ntry", "ntrᵊiː")
        .replace("stry", "strᵊiː")
        .replace("try", "træ̈ː^")
        .replace("why", "wæ̈ː^")
        .replace("tri", "t͡ʃɹi")
        .replace("str", "sṯɹ̥")
        .replace("iche", "iːʃ")
        .replace("uel", "ʲʊ̈ːᵊl")
        .replace("ue", "ʲʊ̈ː")
        .replace("oa", "ɔʊ")
        .replace("ɔʊl", "ɔ̈oɫ")
        .replace("ole", "ɔ̈oɫ")
        .replace("ool", "ɵ̠ːɫ")
        .replace("ull", "ʊ̈ɫ")
        .replace("oll", "ɔ̈ɫ")
        .replace("old", "ɔ̈od")
        .replace("ont#", "ɔʊnt#")
        .replace("ond", "ənd̥")
        .replace("go#", "gɔʊ#")
        .replace("go^", "gɔʊ^")
        .replace("goes#", "gɔʊz̥#")
        .replace("shoe", "shʊ̈ː")
        .replace("toe", "tɔʊ")
        .replace("oes", "ɐ̠s")
        .replace("cous", "kʰɜ̹s")
        .replace("touch", "tɜ̹t͡ʃ")
        .replace("oung", "ɜ̹ng")
        .replace("ouch", "æɔ̈t͡ʃ")
        .replace("oud", "æɔ̈d")
        .replace("oubt", "æɔ̈t")
        .replace("ought", "ɔ̈t")
        .replace("ogh", "ɔʊg")
        .replace("though", "thɔʊ")
        .replace("ough", "ɜ̹f")
        .replace("ug", "ɜ̹g")
        .replace("eno", "ino")
        .replace("mould", "mɔʊld")
        .replace("ould", "ʊ̈d")
        .replace("ountr", "ɜ̹ntr")
        .replace("oun", "ɛ̞ɔ̽n")
        .replace("ou", "ɵ̠→")
        .replace("ooe", "ɵ̠ː")
        .replace("lood", "lɐ̠d")
        .replace("oor#", "ɔ̈ː#")
        .replace("oor", "ɔ̈ːr")
        .replace("oo", "ɵ̠ː")
        .replace("ose", "ɔʊse")
        .replace("ocus", "ɔʊkəs")
        .replace("use", "ʲʊ̈ːz")
        .replace("#uʒ", "#ʲʊ̈ːʒ")
        .replace("#us", "#ʲʊ̈ːs")
        .replace("#busi", "#bɘsi")
        .replace("usi", "ʲʊ̈ːzi")
        .replace("actu", "ækʃʊ̈ː")
        .replace("act", "ækt")
        .replace("alf", "ɐ̠ːf")
        .replace("alk", "ɔ̈ːk")
        .replace("alm", "ɐ̞̃ːm")
        .replace("geo", "d͡ʒiɔ̈")
        .replace("gen#", "d͡ʒən#")
        .replace("gen", "d͡ʒen")
        .replace("gic", "d͡ʒic")
        .replace("eith", "æ̈ëð")
        .replace("ewe", "ʲʊ̈ːe")
        .replace("ew", "ʲʊ̈ː")
        .replace("une", "ʲʊ̈ːne")
        .replace("uni", "ʲʊ̈ːni")
        .replace("tʲʊ̈ːn", "t͡ʃʊ̈ːn")
        .replace("dʲʊ̈ːn", "d͡ʒʊ̈ːn")
        .replace("ume", "ʲʊ̈ːm")
        .replace("umi", "ʲʊ̈ːmi")
        .replace("assʲʊ̈ːm", "əʃʊ̈ːm")
        .replace("upi", "ʲʊ̈ːpi")
        .replace("syd", "sid")
        .replace("ute", "ʲʊ̈ːte")
        .replace("uti", "ʲʊ̈ːti")
        .replace("cove", "cɜ̹ve")
        .replace("kʰove", "kʰɜ̹ve")
        .replace("move", "mʊ̈ːv")
        .replace("movi", "mʊ̈ːvi")
        .replace("love", "lɔ̜ve")
        .replace("lovi", "lɔ̜vi")
        .replace("#kʰome", "#kʰɔ̜me")
        .replace("#kʰomi", "#kʰɔ̜mi")
        .replace("to# #dove", "to# #dɔʊv")
        .replace("dove", "dɔ̜v")
        .replace("ove", "ɔʊve")
        .replace("uck", "ɜ̹k")
        .replace("some", "sɜ̹m")
        .replace("ome", "ɔʊm")
        .replace("omo", "ɔʊmɔʊ")
        .replace("opef", "ɔʊpf")
        .replace("opel", "ɔʊpl")
        .replace("ope", "ɔʊpe")
        .replace("opi", "ɔʊpi")
        .replace("ode", "ɔʊd̥e")
        .replace("ote", "ɔʊt")
        .replace("oti", "ɔʊti")
        .replace("oto", "ɔʊto")
        .replace("ony", "ɔʊny")
        .replace("onic", "ɔ̈nɘkʰ")
        .replace("uild", "i^ld")
        .replace("ildren", "il̥^d̠͡zɹ̥ən")
        .replace("dr", "d̠͡zɹ̥")
        .replace("ult", "ɐ̠ɫt^")
        .replace("ild", "æ̈ːld")
        .replace("uilt", "ilt")
        .replace("nuin", "nʲʊ̈ːin")
        .replace("uin", "ʊ̈ːin")
        .replace("cuit", "kʰit")
        .replace("guit", "git")
        .replace("uit", "ʊ̈ːt")
        .replace("#to", "#tʊ̈^")
        .replace("off", "ɔ̈f")
        .replace("of", "ɔ̈v")
        .replace("uff", "ɜ̹f")
        .replace("uy", "æ̈ː")
        .replace("sson", "sᵊn")
        .replace("gone", "gɒ̜̽n")
        .replace("pon", "pɒ̜̽n")
        .replace("phon", "fɒ̜̽n")
        .replace("bone", "bɔʊne")
        .replace("lone", "lɔʊne")
        .replace("tone", "tɔʊne")
        .replace("ton", "tɔ̃n")
        .replace("zon", "zɔ̃n")
        .replace("once", "wɜ̹̃ns")
        .replace("one", "wɜ̹̃n")
        .replace("#only", "#ɔʊnly")
        .replace("#on", "#ɔ̈n")
        .replace("engl", "ingl")
        .replace("ng#", "n#")
        .replace("finge", "fing^e")
        .replace("linge", "ling^e")
        .replace("ringe", "ring^e")
        .replace("singe", "sing^e")
        .replace("onge", "ong^e")
        .replace("ng", "ŋg")
        .replace("ŋgiŋ", "ŋd͡ʒiŋ")
        .replace("aŋgel", "e̞ɪŋd͡ʒel")
        .replace("ŋge", "ŋd͡ʒe")
        .replace("igi", "id͡ʒi")
        .replace("aŋ", "ɶ̜̃ŋ")
        .replace("i^iŋ", "iŋ")
        .replace("oŋ", "ɔ̃ŋ")
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
        .replace("un", "ɜ̹̃n")
        .replace("uŋ", "ɜ̹̃ŋ")
        .replace("up", "ɜ̹p")
        .replace("ub", "ɜ̹b")
        .replace("ute", "ʲʊ̈ːte")
        .replace("uto", "ʲʊ̈ːto")
        .replace("put", "pʊ̈t")
        .replace("uth", "ʉθ")
        .replace("month", "mɜ̹̃nθ")
        .replace("bath#", "bɐ̠ːθ#")
        .replace("path", "pæθ")
        .replace("bab#", "bæb#")
        .replace("uct", "ɜ̹ct")
        .replace("ut", "ɜ̹t")
        .replace("ud", "ɜ̹d")
        .replace("but#", "bət#")
        .replace("ak", "æk")
        .replace("ack", "æk")
        .replace("ai", "e̞ɪ^")
        .replace("ik", "ikʰ")
        .replace("this", "ðis^")
        .replace("is#", "iz̥#")
        .replace("ẽ̞n̪th", "ẽn̪θ")
        .replace("othes", "ɔʊðz")
        .replace("oth", "ɐ̠ð")
        .replace("ythm", "iðᵊm")
        .replace("yme", "æ̈ːme")
        .replace("gh", "g")
        .replace("than", "ðə̃n̪")
        .replace("thẽn̪", "ðẽn̪")
        .replace("these", "ðiːz")
        .replace("the#", "ðə#")
        .replace("thi", "θi")
        .replace("th", "ð")
        .replace("two", "tʰʊ̈ː")
        .replace("pt", "p̚t")
        .replace("spu", "spʲʊ̈ː")
        .replace("pu", "pʰu")
        .replace("pʊ̈", "pʰʊ̈")
        .replace("tʊ̈", "tʰʊ̈")
        .replace("ief", "ᵊiːf")
        .replace("dge", "d͡ʒe")
        .replace("rant", "^rənt")
        .replace("want", "wɒ̜̽nt")
        .replace("#ant", "#ɛ̞ː^nt")
        .replace("can", "kʰɛ̞ːn")
        .replace("ken", "kʰẽn")
        .replace("ank", "ɛ̞ːnk")
        .replace("an#", "ɛ̞ːn#")
        .replace("#ɛ̞ːn", "#an")
        .replace("ish", "ɘʃ")
        .replace("lush", "lɜ̹ʃ")
        .replace("rush", "rɜ̹ʃ")
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
        .replace("#hi", "#hæ̈ː^")
        .replace("sto", "stɔ̈")
        .replace("nf", "ɱf")
        .replace("from", "frɔ̃m")
        .replace("oɱ", "ə̃ɱ")
        .replace("om", "ə̃m")
        .replace("bad", "bæd̥")
        .replace("hab", "hæb")
        .replace("had", "hæd")
        .replace("gla", "ɡlæ")
        .replace("gra", "ɡɹ̥æ")
        .replace("suk", "sək")
        .replace("app", "æp")
        .replace("ap", "æp")
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
        .replace("elæ̈ːness#", "linəz̥#")
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
        .replace("yes#", "yez̥#")
        .replace("es#", "z̥#")
        .replace("əs#", "əz̥#")
        .replace("sas", "zəz̥")
        .replace("as#", "æz̥#")
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
        .replace("god", "gɒ̜̽d")
        .replace("dop", "dɔ̈p")
        .replace("#be#", "#bᵊiː#")
        .replace("#bet", "#b^et")
        .replace("#bev", "#b^ev")
        .replace("#be", "#bi")
        .replace("#me#", "#mi^#")
        .replace("t͡ʃa", "t͡ʃæ")
        .replace("nal", "nᵊl")
        .replace("ral", "ɹᵊl")
        .replace("#re", "#ɹi")
        .replace("uism#", "əizəm#")
        .replace("ation", "e̞ɪʃŋ")
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
        .replace("#ly", "#æ̈ː^")
        .replace("ely", "l̥ᵊiː")
        .replace("ily", "l̥ᵊiː")
        .replace("ly", "l̥ᵊiː")
        .replace("fy", "fæ̈ː")
        .replace("el#", "əɫ#")
        .replace("els#", "əɫz̥#")
        .replace("ll", "l")
        .replace("ls#", "ɫz̥#")
        .replace("ical", "ikᵊl")
        .replace("ic#", "əkʰ#")
        .replace("ce#", "s#")
        .replace("#ive#", "#æ̈ːv#")
        .replace("five", "fæ̈ːv")
        .replace("live", "læ̈ːv")
        .replace("vive", "væ̈ːv")
        .replace("e#", "#")
        .replace("#do#", "#dʊ̈#")
        .replace("#do^", "#dʊ̈^")
        .replace("#to#", "#tə#")
        .replace("o#", "ɔʊ#")
        .replace("os#", "ɔʊz̥#")
        .replace("θor", "θə")
        .replace("#or#", "#ɔ̈ː#")
        .replace("or#", "ə#")
        .replace("or", "ɔ̈ː")
        .replace("to", "tə")
        .replace("cy#", "sᵊiː#")
        .replace("logy", "ləd͡ʒᵊiː")
        .replace("uoy", "oy")
        .replace("oy", "o̞ɪ")
        .replace("o̞ɪs", "o̞ɪz̥")
        .replace("y#", "ᵊiː#")
        .replace("yr", "ᵊiːr")
        .replace("yw", "ᵊiːw")
        .replace("e-", "iː")
        .replace("#eco", "#iːkɔʊ")
        .replace("ci", "si")
        .replace("caf", "cæf")
        .replace("eful", "fʊ̈ɫ")
        .replace("ful", "fʊ̈ɫ")
        .replace("diff", "dif")
        .replace("uard", "ɐ̠ːd")
        .replace("ard", "ɐ̠ːd")
        .replace("fé", "fæi")
        .replace("mach", "məkʰ")
        .replace("nnel", "nnᵊl")
        .replace("watch", "wɔ̈t͡ʃ")
        .replace("atch", "æt͡ʃ")
        .replace("tch", "t͡ʃ")
        .replace("ch#", "t͡ʃ#")
        .replace("chæ", "t͡ʃæ")
        .replace("chʊ̈", "t͡ʃʊ̈")
        .replace("chec", "t͡ʃec")
        .replace("cho", "t͡ʃo")
        .replace("ch", "k")
        .replace("cc", "k")
        .replace("ck#", "kʰ#")
        .replace("ck", "k")
        .replace('c', "k")
        .replace("kæ", "kʰæ")
        .replace("kl", "kl̥")
        .replace("fr", "fɹ̥")
        .replace("gr", "ɡɹ̥")
        .replace("tr", "ṯɹ̥")
        .replace("um", "ɐ̃m")
        .replace("pʰut", "pʰʊ̈t")
        .replace("ʊ̈k", "ʊ̈kʰ")
        .replace("æk", "ækʰ")
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
        .replace('s', "s̺")
        .replace("s̺̬", "s̬")
        .replace("i#", "æ̈ː#")
        .replace("ph", "f")
        .replace("ff", "f")
        .replace("pp", "p")
        .replace("pf", "f")
        .replace("oh#", "ɔʊ#")
        .replace("h#", "#")
        .replace("mm", "m")
        .replace("nb", "mb")
        .replace("mb#", "m#")
        .replace("æn", "ɛ̞ːn")
        .replace("ɶ̜̃n", "ɛ̞ːn")
        .replace("umn", "ᵊm")
        .replace("ymn", "im")
        .replace("mn#", "m#")
        .replace("nn", "n")
        .replace("s̺nt", "zᵊnt")
        .replace('j', "d͡ʒ")
        .replace("d͡ʒ|", "j")
        .replace("#ʲʊ̈ː", "#jʊ̈ː")
        .replace("#ʲʉ̟̃ː", "#jʉ̟̃ː")
        .replace("at# #an", "aɹ# #an")
        .replace('y', "j")
        .replace(['a', 'o'], "ə")
        .replace("ɔ̈ə", "ɔ̈o")
        .replace("ə̞ɪ", "o̞ɪ")
        .replace("ə̝ː", "o̝ː")
        .replace('e', "ə̟")
        .replace("ə̟̞ɪ", "e̞ɪ")
        .replace("ə̟̈", "ë")
        .replace("ft", "ft̪")
        .replace("ət", "ət̪")
        .replace("ət̪#.#", "æt̪#.#")
        .replace("ət̪#,#", "æt̪#,#")
        .replace("t#", "t̚#")
        .replace("t̚# #j", "t̠# #j")
        .replace("tt", "ð")
        .replace("l#", "ɫ#")
        .replace("u̟ːɫ#", "u̟ːɫ̚#")
        .replace("ʊ̈ɫ#", "ʊ̈ɫ̚#")
        .replace("oɫ#", "oɫ̚#")
        .replace("pr", "pɹ̥")
        .replace("s̺pɹ̥", "s̺pɹ")
        .replace("rr", "ɹ")
        .replace('r', "ɹ")
        .replace("e̽ɹ", "e̽ɹ̥")
        .replace("e̽t̚# #æ̞", "e̽ɹ̪# #æ̞")
        .replace("nt̚#", "nt̥^#")
        .replace("ə̟t̚#", "ə̟t̚^#")
        .replace("t̚#", "#")
        .replace("nk", "ŋk")
        .replace("v#", "ʋ#")
        .replace("ɑ̝e̽ɫ", "ɑ̝e̽ᵊɫ")
        .replace("iːɫ", "iːᵊɫ")
        .replace("d# #j", "d̠͡z# #j")
        .replace("ʊ̈ːld", "ʊ̈ːd̥")
        .replace("ld", "l̥d")
        .replace("lw", "l̥w")
        .replace("lm", "ɫm")
        .replace("d# #ð", "d̪̚# #ð")
        .replace("nd#", "nd̚^#")
        .replace("ɵ̠ːd#", "ɵ̠ːd̚#")
        .replace("ːd#", "ː#")
        .replace("d#", "d̥#")
        .replace("d̥d̥#", "dəd̥#")
        .replace("nd̥# #ɑ̝", "n# #ɑ̝")
        .replace("ðə# #ə", "ðiː# #ə")
        .replace("ðə# #ɐ̠", "ðiː# #ɐ̠")
        .replace("ðə# #e̽", "ðiː# #e̽")
        .replace("ðə# #j", "ðiː# #j")
        .replace("e̽ː# #i", "e̽ːʳ# #i")
        .replace("is̺", "^ɪ̠s̺")
        .replace("is̬", "^ɪ̠s̬")
        .replace("iz̥", "^ɪ̠z̥")
        .replace("iɫ", "ɪɫ")
        .replace("ë^e̽z̥#", "ëz̥#")
        .replace("in", "en")
        .replace("im", "em")
        .replace("iŋ", "eŋ")
        .replace("#i", "#ᵊiː")
        .replace("n# #m", "m# #m")
        .replace("n# #n", "# #n")
        .replace("iː̃", "ĩː")
        .replace("i", "i̠")
        .replace("ʰʰ", "ʰ")
        .replace("o̝ːs̺e̽", "o̝ːs̺")
        .replace("ɵ̠→", "ö")
        .replace("gg", "g")
        .replace('g', "ɡ")
        .replace('f', "f̟")
        .replace('^', "");

    let result = engpncend(strmod);

    if usefile == "new" {
        let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
        file.write_all("DALLAS, TX, US: [IN ALPHA]".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "old" {
        let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
        file.write_all("DALLAS, TX, US: [IN ALPHA]".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("{}", cyan.to_owned() + "Dallas" + reset + ", " + cyan + "TX" + reset + ", " + cyan + "US" + reset + ": " + red + "[in alpha]" + reset);
        println!();
        print!("{yellow}");
        println!("{result}");
        print!("{reset}");
    }
}
