// LNGCNV VERSION 1.9.2 / MIT LICENSE / COPYRIGHT © 2022–2024 PIOTR BAJDEK

// MODULE MODPOL

// CLIPPY LINTS

#![deny(clippy::no_effect_replace)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::similar_names, clippy::too_many_lines, clippy::unicode_not_nfc)]
#![allow(clippy::string_lit_as_bytes)]

// IMPORTS

use std::fs::OpenOptions;
use std::io::Write;

// SIMPLIFY INTERPUNCTION

fn polpncbeg(abbrev: &str) -> String {
    let pncbeg = &abbrev.replace([';', '!', '?'], ".").replace(").", ".").replace("),", ",").replace([')', ':'], ",").replace("--", " – ").replace('(', "∣ .").replace(' ', "# #").replace('.', "#.#").replace(',', "#,#");
    pncbeg.to_string()
}

// REMOVE INTERPUNCTION

fn polpncend(strmod: &str) -> String {
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

// ABBREVIATIONS

fn polabbrev(lowercase: &str) -> String {
    let abbrev = &lowercase
        .replace(" cm ", " centymetrów ")
        .replace(" dr.", " doktora")
        .replace(" dr ", " doktor ")
        .replace(".dr ", ".doktor ")
        .replace(" dyr.", " dyrektor")
        .replace(".dyr.", ".dyrektor")
        .replace(" gen.", " generał")
        .replace(".gen.", ".generał")
        .replace(" gł.", " głównie")
        .replace(" godz.", " godzinie")
        .replace(" gr ", " groszy ")
        .replace(" gr.", " groszy.")
        .replace(" hab.", " habilitowany")
        .replace(" i in.", " i inne.")
        .replace(" inż.", " inżynier")
        .replace(".inż.", ".inżynier")
        .replace(" im.", " imienia")
        .replace(" itd.", " i tak dalej.")
        .replace(" itp.", " i tym podobne.")
        .replace(" jw.", " jak wyżej")
        .replace(" k.", " koło")
        .replace(" kg", " kilogramów")
        .replace(" med.", " medycyny")
        .replace("m.in.", "między innymi")
        .replace("mgr.", "magistra")
        .replace("mgr ", "magister ")
        .replace(" mld ", " miliardów ")
        .replace(" mln ", " milionów ")
        .replace(" n.", " nad")
        .replace("np.", "na przykład")
        .replace("n.p.m.", "nad poziomem morza")
        .replace("p.n.e.", "przed naszą erą")
        .replace("n.e.", "naszej ery")
        .replace("prof.", "profesor")
        .replace("p.s. ", "peʔes, ")
        .replace("str.", "stronie")
        .replace("tj.", "to jest")
        .replace(" tys.", " tysięcy")
        .replace("tzn.", "to znaczy")
        .replace("tzw.", "tak zwany")
        .replace("wg ", "według ")
        .replace(" ww.", " wyżej wymieniony")
        .replace(" zł ", " złotych ")
        .replace(" zł.", " złotych.");
    abbrev.to_string()
}

// FOREIGN WORDS

fn polpalesp(pncbeg: &str) -> String {
    let palesp = &pncbeg.replace("chopin", "szopen").replace("covid", "kowid").replace("lockdown", "lokdałn").replace("weekend", "łikend");
    palesp.to_string()
}

//   ++++++++++   ++++++++++   ++++++++++

// CZĘSTOCHOWA: IPA

pub fn polplczestochowa(original_text: &str, usefile: &str, outputfile: &str, reset: &str, red: &str, cyan: &str, yellow: &str) {
    let dotend = original_text.to_owned() + ".";
    let dotbeg = ".".to_owned() + &dotend;
    let lowercase = &dotbeg.to_lowercase();
    let abbrev = polabbrev(&lowercase);
    let pncbeg = polpncbeg(&abbrev);
    let palesp = polpalesp(&pncbeg);

    let strmod = &palesp
        .replace("dż", "ɖ͡ʐ")
        .replace("dź", "d͡ʑ")
        .replace("dz", "^d͡z^")
        .replace("od#", "od̥#")
        .replace("odt", "od̥t")
        .replace("odp", "od̥p")
        .replace("ód", "ód̥")
        .replace("ób", "ób̥")
        .replace("bc", "b̥c")
        .replace("bs", "b̥s")
        .replace("dc", "d̥c")
        .replace("wsk", "v̥sk")
        .replace("wt", "v̥t")
        .replace("dt", "d̥t")
        .replace("dk", "d̥k")
        .replace('t', "t̪")
        .replace("st̪rz", "ʂʈ͡ʂ")
        .replace("t̪rz", "ṯʃ")
        .replace("prz", "pʂ")
        .replace("t̪r", "ṯɾ̥")
        .replace("rt̪", "ɾ̥ṯ")
        .replace("t̪m", "ṯm̥")
        .replace("st̪k", "st̥k")
        .replace("izm", "iz̥m̥")
        .replace("#z#", "#z̥#")
        .replace("ans", "an̥s")
        .replace("sz", "ʃ")
        .replace("śln", "śl̥n")
        .replace("ślm", "śl̥m")
        .replace("śń", "śɲ̥")
        .replace('ń', "ɲ")
        .replace("ni", "ɲi")
        .replace("ɲi#", "ɲʲi#")
        .replace("ɲia", "ɲʲa")
        .replace("ɲią", "ɲʲą")
        .replace("ɲio", "ɲʲo")
        .replace("ɲie", "ɲʲe")
        .replace("ɲa", "ɲʲa")
        .replace("ɲu", "ɲʲu")
        .replace("cz", "ʈ͡ʂ")
        .replace("zn", "z̥n")
        .replace("ch", "x")
        .replace('h', "x")
        .replace("zx", "z̥x")
        .replace('ś', "ɕ")
        .replace("si#", "ɕi#")
        .replace("sia", "ɕa")
        .replace("sią", "ɕą")
        .replace("sio", "ɕo")
        .replace("sie", "ɕe")
        .replace("się", "ɕę")
        .replace("siu", "ɕu")
        .replace("si", "ɕi")
        .replace('s', "s̻")
        .replace("mc", "m̥c")
        .replace("nc", "n̥c")
        .replace("ci#", "t͡ɕi#")
        .replace("cia", "t͡ɕa")
        .replace("cią", "t͡ɕą")
        .replace("cio", "t͡ɕo")
        .replace("cie", "t͡ɕe")
        .replace("cię", "t͡ɕę")
        .replace("ciu", "t͡ɕu")
        .replace("ci", "t͡ɕi")
        .replace('c', "t͡s")
        .replace('ć', "t͡ɕ")
        .replace("xrz", "xʂ")
        .replace("rz", "ʒ")
        .replace('r', "ɾ")
        .replace('ż', "ʒ")
        .replace("ʒ#", "ʒ̥#")
        .replace("zi#", "ʑi#")
        .replace("zia", "ʑa")
        .replace("zią", "ʑą")
        .replace("zio", "ʑo")
        .replace("zie", "ʑe")
        .replace("zi", "ʑi")
        .replace('ź', "ʑ")
        .replace("ei#", "eʲi#")
        .replace("ia", "ʲa")
        .replace("ią", "ʲą")
        .replace("io", "ʲo")
        .replace("ie", "ʲe")
        .replace("ię", "ʲę")
        .replace("ii", "ʲi")
        .replace("iu", "ʲu")
        .replace("ng", "ŋɡ")
        .replace("nk", "ŋk")
        .replace('g', "ɡ")
        .replace("wk", "v̥k")
        .replace('w', "v")
        .replace("bł", "b̥w̥")
        .replace('ł', "w")
        .replace("m#", "ɱ#")
        .replace("n#", "ŋ#")
        .replace("mf", "ɱf")
        .replace("mv", "ɱv")
        .replace("ju", "jü")
        .replace("aʈ͡ʂ", "ɐʈ͡ʂ")
        .replace("ʈ͡ʂa", "ʈ͡ʂɐ")
        .replace("at͡s", "ɐt͡s")
        .replace("t͡sa", "t͡sɐ")
        .replace("aʃ", "ɐʃ")
        .replace("ʃa", "ʃɐ")
        .replace("aʂ", "ɐʂ")
        .replace("ʂa", "ʂɐ")
        .replace("s̻a", "s̻ɐ")
        .replace("as̻", "ɐs̻")
        .replace("va", "vɐ")
        .replace("av", "ɐv")
        .replace('a', "ä")
        .replace("eʈ͡ʂ", "ɛ̈ʈ͡ʂ")
        .replace("ʈ͡ʂe", "ʈ͡ʂɛ̈")
        .replace("et͡s", "ɛ̈t͡s")
        .replace("t͡se", "t͡sɛ̈")
        .replace("eʃ", "ɛ̈ʃ")
        .replace("ʃe", "ʃɛ̈")
        .replace("eʂ", "ɛ̈ʂ")
        .replace("ʂe", "ʂɛ̈")
        .replace("s̻e", "s̻ɛ̈")
        .replace("es̻", "ɛ̈s̻")
        .replace('e', "ɛ̝")
        .replace('ę', "ɛ̝̃")
        .replace("oʈ͡ʂ", "ɔ̈ʈ͡ʂ")
        .replace("ʈ͡ʂo", "ʈ͡ʂɔ̈")
        .replace("ot͡s", "ɔ̈t͡s")
        .replace("t͡so", "t͡sɔ̈")
        .replace("oʃ", "ɔ̈ʃ")
        .replace("ʃo", "ʃɔ̈")
        .replace("oʂ", "ɔ̈ʂ")
        .replace("ʂo", "ʂɔ̈")
        .replace("s̻o", "s̻ɔ̈")
        .replace("os̻", "ɔ̈s̻")
        .replace('o', "ɔ̝")
        .replace('ą', "ɔ̝̃")
        .replace('ó', "u")
        .replace("ü̞", "ü")
        .replace("yʈ͡ʂ", "ɘʈ͡ʂ")
        .replace("ʈ͡ʂy", "ʈ͡ʂɘ")
        .replace("yt͡s", "ɘt͡s")
        .replace("t͡sy", "t͡sɘ")
        .replace("yʃ", "ɘʃ")
        .replace("ʃy", "ʃɘ")
        .replace("yʂ", "ɘʂ")
        .replace("ʂy", "ʂɘ")
        .replace("s̻y", "s̻ɘ")
        .replace("ys̻", "ɘs̻")
        .replace('y', "ɘ̟")
        .replace("ɱi", "mʲi")
        .replace("ʲʲ", "ʲ")
        .replace('z', "z̪")
        .replace("d͡z̪", "d͡z")
        .replace("z̪̥", "z̥")
        .replace('^', "");

    let result = polpncend(strmod);

    if usefile == "new" {
        let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
        file.write_all("CZĘSTOCHOWA, MAŁOPOLSKA REGION:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "old" {
        let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
        file.write_all("CZĘSTOCHOWA, MAŁOPOLSKA REGION:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("{}", cyan.to_owned() + "Częstochowa" + reset + ", " + cyan + "Małopolska Region" + reset + ":");
        println!();
        print!("{yellow}");
        println!("{result}");
        print!("{reset}");
    }
}

//   ++++++++++   ++++++++++   ++++++++++

// TORUŃ: IPA

pub fn polpltorun(original_text: &str, usefile: &str, outputfile: &str, reset: &str, red: &str, cyan: &str, yellow: &str) {
    let dotend = original_text.to_owned() + ".";
    let dotbeg = ".".to_owned() + &dotend;
    let lowercase = &dotbeg.to_lowercase();
    let abbrev = polabbrev(&lowercase);
    let pncbeg = polpncbeg(&abbrev);
    let palesp = polpalesp(&pncbeg);

    let strmod = &palesp
        .replace("dż", "ɖ͡ʐ")
        .replace("dź", "d͡ʑ")
        .replace("dz", "^d͡z^")
        .replace("od#", "ot#")
        .replace("odt", "ott")
        .replace("odp", "otp")
        .replace("ód", "ód̥")
        .replace("ób", "óp")
        .replace("ów", "óɸ")
        .replace("bc", "pc")
        .replace("bs", "ps")
        .replace("dc", "d̥c")
        .replace("wt", "ɸt")
        .replace("dt", "tt")
        .replace("dk", "tk")
        .replace('t', "t̪")
        .replace("st̪rz", "st̪ʂ̬")
        .replace("t̪rz", "t̪ʂ")
        .replace("prz", "pʂ")
        .replace("wsk", "sk")
        .replace("st̪k", "sk")
        .replace("izm", "il͡ʒ̥m̥")
        .replace("#z#", "#l͡ʒ̥#")
        .replace("ans", "an̥s")
        .replace("sz", "ʂ")
        .replace("śln", "śl̥n")
        .replace("ślm", "śl̥m")
        .replace("śń", "śɲ̥")
        .replace('ń', "ɲ")
        .replace("ni", "ɲi")
        .replace("ɲi#", "ɲʲi#")
        .replace("ɲia", "ɲʲa")
        .replace("ɲią", "ɲʲą")
        .replace("ɲio", "ɲʲo")
        .replace("ɲie", "ɲʲe")
        .replace("ɲa", "ɲʲa")
        .replace("ɲu", "ɲʲu")
        .replace("cz", "ʈ͡ʂ̞")
        .replace("zn", "l͡ʒ̥n")
        .replace("ch", "χ")
        .replace('h', "χ")
        .replace("zχ", "sχ")
        .replace('ś', "ç")
        .replace("si#", "çi#")
        .replace("sia", "ça")
        .replace("sią", "çą")
        .replace("sio", "ço")
        .replace("sie", "çe")
        .replace("się", "çę")
        .replace("siu", "çu")
        .replace("si", "çi")
        .replace('s', "ɬ̻")
        .replace("mc", "m̥c")
        .replace("nc", "n̥c")
        .replace("ci#", "c͡çi#")
        .replace("cia", "c͡ça")
        .replace("cią", "c͡çą")
        .replace("cio", "c͡ço")
        .replace("cie", "c͡çe")
        .replace("cię", "c͡çę")
        .replace("ciu", "c͡çu")
        .replace("ci", "c͡çi")
        .replace('c', "t̪͡s̪")
        .replace("t̪͡s̪͡ç", "c͡ç")
        .replace('ć', "c͡ç")
        .replace("χrz", "χʂ")
        .replace("rz", "ʐ")
        .replace('r', "ɾ̪")
        .replace("ż#", "ʂ#")
        .replace('ż', "ʐ")
        .replace("zi#", "ʑi#")
        .replace("zia", "ʑa")
        .replace("zią", "ʑą")
        .replace("zio", "ʑo")
        .replace("zie", "ʑe")
        .replace("zi", "ʑi")
        .replace('ź', "ʑ")
        .replace("ei#", "eʲi#")
        .replace("ia", "ʲa")
        .replace("ią", "ʲą")
        .replace("io", "ʲo")
        .replace("ie", "ʲe")
        .replace("ię", "ʲę")
        .replace("ii", "ʲi")
        .replace("iu", "ʲu")
        .replace("ng", "ŋɡ")
        .replace("nk", "ŋk")
        .replace('g', "ɡ")
        .replace("wk", "v̥k")
        .replace('w', "v")
        .replace("dłu", "dw̥u")
        .replace("ɡłu", "ɡw̥u")
        .replace("ɡłó", "ɡw̥u")
        .replace('ł', "w")
        .replace("m#", "ɱ#")
        .replace("n#", "ŋ#")
        .replace("mf", "ɱf")
        .replace("mv", "ɱv")
        .replace('a', "ɐ")
        .replace('e', "ɛ")
        .replace("ę#", "ɛ#")
        .replace("ęb", "ɛɱb")
        .replace("ęp", "ɛɱp")
        .replace('ę', "ɛŋ")
        .replace('o', "ɔ")
        .replace("ą#", "ɔɱ#")
        .replace('ą', "ɔŋ")
        .replace('ó', "u")
        .replace('u', "u̞")
        .replace('y', "ɘ̟")
        .replace("ɱi", "mʲi")
        .replace("ʲʲ", "ʲ")
        .replace('i', "i̞")
        .replace("pɾ̪", "p̪ɾ̪")
        .replace("bɾ̪", "b̪ɾ̪")
        .replace("tɾ̪", "t̪ɾ̪")
        .replace("dɾ̪", "d̪ɾ̪")
        .replace('z', "z̻")
        .replace("d͡z̻", "d͡z")
        .replace('^', "");

    let result = polpncend(strmod);

    if usefile == "new" {
        let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
        file.write_all("TORUŃ, WIELKOPOLSKA REGION:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "old" {
        let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
        file.write_all("TORUŃ, WIELKOPOLSKA REGION:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("{}", cyan.to_owned() + "Toruń" + reset + ", " + cyan + "Wielkopolska Region" + reset + ":");
        println!();
        print!("{yellow}");
        println!("{result}");
        print!("{reset}");
    }
}

//   ++++++++++   ++++++++++   ++++++++++

// WARSZAWA: IPA

pub fn polplwarszawa(original_text: &str, usefile: &str, outputfile: &str, reset: &str, red: &str, cyan: &str, yellow: &str) {
    let dotend = original_text.to_owned() + ".";
    let dotbeg = ".".to_owned() + &dotend;
    let lowercase = &dotbeg.to_lowercase();
    let abbrev = polabbrev(&lowercase);
    let pncbeg = polpncbeg(&abbrev);
    let palesp = polpalesp(&pncbeg);

    let strmod = &palesp
        .replace("ąc", "ɒŋc")
        .replace("ąć", "ɒɲć")
        .replace("ąb", "ɒɱb")
        .replace("ąp", "ɒɱp")
        .replace("ąd", "ɒŋd")
        .replace("ąt", "ɒŋt")
        .replace("ąs", "ɒŋs")
        .replace("ąś", "ɒŋś")
        .replace("ąż", "ɒɲż")
        .replace("ąk", "ɒŋk")
        .replace("ęc", "ɶɲc")
        .replace("ęć", "ɶɲć")
        .replace("ęb", "ɶɱb")
        .replace("ęp", "ɶɱp")
        .replace("ęd", "ɶŋd")
        .replace("ęt", "ɶŋt")
        .replace("ęk", "ɶɲk")
        .replace("ęs", "ɶŋs")
        .replace("ęś", "ɶŋś")
        .replace("ęż", "ɶɲż")
        .replace("dż", "ɖ͡ʐ")
        .replace("dź", "d͡ʑ")
        .replace("dz", "^d͡z^")
        .replace("od#", "od̥#")
        .replace("odt", "od̥t")
        .replace("odp", "od̥p")
        .replace("ód", "ód̥")
        .replace("ób", "ób̥")
        .replace("bc", "b̥c")
        .replace("bs", "b̥s")
        .replace("dc", "d̥c")
        .replace("wsk", "v̥sk")
        .replace("wt", "v̥t")
        .replace("dt", "d̥t")
        .replace("dk", "d̥k")
        .replace('t', "t̪")
        .replace("t̪rz", "t̪ʂ̬")
        .replace("prz", "pʂ̬")
        .replace("st̪k", "st̥k")
        .replace("izm", "il͡ʒ̥m̥")
        .replace("#z#", "#l͡ʒ̥#")
        .replace("ans", "an̥s")
        .replace("sz", "ʂ")
        .replace("śln", "śl̥n")
        .replace("ślm", "śl̥m")
        .replace("śń", "śɲ̥")
        .replace('ń', "ɲ")
        .replace("ni", "ɲi")
        .replace("ɲi#", "ɲʲi#")
        .replace("ɲia", "ɲʲa")
        .replace("ɲią", "ɲʲą")
        .replace("ɲio", "ɲʲo")
        .replace("ɲie", "ɲʲe")
        .replace("ɲa", "ɲʲa")
        .replace("ɲu", "ɲʲu")
        .replace("cz", "ʈ͡ʂ̞")
        .replace("zn", "l͡ʒ̥n")
        .replace("ch", "x")
        .replace('h', "x")
        .replace("zx", "l͡ʒ̥x")
        .replace('ś', "ç")
        .replace("si#", "çi#")
        .replace("sia", "ça")
        .replace("sią", "çą")
        .replace("sio", "ço")
        .replace("sie", "çe")
        .replace("się", "çę")
        .replace("siu", "çu")
        .replace("si", "çi")
        .replace('s', "ɬ̻")
        .replace("mc", "m̥c")
        .replace('n', "n̪")
        .replace("ci#", "c͡çi#")
        .replace("cia", "c͡ça")
        .replace("cią", "c͡çą")
        .replace("cio", "c͡ço")
        .replace("cie", "c͡çe")
        .replace("cię", "c͡çę")
        .replace("ciu", "c͡çu")
        .replace("ci", "c͡çi")
        .replace('c', "t̪͡s̪")
        .replace("t̪͡s̪͡ç", "c͡ç")
        .replace('ć', "c͡ç")
        .replace("xrz", "xʂ")
        .replace("rz", "ʐ")
        .replace('r', "ɾ")
        .replace("ż#", "ʂ#")
        .replace("ʂ#", "ʂ̬#")
        .replace('ż', "ʐ")
        .replace("zi#", "ʑi#")
        .replace("zia", "ʑa")
        .replace("zią", "ʑą")
        .replace("zio", "ʑo")
        .replace("zie", "ʑe")
        .replace("zi", "ʑi")
        .replace('ź', "ʑ")
        .replace("ei#", "eʲi#")
        .replace("ia", "ʲa")
        .replace("ią", "ʲą")
        .replace("io", "ʲo")
        .replace("ie", "ʲe")
        .replace("ię", "ʲę")
        .replace("ii", "ʲi")
        .replace("iu", "ʲu")
        .replace('g', "ɡ")
        .replace("wk", "v̥k")
        .replace('w', "v")
        .replace("dłu", "dw̥u")
        .replace("ɡłu", "ɡw̥u")
        .replace("ɡłó", "ɡw̥u")
        .replace('ł', "w")
        .replace('l', "ɫ")
        .replace("#ɫ", "#l")
        .replace("m# #f", "ɱ# #f")
        .replace("m# #v", "ɱ# #v")
        .replace("mf", "ɱf")
        .replace("mv", "ɱv")
        .replace('a', "æ̞")
        .replace('e', "ɶ")
        .replace("ę#", "ɶ#")
        .replace('ę', "ɶw̥")
        .replace('o', "ɒ")
        .replace('ą', "ɒw̥")
        .replace('ó', "u")
        .replace('u', "ʊ")
        .replace('y', "ɘ")
        .replace("ɱi", "mʲi")
        .replace("ʲʲ", "ʲ")
        .replace('i', "ɪ")
        .replace('z', "z̻")
        .replace("d͡z̻", "d͡z")
        .replace('^', "");

    let result = polpncend(strmod);

    if usefile == "new" {
        let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
        file.write_all("WARSZAWA, MAZOWSZE REGION:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "old" {
        let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
        file.write_all("WARSZAWA, MAZOWSZE REGION:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("{}", cyan.to_owned() + "Warszawa" + reset + ", " + cyan + "Mazowsze Region" + reset + ":");
        println!();
        print!("{yellow}");
        println!("{result}");
        print!("{reset}");
    }
}
