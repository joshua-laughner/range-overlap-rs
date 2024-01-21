#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RangeOverlap {
    /// The second range is fully within the first, meaning that all values from the second are also in the first
    AContainsB,

    /// The first range is fully within the second, meaning that all values from the first are also in the second
    AInsideB,

    /// The end of the first range overlaps with the start of the second. This does *not* includes the case where
    /// the start values are the same in both ranges and the first range ends before the second (that is considered
    /// `AContainsB`). Note that the end value of the first must be greater than (not equal to) the start value of the 
    /// second, as range end values are assumed to be exclusive.
    AEndsInB,

    /// The start of the first range overlaps the end of the second. This does *not* include the case where the
    /// end values are the same in both ranges and the first starts after the second (that is considered `AContainsB`). 
    /// Note that the end value of the second must be greater than (not equal to) the start value of the first, as end 
    /// values are assumed to be exclusive.
    AStartsInB,

    /// The bounds of both ranges are exactly the same.
    AEqualsB,

    /// There is no overlap between the two ranges; if the end value of one range equals the start value
    /// of another, that is no overlap because ranges are assumed to be exclusive.
    None
}

impl RangeOverlap {
    pub fn has_overlap(&self) -> bool {
        if let Self::None = self {
            false
        } else {
            true
        }
    }
}

pub fn classify<T: PartialOrd>(a_start: T, a_end: T, b_start: T, b_end: T) -> RangeOverlap {
    if a_start == b_start && a_end == b_end {
        RangeOverlap::AEqualsB
    } else if a_start <= b_start && a_end >= b_end {
        RangeOverlap::AContainsB
    } else if a_start < b_start && a_end > b_start && a_end <= b_end {
        RangeOverlap::AEndsInB
    } else if a_start > b_start && a_start < b_end && a_end > b_end {
        RangeOverlap::AStartsInB
    } else if a_start >= b_end || b_start >= a_end {
        RangeOverlap::None
    } else {
        RangeOverlap::AInsideB
    }
}

pub fn classify_open<T: PartialOrd>(a_start: Option<T>, a_end: Option<T>, b_start: Option<T>, b_end: Option<T>) -> RangeOverlap {
    match (a_start, a_end, b_start, b_end) {
        (None, None, None, None) => RangeOverlap::AEqualsB,
        (None, None, None, Some(_)) => RangeOverlap::AContainsB,
        (None, None, Some(_), None) => RangeOverlap::AContainsB,
        (None, None, Some(_), Some(_)) => RangeOverlap::AContainsB,
        (None, Some(_), None, None) => RangeOverlap::AEndsInB,
        (None, Some(ea), None, Some(eb)) => {
            if ea == eb {
                RangeOverlap::AEqualsB
            } else if ea < eb {
                RangeOverlap::AInsideB
            } else {
                RangeOverlap::AContainsB
            }
        },
        (None, Some(ea), Some(sa), None) => {
            if ea <= sa {
                RangeOverlap::None
            } else {
                RangeOverlap::AEndsInB
            }
        },
        (None, Some(ea), Some(sb), Some(eb)) => {
            if ea <= sb {
                RangeOverlap::None
            } else if ea > sb && ea < eb {
                RangeOverlap::AEndsInB
            } else {
                RangeOverlap::AContainsB
            }
        },
        (Some(_), None, None, None) => RangeOverlap::AStartsInB,
        (Some(sa), None, None, Some(eb)) => {
            if sa >= eb {
                RangeOverlap::None
            } else {
                RangeOverlap::AStartsInB
            }
        },
        (Some(sa), None, Some(sb), None) => {
            if sa == sb {
                RangeOverlap::AEqualsB
            } else if sa < sb {
                RangeOverlap::AContainsB
            } else {
                RangeOverlap::AInsideB
            }
        },
        (Some(sa), None, Some(sb), Some(eb)) => {
            if sa <= sb {
                RangeOverlap::AContainsB
            } else if sa < eb {
                RangeOverlap::AStartsInB
            } else {
                RangeOverlap::None
            }
        },
        (Some(_), Some(_), None, None) => RangeOverlap::AInsideB,
        (Some(sa), Some(ea), None, Some(eb)) => {
            if eb <= sa {
                RangeOverlap::None
            } else if ea <= eb {
                RangeOverlap::AInsideB
            } else {
                RangeOverlap::AStartsInB
            }
        },
        (Some(sa), Some(ea), Some(sb), None) => {
            if sb >= ea {
                RangeOverlap::None
            } else if sa >= sb {
                RangeOverlap::AInsideB
            } else {
                RangeOverlap::AEndsInB
            }
        },
        (Some(sa), Some(ea), Some(sb), Some(eb)) => {
            classify(sa, ea, sb, eb)
        }
    }
}

