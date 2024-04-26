// LNGCNV VERSION 1.9.1 / MIT LICENSE / COPYRIGHT © 2022–2024 PIOTR BAJDEK

// MODULE LANG

// CLIPPY LINTS

#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::cognitive_complexity, clippy::missing_panics_doc, clippy::too_many_lines)]

// IMPORTS

use std::env;
use std::fs;
use std::io;
use std::path::Path;

pub mod modeng_au;
pub mod modeng_ort;
pub mod modeng_us;
pub mod modlat;
pub mod modpol;
pub mod modque;
pub mod modspa;
pub mod modtca;

// LANGLIST

pub fn list(reset: &str, red: &str, cyan: &str, yellow: &str) {
    // COLLECT ARGUMENTS FOR CONVERSIONS

    let args: Vec<String> = env::args().collect();

    let arg_cnt = args.len();
    assert!((arg_cnt != 1), "{}", &(red.to_owned() + "Missing arguments! See: --help" + reset));

    let input1 = args.get(1).expect(&(red.to_owned() + "Invalid or missing arguments! See: --help" + reset));
    let input2 = args.get(2).expect(&(red.to_owned() + "Invalid or missing arguments! See: --help" + reset));

    //   ++++++++++   ++++++++++   ++++++++++

    // ENGLISH: IPA

    if input1 == "--ipa" && input2 == "--eng" || input1 == "--eng" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modeng_au::engaucanberra(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modeng_us::engusdallas(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: ENGLISH IPA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--eng";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: ENGLISH IPA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modeng_au::engaucanberra(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        let usefile = "old";
                        modeng_us::engusdallas(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modeng_au::engaucanberra(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modeng_us::engusdallas(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: ENGLISH IPA

                modeng_au::engaucanberra(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                let usefile = "old";
                modeng_us::engusdallas(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: ENGLISH IPA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modeng_au::engaucanberra(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        let usefile = "old";
                        modeng_us::engusdallas(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modeng_au::engaucanberra(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modeng_us::engusdallas(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: ENGLISH IPA

                modeng_au::engaucanberra(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                let usefile = "old";
                modeng_us::engusdallas(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: ENGLISH IPA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modeng_au::engaucanberra(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modeng_us::engusdallas(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // ENGLISH: IPA --eng.au-canberra

    if input1 == "--ipa" && input2 == "--eng.au-canberra" || input1 == "--eng.au-canberra" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modeng_au::engaucanberra(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: CANBERRA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--eng.au-canberra";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: CANBERRA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modeng_au::engaucanberra(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modeng_au::engaucanberra(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: CANBERRA

                modeng_au::engaucanberra(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: CANBERRA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modeng_au::engaucanberra(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modeng_au::engaucanberra(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: CANBERRA

                modeng_au::engaucanberra(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: CANBERRA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modeng_au::engaucanberra(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // ENGLISH: IPA --eng.us-dallas

    if input1 == "--ipa" && input2 == "--eng.us-dallas" || input1 == "--eng.us-dallas" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modeng_us::engusdallas(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: DALLAS
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--eng.us-dallas";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: DALLAS
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modeng_us::engusdallas(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modeng_us::engusdallas(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: DALLAS

                modeng_us::engusdallas(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: DALLAS
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modeng_us::engusdallas(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modeng_us::engusdallas(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: DALLAS

                modeng_us::engusdallas(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: DALLAS

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modeng_us::engusdallas(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // ENGLISH: ORTHOGRAPHY

    if input1 == "--ort" && input2 == "--eng" || input1 == "--eng" && input2 == "--ort" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modeng_ort::ortuseng(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: ENGLISH ORTHOGRAPHY
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ort--eng";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: ENGLISH ORTHOGRAPHY
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modeng_ort::ortuseng(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modeng_ort::ortuseng(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: ENGLISH ORTHOGRAPHY

                modeng_ort::ortuseng(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: ENGLISH ORTHOGRAPHY
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modeng_ort::ortuseng(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modeng_ort::ortuseng(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: ENGLISH ORTHOGRAPHY

                modeng_ort::ortuseng(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: ENGLISH ORTHOGRAPHY

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modeng_ort::ortuseng(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // LATIN: IPA

    if input1 == "--ipa" && input2 == "--lat" || input1 == "--lat" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modlat::ipalat(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: LATIN IPA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--lat";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: LATIN IPA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modlat::ipalat(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modlat::ipalat(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: LATIN IPA

                modlat::ipalat(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: LATIN IPA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modlat::ipalat(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modlat::ipalat(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: LATIN IPA

                modlat::ipalat(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: LATIN IPA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modlat::ipalat(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // LATIN: ORTHOGRAPHY

    if input1 == "--ort" && input2 == "--lat" || input1 == "--lat" && input2 == "--ort" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modlat::ortlat(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: LATIN ORTHOGRAPHY
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ort--lat";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: LATIN ORTHOGRAPHY
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modlat::ortlat(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modlat::ortlat(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: LATIN ORTHOGRAPHY

                modlat::ortlat(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: LATIN ORTHOGRAPHY
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modlat::ortlat(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modlat::ortlat(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: LATIN ORTHOGRAPHY

                modlat::ortlat(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: LATIN ORTHOGRAPHY

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modlat::ortlat(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // POLISH: IPA

    if input1 == "--ipa" && input2 == "--pol" || input1 == "--pol" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modpol::polplczestochowa(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modpol::polpltorun(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modpol::polplwarszawa(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: POLISH IPA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--pol";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: POLISH IPA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modpol::polplczestochowa(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        let usefile = "old";
                        modpol::polpltorun(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modpol::polplwarszawa(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modpol::polplczestochowa(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modpol::polpltorun(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modpol::polplwarszawa(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: POLISH IPA

                modpol::polplczestochowa(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                let usefile = "old";
                modpol::polpltorun(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                modpol::polplwarszawa(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: POLISH IPA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modpol::polplczestochowa(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        let usefile = "old";
                        modpol::polpltorun(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modpol::polplwarszawa(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modpol::polplczestochowa(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modpol::polpltorun(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modpol::polplwarszawa(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: POLISH IPA

                modpol::polplczestochowa(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                let usefile = "old";
                modpol::polpltorun(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modpol::polplwarszawa(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: POLISH IPA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modpol::polplczestochowa(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modpol::polpltorun(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modpol::polplwarszawa(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // POLISH: IPA --pol.pl-czestochowa

    if input1 == "--ipa" && input2 == "--pol.pl-czestochowa" || input1 == "--pol.pl-czestochowa" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modpol::polplczestochowa(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: CZĘSTOCHOWA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--pol.pl-czestochowa";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: CZĘSTOCHOWA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modpol::polplczestochowa(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modpol::polplczestochowa(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: CZĘSTOCHOWA

                modpol::polplczestochowa(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: CZĘSTOCHOWA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modpol::polplczestochowa(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modpol::polplczestochowa(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: CZĘSTOCHOWA

                modpol::polplczestochowa(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: CZĘSTOCHOWA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modpol::polplczestochowa(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // POLISH: IPA --pol.pl-torun

    if input1 == "--ipa" && input2 == "--pol.pl-torun" || input1 == "--pol.pl-torun" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modpol::polpltorun(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: TORUŃ
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--pol.pl-torun";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: TORUŃ
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modpol::polpltorun(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modpol::polpltorun(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: TORUŃ

                modpol::polpltorun(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: TORUŃ
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modpol::polpltorun(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modpol::polpltorun(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: TORUŃ

                modpol::polpltorun(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: TORUŃ

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modpol::polpltorun(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // POLISH: IPA --pol.pl-warszawa

    if input1 == "--ipa" && input2 == "--pol.pl-warszawa" || input1 == "--pol.pl-warszawa" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modpol::polplwarszawa(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: WARSZAWA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--pol.pl-warszawa";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: WARSZAWA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modpol::polplwarszawa(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modpol::polplwarszawa(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: WARSZAWA

                modpol::polplwarszawa(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: WARSZAWA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modpol::polplwarszawa(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modpol::polplwarszawa(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: WARSZAWA

                modpol::polplwarszawa(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: WARSZAWA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modpol::polplwarszawa(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // AYACUCHO QUECHUA: IPA

    if input1 == "--ipa" && input2 == "--que" || input1 == "--que" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modque::ipaque(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: AYACUCHO QUECHUA IPA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--que";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: AYACUCHO QUECHUA IPA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modque::ipaque(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modque::ipaque(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: AYACUCHO QUECHUA IPA

                modque::ipaque(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: AYACUCHO QUECHUA IPA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modque::ipaque(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modque::ipaque(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: AYACUCHO QUECHUA IPA

                modque::ipaque(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: AYACUCHO QUECHUA IPA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modque::ipaque(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // AYACUCHO QUECHUA: DIALECT

    if input1 == "--lct" && input2 == "--que" || input1 == "--que" && input2 == "--lct" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modque::quelct(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: AYACUCHO QUECHUA DIALECT
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--lct--que";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: AYACUCHO QUECHUA DIALECT
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modque::quelct(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modque::quelct(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: AYACUCHO QUECHUA DIALECT

                modque::quelct(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: AYACUCHO QUECHUA DIALECT
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modque::quelct(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modque::quelct(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: AYACUCHO QUECHUA DIALECT

                modque::quelct(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: AYACUCHO QUECHUA DIALECT

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modque::quelct(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // QUECHUA: ORTHOGRAPHY

    if input1 == "--ort" && input2 == "--que" || input1 == "--que" && input2 == "--ort" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modque::ortquetri(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modque::ortquepen(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: QUECHUA ORTHOGRAPHY
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ort--que";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: QUECHUA ORTHOGRAPHY
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modque::ortquetri(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modque::ortquetri(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: QUECHUA ORTHOGRAPHY

                modque::ortquetri(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: QUECHUA ORTHOGRAPHY
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modque::ortquetri(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        let usefile = "old";
                        modque::ortquepen(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modque::ortquetri(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modque::ortquepen(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: QUECHUA ORTHOGRAPHY

                modque::ortquetri(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                let usefile = "old";
                modque::ortquepen(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: QUECHUA ORTHOGRAPHY

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modque::ortquetri(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modque::ortquepen(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SPANISH: IPA

    if input1 == "--ipa" && input2 == "--spa" || input1 == "--spa" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modspa::spacoarauca(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spacobogota(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spacobucaramanga(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spacocali(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spacoleticia(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spacomedellin(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spaconeiva(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spacopasto(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spacoquibdo(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spacosantamarta(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spaesbilbao(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spaescadiz(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spaesmadrid(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spamxciudaddemexico(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spauymontevideo(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: SPANISH IPA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--spa";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: SPANISH IPA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacoarauca(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        let usefile = "old";
                        modspa::spacobogota(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spacobucaramanga(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spacocali(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spacoleticia(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spacomedellin(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spaconeiva(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spacopasto(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spacoquibdo(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spacosantamarta(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spaesbilbao(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spaescadiz(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spaesmadrid(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spamxciudaddemexico(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spauymontevideo(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacoarauca(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spacobogota(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spacobucaramanga(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spacocali(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spacoleticia(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spacomedellin(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spaconeiva(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spacopasto(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spacoquibdo(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spacosantamarta(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spaesbilbao(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spaescadiz(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spaesmadrid(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spamxciudaddemexico(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modspa::spauymontevideo(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: SPANISH IPA

                modspa::spacoarauca(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                let usefile = "old";
                modspa::spacobogota(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                modspa::spacobucaramanga(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                modspa::spacocali(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                modspa::spacoleticia(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                modspa::spacomedellin(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                modspa::spaconeiva(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                modspa::spacopasto(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                modspa::spacoquibdo(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                modspa::spacosantamarta(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                modspa::spaesbilbao(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                modspa::spaescadiz(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                modspa::spaesmadrid(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                modspa::spamxciudaddemexico(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                modspa::spauymontevideo(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: SPANISH IPA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacoarauca(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        let usefile = "old";
                        modspa::spacobogota(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spacobucaramanga(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spacocali(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spacoleticia(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spacomedellin(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spaconeiva(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spacopasto(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spacoquibdo(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spacosantamarta(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spaesbilbao(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spaescadiz(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spaesmadrid(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spamxciudaddemexico(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spauymontevideo(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacoarauca(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spacobogota(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spacobucaramanga(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spacocali(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spacoleticia(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spacomedellin(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spaconeiva(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spacopasto(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spacoquibdo(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spacosantamarta(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spaesbilbao(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spaescadiz(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spaesmadrid(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spamxciudaddemexico(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modspa::spauymontevideo(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: SPANISH IPA

                modspa::spacoarauca(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                let usefile = "old";
                modspa::spacobogota(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spacobucaramanga(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spacocali(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spacoleticia(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spacomedellin(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spaconeiva(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spacopasto(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spacoquibdo(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spacosantamarta(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spaesbilbao(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spaescadiz(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spaesmadrid(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spamxciudaddemexico(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modspa::spauymontevideo(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: SPANISH IPA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modspa::spacoarauca(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modspa::spacobogota(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modspa::spacobucaramanga(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modspa::spacocali(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modspa::spacoleticia(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modspa::spacomedellin(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modspa::spaconeiva(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modspa::spacopasto(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modspa::spacoquibdo(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modspa::spacosantamarta(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modspa::spaesbilbao(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modspa::spaescadiz(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modspa::spaesmadrid(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modspa::spamxciudaddemexico(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modspa::spauymontevideo(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SPANISH: IPA --spa.co-arauca

    if input1 == "--ipa" && input2 == "--spa.co-arauca" || input1 == "--spa.co-arauca" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modspa::spacoarauca(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: ARAUCA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--spa.co-arauca";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: ARAUCA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacoarauca(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacoarauca(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: ARAUCA

                modspa::spacoarauca(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: ARAUCA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacoarauca(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacoarauca(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: ARAUCA

                modspa::spacoarauca(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: ARAUCA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modspa::spacoarauca(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SPANISH: IPA --spa.co-bogota

    if input1 == "--ipa" && input2 == "--spa.co-bogota" || input1 == "--spa.co-bogota" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modspa::spacobogota(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: BOGOTÁ
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--spa.co-bogota";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: BOGOTÁ
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacobogota(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacobogota(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: BOGOTÁ

                modspa::spacobogota(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: BOGOTÁ
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacobogota(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacobogota(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: BOGOTÁ

                modspa::spacobogota(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: BOGOTÁ

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modspa::spacobogota(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SPANISH: IPA --spa.co-bucaramanga

    if input1 == "--ipa" && input2 == "--spa.co-bucaramanga" || input1 == "--spa.co-bucaramanga" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modspa::spacobucaramanga(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: BUCARAMANGA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--spa.co-bucaramanga";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: BUCARAMANGA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacobucaramanga(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacobucaramanga(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: BUCARAMANGA

                modspa::spacobucaramanga(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: BUCARAMANGA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacobucaramanga(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacobucaramanga(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: BUCARAMANGA

                modspa::spacobucaramanga(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: BUCARAMANGA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modspa::spacobucaramanga(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SPANISH: IPA --spa.co-cali

    if input1 == "--ipa" && input2 == "--spa.co-cali" || input1 == "--spa.co-cali" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modspa::spacocali(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: CALI
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--spa.co-cali";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: CALI
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacocali(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacocali(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: CALI

                modspa::spacocali(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: CALI
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacocali(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacocali(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: CALI

                modspa::spacocali(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: CALI

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modspa::spacocali(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SPANISH: IPA --spa.co-leticia

    if input1 == "--ipa" && input2 == "--spa.co-leticia" || input1 == "--spa.co-leticia" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modspa::spacoleticia(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: LETICIA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--spa.co-leticia";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: LETICIA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacoleticia(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacoleticia(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: LETICIA

                modspa::spacoleticia(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: LETICIA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacoleticia(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacoleticia(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: LETICIA

                modspa::spacoleticia(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: LETICIA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modspa::spacoleticia(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SPANISH: IPA --spa.co-medellin

    if input1 == "--ipa" && input2 == "--spa.co-medellin" || input1 == "--spa.co-medellin" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modspa::spacomedellin(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: MEDELLÍN
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--spa.co-medellin";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: MEDELLÍN
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacomedellin(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacomedellin(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: MEDELLÍN

                modspa::spacomedellin(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: MEDELLÍN
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacomedellin(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacomedellin(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: MEDELLÍN

                modspa::spacomedellin(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: MEDELLÍN
        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modspa::spacomedellin(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SPANISH: IPA --spa.co-neiva

    if input1 == "--ipa" && input2 == "--spa.co-neiva" || input1 == "--spa.co-neiva" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modspa::spaconeiva(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: NEIVA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--spa.co-neiva";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: NEIVA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spaconeiva(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spaconeiva(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: NEIVA

                modspa::spaconeiva(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: NEIVA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spaconeiva(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spaconeiva(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: NEIVA

                modspa::spaconeiva(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: NEIVA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modspa::spaconeiva(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SPANISH: IPA --spa.co-pasto

    if input1 == "--ipa" && input2 == "--spa.co-pasto" || input1 == "--spa.co-pasto" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modspa::spacopasto(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: PASTO
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--spa.co-pasto";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: PASTO
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacopasto(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacopasto(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: PASTO

                modspa::spacopasto(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: PASTO
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacopasto(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacopasto(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: PASTO

                modspa::spacopasto(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: PASTO

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modspa::spacopasto(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SPANISH: IPA --spa.co-quibdo

    if input1 == "--ipa" && input2 == "--spa.co-quibdo" || input1 == "--spa.co-quibdo" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modspa::spacoquibdo(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: QUIBDÓ
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--spa.co-quibdo";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: QUIBDÓ
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacoquibdo(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacoquibdo(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: QUIBDÓ

                modspa::spacoquibdo(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: QUIBDÓ
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacoquibdo(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacoquibdo(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: QUIBDÓ

                modspa::spacoquibdo(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: QUIBDÓ

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modspa::spacoquibdo(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SPANISH: IPA --spa.co-santa_marta

    if input1 == "--ipa" && input2 == "--spa.co-santa_marta" || input1 == "--spa.co-santa_marta" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modspa::spacosantamarta(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: SANTA MARTA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--spa.co-santa_marta";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: SANTA MARTA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacosantamarta(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacosantamarta(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: SANTA MARTA

                modspa::spacosantamarta(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: SANTA MARTA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spacosantamarta(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spacosantamarta(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: SANTA MARTA

                modspa::spacosantamarta(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: SANTA MARTA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modspa::spacosantamarta(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SPANISH: IPA --spa.es-bilbao

    if input1 == "--ipa" && input2 == "--spa.es-bilbao" || input1 == "--spa.es-bilbao" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modspa::spaesbilbao(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: BILBAO
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--spa.es-bilbao";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: BILBAO
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spaesbilbao(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spaesbilbao(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: BILBAO

                modspa::spaesbilbao(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: BILBAO
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spaesbilbao(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spaesbilbao(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: BILBAO

                modspa::spaesbilbao(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: BILBAO

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modspa::spaesbilbao(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SPANISH: IPA --spa.es-cadiz

    if input1 == "--ipa" && input2 == "--spa.es-cadiz" || input1 == "--spa.es-cadiz" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modspa::spaescadiz(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: CÁDIZ
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--spa.es-cadiz";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: CÁDIZ
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spaescadiz(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spaescadiz(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: CÁDIZ

                modspa::spaescadiz(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: CÁDIZ
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spaescadiz(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spaescadiz(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: CÁDIZ

                modspa::spaescadiz(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: CÁDIZ

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modspa::spaescadiz(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SPANISH: IPA --spa.es-madrid

    if input1 == "--ipa" && input2 == "--spa.es-madrid" || input1 == "--spa.es-madrid" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modspa::spaesmadrid(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: MADRID
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--spa.es-madrid";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: MADRID
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spaesmadrid(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spaesmadrid(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: MADRID

                modspa::spaesmadrid(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: MADRID
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spaesmadrid(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spaesmadrid(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: MADRID

                modspa::spaesmadrid(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: MADRID

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modspa::spaesmadrid(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SPANISH: IPA --spa.mx-ciudad_de_mexico

    if input1 == "--ipa" && input2 == "--spa.mx-ciudad_de_mexico" || input1 == "--spa.mx-ciudad_de_mexico" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modspa::spamxciudaddemexico(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: CIUDAD DE MÉXICO
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--spa.mx-ciudad_de_mexico";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: CIUDAD DE MÉXICO
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spamxciudaddemexico(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spamxciudaddemexico(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: CIUDAD DE MÉXICO

                modspa::spamxciudaddemexico(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: CIUDAD DE MÉXICO
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spamxciudaddemexico(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spamxciudaddemexico(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: CIUDAD DE MÉXICO

                modspa::spamxciudaddemexico(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: CIUDAD DE MÉXICO

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modspa::spamxciudaddemexico(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SPANISH: IPA --spa.uy-montevideo

    if input1 == "--ipa" && input2 == "--spa.uy-montevideo" || input1 == "--spa.uy-montevideo" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modspa::spauymontevideo(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: MONTEVIDEO
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--spa.uy-montevideo";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: MONTEVIDEO
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spauymontevideo(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spauymontevideo(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: MONTEVIDEO

                modspa::spauymontevideo(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: MONTEVIDEO
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modspa::spauymontevideo(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modspa::spauymontevideo(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: MONTEVIDEO

                modspa::spauymontevideo(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: MONTEVIDEO

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modspa::spauymontevideo(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // TIKUNA: IPA

    if input1 == "--ipa" && input2 == "--tca" || input1 == "--tca" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modtca::tcacoriocotuhe(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modtca::tcapecushillococha(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modtca::tcaconazareth(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modtca::tcabrumariacu(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modtca::tcabrvilabetania(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: TIKUNA IPA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--tca";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: TIKUNA IPA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modtca::tcacoriocotuhe(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        let usefile = "old";
                        modtca::tcapecushillococha(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modtca::tcaconazareth(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modtca::tcabrumariacu(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modtca::tcabrvilabetania(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modtca::tcacoriocotuhe(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modtca::tcapecushillococha(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modtca::tcaconazareth(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modtca::tcabrumariacu(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modtca::tcabrvilabetania(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: TIKUNA IPA

                modtca::tcacoriocotuhe(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                let usefile = "old";
                modtca::tcapecushillococha(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                modtca::tcaconazareth(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                modtca::tcabrumariacu(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                modtca::tcabrvilabetania(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: TIKUNA IPA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modtca::tcacoriocotuhe(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        let usefile = "old";
                        modtca::tcapecushillococha(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modtca::tcaconazareth(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modtca::tcabrumariacu(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modtca::tcabrvilabetania(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modtca::tcacoriocotuhe(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modtca::tcapecushillococha(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modtca::tcaconazareth(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modtca::tcabrumariacu(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modtca::tcabrvilabetania(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: TIKUNA IPA

                modtca::tcacoriocotuhe(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                let usefile = "old";
                modtca::tcapecushillococha(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modtca::tcaconazareth(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modtca::tcabrumariacu(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modtca::tcabrvilabetania(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: TIKUNA IPA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modtca::tcacoriocotuhe(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modtca::tcapecushillococha(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modtca::tcaconazareth(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modtca::tcabrumariacu(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modtca::tcabrvilabetania(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // TIKUNA: IPA --tca.br-umariacu

    if input1 == "--ipa" && input2 == "--tca.br-umariacu" || input1 == "--tca.br-umariacu" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modtca::tcabrumariacu(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: UMARIAÇU
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--tca.br-umariacu";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: UMARIAÇU
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modtca::tcabrumariacu(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modtca::tcabrumariacu(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: UMARIAÇU

                modtca::tcabrumariacu(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: UMARIAÇU
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modtca::tcabrumariacu(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modtca::tcabrumariacu(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: UMARIAÇU

                modtca::tcabrumariacu(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: UMARIAÇU

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modtca::tcabrumariacu(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // TIKUNA: IPA --tca.br-vila_betania

    if input1 == "--ipa" && input2 == "--tca.br-vila_betania" || input1 == "--tca.br-vila_betania" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modtca::tcabrvilabetania(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: VILA BETÂNIA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--tca.br-vila_betania";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: VILA BETÂNIA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modtca::tcabrvilabetania(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modtca::tcabrvilabetania(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: VILA BETÂNIA

                modtca::tcabrvilabetania(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: VILA BETÂNIA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modtca::tcabrvilabetania(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modtca::tcabrvilabetania(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: VILA BETÂNIA

                modtca::tcabrvilabetania(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: VILA BETÂNIA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modtca::tcabrvilabetania(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // TIKUNA: IPA --tca.co-nazareth

    if input1 == "--ipa" && input2 == "--tca.co-nazareth" || input1 == "--tca.co-nazareth" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modtca::tcaconazareth(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: NAZARETH
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--tca.co-nazareth";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: NAZARETH
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modtca::tcaconazareth(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modtca::tcaconazareth(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: NAZARETH

                modtca::tcaconazareth(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: NAZARETH
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modtca::tcaconazareth(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modtca::tcaconazareth(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: NAZARETH

                modtca::tcaconazareth(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: NAZARETH

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modtca::tcaconazareth(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // TIKUNA: IPA --tca.co-rio_cotuhe

    if input1 == "--ipa" && input2 == "--tca.co-rio_cotuhe" || input1 == "--tca.co-rio_cotuhe" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modtca::tcacoriocotuhe(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: RIO COTUHÉ
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--tca.co-rio_cotuhe";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: RIO COTUHÉ
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modtca::tcacoriocotuhe(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modtca::tcacoriocotuhe(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: RIO COTUHÉ

                modtca::tcacoriocotuhe(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: RIO COTUHÉ
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modtca::tcacoriocotuhe(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modtca::tcacoriocotuhe(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: RIO COTUHÉ

                modtca::tcacoriocotuhe(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: RIO COTUHÉ

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modtca::tcacoriocotuhe(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // TIKUNA: IPA --tca.pe-cushillococha

    if input1 == "--ipa" && input2 == "--tca.pe-cushillococha" || input1 == "--tca.pe-cushillococha" && input2 == "--ipa" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modtca::tcapecushillococha(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: CUSHILLOCOCHA
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ipa--tca.pe-cushillococha";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: CUSHILLOCOCHA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modtca::tcapecushillococha(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modtca::tcapecushillococha(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: CUSHILLOCOCHA

                modtca::tcapecushillococha(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: CUSHILLOCOCHA
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modtca::tcapecushillococha(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modtca::tcapecushillococha(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: CUSHILLOCOCHA

                modtca::tcapecushillococha(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: CUSHILLOCOCHA

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modtca::tcapecushillococha(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // TIKUNA: ORTHOGRAPHY

    if input1 == "--ort" && input2 == "--tca" || input1 == "--tca" && input2 == "--ort" {
        let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));

        // REPL MODE
        if str0 == "-r" || str0 == "--repl" {
            println!();
            println!("{}", yellow.to_owned() + "LNGCNV INTERACTIVE SHELL (REPL MODE)" + reset);
            println!();
            println!("Enter your query... Type {}", cyan.to_owned() + "-q" + reset + " to quit:");
            println!();
            loop {
                let mut input_repl = String::new();
                io::stdin().read_line(&mut input_repl).expect("Unable to read entered data");
                let original_text: &str = input_repl.trim();
                if original_text == "-q" {
                    return;
                }
                let usefile = "terminal";
                let outputfile = "0";
                modtca::orttcabr(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modtca::orttcaco(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modtca::orttcapeilv(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modtca::orttcapeformabiap(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!();
            }
        }

        // FROM A FILE: TIKUNA ORTHOGRAPHY
        if str0 == "-i" || str0 == "--input" {
            let usefile = "new";
            let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));

            // DEFAULT OUTPUT FILE
            if arg_cnt == 5 {
                let inputfile_arg = inputfile.as_str();
                let inputfile_str: &str = inputfile_arg.trim();
                let outputfile = inputfile_str.to_owned() + "--ort--tca";
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(&outputfile).is_file();

                // FILE ALREADY EXISTS: TIKUNA ORTHOGRAPHY
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + &outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modtca::orttcabr(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        let usefile = "old";
                        modtca::orttcaco(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modtca::orttcapeilv(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modtca::orttcapeformabiap(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modtca::orttcabr(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modtca::orttcaco(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modtca::orttcapeilv(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        modtca::orttcapeformabiap(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: TIKUNA ORTHOGRAPHY

                modtca::orttcabr(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                let usefile = "old";
                modtca::orttcaco(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                modtca::orttcapeilv(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                modtca::orttcapeformabiap(&original_text, usefile, &outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }

            // USER-DEFINED OUTPUT FILE
            let output = args.get(5).expect(&(red.to_owned() + "Missing arguments! Use option: --output See: --help" + reset));
            if output == "-o" || output == "--output" {
                let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
                let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

                let fileexists: bool = Path::new(outputfile).is_file();

                // FILE ALREADY EXISTS: TIKUNA ORTHOGRAPHY
                if fileexists {
                    println!("{}", red.to_owned() + "The file " + outputfile + " already exists!" + reset + " Overwrite (" + cyan + "o" + reset + ")/ Append (" + cyan + "a" + reset + ")/ Cancel (" + cyan + "other key" + reset + ")" + cyan);
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Unable to read entered data");
                    print!("{reset}");
                    let option: &str = answer.trim();

                    if option == "o" {
                        modtca::orttcabr(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        let usefile = "old";
                        modtca::orttcaco(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modtca::orttcapeilv(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modtca::orttcapeformabiap(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("File {}", outputfile.clone() + " overwritten");
                    } else if option == "a" {
                        let usefile = "old";
                        modtca::orttcabr(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modtca::orttcaco(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modtca::orttcapeilv(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        modtca::orttcapeformabiap(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                        println!("Data appended to the file {outputfile}");
                    } else {
                        println!("Operation aborted");
                    }
                    return;
                }
                // FILE DOES NOT EXIST: TIKUNA ORTHOGRAPHY

                modtca::orttcabr(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                let usefile = "old";
                modtca::orttcaco(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modtca::orttcapeilv(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                modtca::orttcapeformabiap(&original_text, usefile, outputfile, reset, red, cyan, yellow);
                println!("Data written to the file {outputfile}");
                return;
            }
            panic!("{}", &(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset))
        }
        // FROM THE COMMAND LINE: TIKUNA ORTHOGRAPHY

        let original_text = str0;
        let usefile = "terminal";
        let outputfile = "0";
        modtca::orttcabr(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modtca::orttcaco(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modtca::orttcapeilv(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        modtca::orttcapeformabiap(&original_text, usefile, outputfile, reset, red, cyan, yellow);
        println!();
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // INVALID ARGUMENTS

    panic!("{}", red.to_owned() + "Invalid arguments provided! See: --help" + reset);
}
