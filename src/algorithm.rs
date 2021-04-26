/*
Line segment intersection detection library.

Copyright (C) 2021 eadf https://github.com/eadf

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version.

This program is distributed in the hope that it will be useful, but WITHOUT
ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program. If not, see <https://www.gnu.org/licenses/>.

Also add information on how to contact you by electronic and paper mail.

If the program does terminal interaction, make it output a short notice like
this when it starts in an interactive mode:

intersection2d Copyright (C) 2021 eadf

This program comes with ABSOLUTELY NO WARRANTY; for details type `show w'.

This is free software, and you are welcome to redistribute it under certain
conditions; type `show c' for details.

The hypothetical commands `show w' and `show c' should show the appropriate
parts of the General Public License. Of course, your program's commands might
be different; for a GUI interface, you would use an "about box".

You should also get your employer (if you work as a programmer) or school,
if any, to sign a "copyright disclaimer" for the program, if necessary. For
more information on this, and how to apply and follow the GNU GPL, see <https://www.gnu.org/licenses/>.

The GNU General Public License does not permit incorporating your program
into proprietary programs. If your program is a subroutine library, you may
consider it more useful to permit linking proprietary applications with the
library. If this is what you want to do, use the GNU Lesser General Public
License instead of this License. But first, please read <https://www.gnu.org/
licenses /why-not-lgpl.html>.
 */

use super::{intersect, ulps_eq_c};
use core::fmt;
use num_traits::{Float, Zero};
use std::cmp;
use std::convert::identity;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct SiteEventKey<T>
where
    T: Float + approx::UlpsEq + geo::CoordFloat,
    T::Epsilon: Copy,
{
    pub pos: geo::Coordinate<T>,
}

impl<T> SiteEventKey<T>
where
    T: Float + approx::UlpsEq + Copy + geo::CoordFloat,
    T::Epsilon: Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Self {
            pos: geo::Coordinate { x, y },
        }
    }
}

impl<T> Debug for SiteEventKey<T>
where
    T: Float + approx::UlpsEq + Copy + geo::CoordFloat,
    T::Epsilon: Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.pos.x)
            .field(&self.pos.y)
            .finish()
    }
}

impl<T> PartialOrd for SiteEventKey<T>
where
    T: Float + geo::CoordFloat + approx::AbsDiffEq + approx::UlpsEq,
    T::Epsilon: Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if approx::ulps_eq!(&self.pos.y, &other.pos.y) {
            if approx::ulps_eq!(&self.pos.x, &other.pos.x) {
                Some(cmp::Ordering::Equal)
            } else {
                self.pos.x.partial_cmp(&other.pos.x)
            }
        } else {
            self.pos.y.partial_cmp(&other.pos.y)
        }
    }
}

impl<T> PartialEq for SiteEventKey<T>
where
    T: Float + geo::CoordFloat + approx::AbsDiffEq + approx::UlpsEq,
    T::Epsilon: Copy,
{
    fn eq(&self, other: &Self) -> bool {
        ulps_eq_c(&self.pos, &other.pos)
    }
}

/// A container struct that keeps track of the lines around a pivot point.
/// It only stores the lines with highest x value left of pivot point, and lines with lowest x
/// value right of the point. Secondarily it prioritizes according to slope of the line, lines
/// leaning towards pivot point have priority.
struct MinMax<T>
where
    T: Float + approx::UlpsEq + geo::CoordFloat,
    T::Epsilon: Copy,
{
    best_left: Option<T>,
    slope: MinMaxSlope<T>,
    best_right: Option<T>,
}

