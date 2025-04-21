NAME
rcat – concatenate and print files

SYNOPSIS
rcat [-bns] [file ...]

DESCRIPTION
The rcat utility reads files sequentially, writing them to the standard output. The file operands
are processed in command-line order. If file is a single dash (-) or absent, rcat reads from
the standard input.

     The options are as follows:

     -b      Number the non-blank output lines, starting at 1.

     -n      Number all the output lines, starting at 1.

     -s      Squeeze multiple adjacent empty lines, causing the output to be single spaced.
