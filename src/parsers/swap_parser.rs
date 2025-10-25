use base64::{engine::general_purpose, Engine as _};
use byteorder::{LittleEndian, ReadBytesExt};
use serde_json::json;
use std::io::Cursor;
use anyhow::Context;

pub  fn parse_swap_log() -> anyhow::Result<()> {
    let ray_log = std::env::args().nth(2).unwrap_or_else(|| {
        // 預設示範字串（你也可以從命令列傳入）
        "AzWn51r/AAAAAAAAAAAAAAACAAAAAAAAADWn51r/AAAA9l8fZ/4yNACXGFdjJgAAAPVOuwAAAAAA".to_string()
    });

    let data_bytes = general_purpose::STANDARD.decode(&ray_log)?;
    let mut rdr = Cursor::new(&data_bytes);

    // 1 byte tag (u8)
    let tag = rdr.read_u8().context("read tag failed")?;
    // two u64 little-endian (amount_in, minimum_amount_out)
    let amount_in = rdr
        .read_u64::<LittleEndian>()
        .context("read amount_in failed")?;
    let minimum_amount_out = rdr
        .read_u64::<LittleEndian>()
        .context("read minimum_amount_out failed")?;

    // 其餘 bytes（hex 列表）
    let mut remaining = Vec::new();
    while let Ok(b) = rdr.read_u8() {
        remaining.push(format!("{:02X}", b));
    }

    let result = json!({
        "tag": tag,
        "instruction": match tag {
            3 => "SwapBaseIn",
            4 => "SwapBaseOut",
            2 => "SomeOther", // 視版本而定
            _ => "Unknown"
        },
        "amount_in": amount_in,
        "minimum_amount_out": minimum_amount_out,
        "remaining_hex": remaining.join(" ")
    });

    println!("{}", serde_json::to_string_pretty(&result)?);

    Ok(())
}