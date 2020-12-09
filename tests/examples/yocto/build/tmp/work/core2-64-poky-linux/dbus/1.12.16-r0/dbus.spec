Summary: D-Bus message bus
Name: dbus
Version: 1.12.16
Release: r0
License: AFL-2.1 | GPLv2+
Group: base
Packager: Poky <poky@lists.yoctoproject.org>
URL: https://dbus.freedesktop.org
BuildRequires: autoconf-archive
BuildRequires: autoconf-native
BuildRequires: automake-native
BuildRequires: base-files
BuildRequires: base-passwd
BuildRequires: expat
BuildRequires: gettext-native
BuildRequires: gnu-config-native
BuildRequires: initscripts
BuildRequires: libsm
BuildRequires: libtool-cross
BuildRequires: libtool-native
BuildRequires: pkgconfig-native
BuildRequires: shadow
BuildRequires: shadow-native
BuildRequires: shadow-sysroot
BuildRequires: update-rc.d
BuildRequires: virtual/libc
BuildRequires: virtual/libintl
BuildRequires: virtual/libx11
BuildRequires: virtual/x86_64-poky-linux-compilerlibs
BuildRequires: virtual/x86_64-poky-linux-gcc

%description
D-Bus is a message bus system, a simple way for applications to talk to one
another. In addition to interprocess communication, D-Bus helps coordinate
process lifecycle; it makes it simple and reliable to code a \"single
instance\" application or daemon, and to launch applications and daemons on
demand when their services are needed.

%package -n dbus-src
Summary: D-Bus message bus - Source files
License: AFL-2.1 | GPLv2+
Group: devel

%description -n dbus-src
D-Bus is a message bus system, a simple way for applications to talk to one
another. In addition to interprocess communication, D-Bus helps coordinate
process lifecycle; it makes it simple and reliable to code a \"single
instance\" application or daemon, and to launch applications and daemons on
demand when their services are needed.  This package contains sources for
debugging purposes.

%package -n dbus-dbg
Summary: D-Bus message bus - Debugging files
License: AFL-2.1 | GPLv2+
Group: devel
Recommends: dbus-lib-dbg
Recommends: expat-dbg
Recommends: glibc-dbg
Recommends: libx11-dbg

%description -n dbus-dbg
D-Bus is a message bus system, a simple way for applications to talk to one
another. In addition to interprocess communication, D-Bus helps coordinate
process lifecycle; it makes it simple and reliable to code a \"single
instance\" application or daemon, and to launch applications and daemons on
demand when their services are needed.  This package contains ELF symbols
and related sources for debugging purposes.

%package -n libdbus-1-3
Summary: D-Bus message bus
License: AFL-2.1 | GPLv2+
Group: base
Requires: libc.so.6()(64bit)
Requires: libc.so.6(GLIBC_2.10)(64bit)
Requires: libc.so.6(GLIBC_2.14)(64bit)
Requires: libc.so.6(GLIBC_2.17)(64bit)
Requires: libc.so.6(GLIBC_2.2.5)(64bit)
Requires: libc.so.6(GLIBC_2.3.4)(64bit)
Requires: libc.so.6(GLIBC_2.4)(64bit)
Requires: libc6 >= 2.31+git0+6fdf971c9d
Requires: libpthread.so.0()(64bit)
Requires: libpthread.so.0(GLIBC_2.2.5)(64bit)
Requires: libpthread.so.0(GLIBC_2.3.2)(64bit)
Requires: libpthread.so.0(GLIBC_2.3.3)(64bit)
Requires: rtld(GNU_HASH)
Requires(post): libc.so.6()(64bit)
Requires(post): libc.so.6(GLIBC_2.10)(64bit)
Requires(post): libc.so.6(GLIBC_2.14)(64bit)
Requires(post): libc.so.6(GLIBC_2.17)(64bit)
Requires(post): libc.so.6(GLIBC_2.2.5)(64bit)
Requires(post): libc.so.6(GLIBC_2.3.4)(64bit)
Requires(post): libc.so.6(GLIBC_2.4)(64bit)
Requires(post): libc6 >= 2.31+git0+6fdf971c9d
Requires(post): libpthread.so.0()(64bit)
Requires(post): libpthread.so.0(GLIBC_2.2.5)(64bit)
Requires(post): libpthread.so.0(GLIBC_2.3.2)(64bit)
Requires(post): libpthread.so.0(GLIBC_2.3.3)(64bit)
Requires(post): rtld(GNU_HASH)
Recommends: dbus
Provides: dbus-lib = 1.12.16
Provides: libdbus-1.so.3()(64bit)
Provides: libdbus-1.so.3(LIBDBUS_1_3)(64bit)
Provides: libdbus-1.so.3(LIBDBUS_PRIVATE_1.12.16)(64bit)

