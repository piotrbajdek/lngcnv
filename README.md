# OVERVIEW OF LNGCNV

'lngcnv' may stand for _'language converter'_, _'linguistic converter'_, _'lenguaje convertido'_, and the like.

[lngcnv](https://github.com/piotrbajdek/lngcnv) **(1)** displays the exact phonetic (not just phonemic) pronunciation of a phrase in the International Phonetic Alphabet (IPA), **(2)** translates between different (e.g., dialectal) varieties of a language, and **(3)** converts between different (e.g., regional) norms of orthography.

A word or some piece of text can be provided either **(a)** as an argument directly from the command line or **(b)** in a file, such as a hundreds of pages long book.

Six modern and ancient languages are currently supported, often including several dialects.

The program's source code and all linguistic algorithms have been created entirely from scratch, and are distributed under the terms of the [MIT License](https://github.com/piotrbajdek/lngcnv/blob/main/LICENSE.md). [lngcnv](https://github.com/piotrbajdek/lngcnv) is written in Rust for high performance, code correctness, and ease of long-term development.

Use [Charis SIL](https://software.sil.org/charis/download/), [Doulos SIL](https://software.sil.org/doulos/download/), or [Gentium Plus](https://software.sil.org/gentium/download/) font for good rendering of characters of the International Phonetic Alphabet (see [1](https://www.internationalphoneticassociation.org/IPAcharts/inter_chart_2018/IPA_2018.html), [2](https://ipahelp.languagetechnology.org/), [3](https://en.wikipedia.org/wiki/IPA_vowel_chart_with_audio), [4](https://en.wikipedia.org/wiki/IPA_pulmonic_consonant_chart_with_audio)).

# SUPPORTED LANGUAGES

**1. English:** pronunciation & orthography

– Pronunciation of Australian English (Canberra, ACT)

– Transcribe into American English

**2. Latin:** pronunciation & orthography

– Reconstructed pronunciation of Classical Latin

– Transcribe into the ancient orthographic convention (before the 2nd c. AD)

**3. Polish:** three variants of pronunciation

– Pronunciation from Częstochowa, Małopolska Region

– Pronunciation from Toruń, Wielkopolska Region

– Pronunciation from Warszawa, Mazowsze Region

**4. Quechua:** pronunciation & dialect translation & orthography

– Pronunciation of Chanka/ Ayacucho Quechua (Wanta)

– Translate into Ayacucho Quechua from other varieties of Southern Quechua

– Transcribe between the trivocalic and the pentavocalic orthographies

**5. Spanish:** five variants of pronunciation

– Pronunciation of Colombian Spanish (Bogotá; Medellín)

– Pronunciation of Mexican Spanish (Ciudad de México)

– Pronunciation of Spanish of Spain (Cádiz; Madrid)

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

[lngcnv](https://github.com/piotrbajdek/lngcnv) should run smoothly on **Windows** and **macOS**, and can be installed by the use of [cargo](https://www.rust-lang.org/tools/install). Yet, it is being developed and primarily tested on **Fedora Linux**.

lngcnv v1.6.0-beta.9:

– Was successfully tested on Arch Linux, Fedora Linux 37, Rocky Linux 9, openSUSE Tumbleweed, Ubuntu 22.04 and Ubuntu 22.10.

– Failed to run on Debian 11 and openSUSE Leap 15.4 due to an old glibc version (required >2.34).

## METHOD 1 – BY THE USE OF CARGO

**1.** Install from crates.io by the use of cargo:

_cargo install lngcnv \--version 1.6.0-beta.9_

By default, the file will be downloaded to `.cargo/bin/`, a hidden folder in your home directory.

**2a.** For convenience, you will probably want to copy lngcnv to `/usr/bin/` as in Method 2 (3a, 3b).

**2b.** Alternatively, add `~/.cargo/bin` directory to your PATH variable (can be set up by [rustup](https://www.rust-lang.org/tools/install)).

## METHOD 2 – LINUX UNIVERSAL BINARIES

**1.** Download the distro-independent [binary](https://github.com/piotrbajdek/lngcnv/releases/download/v1.6.0-beta.9/lngcnv) of lngcnv from GitHub.

**2.** Make the file executable:

_sudo chmod +x ./lngcnv_

**3a.** On most Linux distros, install lngcnv via copying the binary to `/usr/bin/`:

_sudo cp lngcnv /usr/bin/_

**3b.** On Fedora Silverblue / Kinoite:

_sudo cp lngcnv /var/usrlocal/bin/_

## METHOD 3 – DISTRO-SPECIFIC PACKAGES

Distro-specific packages are also available for download for [.rpm](https://github.com/piotrbajdek/lngcnv/releases/download/v1.6.0-beta.9/lngcnv-1.6.0-beta.9-1.x86_64.rpm)- and [.deb](https://github.com/piotrbajdek/lngcnv/releases/download/v1.6.0-beta.9/lngcnv_1.6.0~beta.9_amd64.deb)-based Linux distros. Installation instructions:

Fedora Linux / RHEL / openSUSE:

_sudo rpm -i lngcnv-1.6.0-beta.9-1.x86_64.rpm_

Fedora Silverblue / Kinoite:

_rpm-ostree install lngcnv-1.6.0-beta.9-1.x86_64.rpm_

Ubuntu:

_sudo dpkg -i lngcnv_1.6.0~beta.9_amd64.deb_

## METHOD 4 – MANUAL COMPILATION

Download and unpack the lngcnv [source](https://github.com/piotrbajdek/lngcnv/archive/refs/tags/v1.6.0-beta.9.zip) from GitHub. Then, build and install the program:

_cargo build \--release && sudo cp target/release/lngcnv /usr/bin/_

# LNGCNV CRATE ON CRATES.IO

The Rust community’s crate registry

https://crates.io/crates/lngcnv/1.6.0-beta.9
