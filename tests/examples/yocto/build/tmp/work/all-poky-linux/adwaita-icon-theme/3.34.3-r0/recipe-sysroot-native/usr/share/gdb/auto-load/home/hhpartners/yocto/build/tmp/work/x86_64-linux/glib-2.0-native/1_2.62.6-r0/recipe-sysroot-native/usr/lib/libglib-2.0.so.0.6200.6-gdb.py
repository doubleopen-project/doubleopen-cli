import sys
import gdb

# Update module path.
dir_ = '/home/hhpartners/yocto/build/tmp/work/all-poky-linux/adwaita-icon-theme/3.34.3-r0/recipe-sysroot-native/usr/share/glib-2.0/gdb'
if not dir_ in sys.path:
    sys.path.insert(0, dir_)

from glib_gdb import register
register (gdb.current_objfile ())
