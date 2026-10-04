# French Press
## This is a simple project iam rebuilding to learn about good coding practices.


## what am i planning to do?
I am trying to build a compression tool which takes in a file path and outputs a huffman compressed output file. for the mvp the tool
only need to take an input file output the output file. with two flags one for decompression and one for comperssed.

## Inputs
Filepath
Flags(-dec,-enc)

## Outputs
compresssed file.
or
decompressed file.

## Dont do's
no custom or extra flags
no streams handling.
no input from stdin.

## File Layout.(draft)

Header
* u32: magic byte to verify identity.
* u16: stores the number of bytes.
* (u8,u64): the frequency table

Body
* compressed data.

## Modules

* input handler
* encoder
* decoder

## Basic funcitons

- [ ] count frequencies
- [ ] build the huffman tree
- [ ] traverse the tree to replace each charector in the file
- [ ] write the the file header.
  
what will go wrong here?
the file not existing.

in case of the encoding what is the pipeline?
read the file count frequencies of each charectors, build the huffman tree,
use it to replace bytes, write the code table and replaced byte to a file 

in case of the decoding what is the pipeline?
read the header part, build the codes using cannonnical huffman codes, decode the data.
output to a file.
