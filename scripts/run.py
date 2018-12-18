import argparse
import contextlib
import glob
import os
import subprocess

import ninja


# Helpers

def glob_from(root, pattern):
    return [os.path.relpath(f, root)
            for f in glob.glob(os.path.join(root, pattern), recursive=True)]


def stripext(f):
    return os.path.splitext(f)[0]


def sh(cmd, **args):
    code = subprocess.call(cmd.format(**args), shell=True)
    if code:
        exit(code)


# Commands

def generate(build_file, release):
    with open(build_file, 'w') as f, \
         contextlib.closing(ninja.Writer(f, width=120)) as n:
        n.variable('builddir', 'build')
        n.variable('projectdir', '.')
        n.variable('cxx', 'g++')
        n.variable('cxxflags',
                   '-c -fpic -Wall -Wextra -Werror -std=c++17 {variant_flags} {include_flags}'.format(
                       variant_flags='-O3 -mtune=native' if release else '-O0 -g',
                       include_flags=' '.join('-I ' + d for d in [
                           '$projectdir/include',
                           '$builddir/gen',
                           '$builddir/third-party/include',
                           '/usr/include/python3.6m',
                           '/usr/local/lib/python3.6/dist-packages/numpy/core/include']),
                   ))
        n.variable('linkflags', '-Wl,--no-undefined')

        n.rule('cxx', '$cxx $cxxflags $in -MMD -MF $out.d -o $out', depfile='$out.d', deps='gcc')
        n.rule('link', '$cxx $linkflags $in -o $out $libs')
        n.rule('download', 'wget -qO $out $url')
        n.rule('flatc', 'flatc --python --cpp --gen-object-api -o $dir $in')

        # Pre-build

        n.build('$builddir/third-party/include/catch.hpp',
                'download',
                variables=dict(url='https://github.com/catchorg/Catch2/releases/download/v2.3.0/catch.hpp'))

        n.build('$builddir/gen/flategy_data_generated.h',
                'flatc',
                '$projectdir/src/flategy_data.fbs',
                variables=dict(dir='$builddir/gen'),
                implicit_outputs='$builddir/gen/flategy_data/')

        # Main build

        for cpp in glob_from('src', '**/*.cpp'):
            n.build('$builddir/obj/{}.o'.format(stripext(cpp)),
                    'cxx',
                    '$projectdir/src/{}'.format(cpp),
                    order_only=['$builddir/third-party/include/catch.hpp',
                                '$builddir/gen/flategy_data_generated.h'])

        n.build('$builddir/libflategy.so',
                'link',
                ['$builddir/obj/{}.o'.format(stripext(cpp)) for cpp in glob_from('src', '*.cpp')],
                variables=dict(libs='-lpython3.6m', linkflags='-shared'))

        n.build('$builddir/tests',
                'link',
                ['$builddir/obj/{}.o'.format(stripext(cpp)) for cpp in glob_from('src', 'test/*.cpp')],
                variables=dict(libs='-L$builddir -lflategy'),
                implicit='$builddir/libflategy.so')


def build(targets, **generate_args):
    generate(**generate_args)
    sh('ninja {targets}', targets=' '.join(targets))
    # Make sure we can easily view the built files
    sh('find build -type d -exec chmod ugo+rx {{}} +')
    sh('find build -type f -exec chmod ugo+r {{}} +')


def test(**generate_args):
    print('### Building...')
    build(['build/tests'], **generate_args)
    print('### C++ tests...')
    sh('env LD_LIBRARY_PATH=build ./build/tests')
    print('### Python tests...')
    sh('pytest flategy')
    print('### Python lint...')
    sh('flake8')


def download_js(**ignored):
    base = 'static/lib'
    libraries = [
        ('jquery.js', 'https://code.jquery.com/jquery-3.3.1.slim.min.js'),
        ('bootstrap.css', 'https://stackpath.bootstrapcdn.com/bootstrap/4.1.3/css/bootstrap.min.css'),
        ('bootstrap.js', 'https://stackpath.bootstrapcdn.com/bootstrap/4.1.3/js/bootstrap.min.js'),
        ('popper.js', 'https://cdnjs.cloudflare.com/ajax/libs/popper.js/1.14.3/umd/popper.min.js'),
    ]
    sh('mkdir -p {base}', base=base)
    for name, src in libraries:
        sh('wget -qO {dest} {src}', src=src, dest=os.path.join(base, name))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Flategy task runner')
    parser.add_argument('--build-file', default='build.ninja', help='path to Ninja build file')
    parser.add_argument('--release', action='store_true', help='compile with optimization')
    parser.set_defaults(action=test)
    subs = parser.add_subparsers()

    subs.add_parser('generate').set_defaults(action=generate)

    p = subs.add_parser('build')
    p.add_argument('targets', nargs='*', default=[])
    p.set_defaults(action=build)

    subs.add_parser('test').set_defaults(action=test)

    subs.add_parser('download-js').set_defaults(action=download_js)

    args = vars(parser.parse_args())
    args.pop('action')(**args)
