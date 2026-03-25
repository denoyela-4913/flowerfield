pub fn annotate(garden: &[&str]) -> Vec<String> {
    // todo!(
    //    "\nAnnotate each square of the given garden with the number of flowers that surround said square (blank if there are no surrounding flowers):\n{garden:#?}\n"
    for i in 0 .. garden.len() { 
        println! ("{}", garden[i])
    };
    
    let mut copyvec : Vec::<String> = Vec::new();
    let mut cur : String = String::new();
    let mut prev : String; // Do not initialize next and prev
    let mut next : String; // Do not initialize next and prev

    for i in 0 .. garden.len() {  
        prev = cur.clone();
        cur = String::from(garden[i]);
        if i + 1 < garden.len() { next = String::from(garden[i+1]); } else { next = String::new(); };
        // cur = String::from("7");
        // let mut cur_pos = 0;

        let mut nb_neighbor;

        for cur_pos in 0 .. cur.len() {
            nb_neighbor = 0;
            if cur.chars().nth(cur_pos) == Some('*') {
                // Handle flower logic here
                // No change on cur_pos when a flower is found
                //copyvec.push(String::from(cur.clone()));
                println! ("star found at row {}, cur_pos = {}", i, cur_pos);

            } else if cur.chars().nth(cur_pos) == Some(' ') {
                if cur_pos > 0 {
                    if cur.chars().nth(cur_pos-1) == Some('*') {nb_neighbor = nb_neighbor+1;}
                    if ! prev.is_empty() && prev.chars().nth(cur_pos-1) == Some('*') {nb_neighbor = nb_neighbor+1;}
                    if ! next.is_empty() && next.chars().nth(cur_pos-1) == Some('*') {nb_neighbor = nb_neighbor+1;}
                }
                if cur_pos < cur.len()-1 {  
                    if cur.chars().nth(cur_pos+1) == Some('*') {nb_neighbor = nb_neighbor+1;}
                    if ! prev.is_empty() && prev.chars().nth(cur_pos+1) == Some('*') {nb_neighbor = nb_neighbor+1;}
                    if ! next.is_empty() && next.chars().nth(cur_pos+1) == Some('*') {nb_neighbor = nb_neighbor+1;}
                }
                if ! prev.is_empty() && prev.chars().nth(cur_pos) == Some('*') {nb_neighbor = nb_neighbor+1;}
                if ! next.is_empty() && next.chars().nth(cur_pos) == Some('*') {nb_neighbor = nb_neighbor+1;}
                if nb_neighbor == 0 {
                    // copyvec.push(String::from(cur.clone()));
                    println! ("no neighbor found at row {}, cur_pos = {}", i, cur_pos);
                }
                else {
                    println! ("row = {} | cur_pos = {} | nb_neighbor = {}", i, cur_pos, nb_neighbor);
                    // cur.chars().nth(cur_pos) = String::from("A");
                    // cur = String::from("A");
                    let begin : String = cur[0..cur_pos].into();
                    let end : String = cur[cur_pos+1..].into();
                    cur = format!("{}{}{}", begin, nb_neighbor, end);
                }
            }
        } // for loop on cur_pos
        copyvec.push(String::from(cur.clone()));
        println! ("prev = {}|cur = {}|next = {}|", prev, cur, next);
        // copyvec.push(String::from(garden[i]));
    }; // for loop on garden[i]
    // let mut a : Vec::<String> = Vec::new();
    // a.push(String::from(" 2*2 "));
    // a.push(String::from("25*52"));
    // a.push(String::from("*****"));
    // a.push(String::from("25*52"));
    // a.push(String::from(" 2*2 "));
    return copyvec;
} // fn annotate

fn main() {
    println!("Hello, world! flower field");
    #[rustfmt::skip]
    let (input, expected) = (&[
    "  *  ",
    "  *  ",
    "*****",
    "  *  ",
    "  *  ",
    ], &[
    " 2*2 ",
    "25*52",
    "*****",
    "25*52",
    " 2*2 ",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}