%description -n libdbus-1-3
D-Bus is a message bus system, a simple way for applications to talk to one
another. In addition to interprocess communication, D-Bus helps coordinate
process lifecycle; it makes it simple and reliable to code a \"single
instance\" application or daemon, and to launch applications and daemons on
demand when their services are needed.

%package -n dbus-staticdev
Summary: D-Bus message bus - Development files (Static Libraries)
License: AFL-2.1 | GPLv2+
Group: devel
Requires: dbus-dev = 1.12.16-r0

%description -n dbus-staticdev
D-Bus is a message bus system, a simple way for applications to talk to one
another. In addition to interprocess communication, D-Bus helps coordinate
process lifecycle; it makes it simple and reliable to code a \"single
instance\" application or daemon, and to launch applications and daemons on
demand when their services are needed.  This package contains static
libraries for software development.

%package -n dbus-dev
Summary: D-Bus message bus - Development files
License: AFL-2.1 | GPLv2+
Group: devel
Requires: dbus-1 = 1.12.16-r0
Requires: libc.so.6()(64bit)
Requires: libc.so.6(GLIBC_2.2.5)(64bit)
Requires: libc.so.6(GLIBC_2.3.4)(64bit)
Requires: libc.so.6(GLIBC_2.4)(64bit)
Requires: libc.so.6(GLIBC_2.7)(64bit)
Requires: libc6 >= 2.31+git0+6fdf971c9d
Requires: libdbus-1-3 >= 1.12.16
Requires: libdbus-1.so.3()(64bit)
Requires: libdbus-1.so.3(LIBDBUS_1_3)(64bit)
Requires: libdbus-1.so.3(LIBDBUS_PRIVATE_1.12.16)(64bit)
Requires: libpthread.so.0()(64bit)
Requires: libpthread.so.0(GLIBC_2.2.5)(64bit)
Requires: rtld(GNU_HASH)
Recommends: autoconf-archive-dev
Recommends: base-files-dev
Recommends: base-passwd-dev
Recommends: dbus-lib-dev
Recommends: dbus-test-ptest-dev
Recommends: expat-dev
Recommends: glibc-dev
Recommends: initd-functions-dev
Recommends: initscripts-dev
Recommends: libsm-dev
Recommends: libx11-dev
Recommends: shadow-dev
Recommends: shadow-sysroot-dev
Recommends: update-rc.d-dev

%description -n dbus-dev
D-Bus is a message bus system, a simple way for applications to talk to one
another. In addition to interprocess communication, D-Bus helps coordinate
process lifecycle; it makes it simple and reliable to code a \"single
instance\" application or daemon, and to launch applications and daemons on
demand when their services are needed.  This package contains symbolic
links, header files, and related items necessary for software development.

%package -n dbus-doc
Summary: D-Bus message bus - Documentation files
License: AFL-2.1 | GPLv2+
Group: doc

%description -n dbus-doc
D-Bus is a message bus system, a simple way for applications to talk to one
another. In addition to interprocess communication, D-Bus helps coordinate
process lifecycle; it makes it simple and reliable to code a \"single
instance\" application or daemon, and to launch applications and daemons on
demand when their services are needed.  This package contains
documentation.

%package -n dbus-locale
Summary: D-Bus message bus
License: AFL-2.1 | GPLv2+
Group: base

%description -n dbus-locale
D-Bus is a message bus system, a simple way for applications to talk to one
another. In addition to interprocess communication, D-Bus helps coordinate
process lifecycle; it makes it simple and reliable to code a \"single
instance\" application or daemon, and to launch applications and daemons on
demand when their services are needed.

