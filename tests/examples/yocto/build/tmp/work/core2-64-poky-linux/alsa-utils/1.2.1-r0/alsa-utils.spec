Summary: ALSA sound utilities
Name: alsa-utils
Version: 1.2.1
Release: r0
License: GPLv2+
Group: console/utils
Packager: Poky <poky@lists.yoctoproject.org>
URL: http://www.alsa-project.org
BuildRequires: alsa-lib
BuildRequires: autoconf-native
BuildRequires: automake-native
BuildRequires: gettext-native
BuildRequires: gnu-config-native
BuildRequires: libsamplerate0
BuildRequires: libtool-cross
BuildRequires: libtool-native
BuildRequires: ncurses
BuildRequires: pkgconfig-native
BuildRequires: udev
BuildRequires: virtual/libc
BuildRequires: virtual/x86_64-poky-linux-compilerlibs
BuildRequires: virtual/x86_64-poky-linux-gcc
Requires: alsa-utils-aconnect
Requires: alsa-utils-alsactl
Requires: alsa-utils-alsaloop
Requires: alsa-utils-alsamixer
Requires: alsa-utils-alsatplg
Requires: alsa-utils-alsaucm
Requires: alsa-utils-amixer
Requires: alsa-utils-aplay
Requires: alsa-utils-aseqdump
Requires: alsa-utils-aseqnet
Requires: alsa-utils-iecset
Requires: alsa-utils-midi
Requires: alsa-utils-speakertest

%description
ALSA sound utilities.

%package -n alsa-utils-src
Summary: ALSA sound utilities - Source files
License: GPLv2+
Group: devel

%description -n alsa-utils-src
ALSA sound utilities.  This package contains sources for debugging
purposes.

%package -n alsa-utils-dbg
Summary: ALSA sound utilities - Debugging files
License: GPLv2+
Group: devel
Recommends: alsa-lib-dbg
Recommends: glibc-dbg
Recommends: libatopology-dbg
Recommends: libsamplerate0-dbg
Recommends: ncurses-libformw-dbg
Recommends: ncurses-libmenuw-dbg
Recommends: ncurses-libncursesw-dbg
Recommends: ncurses-libpanelw-dbg
Recommends: ncurses-libtinfo-dbg

%description -n alsa-utils-dbg
ALSA sound utilities.  This package contains ELF symbols and related
sources for debugging purposes.

%package -n alsa-utils-staticdev
Summary: ALSA sound utilities - Development files (Static Libraries)
License: GPLv2+
Group: devel
Requires: alsa-utils-dev = 1.2.1-r0

%description -n alsa-utils-staticdev
ALSA sound utilities.  This package contains static libraries for software
development.

%package -n alsa-utils-dev
Summary: ALSA sound utilities - Development files
License: GPLv2+
Group: devel
Requires: alsa-utils = 1.2.1-r0
Recommends: alsa-lib-dev
Recommends: alsa-utils-aconnect-dev
Recommends: alsa-utils-alsactl-dev
Recommends: alsa-utils-alsaloop-dev
Recommends: alsa-utils-alsamixer-dev
Recommends: alsa-utils-alsatplg-dev
Recommends: alsa-utils-alsaucm-dev
Recommends: alsa-utils-amixer-dev
Recommends: alsa-utils-aplay-dev
Recommends: alsa-utils-aseqdump-dev
Recommends: alsa-utils-aseqnet-dev
Recommends: alsa-utils-iecset-dev
Recommends: alsa-utils-midi-dev
Recommends: alsa-utils-speakertest-dev
Recommends: glibc-dev
Recommends: libatopology-dev
Recommends: libsamplerate0-dev
Recommends: ncurses-dev
Recommends: ncurses-libformw-dev
Recommends: ncurses-libmenuw-dev
Recommends: ncurses-libncursesw-dev
Recommends: ncurses-libpanelw-dev
Recommends: ncurses-libtinfo-dev
Recommends: udev-dev

%description -n alsa-utils-dev
ALSA sound utilities.  This package contains symbolic links, header files,
and related items necessary for software development.

%package -n alsa-utils-doc
Summary: ALSA sound utilities - Documentation files
License: GPLv2+
Group: doc

%description -n alsa-utils-doc
ALSA sound utilities.  This package contains documentation.

