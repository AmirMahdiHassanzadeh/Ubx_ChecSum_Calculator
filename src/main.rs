pub(crate) fn main()
{
    let  example =[  0xB5,0x62,0x01, 
                                0x07,0x54,0x00, 
                                0xBC,0xB5,0x4B, 
                                0x0B,0xE2,0x07, 
                                0x06,0x05,0x04, 
                                0x26,0x0D,0x07, 
                                0x04,0x00,0x00, 
                                0x00,0x8A,0xCA, 
                                0xF7,0x05,0x03, 
                                0x01,0x0A,0x0C, 
                                0x97,0x4A,0x5A,
                                0x57,0xA0,0xD3, 
                                0x3B,0xE9,0xF8, 
                                0xD5,0x00,0x00,
                                0xC6,0xD8,0x00,
                                0x00,0x59,0x05,
                                0x00,0x00,0x1E,
                                0x07,0x00,0x00, 
                                0x09,0x00,0x00, 
                                0x00,0xDB,0xFF, 
                                0xFF,0xFF,0xF5,
                                0xFF,0xFF,0xFF,
                                0x26,0x00,0x00,
                                0x00,0x70,0x85,
                                0x1B,0x01,0xF5,
                                0x00,0x00,0x00, 
                                0x4A,0x28,0x50,
                                0x00,0x90,0x00,
                                0x00,0x00,0x84,
                                0xD3,0x11,0x00,
                                0x93,0x4F];
let mut ck_a  = 0;
let mut ck_b  = 0;
let len =example.len();
//-----------------------------------------------
for i in 2..len-2 {
   // if i == 0 || i == 1 || i == len-2 || i ==len-1{continue;}
   ck_a = ck_a + example[i];
   ck_b = ck_b + ck_a;
}
ck_a = ck_a & 0xFF;
ck_b = ck_b & 0xFF;
//-----------------------------------------------
 println!("{:#x}",ck_a);
 println!("{:#x}",ck_b);
}

