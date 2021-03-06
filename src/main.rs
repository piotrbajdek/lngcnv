// LNGCNV VERSION 1.6.0-ALPHA.3 / MIT LICENSE © 2022 PIOTR BAJDEK

// MAIN FILE

use std::env;
use std::fs;

fn main() {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let blue_underlined = "\x1b[34;4m";
   let cyan = "\x1b[36m";
   let grey = "\x1b[38;5;240m";

// ARGUMENTS ANYWHERE WITHIN THE STRING

   for argument in env::args() {

// ABOUT

      if argument == "-a" || argument == "--about" {
      println!("Program:  {}", yellow.to_owned() + "lngcnv" + reset);
      println!("Version:  1.6.0-alpha.3");
      println!("Date:     July 17, 2022");
      println!("Author:   Piotr Bajdek (Poland)");
      println!("Contact:  {}", blue_underlined.to_owned() + "piotr.bajdek@proton.me" + reset);
      println!("Source:   {}", blue_underlined.to_owned() + "https://github.com/piotrbajdek/lngcnv" + reset);
      println!("License:  MIT License © 2022 Piotr Bajdek");
      return;
      }

// CHANGES

      if argument == "-c" || argument == "--changes" {
      println!("{}", cyan.to_owned() + "--.--.---- v1.6.0 – Added --ipa --spa; Enhanced --ipa --tca; Enhanced source code" + reset);
      println!("15.05.2022 v1.5.1 – Improved source code");
      println!("14.05.2022 v1.5.0 – Enhanced --ipa --tca; Enhanced source code");
      println!("09.05.2022 v1.4.1 – Improved --ipa --eng, --ipa --lat, --lct --que, --ort --eng");
      println!("03.05.2022 v1.4.0 – Added --ipa --eng; Enhanced --ipa --tca, --ort --eng; Improved --ipa --lat");
      println!("14.02.2022 v1.3.0 – Added --lct --que; Added --input --output");
      println!("11.02.2022 v1.2.0 – Added --ipa --que");
      println!("09.02.2022 v1.1.0 – Added --ipa --pol");
      println!("07.02.2022 v1.0.0 – Implemented --ipa --lat, --ipa --tca, --ort --eng, --ort --lat, --ort --que, --ort --tca");
      return;
      }

// HELP

      if argument == "-h" || argument == "--help" {
      println!("Usage:{}", yellow.to_owned() + "    lngcnv [OPTION 1/2] [OPTION 2/1] [STRING]");
      println!("          lngcnv [OPTION 1/2] [OPTION 2/1] [OPTION 3] [FILE] [OPTION 4] [FILE]");
      println!("{}", reset);
      println!("OPTION 1: {}", cyan.to_owned() + "--ipa" + reset + "  Transcribe [STRING/ FILE] into the Intern. Phonetic Alphabet");
      println!("          {}", cyan.to_owned() + "--lct" + reset + "  Translate [STRING/ FILE] between dialects or lang. varieties");
      println!("          {}", cyan.to_owned() + "--ort" + reset + "  Convert [STRING/ FILE] between regional norms of orthography");
      println!("");
      println!("OPTION 2: {}", cyan.to_owned() + "--eng" + reset + "  English | available for: " + cyan + "--ipa" + reset + "," + cyan + "        --ort");
      println!("          --lat{}", reset.to_owned() + "  Latin   | available for: " + cyan + "--ipa" + reset + "," + cyan + "        --ort");
      println!("          --pol{}", reset.to_owned() + "  Polish  | available for: " + cyan + "--ipa");
      println!("          --que{}", reset.to_owned() + "  Quechua | available for: " + cyan + "--ipa" + reset + ", " + cyan + "--lct" + reset + ", " + cyan + "--ort");
      println!("          --spa{}", reset.to_owned() + "  Spanish | available for: " + cyan + "--ipa");
      println!("          --tca{}", reset.to_owned() + "  Tikuna  | available for: " + cyan + "--ipa" + reset + ", " + cyan + "       --ort" + reset);
      println!("");
      println!("Examples:{}", yellow.to_owned() + r#" lngcnv --ipa --lat "Vēnī, vīdī, vīcī.""#);
      println!("          lngcnv --lat --ipa 'Vēnī, vīdī, vīcī.'");
      println!("{}", reset);
      println!("          For multi-word queries always use single or double quotation marks!");
      println!("          For word strings including apostrophes, use double quotation.");
      println!("");
      println!("          In the {}", cyan.to_owned() + "--ipa" + reset + " mode, the output can also be constrained to a selected");
      println!("          variant of pronunciation if two or more are available (see {}", cyan.to_owned() + "-l" + reset + "):");
      println!("");
      println!("          {}", cyan.to_owned() + "--spa" + reset + ": " + grey + "--spa.bo-santa_cruz" + reset + ", " + grey + "--spa.co-bogota" + reset + ", " + grey + "--spa.co-leticia" + reset + ",");
      println!("                 {}", cyan.to_owned() + "--spa.co-medellin" + reset + ", " + grey + "--spa.co-santa_marta" + reset + ", " + grey + "--spa.es-cadiz" + reset + ",");
      println!("                 {}", cyan.to_owned() + "--spa.es-madrid" + reset + ", " + grey + "--spa.mx-ciudad_de_mexico" + cyan);
      println!("          --tca{}", reset.to_owned() + ": " + cyan + "--tca.br-umariacu" + reset + ", " + cyan + "--tca.br-vila_betania" + reset + ", " + cyan + "--tca.co-nazareth" + reset + ",");
      println!("                 {}", cyan.to_owned() + "--tca.co-rio_cotuhe" + reset + ", " + cyan + "--tca.pe-cushillococha" + reset);
      println!("{}", grey);
      println!("                 In grey features not yet implemented during the alpha phase!");
      println!("{}", reset);
      println!("Examples:{}", yellow.to_owned() + " lngcnv --ipa --spa 'Una frase en español'" + reset + "    [display all variants]");
      print!("{}", yellow);
      println!("          lngcnv --ipa --spa.co-bogota 'Una frase en español'{}", reset.to_owned() + "    [Bogotá, CO]");
      println!("");
      println!("OPTION 3:{}", cyan.to_owned() + " -i" + reset + ", " + cyan + "--input" + reset + "      Choose a file to be opened for analysis");
      println!("OPTION 4:{}", cyan.to_owned() +  " -o" + reset + ", " + cyan + "--output" + reset + "     Choose a name for the file to be created");
      println!("");
      println!("Examples:{}", yellow.to_owned() + " lngcnv --ipa --lat --input latin_book.txt --output output_file.txt");
      println!("          lngcnv --lat --ipa --input latin_book.txt --output output_file.txt");
      println!("{}", reset);
      println!("See also: {}", cyan.to_owned() + "-a" + reset + ", " + cyan + "--about" + reset + "      Show contact and program info");
      println!("          {}", cyan.to_owned() + "-c" + reset + ", " + cyan + "--changes" + reset + "    Show simplified change notes");
      println!("          {}", cyan.to_owned() + "-h" + reset + ", " + cyan + "--help" + reset + "       Show this help");
      println!("          {}", cyan.to_owned() + "-l" + reset + ", " + cyan + "--languages" + reset + "  Show additional info");
      println!("          {}", cyan.to_owned() + "-L" + reset + ", " + cyan + "--license" + reset + "    Show licesing information");
      println!("          {}", cyan.to_owned() + "-v" + reset + ", " + cyan + "--version" + reset + "    Show the program version");
      return;
      }

// LANGUAGES

      if argument == "-l" || argument == "--languages" {
      print!("{}", yellow);
      println!("English:{}", reset.to_owned() + "  The " + cyan + "--ipa" + reset + " mode of operation displays the pronunciation of Australian English " + red + "[experimental, still in alpha stage]" + reset + ". In the " + cyan + "--ort" + reset + " mode, the text is transcribed into American English.");
      println!("");
      print!("{}", yellow);
      println!("Latin:{}", reset.to_owned() + "    The modern spelling of Latin and the ancient convention can be used indifferently in the " + cyan + "--ipa" + reset + " mode. To mark the duration of vowels, type either apices or macrons. In the " + cyan + "--ort" + reset + " mode, the text is transcribed into the ancient convention (before the 2nd c. AD).");
      println!("");
      print!("{}", yellow);
      println!("Polish:{}", reset.to_owned() + "   Częstochowa (south-central Poland) is characterised by a fairly 'neutral-sounding' accent (" + cyan + "--ipa" + reset + ").");
      println!("");
      print!("{}", yellow);
      println!("Quechua:{}", reset.to_owned() + "  In the " + cyan + "--ipa" + reset + " mode, Ayacucho Quechua is implemented and the input must be spelled accordingly. The " + cyan + "--lct" + reset + " mode of operation translates into Ayacucho Quechua from other varieties of Southen Quechua. Manual adjustments are necessary despite dialectal features are mostly converted by the algorithm. The " + cyan + "--ort" + reset + " mode allows transcribing between the trivocalic and the pentavocalic orthographies and works fine with any language of the Quechuan Family.");
      println!("");
      print!("{}", yellow);
      println!("Spanish:{}", reset.to_owned() + "  Eight variants of pronunciation are available (" + cyan + "--ipa" + reset + "), including one from Bolivia (Santa Cruz de la Sierra), four from Colombia (Bogotá; Leticia; Medellín; Santa Marta), one from Mexico (Ciudad de México), and two from Spain (Cádiz; Madrid).");
      println!("");
      print!("{}", yellow);
      println!("Tikuna:{}", reset.to_owned() + "   Five variants of pronunciation (Río Cotuhé, CO; Cushillococha, PE; Nazareth, CO; Umariaçu, BR; Vila Betânia, BR) (" + cyan + "--ipa" + reset + ") and four distinct orthographies (Brazil; Colombia; Peru–ILV; Peru–FORMABIAP) (" + cyan + "--ort" + reset + ") are supported. There is no support for tones at this stage of program development. Even so, in the " + cyan + "--ipa" + reset + " mode tonal annotations can be displayed in subscript if inserted manually as numbers beginning from 1 (the lowest) to 6 (the highest tone).");
      println!("          Abbreviations: ILV - Instituto Lingüístico de Verano; FORMABIAP - Formación de Maestros Bilingües de la Amazonía Peruana");
      return;
      }

// LICENSE

      if argument == "-L" || argument == "--license" {
      println!("{}", yellow.to_owned() + "MIT License" + reset);
      println!("");
      println!("Copyright © 2022 Piotr Bajdek");
      println!("");
      println!("Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:");
      println!("");
      println!("The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.");
      println!("");
      println!("THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.");
      return;
      }

// VERSION

      if argument == "-v" || argument == "--version" {
      println!("Version: {}", yellow.to_owned() + "1.6.0-alpha.3" + reset);
      println!("July 17, 2022");
      return;
      }

   }

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

   lngcnv::ipaaueng(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: ENGLISH IPA

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::ipaaueng(&original_text, usefile, outputfile);
   println!("");
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

   lngcnv::ortuseng(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: ENGLISH ORTHOGRAPHY

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::ortuseng(&original_text, usefile, outputfile);
   println!("");
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

   lngcnv::ipalat(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: LATIN IPA

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::ipalat(&original_text, usefile, outputfile);
   println!("");
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

   lngcnv::ortlat(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: LATIN ORTHOGRAPHY

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::ortlat(&original_text, usefile, outputfile);
   println!("");
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

   lngcnv::ipapol(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: POLISH IPA

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::ipapol(&original_text, usefile, outputfile);
   println!("");
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

   lngcnv::ipaque(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: AYACUCHO QUECHUA IPA

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::ipaque(&original_text, usefile, outputfile);
   println!("");
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

   lngcnv::quelct(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: AYACUCHO QUECHUA DIALECT

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::quelct(&original_text, usefile, outputfile);
   println!("");
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

   lngcnv::ortquetri(&original_text, usefile, outputfile);

   let usefile = "old";

   lngcnv::ortquepen(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: QUECHUA ORTHOGRAPHY

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::ortquetri(&original_text, usefile, outputfile);
   lngcnv::ortquepen(&original_text, usefile, outputfile);
   println!("");
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

   lngcnv::spabosantacruz(&original_text, usefile, outputfile);

   let usefile = "old";

   lngcnv::spacobogota(&original_text, usefile, outputfile);
   lngcnv::spacoleticia(&original_text, usefile, outputfile);
   lngcnv::spacomedellin(&original_text, usefile, outputfile);
   lngcnv::spacosantamarta(&original_text, usefile, outputfile);
   lngcnv::spamxciudaddemexico(&original_text, usefile, outputfile);
   lngcnv::spaescadiz(&original_text, usefile, outputfile);
   lngcnv::spaesmadrid(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: SPANISH IPA

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::spabosantacruz(&original_text, usefile, outputfile);
   lngcnv::spacobogota(&original_text, usefile, outputfile);
   lngcnv::spacoleticia(&original_text, usefile, outputfile);
   lngcnv::spacomedellin(&original_text, usefile, outputfile);
   lngcnv::spacosantamarta(&original_text, usefile, outputfile);
   lngcnv::spamxciudaddemexico(&original_text, usefile, outputfile);
   lngcnv::spaescadiz(&original_text, usefile, outputfile);
   lngcnv::spaesmadrid(&original_text, usefile, outputfile);
   println!("");
   return;
   }

//   ++++++++++   ++++++++++   ++++++++++

// SPANISH: IPA --spa.bo-santa_cruz

   if input1 == "--ipa" && input2 == "--spa.bo-santa_cruz" || input1 == "--spa.bo-santa_cruz" && input2 == "--ipa" {

   let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
// FROM A FILE:  SANTA CRUZ DE LA SIERRA
   if str0 == "-i" || str0 == "--input" {
   let usefile = "new";
   let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));      
   let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));      
   if output == "-o" || output == "--output" {
   let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
   let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

   lngcnv::spabosantacruz(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: SANTA CRUZ DE LA SIERRA

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::spabosantacruz(&original_text, usefile, outputfile);
   println!("");
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

   lngcnv::spacobogota(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: BOGOTÁ

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::spacobogota(&original_text, usefile, outputfile);
   println!("");
   return;
   }

//   ++++++++++   ++++++++++   ++++++++++

// SPANISH: IPA --spa.co-leticia

   if input1 == "--ipa" && input2 == "--spa.co-leticia" || input1 == "--spa.co-leticia" && input2 == "--ipa" {

   let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
// FROM A FILE: LETICIA
   if str0 == "-i" || str0 == "--input" {
   let usefile = "new";
   let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));      
   let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));      
   if output == "-o" || output == "--output" {
   let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
   let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

   lngcnv::spacoleticia(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: LETICIA

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::spacoleticia(&original_text, usefile, outputfile);
   println!("");
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

   lngcnv::spacomedellin(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: MEDELLÍN
   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::spacomedellin(&original_text, usefile, outputfile);
   println!("");
   return;
   }

//   ++++++++++   ++++++++++   ++++++++++

// SPANISH: IPA --spa.co-santa_marta

   if input1 == "--ipa" && input2 == "--spa.co-santa_marta" || input1 == "--spa.co-santa_marta" && input2 == "--ipa" {

   let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
// FROM A FILE: SANTA MARTA
   if str0 == "-i" || str0 == "--input" {
   let usefile = "new";
   let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));      
   let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));      
   if output == "-o" || output == "--output" {
   let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
   let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

   lngcnv::spacosantamarta(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: SANTA MARTA

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::spacosantamarta(&original_text, usefile, outputfile);
   println!("");
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

   lngcnv::spaescadiz(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: CÁDIZ

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::spaescadiz(&original_text, usefile, outputfile);
   println!("");
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

   lngcnv::spaesmadrid(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: MADRID

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::spaesmadrid(&original_text, usefile, outputfile);
   println!("");
   return;
   }

//   ++++++++++   ++++++++++   ++++++++++

// SPANISH: IPA --spa.mx-ciudad_de_mexico

   if input1 == "--ipa" && input2 == "--spa.mx-ciudad_de_mexico" || input1 == "--spa.mx-ciudad_de_mexico" && input2 == "--ipa" {

   let str0 = args.get(3).expect(&(red.to_owned() + "No string inserted! See: --help" + reset));
// FROM A FILE: CIUDAD DE MÉXICO:
   if str0 == "-i" || str0 == "--input" {
   let usefile = "new";
   let inputfile = args.get(4).expect(&(red.to_owned() + "No file to read! See: --help" + reset));      
   let output = args.get(5).expect(&(red.to_owned() + "Invalid arguments! Use option: --output See: --help" + reset));      
   if output == "-o" || output == "--output" {
   let outputfile = args.get(6).expect(&(red.to_owned() + "No output file specified! See: --help" + reset));
   let original_text = fs::read_to_string(inputfile).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));

   lngcnv::spamxciudaddemexico(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: CIUDAD DE MÉXICO:

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::spamxciudaddemexico(&original_text, usefile, outputfile);
   println!("");
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

   lngcnv::tcacoriocotuhe(&original_text, usefile, outputfile);

   let usefile = "old";

   lngcnv::tcapecushillococha(&original_text, usefile, outputfile);
   lngcnv::tcaconazareth(&original_text, usefile, outputfile);
   lngcnv::tcabrumariacu(&original_text, usefile, outputfile);
   lngcnv::tcabrvilabetania(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: TIKUNA IPA

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::tcacoriocotuhe(&original_text, usefile, outputfile);
   lngcnv::tcapecushillococha(&original_text, usefile, outputfile);
   lngcnv::tcaconazareth(&original_text, usefile, outputfile);
   lngcnv::tcabrumariacu(&original_text, usefile, outputfile);
   lngcnv::tcabrvilabetania(&original_text, usefile, outputfile);
   println!("");
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

   lngcnv::tcabrumariacu(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: UMARIAÇU

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::tcabrumariacu(&original_text, usefile, outputfile);
   println!("");
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

   lngcnv::tcabrvilabetania(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: VILA BETÂNIA

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::tcabrvilabetania(&original_text, usefile, outputfile);
   println!("");
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

   lngcnv::tcaconazareth(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: NAZARETH

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::tcaconazareth(&original_text, usefile, outputfile);
   println!("");
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

   lngcnv::tcacoriocotuhe(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: RIO COTUHÉ

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::tcacoriocotuhe(&original_text, usefile, outputfile);
   println!("");
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

   lngcnv::tcapecushillococha(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: CUSHILLOCOCHA

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::tcapecushillococha(&original_text, usefile, outputfile);
   println!("");
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

   lngcnv::orttcabr(&original_text, usefile, outputfile);

   let usefile = "old";

   lngcnv::orttcaco(&original_text, usefile, outputfile);
   lngcnv::orttcapeilv(&original_text, usefile, outputfile);
   lngcnv::orttcapeformabiap(&original_text, usefile, outputfile);

   println!("Data written to the file {}", outputfile);
   return;
   }
   }
// FROM THE COMMAND LINE: TIKUNA ORTHOGRAPHY

   let original_text = str0;
   let usefile = "terminal";
   let outputfile = "0";
   lngcnv::orttcabr(&original_text, usefile, outputfile);
   lngcnv::orttcaco(&original_text, usefile, outputfile);
   lngcnv::orttcapeilv(&original_text, usefile, outputfile);
   lngcnv::orttcapeformabiap(&original_text, usefile, outputfile);
   println!("");
   return;
   }

//   ++++++++++   ++++++++++   ++++++++++

// INVALID ARGUMENT [1] AND/OR [2]

   panic!("{}", red.to_owned() + "Invalid arguments provided! See: --help" + reset);

}
