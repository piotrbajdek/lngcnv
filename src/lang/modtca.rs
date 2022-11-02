// LNGCNV VERSION 1.6.0-BETA.9 / MIT LICENSE © 2022 PIOTR BAJDEK

// MODULE MODTCA

// CLIPPY LINTS

#![deny(clippy::no_effect_replace)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::single_char_pattern, clippy::similar_names, clippy::too_many_lines, clippy::unicode_not_nfc)]
#![allow(clippy::string_lit_as_bytes)] // must be as_bytes() because non-ASCII characters are included

// IMPORTS

use std::fs::OpenOptions;
use std::io::Write;

// TONES AND GLOTTAL STOPS

fn tcasym(lowercase: &str) -> String {
    let strsym = &lowercase.replace('1', "₁").replace('2', "₂").replace('3', "₃").replace('4', "₄").replace('5', "₅").replace('6', "₆").replace('x', "ʔ").replace("'", "ʔ").replace('’', "ʔ");
    strsym.to_string()
}

// REMOVE INTERPUNCTION

fn tcapnc(strmod: &str) -> String {
    let result = &strmod
        .replace(". ", " ∥ ")
        .replace('.', "")
        .replace(',', " ∣")
        .replace(';', " ∥")
        .replace(':', " ∣")
        .replace("? ", " ∥ ")
        .replace('?', "")
        .replace('¿', "")
        .replace("! ", " ∥ ")
        .replace('!', "")
        .replace('¡', "")
        .replace(" - ", " ∣ ")
        .replace(" – ", " ∣ ")
        .replace("--", " ∣ ")
        .replace('(', "∣ ")
        .replace(')', " ∣");
    result.to_string()
}

// UMARIAÇU: IPA

