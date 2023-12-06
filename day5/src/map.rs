use std::ops::Range;
use std::str::FromStr;

#[derive(Debug)]
pub enum ParseErr {
    InvalidInput,
}

#[derive(PartialEq, Debug)]
struct MappingRange {
    source_range: Range<u64>,
    destination_range_start: u64,
}

#[derive(PartialEq, Debug)]
pub struct Map {
    pub name: String,
    ranges: Vec<MappingRange>,
}

#[derive(PartialEq, Debug)]
pub struct ApplyRangeResult {
    matched: Option<Range<u64>>,
    unmatched: Vec<Range<u64>>,
}

pub fn range_from_start_len(start: u64, len: u64) -> Range<u64> {
    Range {
        start,
        end: start + len,
    }
}

impl MappingRange {
    fn apply(&self, input: u64) -> Option<u64> {
        if !self.source_range.contains(&input) {
            None
        } else {
            Some(self.destination_range_start + (input - self.source_range.start))
        }
    }

    fn apply_range(&self, range: &Range<u64>) -> ApplyRangeResult {
        let range = range.clone();
        let mut unmatched = vec![];
        let mut matched = None;

        // range is fully contained within
        if self.source_range.start <= range.start && self.source_range.end >= range.end {
            matched = Some(range_from_start_len(
                self.apply(range.start).unwrap(),
                range.count() as u64,
            ));
        }
        // range is fully outside
        else if self.source_range.end <= range.start || self.source_range.start >= range.end {
            unmatched.push(range);
        }
        // range is split by this' start
        else if self.source_range.start > range.start && self.source_range.end >= range.end {
            unmatched.push(Range {
                start: range.start,
                end: self.source_range.start,
            });
            matched = Some(Range {
                start: self.apply(self.source_range.start).unwrap(),
                end: self.apply(range.end - 1).unwrap() + 1,
            });
        }
        // range is split by this' end
        else if self.source_range.start <= range.start && self.source_range.end < range.end {
            matched = Some(Range {
                start: self.apply(range.start).unwrap(),
                end: self.apply(self.source_range.end - 1).unwrap() + 1,
            });
            unmatched.push(Range {
                start: self.source_range.end,
                end: range.end,
            });
        }
        // this is fully contained in range
        else if self.source_range.start > range.start && self.source_range.end < range.end {
            matched = Some(Range {
                start: self.apply(self.source_range.start).unwrap(),
                end: self.apply(self.source_range.end - 1).unwrap() + 1,
            });
            unmatched.push(Range {
                start: range.start,
                end: self.source_range.start,
            });
            unmatched.push(Range {
                start: self.source_range.end,
                end: range.end,
            });
        }
        // hopefully this doesn't happen
        else {
            panic!("self: {:?} range: {:?}", self.source_range, range);
        }

        return ApplyRangeResult { matched, unmatched };
    }
}

impl FromStr for MappingRange {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Option<Vec<u64>> = s
            .split(' ')
            .map(|s| s.trim())
            .map(|s| s.parse::<u64>())
            .map(|r| r.ok())
            .collect();
        let nums = nums.ok_or(ParseErr::InvalidInput)?;
        Ok(MappingRange {
            destination_range_start: nums[0],
            source_range: range_from_start_len(nums[1], nums[2]),
        })
    }
}

impl Map {
    pub fn apply(&self, input: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|r| r.apply(input))
            .unwrap_or(input)
    }

    pub fn apply_ranges(&self, ranges: &mut Vec<Range<u64>>) {
        let mut new_ranges: Vec<Range<u64>> = vec![];
        let mut unmatched_ranges = ranges.clone();

        for mapping_range in &self.ranges {
            let old_unmatched = unmatched_ranges.clone();
            unmatched_ranges.clear();
            for unmatched_range in &old_unmatched {
                let mut result = mapping_range.apply_range(unmatched_range);
                unmatched_ranges.append(&mut result.unmatched);
                if result.matched.is_some() {
                    new_ranges.push(result.matched.unwrap());
                }
            }
        }

        new_ranges.append(&mut unmatched_ranges);
        ranges.clear();
        ranges.append(&mut new_ranges);
    }
}

