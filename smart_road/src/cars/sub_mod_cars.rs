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
        // Vous pouvez maintenant traiter les collisions détectées ici
        let mut slow_down_cars = std::collections::HashSet::new();
        for (i, j) in collisions {
            slow_down_cars.insert(i);
            slow_down_cars.insert(j);
        }
        for car_index in slow_down_cars {
            if let Some(car) = self.cars.get_mut(car_index) {
                car.level_speed = 0;
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


fn detect_collisions(cars: &mut [Car]) -> Vec<(usize, usize)> {
    let mut collisions = Vec::new();
    
    for i in 0..cars.len() {
        for j in i + 1..cars.len() {
            let car_a = &cars[i];
            let car_b = &cars[j];
            
            // Définir les rectangles de collision
            let rect_a = (
                car_a.column,
                car_a.row,
                car_a.size,
                car_a.size,
            );
            let rect_b = (
                car_b.column,
                car_b.row,
                car_b.size,
                car_b.size,
            );
            
            // Vérifier le chevauchement des rectangles
            if rect_a.0 < rect_b.0 + (rect_b.2 as i32)
                && rect_a.0 + (rect_a.2 as i32) > rect_b.0
                && rect_a.1 < rect_b.1 + (rect_b.3 as i32)
                && rect_a.1 + (rect_a.3 as i32) > rect_b.1
            {
                collisions.push((i, j));
            }
        }
    }
    
    collisions
}

fn round_to_three_decimal_places(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}