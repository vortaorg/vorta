common_lib = static_library('test_fs',
    'common.c',
)

common_lib_copy = static_library('test_fs_copy',
    'common_copy.c',
)

common_lib_copy_mmap = static_library('test_fs_copy_mmap',
    'common_copy.c',
    c_args: [
        '-DCOPY_MMAP',
    ],
)

tests = {
    'chmod_stat': {},
    'copy_mmap_rev': {
        'link_with': common_lib_copy_mmap,
    },
    'copy_mmap_seq': {
        'link_with': common_lib_copy_mmap,
    },
    'copy_mmap_whole': {
        'link_with': common_lib_copy_mmap,
    },
    'copy_rev': {
        'link_with': common_lib_copy,
    },
    'copy_seq': {
        'link_with': common_lib_copy,
    },
    'copy_whole': {
        'link_with': common_lib_copy,
    },
    'copy_sendfile': {
        'link_with': common_lib_copy,
    },
    'delete': {},
    'multiple_writers': {
        'link_args': '-lpthread',
    },
    'open_close': {},
    'open_flags': {},
    'read_append': {},
    'read_write': {},
    'read_write_mmap': {},
    'seek_tell': {},
    'seek_tell_truncate': {},
    'stat': {},
    'truncate': {},
}

install_dir = pkglibdir / 'tests' / 'libos' / 'fs'

foreach name, params : tests
    exe = executable(name,
        '@0@.c'.format(name),

        link_with: [
            common_lib,
            params.get('link_with', []),
        ],

        link_args: params.get('link_args', []),

        install: true,
        install_dir: install_dir,
    )
endforeach
