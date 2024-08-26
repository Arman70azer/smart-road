pub fn matrice(length: u32, width: u32) -> Vec<Vec<u32>> {
    let mut matrice_vector: Vec<Vec<u32>> = Vec::new();
    
    for _i in 0..length {
        let mut ligne_with_squares: Vec<u32> = Vec::new();
        
        for j in 0..width {
            ligne_with_squares.push(j);
        }
        
        matrice_vector.push(ligne_with_squares);
    }
    
    matrice_vector
}