impl<T> MinMax<T>
where
    T: Sized + Float + approx::UlpsEq + geo::CoordFloat,
    T::Epsilon: Copy,
{
    fn new() -> Self {
        Self {
            best_left: None,
            best_right: None,
            slope: MinMaxSlope::new(),
        }
    }

    /// keep track of the candidates closest (on both sides) to pivot_x
    fn update(&mut self, pivot_x: T, candidate_x: T, candidate_slope: T, candidate_index: usize) {
        if candidate_x < pivot_x {
            /*println!(
                "Left:looking at {} candidate_x:{} candidate_slope:{}",
                candidate_index, candidate_x, candidate_slope
            );*/
            // handle left side
            if let Some(current_min) = self.best_left {
                if approx::ulps_eq!(&current_min, &candidate_x) {
                    self.slope
                        .update_left(false, candidate_slope, candidate_index);
                } else if current_min < candidate_x {
                    // this candidate is better.
                    self.slope
                        .update_left(true, candidate_slope, candidate_index);
                    self.best_left = Some(candidate_x);
                } else {
                    /*println!(
                        "Left2:rejecting {} candidate_x:{} current_x:{} candidate_slope:{}, current_slope:{}",
                        candidate_index, candidate_x, current_min, candidate_slope, self.slope.best_left.unwrap()
                    )*/
                }
            } else {
                // First sample
                self.best_left = Some(candidate_x);
                self.slope
                    .update_left(false, candidate_slope, candidate_index);
            }
        } else if candidate_x > pivot_x {
            /*println!(
                "Right:looking at {} candidate_x:{} candidate_slope:{}",
                candidate_index, candidate_x, candidate_slope
            );*/
            // handle right side
            if let Some(current_max) = self.best_right {
                if approx::ulps_eq!(&current_max, &candidate_x) {
                    self.slope
                        .update_right(false, candidate_slope, candidate_index);
                } else if current_max > candidate_x {
                    // this candidate is better.
                    self.slope
                        .update_right(true, candidate_slope, candidate_index);
                    self.best_right = Some(candidate_x);
                } else {
                    /*println!(
                        "Right2:rejecting {} candidate_x:{} current_x:{} candidate_slope:{}, current_slope:{}",
                        candidate_index, candidate_x, current_max, candidate_slope, self.slope.best_right.unwrap()
                    )*/
                }
            } else {
                // First sample
                self.best_right = Some(candidate_x);
                self.slope
                    .update_right(false, candidate_slope, candidate_index);
            }
        }
    }

    /// clear all data
    fn clear(&mut self) {
        self.best_left = None;
        self.best_right = None;
        self.slope.clear();
    }
}

struct MinMaxSlope<T>
where
    T: Float + approx::UlpsEq + geo::CoordFloat,
    T::Epsilon: Copy,
{
    best_left: Option<T>, // slope
    candidates_left: Vec<usize>,

    best_right: Option<T>, // slope
    candidates_right: Vec<usize>,
}

impl<T> MinMaxSlope<T>
where
    T: Sized + Float + approx::UlpsEq + geo::CoordFloat,
    T::Epsilon: Copy,
{
    fn new() -> Self {
        Self {
            best_left: None,
            candidates_left: Vec::<usize>::new(),
            best_right: None,
            candidates_right: Vec::<usize>::new(),
        }
    }

    /// sort candidates based on slope, keep only the ones with 'flattest' angle to the left and right
    fn update_both(&mut self, candidate_index: usize, lines: &[geo::Line<T>]) {
        let line = lines[candidate_index];
        let candidate_slope = if approx::ulps_eq!(&line.end.y, &line.start.y) {
            T::infinity()
        } else {
            (line.end.x - line.start.x) / (line.end.y - line.start.y)
        };

        self.update_left(false, candidate_slope, candidate_index);
        self.update_right(false, candidate_slope, candidate_index);
    }

    /// sort candidates based on slope, keep only the best to the left -> lowest abs(negative slope value)
    fn update_left(&mut self, clear: bool, candidate_slope: T, candidate_index: usize) {
        if clear {
            self.candidates_left.clear();
            self.best_left = None;
        }
        /*println!(
            "Left:looking at {} candidate_slope:{}",
            candidate_index, candidate_slope
        );*/
        // handle left side
        if let Some(current_slope) = self.best_left {
            if approx::ulps_eq!(&current_slope, &candidate_slope) {
                // this candidate is just as good as the others already found
                self.candidates_left.push(candidate_index);
            } else if candidate_slope < current_slope {
                // this candidate is better.
                self.candidates_left.clear();
                self.candidates_left.push(candidate_index);
                self.best_left = Some(candidate_slope);
            } else {
                /*println!(
                    "Left1:rejecting {} candidate_slope:{}, current_slope:{}",
                    candidate_index, candidate_slope, current_slope
                )*/
            }
        } else {
            // First sample
            self.best_left = Some(candidate_slope);
            self.candidates_left.push(candidate_index);
        }
        /*println!(
            "Left: best slope is {:?} ,best_left:{}",
            self.candidates_left,
            self.best_left.unwrap()
        )*/
    }

    /// sort candidates based on slope, keep only the best to the left -> lowest positive slope value
    fn update_right(&mut self, clear: bool, candidate_slope: T, candidate_index: usize) {
        /*println!(
            "Right:looking at {} candidate_slope:{}",
            candidate_index, candidate_slope
        );*/
        if clear {
            self.candidates_right.clear();
            self.best_right = None;
        }
        // handle right side
        if let Some(current_slope) = self.best_right {
            if approx::ulps_eq!(&current_slope, &candidate_slope) {
                // this candidate is just as good as the others already found
                self.candidates_right.push(candidate_index);
            } else if candidate_slope > current_slope {
                // this candidate is better.
                self.candidates_right.clear();
                self.candidates_right.push(candidate_index);
                self.best_right = Some(candidate_slope);
            } else {
                /*println!(
                    "Right1:rejecting {} candidate_slope:{}, current_slope:{}",
                    candidate_index, candidate_slope, current_slope
                )*/
            }
        } else {
            // First sample
            self.best_right = Some(candidate_slope);
            self.candidates_right.push(candidate_index);
        }
        /*println!(
            "Right: best slope is {:?} ,best_right:{}",
            self.candidates_right,
            self.best_right.unwrap()
        )*/
    }

    /// clear all data
    fn clear(&mut self) {
        self.best_left = None;
        self.candidates_left.clear();

        self.best_right = None;
        self.candidates_right.clear();
    }
}

