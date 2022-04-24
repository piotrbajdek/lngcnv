# WHAT IS LNGCNV

'lngcnv' may stand for _'language converter'_, _'linguistic converter'_, _'lenguaje convertido'_, and the like.

lngcnv constitutes a set of linguistic tools which allow **(1)** displaying pronunciation of a phrase in the International Phonetic Alphabet, **(2)** translating between different (e.g., dialectal) varieties of a language, and **(3)** converting between different (e.g., regional) norms of orthography when a word or some piece of text is provided either **(a)** as an argument directly from the command line or **(b)** in a file (such like a hundreds of pages long book).

Multiple modern and ancient languages are (projected to be) supported.

# SUPPORTED LANGUAGES

**1. English:** pronunciation & orthography

– Pronunciation of Australian English [experimental, still in alpha stage]

– Transcribe into American English

**2. Latin:** pronunciation & orthography

– Reconstructed pronunciation of Classical Latin

– Transcribe into the ancient orthographic convention (before the 2nd c. AD)

**3. Polish:** pronunciation

– Pronunciation of the Polish Language (Częstochowa)

**4. Quechua:** pronunciation & dialect translation & orthography

– Pronunciation of Ayacucho Quechua (Wanta)

– Translate into Ayacucho Quechua from other varieties of Southern Quechua

– Transcribe between the trivocalic and the pentavocalic orthographies

**5. Tikuna:** pronunciation & orthography

– Two variants of pronunciation (Nazareth, CO; Umariaçu, BR)

– Four distinct orthographies (Brazil; Colombia; Peru–ILV; Peru–FORMABIAP)

Support for more languages including the pronunciation of **Spanish** is under development.

# INSTALLATION ON LINUX

## METHOD 1

**1.** Install from crates.io by the use of cargo:

_cargo install lngcnv_

By default, the file will be downloaded to .cargo/bin/, a hidden folder in your home directory.

**2a.** For convenience, you will probably want to copy lngcnv to /usr/bin/ as in Method 2 (3a, 3b).

**2b.** Alternatively, add ~/.cargo/bin directory to your PATH variable (see documentation of your shell).

## METHOD 2

**1.** Download the binary 'lngcnv' for Linux x86_64 from GitHub:

https://github.com/piotrbajdek/lngcnv

**2.** Make the file executable:

_sudo chmod +x ./lngcnv_

**3a.** Install lngcnv via copying the binary to /usr/bin/

_sudo cp lngcnv /usr/bin/_

**3b.** On Fedora Silverblue / Kinoite:

_sudo cp lngcnv /var/usrlocal/bin/_

## METHOD 3

Download the lngcnv source from GitHub. Then, build and install the program:

_cargo build && sudo cp target/debug/lngcnv /usr/bin/_

# THE OFFICIAL LNGCNV FORUM

The best place to ask questions or share ideas on lngcnv:

https://github.com/piotrbajdek/lngcnv/discussions

# CHECK OUT THE LNGCNV WIKI

https://github.com/piotrbajdek/lngcnv/wiki

# LNGCNV ON CRATES.IO

The Rust community’s crate registry

https://crates.io/crates/lngcnv
