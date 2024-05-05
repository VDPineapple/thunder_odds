use rand::Rng;

const MINUTES: usize = 60*20;

pub fn format_start_time(time: usize) -> String {
    let hours: usize = time / (MINUTES * 60);
    let minutes: usize = (time - hours * MINUTES * 60) / MINUTES;

    let hours: String = if hours.to_string().len() == 1 { format!("0{}", hours) } else { hours.to_string() };
    let minutes: String = if minutes.to_string().len() == 1 { format!("0{}", minutes) } else { minutes.to_string() };

    format!("{}:{}", hours, minutes)
}

fn main() {
    // CHANGE THESE VALUES
    let cutoff = 300 * MINUTES;
    let record_interval = 10 * MINUTES;
    let min_thunder_duration = 1 * MINUTES;
    let earliest_thunder = 35 * MINUTES;
    let iterations = 1000000;

    // DON'T CHANGE THESE VALUES
    let mut thunders: Vec<usize> = vec![0; cutoff / record_interval];
    let mut thunder_durations: f64 = 0.0;
    let mut rain_durations: f64 = 0.0;
    let mut thunder_counts: usize = 0;
    for _ in 0..iterations {
        let mut rain = rand::thread_rng().gen_range(10*MINUTES..150*MINUTES);
        let mut thunder = rand::thread_rng().gen_range(10*MINUTES..150*MINUTES);
        let mut is_raining = false;
        let mut is_thundering = false;
        let mut time = 0;
        let mut thunder_start = 0;
        let mut rain_start = 0;
        let mut has_thundered = vec![false; cutoff / record_interval];

        while time < cutoff {
            
            if rain == 0 && !is_raining {
                is_raining = true;
                rain = rand::thread_rng().gen_range(10*MINUTES..20*MINUTES);
                rain_start = time;
            } else if rain == 0 && is_raining {
                is_raining = false;
                rain = rand::thread_rng().gen_range(10*MINUTES..150*MINUTES);
                rain_durations += (time - rain_start) as f64 / 1000.0;
                rain_start = 0;

                if thunder_start != 0 {
                    if (time - thunder_start) > min_thunder_duration && time >= earliest_thunder {
                        // For all intervals above time, add one to the corresponding index in the thunders vector
                        for i in (time / record_interval)..thunders.len() {
                            if !has_thundered[i] {
                                thunders[i] += 1;
                                has_thundered[i] = true;
                            }
                        }
                    }
                    thunder_durations += (time - thunder_start) as f64 / 1000.0;
                    thunder_counts += 1;
                    thunder_start = 0;
                }
            }
            
            if thunder == 0 && !is_thundering {
                is_thundering = true;
                thunder = rand::thread_rng().gen_range(3*MINUTES..13*MINUTES);
            } else if thunder == 0 && is_thundering {
                is_thundering = false;
                thunder = rand::thread_rng().gen_range(10*MINUTES..150*MINUTES);
                if thunder_start != 0 {
                    if (time - thunder_start) > min_thunder_duration && time >= earliest_thunder {
                        // For all intervals above time, add one to the corresponding index in the thunders vector
                        for i in (time / record_interval)..thunders.len() {
                            if !has_thundered[i] {
                                thunders[i] += 1;
                                has_thundered[i] = true;
                            }
                        }
                    }
                    thunder_durations += (time - thunder_start) as f64 / 1000.0;
                    thunder_counts += 1;
                    thunder_start = 0;
                }
            }
            
            if is_raining && is_thundering {
                // if time >= 50 * 60 * 20 {
                    // has_thundered = true;
                // }
                if thunder_start == 0 {
                    thunder_start = time;
                }
            }
            let diff = rain.min(thunder);
            time += diff;
            rain -= diff;
            thunder -= diff;
        }


        if (cutoff - thunder_start) > min_thunder_duration && thunder_start != 0 {
            // For all intervals above thunder_start, add one to the corresponding index in the thunders vector
            for i in (thunder_start / record_interval)..thunders.len() {
                if !has_thundered[i] {
                    thunders[i] += 1;
                    has_thundered[i] = true;
                }
            }
        }

        if rain_start != 0 {
            rain_durations += (cutoff - rain_start) as f64 / 1000.0;
        }

        if thunder_durations < 0.0 {
            println!("Overflow")
        }
    }
    
    // Print cumulative thunder odds
    for i in 0..thunders.len() {
        let odds = thunders[i] as f64 / iterations as f64;
        println!("Odds of thunder before {}: {}", format_start_time((i+1) * record_interval), odds);
        // println!("{}",odds);
    }

    println!("Average thunder duration: {}", (thunder_durations / thunder_counts as f64) * 1000.0);
    // percentage of time that is thunder
    println!("Proportion of time that is thunder: {}", (thunder_durations / (cutoff as f64 * iterations as f64)) * 1000.0);
    println!("Proportion of time that is rain: {}", (rain_durations / (cutoff as f64 * iterations as f64)) * 1000.0);
}
