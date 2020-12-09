Summary: GNU Project parser generator (yacc replacement)
Name: bison
Version: 3.5.3
Release: r0
License: GPLv3
Group: devel
Packager: Poky <poky@lists.yoctoproject.org>
URL: http://www.gnu.org/software/bison/
BuildRequires: autoconf-native
BuildRequires: automake-native
BuildRequires: bison-native
BuildRequires: flex-native
BuildRequires: gettext-native
BuildRequires: gnu-config-native
BuildRequires: libtool-cross
BuildRequires: libtool-native
BuildRequires: texinfo-dummy-native
BuildRequires: virtual/libc
BuildRequires: virtual/x86_64-poky-linux-compilerlibs
BuildRequires: virtual/x86_64-poky-linux-gcc
Requires: /bin/sh
Requires: libc.so.6()(64bit)
Requires: libc.so.6(GLIBC_2.14)(64bit)
Requires: libc.so.6(GLIBC_2.15)(64bit)
Requires: libc.so.6(GLIBC_2.17)(64bit)
Requires: libc.so.6(GLIBC_2.2.5)(64bit)
Requires: libc.so.6(GLIBC_2.3)(64bit)
Requires: libc.so.6(GLIBC_2.3.4)(64bit)
Requires: libc.so.6(GLIBC_2.4)(64bit)
Requires: libc.so.6(GLIBC_2.8)(64bit)
Requires: libc6 >= 2.31+git0+6fdf971c9d
Requires: rtld(GNU_HASH)

%description
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.

%package -n bison-src
Summary: GNU Project parser generator (yacc replacement) - Source files
License: GPLv3
Group: devel

%description -n bison-src
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains sources for debugging
purposes.

%package -n bison-dbg
Summary: GNU Project parser generator (yacc replacement) - Debugging files
License: GPLv3
Group: devel
Recommends: glibc-dbg

%description -n bison-dbg
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains ELF symbols and
related sources for debugging purposes.

%package -n bison-staticdev
Summary: GNU Project parser generator (yacc replacement) - Development files (Static Libraries)
License: GPLv3
Group: devel
Requires: bison-dev = 3.5.3-r0

%description -n bison-staticdev
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains static libraries for
software development.

%package -n bison-dev
Summary: GNU Project parser generator (yacc replacement) - Development files
License: GPLv3
Group: devel
Requires: bison = 3.5.3-r0
Recommends: glibc-dev

%description -n bison-dev
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains symbolic links,
header files, and related items necessary for software development.

%package -n bison-doc
Summary: GNU Project parser generator (yacc replacement) - Documentation files
License: GPLv3
Group: doc

%description -n bison-doc
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains documentation.

%package -n bison-locale-af
Summary: GNU Project parser generator (yacc replacement) - af translations
License: GPLv3
Group: devel
Recommends: virtual-locale-af
Provides: af-translation
Provides: bison-locale

%description -n bison-locale-af
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the af locale.

%package -n bison-locale-ast
Summary: GNU Project parser generator (yacc replacement) - ast translations
License: GPLv3
Group: devel
Recommends: virtual-locale-ast
Provides: ast-translation
Provides: bison-locale

%description -n bison-locale-ast
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the ast locale.

%package -n bison-locale-be
Summary: GNU Project parser generator (yacc replacement) - be translations
License: GPLv3
Group: devel
Recommends: virtual-locale-be
Provides: be-translation
Provides: bison-locale

%description -n bison-locale-be
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the be locale.

%package -n bison-locale-bg
Summary: GNU Project parser generator (yacc replacement) - bg translations
License: GPLv3
Group: devel
Recommends: virtual-locale-bg
Provides: bg-translation
Provides: bison-locale

%description -n bison-locale-bg
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the bg locale.

%package -n bison-locale-ca
Summary: GNU Project parser generator (yacc replacement) - ca translations
License: GPLv3
Group: devel
Recommends: virtual-locale-ca
Provides: bison-locale
Provides: ca-translation

%description -n bison-locale-ca
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the ca locale.

%package -n bison-locale-cs
Summary: GNU Project parser generator (yacc replacement) - cs translations
License: GPLv3
Group: devel
Recommends: virtual-locale-cs
Provides: bison-locale
Provides: cs-translation

%description -n bison-locale-cs
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the cs locale.

%package -n bison-locale-da
Summary: GNU Project parser generator (yacc replacement) - da translations
License: GPLv3
Group: devel
Recommends: virtual-locale-da
Provides: bison-locale
Provides: da-translation

