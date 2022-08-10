fn main() {
    #[cfg(feature = "sys_info")]
    {
        use e_utils::info::Info;
        let info = Info::new();
        println!("info {:?}", info.get_monitor());
    }
}
