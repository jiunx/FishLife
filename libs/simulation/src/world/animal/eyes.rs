use std::f32::consts::{FRAC_PI_4, PI};

use nalgebra::{wrap, Point2, Rotation2, Vector2};

use crate::world::Food;

const FOV_RANGE: f32 = 0.25;
const FOV_ANGLE: f32 = PI + FRAC_PI_4;
const CELLS: usize = 9;

#[derive(Debug)]
pub struct Eyes {
    fov_range: f32,
    fov_angle: f32,
    cells: usize,
}

impl Eyes {
    fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self {
            fov_range,
            fov_angle,
            cells,
        }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn process_vision(
        &self,
        position: Point2<f32>,
        rotation: Rotation2<f32>,
        foods: &[Food],
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];

        for food in foods {
            let vec = food.position - position;
            let dist = vec.norm();
            if dist > self.fov_range {
                continue;
            }
            // Returns vector's direction relative to the X axis, that is:
            //
            //   --> = 0° = 0
            //
            //    |  = 90° = PI / 2
            //    v
            //
            //   <--- = 180° = PI
            //
            // (if you've been measuring rotations before - this is atan2
            // in disguise.)
            let angle = Rotation2::rotation_between(&Vector2::y(), &vec).angle();
            // Because our fish is *also* rotated, we have to include its
            // rotation too:
            let angle = angle - rotation.angle();
            // Rotation is wrapping (from -PI to PI), that is:
            //
            //   = angle of 2*PI
            //   = angle of PI    (because 2*PI >= PI)
            //   = angle of 0     (          PI >= PI)
            //                    (           0 < PI)
            //
            //  angle of 2*PI + PI/2
            //  = angle of 1*PI + PI/2  (because 2*PI + PI/2 >= PI)
            //  = angle of PI/2         (          PI + PI/2 >= PI)
            //                          (               PI/2 < PI)
            //
            //  angle of -2.5*PI
            //  = angle of -1.5*PI  (because -2.5*PI <= -PI)
            //  = angle of -0.5*PI  (        -1.5*PI <= -PI)
            //                      (        -0.5*PI > -PI)
            //
            // Intuitively:
            //
            // - when you rotate yourself twice around the axis, it's the
            //   same as if you rotated once, as if you've never rotated
            //   at all.
            //
            //   (your bony labyrinth might have a different opinion tho.)
            //
            // - when you rotate by 90° and then by 360°, it's the same
            //   as if you rotated only by 90° (*or* by 270°, just in the
            //   opposite direction).
            let angle = wrap(angle, -PI, PI);
            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                continue;
            }

            let angle = angle + self.fov_angle / 2.0;
            let cell = angle / self.fov_angle;
            let cell = cell * (self.cells as f32);
            let cell = (cell as usize).min(cells.len() - 1);
            let energy = (self.fov_range - dist) / self.fov_range;

            cells[cell] += energy;
        }

        cells
    }
}

impl Default for Eyes {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn food(x: f32, y: f32) -> Food {
        Food {
            position: Point2::new(x, y),
            color: "rgb(0, 0, 0)".to_string(),
        }
    }

    struct TestCase {
        foods: Vec<Food>,
        fov_range: f32,
        fov_angle: f32,
        x: f32,
        y: f32,
        rot: f32,
        expected_vision: &'static str,
    }

    const TEST_EYE_CELLS: usize = 13;

    impl TestCase {
        fn run(self) {
            let eye = Eyes::new(self.fov_range, self.fov_angle, TEST_EYE_CELLS);

            let actual_vision = eye.process_vision(
                Point2::new(self.x, self.y),
                Rotation2::new(self.rot),
                &self.foods,
            );
            let actual_vision: Vec<_> = actual_vision
                .into_iter()
                .map(|cell| {
                    // As a reminder, the higher cell's value, the closer
                    // the food is:

                    if cell >= 0.7 {
                        // <0.7, 1.0>
                        // food is right in front of us
                        "#"
                    } else if cell >= 0.3 {
                        // <0.3, 0.7)
                        // food is somewhat further
                        "+"
                    } else if cell > 0.0 {
                        // <0.0, 0.3)
                        // food is pretty far away
                        "."
                    } else {
                        // 0.0
                        // no food in sight, this cell sees empty space
                        " "
                    }
                })
                .collect();
            let actual_vision = actual_vision.join("");

            assert_eq!(actual_vision, self.expected_vision);
        }
    }

    mod different_fov_ranges {
        use super::*;
        use test_case::test_case;

