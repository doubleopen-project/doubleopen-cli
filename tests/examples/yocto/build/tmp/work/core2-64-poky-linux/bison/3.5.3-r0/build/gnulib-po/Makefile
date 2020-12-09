# Makefile for PO directory in any package using GNU gettext.
# Copyright (C) 1995-1997, 2000-2007, 2009-2010 by Ulrich Drepper <drepper@gnu.ai.mit.edu>
#
# This file can be copied and used freely without restrictions.  It can
# be used in projects which are not available under the GNU General Public
# License but which still want to provide support for the GNU gettext
# functionality.
# Please note that the actual code of GNU gettext is covered by the GNU
# General Public License and is *not* in the public domain.
#
# Origin: gettext-0.19
GETTEXT_MACRO_VERSION = 0.19

PACKAGE = bison
VERSION = 3.5.3
PACKAGE_BUGREPORT = bug-bison@gnu.org

SED = sed
SHELL = /bin/sh


srcdir = ../../bison-3.5.3/gnulib-po
top_srcdir = ../../bison-3.5.3
VPATH = ../../bison-3.5.3/gnulib-po

prefix = /usr
exec_prefix = /usr
datarootdir = ${prefix}/share
datadir = /usr/share
localedir = ${datarootdir}/locale
gettextsrcdir = $(datadir)/gettext/po

INSTALL = /home/hhpartners/yocto/build/tmp/hosttools/install -c
INSTALL_DATA = ${INSTALL} -m 644

# We use $(mkdir_p).
# In automake <= 1.9.x, $(mkdir_p) is defined either as "mkdir -p --" or as
# "$(mkinstalldirs)" or as "$(install_sh) -d". For these automake versions,
# ${SHELL} /home/hhpartners/yocto/build/tmp/work/core2-64-poky-linux/bison/3.5.3-r0/bison-3.5.3/build-aux/install-sh does not start with $(SHELL), so we add it.
# In automake >= 1.10, $(MKDIR_P) is derived from ${MKDIR_P}, which is defined
# either as "/path/to/mkdir -p" or ".../install-sh -c -d". For these automake
# versions, $(mkinstalldirs) and $(install_sh) are unused.
mkinstalldirs = $(SHELL) ${SHELL} /home/hhpartners/yocto/build/tmp/work/core2-64-poky-linux/bison/3.5.3-r0/bison-3.5.3/build-aux/install-sh -d
install_sh = $(SHELL) ${SHELL} /home/hhpartners/yocto/build/tmp/work/core2-64-poky-linux/bison/3.5.3-r0/bison-3.5.3/build-aux/install-sh
MKDIR_P = /home/hhpartners/yocto/build/tmp/hosttools/mkdir -p
mkdir_p = $(MKDIR_P)

GMSGFMT_ = /home/hhpartners/yocto/build/tmp/work/core2-64-poky-linux/bison/3.5.3-r0/recipe-sysroot-native/usr/bin/msgfmt
GMSGFMT_no = /home/hhpartners/yocto/build/tmp/work/core2-64-poky-linux/bison/3.5.3-r0/recipe-sysroot-native/usr/bin/msgfmt
GMSGFMT_yes = /home/hhpartners/yocto/build/tmp/work/core2-64-poky-linux/bison/3.5.3-r0/recipe-sysroot-native/usr/bin/msgfmt
GMSGFMT = $(GMSGFMT_$(USE_MSGCTXT))
MSGFMT_ = /home/hhpartners/yocto/build/tmp/work/core2-64-poky-linux/bison/3.5.3-r0/recipe-sysroot-native/usr/bin/msgfmt
MSGFMT_no = /home/hhpartners/yocto/build/tmp/work/core2-64-poky-linux/bison/3.5.3-r0/recipe-sysroot-native/usr/bin/msgfmt
MSGFMT_yes = @MSGFMT_015@
MSGFMT = $(MSGFMT_$(USE_MSGCTXT))
XGETTEXT_ = /home/hhpartners/yocto/build/tmp/work/core2-64-poky-linux/bison/3.5.3-r0/recipe-sysroot-native/usr/bin/xgettext
XGETTEXT_no = /home/hhpartners/yocto/build/tmp/work/core2-64-poky-linux/bison/3.5.3-r0/recipe-sysroot-native/usr/bin/xgettext
XGETTEXT_yes = /home/hhpartners/yocto/build/tmp/work/core2-64-poky-linux/bison/3.5.3-r0/recipe-sysroot-native/usr/bin/xgettext
XGETTEXT = $(XGETTEXT_$(USE_MSGCTXT))
MSGMERGE = msgmerge
MSGMERGE_UPDATE = /home/hhpartners/yocto/build/tmp/work/core2-64-poky-linux/bison/3.5.3-r0/recipe-sysroot-native/usr/bin/msgmerge --update
MSGINIT = msginit
MSGCONV = msgconv
MSGFILTER = msgfilter

POFILES =  $(srcdir)/af.po $(srcdir)/be.po $(srcdir)/bg.po $(srcdir)/ca.po $(srcdir)/cs.po $(srcdir)/da.po $(srcdir)/de.po $(srcdir)/el.po $(srcdir)/eo.po $(srcdir)/es.po $(srcdir)/et.po $(srcdir)/eu.po $(srcdir)/fi.po $(srcdir)/fr.po $(srcdir)/ga.po $(srcdir)/gl.po $(srcdir)/hu.po $(srcdir)/it.po $(srcdir)/ja.po $(srcdir)/ko.po $(srcdir)/ms.po $(srcdir)/nb.po $(srcdir)/nl.po $(srcdir)/pl.po $(srcdir)/pt.po $(srcdir)/pt_BR.po $(srcdir)/ro.po $(srcdir)/ru.po $(srcdir)/rw.po $(srcdir)/sk.po $(srcdir)/sl.po $(srcdir)/sr.po $(srcdir)/sv.po $(srcdir)/tr.po $(srcdir)/uk.po $(srcdir)/vi.po $(srcdir)/zh_CN.po $(srcdir)/zh_TW.po
GMOFILES =  $(srcdir)/af.gmo $(srcdir)/be.gmo $(srcdir)/bg.gmo $(srcdir)/ca.gmo $(srcdir)/cs.gmo $(srcdir)/da.gmo $(srcdir)/de.gmo $(srcdir)/el.gmo $(srcdir)/eo.gmo $(srcdir)/es.gmo $(srcdir)/et.gmo $(srcdir)/eu.gmo $(srcdir)/fi.gmo $(srcdir)/fr.gmo $(srcdir)/ga.gmo $(srcdir)/gl.gmo $(srcdir)/hu.gmo $(srcdir)/it.gmo $(srcdir)/ja.gmo $(srcdir)/ko.gmo $(srcdir)/ms.gmo $(srcdir)/nb.gmo $(srcdir)/nl.gmo $(srcdir)/pl.gmo $(srcdir)/pt.gmo $(srcdir)/pt_BR.gmo $(srcdir)/ro.gmo $(srcdir)/ru.gmo $(srcdir)/rw.gmo $(srcdir)/sk.gmo $(srcdir)/sl.gmo $(srcdir)/sr.gmo $(srcdir)/sv.gmo $(srcdir)/tr.gmo $(srcdir)/uk.gmo $(srcdir)/vi.gmo $(srcdir)/zh_CN.gmo $(srcdir)/zh_TW.gmo
UPDATEPOFILES =  af.po-update be.po-update bg.po-update ca.po-update cs.po-update da.po-update de.po-update el.po-update eo.po-update es.po-update et.po-update eu.po-update fi.po-update fr.po-update ga.po-update gl.po-update hu.po-update it.po-update ja.po-update ko.po-update ms.po-update nb.po-update nl.po-update pl.po-update pt.po-update pt_BR.po-update ro.po-update ru.po-update rw.po-update sk.po-update sl.po-update sr.po-update sv.po-update tr.po-update uk.po-update vi.po-update zh_CN.po-update zh_TW.po-update
DUMMYPOFILES =  af.nop be.nop bg.nop ca.nop cs.nop da.nop de.nop el.nop eo.nop es.nop et.nop eu.nop fi.nop fr.nop ga.nop gl.nop hu.nop it.nop ja.nop ko.nop ms.nop nb.nop nl.nop pl.nop pt.nop pt_BR.nop ro.nop ru.nop rw.nop sk.nop sl.nop sr.nop sv.nop tr.nop uk.nop vi.nop zh_CN.nop zh_TW.nop
DISTFILES.common = Makefile.in.in remove-potcdate.sin \
$(DISTFILES.common.extra1) $(DISTFILES.common.extra2) $(DISTFILES.common.extra3)
DISTFILES = $(DISTFILES.common) Makevars POTFILES.in \
$(POFILES) $(GMOFILES) \
$(DISTFILES.extra1) $(DISTFILES.extra2) $(DISTFILES.extra3)

