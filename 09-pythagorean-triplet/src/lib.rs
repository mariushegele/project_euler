type Distance = u64;

#[derive(PartialEq, Debug)]
struct PythagoreanTriplet {
    a: Distance,
    b: Distance,
    c: Distance
}

impl PythagoreanTriplet {
    fn product(&self) -> Distance {
        self.a * self.b * self.c
    }

    fn from_side_a_and_perimeter(a: Distance, perimeter: Distance) -> Option<Self> {
        let nom: Distance = perimeter.pow(2) - 2 * perimeter * a;
        let denom: Distance = 2 * perimeter - 2 * a;

        if nom % denom == 0 {
            return Some(Self {
                a,
                b: nom / denom,
                c: perimeter - a - nom / denom
            })
        }

        return None;
    }
}

fn pythagorean_triplet_product(perimeter: u64) -> u64 {
    for a in 1..perimeter {
        match PythagoreanTriplet::from_side_a_and_perimeter(a, perimeter) {
            Some(triplet) => return triplet.product(),
            None => ()
        }
    }

    panic!("no triplet found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pythagorean_triplet() {
        assert_eq!(PythagoreanTriplet::from_side_a_and_perimeter(3, 3 + 4 + 5), 
                   Some(PythagoreanTriplet {a: 3, b: 4, c: 5}));
    }

    #[test]
    fn test_pythagorean_triplet_product() {
        assert_eq!(pythagorean_triplet_product(3 + 4 + 5), 3 * 4 * 5);
        assert_eq!(pythagorean_triplet_product(1000), 31875000);
    }
}