pub fn open_ranges_overlap<T: PartialOrd>(a_start: Option<T>, a_end: Option<T>, b_start: Option<T>, b_end: Option<T>) -> bool {
    classify_open(a_start, a_end, b_start, b_end).has_overlap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_range_overlap_bool() {
        let r1_start = 1;
        let r1_end = 20;
        let r2_before = -20;
        let r2_before2 = -10;
        let r2_between = 10;
        let r2_after = 30;
        let r2_after2 = 40;

        // Test when both ranges are open ended, making sure that the result is symmetrical
        assert_eq!(open_ranges_overlap(Some(r1_start), None, Some(r2_before), None), true);
        assert_eq!(open_ranges_overlap(Some(r1_start), None, Some(r2_between), None), true);
        assert_eq!(open_ranges_overlap(Some(r1_start), None, Some(r2_after), None), true);

        assert_eq!(open_ranges_overlap(Some(r2_before), None, Some(r1_start), None), true);
        assert_eq!(open_ranges_overlap(Some(r2_between), None, Some(r1_start), None), true);
        assert_eq!(open_ranges_overlap(Some(r2_after), None, Some(r1_start), None), true);

        // Test when one range has an end date - the only non-overlapping cases should be
        // when the start date of the open ended range is after the end date of the closed
        // range.
        assert_eq!(open_ranges_overlap(Some(r1_start), Some(r1_end), Some(r2_before), None), true);
        assert_eq!(open_ranges_overlap(Some(r1_start), Some(r1_end), Some(r2_between), None), true);
        assert_eq!(open_ranges_overlap(Some(r1_start), Some(r1_end), Some(r2_after), None), false);

        assert_eq!(open_ranges_overlap(Some(r2_before), None, Some(r1_start), Some(r1_end)), true);
        assert_eq!(open_ranges_overlap(Some(r2_between), None, Some(r1_start), Some(r1_end)), true);
        assert_eq!(open_ranges_overlap(Some(r2_after), None, Some(r1_start), Some(r1_end)), false);

        // Test when both ranges have end dates - the non-overlapping cases should be 
        // when either ranges' start date is after the other one's end date
        assert_eq!(open_ranges_overlap(Some(r1_start), Some(r1_end), Some(r2_before), Some(r2_before2)), false);
        assert_eq!(open_ranges_overlap(Some(r1_start), Some(r1_end), Some(r2_before), Some(r2_between)), true);
        assert_eq!(open_ranges_overlap(Some(r1_start), Some(r1_end), Some(r2_between), Some(r2_after)), true);
        assert_eq!(open_ranges_overlap(Some(r1_start), Some(r1_end), Some(r2_after), Some(r2_after2)), false);

        assert_eq!(open_ranges_overlap(Some(r2_before), Some(r2_before2), Some(r1_start), Some(r1_end)), false);
        assert_eq!(open_ranges_overlap(Some(r2_before), Some(r2_between), Some(r1_start), Some(r1_end)), true);
        assert_eq!(open_ranges_overlap(Some(r2_between), Some(r2_after), Some(r1_start), Some(r1_end)), true);
        assert_eq!(open_ranges_overlap(Some(r2_after), Some(r2_after2), Some(r1_start), Some(r1_end)), false);
    }

    #[test]
    fn test_open_range_overlap_classification() {
        // A == B
        assert_eq!(classify_open(Some(1), Some(10), Some(1), Some(10)), RangeOverlap::AEqualsB);
        assert_eq!(classify_open(None, Some(10), None, Some(10)), RangeOverlap::AEqualsB);
        assert_eq!(classify_open(Some(1), None, Some(1), None), RangeOverlap::AEqualsB);
        assert_eq!(classify_open::<i32>(None, None, None, None), RangeOverlap::AEqualsB);

        // A contains B
        assert_eq!(classify_open(Some(1), Some(100), Some(50), Some(60)), RangeOverlap::AContainsB);
        assert_eq!(classify_open(None, Some(100), Some(50), Some(60)), RangeOverlap::AContainsB);
        assert_eq!(classify_open(Some(1), None, Some(50), Some(60)), RangeOverlap::AContainsB);
        assert_eq!(classify_open(None, None, Some(50), Some(60)), RangeOverlap::AContainsB);
        assert_eq!(classify_open(None, Some(100), None, Some(60)), RangeOverlap::AContainsB);
        assert_eq!(classify_open(Some(1), None, Some(50), None), RangeOverlap::AContainsB);
        assert_eq!(classify_open(None, None, None, Some(60)), RangeOverlap::AContainsB);
        assert_eq!(classify_open(None, None, Some(50), None), RangeOverlap::AContainsB);

        // (edge cases with equal start or end values)
        assert_eq!(classify_open(None, Some(50), Some(1), Some(50)), RangeOverlap::AContainsB);
        assert_eq!(classify_open(Some(1), None, Some(1), Some(50)), RangeOverlap::AContainsB);
        assert_eq!(classify_open(Some(10), Some(50), Some(10), Some(20)), RangeOverlap::AContainsB);
        assert_eq!(classify_open(Some(10), Some(50), Some(40), Some(50)), RangeOverlap::AContainsB);

        // A inside B
        assert_eq!(classify_open(Some(50), Some(60), Some(1), Some(100)), RangeOverlap::AInsideB);
        assert_eq!(classify_open(Some(50), Some(60), None, Some(100)), RangeOverlap::AInsideB);
        assert_eq!(classify_open(Some(50), Some(60), Some(1), None), RangeOverlap::AInsideB);
        assert_eq!(classify_open(None, Some(60), None, Some(100)), RangeOverlap::AInsideB);
        assert_eq!(classify_open(Some(50), None, Some(1), None), RangeOverlap::AInsideB);

        // (edge cases with equal start or end dates)
        assert_eq!(classify_open(Some(1), Some(50), Some(1), None), RangeOverlap::AInsideB);
        assert_eq!(classify_open(Some(1), Some(50), None, Some(50)), RangeOverlap::AInsideB);

        // These are cases that showed up when using similar code with dates,
        // so converted dates to day-of-year for this test
        //  Original dates: 2017-1-1, 2017-12-01, 2017-1-1, None
        assert_eq!(classify_open(Some(1), Some(335), Some(1), None), RangeOverlap::AInsideB);
        //  Original date: 2004-12-01, 2005-01-01, 2004-07-01, 2005-01-01
        assert_eq!(classify_open(Some(336), Some(366), Some(183), Some(366)), RangeOverlap::AInsideB);

        // A ends in B
        assert_eq!(classify_open(Some(1), Some(75), Some(25), Some(99)), RangeOverlap::AEndsInB);
        assert_eq!(classify_open(None, Some(75), Some(25), Some(99)), RangeOverlap::AEndsInB);
        assert_eq!(classify_open(None, Some(75), Some(25), None), RangeOverlap::AEndsInB);
        assert_eq!(classify_open(None, Some(75), None, None), RangeOverlap::AEndsInB);

        // A starts in B
        assert_eq!(classify_open(Some(50), Some(99), Some(1), Some(75)), RangeOverlap::AStartsInB);
        assert_eq!(classify_open(Some(50), None, Some(1), Some(75)), RangeOverlap::AStartsInB);
        assert_eq!(classify_open(Some(50), None, None, Some(75)), RangeOverlap::AStartsInB);
        assert_eq!(classify_open(Some(50), None, None, None), RangeOverlap::AStartsInB);

        // No overlap
        assert_eq!(classify_open(Some(1), Some(25), Some(50), Some(75)), RangeOverlap::None);
        assert_eq!(classify_open(Some(50), Some(75), Some(1), Some(25)), RangeOverlap::None);
        assert_eq!(classify_open(None, Some(25), Some(50), Some(99)), RangeOverlap::None);
        assert_eq!(classify_open(Some(1), Some(25), Some(50), None), RangeOverlap::None);

    }

}