/// SiteEvents contains the events happening at a specific point.
/// Line segments have their start and end positions arranged so that line.start.y < line.end.y
/// Sorting is based on their Y-coordinate, secondary the X-coordinate. (line.start.x < line.end.x)
///
/// The 'drop' list contains the line segments that ends in the event point.
/// The 'add' list contains the line segments that starts in the event point.
/// The 'intersection' list contains the line segments that intersects at the event point.
///
pub struct SiteEvent<T>
where
    T: Float + approx::UlpsEq + geo::CoordFloat,
    T::Epsilon: Copy,
{
    drop: Option<Vec<usize>>,
    add: Option<Vec<usize>>,
    intersection: Option<Vec<usize>>,
    #[doc(hidden)]
    pd: PhantomData<T>,
}

impl<T> SiteEvent<T>
where
    T: Float + approx::UlpsEq + geo::CoordFloat,
    T::Epsilon: Copy,
{
    pub(crate) fn with_intersection(i: &[usize]) -> Self {
        Self {
            drop: None,
            add: None,
            intersection: Some(i.to_vec()),
            pd: PhantomData,
        }
    }

    pub(crate) fn with_drop(l: &[usize]) -> Self {
        Self {
            drop: Some(l.to_vec()),
            add: None,
            intersection: None,
            pd: PhantomData,
        }
    }

    pub(crate) fn with_add(l: &[usize]) -> Self {
        Self {
            drop: None,
            add: Some(l.to_vec()),
            intersection: None,
            pd: PhantomData,
        }
    }

    pub fn get_intersections(&self) -> &Option<Vec<usize>> {
        &self.intersection
    }
}

/// Returns *one* point of intersection between the `sweepline` and `other`
/// Second return value is the slope of the line
fn sweepline_intersection<T>(sweepline: geo::Coordinate<T>, other: &geo::Line<T>) -> Option<(T, T)>
where
    T: Float + Zero + geo::CoordFloat + approx::AbsDiffEq + approx::UlpsEq,
    T::Epsilon: Copy,
{
    // line equation: y=slope*x+d => d=y-slope*x => x = (y-d)/slope
    let y1 = other.start.y;
    let y2 = other.end.y;
    let x1 = other.start.x;
    let x2 = other.end.x;
    if approx::ulps_eq!(&y1, &y2) {
        // horizontal line: return to the point right of sweepline.x, if any
        // Any point to the left are supposedly already handled.
        if sweepline.x < x2 {
            return Some((x2, T::zero()));
        } else {
            return None;
        }
    }

    if approx::ulps_eq!(&x1, &x2) {
        return Some((x1, T::infinity()));
    }

    let slope: T = (y2 - y1) / (x2 - x1);
    if slope.is_nan() {
        panic!("D==0 should not happen!");
        //return None;
    }
    let d = y1 - slope * x1;
    Some(((sweepline.y - d) / slope, slope))
}