%package -n alsa-utils-alsamixer
Summary: ncurses-based control for ALSA mixer and settings
License: GPLv2+
Group: console/utils
Requires: libasound.so.2()(64bit)
Requires: libasound.so.2(ALSA_0.9)(64bit)
Requires: libasound2 >= 1.2.1.2
Requires: libc.so.6()(64bit)
Requires: libc.so.6(GLIBC_2.14)(64bit)
Requires: libc.so.6(GLIBC_2.2.5)(64bit)
Requires: libc.so.6(GLIBC_2.3.4)(64bit)
Requires: libc.so.6(GLIBC_2.4)(64bit)
Requires: libc.so.6(GLIBC_2.8)(64bit)
Requires: libc6 >= 2.31+git0+6fdf971c9d
Requires: libformw.so.5()(64bit)
Requires: libformw5 >= 6.2
Requires: libm.so.6()(64bit)
Requires: libm.so.6(GLIBC_2.2.5)(64bit)
Requires: libm.so.6(GLIBC_2.29)(64bit)
Requires: libmenuw.so.5()(64bit)
Requires: libmenuw5 >= 6.2
Requires: libncursesw.so.5()(64bit)
Requires: libncursesw5 >= 6.2
Requires: libpanelw.so.5()(64bit)
Requires: libpanelw5 >= 6.2
Requires: libpthread.so.0()(64bit)
Requires: libpthread.so.0(GLIBC_2.2.5)(64bit)
Requires: libtinfo.so.5()(64bit)
Requires: libtinfo5 >= 6.2
Requires: rtld(GNU_HASH)

%description -n alsa-utils-alsamixer
ALSA sound utilities.

%package -n alsa-utils-alsatplg
Summary: Converts topology text files into binary format for kernel
License: GPLv2+
Group: console/utils
Requires: libasound.so.2()(64bit)
Requires: libasound.so.2(ALSA_0.9)(64bit)
Requires: libasound2 >= 1.2.1.2
Requires: libatopology.so.2()(64bit)
Requires: libatopology.so.2(ALSA_0.9)(64bit)
Requires: libatopology2 >= 1.2.1.2
Requires: libc.so.6()(64bit)
Requires: libc.so.6(GLIBC_2.2.5)(64bit)
Requires: libc.so.6(GLIBC_2.3.4)(64bit)
Requires: libc.so.6(GLIBC_2.4)(64bit)
Requires: libc6 >= 2.31+git0+6fdf971c9d
Requires: rtld(GNU_HASH)

%description -n alsa-utils-alsatplg
ALSA sound utilities.

%package -n alsa-utils-midi
Summary: Miscellaneous MIDI utilities for ALSA
License: GPLv2+
Group: console/utils
Requires: libasound.so.2()(64bit)
Requires: libasound.so.2(ALSA_0.9)(64bit)
Requires: libasound2 >= 1.2.1.2
Requires: libc.so.6()(64bit)
Requires: libc.so.6(GLIBC_2.2.5)(64bit)
Requires: libc.so.6(GLIBC_2.3)(64bit)
Requires: libc.so.6(GLIBC_2.3.4)(64bit)
Requires: libc.so.6(GLIBC_2.4)(64bit)
Requires: libc.so.6(GLIBC_2.8)(64bit)
Requires: libc6 >= 2.31+git0+6fdf971c9d
Requires: libm.so.6()(64bit)
Requires: libm.so.6(GLIBC_2.2.5)(64bit)
Requires: libpthread.so.0()(64bit)
Requires: libpthread.so.0(GLIBC_2.2.5)(64bit)
Requires: rtld(GNU_HASH)

%description -n alsa-utils-midi
ALSA sound utilities.

%package -n alsa-utils-aplay
Summary: Play (and record) sound files using ALSA
License: GPLv2+
Group: console/utils
Requires: libasound.so.2()(64bit)
Requires: libasound.so.2(ALSA_0.9)(64bit)
Requires: libasound.so.2(ALSA_0.9.0rc4)(64bit)
Requires: libasound.so.2(ALSA_0.9.0rc8)(64bit)
Requires: libasound2 >= 1.2.1.2
Requires: libc.so.6()(64bit)
Requires: libc.so.6(GLIBC_2.14)(64bit)
Requires: libc.so.6(GLIBC_2.15)(64bit)
Requires: libc.so.6(GLIBC_2.17)(64bit)
Requires: libc.so.6(GLIBC_2.2.5)(64bit)
Requires: libc.so.6(GLIBC_2.3.2)(64bit)
Requires: libc.so.6(GLIBC_2.3.4)(64bit)
Requires: libc.so.6(GLIBC_2.4)(64bit)
Requires: libc6 >= 2.31+git0+6fdf971c9d
Requires: libm.so.6()(64bit)
Requires: libm.so.6(GLIBC_2.2.5)(64bit)
Requires: libpthread.so.0()(64bit)
Requires: libpthread.so.0(GLIBC_2.2.5)(64bit)
Requires: rtld(GNU_HASH)

