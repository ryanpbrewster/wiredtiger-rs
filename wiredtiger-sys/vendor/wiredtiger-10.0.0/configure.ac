# DO NOT EDIT
# This file is built automatically from build_posix/configure.ac.in.
PACKAGE=wiredtiger
AC_PREREQ(2.63)
AC_INIT(WiredTiger, m4_normalize(m4_include([build_posix/aclocal/version.m4])),
	[support@wiredtiger.com])

m4_include([build_posix/aclocal/version-set.m4])

AC_CONFIG_AUX_DIR([build_posix/gnu-support])
AC_CONFIG_MACRO_DIR([build_posix/aclocal])
AC_CONFIG_SRCDIR([RELEASE_INFO])

# We rely on some automake features for testing (like AM_TEST_ENVIRONMENT)
# that didn't work before 1.11.6.
AM_INIT_AUTOMAKE([1.11.6 foreign parallel-tests subdir-objects])
m4_ifdef([AM_SILENT_RULES], [AM_SILENT_RULES([no])])

# If CFLAGS/CXXFLAGS were not set on entry, default to "-O3 -g"
: ${CFLAGS=-O3 -g ${ADD_CFLAGS}}
: ${CXXFLAGS=-O3 -g ${ADD_CFLAGS}}

AC_PROG_CC(cc gcc)
AC_PROG_CXX(c++ g++)
AM_PROG_AS(as gas)

define([AC_LIBTOOL_LANG_CXX_CONFIG], [:])dnl
define([AC_LIBTOOL_LANG_F77_CONFIG], [:])dnl

# We only turn on C++ if the compiler is named "cc", otherwise, there's no
# reason to believe "c++" can build compatible objects.
#
# Check whether the C++ compiler works by linking a trivial program.
AM_CONDITIONAL([IS_CXX_OK], [test "$CC" = "cc"])
AM_COND_IF([IS_CXX_OK], [], AM_CONDITIONAL([IS_CXX_OK], [test $(expr `"$CC" --version | head -n 1 | grep -o -E "[[[[:digit:]]]].[[[[:digit:]]]].[[[[:digit:]]]]" | uniq`) = $(expr `"$CXX" --version | head -n 1 | grep -o -E "[[[[:digit:]]]].[[[[:digit:]]]].[[[[:digit:]]]]" | uniq`)]))

AM_COND_IF([IS_CXX_OK],
        [AC_CACHE_CHECK([whether the C++ compiler works],
                       [wt_cv_prog_cxx_works],
                       [AC_LANG_PUSH([C++])
                        AC_LINK_IFELSE([AC_LANG_PROGRAM([], [])],
                                       [wt_cv_prog_cxx_works=yes],
                                       [wt_cv_prog_cxx_works=no])
                        AC_LANG_POP([C++])])],
        [AC_MSG_WARN([C++ compiler ignored unless compiler is named "cc" or gcc and g++ versions match])
        wt_cv_prog_cxx_works=no])

AM_CONDITIONAL([HAVE_CXX], [test "$wt_cv_prog_cxx_works" = "yes"])

LT_PREREQ(2.2.6)
LT_INIT([pic-only])
AC_SUBST([LIBTOOL_DEPS])
AM_CONDITIONAL([POSIX_HOST], [true])
AM_CONDITIONAL([WINDOWS_HOST], [false])

AS_CASE([$host_cpu],
	[ppc64*], [wt_cv_powerpc="yes"],
	[elf64lppc], [wt_cv_powerpc="yes"],
	[powerpc*], [wt_cv_powerpc="yes"],
	[wt_cv_powerpc="no"])
AM_CONDITIONAL([POWERPC_HOST], [test "$wt_cv_powerpc" = "yes"])
AS_CASE([$host_cpu],
	[amd*|i[[3456]]86*|pentium|x86*|mips64el*], [wt_cv_x86="yes"], [wt_cv_x86="no"])
AM_CONDITIONAL([X86_HOST], [test "$wt_cv_x86" = "yes"])
AS_CASE([$host_cpu],
	[s390x*], [wt_cv_zseries="yes"],
	[wt_cv_zseries="no"])