POTFILES = \
     ../../bison-3.5.3/lib/_Noreturn.h \
     ../../bison-3.5.3/lib/alignof.h \
     ../../bison-3.5.3/lib/alloca.in.h \
     ../../bison-3.5.3/lib/allocator.c \
     ../../bison-3.5.3/lib/allocator.h \
     ../../bison-3.5.3/lib/areadlink.c \
     ../../bison-3.5.3/lib/areadlink.h \
     ../../bison-3.5.3/lib/arg-nonnull.h \
     ../../bison-3.5.3/lib/argmatch.c \
     ../../bison-3.5.3/lib/argmatch.h \
     ../../bison-3.5.3/lib/asnprintf.c \
     ../../bison-3.5.3/lib/asprintf.c \
     ../../bison-3.5.3/lib/assure.h \
     ../../bison-3.5.3/lib/basename-lgpl.c \
     ../../bison-3.5.3/lib/basename.c \
     ../../bison-3.5.3/lib/binary-io.c \
     ../../bison-3.5.3/lib/binary-io.h \
     ../../bison-3.5.3/lib/bitrotate.c \
     ../../bison-3.5.3/lib/bitrotate.h \
     ../../bison-3.5.3/lib/bitset.c \
     ../../bison-3.5.3/lib/bitset.h \
     ../../bison-3.5.3/lib/bitset/array.c \
     ../../bison-3.5.3/lib/bitset/array.h \
     ../../bison-3.5.3/lib/bitset/base.h \
     ../../bison-3.5.3/lib/bitset/list.c \
     ../../bison-3.5.3/lib/bitset/list.h \
     ../../bison-3.5.3/lib/bitset/stats.c \
     ../../bison-3.5.3/lib/bitset/stats.h \
     ../../bison-3.5.3/lib/bitset/table.c \
     ../../bison-3.5.3/lib/bitset/table.h \
     ../../bison-3.5.3/lib/bitset/vector.c \
     ../../bison-3.5.3/lib/bitset/vector.h \
     ../../bison-3.5.3/lib/bitsetv.c \
     ../../bison-3.5.3/lib/bitsetv.h \
     ../../bison-3.5.3/lib/c++defs.h \
     ../../bison-3.5.3/lib/c-ctype.c \
     ../../bison-3.5.3/lib/c-ctype.h \
     ../../bison-3.5.3/lib/c-strcase.h \
     ../../bison-3.5.3/lib/c-strcasecmp.c \
     ../../bison-3.5.3/lib/c-strcaseeq.h \
     ../../bison-3.5.3/lib/c-strncasecmp.c \
     ../../bison-3.5.3/lib/calloc.c \
     ../../bison-3.5.3/lib/canonicalize-lgpl.c \
     ../../bison-3.5.3/lib/careadlinkat.c \
     ../../bison-3.5.3/lib/careadlinkat.h \
     ../../bison-3.5.3/lib/cdefs.h \
     ../../bison-3.5.3/lib/cloexec.c \
     ../../bison-3.5.3/lib/cloexec.h \
     ../../bison-3.5.3/lib/close-stream.c \
     ../../bison-3.5.3/lib/close-stream.h \
     ../../bison-3.5.3/lib/close.c \
     ../../bison-3.5.3/lib/closeout.c \
     ../../bison-3.5.3/lib/closeout.h \
     ../../bison-3.5.3/lib/concat-filename.c \
     ../../bison-3.5.3/lib/concat-filename.h \
     ../../bison-3.5.3/lib/diffseq.h \
     ../../bison-3.5.3/lib/dirname-lgpl.c \
     ../../bison-3.5.3/lib/dirname.c \
     ../../bison-3.5.3/lib/dirname.h \
     ../../bison-3.5.3/lib/dosname.h \
     ../../bison-3.5.3/lib/dup-safer-flag.c \
     ../../bison-3.5.3/lib/dup-safer.c \
     ../../bison-3.5.3/lib/dup2.c \
     ../../bison-3.5.3/lib/errno.in.h \
     ../../bison-3.5.3/lib/error.c \
     ../../bison-3.5.3/lib/error.h \
     ../../bison-3.5.3/lib/exitfail.c \
     ../../bison-3.5.3/lib/exitfail.h \
     ../../bison-3.5.3/lib/fatal-signal.c \
     ../../bison-3.5.3/lib/fatal-signal.h \
     ../../bison-3.5.3/lib/fcntl.c \
     ../../bison-3.5.3/lib/fcntl.in.h \
     ../../bison-3.5.3/lib/fd-hook.c \
     ../../bison-3.5.3/lib/fd-hook.h \
     ../../bison-3.5.3/lib/fd-safer-flag.c \
     ../../bison-3.5.3/lib/fd-safer.c \
     ../../bison-3.5.3/lib/filename.h \
     ../../bison-3.5.3/lib/float+.h \
     ../../bison-3.5.3/lib/float.c \
     ../../bison-3.5.3/lib/float.in.h \
     ../../bison-3.5.3/lib/fopen-safer.c \
     ../../bison-3.5.3/lib/fopen.c \
     ../../bison-3.5.3/lib/fpending.c \
     ../../bison-3.5.3/lib/fpending.h \
     ../../bison-3.5.3/lib/fprintf.c \
     ../../bison-3.5.3/lib/fpucw.h \
     ../../bison-3.5.3/lib/frexp.c \
     ../../bison-3.5.3/lib/frexpl.c \
     ../../bison-3.5.3/lib/fseterr.c \
     ../../bison-3.5.3/lib/fseterr.h \
     ../../bison-3.5.3/lib/fstat.c \
     ../../bison-3.5.3/lib/fstrcmp.c \
     ../../bison-3.5.3/lib/fstrcmp.h \
     ../../bison-3.5.3/lib/fsync.c \
     ../../bison-3.5.3/lib/getdtablesize.c \
     ../../bison-3.5.3/lib/gethrxtime.c \
     ../../bison-3.5.3/lib/gethrxtime.h \
     ../../bison-3.5.3/lib/getopt-cdefs.in.h \
     ../../bison-3.5.3/lib/getopt-core.h \
     ../../bison-3.5.3/lib/getopt-ext.h \
     ../../bison-3.5.3/lib/getopt-pfx-core.h \
     ../../bison-3.5.3/lib/getopt-pfx-ext.h \
     ../../bison-3.5.3/lib/getopt.c \
     ../../bison-3.5.3/lib/getopt.in.h \
     ../../bison-3.5.3/lib/getopt1.c \
     ../../bison-3.5.3/lib/getopt_int.h \
     ../../bison-3.5.3/lib/getprogname.c \
     ../../bison-3.5.3/lib/getprogname.h \
     ../../bison-3.5.3/lib/getrusage.c \
     ../../bison-3.5.3/lib/gettext.h \
     ../../bison-3.5.3/lib/gettime.c \
     ../../bison-3.5.3/lib/gettimeofday.c \
     ../../bison-3.5.3/lib/gl_array_list.c \
     ../../bison-3.5.3/lib/gl_array_list.h \
     ../../bison-3.5.3/lib/gl_list.c \
     ../../bison-3.5.3/lib/gl_list.h \
     ../../bison-3.5.3/lib/gl_xlist.c \
     ../../bison-3.5.3/lib/gl_xlist.h \
     ../../bison-3.5.3/lib/glthread/lock.c \
     ../../bison-3.5.3/lib/glthread/lock.h \
     ../../bison-3.5.3/lib/glthread/threadlib.c \
     ../../bison-3.5.3/lib/glthread/tls.c \
     ../../bison-3.5.3/lib/glthread/tls.h \
     ../../bison-3.5.3/lib/hard-locale.c \
     ../../bison-3.5.3/lib/hard-locale.h \
     ../../bison-3.5.3/lib/hash.c \
     ../../bison-3.5.3/lib/hash.h \
     ../../bison-3.5.3/lib/intprops.h \
     ../../bison-3.5.3/lib/inttypes.in.h \
     ../../bison-3.5.3/lib/isnan.c \
     ../../bison-3.5.3/lib/isnand-nolibm.h \
     ../../bison-3.5.3/lib/isnand.c \
     ../../bison-3.5.3/lib/isnanf-nolibm.h \
     ../../bison-3.5.3/lib/isnanf.c \
     ../../bison-3.5.3/lib/isnanl-nolibm.h \
     ../../bison-3.5.3/lib/isnanl.c \
     ../../bison-3.5.3/lib/iswblank.c \
     ../../bison-3.5.3/lib/iswdigit.c \
     ../../bison-3.5.3/lib/iswxdigit.c \
     ../../bison-3.5.3/lib/itold.c \
     ../../bison-3.5.3/lib/lc-charset-dispatch.c \
     ../../bison-3.5.3/lib/lc-charset-dispatch.h \
     ../../bison-3.5.3/lib/ldexpl.c \
     ../../bison-3.5.3/lib/libc-config.h \
     ../../bison-3.5.3/lib/limits.in.h \
     ../../bison-3.5.3/lib/localcharset.c \
     ../../bison-3.5.3/lib/localcharset.h \
     ../../bison-3.5.3/lib/locale.in.h \
     ../../bison-3.5.3/lib/localtime-buffer.c \
     ../../bison-3.5.3/lib/localtime-buffer.h \
     ../../bison-3.5.3/lib/lstat.c \
     ../../bison-3.5.3/lib/malloc.c \
     ../../bison-3.5.3/lib/malloca.c \
     ../../bison-3.5.3/lib/malloca.h \
     ../../bison-3.5.3/lib/math.c \
     ../../bison-3.5.3/lib/math.in.h \
     ../../bison-3.5.3/lib/mbchar.c \
     ../../bison-3.5.3/lib/mbchar.h \
     ../../bison-3.5.3/lib/mbfile.c \
     ../../bison-3.5.3/lib/mbfile.h \
     ../../bison-3.5.3/lib/mbrtowc-impl-utf8.h \
     ../../bison-3.5.3/lib/mbrtowc-impl.h \
     ../../bison-3.5.3/lib/mbrtowc.c \
     ../../bison-3.5.3/lib/mbsinit.c \
     ../../bison-3.5.3/lib/mbswidth.c \
     ../../bison-3.5.3/lib/mbswidth.h \
     ../../bison-3.5.3/lib/mbtowc-lock.c \
     ../../bison-3.5.3/lib/mbtowc-lock.h \
     ../../bison-3.5.3/lib/memchr.c \
     ../../bison-3.5.3/lib/memchr.valgrind \
     ../../bison-3.5.3/lib/minmax.h \
     ../../bison-3.5.3/lib/msvc-inval.c \
     ../../bison-3.5.3/lib/msvc-inval.h \
     ../../bison-3.5.3/lib/msvc-nothrow.c \
     ../../bison-3.5.3/lib/msvc-nothrow.h \
     ../../bison-3.5.3/lib/obstack.c \
     ../../bison-3.5.3/lib/obstack.h \
     ../../bison-3.5.3/lib/obstack_printf.c \
     ../../bison-3.5.3/lib/open.c \
     ../../bison-3.5.3/lib/pathmax.h \
     ../../bison-3.5.3/lib/perror.c \
     ../../bison-3.5.3/lib/pipe-safer.c \
     ../../bison-3.5.3/lib/pipe2-safer.c \
     ../../bison-3.5.3/lib/pipe2.c \
     ../../bison-3.5.3/lib/printf-args.c \
     ../../bison-3.5.3/lib/printf-args.h \
     ../../bison-3.5.3/lib/printf-frexp.c \
     ../../bison-3.5.3/lib/printf-frexp.h \
     ../../bison-3.5.3/lib/printf-frexpl.c \
     ../../bison-3.5.3/lib/printf-frexpl.h \
     ../../bison-3.5.3/lib/printf-parse.c \
     ../../bison-3.5.3/lib/printf-parse.h \
     ../../bison-3.5.3/lib/printf.c \
     ../../bison-3.5.3/lib/progname.c \
     ../../bison-3.5.3/lib/progname.h \
     ../../bison-3.5.3/lib/progreloc.c \
     ../../bison-3.5.3/lib/quote.h \
     ../../bison-3.5.3/lib/quotearg.c \
     ../../bison-3.5.3/lib/quotearg.h \
     ../../bison-3.5.3/lib/raise.c \
     ../../bison-3.5.3/lib/rawmemchr.c \
     ../../bison-3.5.3/lib/rawmemchr.valgrind \
     ../../bison-3.5.3/lib/readlink.c \
     ../../bison-3.5.3/lib/realloc.c \
     ../../bison-3.5.3/lib/relocatable.c \
     ../../bison-3.5.3/lib/relocatable.h \
     ../../bison-3.5.3/lib/relocwrapper.c \
     ../../bison-3.5.3/lib/rename.c \
     ../../bison-3.5.3/lib/rmdir.c \
     ../../bison-3.5.3/lib/same-inode.h \
     ../../bison-3.5.3/lib/sched.in.h \
     ../../bison-3.5.3/lib/setenv.c \
     ../../bison-3.5.3/lib/setlocale-lock.c \
     ../../bison-3.5.3/lib/setlocale_null.c \
     ../../bison-3.5.3/lib/setlocale_null.h \
     ../../bison-3.5.3/lib/sig-handler.c \
     ../../bison-3.5.3/lib/sig-handler.h \
     ../../bison-3.5.3/lib/sigaction.c \
     ../../bison-3.5.3/lib/signal.in.h \
     ../../bison-3.5.3/lib/signbitd.c \
     ../../bison-3.5.3/lib/signbitf.c \
     ../../bison-3.5.3/lib/signbitl.c \
     ../../bison-3.5.3/lib/sigprocmask.c \
     ../../bison-3.5.3/lib/size_max.h \
     ../../bison-3.5.3/lib/snprintf.c \
     ../../bison-3.5.3/lib/spawn-pipe.c \
     ../../bison-3.5.3/lib/spawn-pipe.h \
     ../../bison-3.5.3/lib/spawn.in.h \
     ../../bison-3.5.3/lib/spawn_faction_addclose.c \
     ../../bison-3.5.3/lib/spawn_faction_adddup2.c \
     ../../bison-3.5.3/lib/spawn_faction_addopen.c \
     ../../bison-3.5.3/lib/spawn_faction_destroy.c \
     ../../bison-3.5.3/lib/spawn_faction_init.c \
     ../../bison-3.5.3/lib/spawn_int.h \
     ../../bison-3.5.3/lib/spawnattr_destroy.c \
     ../../bison-3.5.3/lib/spawnattr_init.c \
     ../../bison-3.5.3/lib/spawnattr_setflags.c \
     ../../bison-3.5.3/lib/spawnattr_setsigmask.c \
     ../../bison-3.5.3/lib/spawni.c \
     ../../bison-3.5.3/lib/spawnp.c \
     ../../bison-3.5.3/lib/sprintf.c \
     ../../bison-3.5.3/lib/stat-time.c \
     ../../bison-3.5.3/lib/stat-time.h \
     ../../bison-3.5.3/lib/stat-w32.c \
     ../../bison-3.5.3/lib/stat-w32.h \
     ../../bison-3.5.3/lib/stat.c \
     ../../bison-3.5.3/lib/stdbool.in.h \
     ../../bison-3.5.3/lib/stddef.in.h \
     ../../bison-3.5.3/lib/stdint.in.h \
     ../../bison-3.5.3/lib/stdio--.h \
     ../../bison-3.5.3/lib/stdio-impl.h \
     ../../bison-3.5.3/lib/stdio-safer.h \
     ../../bison-3.5.3/lib/stdio.in.h \
     ../../bison-3.5.3/lib/stdlib.in.h \
     ../../bison-3.5.3/lib/stpcpy.c \
     ../../bison-3.5.3/lib/strchrnul.c \
     ../../bison-3.5.3/lib/strchrnul.valgrind \
     ../../bison-3.5.3/lib/strdup.c \
     ../../bison-3.5.3/lib/streq.h \
     ../../bison-3.5.3/lib/strerror-override.c \
     ../../bison-3.5.3/lib/strerror-override.h \
     ../../bison-3.5.3/lib/strerror.c \
     ../../bison-3.5.3/lib/strerror_r.c \
     ../../bison-3.5.3/lib/string.in.h \
     ../../bison-3.5.3/lib/stripslash.c \
     ../../bison-3.5.3/lib/strndup.c \
     ../../bison-3.5.3/lib/strnlen.c \
     ../../bison-3.5.3/lib/strverscmp.c \
     ../../bison-3.5.3/lib/sys_resource.in.h \
     ../../bison-3.5.3/lib/sys_stat.in.h \
     ../../bison-3.5.3/lib/sys_time.in.h \
     ../../bison-3.5.3/lib/sys_times.in.h \
     ../../bison-3.5.3/lib/sys_types.in.h \
     ../../bison-3.5.3/lib/sys_wait.in.h \
     ../../bison-3.5.3/lib/textstyle.in.h \
     ../../bison-3.5.3/lib/time.in.h \
     ../../bison-3.5.3/lib/timespec.c \
     ../../bison-3.5.3/lib/timespec.h \
     ../../bison-3.5.3/lib/timevar.c \
     ../../bison-3.5.3/lib/timevar.h \
     ../../bison-3.5.3/lib/unistd--.h \
     ../../bison-3.5.3/lib/unistd-safer.h \
     ../../bison-3.5.3/lib/unistd.c \
     ../../bison-3.5.3/lib/unistd.in.h \
     ../../bison-3.5.3/lib/unitypes.in.h \
     ../../bison-3.5.3/lib/uniwidth.in.h \
     ../../bison-3.5.3/lib/uniwidth/cjk.h \
     ../../bison-3.5.3/lib/uniwidth/width.c \
     ../../bison-3.5.3/lib/unlink.c \
     ../../bison-3.5.3/lib/unlocked-io.h \
     ../../bison-3.5.3/lib/unsetenv.c \
     ../../bison-3.5.3/lib/vasnprintf.c \
     ../../bison-3.5.3/lib/vasnprintf.h \
     ../../bison-3.5.3/lib/vasprintf.c \
     ../../bison-3.5.3/lib/verify.h \
     ../../bison-3.5.3/lib/vfprintf.c \
     ../../bison-3.5.3/lib/vsnprintf.c \
     ../../bison-3.5.3/lib/vsprintf.c \
     ../../bison-3.5.3/lib/w32spawn.h \
     ../../bison-3.5.3/lib/wait-process.c \
     ../../bison-3.5.3/lib/wait-process.h \
     ../../bison-3.5.3/lib/waitpid.c \
     ../../bison-3.5.3/lib/warn-on-use.h \
     ../../bison-3.5.3/lib/wchar.in.h \
     ../../bison-3.5.3/lib/wctype-h.c \
     ../../bison-3.5.3/lib/wctype.in.h \
     ../../bison-3.5.3/lib/wcwidth.c \
     ../../bison-3.5.3/lib/windows-initguard.h \
     ../../bison-3.5.3/lib/windows-mutex.c \
     ../../bison-3.5.3/lib/windows-mutex.h \
     ../../bison-3.5.3/lib/windows-once.c \
     ../../bison-3.5.3/lib/windows-once.h \
     ../../bison-3.5.3/lib/windows-recmutex.c \
     ../../bison-3.5.3/lib/windows-recmutex.h \
     ../../bison-3.5.3/lib/windows-rwlock.c \
     ../../bison-3.5.3/lib/windows-rwlock.h \
     ../../bison-3.5.3/lib/windows-tls.c \
     ../../bison-3.5.3/lib/windows-tls.h \
     ../../bison-3.5.3/lib/xalloc-die.c \
     ../../bison-3.5.3/lib/xalloc-oversized.h \
     ../../bison-3.5.3/lib/xalloc.h \
     ../../bison-3.5.3/lib/xconcat-filename.c \
     ../../bison-3.5.3/lib/xhash.c \
     ../../bison-3.5.3/lib/xmalloc.c \
     ../../bison-3.5.3/lib/xmemdup0.c \
     ../../bison-3.5.3/lib/xmemdup0.h \
     ../../bison-3.5.3/lib/xreadlink.c \
     ../../bison-3.5.3/lib/xreadlink.h \
     ../../bison-3.5.3/lib/xsize.c \
     ../../bison-3.5.3/lib/xsize.h \
     ../../bison-3.5.3/lib/xstrndup.c \
     ../../bison-3.5.3/lib/xstrndup.h \
     ../../bison-3.5.3/lib/xtime.c \
     ../../bison-3.5.3/lib/xtime.h

