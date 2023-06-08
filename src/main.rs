use skip_list::skip_list::SkipList;
fn main() {
    let mut sl: SkipList<i32, String> = SkipList::new(6);
    sl.insert(6, "111".into());
    sl.insert(7, "222".into());
    let s = sl.remove(&7);
    if let Some(ss) = s {
        println!("{ss}");
    }
    sl.print_list();
}
