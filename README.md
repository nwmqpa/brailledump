# brailledump

This is a simple project aimed at transcribing blobs of binary data using the braille character set.


## How to install

```
cargo install brailledump
```

## How to use

```
; echo "Hello World" | brailledump
00000000| ⠘⠒⠁⠂ ⠘⠃⠚⠈ ⠘⠃⠙⠀ ⠈⠁⠋⠂ ⠈⠁⠉⠉ ⠀⠁⠀⠀ ⠈⠈⠈⠉ ⠈⠁⠉⠉
```

```
; curl http://info.cern.ch | brailledump
00000000| ⠰⣏⣙⡰ ⠸⣇⣡⠐ ⢘⡯⡼⠄ ⢈⡷⢿⢊ ⢨⡗⠛⢄ ⢠⡏⢿⠷ ⢰⡏⢫⢀ ⢸⣧⠕⡰
00000020| ⢠⡟⣟⡣ ⠰⡆⣧⡅ ⢰⡟⠹⣄ ⢸⡟⡼⠤ ⠘⣗⣥⠌ ⠨⡟⣊⣦ ⢨⣇⢻⠲ ⢨⡧⡚⢞
00000040| ⠘⣯⡼⡐ ⠸⡇⣍⣐ ⠘⡧⣘⣨ ⢐⡯⡽⢕ ⢰⡖⣅⡃ ⢠⡿⢻⡂ ⢠⡥⣫⣋ ⠨⡥⣃⡂
00000060| ⢨⣇⠲⡝ ⢈⣇⠈⣈ ⢸⣯⢢⠁ ⠸⡇⠟⠓ ⢰⣇⢻⣱ ⢘⡇⢐⢙ ⢨⡇⠡⡄ ⢰⣇⠖⣲
00000080| ⢈⣅⢃⢊ ⠨⡟⠞⠀ ⢸⡗⠨⣨ ⢠⡿⠛⢆ ⢰⡅⣹⡫ ⠘⣗⡁⡂ ⠰⣯⣖⠺ ⠰⡏⣿⣵
000000a0| ⢨⣿⠒⠄ ⠰⣏⡻⡡ ⠰⡆⣣⣕ ⠀⡿⣿⣢ ⢈⡟⡚⢌ ⢸⡇⣍⡐ ⢠⡯⢩⡁ ⢰⡦⣣⣁
000000c0| ⢠⡧⡟⠓ ⢸⡧⢢⢉ ⢘⢗⠮⠼ ⢸⣻⠠⡥ ⢸⡣⣹⣵ ⢰⡳⡩⡧ ⢘⡗⢴⢭ ⢸⡣⠱⣐
000000e0| ⠸⡍⠌⠂ ⢰⣗⢩⣣ ⢘⡇⢓⢒ ⢸⡟⠸⡖ ⢸⣗⠍⣺ ⢸⡧⡙⢔ ⢠⣧⢀⠥ ⢰⡿⢹⢁
00000100| ⢀⣟⢛⠂ ⢠⣅⢏⠋ ⢨⣷⠒⠌ ⠰⣏⡻⡡ ⠰⡏⣫⣔ ⠀⡷⣿⣫ ⢈⡗⣛⠄ ⢸⡇⡅⢘
00000120| ⢸⣧⠩⡷ ⢸⣧⢸⠼ ⢰⡷⢩⢮ ⢘⣇⡿⠮ ⢨⣇⢟⠋ ⠸⡧⣌⣲ ⢸⢧⢊⣈ ⢠⢇⢻⣳
00000140| ⢸⡎⢮⢋ ⠐⡇⠙⠝ ⢘⣮⢬⠖ ⢸⡓⡑⠄ ⢨⡧⢚⢎ ⠸⡎⠦⠤ ⢸⡿⢰⡥ ⢸⡧⡛⢽
00000160| ⢸⡏⣢⡵ ⢘⡟⢐⢉ ⠸⡯⣸⢐ ⢠⡇⣄⢀ ⢸⡟⣸⣹ ⢘⡗⢈⠚ ⢸⡇⢦⢑ ⠸⡏⠖⠫
00000180| ⢸⡷⡀⢗ ⠘⣯⣦⡑ ⠘⠇⡿⡽ ⠸⣟⣘⠹ ⢘⡯⣶⠍ ⢘⡧⡬⢘ ⠘⣟⣴⡥ ⠰⣇⣶⠒
000001a0| ⢸⡇⠦⣬ ⢰⡗⢴⢠ ⢸⣗⠱⡠ ⢘⡟⣤⡅ ⠨⡷⣪⡮ ⢨⡇⠺⣳ ⢠⡏⡛⠞ ⠐⡇⣦⣅
000001c0| ⢘⡏⠊⢔ ⢘⡷⠭⡏ ⢸⡛⣴⣀ ⢸⣇⢣⢨ ⢨⣇⢒⠹ ⠸⡿⠐⠟ ⢰⣇⢽⠝ ⢘⡇⡀⠃
000001e0| ⢸⡇⣺⠺ ⢰⡇⡐⢆ ⠈⣧⣤⡁ ⠘⠗⡵⡬ ⠸⣏⣂⠡ ⢘⡯⣼⠔ ⢈⡧⡥⢀ ⠐⣗⣴⡶
00000200| ⠠⡏⣍⡂ ⢸⡇⠦⣬ ⢰⡗⢴⢠ ⢸⣗⠱⡠ ⢘⡟⣤⡅ ⠨⡷⣪⡮ ⢨⡇⠺⣳ ⢠⡏⡛⠞
00000220| ⠰⡅⣿⠭ ⠸⡧⠰⠸ ⢸⣧⢠⠑ ⢘⡗⡉⠋ ⢸⡋⢚⢮ ⠨⡋⠨⠠ ⢰⣣⠀⠕ ⢰⡋⡭⠇
00000240| ⢘⡏⡁⠊ ⢸⡯⢲⢿ ⠸⡗⠅⠊ ⢸⣇⢠⣹ ⢸⡿⢐⢍ ⢰⡇⠲⡲ ⠘⡗⠉⠂ ⢸⣷⢢⣘
00000260| ⠈⡃⣖⣞ ⢘⡯⠤⡙ ⢀⡗⣶⣦ ⢨⡷⢲⠡ ⢨⣇⡿⢛ ⠘⣯⣶⡅ ⠘⣃⣏⠕ ⠀⡿⣿⣂
00000280| ⠈⠁⠁⠀ ⠈⠉⠈⠀ ⠈⠁⠉⠈ ⠈⠁⠉⠀ ⠀⠉⠉⠁ ⠀⠀⠁⠁ ⠀⠀⠀⠀ ⠀⠀⠀⠀
```

## How to read

Each byte is inidividually printed horizontally, such as each "block" of 4 braille characters represents 4 different bytes, each 8 bytes distant from another.

Each dot on the braille characters represent a 1 being set, and each missing dot is a 0 in binary encoding.

As such, the following chain:

```
⠘⠒⠁⠂ ⠘⠃⠚⠈ ⠘⠃⠙⠀ ⠈⠁⠋⠂ ⠈⠁⠉⠉ ⠀⠁⠀⠀ ⠈⠈⠈⠉ ⠈⠁⠉⠉
```

Actually represents
0b01001000 0b01100101 0b01101100 0b01101100 0b01101111 0b00100000 0b01010111 0b01101111 0b01110010 0b01101100 0b01100100
```