CATALOGS =  af.gmo be.gmo bg.gmo ca.gmo cs.gmo da.gmo de.gmo el.gmo eo.gmo es.gmo et.gmo eu.gmo fi.gmo fr.gmo ga.gmo gl.gmo hu.gmo it.gmo ja.gmo ko.gmo ms.gmo nb.gmo nl.gmo pl.gmo pt.gmo pt_BR.gmo ro.gmo ru.gmo rw.gmo sk.gmo sl.gmo sr.gmo sv.gmo tr.gmo uk.gmo vi.gmo zh_CN.gmo zh_TW.gmo

POFILESDEPS_ = $(srcdir)/$(DOMAIN).pot
POFILESDEPS_yes = $(POFILESDEPS_)
POFILESDEPS_no =
POFILESDEPS = $(POFILESDEPS_$(PO_DEPENDS_ON_POT))

DISTFILESDEPS_ = update-po
DISTFILESDEPS_yes = $(DISTFILESDEPS_)
DISTFILESDEPS_no =
DISTFILESDEPS = $(DISTFILESDEPS_$(DIST_DEPENDS_ON_UPDATE_PO))

# Makevars gets inserted here. (Don't remove this line!)
## DO NOT EDIT! GENERATED AUTOMATICALLY!
# Copyright (C) 2002-2020 Free Software Foundation, Inc.
#
# This file is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 3 of the License, or
# (at your option) any later version.
#
# This file is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this file.  If not, see <https://www.gnu.org/licenses/>.
#
# As a special exception to the GNU General Public License,
# this file may be distributed as part of a program that
# contains a configuration script generated by Autoconf, under
# the same distribution terms as the rest of that program.
#
# Generated by gnulib-tool.

