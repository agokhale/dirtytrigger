# dirtytrigger
watch binary streams for events and triggertih on them
1735049265 - Tue Dec 24 09:07:45 EST 2024  need tool to trigger a thing  based on an input byte stream, midi -> xdotool ? rust?

why hasn't anyone written this?

https://learn.sparkfun.com/tutorials/midi-tutorial/all

pehaps a little AWK, a little sed but binary search spaces.

yaml but sed input?



cat /dev/umidi0.0 > c123.mid

# hd c123.mid                                                                                                                                                                        :xpi:~/hacks/akaimidi:9:13:33:116inara97:46.1
00000000  90 30 2e 80 30 00 90 3c  2a 80 3c 00 90 48 33 80  |.0..0..<*.<..H3.|
00000010  48 00                                             |H.|
00000012

midi 
- keydown: 0x90, 
-  0x30 - 

/0x90/ {system("xdotool F1"):}
