use crate::matrix::{COLUMN, ROW};

//Toutes ces fn servent à créer le car.path de la voiture et permettent de savoir le chemin de celle-ci

pub fn south_destinations(row: i32, column: i32, cell_size: u32) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = Vec::new();

    result.push((row, column));
    let mut new_row: i32=row/cell_size as i32;
    let mut new_column: i32 = column/cell_size as i32; 
    
    if row == 10*cell_size as i32 && column == COLUMN*cell_size as i32{
        while new_column > 10 {
            new_column-= 1;
            result.push((row, new_column*cell_size as i32));
        }
    }

    if row == 13* cell_size as i32 && column == 0{ 
        while new_column < 8 {
            new_column+= 1;
            result.push((row, new_column*cell_size as i32));
        }
    }

    while new_row < 23 {
        new_row+= 1;
        result.push((new_row*cell_size as i32, new_column*cell_size as i32));
    }
    
    result 
}

pub fn north_destinations(row: i32, column: i32, cell_size: u32) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = Vec::new();

    result.push((row, column));
    let mut new_column: i32 = column/cell_size as i32; 
    let mut new_row = row/cell_size as i32;

    if row == 8*cell_size as i32 && column == COLUMN*cell_size as i32{
        while new_column > 13 {
            new_column-= 1;
            result.push((row, new_column*cell_size as i32));
        }
    }

    if 11*cell_size as i32 == row && column == 0 as i32{
        while new_column < 11 {
            new_column+= 1;
            result.push((row, new_column*cell_size as i32));
        }
    }

    while new_row > -1 {
        new_row -= 1;
        result.push((new_row * cell_size as i32, new_column*cell_size as i32));
    }

    result
}

pub fn east_destination(row: i32, column: i32, cell_size: u32) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = Vec::new();

    result.push((row, column));
    let mut new_row = row/cell_size as i32;
    let mut new_column = column/cell_size as i32;

    if row == 0 && column == 10*cell_size as i32 {
        while new_row < 11 {
            new_row += 1;
            result.push((new_row* cell_size as i32, column));
        }
    }

    if row == ROW*cell_size as i32 && column == 13 * cell_size as i32{
        while new_row >13{
            new_row -= 1;
            result.push((new_row* cell_size as i32, column));
        }
    }

    while new_column < 23 {
        new_column += 1;
        result.push((new_row * cell_size as i32, new_column * cell_size as i32));
    }

    result

}

pub fn west_destination(row: i32, column: i32, cell_size: u32) -> Vec<(i32, i32)> {

    let mut result: Vec<(i32, i32)> = Vec::new();

    result.push((row, column));
    let mut new_row = row/cell_size as i32;
    let mut new_column = column/cell_size as i32;  

    if row == 0 && column == 8*cell_size as i32{
        while new_row < 8 {
            new_row += 1;
            result.push((new_row * cell_size as i32, column));
        }
    }

    if row == ROW*cell_size as i32 && column == 11*cell_size as i32 {
        while new_row > 10 {
            new_row -= 1;
            result.push((new_row * cell_size as i32, column));
        }
    }

    while new_column > -1 {
        new_column -= 1;
        result.push((new_row*cell_size as i32, new_column * cell_size as i32));
    }

    result
}