%description -n alsa-utils-aplay
ALSA sound utilities.

%package -n alsa-utils-amixer
Summary: Command-line control for ALSA mixer and settings
License: GPLv2+
Group: console/utils
Requires: libasound.so.2()(64bit)
Requires: libasound.so.2(ALSA_0.9)(64bit)
Requires: libasound2 >= 1.2.1.2
Requires: libc.so.6()(64bit)
Requires: libc.so.6(GLIBC_2.2.5)(64bit)
Requires: libc.so.6(GLIBC_2.3)(64bit)
Requires: libc.so.6(GLIBC_2.3.4)(64bit)
Requires: libc.so.6(GLIBC_2.4)(64bit)
Requires: libc6 >= 2.31+git0+6fdf971c9d
Requires: libm.so.6()(64bit)
Requires: libm.so.6(GLIBC_2.2.5)(64bit)
Requires: libm.so.6(GLIBC_2.29)(64bit)
Requires: rtld(GNU_HASH)

%description -n alsa-utils-amixer
ALSA sound utilities.

%package -n alsa-utils-aconnect
Summary: ALSA sequencer connection manager
License: GPLv2+
Group: console/utils
Requires: libasound.so.2()(64bit)
Requires: libasound.so.2(ALSA_0.9)(64bit)
Requires: libasound2 >= 1.2.1.2
Requires: libc.so.6()(64bit)
Requires: libc.so.6(GLIBC_2.2.5)(64bit)
Requires: libc.so.6(GLIBC_2.3.4)(64bit)
Requires: libc.so.6(GLIBC_2.4)(64bit)
Requires: libc6 >= 2.31+git0+6fdf971c9d
Requires: libpthread.so.0()(64bit)
Requires: libpthread.so.0(GLIBC_2.2.5)(64bit)
Requires: rtld(GNU_HASH)

%description -n alsa-utils-aconnect
ALSA sound utilities.

%package -n alsa-utils-iecset
Summary: ALSA utility for setting/showing IEC958 (S/PDIF) status bits
License: GPLv2+
Group: console/utils
Requires: libasound.so.2()(64bit)
Requires: libasound.so.2(ALSA_0.9)(64bit)
Requires: libasound2 >= 1.2.1.2
Requires: libc.so.6()(64bit)
Requires: libc.so.6(GLIBC_2.2.5)(64bit)
Requires: libc.so.6(GLIBC_2.3)(64bit)
Requires: libc.so.6(GLIBC_2.3.4)(64bit)
Requires: libc.so.6(GLIBC_2.4)(64bit)
Requires: libc6 >= 2.31+git0+6fdf971c9d
Requires: rtld(GNU_HASH)

%description -n alsa-utils-iecset
ALSA sound utilities.

%package -n alsa-utils-speakertest
Summary: ALSA surround speaker test utility
License: GPLv2+
Group: console/utils
Requires: libasound.so.2()(64bit)
Requires: libasound.so.2(ALSA_0.9)(64bit)
Requires: libasound.so.2(ALSA_0.9.0rc4)(64bit)
Requires: libasound2 >= 1.2.1.2
Requires: libc.so.6()(64bit)
Requires: libc.so.6(GLIBC_2.2.5)(64bit)
Requires: libc.so.6(GLIBC_2.3)(64bit)
Requires: libc.so.6(GLIBC_2.3.4)(64bit)
Requires: libc.so.6(GLIBC_2.4)(64bit)
Requires: libc6 >= 2.31+git0+6fdf971c9d
Requires: libm.so.6()(64bit)
Requires: libm.so.6(GLIBC_2.2.5)(64bit)
Requires: libpthread.so.0()(64bit)
Requires: libpthread.so.0(GLIBC_2.2.5)(64bit)
Requires: rtld(GNU_HASH)