# Usually the message domain is the same as the package name.
# But here it has a '-gnulib' suffix.
DOMAIN = bison-gnulib

# These two variables depend on the location of this directory.
subdir = gnulib-po
top_builddir = ..

# These options get passed to xgettext.
XGETTEXT_OPTIONS = \
  --keyword=_ --flag=_:1:pass-c-format \
  --keyword=N_ --flag=N_:1:pass-c-format \
  --keyword='proper_name:1,"This is a proper name. See the gettext manual, section Names."' \
  --keyword='proper_name_utf8:1,"This is a proper name. See the gettext manual, section Names."' \
  --flag=error:3:c-format --flag=error_at_line:5:c-format

# This is the copyright holder that gets inserted into the header of the
# $(DOMAIN).pot file.  gnulib is copyrighted by the FSF.
COPYRIGHT_HOLDER = Free Software Foundation, Inc.

# This is the email address or URL to which the translators shall report
# bugs in the untranslated strings:
# - Strings which are not entire sentences, see the maintainer guidelines
#   in the GNU gettext documentation, section 'Preparing Strings'.
# - Strings which use unclear terms or require additional context to be
#   understood.
# - Strings which make invalid assumptions about notation of date, time or
#   money.
# - Pluralisation problems.
# - Incorrect English spelling.
# - Incorrect formatting.
# It can be your email address, or a mailing list address where translators
# can write to without being subscribed, or the URL of a web page through
# which the translators can contact you.
MSGID_BUGS_ADDRESS = bug-gnulib@gnu.org

