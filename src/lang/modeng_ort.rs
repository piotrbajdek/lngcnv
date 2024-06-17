// LNGCNV VERSION 1.10.0 / MIT LICENSE / COPYRIGHT © 2022–2024 PIOTR BAJDEK

// MODULE MODENG_ORT

// CLIPPY LINTS

#![deny(clippy::no_effect_replace)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::too_many_lines, clippy::unicode_not_nfc)]
#![allow(clippy::string_lit_as_bytes)]

// IMPORTS

use std::fs::OpenOptions;
use std::io::Write;

// ENGLISH: ORTHOGRAPHY

pub fn ortuseng(original_text: &str, usefile: &str, outputfile: &str, reset: &str, red: &str, cyan: &str, yellow: &str) {
    let space = original_text.to_owned() + " ";
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
        print!("{yellow}");
        println!("{result}");
        print!("{reset}");
    }
}