impl TryFrom<Vec<String>> for Map {
    type Error = ParseErr;
    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let name_line = value.get(0).ok_or(ParseErr::InvalidInput)?;
        let name = name_line.split_once(' ').ok_or(ParseErr::InvalidInput)?.0;
        let ranges: Option<Vec<MappingRange>> = value
            .iter()
            .skip(1)
            .map(|s| MappingRange::from_str(s.as_str()))
            .map(|r| r.ok())
            .collect();
        let ranges = ranges.ok_or(ParseErr::InvalidInput)?;

        Ok(Self {
            name: name.to_string(),
            ranges,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, ops::Range, str::FromStr};

    use crate::map::ApplyRangeResult;

    use super::{Map, MappingRange};

    #[test]
    fn mapping_range_from_str() {
        let input = "50 98 2";
        let range = MappingRange::from_str(input);
        assert!(range.is_ok());
        let range = range.unwrap();
        assert_eq!(range.destination_range_start, 50);
        assert_eq!(
            range.source_range,
            Range {
                start: 98,
                end: 100
            }
        );
    }

    #[test]
    fn mapping_range_apply() {
        let input = "50 98 2";
        let range = MappingRange::from_str(input).unwrap();
        assert_eq!(range.apply(97), None);
        assert_eq!(range.apply(98), Some(50));
        assert_eq!(range.apply(99), Some(51));
        assert_eq!(range.apply(100), None);
    }

    #[test]
    fn mapping_range_apply_range() {
        let input = "52 50 48";
        let range = MappingRange::from_str(input).unwrap();
        // fully inside
        assert_eq!(
            range.apply_range(&(50..98)),
            ApplyRangeResult {
                matched: Some(52..100),
                unmatched: vec![],
            }
        );
        // fully outside
        assert_eq!(
            range.apply_range(&(0..50)),
            ApplyRangeResult {
                matched: None,
                unmatched: vec![(0..50)],
            }
        );
        assert_eq!(
            range.apply_range(&(100..150)),
            ApplyRangeResult {
                matched: None,
                unmatched: vec![(100..150)],
            }
        );
        // split by start
        assert_eq!(
            range.apply_range(&(40..60)),
            ApplyRangeResult {
                matched: Some(52..62),
                unmatched: vec![(40..50)],
            }
        );
        // split by end
        assert_eq!(
            range.apply_range(&(80..110)),
            ApplyRangeResult {
                matched: Some(82..100),
                unmatched: vec![(98..110)],
            }
        );
    }

    #[test]
    fn map_from_vec() {
        let input: Vec<String> = vec!["seed-to-soil map:", "50 98 2", "52 50 48"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let map = Map::try_from(input);
        assert!(map.is_ok());
        let map = map.unwrap();
        assert_eq!(
            map,
            Map {
                name: "seed-to-soil".to_string(),
                ranges: vec![
                    MappingRange {
                        destination_range_start: 50,
                        source_range: Range {
                            start: 98,
                            end: 100
                        }
                    },
                    MappingRange {
                        destination_range_start: 52,
                        source_range: Range { start: 50, end: 98 }
                    }
                ]
            }
        );
    }

    #[test]
    fn map_apply() {
        let input: Vec<String> = vec!["seed-to-soil map:", "50 98 2", "52 50 48"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let map = Map::try_from(input);
        let map = map.unwrap();
        for i in 0..50 {
            assert_eq!(map.apply(i), i);
        }
        for i in 50..98 {
            assert_eq!(map.apply(i), i + 2);
        }
        for i in 98..100 {
            assert_eq!(map.apply(i), i - 48);
        }
        for i in 100..150 {
            assert_eq!(map.apply(i), i);
        }
    }

    #[test]
    fn map_apply_range() {
        let input: Vec<String> = vec!["seed-to-soil map:", "50 98 2", "52 50 48"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let map = Map::try_from(input);
        let map = map.unwrap();
        let mut input = vec![(0..150)];
        map.apply_ranges(&mut input);
        let mut input_set = HashSet::new();
        input_set.extend(input);
        let mut expected = HashSet::new();
        expected.extend(vec![(0..50), (52..100), (50..52), (100..150)]);
        assert_eq!(input_set, expected);
    }
}
