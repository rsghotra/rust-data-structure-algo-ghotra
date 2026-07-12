fn main() {
    

    /*
        creating
     */
    let static_str = "ABCD";

    let empty = String::new();

    let from_lit = String::from("BTC/USD");

    let via_to_string = "to".to_string();
    let cap_string = String::with_capacity(100);

    let ch = 'a';
    let concocted_string = format!("{} - {}", from_lit, via_to_string);

    println!("{concocted_string}");

    
    let s = "BTC/USD";

    println!("String: {s}");
    println!("Length: {}", s.len());

    /*
        Accessing:

        - s.chars().nth(i) = Safe nth char access => Option<char> - O(n)
        - s.chars().next() = Safe first element access => Optiin<char> - O(1) ish, the first character
        - s.get(a..b) = safe slcie access => Option<&str>
    
     */
    
    //indexed access
    if let Some(first) = s.chars().nth(7) {
        println!("first: {}", first);
    }

    //first element access access
    if let Some(first) = s.chars().next() {
        println!("first: {}", first);
    }

    //safe range slice - None instead of panic on bad boundaries

    match s.get(2..4) {
        Some(slice_str) => println!("Printing slice: {}", slice_str),
        None => println!("Out of bound access found"),
    }

    /*
        Adding/Building/Concatenating
        s.push()  - append a single char
        s.push_str() - appena &str
        s += &str - operator form of push_str
        s1 + &s2 => consumes s1
        format! => create a new string but does not consume either input
        .repeat(n) - repeats a &str/String n times
     */

    let mut s = String::from("   BTC  ");
    s.push('/');
    s.push_str("USD");
    s += "-SPOT";

    let combined = format!("{s}!"); //dpes not consume s

    println!("s is {}", s);
    println!("combined is {}", combined);

    let mut s1 = s + static_str;

    //println!("again s is {}", s);

    println!("S1 is {}", s1);


    let rep = "ab".repeat(3);

    println!("Rep is {}", rep);

    /*
        Removing

        - s.pop() => Option<char> = let c = c.pop() =>return the LAST character not byte
        - s.remove(i) => not safe and will panic is its not a char boundry
        - s.truncate(n) => leep only first n elements and will panic is not a clear char boundry
        - s.retain(pred) => keeps only closure meeting characters
        - s.trim() => &str -> does not mutate the original one and return a new borrowed slice with whitespace trimmed

    
     */

    println!("The last popped element is: {:?}", s1.pop());

    let mut pair = String::from("BTC/USD - 123");

    pair.retain(|c| c.is_alphabetic());

    println!("{pair}");

    let trimmed = s1.trim();

    println!("{trimmed}");

    /*
        updating:

        s.insert(i, c) => insert a charater at byte index - can panic
        s.insert_str(i, &str) => insert string at byte indec - can panic

     */

    let mut s = String::from("BTC/USD");

    s.insert(0, 'A');

    s.insert_str(0, "Pair: ");

    let replaced = s.replace("USD", "SOL");

    println!("{replaced}");

    let lower = s.chars().map(|c| c.to_ascii_lowercase()).collect::<String>();

    println!("{lower}");

    /*
        searching
            - contains()
            - starts_with()
            - ends_with()
            - find() -> returns byte index of the first match

     */


     println!("\n--- Exercise 11 ---");

     let s = "BTC/USD";

     println!("Contains: {}", s.contains("BTC"));

     let s1 = "A🦀B";

     for (index, c) in s1.char_indices() {
        println!("{index} {c}");
     }

     if let Some(index) = s.find('B') {
        println!("{index}");
     }

     println!("Pair starts with: {}", s.starts_with("BT"));

    println!("Pair starts with: {}", s.starts_with("US"));

    /*
    
     Sorting

     - Sorting Charaters is 3 step process: 1. Convert char string to Vec<Char> => Sort the vector => then recreate the character string
     - Sorting array of String is simple as we are doing lexicographically
     */

    let s = "adcb";

    let mut chars = s.chars().collect::<Vec<char>>();
    chars.sort();

    let sorted = chars.into_iter().collect::<String>();

    println!("{sorted}");

    let mut words = vec!["banana", "Apple", "Black Plum"];

    words.sort();

    println!("{:?}", words);


    /*
     Iteration:

     s.chars() -> char
     s.bytes() -> u8
     s.char_indices() -> (index, char)
     s.split(patt) -> str
     s.lines() -> &str
     
     */

    //iterate over string and print each cahar

    let pair = "SOL/USD";

    for c in pair.chars() {
        println!("{c}");
    }

    for (index, char) in pair.char_indices() {
        println!("{}, {}", index, char);
    }

    for c in pair.bytes() {
        println!("{c}");
    }

    let text = "A🦀B";

    println!("Char count: {}", text.chars().count());

    let message = "35=D|55=BTCUSD|44=100000";

    for field in message.split('|') {
        if let Some((tag, value)) = field.split_once("=") {
            println!("Tag: {tag}, Value: {value}");
        }
    }

    let multi_line = "hello
    to your
    son
    god bless you
    Laser
    Lhava
    XO
    Astra
    Gattaca
    QRT";

    for line in multi_line.lines() {
        println!("Line found : {line}");
    }

    let message = "35=D|55=BTCUSD|44=100000";
    
    for field in message.split('|') {
        if let Some(tag) = field.strip_prefix("55=") {
            println!("{tag}");
            if let Some(suffix) = field.strip_suffix("USD") {
                println!("{suffix}");
            }
        }
    }

    let owned = String::from("GBP/USD");

    //borrow it as &str wihout allocating

    let borrowed = owned.as_str();

    println!("Owned    : {owned}");
    println!("Borrowed : {borrowed}");

    let owned = borrowed.to_string();

    println!("Owned    : {owned}");
    println!("Borrowed : {borrowed}");

    let text = String::from("dbca");

    let mut chars  = text.chars().collect::<Vec<char>>();

    chars.sort();

    let sorted = chars.into_iter().collect::<String>();
    println!("Original : {text}");
    println!("Sorted   : {sorted}");


     let bytes = vec![66, 84, 67, 47, 85, 83, 68];

     match String::from_utf8(bytes) {
        Ok(text) => println!("Valid UTF-8: {text}"),
        Err(error) => println!("Invalid_UTF8: {:?}", error),
     }

}
