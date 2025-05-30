project('mbedtls', 'c', version: '3.6.2')

cc = meson.get_compiler('c')
host_has_glibc = cc.get_define('__GLIBC__', prefix: '#include <features.h>') != ''

pkgconfig = import('pkgconfig')

mbedtls_libs_output = [
    'libmbedcrypto_gramine.so.16',
    'libmbedcrypto_gramine.so',
    'libmbedtls_gramine.so.21',
    'libmbedtls_gramine.so',
    'libmbedx509_gramine.so.7',
    'libmbedx509_gramine.so',
    'libmbedcrypto_gramine.a',
    'libmbedtls_gramine.a',
    'libmbedx509_gramine.a',
]

# TODO: This is custom_target, because we need to patch mbedtls before compiling.
# PR for patch support in wraps: https://github.com/mesonbuild/meson/pull/4570
mbedtls_libs = custom_target('mbedtls',
    command: [
        find_program('compile-gramine.sh'),
        '@CURRENT_SOURCE_DIR@',
        '@CURRENT_SOURCE_DIR@/mbedtls-3.6.2',
        meson.current_build_dir(),
        '@PRIVATE_DIR@',
        '@OUTPUT@',
        '--',
        'SUFFIX=_gramine',
        'SHARED=1',
    ],

    input: ['mbedtls-3.6.2/Makefile', 'gramine.patch'],

    # NOTE we need real sonames here (.so.N, not .so), please keep synced with
    # mbedtls/library/Makefile, variables SOEXT_{TLS,X509,CRYPTO}
    output: mbedtls_libs_output,

    install: true,
    install_dir: get_option('libdir'),
)

pkgconfig.generate(
    name: 'mbedtls_gramine',
    filebase: 'mbedtls_gramine',
    description: 'A version of mbedtls patched for Gramine',
    subdirs: 'gramine',
    libraries: [
        '-L${libdir}',
        '-Wl,-rpath,${libdir}',
        '-Wl,--start-group',
        '-lmbedcrypto_gramine',
        '-lmbedtls_gramine',
        '-lmbedx509_gramine',
        '-Wl,--end-group',
    ],
)

if get_option('libc') == 'glibc' and host_has_glibc
    foreach output : mbedtls_libs_output
        meson.add_install_script('/bin/sh', '-c',
            ('ln -sf ../../../@0@ ' +
            '"$MESON_INSTALL_DESTDIR_PREFIX"/@1@/gramine/runtime/@2@/').format(
                output, get_option('libdir'), get_option('libc')))
    endforeach
endif

# We rely on the fact that for `mbedtls_gramine` package, we don't need any changes in the default
# mbedTLS headers
install_subdir('mbedtls-3.6.2/include/mbedtls', install_dir: get_option('includedir') / 'gramine')
install_subdir('mbedtls-3.6.2/include/psa', install_dir: get_option('includedir') / 'gramine')

mbedtls_pal_libs = custom_target('mbedtls_pal',
    command: [
        find_program('compile-pal.sh'),
        '@CURRENT_SOURCE_DIR@',
        '@CURRENT_SOURCE_DIR@/mbedtls-3.6.2',
        meson.current_build_dir(),
        '@PRIVATE_DIR@',
        '@OUTPUT@',
        '--',
        'SUFFIX=_pal',
    ],

    input: ['mbedtls-3.6.2/Makefile', 'gramine.patch'],

    output: [
        'libmbedcrypto_pal.a',
        'libmbedtls_pal.a',
        'libmbedx509_pal.a',
    ],

    build_by_default: true,
)

mbedtls_curl_libs = custom_target('mbedtls_curl',
    command: [
        find_program('compile-curl.sh'),
        '@CURRENT_SOURCE_DIR@',
        '@CURRENT_SOURCE_DIR@/mbedtls-3.6.2',
        meson.current_build_dir(),
        '@PRIVATE_DIR@',
        meson.build_root() / 'subprojects',
        '@OUTPUT@',
    ],

    input: ['mbedtls-3.6.2/Makefile', 'gramine.patch'],

    output: [
        'libmbedcrypto.a',
        'libmbedtls.a',
        'libmbedx509.a',
    ],

    build_by_default: true,
)

mbedtls_inc = include_directories('include', 'mbedtls-3.6.2/include')

mbedtls_static_dep = declare_dependency(
    link_with: [mbedtls_libs[6], mbedtls_libs[7], mbedtls_libs[8]],
    include_directories: mbedtls_inc,
)
mbedtls_pal_dep = declare_dependency(
    # HACK: Apparently Meson considers the `mbedtls_pal_libs` to be "not linkable", because it has
    # multiple outputs; however, it allows picking the outputs one by one.
    link_with: [mbedtls_pal_libs[0], mbedtls_pal_libs[1], mbedtls_pal_libs[2]],
    include_directories: mbedtls_inc,
    compile_args: '-DMBEDTLS_CONFIG_FILE="mbedtls/config-pal.h"',
)
mbedtls_curl_dep = declare_dependency(
    # HACK: Apparently Meson considers the `mbedtls_curl_libs` to be "not linkable", because it has
    # multiple outputs; however, it allows picking the outputs one by one.
    link_with: [mbedtls_curl_libs[0], mbedtls_curl_libs[1], mbedtls_curl_libs[2]],
    include_directories: mbedtls_inc,
    compile_args: '-DMBEDTLS_CONFIG_FILE="mbedtls/config-pal.h"',
)
