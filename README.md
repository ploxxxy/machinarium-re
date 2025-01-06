# Machinarium Reverse Engineered
A collection of tools and scripts to extract files and various data from Machinarium The Definitive Edition mysterious .jpg files

## get_level_hash
Implementation of the hashing algorithm used by the game to get the filename of a level (located in `Machinarium/arch_<platform>/xxxxx.jpg`)

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
Implementation of the decoding algorithm used by the game to store multiple files inside a single level archive

Includes various checks performed by the game during the decoding process 

The processed file will have following format:
- a 48kiB header, mostly empty, but periodically has (from my understanding) `unknown` (4 bytes) + `offset` (4 bytes) + `size` (4 bytes) information about the files stored in the archive
- the payload, which contains multiple files, padded to be divisble by 32
- 16 byte header with the `unknown flag` (4 bytes), `file size` (4 bytes), `checksum` (4 bytes) + `unknown value` (4 bytes) information

---

Next step is to process the archive, extract the files and figure out their respective filenames