%package -n dbus-1
Summary: D-Bus message bus
License: AFL-2.1 | GPLv2+
Group: base
Requires: /bin/sh
Requires: base-files
Requires: base-passwd
Requires: initd-functions
Requires: libX11.so.6()(64bit)
Requires: libc.so.6()(64bit)
Requires: libc.so.6(GLIBC_2.14)(64bit)
Requires: libc.so.6(GLIBC_2.15)(64bit)
Requires: libc.so.6(GLIBC_2.2.5)(64bit)
Requires: libc.so.6(GLIBC_2.3.2)(64bit)
Requires: libc.so.6(GLIBC_2.3.4)(64bit)
Requires: libc.so.6(GLIBC_2.4)(64bit)
Requires: libc.so.6(GLIBC_2.8)(64bit)
Requires: libc.so.6(GLIBC_2.9)(64bit)
Requires: libc6 >= 2.31+git0+6fdf971c9d
Requires: libdbus-1-3 >= 1.12.16
Requires: libdbus-1.so.3()(64bit)
Requires: libdbus-1.so.3(LIBDBUS_1_3)(64bit)
Requires: libdbus-1.so.3(LIBDBUS_PRIVATE_1.12.16)(64bit)
Requires: libexpat.so.1()(64bit)
Requires: libexpat1 >= 2.2.9
Requires: libpthread.so.0()(64bit)
Requires: libpthread.so.0(GLIBC_2.2.5)(64bit)
Requires: libx11-6 >= 1.6.9
Requires: rtld(GNU_HASH)
Requires: shadow
Requires(pre): /bin/sh
Requires(pre): base-files
Requires(pre): base-passwd
Requires(pre): initd-functions
Requires(pre): libX11.so.6()(64bit)
Requires(pre): libc.so.6()(64bit)
Requires(pre): libc.so.6(GLIBC_2.14)(64bit)
Requires(pre): libc.so.6(GLIBC_2.15)(64bit)
Requires(pre): libc.so.6(GLIBC_2.2.5)(64bit)
Requires(pre): libc.so.6(GLIBC_2.3.2)(64bit)
Requires(pre): libc.so.6(GLIBC_2.3.4)(64bit)
Requires(pre): libc.so.6(GLIBC_2.4)(64bit)
Requires(pre): libc.so.6(GLIBC_2.8)(64bit)
Requires(pre): libc.so.6(GLIBC_2.9)(64bit)
Requires(pre): libc6 >= 2.31+git0+6fdf971c9d
Requires(pre): libdbus-1-3 >= 1.12.16
Requires(pre): libdbus-1.so.3()(64bit)
Requires(pre): libdbus-1.so.3(LIBDBUS_1_3)(64bit)
Requires(pre): libdbus-1.so.3(LIBDBUS_PRIVATE_1.12.16)(64bit)
Requires(pre): libexpat.so.1()(64bit)
Requires(pre): libexpat1 >= 2.2.9
Requires(pre): libpthread.so.0()(64bit)
Requires(pre): libpthread.so.0(GLIBC_2.2.5)(64bit)
Requires(pre): libx11-6 >= 1.6.9
Requires(pre): rtld(GNU_HASH)
Requires(pre): shadow
Requires(post): /bin/sh
Requires(post): base-files
Requires(post): base-passwd
Requires(post): initd-functions
Requires(post): libX11.so.6()(64bit)
Requires(post): libc.so.6()(64bit)
Requires(post): libc.so.6(GLIBC_2.14)(64bit)
Requires(post): libc.so.6(GLIBC_2.15)(64bit)
Requires(post): libc.so.6(GLIBC_2.2.5)(64bit)
Requires(post): libc.so.6(GLIBC_2.3.2)(64bit)
Requires(post): libc.so.6(GLIBC_2.3.4)(64bit)
Requires(post): libc.so.6(GLIBC_2.4)(64bit)
Requires(post): libc.so.6(GLIBC_2.8)(64bit)
Requires(post): libc.so.6(GLIBC_2.9)(64bit)
Requires(post): libc6 >= 2.31+git0+6fdf971c9d
Requires(post): libdbus-1-3 >= 1.12.16
Requires(post): libdbus-1.so.3()(64bit)
Requires(post): libdbus-1.so.3(LIBDBUS_1_3)(64bit)
Requires(post): libdbus-1.so.3(LIBDBUS_PRIVATE_1.12.16)(64bit)
Requires(post): libexpat.so.1()(64bit)
Requires(post): libexpat1 >= 2.2.9
Requires(post): libpthread.so.0()(64bit)
Requires(post): libpthread.so.0(GLIBC_2.2.5)(64bit)
Requires(post): libx11-6 >= 1.6.9
Requires(post): rtld(GNU_HASH)
Requires(post): shadow
Requires(preun): /bin/sh
Requires(preun): base-files
Requires(preun): base-passwd
Requires(preun): initd-functions
Requires(preun): libX11.so.6()(64bit)
Requires(preun): libc.so.6()(64bit)
Requires(preun): libc.so.6(GLIBC_2.14)(64bit)
Requires(preun): libc.so.6(GLIBC_2.15)(64bit)
Requires(preun): libc.so.6(GLIBC_2.2.5)(64bit)
Requires(preun): libc.so.6(GLIBC_2.3.2)(64bit)
Requires(preun): libc.so.6(GLIBC_2.3.4)(64bit)
Requires(preun): libc.so.6(GLIBC_2.4)(64bit)
Requires(preun): libc.so.6(GLIBC_2.8)(64bit)
Requires(preun): libc.so.6(GLIBC_2.9)(64bit)
Requires(preun): libc6 >= 2.31+git0+6fdf971c9d
Requires(preun): libdbus-1-3 >= 1.12.16
Requires(preun): libdbus-1.so.3()(64bit)
Requires(preun): libdbus-1.so.3(LIBDBUS_1_3)(64bit)
Requires(preun): libdbus-1.so.3(LIBDBUS_PRIVATE_1.12.16)(64bit)
Requires(preun): libexpat.so.1()(64bit)
Requires(preun): libexpat1 >= 2.2.9
Requires(preun): libpthread.so.0()(64bit)
Requires(preun): libpthread.so.0(GLIBC_2.2.5)(64bit)
Requires(preun): libx11-6 >= 1.6.9
Requires(preun): rtld(GNU_HASH)
Requires(preun): shadow
Requires(postun): /bin/sh
Requires(postun): base-files
Requires(postun): base-passwd
Requires(postun): initd-functions
Requires(postun): libX11.so.6()(64bit)
Requires(postun): libc.so.6()(64bit)
Requires(postun): libc.so.6(GLIBC_2.14)(64bit)
Requires(postun): libc.so.6(GLIBC_2.15)(64bit)
Requires(postun): libc.so.6(GLIBC_2.2.5)(64bit)
Requires(postun): libc.so.6(GLIBC_2.3.2)(64bit)
Requires(postun): libc.so.6(GLIBC_2.3.4)(64bit)
Requires(postun): libc.so.6(GLIBC_2.4)(64bit)
Requires(postun): libc.so.6(GLIBC_2.8)(64bit)
Requires(postun): libc.so.6(GLIBC_2.9)(64bit)
Requires(postun): libc6 >= 2.31+git0+6fdf971c9d
Requires(postun): libdbus-1-3 >= 1.12.16
Requires(postun): libdbus-1.so.3()(64bit)
Requires(postun): libdbus-1.so.3(LIBDBUS_1_3)(64bit)
Requires(postun): libdbus-1.so.3(LIBDBUS_PRIVATE_1.12.16)(64bit)
Requires(postun): libexpat.so.1()(64bit)
Requires(postun): libexpat1 >= 2.2.9
Requires(postun): libpthread.so.0()(64bit)
Requires(postun): libpthread.so.0(GLIBC_2.2.5)(64bit)
Requires(postun): libx11-6 >= 1.6.9
Requires(postun): rtld(GNU_HASH)
Requires(postun): shadow
Recommends: update-rc.d
Provides: dbus = 1.12.16
Provides: dbus-x11
Obsoletes: dbus-x11