# This is the list of locale categories, beyond LC_MESSAGES, for which the
# message catalogs shall be used.  It is usually empty.
EXTRA_LOCALE_CATEGORIES =

# This tells whether the $(DOMAIN).pot file contains messages with an 'msgctxt'
# context.  Possible values are "yes" and "no".  Set this to yes if the
# package uses functions taking also a message context, like pgettext(), or
# if in $(XGETTEXT_OPTIONS) you define keywords with a context argument.
USE_MSGCTXT = no

.SUFFIXES:
.SUFFIXES: .po .gmo .mo .sed .sin .nop .po-create .po-update

.po.mo:
	@echo "$(MSGFMT) -c -o $@ $<"; \
	$(MSGFMT) -c -o t-$@ $< && mv t-$@ $@

.po.gmo:
	@lang=`echo $* | sed -e 's,.*/,,'`; \
	test "$(srcdir)" = . && cdcmd="" || cdcmd="cd $(srcdir) && "; \
	echo "$${cdcmd}rm -f $${lang}.gmo && $(GMSGFMT) -c --statistics --verbose -o $${lang}.gmo $${lang}.po"; \
	cd $(srcdir) && rm -f $${lang}.gmo && $(GMSGFMT) -c --statistics --verbose -o t-$${lang}.gmo $${lang}.po && mv t-$${lang}.gmo $${lang}.gmo

.sin.sed:
	sed -e '/^#/d' $< > t-$@
	mv t-$@ $@


all: all-yes

all-yes: stamp-po
all-no:

# Ensure that the gettext macros and this Makefile.in.in are in sync.
CHECK_MACRO_VERSION = \
	test "$(GETTEXT_MACRO_VERSION)" = "0.20" \
	  || { echo "*** error: gettext infrastructure mismatch: using a Makefile.in.in from gettext version $(GETTEXT_MACRO_VERSION) but the autoconf macros are from gettext version 0.20" 1>&2; \
	       exit 1; \
	     }

# $(srcdir)/$(DOMAIN).pot is only created when needed. When xgettext finds no
# internationalized messages, no $(srcdir)/$(DOMAIN).pot is created (because
# we don't want to bother translators with empty POT files). We assume that
# LINGUAS is empty in this case, i.e. $(POFILES) and $(GMOFILES) are empty.
# In this case, stamp-po is a nop (i.e. a phony target).

# stamp-po is a timestamp denoting the last time at which the CATALOGS have
# been loosely updated. Its purpose is that when a developer or translator
# checks out the package via CVS, and the $(DOMAIN).pot file is not in CVS,
# "make" will update the $(DOMAIN).pot and the $(CATALOGS), but subsequent
# invocations of "make" will do nothing. This timestamp would not be necessary
# if updating the $(CATALOGS) would always touch them; however, the rule for
# $(POFILES) has been designed to not touch files that don't need to be
# changed.
stamp-po: $(srcdir)/$(DOMAIN).pot
	@$(CHECK_MACRO_VERSION)
	test ! -f $(srcdir)/$(DOMAIN).pot || \
	  test -z "$(GMOFILES)" || $(MAKE) $(GMOFILES)
	@test ! -f $(srcdir)/$(DOMAIN).pot || { \
	  echo "touch stamp-po" && \
	  echo timestamp > stamp-poT && \
	  mv stamp-poT stamp-po; \
	}