/// Contains the data the sweep-line intersection algorithm needs to operate.
/// Most of these containers are stored inside an Option. This makes it possible
/// to take() them and make the borrow-checker happy.
pub struct AlgorithmData<T>
where
    T: Float + geo::CoordFloat + approx::AbsDiffEq + approx::UlpsEq,
    T::Epsilon: Copy,
{
    // sweep-line position
    sweepline_pos: geo::Coordinate<T>,
    // Stop when first intersection is found
    stop_at_first_intersection: bool,
    // Allow start&end points to intersect
    // i.e. don't report them as an intersections.
    // An endpoint intersecting any other point of another line will still be
    // counted as an intersection.
    pub ignore_end_point_intersections: bool,
    // The unhandled events
    site_events: Option<rb_tree::RBMap<SiteEventKey<T>, SiteEvent<T>>>,
    // The lines we are considering at any given point in time
    active_lines: Option<ahash::AHashSet<usize>>,
    // A list of intersection points and the line segments involved in each intersection
    result: Option<rb_tree::RBMap<SiteEventKey<T>, Vec<usize>>>,
    intersection_calls: usize,
    // The 'best' lines surrounding the event point but not directly connected to the point.
    neighbour_priority: Option<MinMax<T>>,
    // The 'best' lines directly connected to the event point.
    connected_priority: Option<MinMaxSlope<T>>,
    // The input geometry. These lines are re-arranged so that Line.start.y <= Line.end.y
    // These are never changed while the algorithm is running.
    lines: Vec<geo::Line<T>>,
}

impl<T> Default for AlgorithmData<T>
where
    T: Float + num_traits::ToPrimitive + geo::CoordFloat + approx::AbsDiffEq + approx::UlpsEq,
    T::Epsilon: Copy,
{
    fn default() -> Self {
        Self {
            sweepline_pos: geo::Coordinate {
                x: -T::max_value(),
                y: -T::max_value(),
            },
            stop_at_first_intersection: false,
            ignore_end_point_intersections: false,
            site_events: Some(rb_tree::RBMap::new()),
            lines: Vec::<geo::Line<T>>::new(),
            result: Some(rb_tree::RBMap::new()),
            active_lines: Some(ahash::AHashSet::default()),
            intersection_calls: 0,
            neighbour_priority: Some(MinMax::new()),
            connected_priority: Some(MinMaxSlope::new()),
        }
    }
}