%description -n dbus-1
D-Bus is a message bus system, a simple way for applications to talk to one
another. In addition to interprocess communication, D-Bus helps coordinate
process lifecycle; it makes it simple and reliable to code a \"single
instance\" application or daemon, and to launch applications and daemons on
demand when their services are needed.

%package -n dbus-ptest
Summary: D-Bus message bus
License: AFL-2.1 | GPLv2+
Group: base
Requires: dbus-test-ptest

%description -n dbus-ptest
D-Bus is a message bus system, a simple way for applications to talk to one
another. In addition to interprocess communication, D-Bus helps coordinate
process lifecycle; it makes it simple and reliable to code a \"single
instance\" application or daemon, and to launch applications and daemons on
demand when their services are needed.

%post -n libdbus-1-3
# libdbus-1-3 - postinst
#!/bin/sh
set -e
if [ x"$D" = "x" ]; then
	if [ -x /sbin/ldconfig ]; then /sbin/ldconfig ; fi
fi


%pre -n dbus-1
# dbus-1 - preinst
#!/bin/sh
set -e
bbnote () {
	echo "NOTE: $*"
}
bbwarn () {
	echo "WARNING: $*"
}
bbfatal () {
	echo "ERROR: $*"
	exit 1
}
perform_groupadd () {
	local rootdir="$1"
	local opts="$2"
	bbnote "dbus: Performing groupadd with [$opts]"
	local groupname=`echo "$opts" | awk '{ print $NF }'`
	local group_exists="`grep "^$groupname:" $rootdir/etc/group || true`"
	if test "x$group_exists" = "x"; then
		eval flock -x $rootdir/etc -c \"$PSEUDO groupadd \$opts\" || true
		group_exists="`grep "^$groupname:" $rootdir/etc/group || true`"
		if test "x$group_exists" = "x"; then
			bbfatal "dbus: groupadd command did not succeed."
		fi
	else
		bbnote "dbus: group $groupname already exists, not re-creating it"
	fi
}
perform_useradd () {
	local rootdir="$1"
	local opts="$2"
	bbnote "dbus: Performing useradd with [$opts]"
	local username=`echo "$opts" | awk '{ print $NF }'`
	local user_exists="`grep "^$username:" $rootdir/etc/passwd || true`"
	if test "x$user_exists" = "x"; then
		eval flock -x $rootdir/etc -c  \"$PSEUDO useradd \$opts\" || true
		user_exists="`grep "^$username:" $rootdir/etc/passwd || true`"
		if test "x$user_exists" = "x"; then
			bbfatal "dbus: useradd command did not succeed."
		fi
	else
		bbnote "dbus: user $username already exists, not re-creating it"
	fi
}
perform_groupmems () {
	local rootdir="$1"
	local opts="$2"
	bbnote "dbus: Performing groupmems with [$opts]"
	local groupname=`echo "$opts" | awk '{ for (i = 1; i < NF; i++) if ($i == "-g" || $i == "--group") print $(i+1) }'`
	local username=`echo "$opts" | awk '{ for (i = 1; i < NF; i++) if ($i == "-a" || $i == "--add") print $(i+1) }'`
	bbnote "dbus: Running groupmems command with group $groupname and user $username"
	local mem_exists="`grep "^$groupname:[^:]*:[^:]*:\([^,]*,\)*$username\(,[^,]*\)*$" $rootdir/etc/group || true`"
	if test "x$mem_exists" = "x"; then
		eval flock -x $rootdir/etc -c \"$PSEUDO groupmems \$opts\" || true
		mem_exists="`grep "^$groupname:[^:]*:[^:]*:\([^,]*,\)*$username\(,[^,]*\)*$" $rootdir/etc/group || true`"
		if test "x$mem_exists" = "x"; then
			bbfatal "dbus: groupmems command did not succeed."
		fi
	else
		bbnote "dbus: group $groupname already contains $username, not re-adding it"
	fi
}
OPT=""
SYSROOT=""

