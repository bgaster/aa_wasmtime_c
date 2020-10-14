use anyhow::{Result, anyhow};
use curl::easy::Easy;

pub fn get_string(url: &str) -> Result<String> {
    get_vec(url).map_or(
        Err(anyhow!("Get URL failed")),
        |v| String::from_utf8(v).map_or(Err(anyhow!("From UTF8")), |s| Ok(s)))
}

pub fn get_vec(url: &str) -> Result<Vec<u8>> {
 
    // get the html for GUI
    let mut data = Vec::new();
    let mut handle = Easy::new();
 
    //handle.url(url).unwrap();
    if let Err(_) = handle.url(url) {
        return Err(anyhow!("URL failed"));
    }

    {    
        let mut transfer = handle.transfer();
        if let Err(_) = transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }) {
            return Err(anyhow!("Transfer failed"));   
        }

        if let Err(_) = transfer.perform() {
            return Err(anyhow!("Transfer failed"));
        }
    }

    Ok(data)
}