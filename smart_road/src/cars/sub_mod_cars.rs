use crate::{cars::Car, matrix::{COLUMN, ROW}};
use std::time::{Duration, Instant};

use super::Destinations;

pub struct Cars<'a> {
    pub cars: Vec<Car<'a>>,
    pub collisions: i16,
    pub cars_passed: i16,
    pub max_time: Duration,
    pub min_time: Duration,
    pub close_calls: i16,
    pub min_velocity: f64,
    pub max_velocity: f64,
    pub cars_who_have_collisions: Vec<(i32, i32)>,
    pub close_call_cars: Vec<(i32, i32)>,
}

impl<'a> Cars<'a> {
    pub fn new() -> Self {
        Cars {
            cars: Vec::new(),
            collisions: 0,
            cars_passed: 0,
            close_calls: 0,
            max_time: Duration::new(0, 0),
            min_time: Duration::new(100, 0),
            min_velocity: 0.0,
            max_velocity: 0.0,
            cars_who_have_collisions: Vec::new(),
            close_call_cars: Vec::new(),
        }
    }

    pub fn refresh(&mut self) {
        *self = Cars::new()
    }

    pub fn handle_collisions(&mut self) {
        let mut speeds = Vec::new();

        for (car_index, car) in self.cars.iter().enumerate() {
            let mut level_speed = 2; // Vitesse par défaut



            if car.path.contains(&(0*car.size as i32, 8*car.size as i32))||
            car.path.contains(&(ROW*car.size as i32, 13*car.size as i32))||
            car.path.contains(&(13*car.size as i32, 0*car.size as i32))||
            car.path.contains(&(8*car.size as i32, COLUMN * car.size as i32)){
                speeds.push(3); // Enregistre la vitesse calculée pour cette voiture
                continue;
            }

            for (other_car_index, other_car) in self.cars.iter().enumerate() {
                if collision_with_other_car(car, other_car) && !self.cars_who_have_collisions.contains(&(other_car.name, car.name))
                && !self.cars_who_have_collisions.contains(&(car.name, other_car.name)){
                    self.cars_who_have_collisions.push((car.name, other_car.name));
                    self.collisions+=1;
                }

                if close_call_detect(car, other_car) && !self.cars_who_have_collisions.contains(&(car.name, other_car.name)) && 
                !self.close_call_cars.contains(&(car.name, other_car.name)){
                    self.close_call_cars.push((car.name, other_car.name));
                    self.close_call_cars.push((other_car.name, car.name));
                    self.close_calls+=1;
                }
                if collisions_prevent_with_directions(car, other_car)
                    || car_need_to_stop_now(car, other_car, car_index, other_car_index)
                {
                    level_speed = 0;
                    break;
                }

                if in_matrice(car) && collision_extension(car, other_car){
                    level_speed = 1
                }
            }


            speeds.push(level_speed); // Enregistre la vitesse calculée pour cette voiture
        }

        // Applique les vitesses calculées à chaque voiture
        for (car, &speed) in self.cars.iter_mut().zip(speeds.iter()) {
            car.level_speed = speed;

        }
    }

    pub fn update_cars(&mut self) {
        for car in self.cars.iter_mut() {
            if car.level_speed > 0 {
                car.update_position();
            }
            let now = Instant::now();
            car.timer += now.duration_since(car.last_update);
            car.last_update = now;
        }
    }

    pub fn retain(&mut self, heigth: i32, width: i32) {
        self.update_timer_and_velocity(width, heigth);

        let before = self.cars.len();
        self.cars.retain(|car| {
            car.column >= 0 && car.column <= width && car.row >= 0 && car.row <= heigth
        });
        let after = self.cars.len();

        if before > after {
            self.cars_passed += (before - after) as i16;
        }
    }

    fn update_timer_and_velocity(&mut self, width: i32, heigth: i32) {
        let mut cars_to_remove = Vec::new();
        for car in &self.cars {
            if car.column < 0 || car.column > width || car.row < 0 || car.row > heigth {
                cars_to_remove.push(car);
            }
        }

        for car in cars_to_remove {
            if car.timer > self.max_time {
                let rounded_seconds = round_to_three_decimal_places(car.timer.as_secs_f64());
                self.max_time = Duration::from_secs_f64(rounded_seconds);
            }

            if car.timer < self.min_time || self.min_time == Duration::new(u64::MAX, 999_999_999) {
                let rounded_seconds = round_to_three_decimal_places(car.timer.as_secs_f64());
                self.min_time = Duration::from_secs_f64(rounded_seconds);
            }

            // Calcul de la vitesse
            let path_len = car.path.len() as f64; // Assurez-vous que la longueur du chemin est en f64
            let size = car.size as f64; // Assurez-vous que la taille est en f64
            let timer_secs = car.timer.as_secs_f64(); // Temps en secondes

            let velocity = if timer_secs > 0.0 {
                (path_len * size) / timer_secs
            } else {
                0.0
            };

            // Mettre à jour la vitesse minimale et maximale
            if (velocity < self.min_velocity && velocity > 0.0) || self.min_velocity == 0.0 {
                self.min_velocity = round_to_three_decimal_places(velocity);
            }
            if velocity > self.max_velocity {
                self.max_velocity = round_to_three_decimal_places(velocity);
            }
        }
    }

}

