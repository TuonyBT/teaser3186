use std::collections::{BTreeMap, HashMap};

//const DAYSINMONTH: [[u16; 12]; 2] = [[30, 31, 30, 31, 31, 28, 31, 30, 31, 30, 31, 31], [30, 31, 30, 31, 31, 29, 31, 30, 31, 30, 31, 31]];
//const MONTHS: [&str; 12] = ["September", "October", "November", "December", "January", "February", "March", "April", "May", "June", "July", "August"];

//we know that the eldest of interest was born in January, so we don't care about earlier months in the school year
const DAYSINMONTH: [[u16; 8]; 2] = [[31, 28, 31, 30, 31, 30, 31, 31], [31, 29, 31, 30, 31, 30, 31, 31]];
const MONTHS: [&str; 8] = ["January", "February", "March", "April", "May", "June", "July", "August"];

fn main() {

//  All months to be analysed are in the same year, so we count from the first day of the January of the year in question
//  For the eldest of our group of interest to have been born in the 1980's, the earliest year for rising fives is 1985
//  and the latest year must be 1989

//  This initial routine decides whether or not we are dealing with a leap year
//  In fact, if we are considering repeat weekdays on the same day of the month in an arbitrary year, the only reason two years
//  would display similar patterns is that they have different day counts in at least one month, i.e. leap-year or not
//  In the range we are considering, the only year that will give a unique pattern is the only leap-year, so we can infer before we start
//  that the children in question all have their fifth birthdays in 1988

    for year in 1988..1989 {
        let leap_year = match (year) % 4 {
            0 => 1,
            _ => 0,
        };
        let eldest_jan = test_leap(DAYSINMONTH[leap_year]);

        if eldest_jan.len() > 0 {
            println!("The youngest was born on {} of {:?} {}", eldest_jan[0].0.0 + 1, MONTHS[*(eldest_jan[0].1.iter().last().unwrap())], year - 5);
        }
    }
}

//  The main routine takes an array of the number of days in each month, and groups each day according to its day of the week (relative to 1 Jan)
//  and its day of the month
//  It then lists the months in which each combination falls, returning all combinations among the largest groupings whose first month is January

fn test_leap(dim: [u16; 8]) -> Vec<((usize, u16), Vec<usize>)> {

    let dates_on_weekday = 
        // For each month, how many days are left at the end of the month after all complete weeks
        dim.iter().enumerate().map(|(i, &v)| [dim[..i].iter().sum::<u16>() % 7, v])
        //  For each month, by how many weekdays is the first of that month offset from the first of January
                            .map(|[m, d]| (0..d as usize).map(|md| (md as u16 + m) % 7).collect())
        //  For each day in each month, what is the weekday relative to first of January
                            .map(|month: Vec<u16>| (0..7).map(|d| month.iter().enumerate().filter(|(_i, x)| x == &&d)        
        //  list all (day of week, day of month) combinations for each weekday in a month
                        .map(|(i, v)| (i, *v))
                    .collect::<Vec<(usize, u16)>>())
                    .flatten()

        //  list all (day of week, day of month) combinations for each month
                .collect::<Vec<(usize, u16)>>())

        //  list all (day of week, day of month) combinations for all months
        .collect::<Vec<Vec<(usize, u16)>>>();

    //  create a dictionary of all months that contain days with a given (day of week, day of month) combination
    let mut day_dict = HashMap::<(usize, u16), Vec<usize>>::new();

    for (i, m) in dates_on_weekday.iter().enumerate() {
        for d_d in m.iter() {
            day_dict.entry(*d_d).or_insert(Vec::<usize>::new()).push(i);        
        }
    }

    //  sort this dictionary according the the number of months with that combination
    let mut sorted = BTreeMap::<usize, Vec<((usize, u16), Vec<usize>)>>::new();
    for (k, v) in day_dict.into_iter() {
        let l = v.len();
        sorted.entry(l).or_insert(Vec::<((usize, u16), Vec<usize>)>::new()).push((k, v));        
    }

    //  if this sorted dictionary has any entries, return all entries with the largest possible number of months
    let md_gp = match sorted.last_key_value() {
        Some(g) => (g.1).to_owned().into_iter().filter(|(d_d, _ms)| d_d.0 == g.0 - 1)
                                                                .collect::<Vec<((usize, u16), Vec<usize>)>>(),
        None => Vec::<((usize, u16), Vec<usize>)>::new(),
    };
    //  if there are no entries, the puzzle is not solved, but the function will still complete and return an empty vector

    //  from this list, return only those whose first month is January (ms[0] == 0)
    md_gp.into_iter().filter(|(_d_d, ms)| ms[0] == 0).collect::<Vec<((usize, u16), Vec<usize>)>>()

}