# Note: Target 'all' must not depend on target '$(DOMAIN).pot-update',
# otherwise packages like GCC can not be built if only parts of the source
# have been downloaded.

# This target rebuilds $(DOMAIN).pot; it is an expensive operation.
# Note that $(DOMAIN).pot is not touched if it doesn't need to be changed.
# The determination of whether the package xyz is a GNU one is based on the
# heuristic whether some file in the top level directory mentions "GNU xyz".
# If GNU 'find' is available, we avoid grepping through monster files.
$(DOMAIN).pot-update: $(POTFILES) $(srcdir)/POTFILES.in remove-potcdate.sed
	package_gnu="$(PACKAGE_GNU)"; \
	test -n "$$package_gnu" || { \
	  if { if (LC_ALL=C find --version) 2>/dev/null | grep GNU >/dev/null; then \
		 LC_ALL=C find -L $(top_srcdir) -maxdepth 1 -type f \
			       -size -10000000c -exec grep 'GNU bison' \
			       /dev/null '{}' ';' 2>/dev/null; \
	       else \
		 LC_ALL=C grep 'GNU bison' $(top_srcdir)/* 2>/dev/null; \
	       fi; \
	     } | grep -v 'libtool:' >/dev/null; then \
	     package_gnu=yes; \
	   else \
	     package_gnu=no; \
	   fi; \
	}; \
	if test "$$package_gnu" = "yes"; then \
	  package_prefix='GNU '; \
	else \
	  package_prefix=''; \
	fi; \
	if test -n '$(MSGID_BUGS_ADDRESS)' || test '$(PACKAGE_BUGREPORT)' = '@'PACKAGE_BUGREPORT'@'; then \
	  msgid_bugs_address='$(MSGID_BUGS_ADDRESS)'; \
	else \
	  msgid_bugs_address='$(PACKAGE_BUGREPORT)'; \
	fi; \
	case `$(XGETTEXT) --version | sed 1q | sed -e 's,^[^0-9]*,,'` in \
	  '' | 0.[0-9] | 0.[0-9].* | 0.1[0-5] | 0.1[0-5].* | 0.16 | 0.16.[0-1]*) \
	    $(XGETTEXT) --default-domain=$(DOMAIN) --directory=$(top_srcdir) \
	      --add-comments=TRANSLATORS: $(XGETTEXT_OPTIONS)  --flag=error:3:c-format --flag=error_at_line:5:c-format --flag=asprintf:2:c-format --flag=vasprintf:2:c-format \
	      --files-from=$(srcdir)/POTFILES.in \
	      --copyright-holder='$(COPYRIGHT_HOLDER)' \
	      --msgid-bugs-address="$$msgid_bugs_address" \
	    ;; \
	  *) \
	    $(XGETTEXT) --default-domain=$(DOMAIN) --directory=$(top_srcdir) \
	      --add-comments=TRANSLATORS: $(XGETTEXT_OPTIONS)  --flag=error:3:c-format --flag=error_at_line:5:c-format --flag=asprintf:2:c-format --flag=vasprintf:2:c-format \
	      --files-from=$(srcdir)/POTFILES.in \
	      --copyright-holder='$(COPYRIGHT_HOLDER)' \
	      --package-name="$${package_prefix}bison" \
	      --package-version='3.5.3' \
	      --msgid-bugs-address="$$msgid_bugs_address" \
	    ;; \
	esac
	test ! -f $(DOMAIN).po || { \
	  if test -f $(srcdir)/$(DOMAIN).pot; then \
	    sed -f remove-potcdate.sed < $(srcdir)/$(DOMAIN).pot > $(DOMAIN).1po && \
	    sed -f remove-potcdate.sed < $(DOMAIN).po > $(DOMAIN).2po && \
	    if cmp $(DOMAIN).1po $(DOMAIN).2po >/dev/null 2>&1; then \
	      rm -f $(DOMAIN).1po $(DOMAIN).2po $(DOMAIN).po; \
	    else \
	      rm -f $(DOMAIN).1po $(DOMAIN).2po $(srcdir)/$(DOMAIN).pot && \
	      mv $(DOMAIN).po $(srcdir)/$(DOMAIN).pot; \
	    fi; \
	  else \
	    mv $(DOMAIN).po $(srcdir)/$(DOMAIN).pot; \
	  fi; \
	}

# This rule has no dependencies: we don't need to update $(DOMAIN).pot at
# every "make" invocation, only create it when it is missing.
# Only "make $(DOMAIN).pot-update" or "make dist" will force an update.
$(srcdir)/$(DOMAIN).pot:
	$(MAKE) $(DOMAIN).pot-update

# This target rebuilds a PO file if $(DOMAIN).pot has changed.
# Note that a PO file is not touched if it doesn't need to be changed.
$(POFILES): $(POFILESDEPS)
	@lang=`echo $@ | sed -e 's,.*/,,' -e 's/\.po$$//'`; \
	if test -f "$(srcdir)/$${lang}.po"; then \
	  test -f $(srcdir)/$(DOMAIN).pot || $(MAKE) $(srcdir)/$(DOMAIN).pot; \
	  test "$(srcdir)" = . && cdcmd="" || cdcmd="cd $(srcdir) && "; \
	  echo "$${cdcmd}$(MSGMERGE_UPDATE) $(MSGMERGE_OPTIONS) --lang=$${lang} $${lang}.po $(DOMAIN).pot"; \
	  cd $(srcdir) \
	    && { case `$(MSGMERGE_UPDATE) --version | sed 1q | sed -e 's,^[^0-9]*,,'` in \
	           '' | 0.[0-9] | 0.[0-9].* | 0.1[0-7] | 0.1[0-7].*) \
	             $(MSGMERGE_UPDATE) $(MSGMERGE_OPTIONS) $${lang}.po $(DOMAIN).pot;; \
	           *) \
	             $(MSGMERGE_UPDATE) $(MSGMERGE_OPTIONS) --lang=$${lang} $${lang}.po $(DOMAIN).pot;; \
	         esac; \
	       }; \
	else \
	  $(MAKE) $${lang}.po-create; \
	fi


install: install-exec install-data
install-exec:
install-data: install-data-yes
	if test "$(PACKAGE)" = "gettext-tools"; then \
	  $(mkdir_p) $(DESTDIR)$(gettextsrcdir); \
	  for file in $(DISTFILES.common) Makevars.template; do \
	    $(INSTALL_DATA) $(srcdir)/$$file \
			    $(DESTDIR)$(gettextsrcdir)/$$file; \
	  done; \
	  for file in Makevars; do \
	    rm -f $(DESTDIR)$(gettextsrcdir)/$$file; \
	  done; \
	else \
	  : ; \
	fi
