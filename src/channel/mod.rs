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
    bytes[0] = (frequency - 40000000) as u8;
    bytes[1] = (((frequency - 40000000) >> 8) + 0x5A) as u8;
    bytes[2] = (((frequency - 40000000) >> 16) + 0x62) as u8;

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
            _unknown3: value & 0x04 != 0,
            bandwidth: if value & 0x02 == 0 {
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
            | (value._unknown3 as u8) << 2
            | ((value.bandwidth == Bandwidth::Narrow) as u8) << 1
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
            [0x1a, 0x80] => Ok(Dcs::D032N),
            [0x27, 0x80] => Ok(Dcs::D047N),
            [0x29, 0x80] => Ok(Dcs::D051N),
            [0x2b, 0x80] => Ok(Dcs::D053N),
            [0x35, 0x80] => Ok(Dcs::D065N),
            [0x4e, 0x80] => Ok(Dcs::D116N),
            [0xa3, 0x80] => Ok(Dcs::D243N),
            [0x86, 0x81] => Ok(Dcs::D606N),
            [0xa3, 0xc0] => Ok(Dcs::D243I),
            [0xda, 0xc0] => Ok(Dcs::D332I),
            [0xd9, 0xc1] => Ok(Dcs::D731I),
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
            Dcs::D032N => vec![0x1a, 0x80],
            Dcs::D047N => vec![0x27, 0x80],
            Dcs::D051N => vec![0x29, 0x80],
            Dcs::D053N => vec![0x2b, 0x80],
            Dcs::D065N => vec![0x35, 0x80],
            Dcs::D116N => vec![0x4e, 0x80],
            Dcs::D243N => vec![0xa3, 0x80],
            Dcs::D423N => vec![0x13, 0x81],
            Dcs::D606N => vec![0x86, 0x81],
            Dcs::D743N => vec![0xe3, 0x81],
            Dcs::D243I => vec![0xa3, 0xc0],
            Dcs::D332I => vec![0xda, 0xc0],
            Dcs::D516I => vec![0x4e, 0xc1],
            Dcs::D731I => vec![0xd9, 0xc1],
            Dcs::D743I => vec![0xe3, 0xc1],
            Dcs::D754I => vec![0xec, 0xc1],
            _ => unimplemented!(),
        }
    }
}
