GENCCODE_ASSEMBLY_TYPE=-a gcc
SO=so
SOBJ=so
A=a
LIBPREFIX=lib
LIB_EXT_ORDER=.66.1
COMPILE=gcc  -isystem/home/hhpartners/yocto/build/tmp/work/all-poky-linux/adwaita-icon-theme/3.34.3-r0/recipe-sysroot-native/usr/include -D_REENTRANT  -DU_HAVE_ELF_H=1 -DU_HAVE_STRTOD_L=1 -DU_HAVE_XLOCALE_H=0  -DU_ATTRIBUTE_DEPRECATED= -isystem/home/hhpartners/yocto/build/tmp/work/all-poky-linux/adwaita-icon-theme/3.34.3-r0/recipe-sysroot-native/usr/include -O2 -pipe -std=c11 -Wall -pedantic -Wshadow -Wpointer-arith -Wmissing-prototypes -Wwrite-strings   -c
LIBFLAGS=-I/home/hhpartners/yocto/build/tmp/work/all-poky-linux/adwaita-icon-theme/3.34.3-r0/recipe-sysroot-native/usr/include -DPIC -fPIC
GENLIB=gcc  -isystem/home/hhpartners/yocto/build/tmp/work/all-poky-linux/adwaita-icon-theme/3.34.3-r0/recipe-sysroot-native/usr/include -O2 -pipe -std=c11 -Wall -pedantic -Wshadow -Wpointer-arith -Wmissing-prototypes -Wwrite-strings   -L/home/hhpartners/yocto/build/tmp/work/all-poky-linux/adwaita-icon-theme/3.34.3-r0/recipe-sysroot-native/usr/lib                         -L/home/hhpartners/yocto/build/tmp/work/all-poky-linux/adwaita-icon-theme/3.34.3-r0/recipe-sysroot-native/lib                         -Wl,--enable-new-dtags                         -Wl,-rpath-link,/home/hhpartners/yocto/build/tmp/work/all-poky-linux/adwaita-icon-theme/3.34.3-r0/recipe-sysroot-native/usr/lib                         -Wl,-rpath-link,/home/hhpartners/yocto/build/tmp/work/all-poky-linux/adwaita-icon-theme/3.34.3-r0/recipe-sysroot-native/lib                         -Wl,-rpath,/home/hhpartners/yocto/build/tmp/work/all-poky-linux/adwaita-icon-theme/3.34.3-r0/recipe-sysroot-native/usr/lib                         -Wl,-rpath,/home/hhpartners/yocto/build/tmp/work/all-poky-linux/adwaita-icon-theme/3.34.3-r0/recipe-sysroot-native/lib                         -Wl,-O1 -Wl,--allow-shlib-undefined -Wl,--dynamic-linker=/home/hhpartners/yocto/build/tmp/sysroots-uninative/x86_64-linux/lib/ld-linux-x86-64.so.2  -shared -Wl,-Bsymbolic
LDICUDTFLAGS=-nodefaultlibs -nostdlib
LD_SONAME=-Wl,-soname -Wl,
RPATH_FLAGS=
BIR_LDFLAGS=-Wl,-Bsymbolic
AR=ar
ARFLAGS=r
RANLIB=ranlib
INSTALL_CMD=/home/hhpartners/yocto/build/tmp/hosttools/install -c
