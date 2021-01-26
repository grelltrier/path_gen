use std::collections::HashMap;
use std::vec::Vec;

use super::{Path, Point};

fn segments(gesture: Path, word: &str, button_coordinates: HashMap<String, Point>) -> Path {
    let mut points: Vec<&Point> = gesture.waypoints.iter().skip(2).collect(); // Skip the first two point
                                                                              // Remove the last two points
    points.pop();
    points.pop();

    let keys = super::key_centers(word, button_coordinates);
    let segments = Vec::new();

    let mut key_iterator = keys.iter().peekable();
    while let Some(key) = key_iterator.next() {
        // Skip keys if they are the same as the previous one
        let next_key = key_iterator.peek();
        if key == next_key {
            continue;
        }
        let mut segment = Vec::new();
        let mut d = f64::MAX;
        for point in points {
            let d_new = point.distance(*key);
            if d < d_new || d_new < point.distance_option(next_key) {
                segment.push(point);
                d = d_new;
            }
            else {
                break;
            }
        }
        remove_points(points,segment);
    }

    /*
    9:  for p ∈ pointsdo
    10: d' <- Distance(p, keys_k)                           Euclidean distance
    11: if d<d'or d' < Distance(p, keys_(1+k)) then
    12: append p to segment
    13: d <- d'
    14: else break                                        Segment for keys_k complete
    15: remove all points in segment from points
    16: append segment to segments
    17: prepend first two points from gesture to segments_1
    18: append [all remaining points and the last two points from gesture] to segments
    19: return segments*/

    gesture //delete this row
}

/*1:  Segments (gesture, word)
2:  points <- gesture                                except the first two and last two points
3:  keys<- KeyCentres(word)
4:  segments<- empty array
5:  for 1 ...| keys | -1do                          <= Unable to read properly!!
6:  if keys_k =keys_(k+1)then continue
7:  segment<- empty array                            Points for keys_k
8:  d <- infinite
9:  for p ∈ pointsdo
10: d' <- Distance(p, keys_k)                           Euclidean distance
11: if d<d'or d' < Distance(p, keys_(1+k)) then
12: append p to segment
13: d <- d'
14: else break                                        Segment for keys_k complete
15: remove all points in segment from points
16: append segment to segments
17: prepend first two points from gesture to segments_1
18: append [all remaining points and the last two points from gesture] to segments
19: return segments*/