%description -n bison-locale-da
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the da locale.

%package -n bison-locale-de
Summary: GNU Project parser generator (yacc replacement) - de translations
License: GPLv3
Group: devel
Recommends: virtual-locale-de
Provides: bison-locale
Provides: de-translation

%description -n bison-locale-de
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the de locale.

%package -n bison-locale-el
Summary: GNU Project parser generator (yacc replacement) - el translations
License: GPLv3
Group: devel
Recommends: virtual-locale-el
Provides: bison-locale
Provides: el-translation

%description -n bison-locale-el
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the el locale.

%package -n bison-locale-eo
Summary: GNU Project parser generator (yacc replacement) - eo translations
License: GPLv3
Group: devel
Recommends: virtual-locale-eo
Provides: bison-locale
Provides: eo-translation

%description -n bison-locale-eo
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the eo locale.

%package -n bison-locale-es
Summary: GNU Project parser generator (yacc replacement) - es translations
License: GPLv3
Group: devel
Recommends: virtual-locale-es
Provides: bison-locale
Provides: es-translation

%description -n bison-locale-es
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the es locale.

%package -n bison-locale-et
Summary: GNU Project parser generator (yacc replacement) - et translations
License: GPLv3
Group: devel
Recommends: virtual-locale-et
Provides: bison-locale
Provides: et-translation

%description -n bison-locale-et
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the et locale.

%package -n bison-locale-eu
Summary: GNU Project parser generator (yacc replacement) - eu translations
License: GPLv3
Group: devel
Recommends: virtual-locale-eu
Provides: bison-locale
Provides: eu-translation

%description -n bison-locale-eu
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the eu locale.

%package -n bison-locale-fi
Summary: GNU Project parser generator (yacc replacement) - fi translations
License: GPLv3
Group: devel
Recommends: virtual-locale-fi
Provides: bison-locale
Provides: fi-translation

%description -n bison-locale-fi
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the fi locale.

%package -n bison-locale-fr
Summary: GNU Project parser generator (yacc replacement) - fr translations
License: GPLv3
Group: devel
Recommends: virtual-locale-fr
Provides: bison-locale
Provides: fr-translation

%description -n bison-locale-fr
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the fr locale.

%package -n bison-locale-ga
Summary: GNU Project parser generator (yacc replacement) - ga translations
License: GPLv3
Group: devel
Recommends: virtual-locale-ga
Provides: bison-locale
Provides: ga-translation

%description -n bison-locale-ga
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the ga locale.

%package -n bison-locale-gl
Summary: GNU Project parser generator (yacc replacement) - gl translations
License: GPLv3
Group: devel
Recommends: virtual-locale-gl
Provides: bison-locale
Provides: gl-translation

%description -n bison-locale-gl
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the gl locale.

%package -n bison-locale-hr
Summary: GNU Project parser generator (yacc replacement) - hr translations
License: GPLv3
Group: devel
Recommends: virtual-locale-hr
Provides: bison-locale
Provides: hr-translation

%description -n bison-locale-hr
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the hr locale.

%package -n bison-locale-hu
Summary: GNU Project parser generator (yacc replacement) - hu translations
License: GPLv3
Group: devel
Recommends: virtual-locale-hu
Provides: bison-locale
Provides: hu-translation

%description -n bison-locale-hu
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the hu locale.

%package -n bison-locale-ia
Summary: GNU Project parser generator (yacc replacement) - ia translations
License: GPLv3
Group: devel
Recommends: virtual-locale-ia
Provides: bison-locale
Provides: ia-translation

%description -n bison-locale-ia
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the ia locale.

%package -n bison-locale-id
Summary: GNU Project parser generator (yacc replacement) - id translations
License: GPLv3
Group: devel
Recommends: virtual-locale-id
Provides: bison-locale
Provides: id-translation

%description -n bison-locale-id
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the id locale.

%package -n bison-locale-it
Summary: GNU Project parser generator (yacc replacement) - it translations
License: GPLv3
Group: devel
Recommends: virtual-locale-it
Provides: bison-locale
Provides: it-translation

%description -n bison-locale-it
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the it locale.

%package -n bison-locale-ja
Summary: GNU Project parser generator (yacc replacement) - ja translations
License: GPLv3
Group: devel
Recommends: virtual-locale-ja
Provides: bison-locale
Provides: ja-translation

%description -n bison-locale-ja
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the ja locale.

%package -n bison-locale-ko
Summary: GNU Project parser generator (yacc replacement) - ko translations
License: GPLv3
Group: devel
Recommends: virtual-locale-ko
Provides: bison-locale
Provides: ko-translation