%description -n alsa-utils-speakertest
ALSA sound utilities.

%package -n alsa-utils-aseqnet
Summary: Network client/server for ALSA sequencer
License: GPLv2+
Group: console/utils
Requires: libasound.so.2()(64bit)
Requires: libasound.so.2(ALSA_0.9)(64bit)
Requires: libasound2 >= 1.2.1.2
Requires: libc.so.6()(64bit)
Requires: libc.so.6(GLIBC_2.14)(64bit)
Requires: libc.so.6(GLIBC_2.2.5)(64bit)
Requires: libc.so.6(GLIBC_2.3)(64bit)
Requires: libc.so.6(GLIBC_2.3.4)(64bit)
Requires: libc.so.6(GLIBC_2.4)(64bit)
Requires: libc6 >= 2.31+git0+6fdf971c9d
Requires: libpthread.so.0()(64bit)
Requires: libpthread.so.0(GLIBC_2.2.5)(64bit)
Requires: rtld(GNU_HASH)

%description -n alsa-utils-aseqnet
ALSA sound utilities.

%package -n alsa-utils-aseqdump
Summary: Shows the events received at an ALSA sequencer port
License: GPLv2+
Group: console/utils
Requires: libasound.so.2()(64bit)
Requires: libasound.so.2(ALSA_0.9)(64bit)
Requires: libasound2 >= 1.2.1.2
Requires: libc.so.6()(64bit)
Requires: libc.so.6(GLIBC_2.2.5)(64bit)
Requires: libc.so.6(GLIBC_2.3.4)(64bit)
Requires: libc.so.6(GLIBC_2.4)(64bit)
Requires: libc6 >= 2.31+git0+6fdf971c9d
Requires: rtld(GNU_HASH)

%description -n alsa-utils-aseqdump
ALSA sound utilities.

%package -n alsa-utils-alsactl
Summary: Saves/restores ALSA-settings in /etc/asound.state
License: GPLv2+
Group: console/utils
Requires: libasound.so.2()(64bit)
Requires: libasound.so.2(ALSA_0.9)(64bit)
Requires: libasound2 >= 1.2.1.2
Requires: libc.so.6()(64bit)
Requires: libc.so.6(GLIBC_2.14)(64bit)
Requires: libc.so.6(GLIBC_2.15)(64bit)
Requires: libc.so.6(GLIBC_2.2.5)(64bit)
Requires: libc.so.6(GLIBC_2.3)(64bit)
Requires: libc.so.6(GLIBC_2.3.2)(64bit)
Requires: libc.so.6(GLIBC_2.3.4)(64bit)
Requires: libc.so.6(GLIBC_2.4)(64bit)
Requires: libc.so.6(GLIBC_2.7)(64bit)
Requires: libc.so.6(GLIBC_2.9)(64bit)
Requires: libc6 >= 2.31+git0+6fdf971c9d
Requires: libpthread.so.0()(64bit)
Requires: libpthread.so.0(GLIBC_2.2.5)(64bit)
Requires: rtld(GNU_HASH)
Recommends: alsa-states

%description -n alsa-utils-alsactl
ALSA sound utilities.

%package -n alsa-utils-alsaloop
Summary: ALSA PCM loopback utility
License: GPLv2+
Group: console/utils
Requires: libasound.so.2()(64bit)
Requires: libasound.so.2(ALSA_0.9)(64bit)
Requires: libasound.so.2(ALSA_0.9.0rc4)(64bit)
Requires: libasound2 >= 1.2.1.2
Requires: libc.so.6()(64bit)
Requires: libc.so.6(GLIBC_2.14)(64bit)
Requires: libc.so.6(GLIBC_2.2.5)(64bit)
Requires: libc.so.6(GLIBC_2.3)(64bit)
Requires: libc.so.6(GLIBC_2.3.4)(64bit)
Requires: libc.so.6(GLIBC_2.4)(64bit)
Requires: libc6 >= 2.31+git0+6fdf971c9d
Requires: libpthread.so.0()(64bit)
Requires: libpthread.so.0(GLIBC_2.2.5)(64bit)
Requires: libsamplerate.so.0()(64bit)
Requires: libsamplerate.so.0(libsamplerate.so.0.0)(64bit)
Requires: libsamplerate.so.0(libsamplerate.so.0.1)(64bit)
Requires: libsamplerate0 >= 0.1.9
Requires: rtld(GNU_HASH)

