/// Convert an IPV4 string to a big-endian u32
pub fn ipv4_to_int(s: &str) -> u32
{
    let mut octets = s.split('.').filter_map(|octet| {
        let trimmed = octet.trim();
        if trimmed.is_empty()
        {
            None
        }
        else
        {
            Some(trimmed.parse::<u32>().unwrap_or_default())
        }
    });


    // We shift all elements to store each octet (u8) into a u32
    // Example source:
    // 192.168.1.19
    //
    // Example binary translation as separate `u8`'s:
    // 11000000 10101000 00000001 00010011
    //
    // Example final u32:
    // 11000000_10101000_00000001_00010011
    // 
    // To get this we shift everything into place then add everything.
    let a = octets.next().unwrap_or_default() << 24;
    let b = octets.next().unwrap_or_default() << 16;
    let c = octets.next().unwrap_or_default() << 8;
    let d = octets.next().unwrap_or_default();

    u32::to_be(a + b + c + d)
}

/// Convert a big-endian IPV4 address to a string.
pub fn int_to_ipv4(i: u32) -> String
{
    let i = i.to_le();
    
    // Example source:
    // 11000000_10101000_00000001_00010011
    //
    // Example masks: 
    // 00000000_00000000_00000000_11111111 : octet 1
    // 00000000_00000000_11111111_00000000 : octet 2
    // 00000000_11111111_00000000_00000000 : octet 3
    // 11111111_00000000_00000000_00000000 : octet 4
    //
    // >> 24 to align the highest order element (the first element).
    // We've shifted everything over, so we don't need to mask anything.
    // >> 16 to align the second element, then mask off what we want.
    // >> 8 to align the third octet, then mask what we want.
    // Mask the final octet.

    // 0xFF == 0b1111_1111
    format!(
        "{}.{}.{}.{}",
        i & 0xFF,
        (i >> 08) & 0xFF,
        (i >> 16) & 0xFF,
        i >> 24,
    )
    
}
/// An iterator that produces only the unique elements from the iterator it was
/// called on
struct Unique<'a, I>
where
    I: Iterator<Item = &'a str>,
{
    iter: I,
    seen: Vec<String>,
}

impl<'a, I> Iterator for Unique<'a, I>
where
    I: Iterator<Item = &'a str>,
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item>
    {
        for item in &mut self.iter
        {
            let item_str = item.to_owned();
            if !self.seen.contains(&item_str)
            {
                self.seen.push(item_str);
                return Some(item);
            }
        }
        None
    }
}

/// An extension trait that adds a `unique` method to the `Iterator` trait
trait UniqueIterator<'a>: Iterator<Item = &'a str>
{
    fn unique(self) -> Unique<'a, Self>
    where
        Self: Sized,
    {
        Unique {
            iter: self,
            seen: Vec::new(),
        }
    }
}

impl<'a, I> UniqueIterator<'a> for I where I: Iterator<Item = &'a str> {}
