// LNGCNV VERSION 1.8.9 / MIT LICENSE / COPYRIGHT © 2022–2023 PIOTR BAJDEK

// MODULE MODQUE

// CLIPPY LINTS

#![deny(clippy::no_effect_replace)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::too_many_lines, clippy::unicode_not_nfc)]
#![allow(clippy::string_lit_as_bytes)]

// IMPORTS

use std::fs::OpenOptions;
use std::io::Write;

// SIMPLIFY INTERPUNCTION

fn quepncbeg(lowercase: &str) -> String {
    let pncbeg = &lowercase
        .replace([';', '!', '?', '¡', '¿'], ".")
        .replace(").", ".")
        .replace("),", ",")
        .replace(" «", ", ")
        .replace(".«", "")
        .replace("».", ".")
        .replace([')', ':', '»'], ",")
        .replace("--", " – ")
        .replace('(', "∣ .")
        .replace(' ', "# #")
        .replace('.', "#.#")
        .replace(',', "#,#");
    pncbeg.to_string()
}

// REMOVE INTERPUNCTION

fn quepncend(strmod: &str) -> String {
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

// AYACUCHO QUECHUA: IPA

pub fn ipaque(original_text: &str, usefile: &str, outputfile: &str, reset: &str, red: &str, cyan: &str, yellow: &str) {
    let dotend = original_text.to_owned() + ".";
    let dotbeg = ".".to_owned() + &dotend;
    let lowercase = &dotbeg.to_lowercase();
    let pncbeg = quepncbeg(lowercase);

    let strmod = &pncbeg
        .replace("sh", "ch")
        .replace("chh", "ch")
        .replace("kh", "k")
        .replace("th", "t")
        .replace("ph", "p")
        .replace("qh", "q")
        .replace('j', "q")
        .replace(['\'', '’'], "")
        .replace("qu", "qo")
        .replace("uq", "oq")
        .replace("qi", "qe")
        .replace("iq", "eq")
        .replace('á', "a")
        .replace('í', "i")
        .replace("nk", "ŋk")
        .replace("an#", "aŋ#")
        .replace("ma", "mæ")
        .replace("mis", "mɪ̹s")
        .replace("mich", "mɪ̹ch")
        .replace("cha", "chä")
        .replace('g', "ɡ")
        .replace("ka", "kæ̞")
        .replace("da", "dæ")
        .replace('d', "d̥")
        .replace("lla", "llæ")
        .replace("ll", "ʎ")
        .replace('ñ', "ɲ")
        .replace("pa", "pä")
        .replace('f', "f̟")
        .replace('q', "χ")
        .replace('r', "ɾ")
        .replace("ɾχ", "ɾ̥χ")
        .replace("χa", "χæ̈")
        .replace("sa", "sä")
        .replace('s', "s̻")
        .replace("ci", "s̻i")
        .replace("ce", "s̻e")
        .replace('z', "s̻")
        .replace("ch", "t̠͡ʃ")
        .replace("wa", "wæ")
        .replace("ya", "yæ")
        .replace('y', "j")
        .replace('o', "ɵ̠")
        .replace('e', "ɘ̟")
        .replace('u', "u̞")
        .replace('i', "i̞");

    let result = quepncend(strmod);

    if usefile == "new" {
        let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
        file.write_all("AYACUCHO QUECHUA (WANTA, PE):".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "old" {
        let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
        file.write_all("AYACUCHO QUECHUA (WANTA, PE):".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("{}", cyan.to_owned() + "Ayacucho Quechua" + reset + " (" + cyan + "Wanta" + reset + ", " + cyan + "PE" + reset + "):");
        println!();
        print!("{yellow}");
        println!("{result}");
        print!("{reset}");
    }
}

//   ++++++++++   ++++++++++   ++++++++++

// AYACUCHO QUECHUA: DIALECT

pub fn quelct(original_text: &str, usefile: &str, outputfile: &str, reset: &str, red: &str, cyan: &str, yellow: &str) {
    let space = original_text.to_owned() + " ";
    let result = &space
        .replace('o', "u")
        .replace('e', "i")
        .replace("chh", "ch")
        .replace("kh", "k")
        .replace("th", "t")
        .replace("ph", "p")
        .replace("qh", "q")
        .replace('j', "q")
        .replace(['\'', '’'], "")
        .replace("san", "chkan")
        .replace("shan", "chkan")
        .replace("shyan", "chkan")
        .replace("shwan", "swan")
        .replace("sh", "ch")
        .replace(" aska ", " achka ")
        .replace("qti", "pti")
        .replace("lqu", "llqu")
        .replace("lllqu", "llqu")
        .replace("ulqi", "ullqi")
        .replace("an ", "am ")
        .replace("an.", "am.")
        .replace("an,", "am,")
        .replace("an;", "am;")
        .replace("an:", "am:")
        .replace("an!", "am!")
        .replace("an?", "am?")
        .replace("chkam", "chkan")
        .replace("hanam", "hanan")
        .replace("Hanam", "Hanan")
        .replace("kanam", "kanan")
        .replace("Kanam", "Kanan")
        .replace(" kunam", " kunan")
        .replace("Kunam", "Kunan")
        .replace(" ñam ", " ñan ")
        .replace(" yam ", " ñan ")
        .replace("mam ", "man ")
        .replace("mam.", "man.")
        .replace("mam,", "man,")
        .replace("mam;", "man;")
        .replace("mam:", "man:")
        .replace("mam!", "man!")
        .replace("mam?", "man?")
        .replace("munam", "munan")
        .replace("swam", "swan")
        .replace("wam ", "wan ")
        .replace("wam.", "wan.")
        .replace("wam,", "wan,")
        .replace("wam;", "wan;")
        .replace("wam:", "wan:")
        .replace("wam!", "wan!")
        .replace("wam?", "wan?")
        .replace("un ", "um ")
        .replace("un.", "um.")
        .replace("un,", "um,")
        .replace("un;", "um;")
        .replace("un:", "um:")
        .replace("un!", "um!")
        .replace("un?", "um?")
        .replace("hatum", "hatun")
        .replace("Hatum", "Hatun")
        .replace("sum ", "sun ")
        .replace("sum.", "sun.")
        .replace("sum,", "sun,")
        .replace("sum;", "sun;")
        .replace("sum:", "sun:")
        .replace("sum!", "sun!")
        .replace("sum?", "sun?")
        .replace("squm", "squn")
        .replace(" ukum ", " ukun ")
        .replace("in ", "im ")
        .replace("in.", "im.")
        .replace("in,", "im,")
        .replace("in;", "im;")
        .replace("in:", "im:")
        .replace("in!", "im!")
        .replace("in?", "im?")
        .replace(" nim ", " nin ")
        .replace("allim", "allin")
        .replace("Allim", "Allin")
        .replace("knim", "knin")
        .replace("nnim", "nnin")
        .replace("qnim", "qnin")
        .replace("ptim", "ptin")
        .replace("stim", "stin")
        .replace("ntim ", "ntin ")
        .replace("ntim.", "ntin.")
        .replace("ntim,", "ntin,")
        .replace("ntim;", "ntin;")
        .replace("ntim:", "ntin:")
        .replace("ntim?", "ntin?")
        .replace("ntim!", "ntin!")
        .replace("urim ", "urin ")
        .replace("urim.", "urin.")
        .replace("urim,", "urin,")
        .replace("urim;", "urin;")
        .replace("urim:", "urin:")
        .replace("urim?", "urin?")
        .replace("urim!", "urin!")
        .replace("Urim", "Urin")
        .replace("ynim", "ynin")
        .replace(" chanim", " chanin")
        .replace("llimpim", "llimpin")
        .replace(" sapim ", " sapin ")
        .replace(" rapim ", " rapin ")
        .replace("q simim", "pa simin")
        .replace("pa simim", "pa simin")
        .replace("q sutim", "pa sutin")
        .replace("pa sutim", "pa sutin")
        .replace("churim", "churin")
        .replace("q churin", "pa churin")
        .replace("turim", "turin")
        .replace("q turin", "pa turin")
        .replace("aqarim ", "aqarin ")
        .replace("wakim", "wakin")
        .replace("Wakim", "Wakin")
        .replace("nuqa", "ñuqa")
        .replace("ñuqaq", "ñuqapa")
        .replace("Nuqa", "Ñuqa")
        .replace("Ñuqaq", "Ñuqapa")
        .replace("nchis", "nchik")
        .replace("nchiq", "nchik")
        .replace("nkichis", "nkichik")
        .replace("nkichiq", "nkichik")
        .replace("ykichis", "ykichik")
        .replace(" qanchik", " qanchis")
        .replace("qankuna", "qamkuna")
        .replace("Qankuna", "Qamkuna")
        .replace(" qanqa ", " qamqa ")
        .replace("Qanqa", "Qampa")
        .replace(" qanpa ", " qampa ")
        .replace("Qanpa", "Qampa")
        .replace("qanpaq", "qampaq")
        .replace("Qanpaq", "Qampaq")
        .replace(" qanwan", " qamwan")
        .replace("Qanwan", "Qamwan")
        .replace("Qan ", "Qam ")
        .replace("personayki", "qam")
        .replace("Personayki", "Qam")
        .replace("llanka", "llamka")
        .replace("Llanka", "Llamka")
        .replace(" uk ", " huk ")
        .replace("kinsa", "kimsa")
        .replace("Kinsa", "Kimsa")
        .replace("pisqa", "pichqa")
        .replace("Pisqa", "Pichqa")
        .replace(" ukya", " upya")
        .replace("Ukya", "Upya")
        .replace(" uqya", " upya")
        .replace("Uqya", "Upya")
        .replace("llillanmi", "llinllam")
        .replace("llillanchu", "llinllachu")
        .replace(" unu ", " yaku ")
        .replace(" unuta ", " yakuta ")
        .replace("haykaq", "haykapi")
        .replace("Haykaq", "Haykapi")
        .replace("maykaq", "haykapi")
        .replace("Maykaq", "Haykapi")
        .replace("mayka", "hayka")
        .replace("Mayka", "Hayka")
        .replace("mashqa", "hayka")
        .replace("Mashqa", "Hayka")
        .replace("punchay", "punchaw")
        .replace("ñaqa", "yaqa")
        .replace("Ñaqa", "Yaqa")
        .replace(" qaya ", " paqarin ")
        .replace(" qayan ", " paqarinmi ")
        .replace("Qaya ", "Paqarin ")
        .replace("Qayan ", "Paqarinmi ")
        .replace("Chh", "Ch")
        .replace("Kh", "K")
        .replace("Th", "T")
        .replace("Ph", "P")
        .replace("Qh", "Q")
        .replace("chum ", "chun ")
        .replace("chum.", "chun.")
        .replace("chum,", "chun,")
        .replace("Uchun ", "Uchum ")
        .replace("q parla", "pa parla")
        .replace("q rima", "pa rima")
        .replace("q simi", "pa simi");

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
        file.write_all("AYACUCHO QUECHUA:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("{}", cyan.to_owned() + "Ayacucho Quechua" + reset + ":");
        println!();
        print!("{yellow}");
        println!("{result}");
        print!("{reset}");
    }
}

//   ++++++++++   ++++++++++   ++++++++++

// QUECHUA: TRIVOCALIC ORTHOGRAPHY

pub fn ortquetri(original_text: &str, usefile: &str, outputfile: &str, reset: &str, red: &str, cyan: &str, yellow: &str) {
    let result = original_text.replace('o', "u").replace('O', "U").replace('e', "i").replace('E', "I");

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
        file.write_all("TRIVOCALIC:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("{}", cyan.to_owned() + "Trivocalic" + reset + ":");
        println!();
        print!("{yellow}");
        println!("{result}");
        print!("{reset}");
    }
}

//   ++++++++++   ++++++++++   ++++++++++

// QUECHUA: PENTAVOCALIC ORTHOGRAPHY

pub fn ortquepen(original_text: &str, usefile: &str, outputfile: &str, reset: &str, red: &str, cyan: &str, yellow: &str) {
    let result = original_text
        .replace("qu", "qo")
        .replace("Qu", "Qo")
        .replace("QU", "QO")
        .replace("qi", "qe")
        .replace("Qi", "Qe")
        .replace("QI", "QE")
        .replace("qhu", "qho")
        .replace("Qhu", "Qho")
        .replace("QHU", "QHO")
        .replace("qhi", "qhe")
        .replace("Qhi", "Qhe")
        .replace("QHI", "QHE")
        .replace("ko", "ku")
        .replace("Ko", "Ku")
        .replace("KO", "KU")
        .replace("ke", "ki")
        .replace("Ke", "Ki")
        .replace("KE", "KI")
        .replace("kho", "khu")
        .replace("Kho", "Khu")
        .replace("KHO", "KHU")
        .replace("khe", "khi")
        .replace("Khe", "Khi")
        .replace("KHE", "KHI")
        .replace("q'u", "q'o")
        .replace("Q'u", "Q'o")
        .replace("Q'U", "Q'O")
        .replace("q'i", "q'e")
        .replace("Q'i", "Q'e")
        .replace("Q'I", "Q'E")
        .replace("q’u", "q’o")
        .replace("Q’u", "Q’o")
        .replace("Q’U", "Q’O")
        .replace("q’i", "q’e")
        .replace("Q’i", "Q’e")
        .replace("Q’I", "Q’E")
        .replace("uq", "oq")
        .replace("Uq", "Oq")
        .replace("UQ", "OQ")
        .replace("iq", "eq")
        .replace("Iq", "Eq")
        .replace("IQ", "EQ");

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
        file.write_all("PENTAVOCALIC:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("{}", cyan.to_owned() + "Pentavocalic" + reset + ":");
        println!();
        print!("{yellow}");
        println!("{result}");
        print!("{reset}");
    }
}
