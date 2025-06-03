## PRELIMINARY Project 1 feedback

## COMMENTS
```
gen-hashes: doesn't work without the `--algorithm` option (no points taken off for this),
            scrypt not supported.
hashes-dump files generated output and do not match test files for scrypt.
```

## GRADING

* CREDITS.md Updated (-1000 if missing) ✅ 

* HONESTY.md Included (-1000 if missing) ✅ 

* README.md Included (-1000 if missing) ✅ 

* cargo fmt Clean (-1000 if changes) ✅ 

* Program Compiles (-1000 if incorrect binary/library name) ✅ 
___
* Program Compiles (25 points) : 25
___
* gen-passwords --chars (5 points) : 5
* gen-passwords --out-file (5 points) : 5
* gen-passwords --threads (5 points) : 5
* gen-passwords --num (5 points) : 5
___
* gen-hashes --in-file (5 points) : 5
* gen-hashes --out-file (5 points) : 5
* gen-hashes --threads (5 points) : 5
* gen-hashes --algorithm (5 points) : 5
* Support for md5, sha256, sha3 512, scrypt (5 points) : 4
___
* dump-hashes Functionality (10 points) : 9

* Comprehensive Documentation (2.5 points) : 2

* No warnings from cargo check (5 points) : 5

* No warnings from cargo clippy (5 points) : 5

* Generic Hashing Algorithm Support (10 points) : 

* Additional Hashing Algorithm (0.25 points each) : 1

* Proper Error Handling (2.5 points) : 2.5

* No unwraps or expects (5 points) : 5

* Proper Logging (2.5 points) : 1
___
* Cool Factor (Unlimited points) : 



### Total

?/112.75