%description -n bison-locale-ko
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the ko locale.

%package -n bison-locale-ky
Summary: GNU Project parser generator (yacc replacement) - ky translations
License: GPLv3
Group: devel
Recommends: virtual-locale-ky
Provides: bison-locale
Provides: ky-translation

%description -n bison-locale-ky
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the ky locale.

%package -n bison-locale-lt
Summary: GNU Project parser generator (yacc replacement) - lt translations
License: GPLv3
Group: devel
Recommends: virtual-locale-lt
Provides: bison-locale
Provides: lt-translation

%description -n bison-locale-lt
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the lt locale.

%package -n bison-locale-lv
Summary: GNU Project parser generator (yacc replacement) - lv translations
License: GPLv3
Group: devel
Recommends: virtual-locale-lv
Provides: bison-locale
Provides: lv-translation

%description -n bison-locale-lv
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the lv locale.

%package -n bison-locale-ms
Summary: GNU Project parser generator (yacc replacement) - ms translations
License: GPLv3
Group: devel
Recommends: virtual-locale-ms
Provides: bison-locale
Provides: ms-translation

%description -n bison-locale-ms
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the ms locale.

%package -n bison-locale-nb
Summary: GNU Project parser generator (yacc replacement) - nb translations
License: GPLv3
Group: devel
Recommends: virtual-locale-nb
Provides: bison-locale
Provides: nb-translation

%description -n bison-locale-nb
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the nb locale.

%package -n bison-locale-nl
Summary: GNU Project parser generator (yacc replacement) - nl translations
License: GPLv3
Group: devel
Recommends: virtual-locale-nl
Provides: bison-locale
Provides: nl-translation

%description -n bison-locale-nl
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the nl locale.

%package -n bison-locale-pl
Summary: GNU Project parser generator (yacc replacement) - pl translations
License: GPLv3
Group: devel
Recommends: virtual-locale-pl
Provides: bison-locale
Provides: pl-translation

%description -n bison-locale-pl
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the pl locale.

%package -n bison-locale-pt
Summary: GNU Project parser generator (yacc replacement) - pt translations
License: GPLv3
Group: devel
Recommends: virtual-locale-pt
Provides: bison-locale
Provides: pt-translation

%description -n bison-locale-pt
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the pt locale.

%package -n bison-locale-pt-br
Summary: GNU Project parser generator (yacc replacement) - pt_BR translations
License: GPLv3
Group: devel
Recommends: virtual-locale-pt-br
Provides: bison-locale
Provides: pt-br-translation

%description -n bison-locale-pt-br
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the pt_BR locale.

%package -n bison-locale-ro
Summary: GNU Project parser generator (yacc replacement) - ro translations
License: GPLv3
Group: devel
Recommends: virtual-locale-ro
Provides: bison-locale
Provides: ro-translation

%description -n bison-locale-ro
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the ro locale.

%package -n bison-locale-ru
Summary: GNU Project parser generator (yacc replacement) - ru translations
License: GPLv3
Group: devel
Recommends: virtual-locale-ru
Provides: bison-locale
Provides: ru-translation

%description -n bison-locale-ru
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the ru locale.

%package -n bison-locale-rw
Summary: GNU Project parser generator (yacc replacement) - rw translations
License: GPLv3
Group: devel
Recommends: virtual-locale-rw
Provides: bison-locale
Provides: rw-translation

%description -n bison-locale-rw
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the rw locale.

%package -n bison-locale-sk
Summary: GNU Project parser generator (yacc replacement) - sk translations
License: GPLv3
Group: devel
Recommends: virtual-locale-sk
Provides: bison-locale
Provides: sk-translation

%description -n bison-locale-sk
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the sk locale.

%package -n bison-locale-sl
Summary: GNU Project parser generator (yacc replacement) - sl translations
License: GPLv3
Group: devel
Recommends: virtual-locale-sl
Provides: bison-locale
Provides: sl-translation

%description -n bison-locale-sl
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the sl locale.

%package -n bison-locale-sq
Summary: GNU Project parser generator (yacc replacement) - sq translations
License: GPLv3
Group: devel
Recommends: virtual-locale-sq
Provides: bison-locale
Provides: sq-translation

%description -n bison-locale-sq
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the sq locale.

%package -n bison-locale-sr
Summary: GNU Project parser generator (yacc replacement) - sr translations
License: GPLv3
Group: devel
Recommends: virtual-locale-sr
Provides: bison-locale
Provides: sr-translation