if test "x$D" != "x"; then
	# Installing into a sysroot
	SYSROOT="$D"
	OPT="--root $D"

	# Make sure login.defs is there, this is to make debian package backend work
	# correctly while doing rootfs.
	# The problem here is that if /etc/login.defs is treated as a config file for
	# shadow package, then while performing preinsts for packages that depend on
	# shadow, there might only be /etc/login.def.dpkg-new there in root filesystem.
	if [ ! -e $D/etc/login.defs -a -e $D/etc/login.defs.dpkg-new ]; then
	    cp $D/etc/login.defs.dpkg-new $D/etc/login.defs
	fi

	# user/group lookups should match useradd/groupadd --root
	export PSEUDO_PASSWD="$SYSROOT"
fi

# If we're not doing a special SSTATE/SYSROOT install
# then set the values, otherwise use the environment
if test "x$UA_SYSROOT" = "x"; then
	# Installing onto a target
	# Add groups and users defined only for this package
	GROUPADD_PARAM="${GROUPADD_PARAM}"
	USERADD_PARAM="--system --home /var/lib/dbus                        --no-create-home --shell /bin/false                        --user-group messagebus"
	GROUPMEMS_PARAM="${GROUPMEMS_PARAM}"
fi

# Perform group additions first, since user additions may depend
# on these groups existing
if test "x`echo $GROUPADD_PARAM | tr -d '[:space:]'`" != "x"; then
	echo "Running groupadd commands..."
	# Invoke multiple instances of groupadd for parameter lists
	# separated by ';'
	opts=`echo "$GROUPADD_PARAM" | cut -d ';' -f 1 | sed -e 's#[ \t]*$##'`
	remaining=`echo "$GROUPADD_PARAM" | cut -d ';' -f 2- | sed -e 's#[ \t]*$##'`
	while test "x$opts" != "x"; do
		perform_groupadd "$SYSROOT" "$OPT $opts"
		if test "x$opts" = "x$remaining"; then
			break
		fi
		opts=`echo "$remaining" | cut -d ';' -f 1 | sed -e 's#[ \t]*$##'`
		remaining=`echo "$remaining" | cut -d ';' -f 2- | sed -e 's#[ \t]*$##'`
	done
