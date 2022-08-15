# OVERVIEW OF LNGCNV

'lngcnv' may stand for _'language converter'_, _'linguistic converter'_, _'lenguaje convertido'_, and the like.

[lngcnv](https://github.com/piotrbajdek/lngcnv) allows **(1)** displaying pronunciation of a phrase in the International Phonetic Alphabet (IPA), **(2)** translating between different (e.g., dialectal) varieties of a language, and **(3)** converting between different (e.g., regional) norms of orthography when a word or some piece of text is provided either **(a)** as an argument directly from the command line or **(b)** in a file, such as a hundreds of pages long book.

Multiple modern and ancient languages are supported, often including several dialects. The program's source code and all linguistic algorithms are created entirely from scratch, and distributed under the MIT license. [lngcnv](https://github.com/piotrbajdek/lngcnv) is written in Rust for high performance, code correctness, and ease of long-term development.

Use Charis SIL, Doulos SIL, or Gentium Plus font for good rendering of characters of the International Phonetic Alphabet.

# SUPPORTED LANGUAGES

**1. English:** pronunciation & orthography

– Pronunciation of Australian English (Sydney, NSW) [experimental, still in alpha stage]

– Transcribe into American English

**2. Latin:** pronunciation & orthography

– Reconstructed pronunciation of Classical Latin

– Transcribe into the ancient orthographic convention (before the 2nd c. AD)

**3. Polish:** pronunciation

– Pronunciation of the Polish Language (Częstochowa)

**4. Quechua:** pronunciation & dialect translation & orthography

– Pronunciation of Chanka/ Ayacucho Quechua (Wanta)

– Translate into Ayacucho Quechua from other varieties of Southern Quechua

– Transcribe between the trivocalic and the pentavocalic orthographies

**5. Spanish:** eight variants of pronunciation

– Pronunciation of Bolivian Spanish (Santa Cruz de la Sierra)

– Pronunciation of Colombian Spanish (Bogotá; Leticia; Medellín; Santa Marta)

– Pronunciation of Mexican Spanish (Ciudad de México)

– Pronunciation of Spanish of Spain (Cádiz; Madrid)

[Some pronunciation variants may be not yet implemented during the alpha phase but all are planned for the release 1.6.0.]

**6. Tikuna/ Ticuna:** pronunciation & orthography

– Five variants of pronunciation (Río Cotuhé, CO; Cushillococha, PE; Nazareth, CO; Umariaçu, BR; Vila Betânia, BR)

– Four distinct orthographies (Brazil; Colombia; Peru–ILV; Peru–FORMABIAP)

# USAGE

[Static link to a changeable image of the _most recent_ version of lngcnv! This may include pre-releases!]

![help-image](https://github.com/piotrbajdek/lngcnv/blob/main/docs/images/help-image.png?raw=true)

# EXAMPLES

[Static links to changeable images of the _most recent_ version of lngcnv! This may include pre-releases!]

![example-image-1](https://github.com/piotrbajdek/lngcnv/blob/main/docs/images/example-image-1.png?raw=true)

![example-image-2](https://github.com/piotrbajdek/lngcnv/blob/main/docs/images/example-image-2.png?raw=true)

# INSTALLATION ON LINUX

[lngcnv](https://github.com/piotrbajdek/lngcnv) should run smoothly on Windows and macOS, and can be installed by the use of [cargo](https://www.rust-lang.org/tools/install). Yet, it is being developed and tested on Fedora Linux.

## METHOD 1

**1.** Install from crates.io by the use of cargo:

_cargo install lngcnv \--version 1.6.0-alpha.4_

By default, the file will be downloaded to `.cargo/bin/`, a hidden folder in your home directory.

**2a.** For convenience, you will probably want to copy lngcnv to `/usr/bin/` as in Method 2 (3a, 3b).

**2b.** Alternatively, add `~/.cargo/bin` directory to your PATH variable (see documentation of your shell).

## METHOD 2

**1.** Download the binary 'lngcnv' for Linux x86_64 from GitHub:

https://github.com/piotrbajdek/lngcnv/releases/tag/v1.6.0-alpha.4

**2.** Make the file executable:

_sudo chmod +x ./lngcnv_

**3a.** Install lngcnv via copying the binary to `/usr/bin/` on most Linux distros:

_sudo cp lngcnv /usr/bin/_

**3b.** On Fedora Silverblue / Kinoite:

_sudo cp lngcnv /var/usrlocal/bin/_

## METHOD 3

Download and unpack the lngcnv source from GitHub. Then, build and install the program:

https://github.com/piotrbajdek/lngcnv/releases/tag/v1.6.0-alpha.4

_cargo build \--release && sudo cp target/release/lngcnv /usr/bin/_

# LNGCNV CRATE ON CRATES.IO

The Rust community’s crate registry

https://crates.io/crates/lngcnv