%description -n bison-locale-sr
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the sr locale.

%package -n bison-locale-sv
Summary: GNU Project parser generator (yacc replacement) - sv translations
License: GPLv3
Group: devel
Recommends: virtual-locale-sv
Provides: bison-locale
Provides: sv-translation

%description -n bison-locale-sv
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the sv locale.

%package -n bison-locale-th
Summary: GNU Project parser generator (yacc replacement) - th translations
License: GPLv3
Group: devel
Recommends: virtual-locale-th
Provides: bison-locale
Provides: th-translation

%description -n bison-locale-th
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the th locale.

%package -n bison-locale-tr
Summary: GNU Project parser generator (yacc replacement) - tr translations
License: GPLv3
Group: devel
Recommends: virtual-locale-tr
Provides: bison-locale
Provides: tr-translation

%description -n bison-locale-tr
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the tr locale.

%package -n bison-locale-uk
Summary: GNU Project parser generator (yacc replacement) - uk translations
License: GPLv3
Group: devel
Recommends: virtual-locale-uk
Provides: bison-locale
Provides: uk-translation

%description -n bison-locale-uk
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the uk locale.

%package -n bison-locale-vi
Summary: GNU Project parser generator (yacc replacement) - vi translations
License: GPLv3
Group: devel
Recommends: virtual-locale-vi
Provides: bison-locale
Provides: vi-translation

%description -n bison-locale-vi
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the vi locale.

%package -n bison-locale-zh-cn
Summary: GNU Project parser generator (yacc replacement) - zh_CN translations
License: GPLv3
Group: devel
Recommends: virtual-locale-zh-cn
Provides: bison-locale
Provides: zh-cn-translation

%description -n bison-locale-zh-cn
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the zh_CN locale.

%package -n bison-locale-zh-tw
Summary: GNU Project parser generator (yacc replacement) - zh_TW translations
License: GPLv3
Group: devel
Recommends: virtual-locale-zh-tw
Provides: bison-locale
Provides: zh-tw-translation

%description -n bison-locale-zh-tw
Bison is a general-purpose parser generator that converts an annotated
context-free grammar into an LALR(1) or GLR parser for that grammar.  Bison
is upward compatible with Yacc: all properly-written Yacc grammars ought to
work with Bison with no change. Anyone familiar with Yacc should be able to
use Bison with little trouble.  This package contains language translation
files for the zh_TW locale.

%files
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/bin"
%dir "/usr/share/bison"
%dir "/usr/share/bison/xslt"
%dir "/usr/share/bison/skeletons"
%dir "/usr/share/bison/m4sugar"
"/usr/share/bison/bison-default.css"
"/usr/share/bison/README.md"
"/usr/share/bison/xslt/xml2text.xsl"
"/usr/share/bison/xslt/bison.xsl"
"/usr/share/bison/xslt/xml2xhtml.xsl"
"/usr/share/bison/xslt/xml2dot.xsl"
"/usr/share/bison/skeletons/glr.cc"
"/usr/share/bison/skeletons/bison.m4"
"/usr/share/bison/skeletons/c++-skel.m4"
"/usr/share/bison/skeletons/lalr1.d"
"/usr/share/bison/skeletons/d.m4"
"/usr/share/bison/skeletons/stack.hh"
"/usr/share/bison/skeletons/java.m4"
"/usr/share/bison/skeletons/variant.hh"
"/usr/share/bison/skeletons/glr.c"
"/usr/share/bison/skeletons/c-skel.m4"
"/usr/share/bison/skeletons/README-D.txt"
"/usr/share/bison/skeletons/lalr1.java"
"/usr/share/bison/skeletons/yacc.c"
"/usr/share/bison/skeletons/location.cc"
"/usr/share/bison/skeletons/c.m4"
"/usr/share/bison/skeletons/c++.m4"
"/usr/share/bison/skeletons/java-skel.m4"
"/usr/share/bison/skeletons/c-like.m4"
"/usr/share/bison/skeletons/d-skel.m4"
"/usr/share/bison/skeletons/lalr1.cc"
"/usr/share/bison/m4sugar/m4sugar.m4"
"/usr/share/bison/m4sugar/foreach.m4"
"/usr/bin/bison"
"/usr/bin/yacc"