fi

if test "x`echo $USERADD_PARAM | tr -d '[:space:]'`" != "x"; then
	echo "Running useradd commands..."
	# Invoke multiple instances of useradd for parameter lists
	# separated by ';'
	opts=`echo "$USERADD_PARAM" | cut -d ';' -f 1 | sed -e 's#[ \t]*$##'`
	remaining=`echo "$USERADD_PARAM" | cut -d ';' -f 2- | sed -e 's#[ \t]*$##'`
	while test "x$opts" != "x"; do
		perform_useradd "$SYSROOT" "$OPT $opts"
		if test "x$opts" = "x$remaining"; then
			break
		fi
		opts=`echo "$remaining" | cut -d ';' -f 1 | sed -e 's#[ \t]*$##'`
		remaining=`echo "$remaining" | cut -d ';' -f 2- | sed -e 's#[ \t]*$##'`
	done
fi

if test "x`echo $GROUPMEMS_PARAM | tr -d '[:space:]'`" != "x"; then
	echo "Running groupmems commands..."
	# Invoke multiple instances of groupmems for parameter lists
	# separated by ';'
	opts=`echo "$GROUPMEMS_PARAM" | cut -d ';' -f 1 | sed -e 's#[ \t]*$##'`
	remaining=`echo "$GROUPMEMS_PARAM" | cut -d ';' -f 2- | sed -e 's#[ \t]*$##'`
	while test "x$opts" != "x"; do
		perform_groupmems "$SYSROOT" "$OPT $opts"
		if test "x$opts" = "x$remaining"; then
			break
		fi
		opts=`echo "$remaining" | cut -d ';' -f 1 | sed -e 's#[ \t]*$##'`
		remaining=`echo "$remaining" | cut -d ';' -f 2- | sed -e 's#[ \t]*$##'`
	done
fi


%post -n dbus-1
# dbus-1 - postinst
set -e
	# If both systemd and sysvinit are enabled, mask the dbus-1 init script
        if false; then
		if [ -n "$D" ]; then
			OPTS="--root=$D"
		fi
		systemctl $OPTS mask dbus-1.service
	fi

	if [ -z "$D" ] && [ -e /etc/init.d/populate-volatile.sh ] ; then
		/etc/init.d/populate-volatile.sh update
	fi
if true && type update-rc.d >/dev/null 2>/dev/null; then
	if [ -n "$D" ]; then
		OPT="-r $D"
	else
		OPT="-s"
	fi
	update-rc.d $OPT dbus-1 start 02 5 3 2 . stop 20 0 1 6 .
fi


%preun -n dbus-1
# dbus-1 - prerm
#!/bin/sh
if [ "$1" = "0" ] ; then
set -e
if true && [ -z "$D" -a -x "/etc/init.d/dbus-1" ]; then
	/etc/init.d/dbus-1 stop || :
fi
fi

%postun -n dbus-1
# dbus-1 - postrm
#!/bin/sh
if [ "$1" = "0" ] ; then
set -e
if true && type update-rc.d >/dev/null 2>/dev/null; then
	if [ -n "$D" ]; then
		OPT="-f -r $D"
	else
		OPT="-f"
	fi
	update-rc.d $OPT dbus-1 remove
fi
fi

