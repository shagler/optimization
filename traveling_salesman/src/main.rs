use rand::Rng;

const TEMPERATURE: f64 = 10000.0;
const COOLING_RATE: f64 = 0.995;

/// Calculate the distance between two cities
fn calculate_distance(first_city: &[f64; 2], second_city: &[f64; 2]) -> f64 {
    ((first_city[0] - second_city[0]).powi(2) + 
     (first_city[1] - second_city[1]).powi(2)).sqrt()
}

/// Calculate the total distance of a given route
fn total_distance(route: &Vec<usize>, cities: &Vec<[f64; 2]>) -> f64 {
    let mut distance = 0.0;
    for i in 0..route.len() - 1 {
        distance += calculate_distance(&cities[route[i]], &cities[route[i + 1]]);
    }
    distance += calculate_distance(&cities[route[route.len() - 1]], &cities[route[0]]);
    distance
}

/// Solve the traveling salesman problem using simulated annealing
fn simulated_annealing(cities: &Vec<[f64; 2]>) {
    // Initialize the random number generator
    let mut rng = rand::thread_rng();
    
    let mut current_route = (0..cities.len()).collect::<Vec<_>>();
    current_route.push(0);

    let mut current_distance = total_distance(&current_route, cities);
    let mut best_route = current_route.clone();
    let mut best_distance = current_distance;
    
    let mut current_temp = TEMPERATURE;
    while current_temp > 1.0 {
        // Select two random cities to swap
        let i = rng.gen_range(1..current_route.len());
        let j = rng.gen_range(1..current_route.len());
        current_route.swap(i, j);
        
        // Calculate the distance of the new route
        let new_distance = total_distance(&current_route, cities);

        // Determine the difference in distance
        let delta_distance = new_distance - current_distance;

        // Decide whether to accept the new route
        if delta_distance > 0.0 {
            let prob = (-delta_distance / current_temp).exp();
            let rand_val: f64 = rng.gen();
            if rand_val < prob {
                current_distance = new_distance;
            }
            else {
                current_route.swap(i, j);
            }
        }
        else {
            current_distance = new_distance;
        }

        // Update the best route and distance
        if current_distance < best_distance {
            best_route = current_route.clone();
            best_distance = current_distance;
        }

        // Update the temperature
        current_temp *= COOLING_RATE;
    }
    println!("Best route found: ");
    for city in &best_route {
        println!("{}", city);
    }
    println!("Total distance: {}", best_distance);
}

fn main() {
    // List of 48 cities (US state capitals) and their (x, y) coordinates. 
    let cities = vec![
        [6734.0, 1453.0], [2233.0, 10.0],   [5530.0, 1424.0], [401.0, 841.0], 
        [3082.0, 1644.0], [7608.0, 4458.0], [7573.0, 3716.0], [7265.0, 1268.0], 
        [6898.0, 1885.0], [1112.0, 2049.0], [5468.0, 2606.0], [5989.0, 2873.0], 
        [4706.0, 2674.0], [4612.0, 2035.0], [6347.0, 2683.0], [6107.0, 669.0], 
        [7611.0, 5184.0], [7462.0, 3590.0], [7732.0, 4723.0], [5900.0, 3561.0],
        [4483.0, 3369.0], [6101.0, 1110.0], [5199.0, 2182.0], [1633.0, 2809.0], 
        [4307.0, 2322.0], [675.0, 1006.0],  [7555.0, 4819.0], [7541.0, 3981.0], 
        [3177.0, 756.0],  [7352.0, 4506.0], [7545.0, 2801.0], [3245.0, 3305.0], 
        [6426.0, 3173.0], [4608.0, 1198.0], [23.0, 2216.0],   [7248.0, 3779.0], 
        [7762.0, 4595.0], [7392.0, 2244.0], [3484.0, 2829.0], [6271.0, 2135.0],
        [4985.0, 140.0],  [1916.0, 1569.0], [7280.0, 4899.0], [7509.0, 3239.0], 
        [10.0, 2676.0],   [6807.0, 2993.0], [5185.0, 3258.0], [3023.0, 1942.0]];

    simulated_annealing(&cities);
}
