fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2); // 1 + 2 = 3

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2); // 1 - 2 = -1
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3); // 1e4 is 10000, -2.5e-3 is -0.0025

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);
    /*
     * true AND false is false
     * true OR false is true
     * NOT true is false
     */ 


    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    /*
     * 0011 AND 0101 is 0001
     * 0011 OR 0101 is 0111
     * 0011 XOR 0101 is 0110
     * 1 << 5 is 32
     * 0x80 >> 2 is 0x20
     */ 

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32); // One million is written as 1000000
}