%description -n alsa-utils-alsaloop
ALSA sound utilities.

%package -n alsa-utils-alsaucm
Summary: ALSA Use Case Manager
License: GPLv2+
Group: console/utils
Requires: libasound.so.2()(64bit)
Requires: libasound.so.2(ALSA_0.9)(64bit)
Requires: libasound2 >= 1.2.1.2
Requires: libc.so.6()(64bit)
Requires: libc.so.6(GLIBC_2.2.5)(64bit)
Requires: libc.so.6(GLIBC_2.3.4)(64bit)
Requires: libc.so.6(GLIBC_2.4)(64bit)
Requires: libc6 >= 2.31+git0+6fdf971c9d
Requires: libpthread.so.0()(64bit)
Requires: libpthread.so.0(GLIBC_2.2.5)(64bit)
Requires: rtld(GNU_HASH)

%description -n alsa-utils-alsaucm
ALSA sound utilities.

%package -n alsa-utils-locale-de
Summary: ALSA sound utilities - de translations
License: GPLv2+
Group: console/utils
Recommends: virtual-locale-de
Provides: alsa-utils-locale
Provides: de-translation

%description -n alsa-utils-locale-de
ALSA sound utilities.  This package contains language translation files for
the de locale.

%package -n alsa-utils-locale-fr
Summary: ALSA sound utilities - fr translations
License: GPLv2+
Group: console/utils
Recommends: virtual-locale-fr
Provides: alsa-utils-locale
Provides: fr-translation

%description -n alsa-utils-locale-fr
ALSA sound utilities.  This package contains language translation files for
the fr locale.

%package -n alsa-utils-locale-ja
Summary: ALSA sound utilities - ja translations
License: GPLv2+
Group: console/utils
Recommends: virtual-locale-ja
Provides: alsa-utils-locale
Provides: ja-translation

%description -n alsa-utils-locale-ja
ALSA sound utilities.  This package contains language translation files for
the ja locale.

%package -n alsa-utils-locale-ru
Summary: ALSA sound utilities - ru translations
License: GPLv2+
Group: console/utils
Recommends: virtual-locale-ru
Provides: alsa-utils-locale
Provides: ru-translation

%description -n alsa-utils-locale-ru
ALSA sound utilities.  This package contains language translation files for
the ru locale.

%post -n alsa-utils-doc
# alsa-utils-doc - postinst
	# only update manual page index caches when manual files are built and installed
	if false; then
		if test -n "$D"; then
			if true; then
				sed "s:\(\s\)/:\1$D/:g" $D/etc/man_db.conf | PSEUDO_UNLOAD=1 qemu-x86_64 -r 3.2.0  -cpu core2duo -L $D -E LD_LIBRARY_PATH=$D/usr/lib:$D/lib $D/usr/bin/mandb -C - -u -q $D/usr/share/man
				chown -R root:root $D/usr/share/man
				mkdir -p $D/var/cache/man
				cd $D/usr/share/man
				find . -name index.db | while read index; do
					mkdir -p $D/var/cache/man/$(dirname ${index})
					mv ${index} $D/var/cache/man/${index}
					chown man:man $D/var/cache/man/${index}
				done
				cd -
			else
				$INTERCEPT_DIR/postinst_intercept delay_to_first_boot alsa-utils-doc mlprefix=
			fi
		else
			mandb -q
		fi
	fi


%postun -n alsa-utils-doc
# alsa-utils-doc - postrm
if [ "$1" = "0" ] ; then
# only update manual page index caches when manual files are built and installed
	if false; then
		mandb -q
	fi
fi

%files
%defattr(-,-,-,-)

