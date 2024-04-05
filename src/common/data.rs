pub fn ror(data: &[u8], pwd: &[u8]) -> Vec<u8> {
  let mut ret = Vec::new();
  for i in 0..pwd.len() {
    let x = data[i] ^ pwd[i];
    ret.push((x << 3) + (x >> 5));
  }
  ret
}

pub fn checksum(data: &[u8]) -> [u8; 4] {
  let mut sum = [0u8; 4];
  let len = data.len();
  let mut i = 0;
  while i + 3 < len {
    sum[0] ^= data[i + 3];
    sum[1] ^= data[i + 2];
    sum[2] ^= data[i + 1];
    sum[3] ^= data[i];
    i += 4;
  }
  if i < len {
    let mut tmp = [0u8; 4];
    for j in (0..4).rev() {
      tmp[j] = data[i];
      i += 1;
    }
    for j in 0..4 {
      sum[j] ^= tmp[j];
    }
  }
  let mut big_integer = u32::from_le_bytes(sum) as u64;
  big_integer *= 1968;
  let bytes = big_integer.to_le_bytes();
  let mut i = 0;
  let mut ret = [0u8; 4];
  for j in (0..4).rev() {
    ret[j] = bytes[i];
    i += 1;
  }
  ret
}

pub fn crc(data: &[u8]) -> [u8; 4] {
  let mut sum: u32 = 0;
  let len = data.len();
  let mut i = 0;

  while i + 1 < len {
    let byte1 = data[i + 1] as u32;
    let byte2 = data[i] as u32;
    sum ^= byte1 << 8 | byte2;
    i += 2;
  }

  let mut result: [u8; 4] = [0; 4];
  result[0] = (sum & 0xFF) as u8;
  result[1] = ((sum >> 8) & 0xFF) as u8;
  result[2] = ((sum >> 16) & 0xFF) as u8;
  result[3] = ((sum >> 24) & 0xFF) as u8;

  result
}
