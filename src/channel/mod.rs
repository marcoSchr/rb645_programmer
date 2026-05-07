use log::error;

pub mod default_channels;
#[cfg(test)]
mod tests;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Channel {
    pub rx_frequency: u32,
    pub tx_frequency: u32,
    pub rx_ctcss_dcs: CtcssDcs,
    pub tx_ctcss_dcs: CtcssDcs,
    pub data: ChannelData,
}

pub fn bytes_from_frequency(frequency: u32) -> Vec<u8> {
    let mut bytes = [0, 0, 0];
    bytes[0] = (frequency as i64 - 40000000) as u8;
    bytes[1] = (((frequency as i64 - 40000000) >> 8) as u8).wrapping_add(0x5A);
    bytes[2] = (((frequency as i64 - 40000000) >> 16) as u8).wrapping_add(0x62);

    let calculated_frequency = frequency_from_bytes(&bytes);

    // Sometimes calculated bytes are off by one
    // To avoid this we calculate the frequency from the bytes and check the difference
    // to the requested frequency
    // We have the 8 possible values hardcoded
    // Off by +/- 65536 and or +/- 256
    let difference = frequency as i64 - calculated_frequency as i64;
    match difference {
        65536 => {
            bytes[2] = bytes[2].wrapping_add(1);
        }
        -65536 => {
            bytes[2] = bytes[2].wrapping_sub(1);
        }
        256 => {
            bytes[1] = bytes[1].wrapping_add(1);
        }
        -256 => {
            bytes[1] = bytes[1].wrapping_sub(1);
        }
        65792 => {
            bytes[1] = bytes[1].wrapping_add(1);
            bytes[2] = bytes[2].wrapping_add(1);
        }
        65280 => {
            bytes[1] = bytes[1].wrapping_sub(1);
            bytes[2] = bytes[2].wrapping_add(1);
        }
        -65792 => {
            bytes[1] = bytes[1].wrapping_add(1);
            bytes[2] = bytes[2].wrapping_sub(1);
        }
        -65280 => {
            bytes[1] = bytes[1].wrapping_sub(1);
            bytes[2] = bytes[2].wrapping_sub(1);
        }
        -16711680 => {
            bytes[2] = bytes[2].wrapping_add(1);
        }
        0 => {}
        x => {
            error!("Unexpected difference: {}", x);
        }
    }
    bytes.to_vec()
}

pub fn frequency_from_bytes(bytes: &[u8]) -> u32 {
    if bytes.len() != 3 {
        return 0;
    }
    let mut frequency: i64 = 40000000;
    frequency += bytes[0] as i64;
    frequency += (bytes[1] as i64 - 0x5A) * 256;
    frequency += (bytes[2] as i64 - 0x62) * 65536;
    frequency as u32
}