%files -n bison-src
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/src"
%dir "/usr/src/debug"
%dir "/usr/src/debug/bison"
%dir "/usr/src/debug/bison/3.5.3-r0"
%dir "/usr/src/debug/bison/3.5.3-r0/build"
%dir "/usr/src/debug/bison/3.5.3-r0/bison-3.5.3"
%dir "/usr/src/debug/bison/3.5.3-r0/build/lib"
%dir "/usr/src/debug/bison/3.5.3-r0/build/lib/sys"
"/usr/src/debug/bison/3.5.3-r0/build/lib/stdlib.h"
"/usr/src/debug/bison/3.5.3-r0/build/lib/textstyle.h"
"/usr/src/debug/bison/3.5.3-r0/build/lib/signal.h"
"/usr/src/debug/bison/3.5.3-r0/build/lib/string.h"
"/usr/src/debug/bison/3.5.3-r0/build/lib/unistd.h"
"/usr/src/debug/bison/3.5.3-r0/build/lib/wchar.h"
"/usr/src/debug/bison/3.5.3-r0/build/lib/stdio.h"
"/usr/src/debug/bison/3.5.3-r0/build/lib/fcntl.h"
"/usr/src/debug/bison/3.5.3-r0/build/lib/sys/time.h"
%dir "/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src"
%dir "/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/print.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/state.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/fixits.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/print.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/location.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/symlist.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/getargs.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/closure.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/nullable.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/print-graph.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/ielr.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/reduce.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/files.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/uniqstr.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/location.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/lalr.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/lalr.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/parse-gram.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/output.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/derives.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/print-graph.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/graphviz.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/AnnotationList.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/gram.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/scan-gram.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/print-xml.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/main.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/symtab.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/named-ref.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/AnnotationList.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/output.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/tables.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/reduce.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/InadequacyList.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/relation.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/conflicts.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/getargs.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/lr0.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/gram.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/complain.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/reader.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/assoc.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/muscle-tab.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/derives.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/scan-skel.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/reader.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/lr0.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/symlist.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/graphviz.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/ielr.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/named-ref.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/files.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/nullable.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/complain.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/symtab.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/assoc.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/uniqstr.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/print-xml.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/conflicts.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/relation.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/flex-scanner.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/scan-code.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/Sbitset.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/InadequacyList.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/muscle-tab.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/closure.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/Sbitset.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/state.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/fixits.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/src/tables.h"
%dir "/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/glthread"
%dir "/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/bitset"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/bitrotate.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/fstrcmp.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/fseterr.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/fatal-signal.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/timevar.def"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/exitfail.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/mbrtowc.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/getopt_int.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/gl_xlist.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/sig-handler.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/xalloc-die.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/fd-safer.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/hard-locale.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/xmalloc.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/bitset.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/c-ctype.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/xalloc-oversized.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/mbchar.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/basename-lgpl.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/xtime.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/hash.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/gl_list.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/c-strcaseeq.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/progname.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/setlocale_null.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/vasnprintf.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/unistd-safer.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/perror.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/vasnprintf.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/xconcat-filename.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/main.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/printf-args.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/diffseq.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/close-stream.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/timevar.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/timevar.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/printf.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/setlocale_null.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/bitsetv.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/dup-safer.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/xstrndup.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/mbswidth.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/argmatch.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/fatal-signal.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/error.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/xmemdup0.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/get-errno.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/getopt-core.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/closeout.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/mbswidth.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/concat-filename.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/close-stream.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/fseterr.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/memchr.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/quote.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/xstrndup.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/yyerror.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/fstrcmp.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/getopt1.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/quotearg.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/printf-parse.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/bitsetv.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/mbfile.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/vfprintf.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/gl_array_list.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/getopt.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/get-errno.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/xalloc.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/wait-process.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/asnprintf.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/obstack.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/xhash.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/localcharset.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/exitfail.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/timespec.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/xmemdup0.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/argmatch.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/progname.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/snprintf.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/fpucw.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/mbchar.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/gl_array_list.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/getopt-ext.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/bitset.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/xsize.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/printf-parse.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/sprintf.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/pipe-safer.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/dirname.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/hash.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/fopen-safer.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/path-join.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/spawn-pipe.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/quotearg.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/wait-process.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/fprintf.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/stdio-safer.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/obstack.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/printf-args.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/spawn-pipe.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/fcntl.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/gethrxtime.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/concat-filename.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/c-strcase.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/hard-locale.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/strerror_r.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/gettime.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/c-strcasecmp.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/path-join.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/localcharset.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/gethrxtime.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/glthread/tls.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/glthread/lock.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/glthread/lock.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/bitset/array.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/bitset/list.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/bitset/table.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/bitset/stats.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/bitset/base.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/bitset/vector.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/bitset/table.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/bitset/list.h"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/bitset/array.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/bitset/vector.c"
"/usr/src/debug/bison/3.5.3-r0/bison-3.5.3/lib/bitset/stats.c"