AM_CONDITIONAL([ZSERIES_HOST], [test "$wt_cv_zseries" = "yes"])
AS_CASE([$host_cpu],
	[aarch64*], [wt_cv_arm64="yes"],
	[wt_cv_arm64="no"])
AM_CONDITIONAL([ARM64_HOST], [test "$wt_cv_arm64" = "yes"])
AS_CASE([$host_cpu],
       [mips64el*], [wt_cv_mips64el="yes"],
       [wt_cv_mips64el="no"])
AM_CONDITIONAL([MIPS64EL_HOST], [test "$wt_cv_mips64el" = "yes"])
AS_CASE([$host_os], [*solaris*], [wt_cv_solaris="yes"], [wt_cv_solaris="no"])

# This is a workaround as part of WT-2459. Currently, clang (v3.7) does not
# support compiling the ASM code we have to perform the CRC checks on PowerPC.
# To compile with clang we need to override the ASM compiler with CCAS to use
# gcc. Unfortunately, doing the compilation in this manner means libtool can't
# determine what tag to use for that one .sx file. If we catch that we are using
# two different compilers for CC and CCAS and we are on a PowerPC system we
# overload the libtool flags to provide CC by default.
if test "$wt_cv_powerpc" = "yes" -a "$CC" != "$CCAS"; then
	[AM_LIBTOOLFLAGS+="--tag=CC"]
fi
AC_SUBST(AM_LIBTOOLFLAGS)

# WiredTiger uses anonymous unions to pad structures. It's part of C11, but
# some compilers require -std=c11 to support them. Turn on that flag for any
# compiler that supports it, except for Solaris, where gcc -std=c11 makes
# some non-C11 prototypes unavailable.
if test "$wt_cv_solaris" = "no"; then
	AX_CHECK_COMPILE_FLAG([-std=c11], [AM_CFLAGS="$AM_CFLAGS -std=c11"])
fi

if test "$GCC" = "yes"; then
	# The Solaris gcc compiler gets the additional -pthreads flag.
	if test "$wt_cv_solaris" = "yes"; then
		AM_CFLAGS="$AM_CFLAGS -pthreads"
	fi

	# ARMv8-A is the 64-bit ARM architecture, turn on the optional CRC
	# instructions.
	if test "$wt_cv_arm64" = "yes"; then
		AM_CFLAGS="$AM_CFLAGS -march=armv8-a+crc"
		# moutline-atomics preserves backwards compatibility with Arm v8.0
		# systems but also supports using Arm v8.1 atomics. The latter can
		# massively improve performance on larger Arm systems. The flag was
		# back ported to gcc8, 9 and is the default in gcc10+. See if  the
		# compiler supports the flag.
		AX_CHECK_COMPILE_FLAG([-moutline-atomics], [AM_CFLAGS="$AM_CFLAGS -moutline-atomics"])
	fi
else
	# The Solaris native compiler gets the additional -mt flag.
	if test "$wt_cv_solaris" = "yes"; then
		AM_CFLAGS="$AM_CFLAGS -mt"
	fi
fi

# Linux requires _GNU_SOURCE to be defined
AS_CASE([$host_os], [linux*], [AM_CFLAGS="$AM_CFLAGS -D_GNU_SOURCE"])

# Configure options.
AM_OPTIONS

# If enable-strict is configured, turn on as much error checking as we can for
# this compiler. Intended for developers, and only works for gcc/clang, but it
# fills a need.
if test "$wt_cv_enable_strict" = "yes"; then
	wt_cv_cc_version="`$CC --version | sed -eq`"
	case "$wt_cv_cc_version" in
	*clang*)
		AM_CLANG_WARNINGS($wt_cv_cc_version);;
	*cc*|*CC*)					# cc, CC, gcc, GCC
		AM_GCC_WARNINGS($wt_cv_cc_version);;
	*)
		AC_MSG_ERROR(
		    [--enable-strict does not support "$wt_cv_cc_version".]);;
	esac

	AM_CFLAGS="$AM_CFLAGS $wt_cv_strict_warnings"