%files -n dbus-src
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/src"
%dir "/usr/src/debug"
%dir "/usr/src/debug/dbus"
%dir "/usr/src/debug/dbus/1.12.16-r0"
%dir "/usr/src/debug/dbus/1.12.16-r0/build"
%dir "/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16"
%dir "/usr/src/debug/dbus/1.12.16-r0/build/dbus"
"/usr/src/debug/dbus/1.12.16-r0/build/dbus/dbus-arch-deps.h"
%dir "/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/tools"
%dir "/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus"
%dir "/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/tools/tool-common.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/tools/dbus-monitor.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/tools/dbus-print-message.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/tools/dbus-update-activation-environment.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/tools/dbus-print-message.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/tools/dbus-launch-x11.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/tools/test-tool.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/tools/dbus-launch.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/tools/dbus-launch.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/tools/dbus-echo.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/tools/dbus-run-session.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/tools/dbus-uuidgen.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/tools/tool-common.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/tools/dbus-cleanup-sockets.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/tools/dbus-spam.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/tools/dbus-send.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/expirelist.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/activation.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/dir-watch-inotify.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/dispatch.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/utils.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/audit.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/policy.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/config-parser-common.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/apparmor.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/bus.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/desktop-file.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/config-parser-trivial.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/connection.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/audit.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/main.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/selinux.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/driver.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/config-parser-trivial.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/desktop-file.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/bus.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/apparmor.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/config-parser.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/signals.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/services.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/services.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/signals.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/connection.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/activation-helper.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/expirelist.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/config-parser.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/selinux.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/stats.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/activation.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/dir-watch.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/utils.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/dispatch.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/policy.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/driver.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/config-parser-common.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/activation-helper.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/activation-helper-bin.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/bus/config-loader-expat.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-credentials.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-socket-set.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-timeout.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-uuidgen.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-pending-call.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-sha.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-nonce.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-transport-unix.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-userdb.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-threads.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-dataslot.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-shell.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-marshal-header.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-marshal-byteswap.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-string-util.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-memory.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-string.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-connection.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-transport-socket.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-types.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-transport.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-auth.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-userdb.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-bus.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-spawn.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-message-internal.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-resources.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-misc.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-marshal-basic.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-object-tree.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-keyring.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-list.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-misc.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-transport-socket.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-file.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-list.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-marshal-byteswap.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-server.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-bus.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-socket-set-epoll.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-string.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-signature.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-pending-call.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-asv-util.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-keyring.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-auth.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-asv-util.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-mempool.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-internals.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-shell.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-dataslot.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-sysdeps-util-unix.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-credentials.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-connection.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-pipe-unix.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-internals.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-syntax.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-message-util.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-string-private.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-message.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-socket-set.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-resources.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-errors.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-sysdeps-util.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-transport.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-threads-internal.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-object-tree.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-sysdeps-pthread.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-transport-protected.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-watch.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-userdb-util.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-hash.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-timeout.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-message-private.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-server-unix.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-pipe.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-sysdeps.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-mainloop.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-address.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-server-protected.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-socket-set-poll.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-pipe.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-file-unix.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-connection-internal.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-signature.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-pending-call-internal.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-shared.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-syntax.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-uuidgen.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-server-socket.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-marshal-validate.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-marshal-recursive.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-marshal-validate.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-sysdeps-unix.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-mempool.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-address.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-marshal-header.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-spawn.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-memory.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-mainloop.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-marshal-basic.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-nonce.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-marshal-recursive.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-hash.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-errors.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-sysdeps-unix.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-threads.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-server-socket.c"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-server.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-message.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-watch.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-sha.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-valgrind-internal.h"
"/usr/src/debug/dbus/1.12.16-r0/dbus-1.12.16/dbus/dbus-sysdeps.c"

%files -n dbus-dbg
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/libexec"
%dir "/usr/bin"
%dir "/usr/lib"
%dir "/usr/libexec/.debug"
"/usr/libexec/.debug/dbus-daemon-launch-helper"
%dir "/usr/bin/.debug"
"/usr/bin/.debug/dbus-run-session"
"/usr/bin/.debug/dbus-update-activation-environment"
"/usr/bin/.debug/dbus-monitor"
"/usr/bin/.debug/dbus-send"
"/usr/bin/.debug/dbus-test-tool"
"/usr/bin/.debug/dbus-daemon"
"/usr/bin/.debug/dbus-cleanup-sockets"
"/usr/bin/.debug/dbus-launch"
"/usr/bin/.debug/dbus-uuidgen"
%dir "/usr/lib/.debug"
"/usr/lib/.debug/libdbus-1.so.3.19.11"

%files -n libdbus-1-3
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/lib"
"/usr/lib/libdbus-1.so.3.19.11"
"/usr/lib/libdbus-1.so.3"

