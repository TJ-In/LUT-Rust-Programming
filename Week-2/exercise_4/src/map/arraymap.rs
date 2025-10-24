pub fn draw_map(pos: (usize, usize)) {
    let mut matrix = [["*"; 5]; 5];
    matrix[pos.0][pos.1] = "x";
    for row in matrix {
        let mut vec = vec![];
        for block in row {
            vec.push(block);
        }
        println!("{}", vec.join(" "));
    }
}
pub fn create_map() -> [[char;5];5] {
    let mut map = [['*';5];5];
    map[2][2] = 'x';
    map
}
