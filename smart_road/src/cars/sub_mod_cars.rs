use crate::cars::Car;
use std::time::{Instant, Duration};

pub struct Cars<'a> {
    pub cars: Vec<Car<'a>>,
    pub collisions: i16,
    pub cars_passed: i16,
    pub max_time: Duration,
    pub min_time: Duration,
    pub close_calls: i16,
    pub min_velocity: f64,
    pub max_velocity: f64
}

impl <'a>Cars<'a> {
    pub fn new() -> Self {
        Cars {
            cars: Vec::new(),
            collisions: 0,
            cars_passed: 0,
            close_calls:0,
            max_time: Duration::new(0, 0),
            min_time: Duration::new(100, 0),
            min_velocity: 0.0,
            max_velocity: 0.0,
        }
    }

    pub fn refresh(&mut self) {
        self.cars.clear();
        self.cars_passed = 0;
        self.close_calls = 0;
        self.collisions = 0;
        self.max_time = Duration::new(0, 0); 
        self.min_time = Duration::new(100, 0);
        self.max_velocity = 0.0;
        self.min_velocity = 0.0;
    }

    pub fn handle_collisions(&mut self) {
        let collisions = detect_collisions(&mut self.cars); // Appelle la méthode pour détecter les collisions
        let mut slow_down_cars = std::collections::HashSet::new();
        let mut stop_cars = std::collections::HashSet::new();
    
        for (i, j, zone) in collisions {
            match zone {
                "middle" => {
                    slow_down_cars.insert(i);
                    slow_down_cars.insert(j);
                }
                "low" => {
                    stop_cars.insert(i);
                    stop_cars.insert(j);
                }
                _ => {}
            }
        }
    
        // Réaction aux collisions dans la zone middle (ralentir)
        for car_index in slow_down_cars {
            if let Some(car) = self.cars.get_mut(car_index) {
                car.level_speed = 3; // Ralentir la voiture
            }
        }
    
        // Réaction aux collisions dans la zone low (arrêter)
        for car_index in stop_cars {
            if let Some(car) = self.cars.get_mut(car_index) {
                car.level_speed = 1; // Arrêter la voiture
            }
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

    pub fn retain(&mut self, heigth: i32, width: i32 ){
        
        self.update_timer_and_velocity(width, heigth);

        let before = self.cars.len();
        self.cars.retain(|car| {
            car.column >= 0
                && car.column <= width
                && car.row >= 0
                && car.row <= heigth
        });
        let after = self.cars.len();

        if before > after {
            self.cars_passed+= (before-after) as i16;
        }
    }

    fn update_timer_and_velocity(&mut self, width: i32, heigth: i32){
       
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


pub fn detect_collisions(cars: &mut [Car]) -> Vec<(usize, usize, &'static str)> {
    let mut collisions = Vec::new();
    
    for i in 0..cars.len() {
        for j in i + 1..cars.len() {
            let car_a = &cars[i];
            let car_b = &cars[j];
            
            // Obtenez les deux paires de rectangles de collision
            let (rect_a_middle, rect_a_low) = expand_collision_rect(car_a);
            let (rect_b_middle, rect_b_low) = expand_collision_rect(car_b);
            
            // Vérifiez les chevauchements pour la zone middle
            if rectangles_overlap(rect_a_middle, rect_b_middle) {
                collisions.push((i, j, "middle"));
            }
            // Vérifiez les chevauchements pour la zone low
            else if rectangles_overlap(rect_a_low, rect_b_low) {
                collisions.push((i, j, "low"));
            }
        }
    }
    if collisions.len()>0{
        println!("collisions: {:?}", collisions);
    }
    collisions
}

fn round_to_three_decimal_places(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

fn expand_collision_rect(car: &Car) -> ((i32, i32, i32, i32), (i32, i32, i32, i32)) {
    let radians = car.destination.to_radians();
    let dmx = (radians.cos() * car.collision_extension_midlle as f32) as i32;
    let dmy = (radians.sin() * car.collision_extension_midlle as f32) as i32;
    let dlx = (radians.cos() * car.collision_extension_low as f32) as i32;
    let dly = (radians.sin() * car.collision_extension_low as f32) as i32;

    // Calculer l'extension de collision en fonction de la direction
    let extension_midlle_x = dmx;
    let extension_midlle_y = dmy;
    let extension_low_x = dlx;
    let extension_low_y = dly;
    
    ((
        car.column - (car.size as i32 * 2) + extension_midlle_x,
        car.row - (car.size as i32 * 2) + extension_midlle_y,
        car.size as i32 + car.collision_extension_midlle as i32,
        car.size as i32 + car.collision_extension_midlle as i32,
    ),
    (
        car.column - (car.size as i32 * 2) - extension_low_x,
        car.row - (car.size as i32 * 2) - extension_low_y,
        car.size as i32 + car.collision_extension_low as i32,
        car.size as i32 + car.collision_extension_low as i32,
    ))
}

fn rectangles_overlap(rect1: (i32, i32, i32, i32), rect2: (i32, i32, i32, i32)) -> bool {
    rect1.0 < rect2.0 + rect2.2
        && rect1.0 + rect1.2 > rect2.0
        && rect1.1 < rect2.1 + rect2.3
        && rect1.1 + rect1.3 > rect2.1
}