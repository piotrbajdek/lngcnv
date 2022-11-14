// LNGCNV VERSION 1.6.1 / MIT LICENSE © 2022 PIOTR BAJDEK

// MODULE MODENG

// CLIPPY LINTS

#![deny(clippy::no_effect_replace)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::too_many_lines, clippy::unicode_not_nfc)]
#![allow(clippy::string_lit_as_bytes)] // must be as_bytes() because non-ASCII characters are included

// IMPORTS

use std::fs::OpenOptions;
use std::io::Write;

// SIMPLIFY INTERPUNCTION

fn engpncbeg(lowercase: &str) -> String {
    let pncbeg = &lowercase.replace([';', '!', '?'], ".").replace(").", ".").replace("),", ",").replace([')', ':'], ",").replace("--", " – ").replace('(', "∣ .");
    pncbeg.to_string()
}

// REMOVE INTERPUNCTION

fn engpncend(strmod: &str) -> String {
    let result = &strmod.replace(',', " ∣").replace(". ", " ∥ ").replace('.', "").replace(" - ", " ∣ ").replace(" – ", " ∣ ").replace("∣ ∣", "∣").replace("∣ ∥", "∥");
    result.to_string()
}

// ENGLISH: IPA

pub fn engaucanberra(original_text: &str, usefile: &str, outputfile: &str, reset: &str, red: &str, cyan: &str, yellow: &str) {
    let dotend = original_text.to_owned() + "."; // mark word ending
    let dotbeg = ".".to_owned() + &dotend; // word beginning
    let lowercase = &dotbeg.to_lowercase();
    let pncbeg = engpncbeg(lowercase);

    let strmod = &pncbeg
        .replace("i'd", "ɑ̝ëd")
        .replace("i’d", "ɑ̝ëd")
        .replace("i'll", "ɑ̝ël")
        .replace("i’ll", "ɑ̝ël")
        .replace("i'm", "ɑ̝ëm")
        .replace("i’m", "ɑ̝ëm")
        .replace(['\'', '’'], "")
        .replace("etc.", "etˈsæ̠t͡ʃɹə.")
        .replace("eah", "æ̠ə")
        .replace("ballet", "bɐ̠lei^")
        .replace("caramel", "ˈkʰɶ̜ɹməɫ")
        .replace("christmas", "krismas")
        .replace("debris", "dibɹiː")
        .replace("entrepreneur", "ˌɔ̈nt̠ɹ̥əpɹəˈnɜː")
        .replace("garage", "ˈɡɐ̠ɹəd͡ʒ")
        .replace("listen", "lisᵊn")
        .replace("often", "ɔ̈fᵊn")
        .replace("perhaps", "peːhɶ̜ps")
        .replace("video", "vidɞ̜ʉ̞")
        .replace("comm", "kʰɔ̈mm")
        .replace("ionaire", "io̝ːnə")
        .replace("alumi", "ɶ̜lʲʉ̟ːmi")
        .replace("nium", "nʲəm")
        .replace("subtl", "sɐ̠tl")
        .replace("debt", "det")
        .replace("wedn", "wɛn")
        .replace("yed", "yd̥")
        .replace("bb", "b")
        .replace("cce", "kse")
        .replace("cci", "ksi")
        .replace("musc", "mɐ̠sc")
        .replace("psycho", "sɑ̝ëko")
        .replace("scr", "skr")
        .replace("sc", "s")
        .replace("cen", "sen")
        .replace("dd", "d̥")
        .replace("choir", "kwɑ̝ëə")
        .replace("ghost", "gɞ̜ʉ̞st")
        .replace("gue", "ge")
        .replace("gui", "gi")
        .replace("kn", "n")
        .replace("que ", "k ")
        .replace("que.", "k.")
        .replace("que,", "k,")
        .replace("qu", "kw")
        .replace("rh", "ɹ")
        .replace("ttem", "tʰem")
        .replace("tten", "tʰen")
        .replace("wr", "ɹ")
        .replace("wha", "wɔ̈")
        .replace("dess", "diss")
        .replace("pad", "pɶ̜d")
        .replace("ve read", "ve ɹeːd")
        .replace("has read", "has ɹeːd")
        .replace("nt read", "nt ɹeːd")
        .replace("ered", "əd̥")
        .replace("bare", "beː")
        .replace("bari", "beːɹi")
        .replace("care", "kʰeː")
        .replace("cari", "kʰeːɹi")
        .replace("dare", "deː")
        .replace("dari", "deːɹi")
        .replace("mare", "meː")
        .replace("mari", "meːɹi")
        .replace("share", "ʃeː")
        .replace("shari", "ʃeːɹi")
        .replace("tare", "teː")
        .replace("tari", "teːɹi")
        .replace("ware", "weː")
        .replace("esta", "ista")
        .replace("est ", "ist ")
        .replace("est.", "ist.")
        .replace("est,", "ist,")
        .replace("bist", "best")
        .replace("rist", "rest")
        .replace("uist", "est")
        .replace("wist", "west")
        .replace("answ", "ɶ̜ns")
        .replace("ans", "ɶ̜ns")
        .replace("sword", "sɞːd̥")
        .replace("ord", "ɘ̟ːd")
        .replace("orld", "ɘ̟ːld")
        .replace("ork", "ɘ̟ːk")
        .replace("orth", "o̝ːθ")
        .replace("ore̽st", "o̝ɹest")
        .replace("ore a", "o̝ːɹ a")
        .replace("ore e", "o̝ːɹ e")
        .replace("ore o", "o̝ːɹ o")
        .replace("ore ɔ̈", "o̝ːɹ ɔ̈")
        .replace("ore", "o̝ː")
        .replace("ory", "ɔ̈ːry")
        .replace(" ori", " əɹi")
        .replace(".ori", ".əɹi")
        .replace("air", "eː")
        .replace("there", "ðeː")
        .replace("were", "weː")
        .replace("circ", "sɘ̟ːc")
        .replace("ira", "ɑ̝ëra")
        .replace("iro", "ɑ̝ëro")
        .replace("ire", "ɑ̝ëə")
        .replace("irr", "iɹ")
        .replace("ir", "ɘ̟ː")
        .replace("isl", "ɑ̝ël")
        .replace("ience", "iənse")
        .replace("rious", "ɹiəs")
        .replace("cious", "ʃiəs")
        .replace("tious", "ʃiəs")
        .replace("amous", "ɶ̃ëmous")
        .replace("ous ", "əs ")
        .replace("ous,", "əs,")
        .replace("ous.", "əs.")
        .replace("ous", "æ̞ɞs")
        .replace("bour", "bə")
        .replace("iour", "j|ə")
        .replace("nour", "nə")
        .replace("olour", "ɔ̈lə")
        .replace("youre", "j|ʉ̟ːə")
        .replace("your", "j|ʉ̟ː")
        .replace("ource", "o̝ːse")
        .replace("ourse", "o̝ːse")
        .replace("four", "fo̝ː")
        .replace("fut", "fʲʉ̟ːt")
        .replace("our", "æ̞ɞə")
        .replace("easure", "eːʒə")
        .replace("eisure", "ɜʒə")
        .replace("sure", "ʃʲɔ̈ː")
        .replace("mature", "mat͡ʃʲʉ̟ːə")
        .replace("ture", "t͡ʃə")
        .replace("ure", "ʲʉ̟ːə")
        .replace("urr", "ɐ̠ɹ")
        .replace(" uri", "j|ʉ̟ːɹə")
        .replace(".uri", "j|ʉ̟ːɹə")
        .replace("uri", "ɘ̟ɹi")
        .replace("ur", "ɘ̟ː")
        .replace("ounce", "æɞ̃nce")
        .replace("ounci", "æɞ̃nsi")
        .replace("ound", "æɞ̃nd̥")
        .replace("ceipt", "ceit")
        .replace("own", "æ̃ɞn")
        .replace("ow", "æ̞ɞ")
        .replace("wea", "weˑ")
        .replace("eady", "edy")
        .replace("eas", "ᵊiːz̥")
        .replace("eason", "ᵊiːz̥ᵊn")
        .replace("ear ", "iə ")
        .replace("eart", "ɐ̠ːt")
        .replace("ear", "eː")
        .replace("eali", "ᵊiːəli")
        .replace("who ", "hʉ̟ː ")
        .replace("who", "ho")
        .replace("wh", "w")
        .replace("aunt", "ɐ̠ːnt")
        .replace("daughter", "dɔ̈tə")
        .replace("au", "ɔ̈")
        .replace("any", "æ̃ni")
        .replace("arm", "õ̝ːm")
        .replace("arn", "õ̝ːn")
        .replace("harr", "hɶ̜ɹ")
        .replace("arr", "əɹ")
        .replace("are a", "ɐ̠ːɹ a")
        .replace("are e", "ɐ̠ːɹ e")
        .replace("are o", "ɐ̠ːɹ o")
        .replace("are ", "ɐ̠ː ")
        .replace("eyre", "eː")
        .replace("re ", "ə ")
        .replace("wond", "wɐ̠nd")
        .replace("archi", "ɐ̠ːkʰə")
        .replace("arch", "ɐ̠ːk")
        .replace(" ar", " əɹ")
        .replace(".ar", ".əɹ")
        .replace("liar", "lɑ̝ëə")
        .replace("ara", "ɐ̠ːɹa")
        .replace("ar a", "ɐ̠ːɹ a")
        .replace("ar e", "ɐ̠ːɹ e")
        .replace("ar o", "ɐ̠ːɹ o")
        .replace("ari", "ɐ̠ːɹi")
        .replace("ar", "ɐ̠ː")
        .replace("ust", "ɐ̠st")
        .replace("tri", "t͡ʃɹi")
        .replace("str", "sṯɹ̥")
        .replace("spr", "spɹ")
        .replace("pr", "pɹ̥")
        .replace("thought", "θought")
        .replace("outh", "æ̞ɞθ")
        .replace("out", "æ̞ɞt")
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
        .replace(" ev", " iv")
        .replace(".ev", ".iv")
        .replace("cially", "ʃl̥ᵊiː")
        .replace("dia", "dɑ̝ëə")
        .replace("gian", "d͡ʒɑ̝ëən")
        .replace("alian", "æ̠ëlᵊiːən")
        .replace("ia", "iːə")
        .replace("ile", "ɑ̝ëɫ")
        .replace("ance", "æ̃nse")
        .replace("anci", "æ̃nsi")
        .replace("astle", "ɶ̜sᵊl")
        .replace("stle", "sᵊl")
        .replace("trast", "trɶ̜st")
        .replace("and", "æ̃nd")
        .replace("vel ", "vᵊɫ ")
        .replace("vel.", "vᵊɫ.")
        .replace("vel,", "vᵊɫ,")
        .replace("ain", "æ̠ën")
        .replace("behav", "bihæ̠ëv")
        .replace("hav", "hɶ̜v")
        .replace("ave", "æ̠ëv")
        .replace("avi", "æ̠ëvi")
        .replace("ame", "æ̞̃ëm")
        .replace("ami", "æ̞̃ëmi")
        .replace("ache", "æ̠ëk̥ʰ")
        .replace("ake", "æ̠ëk̥e")
        .replace("aki", "æ̠ëk̥i")
        .replace("ace", "æ̠ës")
        .replace("aci", "æ̠ësi")
        .replace("ade", "æ̠ëd̥")
        .replace("adi", "æ̠ëd̥i")
        .replace("ase", "æ̠ëse")
        .replace("asi", "æ̠ësi")
        .replace("maze", "mæ̃ëze")
        .replace("mazi", "mæ̃ëzi")
        .replace("nice", "nɑ̝ëse")
        .replace("pɹ̥ove", "pɹ̥ʉ̟ːve")
        .replace("pɹ̥ovid", "pɹ̥əvid")
        .replace("pɹ̥ovi", "pɹ̥ʉ̟ːvi")
        .replace("ibe", "ɑ̝ëbe")
        .replace("ide", "ɑ̝ëd̥e")
        .replace("idi", "ɑ̝ëd̥i")
        .replace("ɑ̝ëd̥ea", "ɑ̝ëd̥ᵊiːa")
        .replace("ea", "ᵊiː")
        .replace("water", "wɔ̈ðə")
        .replace("man", "mɶ̜n")
        .replace("ckage", "ckid͡ʒe")
        .replace("nage", "nid͡ʒe")
        .replace("ssage", "sid͡ʒe")
        .replace("uage", "wid͡ʒe")
        .replace("age", "æ̠ëd͡ʒe")
        .replace("magi", "mɶ̜d͡ʒi")
        .replace("agi", "æ̠ëd͡ʒi")
        .replace("ale", "æ̠ële")
        .replace("ike", "ɑ̝ëke")
        .replace("iki", "ɑ̝ëki")
        .replace("ife", "ɑ̝ëf")
        .replace("ime", "ɑ̝ëme")
        .replace("machine", "məʃin")
        .replace("line", "lɑ̝ën")
        .replace("mine", "mɑ̝ën")
        .replace("iny", "ɑ̝ëny")
        .replace("ini", "ɑ̝ëni")
        .replace("ison", "isᵊn")
        .replace("itely", "ʔl̥ᵊiː")
        .replace("cite", "sɑ̝ëte")
        .replace("ite", "ɑ̝ëte")
        .replace("ate", "æ̠ëte")
        .replace("iti", "ɑ̝ëti")
        .replace("ita", "ɑ̝ëta")
        .replace("ɑ̝ëtc", "itec")
        .replace("isa", "ɑ̝ësa")
        .replace("ise", "ɑ̝ëse")
        .replace("isi", "ɑ̝ësi")
        .replace("iso", "ɑ̝ësə")
        .replace("iza", "ɑ̝ësa")
        .replace("ize", "ɑ̝ëse")
        .replace("izi", "ɑ̝ësi")
        .replace("pɹ̥i", "pɹ̥ɑ̝ë")
        .replace("rive", "rɑ̝ëve")
        .replace("rivi", "rɑ̝ëvi")
        .replace("ype", "ɑ̝ëpe")
        .replace("ypi", "ɑ̝ëpi")
        .replace("ay", "æ̠ë^")
        .replace("they", "thæ̠ë^")
        .replace("ey ", "y ")
        .replace("ey,", "y,")
        .replace("ey.", "y.")
        .replace("ey", "æ̠ë^")
        .replace("aw", "ɔ̈")
        .replace("aste", "æ̠ëst")
        .replace("asthma", "ɶ̜smə")
        .replace("ast", "ɐ̠ːst")
        .replace("etable", "tbɫ")
        .replace("ortable", "tbɫ")
        .replace("shal", "ʃʲɶ̜l")
        .replace("flat", "flɶ̜t")
        .replace("gla", "ɡlɶ̜")
        .replace("gra", "ɡɹ̥ɶ̜")
        .replace("lass", "lɶ̜ss")
        .replace("am", "ɶ̜m")
        .replace("sylla", "silə")
        .replace("berra", "brra")
        .replace("able", "æ̠ëbɫ")
        .replace("cæ̠ë", "kʰæ̠ë")
        .replace("tæ̠ë", "tʰæ̠ë")
        .replace("imme", "ə̃mᵊiː")
        .replace("was", "wɔ̈z̥")
        .replace("obli", "əblɑ̝ë")
        .replace("ob", "əb")
        .replace("oubl", "ɐ̠bl")
        .replace("ble ", "bɫ ")
        .replace("ble.", "bɫ.")
        .replace("ble,", "bɫ,")
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
        .replace("cry", "kɹ̥ɑ̝ë^")
        .replace("dry", "dɹ̥ɑ̝ë^")
        .replace("fly", "flɑ̝ë^")
        .replace("shy", "ʃɑ̝ë^")
        .replace("sky", "skɑ̝ë^")
        .replace("try", "trɑ̝ë^")
        .replace("why", "wɑ̝ë^")
        .replace("iche", "iːʃ")
        .replace("uel", "ʲʉ̟ːᵊl")
        .replace("oa", "ɞ̜ʉ̞")
        .replace("cɞ̜ʉ̞", "kʰɞ̜ʉ̞")
        .replace("ɞ̜ʉ̞l", "ɔ̈o̝ːɫ")
        .replace("ole", "ɔ̈o̝ːɫ")
        .replace("ool", "uːɫ")
        .replace("oll", "ɔ̈ɫ")
        .replace("old", "ɔ̈o̝ːd")
        .replace("dont", "dɞ̜ʉ̞nt̚ ")
        .replace("ond", "ənd̥")
        .replace("oes", "ɐ̠s")
        .replace("cous", "kʰɐ̠s")
        .replace("touch", "tɐ̠t͡ʃ")
        .replace("oung", "ɐ̠ng")
        .replace("ouch", "æ̞ɞt͡ʃ")
        .replace("oud", "æ̞ɞd")
        .replace("oubt", "æ̞ɞt")
        .replace("ought", "ɔ̈t")
        .replace("eno", "ino")
        .replace("though", "thɞ̜ʉ̞")
        .replace("ough", "ɐ̠f")
        .replace("ug", "ɐ̠g")
        .replace("could", "kʰʊ̟d")
        .replace("ould", "ʊ̟d")
        .replace("ouse", "ɑ̝ëse")
        .replace("ou", "ʉ̟ː")
        .replace("ost", "æ̞ɞst")
        .replace("ooe", "ʉ̟ː")
        .replace("lood", "lɐ̠d")
        .replace("oo", "ʊ̟")
        .replace("ose", "ɞ̜ʉ̞se")
        .replace("ocus", "ɞ̜ʉ̞kəs")
        .replace("use", "ʲʉ̟ːz")
        .replace("usi", "ʲʉ̟ːzi")
        .replace("actu", "ɶ̜kʃʉ̟ː")
        .replace("act", "ɶ̜kt")
        .replace("alf", "ɐ̠ːf")
        .replace("alk", "ɔ̈ːk")
        .replace("alm", "ɐ̞̃ːm")
        .replace("geo", "d͡ʒiɔ̈")
        .replace("gen", "d͡ʒen")
        .replace("gic", "d͡ʒic")
        .replace("eith", "iːð")
        .replace("urth", "eːð")
        .replace("ura", "ʲʉ̟ːra")
        .replace("ewe", "ʲʉ̟ːe")
        .replace("ew", "ʲʉ̟ː")
        .replace("une", "ʲʉ̟̃ːn")
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
        .replace("move", "mʉ̟ːv")
        .replace("movi", "mʉ̟ːvi")
        .replace("love", "lɔ̜ve")
        .replace("lovi", "lɔ̜vi")
        .replace("to dove", "to dɞ̜ʉ̞v")
        .replace("dove", "dɔ̜v")
        .replace("ove", "ɞ̜ʉ̞ve")
        .replace("uck", "ɐ̠k")
        .replace("some", "sɐ̠m")
        .replace("come", "kʰɐ̠m")
        .replace("ome", "ɞ̜ʉ̞m")
        .replace("opef", "ɞ̜ʉ̞pf")
        .replace("opel", "ɞ̜ʉ̞pl")
        .replace("ope", "ɞ̜ʉ̞pe")
        .replace("opi", "ɞ̜ʉ̞pi")
        .replace("ode", "ɞ̜ʉ̞d̥e")
        .replace("ote", "ɞ̜ʉ̞t")
        .replace("oti", "ɞ̜ʉ̞ti")
        .replace("oto", "ɞ̜ʉ̞to")
        .replace("ony", "ɞ̜ʉ̞ny")
        .replace("uild", "ild")
        .replace("uilt", "ilt")
        .replace("uin", "uːin")
        .replace("cuit", "kʰit")
        .replace("guit", "git")
        .replace("uit", "uːt")
        .replace("off", "ɔ̈f")
        .replace("of", "ɔ̈v")
        .replace("uff", "ɐ̠f")
        .replace("uy", "ɑ̝ë")
        .replace("sson", "sᵊn")
        .replace("con", "kə̃n")
        .replace("gone", "gɔ̝̃n")
        .replace("pon", "pɔ̝̃n")
        .replace("phon", "fɔ̝̃n")
        .replace("bone", "bɞ̜ʉ̞n")
        .replace("tone", "tɞ̜ʉ̞n")
        .replace("ton", "tɔ̝̃n")
        .replace("zon", "zɔ̝̃n")
        .replace("once", "wɐ̃ns")
        .replace("one", "wɐ̃n")
        .replace("engl", "ingl")
        .replace("ng ", "ŋ ")
        .replace("ng.", "ŋ.")
        .replace("ng,", "ŋ,")
        .replace("ng", "ŋg")
        .replace("aŋ", "ɶ̜̃ŋ")
        .replace("tiŋ", "ɾĩŋ")
        .replace("i^iŋ", "iŋ")
        .replace("iŋ", "ĩŋ")
        .replace("oŋ", "ɔ̝̃ŋ")
        .replace("ent ", "ənt ")
        .replace("ent.", "ənt.")
        .replace("ent,", "ənt,")
        .replace("ene", "iːn")
        .replace("en", "en̪")
        .replace("in", "ĩn")
        .replace("im", "ĩm")
        .replace("un", "ɐ̃n")
        .replace("up", "ɐ̠p")
        .replace("ub", "ɐ̠b")
        .replace("ute", "ʲʉ̟ːte")
        .replace("uto", "ʲʉ̟ːto")
        .replace("put", "pʊ̟t")
        .replace("ut", "ɐ̠t")
        .replace("ud", "ɐ̠d")
        .replace("but ", "bət ")
        .replace("but,", "bət,")
        .replace("ak", "ɶ̜k")
        .replace("ack", "ɶ̜k")
        .replace("ai", "æ̠ë^")
        .replace("ik", "ikʰ")
        .replace("this", "ðis^")
        .replace("is ", "iz̥ ")
        .replace("is.", "iz̥.")
        .replace("is,", "iz̥,")
        .replace("ẽ̞n̪th", "ẽn̪θ")
        .replace("othes", "ɞ̜ʉ̞ðz")
        .replace("oth", "ɐ̠ð")
        .replace("uth", "ʉθ")
        .replace("ythm", "iðᵊm")
        .replace("yme", "ɑ̝ëme")
        .replace("gh", "g")
        .replace("than", "ðə̃n̪")
        .replace("thẽn̪", "ðẽn̪")
        .replace("these", "ðiːz")
        .replace("the ", "ðə ")
        .replace("thi", "θi")
        .replace("th", "ð")
        .replace("two", "tʰʉ̟ː")
        .replace("pt", "p̚t")
        .replace("spu", "spʲʉ̟ː")
        .replace("pu", "pʰu")
        .replace("pʊ̟", "pʰʊ̟")
        .replace("tʊ̟", "tʰʊ̟")
        .replace("dge", "d͡ʒe")
        .replace("cant", "kʰɐ̞̃ːnt")
        .replace("can", "kʰæ̃ːn")
        .replace("ken", "kʰɛ̃ːn")
        .replace("we ", "wᵊiː ")
        .replace("we.", "wᵊiː.")
        .replace("we,", "wᵊiː,")
        .replace("she ", "ʃᵊiː ")
        .replace("she.", "ʃᵊiː.")
        .replace("she,", "ʃᵊiː,")
        .replace("shes ", "ʃᵊiːz ")
        .replace("he ", "hᵊiː ")
        .replace("he.", "hᵊiː.")
        .replace("he,", "hᵊiː,")
        .replace("hes ", "hᵊiːz ")
        .replace("hi", "hɑ̝ë^")
        .replace("ish", "ɘʃ")
        .replace("sh", "ʃ")
        .replace("sto", "stɔ̈")
        .replace("nf", "ɱf")
        .replace("from", "fɹɔ̝̃m")
        .replace("oɱ", "ə̃ɱ")
        .replace("om", "ə̃m")
        .replace("bad", "bɶ̜d̥")
        .replace("hab", "hɶ̜b")
        .replace("had", "hɶ̜d")
        .replace("suk", "sək")
        .replace("app", "ɶ̜p")
        .replace("ap", "ɶ̜p")
        .replace("bit", "bət")
        .replace("rot", "ɹət")
        .replace("sys", "səs")
        .replace("ot", "ɔ̈t")
        .replace("ert", "eːt")
        .replace("ere ", "iə ")
        .replace("ere.", "iə.")
        .replace("ere,", "iə,")
        .replace("mber", "mer")
        .replace("er a", "əɹ a")
        .replace("er e", "əɹ e")
        .replace("er o", "əɹ o")
        .replace("er ", "ə ")
        .replace("er.", "ə.")
        .replace("er,", "ə,")
        .replace("em ", "əm ")
        .replace("em.", "əm.")
        .replace("em,", "əm,")
        .replace("ussy", "ʊ̟sy")
        .replace("usy", "ɘsy")
        .replace("ss", "s")
        .replace("ses", "zəz̥")
        .replace("es ", "z̥ ")
        .replace("es.", "z̥.")
        .replace("es,", "z̥,")
        .replace("sas", "zəz̥")
        .replace("as ", "əz̥ ")
        .replace("as.", "əz̥.")
        .replace("as,", "əz̥,")
        .replace("ɑ̝ës ", "ɑ̝ëz̥ ")
        .replace("ɑ̝ës.", "ɑ̝ëz̥.")
        .replace("ɑ̝ës,", "ɑ̝ëz̥,")
        .replace("ped ", "p̚t ")
        .replace("ped.", "p̚t.")
        .replace("ped,", "p̚t,")
        .replace("d͡ʒed", "d͡ʒd̚")
        .replace("d͡ʒd", "d͡ʒd̚")
        .replace("ked", "kt")
        .replace("k̥ed", "kt")
        .replace("ed ", "ᵊd̥ ")
        .replace("ed.", "ᵊd̥.")
        .replace("ed,", "ᵊd̥,")
        .replace("bəd̥", "bed̥")
        .replace("bod", "bɔ̈d")
        .replace(" be ", " bi^ ")
        .replace(".be ", ".bi^ ")
        .replace(" be.", " bi.")
        .replace(" be,", " bi,")
        .replace(".be,", ".bi,")
        .replace(" be", " bi")
        .replace(".be", ".bi")
        .replace(" me ", " mi^ ")
        .replace(" me.", " mi.")
        .replace(" me,", " mi,")
        .replace(".me,", ".mi,")
        .replace("cha", "t͡ʃɶ̜")
        .replace("nal", "nᵊl")
        .replace("ral", "ɹᵊl")
        .replace(" all ", " ɔ̈ːl ")
        .replace(".all ", ".ɔ̈ːl ")
        .replace(" all", " əl")
        .replace(".all", ".əl")
        .replace("all", "ɔ̈ːl")
        .replace(" al", " ɔ̈l")
        .replace(".al", ".ɔ̈ːl")
        .replace(" re", " ɹi")
        .replace(".re", ".ɹi")
        .replace("ation", "æ̠ëʃŋ")
        .replace("tion", "ʃŋ")
        .replace("sion", "ʃŋ")
        .replace("zion", "ʒᵊŋ")
        .replace("cky", "kiː")
        .replace("ctly", "t͡ʃl̥ᵊiː")
        .replace("tely", "ʔl̥ᵊiː")
        .replace("llery", "l̥ɹᵊiː")
        .replace("ily", "l̥ᵊiː")
        .replace("ly", "l̥ᵊiː")
        .replace("ical", "ikᵊl")
        .replace("ic ", "ikʰ ")
        .replace("ic.", "ikʰ.")
        .replace("ic,", "ikʰ,")
        .replace("ce ", "s ")
        .replace("ce.", "s.")
        .replace("ce,", "s,")
        .replace(" ive ", " ɑ̝ëv ")
        .replace(".ive ", ".ɑ̝ëv ")
        .replace("five", "fɑ̝ëv")
        .replace("e ", " ")
        .replace("e.", ".")
        .replace("e,", ",")
        .replace(" do ", " dʊ̟ ")
        .replace(" do.", " dʊ̟.")
        .replace(" do,", " dʊ̟,")
        .replace(".do ", ".dʊ̟ ")
        .replace(" to ", " tə ")
        .replace(" to.", " tʊ̟.")
        .replace("o ", "ɞ̜ʉ̞ ")
        .replace("o.", "ɞ̜ʉ̞.")
        .replace("o,", "ɞ̜ʉ̞,")
        .replace("or", "ɔ̈ː")
        .replace("to", "tə")
        .replace(" on ", " ɔ̈n ")
        .replace(" on.", " ɔ̈n.")
        .replace(" on,", " ɔ̈n,")
        .replace("cy ", "sᵊiː ")
        .replace("cy.", "sᵊiː.")
        .replace("cy,", "sᵊiː,")
        .replace("logy", "ləd͡ʒᵊiː")
        .replace("y ", "ᵊiː ")
        .replace("y.", "ᵊiː.")
        .replace("y,", "ᵊiː,")
        .replace("e-", "iː")
        .replace(" eco", " iːkɞ̜ʉ̞")
        .replace(".eco", ".iːkɞ̜ʉ̞")
        .replace("ci", "si")
        .replace("caf", "cɶ̜f")
        .replace("eful", "fʊ̟l")
        .replace("ful", "fʊ̟l")
        .replace("diff", "dif")
        .replace("fé", "fɶi")
        .replace("mach", "məkʰ")
        .replace("nnel", "nnᵊl")
        .replace("tch", "t͡ʃ")
        .replace("ch ", "t͡ʃ ")
        .replace("ch.", "t͡ʃ.")
        .replace("ch,", "t͡ʃ,")
        .replace("chɶ̜", "t͡ʃɶ̜")
        .replace("chʊ̟", "t͡ʃʊ̟")
        .replace("chec", "t͡ʃec")
        .replace("cho", "t͡ʃo")
        .replace("ch", "k")
        .replace("cc", "k")
        .replace("ck", "k")
        .replace('c', "k")
        .replace("kɶ̜", "kʰɶ̜")
        .replace("kl", "kl̥")
        .replace("gr", "ɡɹ̥")
        .replace("tr", "ṯɹ̥")
        .replace("um", "ɐ̃m")
        .replace("pʰut", "pʰʊ̟t")
        .replace("ʊ̟k", "ʊ̟kʰ")
        .replace("ɶ̜k", "ɶ̜kʰ")
        .replace("ck", "k")
        .replace("dɹ", "d͡ʒɹ̥")
        .replace("dst", "d.st")
        .replace("ds", "d͡z")
        .replace("d̥s", "d͡z")
        .replace("ns ", "nz̥ ")
        .replace("ns.", "nz̥.")
        .replace("ns,", "nz̥,")
        .replace("ys ", "yz̥ ")
        .replace("ys.", "yz̥.")
        .replace("ys,", "yz̥,")
        .replace("ŋs", "ŋs̬")
        .replace("æ̠ës", "æ̠ëz̥")
        .replace("exp", "iksp")
        .replace("exs", "eks")
        .replace("ext", "ekst")
        .replace("ex", "ɘ̟gz")
        .replace('x', "gz")
        .replace('s', "s̻")
        .replace("s̻̬", "s̬")
        .replace("i ", "ɑ̝ë ")
        .replace("ph", "f")
        .replace("ff", "f")
        .replace("pp", "p")
        .replace("pf", "f")
        .replace("oh ", "ɞ̜ʉ̞ ")
        .replace("oh.", "ɞ̜ʉ̞.")
        .replace("oh,", "ɞ̜ʉ̞,")
        .replace("h ", " ")
        .replace("h.", ".")
        .replace("h,", ",")
        .replace("mm", "m")
        .replace("mb ", "m ")
        .replace("mb.", "m.")
        .replace("mb,", "m,")
        .replace(" am ", " ɶ̜̃m ")
        .replace(" am.", " ɶ̜̃m.")
        .replace(" am,", " ɶ̜̃m,")
        .replace(".am ", ".ɶ̜̃m ")
        .replace("damn", "dɶ̜̃m")
        .replace("amn ", "ɶ̜̃m ")
        .replace("amn.", "ɶ̜̃m.")
        .replace("amn,", "ɶ̜̃m,")
        .replace("ɶ̜n", "æ̃n")
        .replace("ɶ̜̃n", "æ̃n")
        .replace("umn", "ᵊm")
        .replace("ymn", "im")
        .replace("nn", "n")
        .replace("s̻nt", "zᵊnt̚")
        .replace("nt ", "nt̚  ")
        .replace("nt.", "nt̚ .")
        .replace("nt,", "nt̚ ,")
        .replace('j', "d͡ʒ")
        .replace("d͡ʒ|", "j")
        .replace(" ʲʉ̟ː", " jʉ̟ː")
        .replace(".ʲʉ̟ː", ".jʉ̟ː")
        .replace(" ʲʉ̟̃ː", " jʉ̟̃ː")
        .replace(".ʲʉ̟̃ː", ".jʉ̟̃ː")
        .replace("oy", "o̝ɪ")
        .replace("at an", "aɹ an")
        .replace('y', "j")
        .replace('a', "ə")
        .replace("ə̹i", "ɑ̝ë")
        .replace('e', "e̽")
        .replace("e̽̈", "ë")
        .replace('i', "i̞")
        .replace("i̞ː", "iː")
        .replace("ĩ̞", "ɪ̃")
        .replace("ət", "ət̪")
        .replace("ət̪.", "ɶ̜t̪.")
        .replace("ət̪,", "ɶ̜t̪,")
        .replace("e̽t ", "e̽t̚ ")
        .replace("e̽t.", "e̽t̚.")
        .replace("e̽t,", "e̽t̚,")
        .replace("tt", "ð")
        .replace("ːt", "ːt̠")
        .replace("rr", "ɹ")
        .replace('r', "ɹ")
        .replace("ɹɹ", "ɹ")
        .replace("ll", "l")
        .replace("l ", "ɫ ")
        .replace("l.", "ɫ.")
        .replace("l,", "ɫ,")
        .replace("ɑ̝e̽ɫ", "ɑ̝e̽ᵊɫ")
        .replace("iːɫ", "iːᵊɫ")
        .replace("ʉ̟ːld", "ʉ̟ːd̥")
        .replace("ld", "l̥d")
        .replace("lw", "l̥w")
        .replace("t̪ ð", "t̪̚ ð")
        .replace("d ð", "d̪̚ ð")
        .replace("d ", "d̥ ")
        .replace("d.", "d̥.")
        .replace("d,", "d̥,")
        .replace("ðə ə", "ðiː ə")
        .replace("ðə ɐ̠", "ðiː ɐ̠")
        .replace("i̞ i̞", "i̞")
        .replace("n m", "m m")
        .replace("n n", " n")
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
        print!("{}", yellow);
        println!("{}", result);
        print!("{}", reset);
    }
}

