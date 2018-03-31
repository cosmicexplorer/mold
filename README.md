`mold` (Mach-O ld)
==================

`mold` is a linker emulating OSX `ld` written in Rust to support linking 64-bit Mach-O files using the [GNU bfd library](https://ftp.gnu.org/old-gnu/Manuals/bfd-2.9.1/html_chapter/bfd_2.html). `mold` is licensed under the [Apache 2.0 license](./LICENSE).

# Progress
## Hello, World!

In an OSX High Sierra environment:
``` bash
> /usr/bin/clang -c test.c -fno-lto
> ld -execute -arch x86_64 -macosx_version_min 10.13.0 -o test test.o -lSystem /Library/Developer/CommandLineTools/usr/lib/clang/9.0.0/lib/darwin/libclang_rt.osx.a
> ./test
hello, world!
```

(see [`lib::make_executable()`](./src/lib.rs))

# Arguments

*Note:* make `-h`/`--help` usable like any other normal cli tool!!!!

## File/Path
- [ ] `-o <path>`
- [ ] `-l<name>`
- [ ] `-L<dir>`
- [ ] `-framework <name>[,<suffix>]`
- [ ] `-F<dir>`

## Output Configuration
- [ ] `-execute`
- [ ] `-dylib`
- [ ] `-bundle`
- [ ] `-r`
- [ ] `-dynamic`
- [ ] `-arch <arch_name>`

# Links
- [Apple Mach-O docs](https://developer.apple.com/library/content/documentation/DeveloperTools/Conceptual/MachOTopics/0-Introduction/introduction.html)
- [Mach-O file parsing walkthrough](https://lowlevelbits.org/parsing-mach-o-files/)
- [Mach-O file format reference from this github repo](https://github.com/aidansteele/osx-abi-macho-file-format-reference)
- [Mach-O on wikipedia](https://en.wikipedia.org/wiki/Mach-O)

# License
- [Apache 2.0](./LICENSE)