%files -n bison-dbg
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/bin"
%dir "/usr/bin/.debug"
"/usr/bin/.debug/bison"

%files -n bison-staticdev
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/lib"
"/usr/lib/liby.a"

%files -n bison-dev
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/aclocal"
"/usr/share/aclocal/bison-i18n.m4"

%files -n bison-doc
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/doc"
%dir "/usr/share/info"
%dir "/usr/share/doc/bison"
%dir "/usr/share/doc/bison/examples"
"/usr/share/doc/bison/THANKS"
"/usr/share/doc/bison/COPYING"
"/usr/share/doc/bison/TODO"
"/usr/share/doc/bison/README"
"/usr/share/doc/bison/NEWS"
"/usr/share/doc/bison/AUTHORS"
%dir "/usr/share/doc/bison/examples/java"
%dir "/usr/share/doc/bison/examples/c++"
%dir "/usr/share/doc/bison/examples/c"
%dir "/usr/share/doc/bison/examples/d"
"/usr/share/doc/bison/examples/README.md"
"/usr/share/doc/bison/examples/java/Makefile"
"/usr/share/doc/bison/examples/java/README.md"
"/usr/share/doc/bison/examples/java/Calc.y"
%dir "/usr/share/doc/bison/examples/c++/calc++"
"/usr/share/doc/bison/examples/c++/variant-11.yy"
"/usr/share/doc/bison/examples/c++/simple.yy"
"/usr/share/doc/bison/examples/c++/Makefile"
"/usr/share/doc/bison/examples/c++/README.md"
"/usr/share/doc/bison/examples/c++/variant.yy"
"/usr/share/doc/bison/examples/c++/calc++/scanner.ll"
"/usr/share/doc/bison/examples/c++/calc++/driver.cc"
"/usr/share/doc/bison/examples/c++/calc++/Makefile"
"/usr/share/doc/bison/examples/c++/calc++/driver.hh"
"/usr/share/doc/bison/examples/c++/calc++/parser.yy"
"/usr/share/doc/bison/examples/c++/calc++/calc++.cc"
"/usr/share/doc/bison/examples/c++/calc++/README.md"
%dir "/usr/share/doc/bison/examples/c/calc"
%dir "/usr/share/doc/bison/examples/c/mfcalc"
%dir "/usr/share/doc/bison/examples/c/lexcalc"
%dir "/usr/share/doc/bison/examples/c/rpcalc"
%dir "/usr/share/doc/bison/examples/c/reccalc"
"/usr/share/doc/bison/examples/c/README.md"
"/usr/share/doc/bison/examples/c/calc/calc.y"
"/usr/share/doc/bison/examples/c/calc/Makefile"
"/usr/share/doc/bison/examples/c/calc/README.md"
"/usr/share/doc/bison/examples/c/mfcalc/calc.h"
"/usr/share/doc/bison/examples/c/mfcalc/mfcalc.y"
"/usr/share/doc/bison/examples/c/mfcalc/Makefile"
"/usr/share/doc/bison/examples/c/lexcalc/scan.l"
"/usr/share/doc/bison/examples/c/lexcalc/Makefile"
"/usr/share/doc/bison/examples/c/lexcalc/parse.y"
"/usr/share/doc/bison/examples/c/lexcalc/README.md"
"/usr/share/doc/bison/examples/c/rpcalc/Makefile"
"/usr/share/doc/bison/examples/c/rpcalc/rpcalc.y"
"/usr/share/doc/bison/examples/c/reccalc/scan.l"
"/usr/share/doc/bison/examples/c/reccalc/Makefile"
"/usr/share/doc/bison/examples/c/reccalc/parse.y"
"/usr/share/doc/bison/examples/c/reccalc/README.md"
"/usr/share/doc/bison/examples/d/calc.y"
"/usr/share/doc/bison/examples/d/Makefile"
"/usr/share/doc/bison/examples/d/README.md"
"/usr/share/info/bison.info"

%files -n bison-locale-af
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/af"
%dir "/usr/share/locale/af/LC_MESSAGES"
"/usr/share/locale/af/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-ast
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/ast"
%dir "/usr/share/locale/ast/LC_MESSAGES"
"/usr/share/locale/ast/LC_MESSAGES/bison-runtime.mo"

