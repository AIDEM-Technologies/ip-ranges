#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use itertools::Itertools;
    use cidr_utils::cidr::Ipv4Cidr;

    #[test]
    fn soundness_check() {
        let s = include_str!("../datacenter-cidr.txt");
        let cidr = s
            .lines()
            .map(|line| {
                let index = line.find(':').expect("Infallible");
                line[index + 1..].to_string()
            })
            .map(|s| Ipv4Cidr::from_str(&s).expect("Infallible"))
            .collect_vec();
        let range_collection = crate::IpRanges::new_from_cidr(cidr);
        assert!(range_collection.contains_ip(std::net::Ipv4Addr::new(54,192,0,1)));
        assert!(range_collection.contains_ip(std::net::Ipv4Addr::new(54,192,255,254)));
        assert!(range_collection.contains_ip(std::net::Ipv4Addr::new(80,209,252,1)));
        assert!(range_collection.contains_ip(std::net::Ipv4Addr::new(80,209,253,254)));
        assert!(!range_collection.contains_ip(std::net::Ipv4Addr::new(0,0,0,0)));
        assert!(!range_collection.contains_ip(std::net::Ipv4Addr::new(192,168,0,1)));
        assert!(!range_collection.contains_ip(std::net::Ipv4Addr::new(198,135,41,113)));
    }

    #[test]
    fn soundness_check_ipv6() {
        let cidr = vec![cidr_utils::cidr::Ipv6Cidr::from_str("2001:db8::/64").unwrap()];
        let range_collection = crate::IpRangesV6::new_from_cidr(cidr);
        assert!(range_collection.contains_ip(std::net::Ipv6Addr::from_str("2001:0db8:0000:0000:0000:0000:0000:0000").unwrap()));
        assert!(!range_collection.contains_ip(std::net::Ipv6Addr::from_str("2001:0db7:ffff:ffff:ffff:ffff:ffff:ffff").unwrap()));
        assert!(range_collection.contains_ip(std::net::Ipv6Addr::from_str("2001:0db8:0000:0000:ffff:ffff:ffff:ffff").unwrap()));
        assert!(!range_collection.contains_ip(std::net::Ipv6Addr::from_str("2001:0db8:0000:0001:0000:0000:0000:0000").unwrap()));
    }
}