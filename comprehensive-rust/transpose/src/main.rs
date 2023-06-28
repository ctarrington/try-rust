fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed = [[0; 3]; 3];
    for row_index in 0..matrix.len() {
        for column_index in 0..matrix.len() {
            transposed[row_index][column_index] = matrix[column_index][row_index];
        }
    }

    transposed
}

#[test]
fn test_transpose() {
    let matrix = [
        [1,2,3], //
        [4,5,6],
        [7,8,9]
    ];
    let expected = [
        [1,4,7],
        [2,5,8],
        [3,6,9]
    ];
    assert_eq!(transpose(matrix), expected);
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row_index in 0..matrix.len() {
        let row = matrix[row_index];
        for column_index in 0..row.len() {
            print!(" {}", row[column_index]);
        }
        print!("\n");
    }
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}
