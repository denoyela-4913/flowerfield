pub fn annotate(garden: &[&str]) -> Vec<String> {
    // todo!(
    //    "\nAnnotate each square of the given garden with the number of flowers that surround said square (blank if there are no surrounding flowers):\n{garden:#?}\n"
    
    // DEBUG : print the input garden
    /* for i in 0 .. garden.len() { 
        println! ("{}", garden[i])
    }; */
    
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

                // DEBUG : print row and cur_pos when a flower is found
                // println! ("star found at row {}, cur_pos = {}", i, cur_pos);

                continue;

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

                    // No neighbor found, but we should not replace the char in cur string with '0', 
                    // we should keep it as ' ' (blank)

                    // DEBUG : print row and cur_pos when no neighbor is found
                    // println! ("No neighbor found at row {}, cur_pos = {}", i, cur_pos);

                    continue;
                }
                else {
                    // DEBUG: print row, cur_pos and the number of neighbor found
                    // println! ("row = {} | cur_pos = {} | nb_neighbor = {}", i, cur_pos, nb_neighbor);
                    
                    // Neighbor found, we should replace the char in cur string with the number of neighbor found
                    // Choosen solution = More elegant, replacing only 1 char in cur string + use '&' instead of 'as_str()'
                    cur.replace_range(cur_pos..cur_pos+1, &nb_neighbor.to_string());
                } // if nb_neighbor == 0
            } // if cur.chars().nth(cur_pos) == Some('*') - else if cur.chars().nth(cur_pos) == Some(' ')
        } // for loop on cur_pos

        copyvec.push(cur.clone());

        // DEBUG: Print the current state of cur, prev and next strings
        // println! ("prev = {}|cur = {}|next = {}|", prev, cur, next);
        
    }; // for loop on garden[i]
    
    copyvec
} // fn annotate

fn main() {
    println!("Hello, world! flower field");
    
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
    // Do 1 test to check algorithm correctness
    assert_eq!(actual, expected);
}
