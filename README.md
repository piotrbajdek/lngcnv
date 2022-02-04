# WHAT IS LNGCNV

lngcnv may stand for _"language converter"_, _"linguistic converter"_, _"lenguaje convertido"_, and the like.

lngcnv constitutes a set of linguistic tools which allow **(a)** displaying pronunciation of a phrase and **(b)** converting between different (e.g., regional) norms of orthography when a word or some piece of text is provided as an argument directly from the command line.

Multiple modern and ancient languages are (projected to be) supported.

# SUPPORTED LANGUAGES

**1. English:** orthography

**2. Latin:** pronunciation & orthography

**3. Quechua:** orthography

**4. Tikuna:** pronunciation & orthography

Support for more languages (including **Polish**, **Spanish**) is under active development.

# INSTALLATION ON LINUX

**METHOD 1**

Install from crates.io by the use of cargo:

_sudo cargo install lngcnv_

**METHOD 2**

**1.** Download the binary "lngcnv" for Linux x86_64 from GitHub.

**2.** Make the file executable:

_sudo chmod +x ./lngcnv_

**3a.** Install lngcnv via copying the binary to /usr/bin/

_sudo cp lngcnv /usr/bin/_

**3b.** On Fedora Silverblue / Kinoite:

_sudo cp lngcnv /var/usrlocal/bin/_

**METHOD 3**

Download the lngcnv source from GitHub, build, rename and install the program:

_rustc ./main.rs_

_mv main lngcnv_

_sudo cp lngcnv /usr/bin/_

# CHECK OUT THE LNGCNV WIKI:

https://github.com/piotrbajdek/lngcnv/wiki
