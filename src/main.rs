use std::collections::{BTreeMap, HashMap};

const DAYSINMONTH: [[u16; 12]; 2] = [[30, 31, 30, 31, 31, 28, 31, 30, 31, 30, 31, 31], [30, 31, 30, 31, 31, 29, 31, 30, 31, 30, 31, 31]];
const MONTHS: [&str; 12] = ["September", "October", "November", "December", "January", "February", "March", "April", "May", "June", "July", "August"];


fn main() {

    for year in 1984..1990 {
        let leap_year = match (year + 1) % 4 {
            0 => 1,
            _ => 0,
        };
        let eldest_jan = test_leap(DAYSINMONTH[leap_year]);


        if eldest_jan.len() > 0 {

            //        println!("Of these, the group whose eldest was born in January {:?}", eldest_jan);
            //        println!("All born on day {} of the month and day {} of the week", eldest_jan[0].0.0 + 1, eldest_jan[0].0.1 + 1);
            println!("The youngest was born on {} of {:?} {}", eldest_jan[0].0.0 + 1, MONTHS[*(eldest_jan[0].1.iter().last().unwrap())], year - 4);
                
        }
            

    }


}

fn test_leap(dim: [u16; 12]) -> Vec<((usize, u16), Vec<usize>)> {

    let weekday_offset_month = dim.iter().enumerate().map(|(i, &v)| [dim[..i].iter().sum::<u16>() % 7, v]).collect::<Vec<[u16; 2]>>();
    let weekday_offset_day = weekday_offset_month.iter().map(|[m, d]| (0..*d as usize).map(|md| (md as u16 + m) % 7).collect()).collect::<Vec<Vec<u16>>>();
    let dates_on_weekday = weekday_offset_day.iter().map(|month| (0..7)
                                            .map(|d| month.iter()
                                                .enumerate().filter(|(_i, x)| x == &&d)
                                                .map(|(i, v)| (i, *v))
                                                .collect::<Vec<(usize, u16)>>())
                                                .flatten()
                                            .collect::<Vec<(usize, u16)>>())
                                        .collect::<Vec<Vec<(usize, u16)>>>();

    let mut day_dict = HashMap::<(usize, u16), Vec<usize>>::new();
    for (i, m) in dates_on_weekday.iter().enumerate() {
        for d_d in m.iter() {
//            print!("{:?} ", d_d);    
            day_dict.entry(*d_d).or_insert(Vec::<usize>::new()).push(i);        
        }
    }
    let mut sorted = BTreeMap::<usize, Vec<((usize, u16), Vec<usize>)>>::new();
    for (k, v) in day_dict.into_iter() {
        let l = v.len();
        sorted.entry(l).or_insert(Vec::<((usize, u16), Vec<usize>)>::new()).push((k, v));        
    }

    let md_gp = match sorted.last_key_value() {
        Some(g) => (g.1).to_owned().into_iter().filter(|(d_d, _ms)| d_d.0 == g.0 - 1)
                                                                .collect::<Vec<((usize, u16), Vec<usize>)>>(),
        None => Vec::<((usize, u16), Vec<usize>)>::new(),
    };

    md_gp.into_iter().filter(|(_d_d, ms)| ms[0] == 4).collect::<Vec<((usize, u16), Vec<usize>)>>()

}