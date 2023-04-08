# RaceNG
Revolutionary, innovative, groundbreaking random number generator using race conditions written in Rust. I wrote this in like an hour because I thought it would be funny (it was). I should not need to tell you this is not a reliable source of RNG you should rely on. If you do end up using it for smthn, please DM me on discord, I want to know. 

How 2 use
-------------
1. `cargo add RaceNG` (Yes I uploaded it as a crate)
2. `let result = RaceNG::race(x, y)`
3. PROFIT

Sample output: \
![image](https://user-images.githubusercontent.com/96934612/230705035-2f49ddad-32e8-4682-bbf1-8fdc86915cb5.png)

Explanation 4 nerds
-------------
A race condition occurs when 2 or more threads are trying to use the same variable.

Both threads try to set the variable to a certain value, but due to computer jank, they go about it at different speeds. This means that the value of the shared variable is undefined at a given time. If you print the variable it is *basically* random. 
