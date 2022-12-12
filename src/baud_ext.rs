use pcan_basic::socket::Baudrate;

pub struct NotBaudrate;

pub trait BaudExt: Sized {
    fn try_from_str(value: &str) -> Result<Self, NotBaudrate>;
    fn to_string(&self) -> String;
}

impl BaudExt for Baudrate {
    fn try_from_str(value: &str) -> Result<Baudrate, NotBaudrate> {
        match value {
            "1M" => Ok(Self::Baud1M),
            "800k" => Ok(Self::Baud800K),
            "500k" => Ok(Self::Baud500K),
            "250k" => Ok(Self::Baud250K),
            "125k" => Ok(Self::Baud125K),
            "100k" => Ok(Self::Baud100K),
            "95k" => Ok(Self::Baud95K),
            "83k" => Ok(Self::Baud83),
            "50k" => Ok(Self::Baud50K),
            "47k" => Ok(Self::Baud47K),
            "33k" => Ok(Self::Baud33K),
            "20k" => Ok(Self::Baud20K),
            "10k" => Ok(Self::Baud10K),
            "5k" => Ok(Self::Baud5K),
            _ => Err(NotBaudrate),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::Baud1M => "1M".to_owned(),
            Self::Baud800K => "800k".to_owned(),
            Self::Baud500K => "500k".to_owned(),
            Self::Baud250K => "250k".to_owned(),
            Self::Baud125K => "125k".to_owned(),
            Self::Baud100K => "100k".to_owned(),
            Self::Baud95K => "95k".to_owned(),
            Self::Baud83 => "83k".to_owned(),
            Self::Baud50K => "50k".to_owned(),
            Self::Baud47K => "47k".to_owned(),
            Self::Baud33K => "33k".to_owned(),
            Self::Baud20K => "20k".to_owned(),
            Self::Baud10K => "10k".to_owned(),
            Self::Baud5K => "5k".to_owned(),
        }
    }
}
