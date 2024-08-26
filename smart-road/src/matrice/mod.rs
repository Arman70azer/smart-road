pub fn matrice(length: u8, width: u8) -> Vec<Vec<u8>> {
    let mut matrice_vector: Vec<Vec<u8>> = Vec::new();
    
    for _ in 0..length {
        let mut ligne_with_squares: Vec<u8> = Vec::new();
        
        for square in 0..width {
            ligne_with_squares.push(square);
        }
        
        matrice_vector.push(ligne_with_squares);
    }
    
    matrice_vector
}