        #[test_case(1.0, "      +      ")] // Food is inside the FOV
        #[test_case(0.9, "      +      ")] // ditto
        #[test_case(0.8, "      +      ")] // ditto
        #[test_case(0.7, "      .      ")] // Food slowly disappears
        #[test_case(0.6, "      .      ")] // ditto
        #[test_case(0.5, "             ")] // Food disappeared!
        #[test_case(0.4, "             ")]
        #[test_case(0.3, "             ")]
        #[test_case(0.2, "             ")]
        #[test_case(0.1, "             ")]
        fn test(fov_range: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(0.5, 1.0)],
                fov_angle: std::f32::consts::FRAC_PI_2,
                x: 0.5,
                y: 0.5,
                rot: 0.0,
                fov_range,
                expected_vision,
            }
            .run()
        }
    }

    mod different_rotations {
        use super::*;
        use test_case::test_case;
        #[test_case(0.00 * PI, "         +   ")] // Food is to our right
        #[test_case(0.25 * PI, "        +    ")]
        #[test_case(0.50 * PI, "      +      ")] // Food is in front of us
        #[test_case(0.75 * PI, "    +        ")]
        #[test_case(1.00 * PI, "   +         ")] // Food is to our left
        #[test_case(1.25 * PI, " +           ")]
        #[test_case(1.50 * PI, "            +")] // Food is behind us
        #[test_case(1.75 * PI, "           + ")] // (we continue to see it
        #[test_case(2.00 * PI, "         +   ")] // due to 360° fov_angle.)
        #[test_case(2.25 * PI, "        +    ")]
        #[test_case(2.50 * PI, "      +      ")]
        fn test(rot: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(0.0, 0.5)],
                fov_range: 1.0,
                fov_angle: 2.0 * PI,
                x: 0.5,
                y: 0.5,
                rot,
                expected_vision,
            }
            .run()
        }
    }

    mod different_positions {
        use super::*;
        use test_case::test_case;
        // Checking the X axis:
        // (you can see the bird is "flying away" from the foods)
        #[test_case(0.9, 0.5, "#           #")]
        #[test_case(0.8, 0.5, "  #       #  ")]
        #[test_case(0.7, 0.5, "   +     +   ")]
        #[test_case(0.6, 0.5, "    +   +    ")]
        #[test_case(0.5, 0.5, "    +   +    ")]
        #[test_case(0.4, 0.5, "     + +     ")]
        #[test_case(0.3, 0.5, "     . .     ")]
        #[test_case(0.2, 0.5, "     . .     ")]
        #[test_case(0.1, 0.5, "     . .     ")]
        #[test_case(0.0, 0.5, "             ")]
        //
        // Checking the Y axis:
        // (you can see the bird is "flying alongside" the foods)
        #[test_case(0.5, 0.0, "            +")]
        #[test_case(0.5, 0.1, "          + .")]
        #[test_case(0.5, 0.2, "         +  +")]
        #[test_case(0.5, 0.3, "        + +  ")]
        #[test_case(0.5, 0.4, "      +  +   ")]
        #[test_case(0.5, 0.6, "   +  +      ")]
        #[test_case(0.5, 0.7, "  + +        ")]
        #[test_case(0.5, 0.8, "+  +         ")]
        #[test_case(0.5, 0.9, ". +          ")]
        #[test_case(0.5, 1.0, "+            ")]
        fn test(x: f32, y: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(1.0, 0.4), food(1.0, 0.6)],
                fov_range: 1.0,
                fov_angle: std::f32::consts::FRAC_PI_2,
                rot: 3.0 * std::f32::consts::FRAC_PI_2,
                x,
                y,
                expected_vision,
            }
            .run()
        }
    }

    mod different_fov_angles {
        use super::*;
        use test_case::test_case;
        #[test_case(0.25 * PI, " +         + ")] // FOV is narrow = 2 foods
        #[test_case(0.50 * PI, ".  +     +  .")]
        #[test_case(0.75 * PI, "  . +   + .  ")] // FOV gets progressively
        #[test_case(1.00 * PI, "   . + + .   ")] // wider and wider...
        #[test_case(1.25 * PI, "   . + + .   ")]
        #[test_case(1.50 * PI, ".   .+ +.   .")]
        #[test_case(1.75 * PI, ".   .+ +.   .")]
        #[test_case(2.00 * PI, "+.  .+ +.  .+")] // FOV is wide = 8 foods
        fn test(fov_angle: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![
                    food(0.0, 0.0),
                    food(0.0, 0.33),
                    food(0.0, 0.66),
                    food(0.0, 1.0),
                    food(1.0, 0.0),
                    food(1.0, 0.33),
                    food(1.0, 0.66),
                    food(1.0, 1.0),
                ],
                fov_range: 1.0,
                x: 0.5,
                y: 0.5,
                rot: 3.0 * std::f32::consts::FRAC_PI_2,
                fov_angle,
                expected_vision,
            }
            .run()
        }
    }
}
