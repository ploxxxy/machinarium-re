# Machinarium Reverse Engineered
A collection of tools and scripts to extract files and various data from Machinarium The Definitive Edition mysterious .jpg files

## get_level_hash
The game uses hardcoded values (music, shared, startup, etc), which are then hashed to load the actual files (7005.jpg, etc), located in `Machinarium/arch_<platform>/xxxxx.jpg`. 

All files accessed by the game:
```yaml
music: 7005
shared: 13516
startup: 29625
level00intromenu: 14727510
level01: 16621
level02: 16750
level03: 16879
level04: 17008
level05: 17137
level06: 17266
level07: 17395
level08: 17524
level09: 17653
level10: 16557
level11: 16686
level12: 16815
level13: 16944
level14: 17073
level15: 17202
level16: 17331
level17: 17460
level18: 17589
level19: 17718
level20: 16622
level21: 16751
level22: 16880
level23: 17009
level24: 17138
level25: 17267
level26: 17396
level27: 17525
```


## decode_level_file
Implementation of the decoding algorithm for the archives used in the game for loading levels and other data

Includes various checks performed by the game during the decoding process 

The processed file will have following format:
- a 48kiB header, mostly empty, with peridocal pointers to the archive data
- the payload, which contains multiple files, padded to be divisble by 32
- 16 byte footer with the `unknown flag` (4 bytes), `file size` (4 bytes), `checksum` (4 bytes) + `unknown value` (4 bytes) information

## extract_level_files
Educated guess on how to extract level files from the game's data files, since I haven't actually found the extraction logic in the code yet.

My implementation walks through the header and extracts "pointers" (`unknown` (most likely a checksum) of 4 bytes + `offset` of 4 bytes + `size` of 4 bytes) to the actual files located in the payload. 

Most likely it is impossible to determine the file names just from the archive files, as they share similarities in loading logic with level archives.