//   ++++++++++   ++++++++++   ++++++++++

// ENGLISH: ORTHOGRAPHY

pub fn ortuseng(original_text: &str, usefile: &str, outputfile: &str, reset: &str, red: &str, cyan: &str, yellow: &str) {
    let space = original_text.to_owned() + " "; // mark word ending
    let result = &space
        .replace("celling", "celing")
        .replace("delling", "deling")
        .replace("velling", "veling")
        .replace("geing", "ging")
        .replace("teing", "ting")
        .replace("bour", "bor")
        .replace("iour", "ior")
        .replace("nour", "nor")
        .replace("olour", "olor")
        .replace("centre", "center")
        .replace("metre", "meter")
        .replace("theatre", "theater")
        .replace("fence", "fense")
        .replace("lise", "lize")
        .replace("lyse", "lyze")
        .replace("sation", "zation")
        .replace("veable", "vable")
        .replace("zeable", "zable")
        .replace("aestetic", "estetic")
        .replace("Aestetic", "Estetic")
        .replace("faec", "fec")
        .replace("Faec", "Fec")
        .replace("palaeo", "paleo")
        .replace("Palaeo", "Paleo")
        .replace("manoeuvre", "maneuver")
        .replace("oestr", "estr")
        .replace("oeso", "eso")
        .replace("paed", "ped")
        .replace("aeon", "eon")
        .replace("foet", "fet")
        .replace("haem", "hem")
        .replace("aluminium", "aluminum")
        .replace("Aluminium", "Aluminum")
        .replace(" axe ", " ax ")
        .replace(" axe,", " ax,")
        .replace(" axe.", " ax.")
        .replace("cheque", "check")
        .replace("cumquat", "kumquat")
        .replace("alment", "allment")
        .replace("ilful", "illful")
        .replace("distil", "distill")
        .replace("distilll", "distill")
        .replace("Distil", "Distill")
        .replace("Distilll", "Distill")
        .replace("fulfil", "fulfill")
        .replace("fulfilll", "fulfill")
        .replace("Fulfil", "Fulfill")
        .replace("Fulfilll", "Fulfill")
        .replace("grey", "gray")
        .replace("Grey", "Gray")
        .replace("kerb", "curb")
        .replace("moustache", "mustache")
        .replace("eurone", "euron")
        .replace("nnexe", "nnex")
        .replace("nnexd", "nnexed")
        .replace("nnexs", "nnexes")
        .replace("racoon", "raccoon")
        .replace("Racoon", "Raccoon")
        .replace("mum ", "mom ")
        .replace("mum,", "mom,")
        .replace("mum.", "mom.")
        .replace("Mum,", "Mom,")
        .replace("ogramme", "ogram")
        .replace("logue", "log")
        .replace("sceptical", "skeptical")
        .replace("Sceptical", "Skeptical")
        .replace("CELLING", "CELING")
        .replace("DELLING", "DELING")
        .replace("VELLING", "VELING")
        .replace("GEING", "GING")
        .replace("TEING", "TING")
        .replace("BOUR", "BOR")
        .replace("IOUR", "IOR")
        .replace("NOUR", "NOR")
        .replace("OLOUR", "OLOR")
        .replace("CENTRE", "CENTER")
        .replace("METRE", "METER")
        .replace("THEATRE", "THEATER")
        .replace("FENCE", "FENSE")
        .replace("LISE", "LIZE")
        .replace("LYSE", "LYZE")
        .replace("SATION", "ZATION")
        .replace("VEABLE", "VABLE")
        .replace("ZEABLE", "ZABLE")
        .replace("AESTETIC", "ESTETIC")
        .replace("FAEC", "FEC")
        .replace("PALAEO", "PALEO")
        .replace("MANOEUVRE", "MANEUVER")
        .replace("OESTR", "ESTR")
        .replace("OESO", "ESO")
        .replace("PAED", "PED")
        .replace("AEON", "EON")
        .replace("FOET", "FET")
        .replace("HAEM", "HEM")
        .replace("ALUMINIUM", "ALUMINUM")
        .replace(" AXE ", " AX ")
        .replace(" AXE,", " AX,")
        .replace(" AXE.", " AX.")
        .replace("CHEQUE", "CHECK")
        .replace("CUMQUAT", "KUMQUAT")
        .replace("ALMENT", "ALLMENT")
        .replace("ILFUL", "ILLFUL")
        .replace("DISTIL", "DISTILL")
        .replace("DISTILLL", "DISTILL")
        .replace("FULFIL", "FULFILL")
        .replace("FULFILLL", "FULFILL")
        .replace("GREY", "GRAY")
        .replace("KERB", "CURB")
        .replace("MOUSTACHE", "MUSTACHE")
        .replace("EURONE", "EURON")
        .replace("NNEXE", "NNEX")
        .replace("NNEXD", "NNEXED")
        .replace("NNEXS", "NNEXES")
        .replace("RACOON", "RACCOON")
        .replace("MUM ", "MOM ")
        .replace("MUM,", "MOM,")
        .replace("MUM.", "MOM.")
        .replace("OGRAMME", "OGRAM")
        .replace("LOGUE", "LOG")
        .replace("SCEPTICAL", "SKEPTICAL")
        .replace("liquorice", "licorice")
        .replace("LIQUORICE", "LICORICE")
        .replace("tyre", "tire")
        .replace("TYRE", "TIRE")
        .replace("wellery", "welry")
        .replace("WELLERY", "WELRY");

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
        file.write_all("AMERICAN ENGLISH:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("{}", cyan.to_owned() + "American English" + reset + ":");
        println!();
        print!("{}", yellow);
        println!("{}", result);
        print!("{}", reset);
    }
}