//Renvoie true si la prochaine case est occupé;
fn next_position_occupied(car: &Car, other_car: &Car) -> bool {
    if let Some(next_pos) = car.path.get(car.index_path + 1) {
        return *next_pos == other_car.position;
    }
    false
}

//Renvoie true si la prochaine case est aussi désiré par une autre voiture
fn position_can_be_conflictual(car: &Car, other_car: &Car) -> bool {
    if let (Some(car_next_pos), Some(other_car_next_pos)) = (
        car.path.get(car.index_path + 1),
        other_car.path.get(other_car.index_path + 1),
    ) {
        return *car_next_pos == *other_car_next_pos;
    }
    false
}

//Méthod by Fred
fn collisions_prevent_with_directions(car: &Car, other_car: &Car) -> bool {
    // Vérifie les collisions potentielles pour les directions Est et Ouest
    if (car.destination == Destinations::East && other_car.column > car.column)
        || (car.destination == Destinations::West && other_car.column < car.column)
        // !(car.destination == Destinations::West && other_car.destination == Destinations::East) &&
        // !(car.destination == Destinations::East && other_car.destination == Destinations::West)
    {
        let row_diff = (other_car.row as i32).abs_diff(car.row as i32);
        let column_diff = (other_car.column as i32 - car.column as i32).abs();

        if row_diff < car.size && column_diff < car.collision_extension_midlle {
            return true;
        }
    }

    // Vérifie les collisions potentielles pour les directions Nord et Sud
    if (car.destination == Destinations::North && other_car.row < car.row)
        || (car.destination == Destinations::South && other_car.row > car.row)
    {
        let column_diff = (other_car.column as i32).abs_diff(car.column as i32);
        let row_diff = (other_car.row as i32 - car.row as i32).abs();

        if column_diff < car.size && row_diff < car.collision_extension_midlle {
    
            return true;
        }
    }

    false
}

fn collision_with_other_car(car: &Car, other_car: &Car) -> bool {
    // Calcul de la différence de lignes (row) et de colonnes (column)

    let row_diff = (other_car.row).abs_diff(car.row);
    let column_diff = (other_car.column).abs_diff(car.column);

    // Vérifie si les différences de ligne et de colonne sont inférieures à la taille de la voiture
    row_diff < car.size && column_diff < car.size && row_diff != 0 && column_diff != 0
    
}

fn close_call_detect(car: &Car, other_car: &Car)->bool{
    if (car.destination == Destinations::East && other_car.column > car.column)
        || (car.destination == Destinations::West && other_car.column < car.column)
    {
        let row_diff = (other_car.row as i32).abs_diff(car.row as i32);
        let column_diff = (other_car.column as i32 - car.column as i32).abs();

        if row_diff < car.size && column_diff < car.collision_extension_low {
            return true;
        }
    }

    // Vérifie les collisions potentielles pour les directions Nord et Sud
    if (car.destination == Destinations::North && other_car.row < car.row)
        || (car.destination == Destinations::South && other_car.row > car.row)
    {
        let column_diff = (other_car.column as i32).abs_diff(car.column as i32);
        let row_diff = (other_car.row as i32 - car.row as i32).abs();

        if column_diff < car.size && row_diff < car.collision_extension_low {

            return true;
        }
    }
    false
}


fn collision_extension(car: &Car, other_car: &Car) -> bool {
    for i in 2..=3 {
        if let (Some(car_next_pos), Some(other_car_next_pos)) = (
            car.path.get(car.index_path + i),
            other_car.path.get(other_car.index_path + 1),
        ) {
            if *car_next_pos == *other_car_next_pos || *car_next_pos == other_car.position {
                return true;
            }
        }
    }
    false
}


fn car_need_to_stop_now(
    car: &Car,
    other_car: &Car,
    car_index: usize,
    other_car_index: usize,
) -> bool {
    let position_occupied = next_position_occupied(car, other_car);

    let position_conflict = position_can_be_conflictual(car, other_car);

    if (position_occupied || position_conflict)
        && car_index < other_car_index
        && other_car.level_speed != 0
    {
        return true;
    }
    false
}

fn round_to_three_decimal_places(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

fn in_matrice(car: &Car)-> bool{
    if (car.row > 7*car.size as i32 && car.row < 15*car.size as i32) && (car.column > 7 * car.size as i32 && car.column < 15 * car.size as i32){
        return true;
    }
    false
}
