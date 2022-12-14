fn main() -> Result<(), String> {
    #[cfg(feature = "traceroute")]
    {
        use e_utils::traceroute::Traceroute;
        let target = "114.114.114.114".to_owned();
        let tracert = Traceroute::new(target, None)?;
        println!("{}\n", tracert.get_info());
        for hop in tracert {
            print!("{}", hop.ttl);
            for query_result in &hop.query_result {
                print!(
                    " \t{}ms \t{:?}",
                    query_result.rtt.as_millis(),
                    query_result.addr
                );
            }
            print!("\n");
        }
    }
    Ok(())
}
