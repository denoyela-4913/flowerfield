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

        let mut nb_neighbor;

        // Handle flower logic here
        for cur_pos in 0 .. cur.len() {
            nb_neighbor = 0;
            if cur.chars().nth(cur_pos) == Some('*') {
                
                // No change on cur_pos when a flower is found
                println! ("star found at row {}, cur_pos = {}", i, cur_pos);

            } else if cur.chars().nth(cur_pos) == Some(' ') {
                if cur_pos > 0 {
                    if cur.chars().nth(cur_pos-1) == Some('*') {nb_neighbor += 1;}
                    if ! prev.is_empty() && prev.chars().nth(cur_pos-1) == Some('*') {nb_neighbor += 1;}
                    if ! next.is_empty() && next.chars().nth(cur_pos-1) == Some('*') {nb_neighbor += 1;}
                }
                if cur_pos < cur.len()-1 {  
                    if cur.chars().nth(cur_pos+1) == Some('*') {nb_neighbor += 1;}
                    if ! prev.is_empty() && prev.chars().nth(cur_pos+1) == Some('*') {nb_neighbor += 1;}
                    if ! next.is_empty() && next.chars().nth(cur_pos+1) == Some('*') {nb_neighbor += 1;}
                }
                if ! prev.is_empty() && prev.chars().nth(cur_pos) == Some('*') {nb_neighbor += 1;}
                if ! next.is_empty() && next.chars().nth(cur_pos) == Some('*') {nb_neighbor += 1;}
                if nb_neighbor == 0 {
                    
                    println! ("no neighbor found at row {}, cur_pos = {}", i, cur_pos);
                }
                else {
                    println! ("row = {} | cur_pos = {} | nb_neighbor = {}", i, cur_pos, nb_neighbor);
                    
                    let begin : String = cur[0..cur_pos].into();
                    let end : String = cur[cur_pos+1..].into();
                    cur = format!("{}{}{}", begin, nb_neighbor, end);
                }
            }
        } // for loop on cur_pos

        copyvec.push(cur.clone());
        println! ("prev = {}|cur = {}|next = {}|", prev, cur, next);
        
    }; // for loop on garden[i]
    
    copyvec
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