pub fn tcabrumariacu(original_text: &str, usefile: &str, outputfile: &str) {
    let reset = "\x1b[0m";
    let red = "\x1b[31m";
    let yellow = "\x1b[93m";

    let lowercase = original_text.to_lowercase();
    let strsym = tcasym(&lowercase);

    let strmod = &strsym
        .replace("b", "b̥")
        .replace("cue", "kue")
        .replace("cui", "kui")
        .replace("c", "k")
        .replace("tkh", "ch")
        .replace("kh", "ch")
        .replace("ch", "ʧ")
        .replace("ng", "ŋ")
        .replace("g", "ɡ̥")
        .replace("n", "n̪")
        .replace("nh", "ñ")
        .replace("ñ", "ɲ")
        .replace("que", "ke")
        .replace("qui", "ki")
        .replace("q", "k")
        .replace("r", "ɾ")
        .replace("t", "t̪")
        .replace("y", "d͡ʒ")
        .replace("j", "d͡ʒ")
        .replace("f", "ɸ")
        .replace("z", "s")
        .replace("ç", "s")
        .replace("s", "s̺")
        .replace("v", "ɰ")
        .replace("ɡ", "ɡ̥")
        .replace("ã", "ɑ̃")
        .replace("ã", "ɑ̃")
        .replace("õ", "ɔ̝̃")
        .replace("õ", "ɔ̝̃")
        .replace("ẽ", "ɛ̝̃")
        .replace("ẽ", "ɛ̝̃")
        .replace("ũ", "ʊ̃")
        .replace("ũ", "ʊ̃")
        .replace("ü̃", "ɯ̃")
        .replace("a̰", "ʌ̰")
        .replace("a̱", "ʌ̰")
        .replace("a̠", "ʌ̰")
        .replace("o̰", "ɔ̰")
        .replace("o̱", "ɔ̰")
        .replace("o̠", "ɔ̰")
        .replace("ḛ", "ɛ̰")
        .replace("e̱", "ɛ̰")
        .replace("e̠", "ɛ̰")
        .replace("i̱", "ḭ")
        .replace("i̠", "ḭ")
        .replace("ṵ", "ʊ̰")
        .replace("u̱", "ʊ̰")
        .replace("u̠", "ʊ̰")
        .replace("ṵ̈", "ɯ̰")
        .replace("ü̱", "ɯ̰")
        .replace("ü̠", "ɯ̰")
        .replace("á", "a")
        .replace("é", "ɛ̝")
        .replace("í", "i")
        .replace("ó", "o")
        .replace("ú", "ʊ")
        .replace("ǘ", "ü")
        .replace("à", "a")
        .replace("è", "ɛ̝")
        .replace("ì", "i")
        .replace("ò", "o")
        .replace("ù", "ʊ")
        .replace("ǜ", "ü")
        .replace("w", "ɰ")
        .replace("d͡ʒae", "d͡ʒæi")
        .replace("a", "ɑ̈")
        .replace("e", "ɛ̝")
        .replace("o", "ɔ̝")
        .replace("u", "ʊ")
        .replace("ü", "ɯ̈")
        .replace("ɑ̈ɑ̃", "ɑ̃ː")
        .replace("ɑ̈ɑ̈", "ɑ̈ː");

    let result = tcapnc(strmod);

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
        file.write_all("UMARIAÇU, BR:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("Umariaçu, BR:");
        println!();
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

    let lowercase = original_text.to_lowercase();
    let strsym = tcasym(&lowercase);

    let strmod = &strsym
        .replace("b", "b̥")
        .replace("cue", "kue")
        .replace("cui", "kui")
        .replace("c", "k")
        .replace("tkh", "ch")
        .replace("kh", "ch")
        .replace("ch", "ʧ")
        .replace("ng", "ŋ")
        .replace("g", "ɡ")
        .replace("n", "n̪")
        .replace("nh", "ñ")
        .replace("ñ", "ɲ")
        .replace("que", "ke")
        .replace("qui", "ki")
        .replace("q", "k")
        .replace("r", "ɾ")
        .replace("t", "t̪")
        .replace("y", "d͡ʒ")
        .replace("j", "d͡ʒ")
        .replace("f", "ɸ")
        .replace("z", "s")
        .replace("ç", "s")
        .replace("s", "s̺")
        .replace("v", "ɰ")
        .replace("ɡ", "ɡ̥")
        .replace("ã", "ɐ̝̃")
        .replace("ã", "ɐ̝̃")
        .replace("õ", "ɒ̃")
        .replace("õ", "ɒ̃")
        .replace("ẽ", "ɶ̃")
        .replace("ẽ", "ɶ̃")
        .replace("ũ", "ũ")
        .replace("ü̃", "ɤ̝̃")
        .replace("a̰", "ʌ̰̈")
        .replace("a̱", "ʌ̰̈")
        .replace("a̠", "ʌ̰̈")
        .replace("o̰", "ɒ̰")
        .replace("o̱", "ɒ̰")
        .replace("o̠", "ɒ̰")
        .replace("ḛ", "ɶ̰")
        .replace("e̱", "ɶ̰")
        .replace("e̠", "ɶ̰")
        .replace("i̱", "ḭ")
        .replace("i̠", "ḭ")
        .replace("u̱", "ṵ")
        .replace("u̠", "ṵ")
        .replace("ṵ̈", "ɤ̰")
        .replace("ü̱", "ɤ̰")
        .replace("ü̠", "ɤ̰")
        .replace("á", "a")
        .replace("é", "e")
        .replace("í", "i")
        .replace("ó", "o")
        .replace("ú", "u")
        .replace("ǘ", "ü")
        .replace("à", "a")
        .replace("è", "e")
        .replace("ì", "i")
        .replace("i", "i̞")
        .replace("ò", "o")
        .replace("ù", "u")
        .replace("ǜ", "ü")
        .replace("w", "ɰ")
        .replace("d͡ʒae", "d͡ʒɶi̞")
        .replace("a", "ɐ̝")
        .replace("e", "ɶ")
        .replace("o", "ɒ")
        .replace("u", "ü")
        .replace("ü̃", "ũ")
        .replace("ü", "ɤ̝")
        .replace("ɐ̝ɐ̝̃", "ɐ̝̃ː")
        .replace("ɐ̝ɐ̝", "ɐ̝ː");

    let result = tcapnc(strmod);

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
        file.write_all("VILA BETÂNIA, BR:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("Vila Betânia, BR:");
        println!();
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

    let lowercase = original_text.to_lowercase();
    let strsym = tcasym(&lowercase);

    let strmod = &strsym
        .replace("b", "b̥")
        .replace("w", "ɰ")
        .replace("v", "ɰ")
        .replace("cue", "kue")
        .replace("cui", "kui")
        .replace("c", "k")
        .replace("tkh", "ch")
        .replace("kh", "ch")
        .replace("ch", "ʧ")
        .replace("ng", "ŋ")
        .replace("g", "ɡ")
        .replace("n", "n̪")
        .replace("nh", "ñ")
        .replace("ñ", "ɲ")
        .replace("que", "ke")
        .replace("qui", "ki")
        .replace("q", "k")
        .replace("r", "ɾ")
        .replace("t", "t̪")
        .replace("y", "d͡ʒ")
        .replace("j", "d͡ʒ")
        .replace("f", "ɸ")
        .replace("z", "s")
        .replace("ç", "s")
        .replace("s", "s̺")
        .replace("ã", "ɐ̃")
        .replace("ã", "ɐ̃")
        .replace("ũ", "ʉ̃")
        .replace("ũ", "ʉ̃")
        .replace("ü̃", "ɨ̃")
        .replace("a̰", "ʌ̰")
        .replace("a̱", "ʌ̰")
        .replace("a̠", "ʌ̰")
        .replace("o̱", "o̰")
        .replace("o̠", "o̰")
        .replace("e̱", "ḛ")
        .replace("e̠", "ḛ")
        .replace("i̱", "ḭ")
        .replace("i̠", "ḭ")
        .replace("u̱", "ṵ")
        .replace("u̠", "ṵ")
        .replace("ṵ̈", "ɨ̰")
        .replace("ü̱", "ɨ̰")
        .replace("ü̠", "ɨ̰")
        .replace("á", "a")
        .replace("é", "e")
        .replace("í", "i")
        .replace("ó", "o")
        .replace("ú", "ʉ̱")
        .replace("ǘ", "ü")
        .replace("à", "a")
        .replace("è", "e")
        .replace("ì", "i")
        .replace("ò", "o")
        .replace("ù", "ʉ̱")
        .replace("ǜ", "ü")
        .replace("d͡ʒae", "d͡ʒæː")
        .replace("a", "ɐ")
        .replace("u", "ʉ̱")
        .replace("ü", "ɨ")
        .replace("ʉ̱̰", "ṵ")
        .replace("ɐɐ̃", "ɐ̃ː")
        .replace("ɐɐ", "ɐː");

    let result = tcapnc(strmod);

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
        file.write_all("NAZARETH, CO:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("Nazareth, CO:");
        println!();
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

    let lowercase = original_text.to_lowercase();
    let strsym = tcasym(&lowercase);

    let strmod = &strsym
        .replace("b", "b̥")
        .replace("cue", "kue")
        .replace("cui", "kui")
        .replace("c", "k")
        .replace("tkh", "ch")
        .replace("kh", "ch")
        .replace("ch", "ʧ")
        .replace("ng", "ŋ")
        .replace("g", "ɡ")
        .replace("n", "n̪")
        .replace("nh", "ñ")
        .replace("ñ", "ɲ")
        .replace("que", "ke")
        .replace("qui", "ki")
        .replace("q", "k")
        .replace("r", "ɾ")
        .replace("t", "t̪")
        .replace("y", "d͡ʒ")
        .replace("j", "d͡ʒ")
        .replace("f", "ɸ")
        .replace("z", "s")
        .replace("ç", "s")
        .replace("s", "s̺")
        .replace("v", "ɰ")
        .replace("k", "k̬")
        .replace("ã", "ɐ̃")
        .replace("ã", "ɐ̃")
        .replace("õ", "ɒ̃")
        .replace("õ", "ɒ̃")
        .replace("ẽ", "ɶ̃")
        .replace("ẽ", "ɶ̃")
        .replace("ũ", "ʊ̞̃")
        .replace("ũ", "ʊ̞̃")
        .replace("ü̃", "ə̃")
        .replace("a̰", "ʌ̰")
        .replace("a̱", "ʌ̰")
        .replace("a̠", "ʌ̰")
        .replace("o̰", "ɒ̰")
        .replace("o̱", "ɒ̰")
        .replace("o̠", "ɒ̰")
        .replace("ḛ", "ɶ̰")
        .replace("e̱", "ɶ̰")
        .replace("e̠", "ɶ̰")
        .replace("i̱", "ḭ")
        .replace("i̠", "ḭ")
        .replace("ṵ̈", "ɘ̰")
        .replace("ü̱", "ɘ̰")
        .replace("ü̠", "ɘ̰")
        .replace("á", "a")
        .replace("é", "e")
        .replace("í", "i")
        .replace("ó", "o")
        .replace("ú", "u")
        .replace("ǘ", "ü")
        .replace("à", "a")
        .replace("è", "e")
        .replace("ì", "i")
        .replace("ò", "o")
        .replace("ù", "u")
        .replace("ǜ", "ü")
        .replace("w", "ɰ")
        .replace("ɡü", "ɡüː")
        .replace("k̬ü", "k̬üː")
        .replace("ɾü", "ɾüː")
        .replace("a", "ɐ̝")
        .replace("e", "ɶ")
        .replace("i", "i̞")
        .replace("o", "ɒ")
        .replace("ṵ", "o̰")
        .replace("u̱", "o̰")
        .replace("u̠", "o̰")
        .replace("u", "ö")
        .replace("ü", "ɘ")
        .replace("ɐ̝ɐ̝", "ɐ̝ː");

    let result = tcapnc(strmod);

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
        file.write_all("RÍO COTUHÉ, CO:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("Río Cotuhé, CO:");
        println!();
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

    let lowercase = original_text.to_lowercase();
    let strsym = tcasym(&lowercase);

    let strmod = &strsym
        .replace("b", "b̥")
        .replace("cue", "kue")
        .replace("cui", "kui")
        .replace("c", "k")
        .replace("tkh", "ch")
        .replace("kh", "ch")
        .replace("ch", "ʧ")
        .replace("ng", "ŋ")
        .replace("g", "ɡ")
        .replace("n", "n̪")
        .replace("nh", "ñ")
        .replace("ñ", "ɲ")
        .replace("que", "ke")
        .replace("qui", "ki")
        .replace("q", "k")
        .replace("r", "ɾ")
        .replace("t", "t̪")
        .replace("y", "d͡ʒ")
        .replace("j", "d͡ʒ")
        .replace("f", "ɸ")
        .replace("z", "s")
        .replace("ç", "s")
        .replace("s", "s̺")
        .replace("v", "ɰ")
        .replace("ɡ", "ɡ̥")
        .replace("ã", "ã̹")
        .replace("ã", "ã̹")
        .replace("õ", "ɔ̃")
        .replace("õ", "ɔ̃")
        .replace("ẽ", "ɛ̃")
        .replace("ẽ", "ɛ̃")
        .replace("ũ", "ʊ̃")
        .replace("ũ", "ʊ̃")
        .replace("ü̃", "ɨ̃")
        .replace("a̰", "ʌ̰")
        .replace("a̱", "ʌ̰")
        .replace("a̠", "ʌ̰")
        .replace("o̰", "ɔ̰")
        .replace("o̱", "ɔ̰")
        .replace("o̠", "ɔ̰")
        .replace("ḛ", "ɛ̰")
        .replace("e̱", "ɛ̰")
        .replace("e̠", "ɛ̰")
        .replace("i̱", "ḭ")
        .replace("i̠", "ḭ")
        .replace("ṵ", "ʊ̰")
        .replace("u̱", "ʊ̰")
        .replace("u̠", "ʊ̰")
        .replace("ṵ̈", "ɨ̰")
        .replace("ü̱", "ɨ̰")
        .replace("ü̠", "ɨ̰")
        .replace("á", "a")
        .replace("é", "e")
        .replace("í", "i")
        .replace("ó", "o")
        .replace("ú", "u")
        .replace("ǘ", "ü")
        .replace("à", "a")
        .replace("è", "e")
        .replace("ì", "i")
        .replace("ò", "o")
        .replace("ù", "u")
        .replace("ǜ", "ü")
        .replace("w", "ɰ")
        .replace("d͡ʒae", "d͡ʒɛi")
        .replace("a", "a̹")
        .replace("e", "ɛ")
        .replace("o", "ɔ")
        .replace("u", "ʊ")
        .replace("ü", "ɨ̞")
        .replace("a̹ã̹", "ã̹ː")
        .replace("a̹a̹", "a̹ː");

    let result = tcapnc(strmod);

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
        file.write_all("CUSHILLOCOCHA, PE:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("Cushillococha, PE:");
        println!();
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

    let result = original_text
        .replace("y", "j")
        .replace("Y", "J")
        .replace("ch", "tch")
        .replace("Ch", "Tch")
        .replace("CH", "TCH")
        .replace("ttch", "tch")
        .replace("Ttch", "Tch")
        .replace("TTCH", "TCH")
        .replace("ñ", "nh")
        .replace("Ñ", "NH")
        .replace("ka", "ca")
        .replace("Ka", "Ca")
        .replace("KA", "CA")
        .replace("ka̱", "ca")
        .replace("Ka̱", "Ca")
        .replace("KA̱", "CA")
        .replace("ká", "cá")
        .replace("Ká", "Cá")
        .replace("KÁ", "CÁ")
        .replace("ko", "co")
        .replace("Ko", "Co")
        .replace("KO", "CO")
        .replace("kõ", "cõ")
        .replace("Kõ", "Cõ")
        .replace("KÕ", "CÕ")
        .replace("kó", "có")
        .replace("Kó", "Có")
        .replace("KÓ", "CÓ")
        .replace("ku", "cu")
        .replace("Ku", "Cu")
        .replace("KU", "CU")
        .replace("ku̱", "cu’")
        .replace("Ku̱", "Cu’")
        .replace("KU̱", "CU’")
        .replace("kú", "cú")
        .replace("Kú", "Cú")
        .replace("KÚ", "CÚ")
        .replace("kü", "cü")
        .replace("Kü", "Cü")
        .replace("KÜ", "CÜ")
        .replace("kü’", "cü’")
        .replace("Kü’", "Cü’")
        .replace("KÜ’", "CÜ’")
        .replace("cua", "qua")
        .replace("Cua", "Qua")
        .replace("CUA", "QUA")
        .replace("cua̱", "qua’")
        .replace("Cua̱", "Qua’")
        .replace("CUA̱", "QUA’")
        .replace("kua", "qua")
        .replace("Kua", "Qua")
        .replace("KUA", "QUA")
        .replace("kua̱", "qua’")
        .replace("Kua̱", "Qua’")
        .replace("KUA̱", "QUA’")
        .replace("ki", "qui")
        .replace("Ki", "Qui")
        .replace("KI", "QUI")
        .replace("ke", "que")
        .replace("Ke", "Que")
        .replace("KE", "QUE")
        .replace("ax", "a’")
        .replace("ãx", "ã’")
        .replace("a̱x", "a’")
        .replace("ã̱x", "ã’")
        .replace("áx", "á’")
        .replace("ex", "e’")
        .replace("ẽx", "ẽ’")
        .replace("e̱x", "e’")
        .replace("ẽ̱x", "ẽ’")
        .replace("éx", "é’")
        .replace("ix", "i’")
        .replace("ĩx", "ĩ’")
        .replace("íx", "í’")
        .replace("ox", "o’")
        .replace("õx", "õ’")
        .replace("o̱x", "o’")
        .replace("õ̱x", "õ’")
        .replace("óx", "ó’")
        .replace("ux", "u’")
        .replace("ũx", "ũ’")
        .replace("u̱x", "u’")
        .replace("úx", "ú’")
        .replace("üx", "ü’")
        .replace("ü̃x", "ü̃’")
        .replace("ü̱x", "ü’")
        .replace("ü̱̃x", "ü̃’")
        .replace("ǘx", "ǘ’")
        .replace("AX", "A’")
        .replace("ÃX", "Ã’")
        .replace("A̱X", "A’")
        .replace("Ã̱X", "Ã’")
        .replace("ÁX", "Á’")
        .replace("EX", "E’")
        .replace("ẼX", "Ẽ’")
        .replace("E̱X", "E’")
        .replace("Ẽ̱X", "Ẽ’")
        .replace("ÉX", "É’")
        .replace("IX", "I’")
        .replace("ĨX", "Ĩ’")
        .replace("I̱X", "I’")
        .replace("Ĩ̱X", "Ĩ’")
        .replace("ÍX", "Í’")
        .replace("OX", "O’")
        .replace("ÕX", "Õ’")
        .replace("O̱X", "O’")
        .replace("Õ̱X", "Õ’")
        .replace("ÓX", "Ó’")
        .replace("UX", "U’")
        .replace("ŨX", "Ũ’")
        .replace("U̱X", "U’")
        .replace("Ũ̱X", "Ũ’")
        .replace("ÚX", "Ú’")
        .replace("ÜX", "Ü’")
        .replace("Ü̃X", "Ü̃’")
        .replace("Ü̱X", "Ü’")
        .replace("Ü̱̃X", "Ü̃’")
        .replace("ǗX", "Ǘ’")
        .replace("Ax", "A’")
        .replace("Ãx", "Ã’")
        .replace("A̱x", "A’")
        .replace("Ã̱x", "Ã’")
        .replace("Áx", "Á’")
        .replace("Ex", "E’")
        .replace("Ẽx", "Ẽ’")
        .replace("E̱x", "E’")
        .replace("Ẽ̱x", "Ẽ’")
        .replace("Éx", "É’")
        .replace("Ix", "I’")
        .replace("Ĩx", "Ĩ’")
        .replace("Íx", "Í’")
        .replace("Ox", "O’")
        .replace("Õx", "Õ’")
        .replace("O̱x", "O’")
        .replace("Õ̱x", "Õ’")
        .replace("Óx", "Ó’")
        .replace("Ux", "U’")
        .replace("Ũx", "Ũ’")
        .replace("U̱x", "U’")
        .replace("Ũ̱x", "Ũ’")
        .replace("Úx", "Ú’")
        .replace("Üx", "Ü’")
        .replace("Ü̃x", "Ü̃’")
        .replace("Ü̱x", "Ü’")
        .replace("Ü̱̃x", "Ü̃’")
        .replace("Ǘx", "Ǘ’");

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
        file.write_all("BRAZIL:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("Brazil:");
        println!();
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

    let result = original_text
        .replace("j", "y")
        .replace("J", "Y")
        .replace("tch", "ch")
        .replace("Tch", "Ch")
        .replace("TCH", "CH")
        .replace("nh", "ñ")
        .replace("Nh", "Ñ")
        .replace("NH", "Ñ")
        .replace("c", "k")
        .replace("C", "K")
        .replace("kh", "ch")
        .replace("Kh", "Ch")
        .replace("KH", "CH")
        .replace("qui", "ki")
        .replace("Qui", "Ki")
        .replace("QUI", "KI")
        .replace("que", "ke")
        .replace("Que", "Ke")
        .replace("QUE", "KE")
        .replace("q", "k")
        .replace("Q", "K")
        .replace("x", "")
        .replace("X", "")
        .replace("'", "")
        .replace('’', "");

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
        file.write_all("COLOMBIA:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("Colombia:");
        println!();
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

    let result = original_text
        .replace("j", "y")
        .replace("J", "Y")
        .replace("tch", "ch")
        .replace("Tch", "Ch")
        .replace("TCH", "CH")
        .replace("nh", "ñ")
        .replace("Nh", "Ñ")
        .replace("NH", "Ñ")
        .replace("q", "c")
        .replace("Q", "C")
        .replace("à", "a")
        .replace("ò", "o")
        .replace("è", "e")
        .replace("ì", "i")
        .replace("ù", "u")
        .replace("ǜ", "ü")
        .replace("À", "A")
        .replace("Ò", "O")
        .replace("È", "E")
        .replace("Ì", "I")
        .replace("Ù", "U")
        .replace("Ǜ", "Ü")
        .replace("ki", "qui")
        .replace("Ki", "Qui")
        .replace("KI", "QUI")
        .replace("ke", "que")
        .replace("Ke", "Que")
        .replace("KE", "QUE")
        .replace("kí", "quí")
        .replace("Kí", "Quí")
        .replace("KÍ", "QUÍ")
        .replace("ké", "qué")
        .replace("Ké", "Qué")
        .replace("KÉ", "QUÉ")
        .replace("k", "c")
        .replace("K", "C")
        .replace("a'", "ax")
        .replace("ã'", "ãx")
        .replace("a̱'", "a̱x")
        .replace("ã̱'", "ã̱x")
        .replace("á'", "áx")
        .replace("e'", "ex")
        .replace("ẽ'", "ẽx")
        .replace("e̱'", "e̱x")
        .replace("ẽ̱'", "ẽ̱x")
        .replace("é'", "éx")
        .replace("i'", "ix")
        .replace("ĩ'", "ĩx")
        .replace("i̱'", "i̱x")
        .replace("ĩ̱'", "ĩ̱x")
        .replace("í'", "íx")
        .replace("o'", "ox")
        .replace("õ'", "õx")
        .replace("o̱'", "o̱x")
        .replace("õ̱'", "õ̱x")
        .replace("ó'", "óx")
        .replace("u'", "ux")
        .replace("ũ'", "ũx")
        .replace("u̱'", "u̱x")
        .replace("ũ̱'", "ũ̱x")
        .replace("ú'", "úx")
        .replace("ü'", "üx")
        .replace("ü̃'", "ü̃x")
        .replace("ü̱'", "ü̱x")
        .replace("ü̱̃'", "ü̱̃x")
        .replace("ǘ'", "ǘx")
        .replace("'", "x")
        .replace("a’", "ax")
        .replace("ã’", "ãx")
        .replace("a̱’", "a̱x")
        .replace("ã̱’", "ã̱x")
        .replace("á’", "áx")
        .replace("e’", "ex")
        .replace("ẽ’", "ẽx")
        .replace("e̱’", "e̱x")
        .replace("ẽ̱’", "ẽ̱x")
        .replace("é’", "éx")
        .replace("i’", "ix")
        .replace("ĩ’", "ĩx")
        .replace("i̱’", "i̱x")
        .replace("ĩ̱’", "ĩ̱x")
        .replace("í’", "íx")
        .replace("o’", "ox")
        .replace("õ’", "õx")
        .replace("o̱’", "o̱x")
        .replace("õ̱’", "õ̱x")
        .replace("ó’", "óx")
        .replace("u’", "ux")
        .replace("ũ’", "ũx")
        .replace("u̱’", "u̱x")
        .replace("ũ̱’", "ũ̱x")
        .replace("ú’", "úx")
        .replace("ü’", "üx")
        .replace("ü̃’", "ü̃x")
        .replace("ü̱’", "ü̱x")
        .replace("ü̱̃’", "ü̱̃x")
        .replace("ǘ’", "ǘx")
        .replace('’', "x");

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
        file.write_all("PERU (ILV):".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("Peru (ILV):");
        println!();
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

    let result = original_text
        .replace("j", "y")
        .replace("J", "Y")
        .replace("tch", "ch")
        .replace("Tch", "Ch")
        .replace("TCH", "CH")
        .replace("nh", "ñ")
        .replace("Nh", "Ñ")
        .replace("NH", "Ñ")
        .replace("à", "a")
        .replace("ò", "o")
        .replace("è", "e")
        .replace("ì", "i")
        .replace("ù", "u")
        .replace("ǜ", "ü")
        .replace("À", "A")
        .replace("Ò", "O")
        .replace("È", "E")
        .replace("Ì", "I")
        .replace("Ù", "U")
        .replace("Ǜ", "Ü")
        .replace("á", "a")
        .replace("ó", "o")
        .replace("é", "e")
        .replace("í", "i")
        .replace("ú", "u")
        .replace("ǘ", "ü")
        .replace("Á", "A")
        .replace("É", "E")
        .replace("Í", "I")
        .replace("Ó", "O")
        .replace("Ú", "U")
        .replace("Ǘ", "Ü")
        .replace("c", "k")
        .replace("C", "K")
        .replace("kh", "ch")
        .replace("Kh", "Ch")
        .replace("KH", "CH")
        .replace("qui", "ki")
        .replace("Qui", "Ki")
        .replace("QUI", "KI")
        .replace("que", "ke")
        .replace("Que", "Ke")
        .replace("QUE", "KE")
        .replace("q", "k")
        .replace("Q", "K")
        .replace("a'", "ax")
        .replace("ã'", "ãx")
        .replace("a̱'", "ax")
        .replace("ã̱'", "ãx")
        .replace("e'", "ex")
        .replace("ẽ'", "ẽx")
        .replace("e̱'", "ex")
        .replace("ẽ̱'", "ẽx")
        .replace("i'", "ix")
        .replace("ĩ'", "ĩx")
        .replace("i̱'", "ix")
        .replace("ĩ̱'", "ĩx")
        .replace("o'", "ox")
        .replace("õ'", "õx")
        .replace("o̱'", "ox")
        .replace("õ̱'", "õx")
        .replace("u'", "ux")
        .replace("ũ'", "ũx")
        .replace("u̱'", "ux")
        .replace("ũ̱'", "ũx")
        .replace("ü'", "üx")
        .replace("ü̃'", "ü̃x")
        .replace("ü̱'", "üx")
        .replace("ü̱̃'", "ü̃x")
        .replace("A̱", "AX")
        .replace("Ã̱", "ÃX")
        .replace("E̱", "EX")
        .replace("Ẽ̱", "ẼX")
        .replace("I̱", "IX")
        .replace("Ĩ̱", "ĨX")
        .replace("O̱", "OX")
        .replace("Õ̱", "ÕX")
        .replace("U̱", "UX")
        .replace("Ũ̱", "ŨX")
        .replace("Ü̱", "ÜX")
        .replace("Ü̱̃", "Ü̃X")
        .replace("'", "x")
        .replace("a’", "ax")
        .replace("ã’", "ãx")
        .replace("a̱’", "ax")
        .replace("ã̱’", "ãx")
        .replace("e’", "ex")
        .replace("ẽ’", "ẽx")
        .replace("e̱’", "ex")
        .replace("ẽ̱’", "ẽx")
        .replace("i’", "ix")
        .replace("ĩ’", "ĩx")
        .replace("i̱’", "ix")
        .replace("ĩ̱’", "ĩx")
        .replace("o’", "ox")
        .replace("õ’", "õx")
        .replace("o̱’", "ox")
        .replace("õ̱’", "õx")
        .replace("u’", "ux")
        .replace("ũ’", "ũx")
        .replace("u̱’", "ux")
        .replace("ũ̱’", "ũx")
        .replace("ü’", "üx")
        .replace("ü̃’", "ü̃x")
        .replace("ü̱’", "üx")
        .replace("ü̱̃’", "ü̃x")
        .replace('’', "x");

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
        file.write_all("PERU (FORMABIAP):".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("Peru (FORMABIAP):");
        println!();
        print!("{}", yellow);
        println!("{}", result);
        print!("{}", reset);
    }
}
