// LNGCNV VERSION 1.6.0-BETA.10 / MIT LICENSE © 2022 PIOTR BAJDEK

// MODULE LANG

// CLIPPY LINTS

#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::needless_borrow)] // &original_text must be borrowed in fn list()
#![allow(clippy::cognitive_complexity, clippy::missing_panics_doc, clippy::too_many_lines)]

// IMPORTS

use std::env;
use std::fs;
use std::io;
use std::path::Path;

pub mod modeng;
pub mod modlat;
pub mod modpol;
pub mod modque;
pub mod modspa;
pub mod modtca;

// LANGLIST

pub fn list() {
    let reset = "\x1b[0m";
    let red = "\x1b[31m";
    let cyan = "\x1b[36m";

    // COLLECT ARGUMENTS FOR CONVERSIONS

    let args: Vec<String> = env::args().collect();

    let input1 = args.get(1).expect(&(red.to_owned() + "Invalid or missing arguments! See: --help" + reset));
    let input2 = args.get(2).expect(&(red.to_owned() + "Invalid or missing arguments! See: --help" + reset));

    //   ++++++++++   ++++++++++   ++++++++++

    // ENGLISH: IPA

    if input1 == "--ipa" && input2 == "--eng" || input1 == "--eng" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: ENGLISH IPA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: ENGLISH IPA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modeng::engaucanberra(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modeng::engaucanberra(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: ENGLISH IPA

                modeng::engaucanberra(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: ENGLISH IPA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modeng::engaucanberra(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // ENGLISH: ORTHOGRAPHY

    if input1 == "--ort" && input2 == "--eng" || input1 == "--eng" && input2 == "--ort" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: ENGLISH ORTHOGRAPHY
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: ENGLISH ORTHOGRAPHY
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modeng::ortuseng(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modeng::ortuseng(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: ENGLISH ORTHOGRAPHY

                modeng::ortuseng(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: ENGLISH ORTHOGRAPHY

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modeng::ortuseng(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // LATIN: IPA

    if input1 == "--ipa" && input2 == "--lat" || input1 == "--lat" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: LATIN IPA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: LATIN IPA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modlat::ipalat(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modlat::ipalat(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: LATIN IPA

                modlat::ipalat(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: LATIN IPA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modlat::ipalat(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // LATIN: ORTHOGRAPHY

    if input1 == "--ort" && input2 == "--lat" || input1 == "--lat" && input2 == "--ort" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: LATIN ORTHOGRAPHY
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: LATIN ORTHOGRAPHY
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modlat::ortlat(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modlat::ortlat(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: LATIN ORTHOGRAPHY

                modlat::ortlat(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: LATIN ORTHOGRAPHY

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modlat::ortlat(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // POLISH: IPA

    if input1 == "--ipa" && input2 == "--pol" || input1 == "--pol" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: POLISH IPA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: POLISH IPA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modpol::polplczestochowa(&original_text, usefile, outputfile);
                        let usefile = "old";
                        modpol::polpltorun(&original_text, usefile, outputfile);
                        modpol::polplwarszawa(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modpol::polplczestochowa(&original_text, usefile, outputfile);
                        modpol::polpltorun(&original_text, usefile, outputfile);
                        modpol::polplwarszawa(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: POLISH IPA

                modpol::polplczestochowa(&original_text, usefile, outputfile);
                let usefile = "old";
                modpol::polpltorun(&original_text, usefile, outputfile);
                modpol::polplwarszawa(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: POLISH IPA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modpol::polplczestochowa(&original_text, usefile, outputfile);
        modpol::polpltorun(&original_text, usefile, outputfile);
        modpol::polplwarszawa(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // POLISH: IPA --pol.pl-czestochowa

    if input1 == "--ipa" && input2 == "--pol.pl-czestochowa" || input1 == "--pol.pl-czestochowa" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: CZĘSTOCHOWA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: CZĘSTOCHOWA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modpol::polplczestochowa(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modpol::polplczestochowa(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: CZĘSTOCHOWA

                modpol::polplczestochowa(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: CZĘSTOCHOWA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modpol::polplczestochowa(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // POLISH: IPA --pol.pl-torun

    if input1 == "--ipa" && input2 == "--pol.pl-torun" || input1 == "--pol.pl-torun" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: TORUŃ
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: TORUŃ
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modpol::polpltorun(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modpol::polpltorun(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: TORUŃ

                modpol::polpltorun(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: TORUŃ

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modpol::polpltorun(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // POLISH: IPA --pol.pl-warszawa

    if input1 == "--ipa" && input2 == "--pol.pl-warszawa" || input1 == "--pol.pl-warszawa" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: WARSZAWA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: WARSZAWA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modpol::polplwarszawa(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modpol::polplwarszawa(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: WARSZAWA

                modpol::polplwarszawa(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: WARSZAWA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modpol::polplwarszawa(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // AYACUCHO QUECHUA: IPA

    if input1 == "--ipa" && input2 == "--que" || input1 == "--que" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: AYACUCHO QUECHUA IPA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: AYACUCHO QUECHUA IPA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modque::ipaque(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modque::ipaque(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: AYACUCHO QUECHUA IPA

                modque::ipaque(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: AYACUCHO QUECHUA IPA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modque::ipaque(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // AYACUCHO QUECHUA: DIALECT

    if input1 == "--lct" && input2 == "--que" || input1 == "--que" && input2 == "--lct" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: AYACUCHO QUECHUA DIALECT
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: AYACUCHO QUECHUA DIALECT
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modque::quelct(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modque::quelct(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: AYACUCHO QUECHUA DIALECT

                modque::quelct(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: AYACUCHO QUECHUA DIALECT

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modque::quelct(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // QUECHUA: ORTHOGRAPHY

    if input1 == "--ort" && input2 == "--que" || input1 == "--que" && input2 == "--ort" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: QUECHUA ORTHOGRAPHY
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: QUECHUA ORTHOGRAPHY
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modque::ortquetri(&original_text, usefile, outputfile);
                        let usefile = "old";
                        modque::ortquepen(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modque::ortquetri(&original_text, usefile, outputfile);
                        modque::ortquepen(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: QUECHUA ORTHOGRAPHY

                modque::ortquetri(&original_text, usefile, outputfile);
                let usefile = "old";
                modque::ortquepen(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: QUECHUA ORTHOGRAPHY

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modque::ortquetri(&original_text, usefile, outputfile);
        modque::ortquepen(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SPANISH: IPA

    if input1 == "--ipa" && input2 == "--spa" || input1 == "--spa" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: SPANISH IPA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: SPANISH IPA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacobogota(&original_text, usefile, outputfile);
                        let usefile = "old";
                        modspa::spacomedellin(&original_text, usefile, outputfile);
                        modspa::spaescadiz(&original_text, usefile, outputfile);
                        modspa::spaesmadrid(&original_text, usefile, outputfile);
                        modspa::spamxciudaddemexico(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacobogota(&original_text, usefile, outputfile);
                        modspa::spacomedellin(&original_text, usefile, outputfile);
                        modspa::spaescadiz(&original_text, usefile, outputfile);
                        modspa::spaesmadrid(&original_text, usefile, outputfile);
                        modspa::spamxciudaddemexico(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: SPANISH IPA

                modspa::spacobogota(&original_text, usefile, outputfile);
                let usefile = "old";
                modspa::spacomedellin(&original_text, usefile, outputfile);
                modspa::spaescadiz(&original_text, usefile, outputfile);
                modspa::spaesmadrid(&original_text, usefile, outputfile);
                modspa::spamxciudaddemexico(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: SPANISH IPA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modspa::spacobogota(&original_text, usefile, outputfile);
        modspa::spacomedellin(&original_text, usefile, outputfile);
        modspa::spaescadiz(&original_text, usefile, outputfile);
        modspa::spaesmadrid(&original_text, usefile, outputfile);
        modspa::spamxciudaddemexico(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SPANISH: IPA --spa.co-bogota

    if input1 == "--ipa" && input2 == "--spa.co-bogota" || input1 == "--spa.co-bogota" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: BOGOTÁ
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: BOGOTÁ
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacobogota(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacobogota(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: BOGOTÁ

                modspa::spacobogota(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: BOGOTÁ

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modspa::spacobogota(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SPANISH: IPA --spa.co-medellin

    if input1 == "--ipa" && input2 == "--spa.co-medellin" || input1 == "--spa.co-medellin" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: MEDELLÍN
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: MEDELLÍN
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacomedellin(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacomedellin(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: MEDELLÍN

                modspa::spacomedellin(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: MEDELLÍN
        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modspa::spacomedellin(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SPANISH: IPA --spa.es-cadiz

    if input1 == "--ipa" && input2 == "--spa.es-cadiz" || input1 == "--spa.es-cadiz" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: CÁDIZ
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: CÁDIZ
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spaescadiz(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spaescadiz(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: CÁDIZ

                modspa::spaescadiz(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: CÁDIZ

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modspa::spaescadiz(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SPANISH: IPA --spa.es-madrid

    if input1 == "--ipa" && input2 == "--spa.es-madrid" || input1 == "--spa.es-madrid" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: MADRID
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: MADRID
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spaesmadrid(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spaesmadrid(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: MADRID

                modspa::spaesmadrid(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: MADRID

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modspa::spaesmadrid(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SPANISH: IPA --spa.mx-ciudad_de_mexico

    if input1 == "--ipa" && input2 == "--spa.mx-ciudad_de_mexico" || input1 == "--spa.mx-ciudad_de_mexico" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: CIUDAD DE MÉXICO
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: CIUDAD DE MÉXICO
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spamxciudaddemexico(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spamxciudaddemexico(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: CIUDAD DE MÉXICO

                modspa::spamxciudaddemexico(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: CIUDAD DE MÉXICO

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modspa::spamxciudaddemexico(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // TIKUNA: IPA

    if input1 == "--ipa" && input2 == "--tca" || input1 == "--tca" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: TIKUNA IPA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: TIKUNA IPA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modtca::tcacoriocotuhe(&original_text, usefile, outputfile);
                        let usefile = "old";
                        modtca::tcapecushillococha(&original_text, usefile, outputfile);
                        modtca::tcaconazareth(&original_text, usefile, outputfile);
                        modtca::tcabrumariacu(&original_text, usefile, outputfile);
                        modtca::tcabrvilabetania(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modtca::tcacoriocotuhe(&original_text, usefile, outputfile);
                        modtca::tcapecushillococha(&original_text, usefile, outputfile);
                        modtca::tcaconazareth(&original_text, usefile, outputfile);
                        modtca::tcabrumariacu(&original_text, usefile, outputfile);
                        modtca::tcabrvilabetania(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: TIKUNA IPA

                modtca::tcacoriocotuhe(&original_text, usefile, outputfile);
                let usefile = "old";
                modtca::tcapecushillococha(&original_text, usefile, outputfile);
                modtca::tcaconazareth(&original_text, usefile, outputfile);
                modtca::tcabrumariacu(&original_text, usefile, outputfile);
                modtca::tcabrvilabetania(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: TIKUNA IPA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modtca::tcacoriocotuhe(&original_text, usefile, outputfile);
        modtca::tcapecushillococha(&original_text, usefile, outputfile);
        modtca::tcaconazareth(&original_text, usefile, outputfile);
        modtca::tcabrumariacu(&original_text, usefile, outputfile);
        modtca::tcabrvilabetania(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // TIKUNA: IPA --tca.br-umariacu

    if input1 == "--ipa" && input2 == "--tca.br-umariacu" || input1 == "--tca.br-umariacu" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: UMARIAÇU
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: UMARIAÇU
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modtca::tcabrumariacu(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modtca::tcabrumariacu(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: UMARIAÇU

                modtca::tcabrumariacu(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: UMARIAÇU

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modtca::tcabrumariacu(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // TIKUNA: IPA --tca.br-vila_betania

    if input1 == "--ipa" && input2 == "--tca.br-vila_betania" || input1 == "--tca.br-vila_betania" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: VILA BETÂNIA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: VILA BETÂNIA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modtca::tcabrvilabetania(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modtca::tcabrvilabetania(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: VILA BETÂNIA

                modtca::tcabrvilabetania(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: VILA BETÂNIA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modtca::tcabrvilabetania(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // TIKUNA: IPA --tca.co-nazareth

    if input1 == "--ipa" && input2 == "--tca.co-nazareth" || input1 == "--tca.co-nazareth" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: NAZARETH
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: NAZARETH
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modtca::tcaconazareth(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modtca::tcaconazareth(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: NAZARETH

                modtca::tcaconazareth(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: NAZARETH

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modtca::tcaconazareth(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // TIKUNA: IPA --tca.co-rio_cotuhe

    if input1 == "--ipa" && input2 == "--tca.co-rio_cotuhe" || input1 == "--tca.co-rio_cotuhe" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: RIO COTUHÉ
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: RIO COTUHÉ
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modtca::tcacoriocotuhe(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modtca::tcacoriocotuhe(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: RIO COTUHÉ

                modtca::tcacoriocotuhe(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: RIO COTUHÉ

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modtca::tcacoriocotuhe(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // TIKUNA: IPA --tca.pe-cushillococha

    if input1 == "--ipa" && input2 == "--tca.pe-cushillococha" || input1 == "--tca.pe-cushillococha" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: CUSHILLOCOCHA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: CUSHILLOCOCHA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modtca::tcapecushillococha(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modtca::tcapecushillococha(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: CUSHILLOCOCHA

                modtca::tcapecushillococha(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: CUSHILLOCOCHA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modtca::tcapecushillococha(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // TIKUNA: ORTHOGRAPHY

    if input1 == "--ort" && input2 == "--tca" || input1 == "--tca" && input2 == "--ort" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
        // FROM A FILE: TIKUNA ORTHOGRAPHY
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));
            let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: TIKUNA ORTHOGRAPHY
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{}", reset);
                    let option: &str = answer.trim();

                    if option == "o" {
                        modtca::orttcabr(&original_text, usefile, outputfile);
                        let usefile = "old";
                        modtca::orttcaco(&original_text, usefile, outputfile);
                        modtca::orttcapeilv(&original_text, usefile, outputfile);
                        modtca::orttcapeformabiap(&original_text, usefile, outputfile);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modtca::orttcabr(&original_text, usefile, outputfile);
                        modtca::orttcaco(&original_text, usefile, outputfile);
                        modtca::orttcapeilv(&original_text, usefile, outputfile);
                        modtca::orttcapeformabiap(&original_text, usefile, outputfile);
                        println!("Data appended to the file {}", outputfile);
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }

                // FILE DOES NOT EXIST: TIKUNA ORTHOGRAPHY

                modtca::orttcabr(&original_text, usefile, outputfile);
                let usefile = "old";
                modtca::orttcaco(&original_text, usefile, outputfile);
                modtca::orttcapeilv(&original_text, usefile, outputfile);
                modtca::orttcapeformabiap(&original_text, usefile, outputfile);
                println!("Data written to the file {}", outputfile);
            }
            return;
        }
        // FROM THE COMMAND LINE: TIKUNA ORTHOGRAPHY

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modtca::orttcabr(&original_text, usefile, outputfile);
        modtca::orttcaco(&original_text, usefile, outputfile);
        modtca::orttcapeilv(&original_text, usefile, outputfile);
        modtca::orttcapeformabiap(&original_text, usefile, outputfile);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // INVALID ARGUMENTS

    panic!("{}", red.to_owned() + "Invalid arguments provided! See: --help" + reset);
}