impl TryFrom<&[u8]> for Channel {
    type Error = ();
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 11 {
            return Err(());
        }
        if value.iter().all(|&x| x == 0xff) {
            return Err(());
        }
        Ok(Channel {
            rx_frequency: frequency_from_bytes(&value[0..3]),
            tx_frequency: frequency_from_bytes(&value[3..6]),
            rx_ctcss_dcs: CtcssDcs::try_from(&value[6..8])?,
            tx_ctcss_dcs: CtcssDcs::try_from(&value[8..10])?,
            data: value[10].into(),
        })
    }
}
impl From<&Channel> for Vec<u8> {
    fn from(value: &Channel) -> Self {
        let mut output = vec![];
        output.extend(bytes_from_frequency(value.rx_frequency));
        output.extend(bytes_from_frequency(value.tx_frequency));
        output.extend(Vec::<u8>::from(&value.rx_ctcss_dcs));
        output.extend(Vec::<u8>::from(&value.tx_ctcss_dcs));
        output.push((&value.data).into());
        output
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
enum Bandwidth {
    #[default]
    Narrow,
    Wide,
}

#[derive(Debug, Default, Eq, PartialEq)]
enum TxPower {
    #[default]
    Low,
    High,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct ChannelData {
    _unknown0: bool,
    _unknown1: bool,
    _unknown2: bool,
    scan_add: bool,
    tx_power: TxPower,
    _unknown3: bool,
    bandwidth: Bandwidth,
    busy_lock: bool,
}

impl From<u8> for ChannelData {
    fn from(value: u8) -> Self {
        Self {
            _unknown0: value & 0x80 != 0,
            _unknown1: value & 0x40 != 0,
            _unknown2: value & 0x20 != 0,
            scan_add: value & 0x10 == 0,
            tx_power: if value & 0x08 != 0 {
                TxPower::High
            } else {
                TxPower::Low
            },
            _unknown3: value & 0x02 != 0,
            bandwidth: if value & 0x04 == 0 {
                Bandwidth::Wide
            } else {
                Bandwidth::Narrow
            },
            busy_lock: value & 0x01 == 0,
        }
    }
}

impl From<&ChannelData> for u8 {
    fn from(value: &ChannelData) -> Self {
        (value._unknown0 as u8) << 7
            | (value._unknown1 as u8) << 6
            | (value._unknown2 as u8) << 5
            | ((true ^ value.scan_add) as u8) << 4
            | ((value.tx_power == TxPower::High) as u8) << 3
            | (value._unknown3 as u8) << 1
            | ((value.bandwidth == Bandwidth::Narrow) as u8) << 2
            | ((true ^ value.busy_lock) as u8)
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub enum CtcssDcs {
    Ctcss(Ctcss),
    Dcs(Dcs),
    #[default]
    None,
}

impl TryFrom<&[u8]> for CtcssDcs {
    type Error = ();
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(());
        }
        if value[0] == 0xff && value[1] == 0xff {
            Ok(CtcssDcs::None)
        } else if value[1] & 0x80 == 0x80 {
            Ok(CtcssDcs::Dcs(value.try_into()?))
        } else {
            Ok(CtcssDcs::Ctcss(value.try_into()?))
        }
    }
}

impl From<&CtcssDcs> for Vec<u8> {
    fn from(ctcss_dcs: &CtcssDcs) -> Self {
        match ctcss_dcs {
            CtcssDcs::Ctcss(ctcss) => ctcss.into(),
            CtcssDcs::Dcs(dcs) => dcs.into(),
            CtcssDcs::None => vec![0xff, 0xff],
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Ctcss {
    Ctcss670,
    Ctcss693,
    Ctcss719,
    Ctcss744,
    Ctcss770,
    Ctcss797,
    Ctcss825,
    Ctcss854,
    Ctcss885,
    Ctcss915,
    Ctcss948,
    Ctcss974,
    Ctcss1000,
    Ctcss1035,
    Ctcss1072,
    Ctcss1109,
    Ctcss1148,
    Ctcss1188,
    Ctcss1230,
    Ctcss1273,
    Ctcss1318,
    Ctcss1365,
    Ctcss1413,
    Ctcss1462,
    Ctcss1514,
    Ctcss1567,
    Ctcss1598,
    Ctcss1622,
    Ctcss1655,
    Ctcss1679,
    Ctcss1713,
    Ctcss1738,
    Ctcss1773,
    Ctcss1799,
    Ctcss1835,
    Ctcss1862,
    Ctcss1899,
    Ctcss1928,
    Ctcss1966,
    Ctcss1995,
    Ctcss2035,
    Ctcss2065,
    Ctcss2107,
    Ctcss2181,
    Ctcss2257,
    Ctcss2291,
    Ctcss2336,
    Ctcss2418,
    Ctcss2503,
    Ctcss2541,
}

impl From<&Ctcss> for Vec<u8> {
    fn from(ctcss: &Ctcss) -> Self {
        match ctcss {
            Ctcss::Ctcss670 => vec![0x70, 0x06],
            Ctcss::Ctcss693 => vec![0x93, 0x06],
            Ctcss::Ctcss719 => vec![0x19, 0x07],
            Ctcss::Ctcss744 => vec![0x44, 0x07],
            Ctcss::Ctcss770 => vec![0x70, 0x07],
            Ctcss::Ctcss797 => vec![0x97, 0x07],
            Ctcss::Ctcss825 => vec![0x25, 0x08],
            Ctcss::Ctcss854 => vec![0x54, 0x08],
            Ctcss::Ctcss885 => vec![0x85, 0x08],
            Ctcss::Ctcss915 => vec![0x15, 0x09],
            Ctcss::Ctcss948 => vec![0x48, 0x09],
            Ctcss::Ctcss974 => vec![0x74, 0x09],
            Ctcss::Ctcss1000 => vec![0x00, 0x10],
            Ctcss::Ctcss1035 => vec![0x35, 0x10],
            Ctcss::Ctcss1072 => vec![0x72, 0x10],
            Ctcss::Ctcss1109 => vec![0x09, 0x11],
            Ctcss::Ctcss1148 => vec![0x48, 0x11],
            Ctcss::Ctcss1188 => vec![0x88, 0x11],
            Ctcss::Ctcss1230 => vec![0x30, 0x12],
            Ctcss::Ctcss1273 => vec![0x73, 0x12],
            Ctcss::Ctcss1318 => vec![0x18, 0x13],
            Ctcss::Ctcss1365 => vec![0x65, 0x13],
            Ctcss::Ctcss1413 => vec![0x13, 0x14],
            Ctcss::Ctcss1462 => vec![0x62, 0x14],
            Ctcss::Ctcss1514 => vec![0x14, 0x15],
            Ctcss::Ctcss1567 => vec![0x67, 0x15],
            Ctcss::Ctcss1598 => vec![0x98, 0x15],
            Ctcss::Ctcss1622 => vec![0x22, 0x16],
            Ctcss::Ctcss1655 => vec![0x55, 0x16],
            Ctcss::Ctcss1679 => vec![0x79, 0x16],
            Ctcss::Ctcss1713 => vec![0x13, 0x17],
            Ctcss::Ctcss1738 => vec![0x38, 0x17],
            Ctcss::Ctcss1773 => vec![0x73, 0x17],
            Ctcss::Ctcss1799 => vec![0x99, 0x17],
            Ctcss::Ctcss1835 => vec![0x35, 0x18],
            Ctcss::Ctcss1862 => vec![0x62, 0x18],
            Ctcss::Ctcss1899 => vec![0x99, 0x18],
            Ctcss::Ctcss1928 => vec![0x28, 0x19],
            Ctcss::Ctcss1966 => vec![0x66, 0x19],
            Ctcss::Ctcss1995 => vec![0x95, 0x19],
            Ctcss::Ctcss2035 => vec![0x35, 0x20],
            Ctcss::Ctcss2065 => vec![0x65, 0x20],
            Ctcss::Ctcss2107 => vec![0x07, 0x21],
            Ctcss::Ctcss2181 => vec![0x81, 0x21],
            Ctcss::Ctcss2257 => vec![0x57, 0x22],
            Ctcss::Ctcss2291 => vec![0x91, 0x22],
            Ctcss::Ctcss2336 => vec![0x36, 0x23],
            Ctcss::Ctcss2418 => vec![0x18, 0x24],
            Ctcss::Ctcss2503 => vec![0x03, 0x25],
            Ctcss::Ctcss2541 => vec![0x41, 0x25],
        }
    }
}

impl TryFrom<&[u8]> for Ctcss {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
            [0x70, 0x06] => Ok(Ctcss::Ctcss670),
            [0x93, 0x06] => Ok(Ctcss::Ctcss693),
            [0x19, 0x07] => Ok(Ctcss::Ctcss719),
            [0x44, 0x07] => Ok(Ctcss::Ctcss744),
            [0x70, 0x07] => Ok(Ctcss::Ctcss770),
            [0x97, 0x07] => Ok(Ctcss::Ctcss797),
            [0x25, 0x08] => Ok(Ctcss::Ctcss825),
            [0x54, 0x08] => Ok(Ctcss::Ctcss854),
            [0x85, 0x08] => Ok(Ctcss::Ctcss885),
            [0x15, 0x09] => Ok(Ctcss::Ctcss915),
            [0x48, 0x09] => Ok(Ctcss::Ctcss948),
            [0x74, 0x09] => Ok(Ctcss::Ctcss974),
            [0x00, 0x10] => Ok(Ctcss::Ctcss1000),
            [0x35, 0x10] => Ok(Ctcss::Ctcss1035),
            [0x72, 0x10] => Ok(Ctcss::Ctcss1072),
            [0x09, 0x11] => Ok(Ctcss::Ctcss1109),
            [0x48, 0x11] => Ok(Ctcss::Ctcss1148),
            [0x88, 0x11] => Ok(Ctcss::Ctcss1188),
            [0x30, 0x12] => Ok(Ctcss::Ctcss1230),
            [0x73, 0x12] => Ok(Ctcss::Ctcss1273),
            [0x18, 0x13] => Ok(Ctcss::Ctcss1318),
            [0x65, 0x13] => Ok(Ctcss::Ctcss1365),
            [0x13, 0x14] => Ok(Ctcss::Ctcss1413),
            [0x62, 0x14] => Ok(Ctcss::Ctcss1462),
            [0x14, 0x15] => Ok(Ctcss::Ctcss1514),
            [0x67, 0x15] => Ok(Ctcss::Ctcss1567),
            [0x98, 0x15] => Ok(Ctcss::Ctcss1598),
            [0x22, 0x16] => Ok(Ctcss::Ctcss1622),
            [0x55, 0x16] => Ok(Ctcss::Ctcss1655),
            [0x79, 0x16] => Ok(Ctcss::Ctcss1679),
            [0x13, 0x17] => Ok(Ctcss::Ctcss1713),
            [0x38, 0x17] => Ok(Ctcss::Ctcss1738),
            [0x73, 0x17] => Ok(Ctcss::Ctcss1773),
            [0x99, 0x17] => Ok(Ctcss::Ctcss1799),
            [0x35, 0x18] => Ok(Ctcss::Ctcss1835),
            [0x62, 0x18] => Ok(Ctcss::Ctcss1862),
            [0x99, 0x18] => Ok(Ctcss::Ctcss1899),
            [0x28, 0x19] => Ok(Ctcss::Ctcss1928),
            [0x66, 0x19] => Ok(Ctcss::Ctcss1966),
            [0x95, 0x19] => Ok(Ctcss::Ctcss1995),
            [0x35, 0x20] => Ok(Ctcss::Ctcss2035),
            [0x65, 0x20] => Ok(Ctcss::Ctcss2065),
            [0x07, 0x21] => Ok(Ctcss::Ctcss2107),
            [0x81, 0x21] => Ok(Ctcss::Ctcss2181),
            [0x57, 0x22] => Ok(Ctcss::Ctcss2257),
            [0x91, 0x22] => Ok(Ctcss::Ctcss2291),
            [0x36, 0x23] => Ok(Ctcss::Ctcss2336),
            [0x18, 0x24] => Ok(Ctcss::Ctcss2418),
            [0x03, 0x25] => Ok(Ctcss::Ctcss2503),
            [0x41, 0x25] => Ok(Ctcss::Ctcss2541),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Dcs {
    D023N,
    D025N,
    D026N,
    D031N,
    D032N,
    D036N,
    D043N,
    D047N,
    D051N,
    D053N,
    D054N,
    D065N,
    D071N,
    D072N,
    D073N,
    D074N,
    D114N,
    D115N,
    D116N,
    D122N,
    D125N,
    D131N,
    D132N,
    D134N,
    D143N,
    D145N,
    D152N,
    D155N,
    D156N,
    D162N,
    D165N,
    D172N,
    D174N,
    D205N,
    D212N,
    D223N,
    D225N,
    D226N,
    D243N,
    D244N,
    D245N,
    D246N,
    D251N,
    D252N,
    D255N,
    D261N,
    D263N,
    D265N,
    D266N,
    D271N,
    D274N,
    D306N,
    D311N,
    D315N,
    D325N,
    D331N,
    D332N,
    D343N,
    D346N,
    D351N,
    D356N,
    D364N,
    D365N,
    D371N,
    D411N,
    D412N,
    D413N,
    D423N,
    D431N,
    D432N,
    D445N,
    D446N,
    D452N,
    D454N,
    D455N,
    D462N,
    D464N,
    D465N,
    D466N,
    D503N,
    D506N,
    D516N,
    D523N,
    D526N,
    D532N,
    D546N,
    D565N,
    D606N,
    D612N,
    D624N,
    D627N,
    D631N,
    D632N,
    D654N,
    D662N,
    D664N,
    D703N,
    D712N,
    D723N,
    D731N,
    D732N,
    D734N,
    D743N,
    D754N,
    D023I,
    D025I,
    D026I,
    D031I,
    D032I,
    D036I,
    D043I,
    D047I,
    D051I,
    D053I,
    D054I,
    D065I,
    D071I,
    D072I,
    D073I,
    D074I,
    D114I,
    D115I,
    D116I,
    D122I,
    D125I,
    D131I,
    D132I,
    D134I,
    D143I,
    D145I,
    D152I,
    D155I,
    D156I,
    D162I,
    D165I,
    D172I,
    D174I,
    D205I,
    D212I,
    D223I,
    D225I,
    D226I,
    D243I,
    D244I,
    D245I,
    D246I,
    D251I,
    D252I,
    D255I,
    D261I,
    D263I,
    D265I,
    D266I,
    D271I,
    D274I,
    D306I,
    D311I,
    D315I,
    D325I,
    D331I,
    D332I,
    D343I,
    D346I,
    D351I,
    D356I,
    D364I,
    D365I,
    D371I,
    D411I,
    D412I,
    D413I,
    D423I,
    D431I,
    D432I,
    D445I,
    D446I,
    D452I,
    D454I,
    D455I,
    D462I,
    D464I,
    D465I,
    D466I,
    D503I,
    D506I,
    D516I,
    D523I,
    D526I,
    D532I,
    D546I,
    D565I,
    D606I,
    D612I,
    D624I,
    D627I,
    D631I,
    D632I,
    D654I,
    D662I,
    D664I,
    D703I,
    D712I,
    D723I,
    D731I,
    D732I,
    D734I,
    D743I,
    D754I,
}

impl TryFrom<&[u8]> for Dcs {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
            [0x13, 0x80] => Ok(Dcs::D023N),
            [0x15, 0x80] => Ok(Dcs::D025N),
            [0x16, 0x80] => Ok(Dcs::D026N),
            [0x19, 0x80] => Ok(Dcs::D031N),
            [0x1a, 0x80] => Ok(Dcs::D032N),
            [0x1e, 0x80] => Ok(Dcs::D036N),
            [0x23, 0x80] => Ok(Dcs::D043N),
            [0x27, 0x80] => Ok(Dcs::D047N),
            [0x29, 0x80] => Ok(Dcs::D051N),
            [0x2b, 0x80] => Ok(Dcs::D053N),
            [0x2c, 0x80] => Ok(Dcs::D054N),
            [0x35, 0x80] => Ok(Dcs::D065N),
            [0x39, 0x80] => Ok(Dcs::D071N),
            [0x3a, 0x80] => Ok(Dcs::D072N),
            [0x3b, 0x80] => Ok(Dcs::D073N),
            [0x3c, 0x80] => Ok(Dcs::D074N),
            [0x4c, 0x80] => Ok(Dcs::D114N),
            [0x4d, 0x80] => Ok(Dcs::D115N),
            [0x4e, 0x80] => Ok(Dcs::D116N),
            [0x52, 0x80] => Ok(Dcs::D122N),
            [0x55, 0x80] => Ok(Dcs::D125N),
            [0x59, 0x80] => Ok(Dcs::D131N),
            [0x5a, 0x80] => Ok(Dcs::D132N),
            [0x5c, 0x80] => Ok(Dcs::D134N),
            [0x63, 0x80] => Ok(Dcs::D143N),
            [0x65, 0x80] => Ok(Dcs::D145N),
            [0x6a, 0x80] => Ok(Dcs::D152N),
            [0x6d, 0x80] => Ok(Dcs::D155N),
            [0x6e, 0x80] => Ok(Dcs::D156N),
            [0x72, 0x80] => Ok(Dcs::D162N),
            [0x75, 0x80] => Ok(Dcs::D165N),
            [0x7a, 0x80] => Ok(Dcs::D172N),
            [0x7c, 0x80] => Ok(Dcs::D174N),
            [0x85, 0x80] => Ok(Dcs::D205N),
            [0x8a, 0x80] => Ok(Dcs::D212N),
            [0x93, 0x80] => Ok(Dcs::D223N),
            [0x95, 0x80] => Ok(Dcs::D225N),
            [0x96, 0x80] => Ok(Dcs::D226N),
            [0xa3, 0x80] => Ok(Dcs::D243N),
            [0xa4, 0x80] => Ok(Dcs::D244N),
            [0xa5, 0x80] => Ok(Dcs::D245N),
            [0xa6, 0x80] => Ok(Dcs::D246N),
            [0xa9, 0x80] => Ok(Dcs::D251N),
            [0xaa, 0x80] => Ok(Dcs::D252N),
            [0xad, 0x80] => Ok(Dcs::D255N),
            [0xb1, 0x80] => Ok(Dcs::D261N),
            [0xb3, 0x80] => Ok(Dcs::D263N),
            [0xb5, 0x80] => Ok(Dcs::D265N),
            [0xb6, 0x80] => Ok(Dcs::D266N),
            [0xb9, 0x80] => Ok(Dcs::D271N),
            [0xbc, 0x80] => Ok(Dcs::D274N),
            [0xc6, 0x80] => Ok(Dcs::D306N),
            [0xc9, 0x80] => Ok(Dcs::D311N),
            [0xcd, 0x80] => Ok(Dcs::D315N),
            [0xd5, 0x80] => Ok(Dcs::D325N),
            [0xd9, 0x80] => Ok(Dcs::D331N),
            [0xda, 0x80] => Ok(Dcs::D332N),
            [0xe3, 0x80] => Ok(Dcs::D343N),
            [0xe6, 0x80] => Ok(Dcs::D346N),
            [0xe9, 0x80] => Ok(Dcs::D351N),
            [0xee, 0x80] => Ok(Dcs::D356N),
            [0xf4, 0x80] => Ok(Dcs::D364N),
            [0xf5, 0x80] => Ok(Dcs::D365N),
            [0xf9, 0x80] => Ok(Dcs::D371N),
            [0x09, 0x81] => Ok(Dcs::D411N),
            [0x0a, 0x81] => Ok(Dcs::D412N),
            [0x0b, 0x81] => Ok(Dcs::D413N),
            [0x13, 0x81] => Ok(Dcs::D423N),
            [0x19, 0x81] => Ok(Dcs::D431N),
            [0x1a, 0x81] => Ok(Dcs::D432N),
            [0x25, 0x81] => Ok(Dcs::D445N),
            [0x26, 0x81] => Ok(Dcs::D446N),
            [0x2a, 0x81] => Ok(Dcs::D452N),
            [0x2c, 0x81] => Ok(Dcs::D454N),
            [0x2d, 0x81] => Ok(Dcs::D455N),
            [0x32, 0x81] => Ok(Dcs::D462N),
            [0x34, 0x81] => Ok(Dcs::D464N),
            [0x35, 0x81] => Ok(Dcs::D465N),
            [0x36, 0x81] => Ok(Dcs::D466N),
            [0x43, 0x81] => Ok(Dcs::D503N),
            [0x46, 0x81] => Ok(Dcs::D506N),
            [0x4e, 0x81] => Ok(Dcs::D516N),
            [0x53, 0x81] => Ok(Dcs::D523N),
            [0x56, 0x81] => Ok(Dcs::D526N),
            [0x5a, 0x81] => Ok(Dcs::D532N),
            [0x66, 0x81] => Ok(Dcs::D546N),
            [0x75, 0x81] => Ok(Dcs::D565N),
            [0x86, 0x81] => Ok(Dcs::D606N),
            [0x8a, 0x81] => Ok(Dcs::D612N),
            [0x94, 0x81] => Ok(Dcs::D624N),
            [0x97, 0x81] => Ok(Dcs::D627N),
            [0x99, 0x81] => Ok(Dcs::D631N),
            [0x9a, 0x81] => Ok(Dcs::D632N),
            [0xac, 0x81] => Ok(Dcs::D654N),
            [0xb2, 0x81] => Ok(Dcs::D662N),
            [0xb4, 0x81] => Ok(Dcs::D664N),
            [0xc3, 0x81] => Ok(Dcs::D703N),
            [0xca, 0x81] => Ok(Dcs::D712N),
            [0xd3, 0x81] => Ok(Dcs::D723N),
            [0xd9, 0x81] => Ok(Dcs::D731N),
            [0xda, 0x81] => Ok(Dcs::D732N),
            [0xdc, 0x81] => Ok(Dcs::D734N),
            [0xe3, 0x81] => Ok(Dcs::D743N),
            [0xec, 0x81] => Ok(Dcs::D754N),
            [0x13, 0xc0] => Ok(Dcs::D023I),
            [0x15, 0xc0] => Ok(Dcs::D025I),
            [0x16, 0xc0] => Ok(Dcs::D026I),
            [0x19, 0xc0] => Ok(Dcs::D031I),
            [0x1a, 0xc0] => Ok(Dcs::D032I),
            [0x1e, 0xc0] => Ok(Dcs::D036I),
            [0x23, 0xc0] => Ok(Dcs::D043I),
            [0x27, 0xc0] => Ok(Dcs::D047I),
            [0x29, 0xc0] => Ok(Dcs::D051I),
            [0x2b, 0xc0] => Ok(Dcs::D053I),
            [0x2c, 0xc0] => Ok(Dcs::D054I),
            [0x35, 0xc0] => Ok(Dcs::D065I),
            [0x39, 0xc0] => Ok(Dcs::D071I),
            [0x3a, 0xc0] => Ok(Dcs::D072I),
            [0x3b, 0xc0] => Ok(Dcs::D073I),
            [0x3c, 0xc0] => Ok(Dcs::D074I),
            [0x4c, 0xc0] => Ok(Dcs::D114I),
            [0x4d, 0xc0] => Ok(Dcs::D115I),
            [0x4e, 0xc0] => Ok(Dcs::D116I),
            [0x52, 0xc0] => Ok(Dcs::D122I),
            [0x55, 0xc0] => Ok(Dcs::D125I),
            [0x59, 0xc0] => Ok(Dcs::D131I),
            [0x5a, 0xc0] => Ok(Dcs::D132I),
            [0x5c, 0xc0] => Ok(Dcs::D134I),
            [0x63, 0xc0] => Ok(Dcs::D143I),
            [0x65, 0xc0] => Ok(Dcs::D145I),
            [0x6a, 0xc0] => Ok(Dcs::D152I),
            [0x6d, 0xc0] => Ok(Dcs::D155I),
            [0x6e, 0xc0] => Ok(Dcs::D156I),
            [0x72, 0xc0] => Ok(Dcs::D162I),
            [0x75, 0xc0] => Ok(Dcs::D165I),
            [0x7a, 0xc0] => Ok(Dcs::D172I),
            [0x7c, 0xc0] => Ok(Dcs::D174I),
            [0x85, 0xc0] => Ok(Dcs::D205I),
            [0x8a, 0xc0] => Ok(Dcs::D212I),
            [0x93, 0xc0] => Ok(Dcs::D223I),
            [0x95, 0xc0] => Ok(Dcs::D225I),
            [0x96, 0xc0] => Ok(Dcs::D226I),
            [0xa3, 0xc0] => Ok(Dcs::D243I),
            [0xa4, 0xc0] => Ok(Dcs::D244I),
            [0xa5, 0xc0] => Ok(Dcs::D245I),
            [0xa6, 0xc0] => Ok(Dcs::D246I),
            [0xa9, 0xc0] => Ok(Dcs::D251I),
            [0xaa, 0xc0] => Ok(Dcs::D252I),
            [0xad, 0xc0] => Ok(Dcs::D255I),
            [0xb1, 0xc0] => Ok(Dcs::D261I),
            [0xb3, 0xc0] => Ok(Dcs::D263I),
            [0xb5, 0xc0] => Ok(Dcs::D265I),
            [0xb6, 0xc0] => Ok(Dcs::D266I),
            [0xb9, 0xc0] => Ok(Dcs::D271I),
            [0xbc, 0xc0] => Ok(Dcs::D274I),
            [0xc6, 0xc0] => Ok(Dcs::D306I),
            [0xc9, 0xc0] => Ok(Dcs::D311I),
            [0xcd, 0xc0] => Ok(Dcs::D315I),
            [0xd5, 0xc0] => Ok(Dcs::D325I),
            [0xd9, 0xc0] => Ok(Dcs::D331I),
            [0xda, 0xc0] => Ok(Dcs::D332I),
            [0xe3, 0xc0] => Ok(Dcs::D343I),
            [0xe6, 0xc0] => Ok(Dcs::D346I),
            [0xe9, 0xc0] => Ok(Dcs::D351I),
            [0xee, 0xc0] => Ok(Dcs::D356I),
            [0xf4, 0xc0] => Ok(Dcs::D364I),
            [0xf5, 0xc0] => Ok(Dcs::D365I),
            [0xf9, 0xc0] => Ok(Dcs::D371I),
            [0x09, 0xc1] => Ok(Dcs::D411I),
            [0x0a, 0xc1] => Ok(Dcs::D412I),
            [0x0b, 0xc1] => Ok(Dcs::D413I),
            [0x13, 0xc1] => Ok(Dcs::D423I),
            [0x19, 0xc1] => Ok(Dcs::D431I),
            [0x1a, 0xc1] => Ok(Dcs::D432I),
            [0x25, 0xc1] => Ok(Dcs::D445I),
            [0x26, 0xc1] => Ok(Dcs::D446I),
            [0x2a, 0xc1] => Ok(Dcs::D452I),
            [0x2c, 0xc1] => Ok(Dcs::D454I),
            [0x2d, 0xc1] => Ok(Dcs::D455I),
            [0x32, 0xc1] => Ok(Dcs::D462I),
            [0x34, 0xc1] => Ok(Dcs::D464I),
            [0x35, 0xc1] => Ok(Dcs::D465I),
            [0x36, 0xc1] => Ok(Dcs::D466I),
            [0x43, 0xc1] => Ok(Dcs::D503I),
            [0x46, 0xc1] => Ok(Dcs::D506I),
            [0x4e, 0xc1] => Ok(Dcs::D516I),
            [0x53, 0xc1] => Ok(Dcs::D523I),
            [0x56, 0xc1] => Ok(Dcs::D526I),
            [0x5a, 0xc1] => Ok(Dcs::D532I),
            [0x66, 0xc1] => Ok(Dcs::D546I),
            [0x75, 0xc1] => Ok(Dcs::D565I),
            [0x86, 0xc1] => Ok(Dcs::D606I),
            [0x8a, 0xc1] => Ok(Dcs::D612I),
            [0x94, 0xc1] => Ok(Dcs::D624I),
            [0x97, 0xc1] => Ok(Dcs::D627I),
            [0x99, 0xc1] => Ok(Dcs::D631I),
            [0x9a, 0xc1] => Ok(Dcs::D632I),
            [0xac, 0xc1] => Ok(Dcs::D654I),
            [0xb2, 0xc1] => Ok(Dcs::D662I),
            [0xb4, 0xc1] => Ok(Dcs::D664I),
            [0xc3, 0xc1] => Ok(Dcs::D703I),
            [0xca, 0xc1] => Ok(Dcs::D712I),
            [0xd3, 0xc1] => Ok(Dcs::D723I),
            [0xd9, 0xc1] => Ok(Dcs::D731I),
            [0xda, 0xc1] => Ok(Dcs::D732I),
            [0xdc, 0xc1] => Ok(Dcs::D734I),
            [0xe3, 0xc1] => Ok(Dcs::D743I),
            [0xec, 0xc1] => Ok(Dcs::D754I),
            _ => Err(()),
        }
    }
}

impl From<&Dcs> for Vec<u8> {
    fn from(value: &Dcs) -> Self {
        match value {
            Dcs::D023N => vec![0x13, 0x80],
            Dcs::D025N => vec![0x15, 0x80],
            Dcs::D026N => vec![0x16, 0x80],
            Dcs::D031N => vec![0x19, 0x80],
            Dcs::D032N => vec![0x1a, 0x80],
            Dcs::D036N => vec![0x1e, 0x80],
            Dcs::D043N => vec![0x23, 0x80],
            Dcs::D047N => vec![0x27, 0x80],
            Dcs::D051N => vec![0x29, 0x80],
            Dcs::D053N => vec![0x2b, 0x80],
            Dcs::D054N => vec![0x2c, 0x80],
            Dcs::D065N => vec![0x35, 0x80],
            Dcs::D071N => vec![0x39, 0x80],
            Dcs::D072N => vec![0x3a, 0x80],
            Dcs::D073N => vec![0x3b, 0x80],
            Dcs::D074N => vec![0x3c, 0x80],
            Dcs::D114N => vec![0x4c, 0x80],
            Dcs::D115N => vec![0x4d, 0x80],
            Dcs::D116N => vec![0x4e, 0x80],
            Dcs::D122N => vec![0x52, 0x80],
            Dcs::D125N => vec![0x55, 0x80],
            Dcs::D131N => vec![0x59, 0x80],
            Dcs::D132N => vec![0x5a, 0x80],
            Dcs::D134N => vec![0x5c, 0x80],
            Dcs::D143N => vec![0x63, 0x80],
            Dcs::D145N => vec![0x65, 0x80],
            Dcs::D152N => vec![0x6a, 0x80],
            Dcs::D155N => vec![0x6d, 0x80],
            Dcs::D156N => vec![0x6e, 0x80],
            Dcs::D162N => vec![0x72, 0x80],
            Dcs::D165N => vec![0x75, 0x80],
            Dcs::D172N => vec![0x7a, 0x80],
            Dcs::D174N => vec![0x7c, 0x80],
            Dcs::D205N => vec![0x85, 0x80],
            Dcs::D212N => vec![0x8a, 0x80],
            Dcs::D223N => vec![0x93, 0x80],
            Dcs::D225N => vec![0x95, 0x80],
            Dcs::D226N => vec![0x96, 0x80],
            Dcs::D243N => vec![0xa3, 0x80],
            Dcs::D244N => vec![0xa4, 0x80],
            Dcs::D245N => vec![0xa5, 0x80],
            Dcs::D246N => vec![0xa6, 0x80],
            Dcs::D251N => vec![0xa9, 0x80],
            Dcs::D252N => vec![0xaa, 0x80],
            Dcs::D255N => vec![0xad, 0x80],
            Dcs::D261N => vec![0xb1, 0x80],
            Dcs::D263N => vec![0xb3, 0x80],
            Dcs::D265N => vec![0xb5, 0x80],
            Dcs::D266N => vec![0xb6, 0x80],
            Dcs::D271N => vec![0xb9, 0x80],
            Dcs::D274N => vec![0xbc, 0x80],
            Dcs::D306N => vec![0xc6, 0x80],
            Dcs::D311N => vec![0xc9, 0x80],
            Dcs::D315N => vec![0xcd, 0x80],
            Dcs::D325N => vec![0xd5, 0x80],
            Dcs::D331N => vec![0xd9, 0x80],
            Dcs::D332N => vec![0xda, 0x80],
            Dcs::D343N => vec![0xe3, 0x80],
            Dcs::D346N => vec![0xe6, 0x80],
            Dcs::D351N => vec![0xe9, 0x80],
            Dcs::D356N => vec![0xee, 0x80],
            Dcs::D364N => vec![0xf4, 0x80],
            Dcs::D365N => vec![0xf5, 0x80],
            Dcs::D371N => vec![0xf9, 0x80],
            Dcs::D411N => vec![0x09, 0x81],
            Dcs::D412N => vec![0x0a, 0x81],
            Dcs::D413N => vec![0x0b, 0x81],
            Dcs::D423N => vec![0x13, 0x81],
            Dcs::D431N => vec![0x19, 0x81],
            Dcs::D432N => vec![0x1a, 0x81],
            Dcs::D445N => vec![0x25, 0x81],
            Dcs::D446N => vec![0x26, 0x81],
            Dcs::D452N => vec![0x2a, 0x81],
            Dcs::D454N => vec![0x2c, 0x81],
            Dcs::D455N => vec![0x2d, 0x81],
            Dcs::D462N => vec![0x32, 0x81],
            Dcs::D464N => vec![0x34, 0x81],
            Dcs::D465N => vec![0x35, 0x81],
            Dcs::D466N => vec![0x36, 0x81],
            Dcs::D503N => vec![0x43, 0x81],
            Dcs::D506N => vec![0x46, 0x81],
            Dcs::D516N => vec![0x4e, 0x81],
            Dcs::D523N => vec![0x53, 0x81],
            Dcs::D526N => vec![0x56, 0x81],
            Dcs::D532N => vec![0x5a, 0x81],
            Dcs::D546N => vec![0x66, 0x81],
            Dcs::D565N => vec![0x75, 0x81],
            Dcs::D606N => vec![0x86, 0x81],
            Dcs::D612N => vec![0x8a, 0x81],
            Dcs::D624N => vec![0x94, 0x81],
            Dcs::D627N => vec![0x97, 0x81],
            Dcs::D631N => vec![0x99, 0x81],
            Dcs::D632N => vec![0x9a, 0x81],
            Dcs::D654N => vec![0xac, 0x81],
            Dcs::D662N => vec![0xb2, 0x81],
            Dcs::D664N => vec![0xb4, 0x81],
            Dcs::D703N => vec![0xc3, 0x81],
            Dcs::D712N => vec![0xca, 0x81],
            Dcs::D723N => vec![0xd3, 0x81],
            Dcs::D731N => vec![0xd9, 0x81],
            Dcs::D732N => vec![0xda, 0x81],
            Dcs::D734N => vec![0xdc, 0x81],
            Dcs::D743N => vec![0xe3, 0x81],
            Dcs::D754N => vec![0xec, 0x81],
            Dcs::D023I => vec![0x13, 0xc0],
            Dcs::D025I => vec![0x15, 0xc0],
            Dcs::D026I => vec![0x16, 0xc0],
            Dcs::D031I => vec![0x19, 0xc0],
            Dcs::D032I => vec![0x1a, 0xc0],
            Dcs::D036I => vec![0x1e, 0xc0],
            Dcs::D043I => vec![0x23, 0xc0],
            Dcs::D047I => vec![0x27, 0xc0],
            Dcs::D051I => vec![0x29, 0xc0],
            Dcs::D053I => vec![0x2b, 0xc0],
            Dcs::D054I => vec![0x2c, 0xc0],
            Dcs::D065I => vec![0x35, 0xc0],
            Dcs::D071I => vec![0x39, 0xc0],
            Dcs::D072I => vec![0x3a, 0xc0],
            Dcs::D073I => vec![0x3b, 0xc0],
            Dcs::D074I => vec![0x3c, 0xc0],
            Dcs::D114I => vec![0x4c, 0xc0],
            Dcs::D115I => vec![0x4d, 0xc0],
            Dcs::D116I => vec![0x4e, 0xc0],
            Dcs::D122I => vec![0x52, 0xc0],
            Dcs::D125I => vec![0x55, 0xc0],
            Dcs::D131I => vec![0x59, 0xc0],
            Dcs::D132I => vec![0x5a, 0xc0],
            Dcs::D134I => vec![0x5c, 0xc0],
            Dcs::D143I => vec![0x63, 0xc0],
            Dcs::D145I => vec![0x65, 0xc0],
            Dcs::D152I => vec![0x6a, 0xc0],
            Dcs::D155I => vec![0x6d, 0xc0],
            Dcs::D156I => vec![0x6e, 0xc0],
            Dcs::D162I => vec![0x72, 0xc0],
            Dcs::D165I => vec![0x75, 0xc0],
            Dcs::D172I => vec![0x7a, 0xc0],
            Dcs::D174I => vec![0x7c, 0xc0],
            Dcs::D205I => vec![0x85, 0xc0],
            Dcs::D212I => vec![0x8a, 0xc0],
            Dcs::D223I => vec![0x93, 0xc0],
            Dcs::D225I => vec![0x95, 0xc0],
            Dcs::D226I => vec![0x96, 0xc0],
            Dcs::D243I => vec![0xa3, 0xc0],
            Dcs::D244I => vec![0xa4, 0xc0],
            Dcs::D245I => vec![0xa5, 0xc0],
            Dcs::D246I => vec![0xa6, 0xc0],
            Dcs::D251I => vec![0xa9, 0xc0],
            Dcs::D252I => vec![0xaa, 0xc0],
            Dcs::D255I => vec![0xad, 0xc0],
            Dcs::D261I => vec![0xb1, 0xc0],
            Dcs::D263I => vec![0xb3, 0xc0],
            Dcs::D265I => vec![0xb5, 0xc0],
            Dcs::D266I => vec![0xb6, 0xc0],
            Dcs::D271I => vec![0xb9, 0xc0],
            Dcs::D274I => vec![0xbc, 0xc0],
            Dcs::D306I => vec![0xc6, 0xc0],
            Dcs::D311I => vec![0xc9, 0xc0],
            Dcs::D315I => vec![0xcd, 0xc0],
            Dcs::D325I => vec![0xd5, 0xc0],
            Dcs::D331I => vec![0xd9, 0xc0],
            Dcs::D332I => vec![0xda, 0xc0],
            Dcs::D343I => vec![0xe3, 0xc0],
            Dcs::D346I => vec![0xe6, 0xc0],
            Dcs::D351I => vec![0xe9, 0xc0],
            Dcs::D356I => vec![0xee, 0xc0],
            Dcs::D364I => vec![0xf4, 0xc0],
            Dcs::D365I => vec![0xf5, 0xc0],
            Dcs::D371I => vec![0xf9, 0xc0],
            Dcs::D411I => vec![0x09, 0xc1],
            Dcs::D412I => vec![0x0a, 0xc1],
            Dcs::D413I => vec![0x0b, 0xc1],
            Dcs::D423I => vec![0x13, 0xc1],
            Dcs::D431I => vec![0x19, 0xc1],
            Dcs::D432I => vec![0x1a, 0xc1],
            Dcs::D445I => vec![0x25, 0xc1],
            Dcs::D446I => vec![0x26, 0xc1],
            Dcs::D452I => vec![0x2a, 0xc1],
            Dcs::D454I => vec![0x2c, 0xc1],
            Dcs::D455I => vec![0x2d, 0xc1],
            Dcs::D462I => vec![0x32, 0xc1],
            Dcs::D464I => vec![0x34, 0xc1],
            Dcs::D465I => vec![0x35, 0xc1],
            Dcs::D466I => vec![0x36, 0xc1],
            Dcs::D503I => vec![0x43, 0xc1],
            Dcs::D506I => vec![0x46, 0xc1],
            Dcs::D516I => vec![0x4e, 0xc1],
            Dcs::D523I => vec![0x53, 0xc1],
            Dcs::D526I => vec![0x56, 0xc1],
            Dcs::D532I => vec![0x5a, 0xc1],
            Dcs::D546I => vec![0x66, 0xc1],
            Dcs::D565I => vec![0x75, 0xc1],
            Dcs::D606I => vec![0x86, 0xc1],
            Dcs::D612I => vec![0x8a, 0xc1],
            Dcs::D624I => vec![0x94, 0xc1],
            Dcs::D627I => vec![0x97, 0xc1],
            Dcs::D631I => vec![0x99, 0xc1],
            Dcs::D632I => vec![0x9a, 0xc1],
            Dcs::D654I => vec![0xac, 0xc1],
            Dcs::D662I => vec![0xb2, 0xc1],
            Dcs::D664I => vec![0xb4, 0xc1],
            Dcs::D703I => vec![0xc3, 0xc1],
            Dcs::D712I => vec![0xca, 0xc1],
            Dcs::D723I => vec![0xd3, 0xc1],
            Dcs::D731I => vec![0xd9, 0xc1],
            Dcs::D732I => vec![0xda, 0xc1],
            Dcs::D734I => vec![0xdc, 0xc1],
            Dcs::D743I => vec![0xe3, 0xc1],
            Dcs::D754I => vec![0xec, 0xc1],
        }
    }
}