install-data-no: all
install-data-yes: all
	@catalogs='$(CATALOGS)'; \
	for cat in $$catalogs; do \
	  cat=`basename $$cat`; \
	  lang=`echo $$cat | sed -e 's/\.gmo$$//'`; \
	  dir=$(localedir)/$$lang/LC_MESSAGES; \
	  $(mkdir_p) $(DESTDIR)$$dir; \
	  if test -r $$cat; then realcat=$$cat; else realcat=$(srcdir)/$$cat; fi; \
	  $(INSTALL_DATA) $$realcat $(DESTDIR)$$dir/$(DOMAIN).mo; \
	  echo "installing $$realcat as $(DESTDIR)$$dir/$(DOMAIN).mo"; \
	  for lc in '' $(EXTRA_LOCALE_CATEGORIES); do \
	    if test -n "$$lc"; then \
	      if (cd $(DESTDIR)$(localedir)/$$lang && LC_ALL=C ls -l -d $$lc 2>/dev/null) | grep ' -> ' >/dev/null; then \
	        link=`cd $(DESTDIR)$(localedir)/$$lang && LC_ALL=C ls -l -d $$lc | sed -e 's/^.* -> //'`; \
	        mv $(DESTDIR)$(localedir)/$$lang/$$lc $(DESTDIR)$(localedir)/$$lang/$$lc.old; \
	        mkdir $(DESTDIR)$(localedir)/$$lang/$$lc; \
	        (cd $(DESTDIR)$(localedir)/$$lang/$$lc.old && \
	         for file in *; do \
	           if test -f $$file; then \
	             ln -s ../$$link/$$file $(DESTDIR)$(localedir)/$$lang/$$lc/$$file; \
	           fi; \
	         done); \
	        rm -f $(DESTDIR)$(localedir)/$$lang/$$lc.old; \
	      else \
	        if test -d $(DESTDIR)$(localedir)/$$lang/$$lc; then \
	          :; \
	        else \
	          rm -f $(DESTDIR)$(localedir)/$$lang/$$lc; \
	          mkdir $(DESTDIR)$(localedir)/$$lang/$$lc; \
	        fi; \
	      fi; \
	      rm -f $(DESTDIR)$(localedir)/$$lang/$$lc/$(DOMAIN).mo; \
	      ln -s ../LC_MESSAGES/$(DOMAIN).mo $(DESTDIR)$(localedir)/$$lang/$$lc/$(DOMAIN).mo 2>/dev/null || \
	      ln $(DESTDIR)$(localedir)/$$lang/LC_MESSAGES/$(DOMAIN).mo $(DESTDIR)$(localedir)/$$lang/$$lc/$(DOMAIN).mo 2>/dev/null || \
	      cp -p $(DESTDIR)$(localedir)/$$lang/LC_MESSAGES/$(DOMAIN).mo $(DESTDIR)$(localedir)/$$lang/$$lc/$(DOMAIN).mo; \
	      echo "installing $$realcat link as $(DESTDIR)$(localedir)/$$lang/$$lc/$(DOMAIN).mo"; \
	    fi; \
	  done; \
	done

install-strip: install

installdirs: installdirs-exec installdirs-data
installdirs-exec:
installdirs-data: installdirs-data-yes
	if test "$(PACKAGE)" = "gettext-tools"; then \
	  $(mkdir_p) $(DESTDIR)$(gettextsrcdir); \
	else \
	  : ; \
	fi
installdirs-data-no:
installdirs-data-yes:
	@catalogs='$(CATALOGS)'; \
	for cat in $$catalogs; do \
	  cat=`basename $$cat`; \
	  lang=`echo $$cat | sed -e 's/\.gmo$$//'`; \
	  dir=$(localedir)/$$lang/LC_MESSAGES; \
	  $(mkdir_p) $(DESTDIR)$$dir; \
	  for lc in '' $(EXTRA_LOCALE_CATEGORIES); do \
	    if test -n "$$lc"; then \
	      if (cd $(DESTDIR)$(localedir)/$$lang && LC_ALL=C ls -l -d $$lc 2>/dev/null) | grep ' -> ' >/dev/null; then \
	        link=`cd $(DESTDIR)$(localedir)/$$lang && LC_ALL=C ls -l -d $$lc | sed -e 's/^.* -> //'`; \
	        mv $(DESTDIR)$(localedir)/$$lang/$$lc $(DESTDIR)$(localedir)/$$lang/$$lc.old; \
	        mkdir $(DESTDIR)$(localedir)/$$lang/$$lc; \
	        (cd $(DESTDIR)$(localedir)/$$lang/$$lc.old && \
	         for file in *; do \
	           if test -f $$file; then \
	             ln -s ../$$link/$$file $(DESTDIR)$(localedir)/$$lang/$$lc/$$file; \
	           fi; \
	         done); \
	        rm -f $(DESTDIR)$(localedir)/$$lang/$$lc.old; \
	      else \
	        if test -d $(DESTDIR)$(localedir)/$$lang/$$lc; then \
	          :; \
	        else \
	          rm -f $(DESTDIR)$(localedir)/$$lang/$$lc; \
	          mkdir $(DESTDIR)$(localedir)/$$lang/$$lc; \
	        fi; \
	      fi; \
	    fi; \
	  done; \
	done

# Define this as empty until I found a useful application.
installcheck:

uninstall: uninstall-exec uninstall-data
uninstall-exec:
uninstall-data: uninstall-data-yes
	if test "$(PACKAGE)" = "gettext-tools"; then \
	  for file in $(DISTFILES.common) Makevars.template; do \
	    rm -f $(DESTDIR)$(gettextsrcdir)/$$file; \
	  done; \
	else \
	  : ; \
	fi
uninstall-data-no:
uninstall-data-yes:
	catalogs='$(CATALOGS)'; \
	for cat in $$catalogs; do \
	  cat=`basename $$cat`; \
	  lang=`echo $$cat | sed -e 's/\.gmo$$//'`; \
	  for lc in LC_MESSAGES $(EXTRA_LOCALE_CATEGORIES); do \
	    rm -f $(DESTDIR)$(localedir)/$$lang/$$lc/$(DOMAIN).mo; \
	  done; \
	done

check: all

info dvi ps pdf html tags TAGS ctags CTAGS ID:

mostlyclean:
	rm -f remove-potcdate.sed
	rm -f stamp-poT
	rm -f core core.* $(DOMAIN).po $(DOMAIN).1po $(DOMAIN).2po *.new.po
	rm -fr *.o

clean: mostlyclean

distclean: clean
	rm -f Makefile Makefile.in POTFILES *.mo

maintainer-clean: distclean
	@echo "This command is intended for maintainers to use;"
	@echo "it deletes files that may require special tools to rebuild."
	rm -f stamp-po $(GMOFILES)

distdir = $(top_builddir)/$(PACKAGE)-$(VERSION)/$(subdir)
dist distdir:
	test -z "$(DISTFILESDEPS)" || $(MAKE) $(DISTFILESDEPS)
	@$(MAKE) dist2
