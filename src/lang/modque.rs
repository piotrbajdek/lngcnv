// LNGCNV VERSION 1.6.0-BETA.8 / MIT LICENSE © 2022 PIOTR BAJDEK

// MODULE MODQUE

// CLIPPY LINTS

#![deny(clippy::no_effect_replace)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::single_char_pattern, clippy::too_many_lines, clippy::unicode_not_nfc)]
#![allow(clippy::string_lit_as_bytes)] // must be as_bytes() because non-ASCII characters are included

// IMPORTS

use std::fs::OpenOptions;
use std::io::Write;

// AYACUCHO QUECHUA: IPA

pub fn ipaque(original_text: &str, usefile: &str, outputfile: &str) {
    let reset = "\x1b[0m";
    let red = "\x1b[31m";
    let yellow = "\x1b[93m";

    let lowercase = original_text.to_lowercase();
    let result = &lowercase
        .replace("sh", "ch")
        .replace("chh", "ch")
        .replace("kh", "k")
        .replace("th", "t")
        .replace("ph", "p")
        .replace("qh", "q")
        .replace("j", "q")
        .replace("'", "")
        .replace('’', "")
        .replace("qu", "qo")
        .replace("uq", "oq")
        .replace("qi", "qe")
        .replace("iq", "eq")
        .replace("an", "aŋ")
        .replace("ma", "mæ")
        .replace("mis", "mɪ̝s̻")
        .replace("mich", "mɪ̝ch")
        .replace("cha", "chä")
        .replace("ch", "ʧ")
        .replace("g", "ɡ")
        .replace("ka", "kæ̞")
        .replace("da", "dæ")
        .replace("d", "d̥")
        .replace("lla", "llæ")
        .replace("ll", "ʎ")
        .replace("ñ", "ɲ")
        .replace("pa", "pä")
        .replace("f", "f̟")
        .replace("q", "χ")
        .replace("r", "ɾ")
        .replace("ɾχ", "ɾ̥χ")
        .replace("sa", "s̻ä")
        .replace("s", "s̻")
        .replace("ci", "s̻i")
        .replace("ce", "s̻e")
        .replace("z", "s̻")
        .replace("t", "t̪")
        .replace("wa", "wæ")
        .replace("ya", "yæ")
        .replace("y", "j")
        .replace("o", "ʊ̞")
        .replace("e", "ɪ̞")
        .replace("u", "u̞")
        .replace("i", "i̞")
        .replace(',', " ∣")
        .replace(';', " ∥")
        .replace(':', " ∣")
        .replace(". ", " ∥ ")
        .replace('.', "")
        .replace("! ", " ∥ ")
        .replace('!', "")
        .replace('¡', "")
        .replace("? ", " ∥ ")
        .replace('?', "")
        .replace('¿', "")
        .replace('(', "∣ ")
        .replace(')', " ∣")
        .replace(" - ", " ∣ ")
        .replace(" – ", " ∣ ")
        .replace("--", " ∣ ");

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
        file.write_all("AYACUCHO QUECHUA (WANTA):".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("Ayacucho Quechua (Wanta):");
        println!();
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

    let space = original_text.to_owned() + " "; // mark word ending
    let result = &space
        .replace("o", "e")
        .replace("e", "i")
        .replace("chh", "ch")
        .replace("kh", "k")
        .replace("th", "t")
        .replace("ph", "p")
        .replace("qh", "q")
        .replace("j", "q")
        .replace("'", "")
        .replace('’', "")
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
        println!("Ayacucho Quechua:");
        println!();
        print!("{}", yellow);
        println!("{}", result);
        print!("{}", reset);
    }
}

//   ++++++++++   ++++++++++   ++++++++++

// QUECHUA: TRIVOCALIC ORTHOGRAPHY

pub fn ortquetri(original_text: &str, usefile: &str, outputfile: &str) {
    let reset = "\x1b[0m";
    let red = "\x1b[31m";
    let yellow = "\x1b[93m";

    let result = original_text.replace("o", "u").replace("O", "U").replace("e", "i").replace("E", "I");

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
        println!("Trivocalic:");
        println!();
        print!("{}", yellow);
        println!("{}", result);
        print!("{}", reset);
    }
}

//   ++++++++++   ++++++++++   ++++++++++

// QUECHUA: PENTAVOCALIC ORTHOGRAPHY

pub fn ortquepen(original_text: &str, usefile: &str, outputfile: &str) {
    let reset = "\x1b[0m";
    let red = "\x1b[31m";
    let yellow = "\x1b[93m";

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
        println!("Pentavocalic:");
        println!();
        print!("{}", yellow);
        println!("{}", result);
        print!("{}", reset);
    }
}
