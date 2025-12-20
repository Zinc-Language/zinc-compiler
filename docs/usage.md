# Zinc Usage

### Compiler Flags

Zinc contains lots of flags that can customize the behavior of the compiler, ast generation, and more. These flags are passed to the compiler by typing `zinc -<flag>` for **single letter flags** or `zinc --<flag>` for **fully worded flags**, you would replace `<flag>` with your flag name or letter.

Here is an example of a compiler flag being used:
```
zinc --help
```
This would print the usage of zinc and some usefull commands.

**Output**
```
$ zinc --help
usage: zincc [options] <file.zc>
options:
  -h, --help                     Prints this help message
  -o <output path>               Outputs the compiled binary to the specified file
  -sts, --show-token-start       Prints the source code with the token start marker
  -pt, --print-tokens            Prints the raw token contents
  -et, --export-tokens           Exports the raw token contents to a file
  -ph, --print-highlight         Prints the source code with highlighted tokens
```