// LNGCNV VERSION 1.6.0-BETA.11 / MIT LICENSE © 2022 PIOTR BAJDEK

// MODULE MENU

// CLIPPY LINTS

#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::too_many_lines)]

// IMPORTS

use std::env;
use std::process::exit;

// DOCUMENTATION

pub fn documentation() {
    let reset = "\x1b[0m";
    let blue_underlined = "\x1b[34;4m";
    let cyan = "\x1b[36m";
    let yellow = "\x1b[93m";
    let grey = "\x1b[38;5;240m";

    // ARGUMENTS ANYWHERE WITHIN THE STRING

    for argument in env::args() {
        // ABOUT

        if argument == "-a" || argument == "--about" {
            println!("Program:  {}", yellow.to_owned() + "lngcnv" + reset);
            println!("Version:  1.6.0-beta.11");
            println!("Date:     November 8, 2022");
            println!("Author:   Piotr Bajdek (Poland)");
            println!("Contact:  {}", blue_underlined.to_owned() + "piotr.bajdek@proton.me" + reset);
            println!("Source:   {}", blue_underlined.to_owned() + "https://github.com/piotrbajdek/lngcnv" + reset);
            println!("License:  MIT License © 2022 Piotr Bajdek");
            exit(0);
        }

        // CHANGES

        if argument == "-c" || argument == "--changes" {
            println!("{}", cyan.to_owned() + "--.--.---- v1.6.0 – Added --ipa --spa; Enhanced --ipa --pol, --ipa --tca, --ort --eng; Improved --ipa --eng; Enhanced source code" + reset);
            println!("15.05.2022 v1.5.1 – Improved source code");
            println!("14.05.2022 v1.5.0 – Enhanced --ipa --tca; Enhanced source code");
            println!("09.05.2022 v1.4.1 – Improved --ipa --eng, --ipa --lat, --lct --que, --ort --eng");
            println!("03.05.2022 v1.4.0 – Added --ipa --eng; Enhanced --ipa --tca, --ort --eng; Improved --ipa --lat");
            println!("14.02.2022 v1.3.0 – Added --lct --que; Added --input --output");
            println!("11.02.2022 v1.2.0 – Added --ipa --que");
            println!("09.02.2022 v1.1.0 – Added --ipa --pol");
            println!("07.02.2022 v1.0.0 – Implemented --ipa --lat, --ipa --tca, --ort --eng, --ort --lat, --ort --que, --ort --tca");
            exit(0);
        }

        // HELP

        if argument == "-h" || argument == "--help" {
            println!("Usage:{}", yellow.to_owned() + "    lngcnv [OPTION 1/2] [OPTION 2/1] [STRING]");
            println!("          lngcnv [OPTION 1/2] [OPTION 2/1] [OPTION 3] [FILE] [OPTION 4] [FILE]");
            println!("{}", reset);
            println!("OPTION 1: {}", cyan.to_owned() + "--ipa" + reset + "  Transcribe [STRING/ FILE] into the Intern. Phonetic Alphabet");
            println!("          {}", cyan.to_owned() + "--lct" + reset + "  Translate [STRING/ FILE] between dialects or lang. varieties");
            println!("          {}", cyan.to_owned() + "--ort" + reset + "  Convert [STRING/ FILE] between regional norms of orthography");
            println!();
            println!("OPTION 2: {}", cyan.to_owned() + "--eng" + reset + "  English " + grey + "|" + reset + " available for: " + cyan + "--ipa" + reset + "," + cyan + "        --ort");
            println!("          --lat{}", reset.to_owned() + "  Latin   " + grey + "|" + reset + " available for: " + cyan + "--ipa" + reset + "," + cyan + "        --ort");
            println!("          --pol{}", reset.to_owned() + "  Polish  " + grey + "|" + reset + " available for: " + cyan + "--ipa");
            println!("          --que{}", reset.to_owned() + "  Quechua " + grey + "|" + reset + " available for: " + cyan + "--ipa" + reset + ", " + cyan + "--lct" + reset + ", " + cyan + "--ort");
            println!("          --spa{}", reset.to_owned() + "  Spanish " + grey + "|" + reset + " available for: " + cyan + "--ipa");
            println!("          --tca{}", reset.to_owned() + "  Tikuna  " + grey + "|" + reset + " available for: " + cyan + "--ipa" + reset + ", " + cyan + "       --ort" + reset);
            println!();
            println!("Examples:{}", yellow.to_owned() + r#" lngcnv --ipa --lat "Vēnī, vīdī, vīcī.""#);
            println!("          lngcnv --lat --ipa 'Vēnī, vīdī, vīcī.'");
            println!("{}", reset);
            println!("          For multi-word queries always use single or double quotation marks!");
            println!("          For word strings including apostrophes, use double quotation.");
            println!();
            println!("          In the {}", cyan.to_owned() + "--ipa" + reset + " mode, the output can also be constrained to a selected");
            println!("          variant of pronunciation if two or more are available (see {}", cyan.to_owned() + "-l" + reset + "):");
            println!();
            println!("          {}", cyan.to_owned() + "--pol" + reset + ": " + cyan + "--pol.pl-czestochowa" + reset + ", " + cyan + "--pol.pl-torun" + reset + ", " + cyan + "--pol.pl-warszawa");
            println!();
            println!("          --spa{}", reset.to_owned() + ": " + cyan + "--spa.co-bogota" + reset + ", " + cyan + "--spa.co-medellin" + reset + ", " + cyan + "--spa.es-cadiz" + reset + ",");
            println!("                 {}", cyan.to_owned() + "--spa.es-madrid" + reset + ", " + cyan + "--spa.mx-ciudad_de_mexico");
            println!();
            println!("          --tca{}", reset.to_owned() + ": " + cyan + "--tca.br-umariacu" + reset + ", " + cyan + "--tca.br-vila_betania" + reset + ", " + cyan + "--tca.co-nazareth" + reset + ",");
            println!("                 {}", cyan.to_owned() + "--tca.co-rio_cotuhe" + reset + ", " + cyan + "--tca.pe-cushillococha" + reset);
            println!();
            println!("Examples:{}", yellow.to_owned() + " lngcnv --ipa --spa 'Una frase en español'" + reset + "    [display all variants]");
            print!("{}", yellow);
            println!("          lngcnv --ipa --spa.co-bogota 'Una frase en español'{}", reset.to_owned() + "    [Bogotá, CO]");
            println!();
            println!("OPTION 3:{}", cyan.to_owned() + " -i" + reset + ", " + cyan + "--input" + reset + "      Choose a file to open for conversion");
            println!("OPTION 4:{}", cyan.to_owned() + " -o" + reset + ", " + cyan + "--output" + reset + "     Choose a name for the file to create");
            println!();
            println!("Examples:{}", yellow.to_owned() + " lngcnv --ipa --lat --input latin_book.txt --output output_file.txt");
            println!("          lngcnv --lat --ipa --input latin_book.txt --output output_file.txt");
            println!("{}", reset);
            println!("See also: {}", cyan.to_owned() + "-a" + reset + ", " + cyan + "--about" + reset + "      Show contact and program info");
            println!("          {}", cyan.to_owned() + "-c" + reset + ", " + cyan + "--changes" + reset + "    Show simplified change notes");
            println!("          {}", cyan.to_owned() + "-h" + reset + ", " + cyan + "--help" + reset + "       Show this help");
            println!("          {}", cyan.to_owned() + "-l" + reset + ", " + cyan + "--languages" + reset + "  Show additional info");
            println!("          {}", cyan.to_owned() + "-L" + reset + ", " + cyan + "--license" + reset + "    Show licensing information");
            println!("          {}", cyan.to_owned() + "-v" + reset + ", " + cyan + "--version" + reset + "    Show the program version");
            exit(0);
        }

        // LANGUAGES

        if argument == "-l" || argument == "--languages" {
            print!("{}", yellow);
            println!("English:{}", reset.to_owned() + "  The " + cyan + "--ipa" + reset + " mode of operation displays the pronunciation of Australian English (Canberra, ACT). In the " + cyan + "--ort" + reset + " mode, the text is transcribed into American English.");
            println!();
            print!("{}", yellow);
            println!("Latin:{}", reset.to_owned() + "    The modern spelling of Latin and the ancient convention can be used indifferently in the " + cyan + "--ipa" + reset + " mode. To mark the duration of vowels, type either apices or macrons. In the " + cyan + "--ort" + reset + " mode, the text is transcribed into the ancient convention (before the 2nd c. AD).");
            println!();
            print!("{}", yellow);
            println!("Polish:{}", reset.to_owned() + "   Three variants of pronunciation are available (" + cyan + "--ipa" + reset + "): Częstochowa (Małopolska Region), Toruń (Wielkopolska Region), and Warszawa (Mazowsze Region)");
            println!();
            print!("{}", yellow);
            println!("Quechua:{}", reset.to_owned() + "  In the " + cyan + "--ipa" + reset + " mode, Ayacucho Quechua is implemented and the input must be spelled accordingly. The " + cyan + "--lct" + reset + " mode of operation translates into Ayacucho Quechua from other varieties of Southen Quechua. Dialectal features are mostly converted by the algorithm but manual adjustments are necessary. The " + cyan + "--ort" + reset + " mode allows transcribing between the trivocalic and the pentavocalic orthographies and works fine with any language of the Quechuan Family.");
            println!();
            print!("{}", yellow);
            println!("Spanish:{}", reset.to_owned() + "  Five variants of pronunciation are available (" + cyan + "--ipa" + reset + "), including two from Colombia (Bogotá; Medellín), one from Mexico (Ciudad de México), and two from Spain (Cádiz; Madrid).");
            println!();
            print!("{}", yellow);
            println!("Tikuna:{}", reset.to_owned() + "   Five variants of pronunciation (Río Cotuhé, CO; Cushillococha, PE; Nazareth, CO; Umariaçu, BR; Vila Betânia, BR) (" + cyan + "--ipa" + reset + ") and four distinct orthographies (Brazil; Colombia; Peru–ILV; Peru–FORMABIAP) (" + cyan + "--ort" + reset + ") are supported. There is no support for tones at this stage of program development. Even so, in the " + cyan + "--ipa" + reset + " mode tonal annotations can be displayed in subscript if inserted manually as numbers beginning from 1 (the lowest) to 6 (the highest tone).");
            println!("          Abbreviations: ILV - Instituto Lingüístico de Verano; FORMABIAP - Formación de Maestros Bilingües de la Amazonía Peruana");
            exit(0);
        }

        // LICENSE

        if argument == "-L" || argument == "--license" {
            println!("{}", yellow.to_owned() + "MIT License" + reset);
            println!();
            println!("Copyright © 2022 Piotr Bajdek");
            println!();
            println!("Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:");
            println!();
            println!("The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.");
            println!();
            println!("THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.");
            exit(0);
        }

        // VERSION

        if argument == "-v" || argument == "--version" {
            println!("Version: {}", yellow.to_owned() + "1.6.0-beta.11" + reset);
            println!("November 8, 2022");
            exit(0);
        }
    }
}