%files -n bison-locale-be
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/be"
%dir "/usr/share/locale/be/LC_MESSAGES"
"/usr/share/locale/be/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-bg
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/bg"
%dir "/usr/share/locale/bg/LC_MESSAGES"
"/usr/share/locale/bg/LC_MESSAGES/bison.mo"
"/usr/share/locale/bg/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/bg/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-ca
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/ca"
%dir "/usr/share/locale/ca/LC_MESSAGES"
"/usr/share/locale/ca/LC_MESSAGES/bison.mo"
"/usr/share/locale/ca/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/ca/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-cs
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/cs"
%dir "/usr/share/locale/cs/LC_MESSAGES"
"/usr/share/locale/cs/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-da
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/da"
%dir "/usr/share/locale/da/LC_MESSAGES"
"/usr/share/locale/da/LC_MESSAGES/bison.mo"
"/usr/share/locale/da/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/da/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-de
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/de"
%dir "/usr/share/locale/de/LC_MESSAGES"
"/usr/share/locale/de/LC_MESSAGES/bison.mo"
"/usr/share/locale/de/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/de/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-el
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/el"
%dir "/usr/share/locale/el/LC_MESSAGES"
"/usr/share/locale/el/LC_MESSAGES/bison.mo"
"/usr/share/locale/el/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/el/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-eo
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/eo"
%dir "/usr/share/locale/eo/LC_MESSAGES"
"/usr/share/locale/eo/LC_MESSAGES/bison.mo"
"/usr/share/locale/eo/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/eo/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-es
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/es"
%dir "/usr/share/locale/es/LC_MESSAGES"
"/usr/share/locale/es/LC_MESSAGES/bison.mo"
"/usr/share/locale/es/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/es/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-et
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/et"
%dir "/usr/share/locale/et/LC_MESSAGES"
"/usr/share/locale/et/LC_MESSAGES/bison.mo"
"/usr/share/locale/et/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/et/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-eu
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/eu"
%dir "/usr/share/locale/eu/LC_MESSAGES"
"/usr/share/locale/eu/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-fi
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/fi"
%dir "/usr/share/locale/fi/LC_MESSAGES"
"/usr/share/locale/fi/LC_MESSAGES/bison.mo"
"/usr/share/locale/fi/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/fi/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-fr
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/fr"
%dir "/usr/share/locale/fr/LC_MESSAGES"
"/usr/share/locale/fr/LC_MESSAGES/bison.mo"
"/usr/share/locale/fr/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/fr/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-ga
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/ga"
%dir "/usr/share/locale/ga/LC_MESSAGES"
"/usr/share/locale/ga/LC_MESSAGES/bison.mo"
"/usr/share/locale/ga/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/ga/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-gl
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/gl"
%dir "/usr/share/locale/gl/LC_MESSAGES"
"/usr/share/locale/gl/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/gl/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-hr
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/hr"
%dir "/usr/share/locale/hr/LC_MESSAGES"
"/usr/share/locale/hr/LC_MESSAGES/bison.mo"
"/usr/share/locale/hr/LC_MESSAGES/bison-runtime.mo"

%files -n bison-locale-hu
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/hu"
%dir "/usr/share/locale/hu/LC_MESSAGES"
"/usr/share/locale/hu/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/hu/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-ia
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/ia"
%dir "/usr/share/locale/ia/LC_MESSAGES"
"/usr/share/locale/ia/LC_MESSAGES/bison-runtime.mo"

%files -n bison-locale-id
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/id"
%dir "/usr/share/locale/id/LC_MESSAGES"
"/usr/share/locale/id/LC_MESSAGES/bison.mo"
"/usr/share/locale/id/LC_MESSAGES/bison-runtime.mo"

%files -n bison-locale-it
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/it"
%dir "/usr/share/locale/it/LC_MESSAGES"
"/usr/share/locale/it/LC_MESSAGES/bison.mo"
"/usr/share/locale/it/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/it/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-ja
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/ja"
%dir "/usr/share/locale/ja/LC_MESSAGES"
"/usr/share/locale/ja/LC_MESSAGES/bison.mo"
"/usr/share/locale/ja/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/ja/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-ko
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/ko"
%dir "/usr/share/locale/ko/LC_MESSAGES"
"/usr/share/locale/ko/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-ky
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/ky"
%dir "/usr/share/locale/ky/LC_MESSAGES"
"/usr/share/locale/ky/LC_MESSAGES/bison-runtime.mo"

%files -n bison-locale-lt
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/lt"
%dir "/usr/share/locale/lt/LC_MESSAGES"
"/usr/share/locale/lt/LC_MESSAGES/bison-runtime.mo"

