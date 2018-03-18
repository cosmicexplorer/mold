`mold` (Mach-O ld)
==================

`mold` is a linker emulating OSX `ld` written in Rust to support linking 64-bit Mach-O files using the [GNU bfd library](https://ftp.gnu.org/old-gnu/Manuals/bfd-2.9.1/html_chapter/bfd_2.html).

# Progress


# Arguments
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

# License
- [Apache 2.0](./LICENSE)
