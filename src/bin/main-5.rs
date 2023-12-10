use std::fs;

fn main() {
    let filename = "inputs/5.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let mut lines = content.lines();

    let seeds = parse_seeds(lines.next().unwrap());

    // skip to next heading, pop it off, then parse until next empty line; rinse
    // and repeat
    let mut lines = lines.skip_while(|l| l.find(':').is_none());
    lines.next();
    let seed_to_soil_map = parse_range_map(
        lines
            .clone()
            .take_while(|l| !l.is_empty())
            .collect::<Vec<&str>>(),
    );

    let mut lines = lines.skip_while(|l| l.find(':').is_none());
    lines.next();
    let soil_to_fertilizer_map = parse_range_map(
        lines
            .clone()
            .take_while(|l| !l.is_empty())
            .collect::<Vec<&str>>(),
    );

    let mut lines = lines.skip_while(|l| l.find(':').is_none());
    lines.next();
    let fertilizer_to_water_map = parse_range_map(
        lines
            .clone()
            .take_while(|l| !l.is_empty())
            .collect::<Vec<&str>>(),
    );

    let mut lines = lines.skip_while(|l| l.find(':').is_none());
    lines.next();
    let water_to_light_map = parse_range_map(
        lines
            .clone()
            .take_while(|l| !l.is_empty())
            .collect::<Vec<&str>>(),
    );

    let mut lines = lines.skip_while(|l| l.find(':').is_none());
    lines.next();
    let light_to_temperature_map = parse_range_map(
        lines
            .clone()
            .take_while(|l| !l.is_empty())
            .collect::<Vec<&str>>(),
    );

    let mut lines = lines.skip_while(|l| l.find(':').is_none());
    lines.next();
    let temperature_to_humidity_map = parse_range_map(
        lines
            .clone()
            .take_while(|l| !l.is_empty())
            .collect::<Vec<&str>>(),
    );

    let mut lines = lines.skip_while(|l| l.find(':').is_none());
    lines.next();
    let humidity_to_location_map = parse_range_map(
        lines
            .clone()
            .take_while(|l| !l.is_empty())
            .collect::<Vec<&str>>(),
    );

    let lln = seeds
        .iter()
        .map(|&idx| {
            let idx = seed_to_soil_map.convert(idx);
            let idx = soil_to_fertilizer_map.convert(idx);
            let idx = fertilizer_to_water_map.convert(idx);
            let idx = water_to_light_map.convert(idx);
            let idx = light_to_temperature_map.convert(idx);
            let idx = temperature_to_humidity_map.convert(idx);
            humidity_to_location_map.convert(idx)
        })
        .min()
        .unwrap();

    println!(
        "The lowest location number that corresponds to any of the initial seed numbers is {lln}."
    );

    // brute force part 2
    let lln = seeds
        .as_slice()
        .chunks(2)
        .map(|e| {
            let start = e[0];
            let length = e[1];
            // TODO speed this up by choosing idx better
            (start..start + length)
                .map(|idx| {
                    let idx = seed_to_soil_map.convert(idx);
                    let idx = soil_to_fertilizer_map.convert(idx);
                    let idx = fertilizer_to_water_map.convert(idx);
                    let idx = water_to_light_map.convert(idx);
                    let idx = light_to_temperature_map.convert(idx);
                    let idx = temperature_to_humidity_map.convert(idx);
                    humidity_to_location_map.convert(idx)
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap();
    println!(
        "The lowest location number that corresponds to any of the initial seed numbers is {lln}."
    );
}

fn parse_seeds(l: &str) -> Vec<usize> {
    let l = l.split(':').collect::<Vec<&str>>();
    l[1].split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn parse_range_map(lines: Vec<&str>) -> MultiRangeMap {
    let range_maps = lines
        .iter()
        .map(|l| {
            let l = l
                .split(' ')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            RangeMap {
                destination_start: l[0],
                source_start: l[1],
                range_length: l[2],
            }
        })
        .collect::<Vec<RangeMap>>();
    MultiRangeMap { range_maps }
}

#[derive(Debug)]
struct MultiRangeMap {
    range_maps: Vec<RangeMap>,
}

impl MultiRangeMap {
    fn convert(&self, idx: usize) -> usize {
        for rm in self.range_maps.iter() {
            if let Ok(result) = rm.convert(idx) {
                return result;
            }
        }
        idx
    }
}

#[derive(Debug)]
struct RangeMap {
    destination_start: usize,
    source_start: usize,
    range_length: usize,
}

impl RangeMap {
    fn convert(&self, idx: usize) -> Result<usize, OutOfRangeError> {
        // if idx >= self.source_start && idx < self.source_start + self.range_length {
        //     let result = self.destination_start + (idx - self.source_start);
        //     Ok(result)
        // } else {
        //     Err(OutOfRangeError)
        // }
        if idx < self.source_start || idx >= self.source_start + self.range_length {
            Err(OutOfRangeError)
        } else {
            let result = self.destination_start + (idx - self.source_start);
            Ok(result)
        }
    }
}

struct OutOfRangeError;