fi

# Python API
if test "$wt_cv_enable_python" = "yes"; then
	# Only a warning, we need to build release packages without SWIG.
	AX_PKG_SWIG(2.0.4, [],
	    [AC_MSG_WARN([SWIG is required to rebuild Python API.]) &&
              SWIG="SWIG_NOT_FOUND_DURING_CONFIGURE"])
fi

if test "$wt_cv_enable_python" = "yes"; then
	AM_PATH_PYTHON([3.0])
	if test -n "$with_python_prefix" ; then
		PYTHON_INSTALL_ARG="-d $with_python_prefix"
	fi
	AC_SUBST(PYTHON_INSTALL_ARG)
fi

AM_TYPES

AC_PROG_INSTALL

AC_CHECK_HEADERS([x86intrin.h])
AC_CHECK_LIB(pthread, pthread_create)
AC_CHECK_LIB(dl, dlopen)
AC_CHECK_LIB(rt, sched_yield)

AC_CHECK_FUNCS([\
	clock_gettime fallocate ftruncate gettimeofday posix_fadvise posix_fallocate\
	posix_madvise setrlimit strtouq sync_file_range timer_create])

# OS X wrongly reports that it has fdatasync
AS_CASE([$host_os], [darwin*], [], [AC_CHECK_FUNCS([fdatasync])])

# Check for posix_memalign explicitly: it is a builtin in some compilers and
# the generic declaration in AC_CHECK_FUNCS is incompatible.
AX_FUNC_POSIX_MEMALIGN

# Check for POSIX condition variables with monotonic clock support
AC_CACHE_CHECK([for condition waits with monotonic clock support],
  [wt_cv_pthread_cond_monotonic],
  [AC_RUN_IFELSE([AC_LANG_SOURCE([[
#include <errno.h>
#include <pthread.h>
#include <stdlib.h>
#include <time.h>

int main()
{
  int ret;
  pthread_condattr_t condattr;
  pthread_cond_t cond;
  pthread_mutex_t mtx;
  struct timespec ts;

  if ((ret = pthread_condattr_init(&condattr)) != 0) exit(1);
  if ((ret = pthread_condattr_setclock(&condattr, CLOCK_MONOTONIC)) != 0) exit(1);
  if ((ret = pthread_cond_init(&cond, &condattr)) != 0) exit(1);
  if ((ret = pthread_mutex_init(&mtx, NULL)) != 0) exit(1);
  if ((ret = clock_gettime(CLOCK_MONOTONIC, &ts)) != 0) exit(1);
  ts.tv_sec += 1;
  if ((ret = pthread_mutex_lock(&mtx)) != 0) exit(1);
  if ((ret = pthread_cond_timedwait(&cond, &mtx, &ts)) != 0 && ret != EINTR && ret != ETIMEDOUT) exit(1);

  exit(0);
}
    ]])],
    [wt_pthread_cond_monotonic=yes],
    [wt_pthread_cond_monotonic=no],
    [wt_pthread_cond_monotonic=no])])
AC_MSG_RESULT($wt_pthread_cond_monotonic)
if test "$wt_pthread_cond_monotonic" = "yes" ; then
  AC_DEFINE([HAVE_PTHREAD_COND_MONOTONIC], [1],
    [Define to 1 if pthread condition variables support monotonic clocks.])
fi

AC_SYS_LARGEFILE

AC_C_BIGENDIAN

AC_MSG_CHECKING([for a 64-bit build])
AC_COMPUTE_INT(ac_cv_sizeof_void_p, [sizeof(void *)])
if test "$ac_cv_sizeof_void_p" != "8" ; then
    AC_MSG_ERROR([WiredTiger requires a 64-bit build.])
fi
AC_MSG_RESULT(yes)

# Linux requires buffers aligned to 4KB boundaries for O_DIRECT to work.
BUFFER_ALIGNMENT=0
if test "$ax_cv_func_posix_memalign_works" = "yes" ; then
	case "$host_os" in
	linux*)	BUFFER_ALIGNMENT=4096 ;;
	esac