%files -n alsa-utils-src
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/src"
%dir "/usr/src/debug"
%dir "/usr/src/debug/alsa-utils"
%dir "/usr/src/debug/alsa-utils/1.2.1-r0"
%dir "/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1"
%dir "/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer"
%dir "/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/aplay"
%dir "/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/seq"
%dir "/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/amixer"
%dir "/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/amidi"
%dir "/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer"
%dir "/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/iecset"
%dir "/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsaloop"
%dir "/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/topology"
%dir "/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsactl"
%dir "/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsaucm"
%dir "/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/speaker-test"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/waiter.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/container-voc.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/container-au.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/subcmd.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/xfer-libasound-irq-mmap.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/subcmd-transfer.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/container-raw.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/main.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/xfer-libasound-irq-rw.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/xfer-libasound.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/xfer-options.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/container.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/misc.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/waiter-select.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/waiter.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/mapper-multiple.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/xfer-libasound.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/subcmd-list.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/xfer-libasound-timer-mmap.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/frame-cache.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/mapper.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/container.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/xfer.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/waiter-epoll.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/container-riff-wave.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/frame-cache.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/mapper-single.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/waiter-poll.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/xfer.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/axfer/mapper.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/aplay/aplay.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/aplay/formats.h"
%dir "/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/seq/aseqdump"
%dir "/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/seq/aseqnet"
%dir "/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/seq/aconnect"
%dir "/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/seq/aplaymidi"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/seq/aseqdump/aseqdump.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/seq/aseqnet/aseqnet.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/seq/aconnect/aconnect.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/seq/aplaymidi/arecordmidi.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/seq/aplaymidi/aplaymidi.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/amixer/amixer.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/amidi/amidi.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/mainloop.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/widget.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/textbox.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/mixer_widget.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/utils.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/widget.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/colors.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/colors.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/textbox.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/card_select.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/card_select.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/device_name.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/mainloop.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/volume_mapping.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/mixer_widget.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/mixer_display.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/mixer_controls.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/die.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/device_name.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/proc_files.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/volume_mapping.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/mixer_controls.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/mem.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/utils.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/die.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/mem.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/proc_files.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/cli.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsamixer/mixer_display.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/iecset/iecset.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/iecset/iecbits.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsaloop/pcmjob.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsaloop/control.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsaloop/alsaloop.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsaloop/alsaloop.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/topology/topology.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsactl/init_parse.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsactl/daemon.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsactl/lock.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsactl/monitor.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsactl/init_utils_string.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsactl/list.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsactl/alsactl.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsactl/alsactl.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsactl/init_sysdeps.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsactl/init_utils_run.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsactl/utils.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsactl/init_sysfs.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsactl/state.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/alsaucm/usecase.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/speaker-test/pink.h"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/speaker-test/pink.c"
"/usr/src/debug/alsa-utils/1.2.1-r0/alsa-utils-1.2.1/speaker-test/speaker-test.c"

%files -n alsa-utils-dbg
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/sbin"
%dir "/usr/bin"
%dir "/usr/sbin/.debug"
"/usr/sbin/.debug/alsactl"
%dir "/usr/bin/.debug"
"/usr/bin/.debug/axfer"
"/usr/bin/.debug/aseqdump"
"/usr/bin/.debug/aplay"
"/usr/bin/.debug/aseqnet"
"/usr/bin/.debug/amixer"
"/usr/bin/.debug/amidi"
"/usr/bin/.debug/aconnect"
"/usr/bin/.debug/alsamixer"
"/usr/bin/.debug/iecset"
"/usr/bin/.debug/alsaloop"
"/usr/bin/.debug/arecordmidi"
"/usr/bin/.debug/alsaucm"
"/usr/bin/.debug/speaker-test"
"/usr/bin/.debug/aplaymidi"
"/usr/bin/.debug/alsatplg"

%files -n alsa-utils-dev
%defattr(-,-,-,-)

%files -n alsa-utils-doc
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/man"
%dir "/usr/share/man/man1"
%dir "/usr/share/man/fr"
%dir "/usr/share/man/man7"
%dir "/usr/share/man/man8"
"/usr/share/man/man1/alsaloop.1"
"/usr/share/man/man1/aseqnet.1"
"/usr/share/man/man1/aconnect.1"
"/usr/share/man/man1/axfer.1"
"/usr/share/man/man1/speaker-test.1"
"/usr/share/man/man1/alsa-info.sh.1"
"/usr/share/man/man1/aplay.1"
"/usr/share/man/man1/aplaymidi.1"
"/usr/share/man/man1/amixer.1"
"/usr/share/man/man1/axfer-transfer.1"
"/usr/share/man/man1/axfer-list.1"
"/usr/share/man/man1/arecordmidi.1"
"/usr/share/man/man1/aseqdump.1"
"/usr/share/man/man1/arecord.1"
"/usr/share/man/man1/alsactl.1"
"/usr/share/man/man1/amidi.1"
"/usr/share/man/man1/alsamixer.1"
"/usr/share/man/man1/iecset.1"
%dir "/usr/share/man/fr/man8"
"/usr/share/man/fr/man8/alsaconf.8"
"/usr/share/man/man8/alsaconf.8"

