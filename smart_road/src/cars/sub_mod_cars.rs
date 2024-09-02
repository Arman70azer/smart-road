use crate::cars::Car;

pub struct Cars<'a> {
    pub cars: Vec<Car<'a>>,
}

impl <'a>Cars<'a> {
    pub fn new() -> Self {
        Cars {
            cars: Vec::new(),
        }
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
        }
    }

    pub fn num_of_accidents(&self)->i16{
        let mut count = 0;
        for car in &self.cars{
            count+= car.choc;
        }
        count/2
    }

    pub fn retain(&mut self, heigth: i32, width: i32 ){
        self.cars.retain(|car| {
            car.column >= 0
                && car.column <= width
                && car.row >= 0
                && car.row <= heigth
        });
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