impl<T> AlgorithmData<T>
where
    T: Float + num_traits::ToPrimitive + geo::CoordFloat + approx::AbsDiffEq + approx::UlpsEq,
    T::Epsilon: Copy,
{
    pub fn get_sweepline_pos(&self) -> &geo::Coordinate<T> {
        &self.sweepline_pos
    }

    pub fn get_lines(&self) -> &Vec<geo::Line<T>> {
        &self.lines
    }

    pub fn get_results(&self) -> &Option<rb_tree::RBMap<SiteEventKey<T>, Vec<usize>>> {
        &self.result
    }

    /// This removes the results from the AlgorithmData structure
    #[allow(clippy::type_complexity)]
    pub fn take_results<'a>(
        &mut self,
    ) -> Result<
        Box<dyn ExactSizeIterator<Item = (geo::Coordinate<T>, Vec<usize>)> + 'a>,
        super::IntersectError,
    >
    where
        T: 'a,
    {
        if let Some(rv) = self.result.take() {
            Ok(Box::new(rv.into_iter().map(|x| (x.0.pos, x.1))))
        } else {
            Err(super::IntersectError::ResultsAlreadyTaken(
                "Results already taken from structure".to_string(),
            ))
        }
    }

    pub fn get_site_events(&self) -> &Option<rb_tree::RBMap<SiteEventKey<T>, SiteEvent<T>>> {
        &self.site_events
    }

    pub fn get_active_lines(&self) -> &Option<ahash::AHashSet<usize>> {
        &self.active_lines
    }

    pub fn get_intersection_calls(&self) -> usize {
        self.intersection_calls
    }

    pub fn with_stop_at_first_intersection(
        &mut self,
        value: bool,
    ) -> Result<&mut Self, super::IntersectError> {
        self.stop_at_first_intersection = value;
        Ok(self)
    }

    pub fn with_ignore_end_point_intersections(
        &mut self,
        value: bool,
    ) -> Result<&mut Self, super::IntersectError> {
        self.ignore_end_point_intersections = value;
        Ok(self)
    }

    /// Add data to the input lines.
    /// Sort the end point according to the order of SiteEventKey.
    /// Populate the event queue
    /// Todo: this duplicates functionality of 'with_ref_lines()', try to consolidate..
    pub fn with_lines<I>(&mut self, input_iter: I) -> Result<&mut Self, super::IntersectError>
    where
        I: Iterator<Item = geo::Line<T>>,
    {
        let mut site_events = self.site_events.take().unwrap();

        for (index, mut aline) in input_iter.enumerate() {
            if !(aline.start.x.is_finite()
                && aline.start.y.is_finite()
                && aline.end.x.is_finite()
                && aline.end.y.is_finite())
            {
                return Err(super::IntersectError::InvalidData(
                    "Can't check for intersections on non-finite data".to_string(),
                ));
            }

            // Re-arrange so that:
            // SiteEvent.pos.start < SiteEvent.pos.end (primary ordering: pos.y, secondary: pos.x)
            if !(SiteEventKey { pos: aline.start }).lt(&(SiteEventKey { pos: aline.end })) {
                std::mem::swap(&mut aline.start, &mut aline.end);
            };

            self.lines.push(aline);

            let key_start = SiteEventKey { pos: aline.start };
            let key_end = SiteEventKey { pos: aline.end };

            // start points goes into the site_event::add list
            if let Some(mut event) = site_events.get_mut(&key_start) {
                let mut lower = event.add.take().map_or(Vec::<usize>::new(), identity);
                lower.push(index);
                event.add = Some(lower);
            } else {
                let event = SiteEvent::<T>::with_add(&[index]);
                let _ = site_events.insert(key_start, event);
            }

            // end points goes into the site_event::drop list
            if let Some(mut event) = site_events.get_mut(&key_end) {
                let mut upper = event.drop.take().map_or(Vec::<usize>::new(), identity);
                upper.push(index);
                event.drop = Some(upper);
            } else {
                let event = SiteEvent::<T>::with_drop(&[index]);
                let _ = site_events.insert(key_end, event);
            }
        }

        self.site_events = Some(site_events);
        #[cfg(feature = "console_trace")]
        self.debug();
        Ok(self)
    }

    /// Add data to the input lines.
    /// Sort the end point according to the order of SiteEventKey.
    /// Populate the event queue
    /// TODO: is this worth keeping? AlgorithmData always keeps copies of the input geometry anyways
    pub fn with_ref_lines<'a, I>(
        &mut self,
        input_iter: I,
    ) -> Result<&mut Self, super::IntersectError>
    where
        T: 'a,
        I: Iterator<Item = &'a geo::Line<T>>,
    {
        let mut site_events = self.site_events.take().unwrap();

        for (index, aline) in input_iter.enumerate() {
            if !(aline.start.x.is_finite()
                && aline.start.y.is_finite()
                && aline.end.x.is_finite()
                && aline.end.y.is_finite())
            {
                return Err(super::IntersectError::InvalidData(
                    "Can't check for intersections on non-finite data".to_string(),
                ));
            }

            // Re-arrange so that:
            // SiteEvent.pos.start < SiteEvent.pos.end (primary ordering: pos.y, secondary: pos.x)
            let aline =
                if (SiteEventKey { pos: aline.start }).lt(&(SiteEventKey { pos: aline.end })) {
                    geo::Line {
                        start: aline.start,
                        end: aline.end,
                    }
                } else {
                    geo::Line {
                        start: aline.end,
                        end: aline.start,
                    }
                };

            self.lines.push(aline);

            let key_start = SiteEventKey { pos: aline.start };
            let key_end = SiteEventKey { pos: aline.end };

            // start points goes into the site_event::add list
            if let Some(mut event) = site_events.get_mut(&key_start) {
                let mut lower = event.add.take().map_or(Vec::<usize>::new(), identity);
                lower.push(index);
                event.add = Some(lower);
            } else {
                let event = SiteEvent::<T>::with_add(&[index]);
                let _ = site_events.insert(key_start, event);
            }

            // end points goes into the site_event::drop list
            if let Some(mut event) = site_events.get_mut(&key_end) {
                let mut upper = event.drop.take().map_or(Vec::<usize>::new(), identity);
                upper.push(index);
                event.drop = Some(upper);
            } else {
                let event = SiteEvent::<T>::with_drop(&[index]);
                let _ = site_events.insert(key_end, event);
            }
        }

        self.site_events = Some(site_events);
        #[cfg(feature = "console_trace")]
        self.debug();
        Ok(self)
    }

    ///
    /// Add a new intersection event to the event queue
    ///
    fn add_intersection_event(
        &self,
        site_events: &mut rb_tree::RBMap<SiteEventKey<T>, SiteEvent<T>>,
        position: &SiteEventKey<T>,
        intersecting_lines: &[usize],
    ) {
        if let Some(event) = site_events.get_mut(position) {
            // there were already events at this point
            let mut intersections_added = 0;
            for new_intersection in intersecting_lines.iter() {
                // only add this line as an intersection if the intersection lies
                // at the interior of the line (no end point)
                let i_line = self.lines[*new_intersection];
                if ulps_eq_c(&position.pos, &i_line.start) || ulps_eq_c(&position.pos, &i_line.end)
                {
                    continue;
                }

                if let Some(prev_intersections) = &mut event.intersection {
                    prev_intersections.push(*new_intersection);
                } else {
                    let new_vec = vec![*new_intersection];
                    event.intersection = Some(new_vec);
                }
                intersections_added += 1;
            }
            if intersections_added > 0 {
                let mut intersections = event.intersection.take().unwrap();
                intersections.sort_unstable();
                intersections.dedup();
                event.intersection = Some(intersections);
            }
        } else {
            // this is the first event at this point
            let event = SiteEvent::with_intersection(intersecting_lines);
            let _ = site_events.insert(*position, event);
        }
    }

    /// handles input event, returns true when done
    /// If interactive is set, the method will handle only one event for each call
    #[allow(clippy::type_complexity)]
    pub fn compute<'a>(
        &mut self,
    ) -> Result<
        Box<dyn ExactSizeIterator<Item = (geo::Coordinate<T>, Vec<usize>)> + 'a>,
        super::IntersectError,
    >
    where
        T: 'a,
    {
        // this could only happen if first run interactive, but just in case..
        if self.stop_at_first_intersection && self.result.as_ref().map_or(false, |x| !x.is_empty())
        {
            return self.take_results();
        }

        // make the borrow checker happy by breaking the link between self and all the
        // containers and their iterators.
        let mut active_lines = self.active_lines.take().unwrap();
        let mut site_events = self.site_events.take().unwrap();
        let mut result = self.result.take().unwrap();
        let mut neighbour_priority = self.neighbour_priority.take().unwrap();
        let mut connected_priority = self.connected_priority.take().unwrap();

        loop {
            if let Some((key, event)) = site_events.pop_pair() {
                self.handle_event(
                    &key,
                    &event,
                    &mut active_lines,
                    &mut neighbour_priority,
                    &mut connected_priority,
                    &mut site_events,
                    &mut result,
                );
            } else {
                self.sweepline_pos = geo::Coordinate {
                    x: T::max_value(),
                    y: T::max_value(),
                };
                break;
            }
        }

        // put the borrowed containers back
        self.site_events = Some(site_events);
        self.active_lines = Some(active_lines);
        self.result = Some(result);
        self.neighbour_priority = Some(neighbour_priority);
        self.connected_priority = Some(connected_priority);
        self.take_results()
    }

    /// handles input event, returns true when done
    /// You will have call take_results() if the method returns true
    pub fn compute_iterative(&mut self) -> Result<bool, super::IntersectError> {
        if self.stop_at_first_intersection && self.result.as_ref().map_or(false, |x| !x.is_empty())
        {
            return Ok(true);
        }

        // make the borrow checker happy by breaking the link between self and all the
        // containers and their iterators.
        let mut active_lines = self.active_lines.take().unwrap();
        let mut site_events = self.site_events.take().unwrap();
        let mut result = self.result.take().unwrap();
        let mut neighbour_priority = self.neighbour_priority.take().unwrap();
        let mut connected_priority = self.connected_priority.take().unwrap();

        // return value
        let algorithm_is_done: bool;

        algorithm_is_done = if let Some((key, event)) = site_events.pop_pair() {
            self.handle_event(
                &key,
                &event,
                &mut active_lines,
                &mut neighbour_priority,
                &mut connected_priority,
                &mut site_events,
                &mut result,
            );
            false
        } else {
            self.sweepline_pos = geo::Coordinate {
                x: T::max_value(),
                y: T::max_value(),
            };

            true
        };

        // put the borrowed containers back
        self.site_events = Some(site_events);
        self.active_lines = Some(active_lines);
        self.result = Some(result);
        self.neighbour_priority = Some(neighbour_priority);
        self.connected_priority = Some(connected_priority);
        Ok(algorithm_is_done)
    }

    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    fn handle_event(
        &mut self,
        key: &SiteEventKey<T>,
        event: &SiteEvent<T>,
        active_lines: &mut ahash::AHashSet<usize>,
        neighbour_priority: &mut MinMax<T>,
        connected_priority: &mut MinMaxSlope<T>,
        site_events: &mut rb_tree::RBMap<SiteEventKey<T>, SiteEvent<T>>,
        result: &mut rb_tree::RBMap<SiteEventKey<T>, Vec<usize>>,
    ) {
        self.sweepline_pos = key.pos;

        let removed_active_lines = event.drop.iter().flatten().count();
        let added_active_lines = event.add.iter().flatten().count();
        let intersections_found = event.intersection.iter().flatten().count();

        #[cfg(feature = "console_trace")]
        println!("*************************************");
        #[cfg(feature = "console_trace")]
        print!(
            "handle_event() sweepline=({:?},{:?})",
            self.sweepline_pos.x, self.sweepline_pos.y,
        );
        #[cfg(feature = "console_trace")]
        print!(
            ", drop={:?}",
            event.drop.iter().flatten().collect::<Vec<&usize>>()
        );
        #[cfg(feature = "console_trace")]
        print!(
            ", add={:?}",
            event.add.iter().flatten().collect::<Vec<&usize>>()
        );
        #[cfg(feature = "console_trace")]
        print!(
            ", intersection={:?}",
            event.intersection.iter().flatten().collect::<Vec<&usize>>()
        );
        #[cfg(feature = "console_trace")]
        println!(
            ", active:{:?}",
            active_lines.iter().collect::<Vec<&usize>>()
        );

        // Handle points converging at this point:
        // If sum of number of items in 'add' + 'drop' > 1 they must intersect at this point
        if self.ignore_end_point_intersections {
            // only report end point intersections if they collide with
            // a calculated intersection point
            if intersections_found > 0 {
                if removed_active_lines > 0 {
                    self.report_intersections_to_result(
                        result,
                        &self.sweepline_pos.clone(),
                        event.drop.iter().flatten(),
                    );
                }
                if added_active_lines > 0 {
                    self.report_intersections_to_result(
                        result,
                        &self.sweepline_pos.clone(),
                        event.add.iter().flatten(),
                    );
                }
                self.report_intersections_to_result(
                    result,
                    &self.sweepline_pos.clone(),
                    event.intersection.iter().flatten(),
                );
            }
        } else if removed_active_lines + added_active_lines + intersections_found > 1 {
            // report *all* intersections, including the end-to-end intersections
            if removed_active_lines > 0 {
                self.report_intersections_to_result(
                    result,
                    &self.sweepline_pos.clone(),
                    event.drop.iter().flatten(),
                );
            }
            if added_active_lines > 0 {
                self.report_intersections_to_result(
                    result,
                    &self.sweepline_pos.clone(),
                    event.add.iter().flatten(),
                );
            }
            if intersections_found > 0 {
                self.report_intersections_to_result(
                    result,
                    &self.sweepline_pos.clone(),
                    event.intersection.iter().flatten(),
                );
            }
        }

        // remove 'drop' lines from the active lines
        for line_index in event.drop.iter().flatten() {
            let _ = active_lines.remove(line_index);
        }

        //
        // calculate the sweep-line status for this point (aka the 'leftright' struct)
        //
        neighbour_priority.clear();

        'active_lines: for line_index in active_lines.iter() {
            for i in event.intersection.iter().flatten() {
                // lines intersecting at this point can never be left/right candidates
                if i == line_index {
                    continue 'active_lines;
                }
            }

            //print!("(sweepline_intersection id={:?}", line_index);
            if let Some((intersection_x, intersection_slope)) =
                sweepline_intersection(self.sweepline_pos, self.lines.get(*line_index).unwrap())
            {
                //println!(" @{}^{})", intersection_x, intersection_slope);
                neighbour_priority.update(
                    self.sweepline_pos.x,
                    intersection_x,
                    intersection_slope,
                    *line_index,
                );
            }
        }

        // add the newly found lines
        for l in event.add.iter().flatten() {
            let _ = active_lines.insert(*l);
        }

        //println!("Mid active lines: {:?}", active_lines);

        if intersections_found + added_active_lines == 0 {
            #[cfg(feature = "console_trace")]
            println!(
                "neighbours left: {:?}",
                neighbour_priority.slope.candidates_left
            );
            #[cfg(feature = "console_trace")]
            println!(
                "neighbours right: {:?}",
                neighbour_priority.slope.candidates_right
            );
            // this event didn't spawn off new events, check neighbours
            if !neighbour_priority.slope.candidates_left.is_empty()
                && !neighbour_priority.slope.candidates_right.is_empty()
            {
                self.find_new_events(
                    &neighbour_priority.slope.candidates_left,
                    &neighbour_priority.slope.candidates_right,
                    site_events,
                );
            }
        } else {
            connected_priority.clear();
            for l in event.add.iter().flatten() {
                connected_priority.update_both(*l, &self.lines);
            }
            for l in event.intersection.iter().flatten() {
                connected_priority.update_both(*l, &self.lines);
            }
            #[cfg(feature = "console_trace")]
            println!(
                "left connected_priority candidates {:?}",
                connected_priority.candidates_left
            );
            #[cfg(feature = "console_trace")]
            println!(
                "right connected_priority candidates {:?}",
                connected_priority.candidates_right
            );

            if !neighbour_priority.slope.candidates_left.is_empty() {
                #[cfg(feature = "console_trace")]
                println!(
                    "left neighbour_priority candidates {:?}",
                    neighbour_priority.slope.candidates_left
                );

                self.find_new_events(
                    &neighbour_priority.slope.candidates_left,
                    &connected_priority.candidates_left,
                    site_events,
                );
            }
            if !neighbour_priority.slope.candidates_right.is_empty() {
                #[cfg(feature = "console_trace")]
                println!(
                    "right neighbour_priority candidates {:?}",
                    neighbour_priority.slope.candidates_right
                );

                self.find_new_events(
                    &neighbour_priority.slope.candidates_right,
                    &connected_priority.candidates_right,
                    site_events,
                );
            }
        }
        #[cfg(any(test, example))]
        println!("Post active lines: {:?}", active_lines);
        #[cfg(feature = "console_trace")]
        println!();
    }

    fn find_new_events(
        &mut self,
        left: &[usize],
        right: &[usize],
        site_events: &mut rb_tree::RBMap<SiteEventKey<T>, SiteEvent<T>>,
    ) {
        for left_i in left.iter() {
            for right_i in right.iter() {
                let left_l = &self.lines[*left_i];
                let right_l = &self.lines[*right_i];
                if ulps_eq_c(&left_l.end, &right_l.end) {
                    // if endpoints are equal they will already be in the event queue
                    continue;
                }
                #[cfg(feature = "console_trace")]
                print!("testing intersection between {} and {}: ", left_i, right_i);
                self.intersection_calls += 1;
                if let Some(intersection_p) = intersect(left_l, right_l) {
                    let intersection_p = intersection_p.single();
                    // don't allow intersection 'behind' or 'at' current sweep-line position
                    if intersection_p.y >= self.sweepline_pos.y
                        && !(intersection_p.y == self.sweepline_pos.y
                            && intersection_p.x < self.sweepline_pos.x)
                        && !ulps_eq_c(&intersection_p, &self.sweepline_pos)
                    {
                        #[cfg(feature = "console_trace")]
                        println!(
                            "Lines {:?} and {:?} intersects at {:?}",
                            left_i, right_i, intersection_p
                        );

                        self.add_intersection_event(
                            site_events,
                            &SiteEventKey {
                                pos: intersection_p,
                            },
                            &[*left_i, *right_i],
                        )
                    }
                } else {
                    #[cfg(feature = "console_trace")]
                    println!(" no intersection");
                }
            }
        }
    }

    fn report_intersections_to_result<'a, I>(
        &mut self,
        result: &mut rb_tree::RBMap<SiteEventKey<T>, Vec<usize>>,
        pos: &geo::Coordinate<T>,
        intersecting_lines: I,
    ) where
        I: Iterator<Item = &'a usize>,
    {
        let key = SiteEventKey { pos: *pos };

        let value = if let Some(value) = result.get_mut(&key) {
            value
        } else {
            let _ = result.insert(key, Vec::default());
            result.get_mut(&key).unwrap()
        };

        for line in intersecting_lines {
            value.push(*line);
            #[cfg(feature = "console_trace")]
            println!("Reported an intersection {:?} for line #{}", pos, line);
        }
        value.sort_unstable();
    }

    #[cfg(feature = "console_trace")]
    fn debug(&self) {
        println!("Stored item in this order:");
        for k in self.site_events.as_ref().unwrap().iter().map(|kv| *kv.0) {
            print!("{:?}, ", k);
        }
        println!();
    }
}