%files -n alsa-utils-alsamixer
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/bin"
"/usr/bin/alsamixer"

%files -n alsa-utils-alsatplg
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/bin"
"/usr/bin/alsatplg"

%files -n alsa-utils-midi
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/bin"
"/usr/bin/amidi"
"/usr/bin/arecordmidi"
"/usr/bin/aplaymidi"

%files -n alsa-utils-aplay
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/bin"
"/usr/bin/axfer"
"/usr/bin/aplay"
"/usr/bin/arecord"

%files -n alsa-utils-amixer
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/bin"
"/usr/bin/amixer"

%files -n alsa-utils-aconnect
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/bin"
"/usr/bin/aconnect"

%files -n alsa-utils-iecset
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/bin"
"/usr/bin/iecset"

%files -n alsa-utils-speakertest
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/bin"
%dir "/usr/share/sounds"
%dir "/usr/share/alsa"
%dir "/usr/share/sounds/alsa"
"/usr/share/sounds/alsa/Front_Center.wav"
"/usr/share/sounds/alsa/Rear_Right.wav"
"/usr/share/sounds/alsa/Side_Left.wav"
"/usr/share/sounds/alsa/Front_Right.wav"
"/usr/share/sounds/alsa/Rear_Left.wav"
"/usr/share/sounds/alsa/Front_Left.wav"
"/usr/share/sounds/alsa/Noise.wav"
"/usr/share/sounds/alsa/Rear_Center.wav"
"/usr/share/sounds/alsa/Side_Right.wav"
%dir "/usr/share/alsa/speaker-test"
"/usr/share/alsa/speaker-test/sample_map.csv"
"/usr/bin/speaker-test"

%files -n alsa-utils-aseqnet
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/bin"
"/usr/bin/aseqnet"

%files -n alsa-utils-aseqdump
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/bin"
"/usr/bin/aseqdump"

%files -n alsa-utils-alsactl
%defattr(-,-,-,-)
%dir "/var"
%dir "/usr"
%dir "/lib"
%dir "/var/lib"
%dir "/var/lib/alsa"
%dir "/usr/share"
%dir "/usr/sbin"
%dir "/usr/share/alsa"
%dir "/usr/share/alsa/init"
"/usr/share/alsa/init/default"
"/usr/share/alsa/init/00main"
"/usr/share/alsa/init/info"
"/usr/share/alsa/init/hda"
"/usr/share/alsa/init/test"
"/usr/share/alsa/init/help"
"/usr/share/alsa/init/ca0106"
"/usr/sbin/alsactl"
%dir "/lib/udev"
%dir "/lib/udev/rules.d"
"/lib/udev/rules.d/90-alsa-restore.rules"

%files -n alsa-utils-alsaloop
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/bin"
"/usr/bin/alsaloop"

%files -n alsa-utils-alsaucm
%defattr(-,-,-,-)
%dir "/usr"
%dir "/lib"
%dir "/usr/bin"
"/usr/bin/alsaucm"
%dir "/lib/udev"
%dir "/lib/udev/rules.d"
"/lib/udev/rules.d/89-alsa-ucm.rules"

%files -n alsa-utils-locale-de
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/de"
%dir "/usr/share/locale/de/LC_MESSAGES"
"/usr/share/locale/de/LC_MESSAGES/alsa-utils.mo"

%files -n alsa-utils-locale-fr
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/fr"
%dir "/usr/share/locale/fr/LC_MESSAGES"
"/usr/share/locale/fr/LC_MESSAGES/alsa-utils.mo"

%files -n alsa-utils-locale-ja
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/ja"
%dir "/usr/share/locale/ja/LC_MESSAGES"
"/usr/share/locale/ja/LC_MESSAGES/alsaconf.mo"
"/usr/share/locale/ja/LC_MESSAGES/alsa-utils.mo"

%files -n alsa-utils-locale-ru
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/locale"
%dir "/usr/share/locale/ru"
%dir "/usr/share/locale/ru/LC_MESSAGES"
"/usr/share/locale/ru/LC_MESSAGES/alsaconf.mo"