%files -n dbus-dev
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/include"
%dir "/usr/bin"
%dir "/usr/lib"
%dir "/usr/include/dbus-1.0"
%dir "/usr/include/dbus-1.0/dbus"
"/usr/include/dbus-1.0/dbus/dbus-pending-call.h"
"/usr/include/dbus-1.0/dbus/dbus-threads.h"
"/usr/include/dbus-1.0/dbus/dbus-connection.h"
"/usr/include/dbus-1.0/dbus/dbus-types.h"
"/usr/include/dbus-1.0/dbus/dbus-bus.h"
"/usr/include/dbus-1.0/dbus/dbus-misc.h"
"/usr/include/dbus-1.0/dbus/dbus.h"
"/usr/include/dbus-1.0/dbus/dbus-protocol.h"
"/usr/include/dbus-1.0/dbus/dbus-macros.h"
"/usr/include/dbus-1.0/dbus/dbus-signature.h"
"/usr/include/dbus-1.0/dbus/dbus-shared.h"
"/usr/include/dbus-1.0/dbus/dbus-syntax.h"
"/usr/include/dbus-1.0/dbus/dbus-address.h"
"/usr/include/dbus-1.0/dbus/dbus-memory.h"
"/usr/include/dbus-1.0/dbus/dbus-errors.h"
"/usr/include/dbus-1.0/dbus/dbus-server.h"
"/usr/include/dbus-1.0/dbus/dbus-message.h"
"/usr/bin/dbus-test-tool"
%dir "/usr/lib/pkgconfig"
%dir "/usr/lib/dbus-1.0"
%dir "/usr/lib/cmake"
"/usr/lib/libdbus-1.so"
"/usr/lib/pkgconfig/dbus-1.pc"
%dir "/usr/lib/dbus-1.0/include"
%dir "/usr/lib/dbus-1.0/include/dbus"
"/usr/lib/dbus-1.0/include/dbus/dbus-arch-deps.h"
%dir "/usr/lib/cmake/DBus1"
"/usr/lib/cmake/DBus1/DBus1ConfigVersion.cmake"
"/usr/lib/cmake/DBus1/DBus1Config.cmake"

%files -n dbus-doc
%defattr(-,-,-,-)
%dir "/usr"
%dir "/usr/share"
%dir "/usr/share/doc"
%dir "/usr/share/doc/dbus"
%dir "/usr/share/doc/dbus/examples"
"/usr/share/doc/dbus/diagram.svg"
"/usr/share/doc/dbus/system-activation.txt"
"/usr/share/doc/dbus/diagram.png"
"/usr/share/doc/dbus/examples/GetAllMatchRules.py"
"/usr/share/doc/dbus/examples/example-system-enable-stats.conf"
"/usr/share/doc/dbus/examples/example-session-disable-stats.conf"

%files -n dbus-1
%defattr(-,-,-,-)
%dir "/var"
%dir "/usr"
%dir "/etc"
%dir "/var/lib"
%dir "/var/lib/dbus"
%dir "/usr/share"
%dir "/usr/libexec"
%dir "/usr/bin"
%dir "/usr/lib"
%dir "/usr/share/dbus-1"
%dir "/usr/share/xml"
%dir "/usr/share/dbus-1/system.d"
%dir "/usr/share/dbus-1/session.d"
%dir "/usr/share/dbus-1/system-services"
%dir "/usr/share/dbus-1/services"
"/usr/share/dbus-1/session.conf"
"/usr/share/dbus-1/system.conf"
%dir "/usr/share/xml/dbus-1"
"/usr/share/xml/dbus-1/busconfig.dtd"
"/usr/share/xml/dbus-1/introspect.dtd"
"/usr/libexec/dbus-daemon-launch-helper"
"/usr/bin/dbus-run-session"
"/usr/bin/dbus-update-activation-environment"
"/usr/bin/dbus-monitor"
"/usr/bin/dbus-send"
"/usr/bin/dbus-daemon"
"/usr/bin/dbus-cleanup-sockets"
"/usr/bin/dbus-launch"
"/usr/bin/dbus-uuidgen"
%dir "/usr/lib/systemd"
%dir "/usr/lib/systemd/user"
%dir "/usr/lib/systemd/user/sockets.target.wants"
"/usr/lib/systemd/user/dbus.service"
"/usr/lib/systemd/user/dbus.socket"
"/usr/lib/systemd/user/sockets.target.wants/dbus.socket"
%dir "/etc/default"
%dir "/etc/dbus-1"
%dir "/etc/init.d"
%dir "/etc/default/volatiles"
"/etc/default/volatiles/99_dbus"
%config "/etc/dbus-1/session.conf"
%config "/etc/dbus-1/system.conf"
"/etc/init.d/dbus-1"

%files -n dbus-ptest
%defattr(-,-,-,-)

