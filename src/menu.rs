// LNGCNV VERSION 1.8.6 / MIT LICENSE © 2022–2023 PIOTR BAJDEK

// MODULE MENU

// CLIPPY LINTS

#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::too_many_lines)]

// IMPORTS

use std::env;
use std::process::exit;

// DOCUMENTATION

pub fn documentation(reset: &str, blue_underlined: &str, cyan: &str, yellow: &str, grey: &str) {
    // ARGUMENTS ANYWHERE WITHIN THE STRING

    for argument in env::args() {
        // ABOUT

        if argument == "-a" || argument == "--about" {
            println!("{}", grey.to_owned() + "Program" + reset + ":  " + yellow + "lngcnv" + reset);
            println!("{}", grey.to_owned() + "Version" + reset + ":  1.8.6");
            println!("{}", grey.to_owned() + "Date" + reset + ":     March 21, 2023");
            println!("{}", grey.to_owned() + "Author" + reset + ":   Piotr Bajdek");
            println!("{}", grey.to_owned() + "Contact" + reset + ":  " + blue_underlined + "piotr.bajdek@proton.me" + reset);
            println!("{}", grey.to_owned() + "Source" + reset + ":   " + blue_underlined + "https://github.com/piotrbajdek/lngcnv" + reset);
            println!("{}", grey.to_owned() + "License" + reset + ":  MIT License © 2022–2023 Piotr Bajdek");
            exit(0);
        }

        // CHANGES

        if argument == "-c" || argument == "--changes" {
            println!("{}", yellow.to_owned() + "21.03.2023 v1.8.6 – Improved --ipa --eng, --ipa --spa" + reset);
            println!("16.02.2023 {}", yellow.to_owned() + "v1.8.5" + reset + " – Improved --ipa --eng, --ipa --pol" + reset);
            println!("02.02.2023 {}", yellow.to_owned() + "v1.8.4" + reset + " – Enhanced layout and documentation");
            println!("18.01.2023 {}", yellow.to_owned() + "v1.8.3" + reset + " – Improved --ipa --pol, --ipa --que, --ipa --spa");
            println!("13.01.2023 {}", yellow.to_owned() + "v1.8.2" + reset + " – Improved --ipa --pol, --ipa --spa");
            println!("09.01.2023 {}", yellow.to_owned() + "v1.8.1" + reset + " – Improved --ipa --spa");
            println!("03.01.2023 {}", yellow.to_owned() + "v1.8.0" + reset + " – Enhanced --ipa --spa");
            println!("11.12.2022 {}", yellow.to_owned() + "v1.7.0" + reset + " – Added --repl; Enhanced --ipa --spa; Improved --ipa --eng, --ipa --pol");
            println!("14.11.2022 {}", yellow.to_owned() + "v1.6.1" + reset + " – Enhanced source code");
            println!("12.11.2022 {}", yellow.to_owned() + "v1.6.0" + reset + " – Added --ipa --spa; Enhanced --ipa --pol, --ipa --tca, --ort --eng; Improved --ipa --eng; Rewritten source code");
            println!("15.05.2022 {}", yellow.to_owned() + "v1.5.1" + reset + " – Improved source code");
            println!("14.05.2022 {}", yellow.to_owned() + "v1.5.0" + reset + " – Enhanced --ipa --tca; Enhanced source code");
            println!("09.05.2022 {}", yellow.to_owned() + "v1.4.1" + reset + " – Improved --ipa --eng, --ipa --lat, --lct --que, --ort --eng");
            println!("03.05.2022 {}", yellow.to_owned() + "v1.4.0" + reset + " – Added --ipa --eng; Enhanced --ipa --tca, --ort --eng; Improved --ipa --lat");
            println!("14.02.2022 {}", yellow.to_owned() + "v1.3.0" + reset + " – Added --lct --que; Added --input --output");
            println!("11.02.2022 {}", yellow.to_owned() + "v1.2.0" + reset + " – Added --ipa --que");
            println!("09.02.2022 {}", yellow.to_owned() + "v1.1.0" + reset + " – Added --ipa --pol");
            println!("07.02.2022 {}", yellow.to_owned() + "v1.0.0" + reset + " – Implemented --ipa --lat, --ipa --tca, --ort --eng, --ort --lat, --ort --que, --ort --tca");
            exit(0);
        }

        // CITATION

        if argument == "-C" || argument == "--citation" {
            println!("Bajdek, P., 2023. lngcnv (version 1.8.6). [computer software] https://github.com/piotrbajdek/lngcnv");
            exit(0);
        }

        // HELP

        if argument == "-h" || argument == "--help" {
            println!("{}", grey.to_owned() + "1" + reset + "." + grey + " PROGRAM SYNTAX AND LANGUAGE OPTIONS" + reset);
            println!();
            println!("{}", grey.to_owned() + "Usage" + reset + ":" + yellow + "    lngcnv [option 1/2] [option 2/1] [string]" + reset + " [simple terminal queries]" + yellow);
            println!("          lngcnv [option 1/2] [option 2/1] [option 3]{}", reset.to_owned() + "           [REPL " + cyan + "--repl" + reset + "]" + yellow);
            println!("          lngcnv [option 1/2] [option 2/1] [option 3] [file] [optn. 4] [file]");
            println!("{reset}");
            println!("{}", yellow.to_owned() + "option 1" + reset + ": " + cyan + "--ipa" + reset + "  Transcribe [string/ file] into the Intern. Phonetic Alphabet");
            println!("          {}", cyan.to_owned() + "--lct" + reset + "  Translate [string/ file] between dialects or lang. varieties");
            println!("          {}", cyan.to_owned() + "--ort" + reset + "  Convert [string/ file] between regional norms of orthography");
            println!();
            println!("{}", yellow.to_owned() + "option 2" + reset + ": " + cyan + "--eng" + reset + "  English " + grey + "|" + reset + " used with: " + cyan + "--ipa" + reset + ", " + cyan + "       --ort");
            println!("          --lat{}", reset.to_owned() + "  Latin   " + grey + "|" + reset + " used with: " + cyan + "--ipa" + reset + ", " + cyan + "       --ort");
            println!("          --pol{}", reset.to_owned() + "  Polish  " + grey + "|" + reset + " used with: " + cyan + "--ipa");
            println!("          --que{}", reset.to_owned() + "  Quechua " + grey + "|" + reset + " used with: " + cyan + "--ipa" + reset + ", " + cyan + "--lct" + reset + ", " + cyan + "--ort");
            println!("          --spa{}", reset.to_owned() + "  Spanish " + grey + "|" + reset + " used with: " + cyan + "--ipa");
            println!("          --tca{}", reset.to_owned() + "  Tikuna  " + grey + "|" + reset + " used with: " + cyan + "--ipa" + reset + ", " + cyan + "       --ort" + reset);
            println!();
            println!("          In the {}", cyan.to_owned() + "--ipa" + reset + " mode, the output can also be constrained to a selected");
            println!("          variant of pronunciation if two or more are available (see {}", cyan.to_owned() + "-l" + reset + "):");
            println!();
            println!("          {}", cyan.to_owned() + "--pol" + reset + ": " + cyan + "--pol.pl-czestochowa" + reset + ", " + cyan + "--pol.pl-torun" + reset + ", " + cyan + "--pol.pl-warszawa");
            println!();
            println!("          --spa{}", reset.to_owned() + ": " + cyan + "--spa.co-arauca" + reset + ", " + cyan + "--spa.co-bogota" + reset + ", " + cyan + "--spa.co-bucaramanga" + reset + ", [" + grey + "CO" + reset + "]");
            println!("                 {}", cyan.to_owned() + "--spa.co-cali" + reset + ", " + cyan + "--spa.co-leticia" + reset + ", " + cyan + "--spa.co-medellin" + reset + ",     [" + grey + "CO" + reset + "]");
            println!("                 {}", cyan.to_owned() + "--spa.co-neiva" + reset + ", " + cyan + "--spa.co-pasto" + reset + ", " + cyan + "--spa.co-quibdo" + reset + ",        [" + grey + "CO" + reset + "]");
            println!("                 {}", cyan.to_owned() + "--spa.co-santa_marta" + reset + ",                                   [" + grey + "CO" + reset + "]");
            println!("                 {}", cyan.to_owned() + "--spa.es-bilbao" + reset + ", " + cyan + "--spa.es-cadiz" + reset + ", " + cyan + "--spa.es-madrid" + reset + ",       [" + grey + "ES" + reset + "]");
            println!("                 {}", cyan.to_owned() + "--spa.mx-ciudad_de_mexico" + reset + ", " + cyan + "--spa.uy-montevideo" + reset + "      [" + grey + "MX" + reset + ", " + grey + "UY" + reset + "]" + cyan);
            println!();
            println!("          --tca{}", reset.to_owned() + ": " + cyan + "--tca.br-umariacu" + reset + ", " + cyan + "--tca.br-vila_betania" + reset + ", " + cyan + "--tca.co-nazareth" + reset + ",");
            println!("                 {}", cyan.to_owned() + "--tca.co-rio_cotuhe" + reset + ", " + cyan + "--tca.pe-cushillococha" + reset);
            println!();
            println!("{}", grey.to_owned() + "Examples" + reset + ":" + yellow + " lngcnv --ipa --spa 'Una frase en español'" + reset + "    [display all variants]");
            print!("{yellow}");
            println!("          lngcnv --ipa --spa.co-bogota 'Una frase en español'{}", reset.to_owned() + "    [Bogotá, CO]");
            println!();
            println!("{}", grey.to_owned() + "2" + reset + "." + grey + " SIMPLE TERMINAL QUERIES" + reset);
            println!();
            println!("{}", grey.to_owned() + "Examples" + reset + ":" + yellow + r#" lngcnv --ipa --lat "Vēnī, vīdī, vīcī.""#);
            println!("          lngcnv --lat --ipa 'Vēnī, vīdī, vīcī.'");
            println!("{reset}");
            println!("          For multi-word queries, use single or double quotes, and for word");
            println!("          strings containing apostrophes, use double quotes.");
            println!();
            println!("{}", grey.to_owned() + "3" + reset + "." + grey + " THE INTERACTIVE SHELL" + reset);
            println!();
            println!("{}", yellow.to_owned() + "option 3" + reset + ": " + cyan + "-r" + reset + ", " + cyan + "--repl" + reset + "       Interactive shell mode (REPL)    [type " + cyan + "-q" + reset + " to quit]");
            println!();
            println!("{}", grey.to_owned() + "Example" + reset + ":" + yellow + "  lngcnv --ipa --pol.pl-czestochowa --repl" + reset + "        [type in the shell]");
            println!("                                 [avoid enclosing queries in quotation marks]");
            println!();
            println!("{}", grey.to_owned() + "4" + reset + "." + grey + " OPERATIONS ON FILES" + reset);
            println!();
            println!("{}", yellow.to_owned() + "option 3" + reset + ": " + cyan + "-i" + reset + ", " + cyan + "--input" + reset + "      Choose a file to open for conversion");
            println!("{}", yellow.to_owned() + "option 4" + reset + ": " + cyan + "-o" + reset + ", " + cyan + "--output" + reset + "     Specify the name for the output file    [optional]");
            println!();
            println!("{}", grey.to_owned() + "Examples" + reset + ":" + yellow + " lngcnv --ipa --pol --input polish_book.txt --output output_file.txt");
            println!("          lngcnv --ipa --pol --input polish_book{}", reset.to_owned() + "   [default output file name]");
            println!();
            println!("{}", grey.to_owned() + "5" + reset + "." + grey + " PROGRAM DOCUMENTATION" + reset);
            println!();
            println!("{}", grey.to_owned() + "See also" + reset + ": " + cyan + "-a" + reset + ", " + cyan + "--about" + reset + "      Show contact and program information");
            println!("          {}", cyan.to_owned() + "-c" + reset + ", " + cyan + "--changes" + reset + "    Show summarised change notes");
            println!("          {}", cyan.to_owned() + "-C" + reset + ", " + cyan + "--citation" + reset + "   Show citation information for the program");
            println!("          {}", cyan.to_owned() + "-h" + reset + ", " + cyan + "--help" + reset + "       Show the help menu");
            println!("          {}", cyan.to_owned() + "-l" + reset + ", " + cyan + "--languages" + reset + "  Show additional language information");
            println!("          {}", cyan.to_owned() + "-L" + reset + ", " + cyan + "--license" + reset + "    Show licensing information");
            println!("          {}", cyan.to_owned() + "-v" + reset + ", " + cyan + "--version" + reset + "    Show the program version");
            exit(0);
        }

        // LANGUAGES

        if argument == "-l" || argument == "--languages" {
            print!("{yellow}");
            println!("English{}", reset.to_owned() + ":  The " + cyan + "--ipa" + reset + " mode presents the pronunciation of Australian English (Canberra, ACT), while the " + cyan + "--ort" + reset + " mode transcribes the text using the spelling of American English.");
            println!();
            print!("{yellow}");
            println!("Latin{}", reset.to_owned() + ":    In the " + cyan + "--ipa" + reset + " mode, either modern or ancient spellings of Latin may be used, and the duration of vowels can be indicated with apices or macrons. In the " + cyan + "--ort" + reset + " mode, the text is transcribed using the ancient spelling convention of Latin (pre-2nd century AD).");
            println!();
            print!("{yellow}");
            println!("Polish{}", reset.to_owned() + ":   Three variants of pronunciation are available (" + cyan + "--ipa" + reset + "): Częstochowa (Małopolska Region), Toruń (Wielkopolska Region), and Warszawa (Mazowsze Region).");
            println!();
            print!("{yellow}");
            println!("Quechua{}", reset.to_owned() + ":  The " + cyan + "--ipa" + reset + " mode implements Ayacucho Quechua and requires inputs to be spelled accordingly. The " + cyan + "--lct" + reset + " mode translates from other Southern Quechua varieties into Ayacucho Quechua. Although the algorithm attempts to convert dialectal features, manual adjustments may be required. The " + cyan + "--ort" + reset + " mode facilitates transcription between the trivocalic and pentavocalic orthographies, and can be used with any language within the Quechuan Family.");
            println!();
            print!("{yellow}");
            println!("Spanish{}", reset.to_owned() + ":  The software provides 15 different pronunciation choices (" + cyan + "--ipa" + reset + "), including 10 from Colombia (Arauca; Bogotá; Bucaramanga; Cali; Leticia; Medellín; Neiva; Pasto; Quibdó; Santa Marta), three from Spain (Bilbao; Cádiz; Madrid), one from Mexico (Ciudad de México), and one from Uruguay (Montevideo).");
            println!();
            print!("{yellow}");
            println!("Tikuna{}", reset.to_owned() + ":   Five pronunciation variants (Río Cotuhé, CO; Cushillococha, PE; Nazareth, CO; Umariaçu, BR; Vila Betânia, BR) are available in the " + cyan + "--ipa" + reset + " mode, and four distinct orthographies (Brazil; Colombia; Peru–ILV; Peru–FORMABIAP) in the " + cyan + "--ort" + reset + " mode. Currently, the program does not offer tone support. However, tonal annotations can be displayed in subscript using manual input as numbers ranging from 1 (lowest tone) to 6 (highest tone) in the " + cyan + "--ipa" + reset + " mode.");
            println!("          Abbreviations: ILV - Instituto Lingüístico de Verano; FORMABIAP - Formación de Maestros Bilingües de la Amazonía Peruana");
            exit(0);
        }

        // LICENSE

        if argument == "-L" || argument == "--license" {
            println!("{}", yellow.to_owned() + "MIT License" + reset);
            println!();
            println!("Copyright © 2022–2023 Piotr Bajdek");
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
            println!("{}", grey.to_owned() + "Version" + reset + ": " + yellow + "1.8.6" + reset);
            println!("March 21, 2023");
            exit(0);
        }
    }
}