# This is a separate target because 'update-po' must be executed before.
dist2: stamp-po $(DISTFILES)
	dists="$(DISTFILES)"; \
	if test "$(PACKAGE)" = "gettext-tools"; then \
	  dists="$$dists Makevars.template"; \
	fi; \
	if test -f $(srcdir)/$(DOMAIN).pot; then \
	  dists="$$dists $(DOMAIN).pot stamp-po"; \
	fi; \
	if test -f $(srcdir)/ChangeLog; then \
	  dists="$$dists ChangeLog"; \
	fi; \
	for i in 0 1 2 3 4 5 6 7 8 9; do \
	  if test -f $(srcdir)/ChangeLog.$$i; then \
	    dists="$$dists ChangeLog.$$i"; \
	  fi; \
	done; \
	if test -f $(srcdir)/LINGUAS; then dists="$$dists LINGUAS"; fi; \
	for file in $$dists; do \
	  if test -f $$file; then \
	    cp -p $$file $(distdir) || exit 1; \
	  else \
	    cp -p $(srcdir)/$$file $(distdir) || exit 1; \
	  fi; \
	done

update-po: Makefile
	$(MAKE) $(DOMAIN).pot-update
	test -z "$(UPDATEPOFILES)" || $(MAKE) $(UPDATEPOFILES)
	$(MAKE) update-gmo

# General rule for creating PO files.

.nop.po-create:
	@lang=`echo $@ | sed -e 's/\.po-create$$//'`; \
	echo "File $$lang.po does not exist. If you are a translator, you can create it through 'msginit'." 1>&2; \
	exit 1

# General rule for updating PO files.

.nop.po-update:
	@lang=`echo $@ | sed -e 's/\.po-update$$//'`; \
	if test "$(PACKAGE)" = "gettext-tools"; then PATH=`pwd`/../src:$$PATH; fi; \
	tmpdir=`pwd`; \
	echo "$$lang:"; \
	test "$(srcdir)" = . && cdcmd="" || cdcmd="cd $(srcdir) && "; \
	echo "$${cdcmd}$(MSGMERGE) $(MSGMERGE_OPTIONS) --lang=$$lang $$lang.po $(DOMAIN).pot -o $$lang.new.po"; \
	cd $(srcdir); \
	if { case `$(MSGMERGE) --version | sed 1q | sed -e 's,^[^0-9]*,,'` in \
	       '' | 0.[0-9] | 0.[0-9].* | 0.1[0-7] | 0.1[0-7].*) \
	         $(MSGMERGE) $(MSGMERGE_OPTIONS) -o $$tmpdir/$$lang.new.po $$lang.po $(DOMAIN).pot;; \
	       *) \
	         $(MSGMERGE) $(MSGMERGE_OPTIONS) --lang=$$lang -o $$tmpdir/$$lang.new.po $$lang.po $(DOMAIN).pot;; \
	     esac; \
	   }; then \
	  if cmp $$lang.po $$tmpdir/$$lang.new.po >/dev/null 2>&1; then \
	    rm -f $$tmpdir/$$lang.new.po; \
	  else \
	    if mv -f $$tmpdir/$$lang.new.po $$lang.po; then \
	      :; \
	    else \
	      echo "msgmerge for $$lang.po failed: cannot move $$tmpdir/$$lang.new.po to $$lang.po" 1>&2; \
	      exit 1; \
	    fi; \
	  fi; \
	else \
	  echo "msgmerge for $$lang.po failed!" 1>&2; \
	  rm -f $$tmpdir/$$lang.new.po; \
	fi

$(DUMMYPOFILES):

update-gmo: Makefile $(GMOFILES)
	@:

# Recreate Makefile by invoking config.status. Explicitly invoke the shell,
# because execution permission bits may not work on the current file system.
# Use /bin/bash, which is the shell determined by autoconf for the use by its
# scripts, not $(SHELL) which is hardwired to /bin/sh and may be deficient.
Makefile: Makefile.in.in Makevars $(top_builddir)/config.status POTFILES.in LINGUAS
	cd $(top_builddir) \
	  && /bin/bash ./config.status $(subdir)/$@.in po-directories

force:

# Tell versions [3.59,3.63) of GNU make not to export all variables.
# Otherwise a system limit (for SysV at least) may be exceeded.
.NOEXPORT:
# This file, Rules-quot, can be copied and used freely without restrictions.
# Special Makefile rules for English message catalogs with quotation marks.

DISTFILES.common.extra1 = quot.sed boldquot.sed en@quot.header en@boldquot.header insert-header.sin Rules-quot

.SUFFIXES: .insert-header .po-update-en

en@quot.po-create:
	$(MAKE) en@quot.po-update
en@boldquot.po-create:
	$(MAKE) en@boldquot.po-update

en@quot.po-update: en@quot.po-update-en
en@boldquot.po-update: en@boldquot.po-update-en

.insert-header.po-update-en:
	@lang=`echo $@ | sed -e 's/\.po-update-en$$//'`; \
	if test "$(PACKAGE)" = "gettext-tools"; then PATH=`pwd`/../src:$$PATH; GETTEXTLIBDIR=`cd $(top_srcdir)/src && pwd`; export GETTEXTLIBDIR; fi; \
	tmpdir=`pwd`; \
	echo "$$lang:"; \
	ll=`echo $$lang | sed -e 's/@.*//'`; \
	LC_ALL=C; export LC_ALL; \
	cd $(srcdir); \
	if $(MSGINIT) -i $(DOMAIN).pot --no-translator -l $$lang -o - 2>/dev/null \
	   | $(SED) -f $$tmpdir/$$lang.insert-header | $(MSGCONV) -t UTF-8 | \
	   { case `$(MSGFILTER) --version | sed 1q | sed -e 's,^[^0-9]*,,'` in \
	     '' | 0.[0-9] | 0.[0-9].* | 0.1[0-8] | 0.1[0-8].*) \
	       $(MSGFILTER) $(SED) -f `echo $$lang | sed -e 's/.*@//'`.sed \
	       ;; \
	     *) \
	       $(MSGFILTER) `echo $$lang | sed -e 's/.*@//'` \
	       ;; \
	     esac } 2>/dev/null > $$tmpdir/$$lang.new.po \
	     ; then \
	  if cmp $$lang.po $$tmpdir/$$lang.new.po >/dev/null 2>&1; then \
	    rm -f $$tmpdir/$$lang.new.po; \
	  else \
	    if mv -f $$tmpdir/$$lang.new.po $$lang.po; then \
	      :; \
	    else \
	      echo "creation of $$lang.po failed: cannot move $$tmpdir/$$lang.new.po to $$lang.po" 1>&2; \
	      exit 1; \
	    fi; \
	  fi; \
	else \
	  echo "creation of $$lang.po failed!" 1>&2; \
	  rm -f $$tmpdir/$$lang.new.po; \
	fi

en@quot.insert-header: insert-header.sin
	sed -e '/^#/d' -e 's/HEADER/en@quot.header/g' $(srcdir)/insert-header.sin > en@quot.insert-header

en@boldquot.insert-header: insert-header.sin
	sed -e '/^#/d' -e 's/HEADER/en@boldquot.header/g' $(srcdir)/insert-header.sin > en@boldquot.insert-header

mostlyclean: mostlyclean-quot
mostlyclean-quot:
	rm -f *.insert-header