%files -n bison-locale-lv
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/lv"
%dir "/usr/share/locale/lv/LC_MESSAGES"
"/usr/share/locale/lv/LC_MESSAGES/bison-runtime.mo"

%files -n bison-locale-ms
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/ms"
%dir "/usr/share/locale/ms/LC_MESSAGES"
"/usr/share/locale/ms/LC_MESSAGES/bison.mo"
"/usr/share/locale/ms/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/ms/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-nb
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/nb"
%dir "/usr/share/locale/nb/LC_MESSAGES"
"/usr/share/locale/nb/LC_MESSAGES/bison.mo"
"/usr/share/locale/nb/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/nb/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-nl
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/nl"
%dir "/usr/share/locale/nl/LC_MESSAGES"
"/usr/share/locale/nl/LC_MESSAGES/bison.mo"
"/usr/share/locale/nl/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/nl/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-pl
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/pl"
%dir "/usr/share/locale/pl/LC_MESSAGES"
"/usr/share/locale/pl/LC_MESSAGES/bison.mo"
"/usr/share/locale/pl/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/pl/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-pt
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/pt"
%dir "/usr/share/locale/pt/LC_MESSAGES"
"/usr/share/locale/pt/LC_MESSAGES/bison.mo"
"/usr/share/locale/pt/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/pt/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-pt-br
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/pt_BR"
%dir "/usr/share/locale/pt_BR/LC_MESSAGES"
"/usr/share/locale/pt_BR/LC_MESSAGES/bison.mo"
"/usr/share/locale/pt_BR/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/pt_BR/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-ro
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/ro"
%dir "/usr/share/locale/ro/LC_MESSAGES"
"/usr/share/locale/ro/LC_MESSAGES/bison.mo"
"/usr/share/locale/ro/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/ro/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-ru
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/ru"
%dir "/usr/share/locale/ru/LC_MESSAGES"
"/usr/share/locale/ru/LC_MESSAGES/bison.mo"
"/usr/share/locale/ru/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/ru/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-rw
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/rw"
%dir "/usr/share/locale/rw/LC_MESSAGES"
"/usr/share/locale/rw/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-sk
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/sk"
%dir "/usr/share/locale/sk/LC_MESSAGES"
"/usr/share/locale/sk/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-sl
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/sl"
%dir "/usr/share/locale/sl/LC_MESSAGES"
"/usr/share/locale/sl/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/sl/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-sq
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/sq"
%dir "/usr/share/locale/sq/LC_MESSAGES"
"/usr/share/locale/sq/LC_MESSAGES/bison-runtime.mo"

%files -n bison-locale-sr
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/sr"
%dir "/usr/share/locale/sr/LC_MESSAGES"
"/usr/share/locale/sr/LC_MESSAGES/bison.mo"
"/usr/share/locale/sr/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/sr/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-sv
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/sv"
%dir "/usr/share/locale/sv/LC_MESSAGES"
"/usr/share/locale/sv/LC_MESSAGES/bison.mo"
"/usr/share/locale/sv/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/sv/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-th
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/th"
%dir "/usr/share/locale/th/LC_MESSAGES"
"/usr/share/locale/th/LC_MESSAGES/bison-runtime.mo"

%files -n bison-locale-tr
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/tr"
%dir "/usr/share/locale/tr/LC_MESSAGES"
"/usr/share/locale/tr/LC_MESSAGES/bison.mo"
"/usr/share/locale/tr/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/tr/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-uk
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/uk"
%dir "/usr/share/locale/uk/LC_MESSAGES"
"/usr/share/locale/uk/LC_MESSAGES/bison.mo"
"/usr/share/locale/uk/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/uk/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-vi
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/vi"
%dir "/usr/share/locale/vi/LC_MESSAGES"
"/usr/share/locale/vi/LC_MESSAGES/bison.mo"
"/usr/share/locale/vi/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/vi/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-zh-cn
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/zh_CN"
%dir "/usr/share/locale/zh_CN/LC_MESSAGES"
"/usr/share/locale/zh_CN/LC_MESSAGES/bison.mo"
"/usr/share/locale/zh_CN/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/zh_CN/LC_MESSAGES/bison-gnulib.mo"

%files -n bison-locale-zh-tw
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/zh_TW"
%dir "/usr/share/locale/zh_TW/LC_MESSAGES"
"/usr/share/locale/zh_TW/LC_MESSAGES/bison.mo"
"/usr/share/locale/zh_TW/LC_MESSAGES/bison-runtime.mo"
"/usr/share/locale/zh_TW/LC_MESSAGES/bison-gnulib.mo"

