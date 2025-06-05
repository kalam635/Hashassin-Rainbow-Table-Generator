# 🔧 Performance Benchmark

The following benchmark measures the execution time of the `gen-rainbow-table` command using the optimized release binary.


## Commands Executed
```bash
cargo run --bin hashassin gen-passwords --chars 8 --num 5 --out-file passwords.txt
Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 3
Milliseconds      : 773
Ticks             : 37730440
TotalDays         : 4.36694907407407E-05
TotalHours        : 0.00104806777777778
TotalMinutes      : 0.0628840666666667
TotalSeconds      : 3.773044
TotalMilliseconds : 3773.044


```bash
cargo run --bin hashassin gen-hashes --in-file passwords.txt --out-file hashes.bin --algorithm sha256
Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 0
Milliseconds      : 202
Ticks             : 2029805
TotalDays         : 2.34931134259259E-06
TotalHours        : 5.63834722222222E-05
TotalMinutes      : 0.00338300833333333
TotalSeconds      : 0.2029805
TotalMilliseconds : 202.9805

```bash
cargo run --bin hashassin dump-hashes --in-file hashes.bin

Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 0
Milliseconds      : 211
Ticks             : 2116087
TotalDays         : 2.44917476851852E-06
TotalHours        : 5.87801944444444E-05
TotalMinutes      : 0.00352681166666667
TotalSeconds      : 0.2116087
TotalMilliseconds : 211.6087

```bash
cargo run --bin hashassin gen-rainbow-table --in-file passwords.txt --out-file rainbow.bin --threads 4 --num-links 1000 --algorithm sha256

Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 0
Milliseconds      : 174
Ticks             : 1743994
TotalDays         : 2.01851157407407E-06
TotalHours        : 4.84442777777778E-05
TotalMinutes      : 0.00290665666666667
TotalSeconds      : 0.1743994
TotalMilliseconds : 174.3994

```bash
cargo run --bin hashassin dump-rainbow-table --in-file rainbow.bin


Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 0
Milliseconds      : 153
Ticks             : 1536574
TotalDays         : 1.77844212962963E-06
TotalHours        : 4.26826111111111E-05
TotalMinutes      : 0.00256095666666667
TotalSeconds      : 0.1536574
TotalMilliseconds : 153.6574


```bash
cargo run --bin hashassin crack --in-file rainbow.bin --hashes hashes.bin --threads 4

Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 9
Milliseconds      : 917
Ticks             : 99177841
TotalDays         : 0.000114789167824074
TotalHours        : 0.00275494002777778
TotalMinutes      : 0.165296401666667
TotalSeconds      : 9.9177841
TotalMilliseconds : 9917.7841

```bash
cargo run --bin hashassin crack --in-file rainbow.bin --hashes hashes.bin --out-file cracked.txt --threads 4


Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 8
Milliseconds      : 70
Ticks             : 80709289
TotalDays         : 9.34135289351852E-05
TotalHours        : 0.00224192469444444
TotalMinutes      : 0.134515481666667
TotalSeconds      : 8.0709289
TotalMilliseconds : 8070.9289