fi
AC_DEFINE_UNQUOTED(WT_BUFFER_ALIGNMENT_DEFAULT, $BUFFER_ALIGNMENT,
	[Default alignment of buffers used for I/O])

AC_SUBST(AM_CFLAGS)

# Warn that diagnostic builds should not be used in production
if test "$wt_cv_enable_diagnostic" = "yes"; then
	AC_MSG_WARN(
	    [DIAGNOSTIC BUILDS ARE NOT RECOMMENDED FOR PRODUCTION DEPLOYMENT.])
fi

# If we are building in a tree without documentation, check if doxygen is
# available.
if test -f "$srcdir/docs/index.html" ; then
	wt_cv_docs_exist=yes
else
	wt_cv_docs_exist=no
fi

if test "$wt_cv_docs_exist" = "no"; then
	AC_CHECK_PROG([DOXYGEN], [doxygen], [doxygen], [false])
fi

if test "$wt_cv_docs_exist" = "yes" -o "$DOXYGEN" = "doxygen" ; then
	MAN1_PAGES="$srcdir/docs/man/man1/wt.1"
	AC_SUBST(MAN1_PAGES)
	MAN3_PAGES="$srcdir/docs/man/man3/wiredtiger.3"
	AC_SUBST(MAN3_PAGES)
fi

# Output files
AC_CONFIG_HEADERS([wiredtiger_config.h:build_posix/config.hin])

# BEGIN check existence -- maintained by reconf and Make.subdirs
AC_CONFIG_FILES([ext/collators/reverse/Makefile])
AC_CONFIG_FILES([ext/collators/revint/Makefile])
AC_CONFIG_FILES([ext/compressors/lz4/Makefile])
AC_CONFIG_FILES([ext/compressors/nop/Makefile])
AC_CONFIG_FILES([ext/compressors/snappy/Makefile])
AC_CONFIG_FILES([ext/compressors/zlib/Makefile])
AC_CONFIG_FILES([ext/compressors/zstd/Makefile])
AC_CONFIG_FILES([ext/encryptors/nop/Makefile])
AC_CONFIG_FILES([ext/encryptors/rotn/Makefile])
AC_CONFIG_FILES([ext/extractors/csv/Makefile])
AC_CONFIG_FILES([ext/storage_sources/local_store/Makefile])
AC_CONFIG_FILES([ext/test/fail_fs/Makefile])
AC_CONFIG_FILES([lang/python/Makefile])
AC_CONFIG_FILES([test/utility/Makefile])
AC_CONFIG_FILES([examples/c/Makefile])
AC_CONFIG_FILES([test/bloom/Makefile])
AC_CONFIG_FILES([test/checkpoint/Makefile])
AC_CONFIG_FILES([test/cppsuite/Makefile])
AC_CONFIG_FILES([test/csuite/Makefile])
AC_CONFIG_FILES([test/cursor_order/Makefile])
AC_CONFIG_FILES([test/fops/Makefile])
AC_CONFIG_FILES([test/format/Makefile])
AC_CONFIG_FILES([test/fuzz/Makefile])
AC_CONFIG_FILES([test/huge/Makefile])
AC_CONFIG_FILES([test/manydbs/Makefile])
AC_CONFIG_FILES([test/packing/Makefile])
AC_CONFIG_FILES([test/readonly/Makefile])
AC_CONFIG_FILES([test/salvage/Makefile])
AC_CONFIG_FILES([test/syscall/Makefile])
AC_CONFIG_FILES([test/thread/Makefile])
AC_CONFIG_FILES([bench/workgen/Makefile])
AC_CONFIG_FILES([bench/wtperf/Makefile])
AC_CONFIG_FILES([tools/xray_to_optrack/Makefile])
# END check existence

AC_CONFIG_FILES([
	Makefile
	wiredtiger.h:src/include/wiredtiger.in
	wiredtiger_ext.h:src/include/wiredtiger_ext.h
	wiredtiger.pc:build_posix/wiredtiger.pc.in
])
AC_OUTPUT
