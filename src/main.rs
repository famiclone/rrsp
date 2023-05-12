use std::fs::File;
use std::io::{BufReader, Read};
use cpal::traits::HostTrait;

//$06E    2   WORD    (lo, hi) Play speed, in 1/1000000th sec ticks, NTSC (see text)
//$070    8   BYTE    Bankswitch init values (see text, and FDS section)
//$078    2   WORD    (lo, hi) Play speed, in 1/1000000th sec ticks, PAL (see text)
//$07A    1   BYTE    PAL/NTSC bits
//                bit 0: if clear, this is an NTSC tune
//                bit 0: if set, this is a PAL tune
//                bit 1: if set, this is a dual PAL/NTSC tune
//                bits 2-7: reserved, must be 0
//$07B    1   BYTE    Extra Sound Chip Support
//                bit 0: if set, this song uses VRC6 audio
//                bit 1: if set, this song uses VRC7 audio
//                bit 2: if set, this song uses FDS audio
//                bit 3: if set, this song uses MMC5 audio
//                bit 4: if set, this song uses Namco 163 audio
//                bit 5: if set, this song uses Sunsoft 5B audio
//                bit 6: if set, this song uses VT02+ audio
//                bit 7: reserved, must be zero
//$07C    1   BYTE    Reserved for NSF2
//$07D    3   BYTES   24-bit length of contained program data.
//                If 0, all data until end of file is part of the program.
//                If used, can be used to provide NSF2 metadata
//                in a backward compatible way.
//$080    nnn ----    The music program/data follows

struct NSFHeader {
    format_file: [u8; 5], // 0x00 'N','E','S','M',$1A (denotes an NES sound format file)
    version_number: u8,   // Version number $01 (or $02 for NSF2)
    total_songs: u8,      // Total songs   (1=1 song, 2=2 songs, etc)
    starting_song: u8,    // Starting song (1=1st song, 2=2nd song, etc)
    load_address: [u8; 2], // (lo, hi) Load address of data (8000-FFFF)
    init_address: [u8; 2], // (lo, hi) Init address of data (8000-FFFF)
    play_address: [u8; 2], // (lo, hi) Play address of data (8000-FFFF)
    songname: [u8; 32],   // The name of the song, null terminated
    artist: [u8; 32],     // The artist, null terminated
    copyright: [u8; 32],  // Copyright holder
    play_speed_ntsc: [u8; 2],  // (lo, hi) Play speed, in 1/1000000th sec ticks, NTSC (see docs)
    bankswitch_init: [u8; 8], // Bankswitch init values (see docs)
    play_speed_pal: [u8; 2], // (lo, hi) Play speed, in 1/1000000th sec ticks, PAL (see docs)
    //music_data: [u8; 4],
}

impl NSFHeader {
    fn new() -> Self {
        Self {
            format_file: [0; 5],
            version_number: 0,
            total_songs: 0,
            starting_song: 0,
            load_address: [0; 2],
            init_address: [0; 2],
            play_address: [0; 2],
            songname: [0; 32],
            artist: [0; 32],
            copyright: [0; 32],
            play_speed_ntsc: [0; 2],
            bankswitch_init: [0; 8],
            play_speed_pal: [0; 2],
            //music_data: [0; 4],
        }
    }
}

fn print_as_string(bytes: &[u8], msg: Option<&str>) {
    if let Some(msg) = msg {
        print!("{}", msg);
    }
    for b in bytes {
        print!("{}", *b as char);
    }

    println!();
}

fn read_header<'a>(
    header: &'a mut NSFHeader,
    file: &'a mut BufReader<File>,
) -> Result<&'a mut NSFHeader, std::io::Error> {
    file.read_exact(&mut header.format_file).unwrap();
    file.read_exact(&mut header.version_number.to_le_bytes())
        .unwrap();
    file.read_exact(&mut header.total_songs.to_le_bytes())
        .unwrap();
    file.read_exact(&mut header.starting_song.to_le_bytes())
        .unwrap();
    file.read_exact(&mut header.load_address).unwrap();
    file.read_exact(&mut header.init_address).unwrap();
    file.read_exact(&mut header.play_address).unwrap();
    file.read_exact(&mut header.songname).unwrap();
    file.read_exact(&mut header.artist).unwrap();
    file.read_exact(&mut header.copyright).unwrap();
    file.read_exact(&mut header.play_speed_ntsc).unwrap();
    file.read_exact(&mut header.bankswitch_init).unwrap();
    file.read_exact(&mut header.play_speed_pal).unwrap();

    Ok(header)
}

fn main() {
    let mut header = NSFHeader::new();

    let path = String::from("smb.nsf");

    // Open file
    let mut file = BufReader::new(File::open(path).expect("File not found"));

    read_header(&mut header, &mut file).unwrap();

    print_as_string(&header.format_file, Some("Format file: "));
    println!("Version number: {}", header.version_number);
    println!("Total songs: {}", header.total_songs);
    println!("Starting song: {}", header.starting_song);
    println!("Load address: {}", header.load_address[0]);
    println!("Init address: {}", header.init_address[0]);
    println!("Play address: {}", header.play_address[0]);
    print_as_string(&header.songname, Some("Song name: "));
    print_as_string(&header.artist, Some("Artist: "));
    print_as_string(&header.copyright, Some("Copyright: "));
    println!("Play speed NTSC: {}", header.play_speed_ntsc[0]);
    println!("Bankswitch init: {}", header.bankswitch_init[0]);
    println!("Play speed PAL: {}", header.play_speed_pal[0]);
}
