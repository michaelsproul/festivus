use types::*;

// Compute the energy usage for a given stream in joules (J, or Ws).
pub fn integral(data: &[Power], stream: PowerStream) -> i64 {
    let (first, tail) = match data.split_first() {
        Some((first, tail)) => (first, tail),
        None => return 0
    };

    tail.iter().fold((first, 0), |(prev, total), current| {
        let prev_value = match stream {
            Total => prev.total,
            HotWater => prev.hot_water,
            Solar => prev.solar
        };
        let dt = (current.time - prev.time).num_seconds();

        (current, total + (prev_value as i64) * dt)
    }).1
}
