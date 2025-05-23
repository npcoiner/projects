pub fn multiply_2x2(a: [[f32; 2]; 2], b: [[f32; 2]; 2]) -> [[f32; 2]; 2] {
    let mut result = [[0.0; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                result[i][j] = a[i][k] * b[k][j];
            }
        }
    }
    result
}
