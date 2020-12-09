POFILES = ja.po ru.po

DOMAIN = alsaconf
GETTEXT_SRC = ../alsaconf.in

XGETTEXT_OPTIONS = -kxmsg -kxecho

top_builddir = ../..

srcdir = ../../../alsa-utils-1.2.1/alsaconf/po
datadir = /usr/share
exec_prefix = /usr
prefix = /usr
localedir = $(datadir)/locale

USE_NLS = yes
INSTALL = /home/hhpartners/yocto/build/tmp/hosttools/install -c
INSTALL_DATA = ${INSTALL} -m 644
# mkdir_p = $(MKDIR_P)
mkdir_p = mkdir -p --
XGETTEXT = /home/hhpartners/yocto/build/tmp/work/core2-64-poky-linux/alsa-utils/1.2.1-r0/recipe-sysroot-native/usr/bin/xgettext
MSGFMT = /home/hhpartners/yocto/build/tmp/work/core2-64-poky-linux/alsa-utils/1.2.1-r0/recipe-sysroot-native/usr/bin/msgfmt
GMSGFMT = /home/hhpartners/yocto/build/tmp/work/core2-64-poky-linux/alsa-utils/1.2.1-r0/recipe-sysroot-native/usr/bin/msgfmt
MSGMERGE = /home/hhpartners/yocto/build/tmp/work/core2-64-poky-linux/alsa-utils/1.2.1-r0/recipe-sysroot-native/usr/bin/msgmerge
MSGMERGE_UPDATE = /home/hhpartners/yocto/build/tmp/work/core2-64-poky-linux/alsa-utils/1.2.1-r0/recipe-sysroot-native/usr/bin/msgmerge --update

DISTFILES = $(POFILES) Makefile.in
CATALOGS = $(POFILES:.po=.gmo)

all: all-yes

all-yes: $(CATALOGS)
all-no:

.SUFFIXES: .po .gmo .mo

%.mo : $(srcdir)/%.po
	$(MSGFMT) -c -o t-$@ $< && mv t-$@ $@

%.gmo : $(srcdir)/%.po
	@lang=`echo $(srcdir)/$* | sed -e 's,.*/,,'`; \
	rm -f $${lang}.gmo && $(GMSGFMT) -c --statistics -o t-$${lang}.gmo $(srcdir)/$${lang}.po && mv t-$${lang}.gmo $${lang}.gmo

$(DOMAIN).pot: $(GETTEXT_SRC)
	$(XGETTEXT) -o$@ --default-domain=$(DOMAIN) \
	  --add-comments=TRANSLATORS: --language=Shell \
	  $(XGETTEXT_OPTIONS) $<

check check_recursive:

clean:
	rm -f $(CATALOGS) $(POFILES:.po=.mo) $(DOMAIN).pot

update-po: $(DOMAIN).pot
	@for po in $(POFILES); do \
	$(MSGMERGE_UPDATE) $(srcdir)/$$po $(DOMAIN).pot; \
	done

install-data-no:
install-data-yes: all
	$(mkdir_p) $(DESTDIR)$(datadir)
	@catalogs='$(CATALOGS)'; \
	for cat in $$catalogs; do \
	  lang=`echo $$cat | sed -e 's/\.gmo$$//'`; \
	  dir=$(localedir)/$$lang/LC_MESSAGES; \
	  $(mkdir_p) $(DESTDIR)$$dir; \
	  $(INSTALL_DATA) $$cat $(DESTDIR)$$dir/$(DOMAIN).mo; \
	done

install-data: install-data-yes
install: install-data

uninstall-data-no:
uninstall-data-yes:
	@catalogs='$(CATALOGS)'; \
	for cat in $$catalogs; do \
	  cat=`basename $$cat`; \
	  lang=`echo $$cat | sed -e 's/\.gmo$$//'`; \
	  rm -f $(DESTDIR)$(localedir)/$$lang/LC_MESSAGES/$(DOMAIN).mo; \
	done

uninstall-data: uninstall-data-yes
uninstall: uninstall-data

dist distdir: $(DISTFILES)
	@for file in $(DISTFILES); do \
	  cp -p $$file $(distdir); \
	done

distclean: clean
	rm -f Makefile *~
