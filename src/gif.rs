#![allow(dead_code, unused_variables, unused_assignments)]
use std::{
    fs::File,
    io::{self, BufReader, Read},
};

#[derive(Clone,Debug)]
pub struct GIF {
    header: GIFHeader,
    logical_screen_descriptor: LogicalScreenDescriptor,
    global_color_table: Option<GlobalColorTable>,
    images: Vec<GIFImage>,
    trailer: Trailer,
}

#[derive(Clone,Debug)]
pub struct GIFHeader {
    signature: [u8; 3],
    version: [u8; 3],
}

#[derive(Clone,Debug)]
pub struct LogicalScreenDescriptor {
    width: u16,
    height: u16,
    packed_fields: LSDPackedFields,
    background_color_index: u8,
    pixel_aspect_ratio: u8,
}

#[derive(Clone,Debug)]
pub struct LSDPackedFields {
    global_color_table_flag: bool,
    color_resolution: u8,
    sort_flag: bool,
    size_of_global_color_table: u8,
}

#[derive(Clone,Debug)]
pub struct GlobalColorTable {
    colors: Vec<Color>,
}

#[derive(Clone,Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Clone,Debug)]
pub struct ImageDescriptor {
    separator: u8,
    left_position: u16,
    top_position: u16,
    width: u16,
    height: u16,
    packed_fields: IDPackedFields,
}

#[derive(Clone,Debug)]
pub struct IDPackedFields {
    local_color_table_flag: bool,
    interlace_flag: bool,
    sort_flag: bool,
    reserved: u8,
    size_of_local_color_table: u8,
}

#[derive(Clone,Debug)]
pub struct LocalColorTable {
    colors: Vec<Color>,
}

#[derive(Clone,Debug)]
pub struct ImageData {
    lzw_minimum_code_size: u8,
    sub_blocks: Vec<GIFDataSubBlock>,
}

#[derive(Clone,Debug)]
pub struct GIFDataSubBlock {
    size: u8,
    data: Vec<u8>,
}

#[derive(Clone,Debug)]
pub struct GraphicControlExtension {
    extension_introducer: u8,
    graphic_control_label: u8,
    block_size: u8,
    packed_fields: GCEPackedFields,
    delay_time: u16,
    transparent_color_index: u8,
    block_terminator: u8,
}

#[derive(Clone,Debug)]
pub struct GCEPackedFields {
    reserved: u8,
    disposal_method: u8,
    user_input_flag: bool,
    transparent_color_flag: bool,
}

#[derive(Clone,Debug)]
pub struct CommentExtension {
    extension_introducer: u8,
    comment_label: u8,
    block_size: u8,
    comment_data: Vec<GIFDataSubBlock>,
    block_terminator: u8,
}

#[derive(Clone,Debug)]
pub struct PlainTextExtension {
    extension_introducer: u8,
    plain_text_label: u8,
    block_size: u8,
    text_grid_left_position: u16,
    text_grid_top_position: u16,
    text_grid_width: u16,
    text_grid_height: u16,
    character_cell_width: u8,
    character_cell_height: u8,
    text_foreground_color_index: u8,
    text_background_color_index: u8,
    text_data: Vec<GIFDataSubBlock>,
    block_terminator: u8,
}

#[derive(Clone,Debug)]
pub struct ApplicationExtension {
    extension_introducer: u8,
    extension_label: u8,
    block_size: u8,
    application_identifier: [u8; 8],
    application_authentication_code: [u8; 3],
    application_data: Vec<GIFDataSubBlock>,
    block_terminator: u8,
}

#[derive(Clone,Debug)]
pub struct Trailer {
    trailer: u8,
}

#[derive(Clone,Debug)]
pub struct GIFImage {
    graphic_control_extension: Option<GraphicControlExtension>,
    comment_extension: Option<CommentExtension>,
    plain_text_extension: Option<PlainTextExtension>,
    application_extension: Option<ApplicationExtension>,
    image_descriptor: ImageDescriptor,
    local_color_table: Option<LocalColorTable>,
    image_data: ImageData,
}

impl LSDPackedFields {
    fn from_byte(byte: u8) -> LSDPackedFields {
        LSDPackedFields {
            global_color_table_flag: (byte & 0b10000000) != 0,
            color_resolution: (byte & 0b01110000) >> 4,
            sort_flag: (byte & 0b00001000) != 0,
            size_of_global_color_table: byte & 0b00000111,
        }
    }
    fn to_byte(&self) -> u8 {
        let mut byte = 0;
        if self.global_color_table_flag {
            byte |= 0b10000000;
        }
        byte |= self.color_resolution << 4;
        if self.sort_flag {
            byte |= 0b00001000;
        }
        byte |= self.size_of_global_color_table;
        byte
    }
}

impl IDPackedFields {
    fn from_byte(byte: u8) -> IDPackedFields {
        IDPackedFields {
            local_color_table_flag: (byte & 0b10000000) != 0,
            interlace_flag: (byte & 0b01000000) != 0,
            sort_flag: (byte & 0b00100000) != 0,
            reserved: (byte & 0b00011000) >> 3,
            size_of_local_color_table: byte & 0b00000111,
        }
    }
    fn to_byte(&self) -> u8 {
        let mut byte = 0;
        if self.local_color_table_flag {
            byte |= 0b10000000;
        }
        if self.interlace_flag {
            byte |= 0b01000000;
        }
        if self.sort_flag {
            byte |= 0b00100000;
        }
        byte |= self.reserved << 3;
        byte |= self.size_of_local_color_table;
        byte
    }
}

impl GCEPackedFields {
    fn from_byte(byte: u8) -> GCEPackedFields {
        GCEPackedFields {
            reserved: (byte & 0b11100000) >> 5,
            disposal_method: (byte & 0b00011100) >> 2,
            user_input_flag: (byte & 0b00000010) != 0,
            transparent_color_flag: (byte & 0b00000001) != 0,
        }
    }
    fn to_byte(&self) -> u8 {
        let mut byte = 0;
        byte |= self.reserved << 5;
        byte |= self.disposal_method << 2;
        if self.user_input_flag {
            byte |= 0b00000010;
        }
        if self.transparent_color_flag {
            byte |= 0b00000001;
        }
        byte
    }
}

impl GIFDataSubBlock {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.size);
        bytes.extend_from_slice(&self.data);
        bytes
    }
}

pub fn encode_gif(gif: GIF) -> Vec<u8> {
    let mut bytes = Vec::new();

    bytes.extend_from_slice(&gif.header.signature);
    bytes.extend_from_slice(&gif.header.version);

    bytes.extend_from_slice(&gif.logical_screen_descriptor.width.to_le_bytes());
    bytes.extend_from_slice(&gif.logical_screen_descriptor.height.to_le_bytes());

    bytes.push(gif.logical_screen_descriptor.packed_fields.to_byte());

    bytes.push(gif.logical_screen_descriptor.background_color_index);
    bytes.push(gif.logical_screen_descriptor.pixel_aspect_ratio);

    if let Some(global_color_table) = gif.global_color_table {
        for color in global_color_table.colors {
            bytes.push(color.red);
            bytes.push(color.green);
            bytes.push(color.blue);
        }
    }

    for image in gif.images {
        if let Some(graphic_control_extension) = image.graphic_control_extension {
            bytes.push(graphic_control_extension.extension_introducer);
            bytes.push(graphic_control_extension.graphic_control_label);
            bytes.push(graphic_control_extension.block_size);
            bytes.push(graphic_control_extension.packed_fields.to_byte());
            bytes.extend_from_slice(&graphic_control_extension.delay_time.to_le_bytes());
            bytes.push(graphic_control_extension.transparent_color_index);
            bytes.push(graphic_control_extension.block_terminator);
        }
        if let Some(comment_extension) = image.comment_extension {
            bytes.push(comment_extension.extension_introducer);
            bytes.push(comment_extension.comment_label);
            bytes.push(comment_extension.block_size);
            for subblock in comment_extension.comment_data {
                bytes.extend_from_slice(&subblock.to_bytes());
            }
            bytes.push(comment_extension.block_terminator);
        }
        if let Some(plain_text_extension) = image.plain_text_extension {
            bytes.push(plain_text_extension.extension_introducer);
            bytes.push(plain_text_extension.plain_text_label);
            bytes.push(plain_text_extension.block_size);
            bytes.extend_from_slice(&plain_text_extension.text_grid_left_position.to_le_bytes());
            bytes.extend_from_slice(&plain_text_extension.text_grid_top_position.to_le_bytes());
            bytes.extend_from_slice(&plain_text_extension.text_grid_width.to_le_bytes());
            bytes.extend_from_slice(&plain_text_extension.text_grid_height.to_le_bytes());
            bytes.push(plain_text_extension.character_cell_width);
            bytes.push(plain_text_extension.character_cell_height);
            bytes.push(plain_text_extension.text_foreground_color_index);
            bytes.push(plain_text_extension.text_background_color_index);
            for subblock in plain_text_extension.text_data {
                bytes.extend_from_slice(&subblock.to_bytes());
            }
            bytes.push(plain_text_extension.block_terminator);
        }
        if let Some(application_extension) = image.application_extension {
            bytes.push(application_extension.extension_introducer);
            bytes.push(application_extension.extension_label);
            bytes.push(application_extension.block_size);
            bytes.extend_from_slice(&application_extension.application_identifier);
            bytes.extend_from_slice(&application_extension.application_authentication_code);
            for subblock in application_extension.application_data {
                bytes.extend_from_slice(&subblock.to_bytes());
            }
            bytes.push(application_extension.block_terminator);
        }
        bytes.push(image.image_descriptor.separator);
        bytes.extend_from_slice(&image.image_descriptor.left_position.to_le_bytes());
        bytes.extend_from_slice(&image.image_descriptor.top_position.to_le_bytes());
        bytes.extend_from_slice(&image.image_descriptor.width.to_le_bytes());
        bytes.extend_from_slice(&image.image_descriptor.height.to_le_bytes());
        bytes.push(image.image_descriptor.packed_fields.to_byte());
        if let Some(local_color_table) = image.local_color_table {
            for color in local_color_table.colors {
                bytes.push(color.red);
                bytes.push(color.green);
                bytes.push(color.blue);
            }
        }
        bytes.push(image.image_data.lzw_minimum_code_size);
        for subblock in image.image_data.sub_blocks {
            bytes.extend_from_slice(&subblock.to_bytes());
        }
        bytes.push(0);
    }

    bytes
}

#[derive(Debug)]
pub struct GIFError {
    message: String,
}

const GIF_SIGNATURE: [u8; 3] = [0x47, 0x49, 0x46];
const GIF_87A_VERSION: [u8; 3] = [0x38, 0x37, 0x61];
const GIF_89A_VERSION: [u8; 3] = [0x38, 0x39, 0x61];

pub fn file_to_gif(file_path: &str) -> Result<GIF, GIFError> {
    let file = File::open(file_path).expect("Unable to open file");
    let mut reader = BufReader::new(file);
    let mut gif_header_bytes: [u8; 6] = [0; 6];
    reader
        .read_exact(&mut gif_header_bytes)
        .expect("Unable to read gif header");
    if gif_header_bytes[0..3] != GIF_SIGNATURE {
        return Result::Err(GIFError {
            message: "Invalid gif signature".to_string(),
        });
    };
    let gif_header = GIFHeader {
        signature: [
            gif_header_bytes[0],
            gif_header_bytes[1],
            gif_header_bytes[2],
        ],
        version: [
            gif_header_bytes[3],
            gif_header_bytes[4],
            gif_header_bytes[5],
        ],
    };
    let mut logical_screen_descriptor_bytes: [u8; 7] = [0; 7];
    reader
        .read_exact(&mut logical_screen_descriptor_bytes)
        .expect("Unable to read logical screen descriptor");
    let logical_screen_descriptor = LogicalScreenDescriptor {
        width: u16::from_le_bytes([
            logical_screen_descriptor_bytes[0],
            logical_screen_descriptor_bytes[1],
        ]),
        height: u16::from_le_bytes([
            logical_screen_descriptor_bytes[2],
            logical_screen_descriptor_bytes[3],
        ]),
        packed_fields: LSDPackedFields::from_byte(logical_screen_descriptor_bytes[4]),
        background_color_index: logical_screen_descriptor_bytes[5],
        pixel_aspect_ratio: logical_screen_descriptor_bytes[6],
    };
    let global_color_table = if logical_screen_descriptor
        .packed_fields
        .global_color_table_flag
    {
        let mut colors = Vec::new();
        let chunk_length = 2u16.pow(
            (logical_screen_descriptor
                .packed_fields
                .size_of_global_color_table
                + 1)
            .into(),
        );
        for _ in 0..chunk_length {
            let mut color_bytes = [0u8; 3];
            reader
                .read_exact(&mut color_bytes)
                .expect("Unable to read color");
            colors.push(Color {
                red: color_bytes[0],
                green: color_bytes[1],
                blue: color_bytes[2],
            });
        }
        Some(GlobalColorTable { colors })
    } else {
        None
    };
    let mut images: Vec<GIFImage> = Vec::new();
    loop {
        let mut image_frame = GIFImage {
            image_descriptor: ImageDescriptor {
                separator: 0x2C,
                left_position: 0,
                top_position: 0,
                width: 0,
                height: 0,
                packed_fields: IDPackedFields {
                    local_color_table_flag: false,
                    interlace_flag: false,
                    sort_flag: false,
                    reserved: 0,
                    size_of_local_color_table: 0,
                },
            },
            local_color_table: None,
            image_data: ImageData {
                lzw_minimum_code_size: 0,
                sub_blocks: Vec::new(),
            },
            graphic_control_extension: None,
            comment_extension: None,
            plain_text_extension: None,
            application_extension: None,
        };

        let mut separator: [u8; 1] = [0; 1];
        loop {
            reader
                .read_exact(&mut separator)
                .expect("Unable to read separator");
            match separator[0] {
                0x2C => {
                    // Image Descriptor
                    // println!("Image Descriptor");
                    image_frame.image_descriptor.separator = 0x2C;
                    let mut image_descriptor_bytes: [u8; 9] = [0; 9];
                    reader
                        .read_exact(&mut image_descriptor_bytes)
                        .expect("Unable to read image descriptor");
                    image_frame.image_descriptor.left_position =
                        u16::from_le_bytes([image_descriptor_bytes[0], image_descriptor_bytes[1]]);
                    image_frame.image_descriptor.top_position =
                        u16::from_le_bytes([image_descriptor_bytes[2], image_descriptor_bytes[3]]);
                    image_frame.image_descriptor.width =
                        u16::from_le_bytes([image_descriptor_bytes[4], image_descriptor_bytes[5]]);
                    image_frame.image_descriptor.height =
                        u16::from_le_bytes([image_descriptor_bytes[6], image_descriptor_bytes[7]]);
                    image_frame.image_descriptor.packed_fields =
                        IDPackedFields::from_byte(image_descriptor_bytes[8]);
                    if image_frame
                        .image_descriptor
                        .packed_fields
                        .local_color_table_flag
                    {
                        // println!("...with a Local Color Table");
                        let mut colors = Vec::new();
                        let chunk_length = 2u16.pow(
                            (image_frame
                                .image_descriptor
                                .packed_fields
                                .size_of_local_color_table
                                + 1)
                            .into(),
                        );
                        for _ in 0..chunk_length {
                            let mut color_bytes = [0u8; 3];
                            reader
                                .read_exact(&mut color_bytes)
                                .expect("Unable to read color");
                            colors.push(Color {
                                red: color_bytes[0],
                                green: color_bytes[1],
                                blue: color_bytes[2],
                            });
                        }
                        image_frame.local_color_table = Some(LocalColorTable { colors });
                    }
                    let mut lzw_minimum_code_size: [u8; 1] = [0; 1];
                    reader
                        .read_exact(&mut lzw_minimum_code_size)
                        .expect("Unable to read lzw minimum code size");
                    image_frame.image_data.lzw_minimum_code_size = lzw_minimum_code_size[0];
                    let mut sub_block_size: [u8; 1] = [0; 1];
                    loop {
                        reader
                            .read_exact(&mut sub_block_size)
                            .expect("Unable to read sub block size");
                        if sub_block_size[0] == 0 {
                            break;
                        }
                        let mut sub_block_data: Vec<u8> = Vec::new();
                        for _ in 0..sub_block_size[0] {
                            let mut sub_block_byte: [u8; 1] = [0; 1];
                            reader
                                .read_exact(&mut sub_block_byte)
                                .expect("Unable to read sub block byte");
                            sub_block_data.push(sub_block_byte[0]);
                        }
                        image_frame.image_data.sub_blocks.push(GIFDataSubBlock {
                            size: sub_block_size[0],
                            data: sub_block_data,
                        });
                    }
                    images.push(image_frame);
                    break;
                }
                0x21 => {
                    // Extension
                    let mut label: [u8; 1] = [0; 1];
                    reader
                        .read_exact(&mut label)
                        .expect("Unable to read graphic control label");
                    match label[0] {
                        0xF9 => {
                            // Graphic Control Extension
                            // println!("Graphic Control Extension");
                            let mut block_size: [u8; 1] = [0; 1];
                            reader
                                .read_exact(&mut block_size)
                                .expect("Unable to read block size");
                            let mut packed_fields: [u8; 1] = [0; 1];
                            reader
                                .read_exact(&mut packed_fields)
                                .expect("Unable to read packed fields");
                            let mut delay_time: [u8; 2] = [0; 2];
                            reader
                                .read_exact(&mut delay_time)
                                .expect("Unable to read delay time");
                            let mut transparent_color_index: [u8; 1] = [0; 1];
                            reader
                                .read_exact(&mut transparent_color_index)
                                .expect("Unable to read transparent color index");
                            let mut block_terminator: [u8; 1] = [0; 1];
                            reader
                                .read_exact(&mut block_terminator)
                                .expect("Unable to read block terminator");
                            image_frame.graphic_control_extension = Some(GraphicControlExtension {
                                extension_introducer: 0x21,
                                graphic_control_label: 0xF9,
                                block_size: block_size[0],
                                packed_fields: GCEPackedFields::from_byte(packed_fields[0]),
                                delay_time: u16::from_le_bytes([delay_time[0], delay_time[1]]),
                                transparent_color_index: transparent_color_index[0],
                                block_terminator: block_terminator[0],
                            });
                        }
                        0xFE => {
                            // Comment Extension
                            // println!("Comment Extension");
                            let mut block_size: [u8; 1] = [0; 1];
                            reader
                                .read_exact(&mut block_size)
                                .expect("Unable to read block size");
                            let mut comment_data: Vec<GIFDataSubBlock> = Vec::new();
                            loop {
                                let mut subblock_size: [u8; 1] = [0; 1];
                                reader
                                    .read_exact(&mut subblock_size)
                                    .expect("Unable to read comment byte");
                                if subblock_size[0] == 0 {
                                    break;
                                }
                                let mut subblock_data: Vec<u8> = Vec::new();
                                for _ in 0..subblock_size[0] {
                                    let mut subblock_byte: [u8; 1] = [0; 1];
                                    reader
                                        .read_exact(&mut subblock_byte)
                                        .expect("Unable to read comment byte");
                                    subblock_data.push(subblock_byte[0]);
                                }
                                comment_data.push(GIFDataSubBlock {
                                    size: subblock_size[0],
                                    data: subblock_data,
                                });
                            }
                            image_frame.comment_extension = Some(CommentExtension {
                                extension_introducer: 0x21,
                                comment_label: 0xFE,
                                block_size: block_size[0],
                                comment_data,
                                block_terminator: 0x0,
                            });
                        }
                        0x01 => {
                            // Plain Text Extension
                            // println!("Plain Text Extension");
                            let mut block_size: [u8; 1] = [0; 1];
                            reader
                                .read_exact(&mut block_size)
                                .expect("Unable to read block size");
                            let mut text_grid_left_position: [u8; 2] = [0; 2];
                            reader
                                .read_exact(&mut text_grid_left_position)
                                .expect("Unable to read text grid left position");
                            let mut text_grid_top_position: [u8; 2] = [0; 2];
                            reader
                                .read_exact(&mut text_grid_top_position)
                                .expect("Unable to read text grid top position");
                            let mut text_grid_width: [u8; 2] = [0; 2];
                            reader
                                .read_exact(&mut text_grid_width)
                                .expect("Unable to read text grid width");
                            let mut text_grid_height: [u8; 2] = [0; 2];
                            reader
                                .read_exact(&mut text_grid_height)
                                .expect("Unable to read text grid height");
                            let mut character_cell_width: [u8; 1] = [0; 1];
                            reader
                                .read_exact(&mut character_cell_width)
                                .expect("Unable to read character cell width");
                            let mut character_cell_height: [u8; 1] = [0; 1];
                            reader
                                .read_exact(&mut character_cell_height)
                                .expect("Unable to read character cell height");
                            let mut text_foreground_color_index: [u8; 1] = [0; 1];
                            reader
                                .read_exact(&mut text_foreground_color_index)
                                .expect("Unable to read text foreground color index");
                            let mut text_background_color_index: [u8; 1] = [0; 1];
                            reader
                                .read_exact(&mut text_background_color_index)
                                .expect("Unable to read text background color index");
                            let mut text_data: Vec<GIFDataSubBlock> = Vec::new();
                            loop {
                                let mut subblock_size: [u8; 1] = [0; 1];
                                reader
                                    .read_exact(&mut subblock_size)
                                    .expect("Unable to read text byte");
                                if subblock_size[0] == 0 {
                                    break;
                                }
                                let mut block_data: Vec<u8> = Vec::new();
                                for _ in 0..subblock_size[0] {
                                    let mut subblock_byte: [u8; 1] = [0; 1];
                                    reader
                                        .read_exact(&mut subblock_byte)
                                        .expect("Unable to read text byte");
                                    block_data.push(subblock_byte[0]);
                                }
                                text_data.push(GIFDataSubBlock {
                                    size: subblock_size[0],
                                    data: block_data,
                                });
                            }
                            image_frame.plain_text_extension = Some(PlainTextExtension {
                                extension_introducer: 0x21,
                                plain_text_label: 0x01,
                                block_size: block_size[0],
                                text_grid_left_position: u16::from_le_bytes([
                                    text_grid_left_position[0],
                                    text_grid_left_position[1],
                                ]),
                                text_grid_top_position: u16::from_le_bytes([
                                    text_grid_top_position[0],
                                    text_grid_top_position[1],
                                ]),
                                text_grid_width: u16::from_le_bytes([
                                    text_grid_width[0],
                                    text_grid_width[1],
                                ]),
                                text_grid_height: u16::from_le_bytes([
                                    text_grid_height[0],
                                    text_grid_height[1],
                                ]),
                                character_cell_width: character_cell_width[0],
                                character_cell_height: character_cell_height[0],
                                text_foreground_color_index: text_foreground_color_index[0],
                                text_background_color_index: text_background_color_index[0],
                                text_data,
                                block_terminator: 0,
                            });
                        }
                        0xFF => {
                            // Application Extension
                            // println!("Application Extension");
                            let mut block_size: [u8; 1] = [0; 1];
                            let mut application_identifier: [u8; 8] = [0; 8];
                            let mut application_authentication_code: [u8; 3] = [0; 3];
                            reader
                                .read_exact(&mut block_size)
                                .expect("Unable to read block size");
                            reader
                                .read_exact(&mut application_identifier)
                                .expect("Unable to read application identifier");
                            reader
                                .read_exact(&mut application_authentication_code)
                                .expect("Unable to read application authentication code");
                            // println!(
                            //     "Application Identifier: {}\nAuthentication Code: {}\nBlock Size: {}",
                            //     String::from_utf8_lossy(&application_identifier),
                            //     String::from_utf8_lossy(&application_authentication_code),
                            //     block_size[0]
                            // );
                            let mut application_data: Vec<GIFDataSubBlock> = Vec::new();
                            loop {
                                let mut subblock_size: [u8; 1] = [0; 1];
                                reader
                                    .read_exact(&mut subblock_size)
                                    .expect("Unable to subblock size");
                                if subblock_size[0] == 0 {
                                    break;
                                }
                                let mut block_data: Vec<u8> = Vec::new();
                                for _ in 0..subblock_size[0] {
                                    let mut subblock_byte: [u8; 1] = [0; 1];
                                    reader
                                        .read_exact(&mut subblock_byte)
                                        .expect("Unable to read application byte");
                                    block_data.push(subblock_byte[0]);
                                }
                                application_data.push(GIFDataSubBlock {
                                    size: subblock_size[0],
                                    data: block_data,
                                });
                            }
                            // for subblock in &application_data {
                            //     println!(
                            //         "Subblock Size: {}\nSubblock Data: {:X?}\nWhich has a length of {}",
                            //         subblock.size,
                            //         &subblock.data,
                            //         subblock.data.len()
                            //     );
                            // }
                            // println!("There are {} subblocks", application_data.len());
                            image_frame.application_extension = Some(ApplicationExtension {
                                extension_introducer: 0x21,
                                extension_label: 0xFF,
                                block_size: block_size[0],
                                application_identifier,
                                application_authentication_code,
                                application_data,
                                block_terminator: 0,
                            });
                        }
                        _ => {
                            return Result::Err(GIFError {
                                message: "Invalid extension label".to_string(),
                            });
                        }
                    }
                }
                0x3B => {
                    break;
                }
                _ => {
                    return Result::Err(GIFError {
                        message: format!("Invalid separator: {}", separator[0]),
                    });
                }
            }
        }
        if separator[0] == 0x3B {
            break;
        }
    }

    Result::Ok(GIF {
        header: gif_header,
        logical_screen_descriptor,
        global_color_table,
        images,
        trailer: Trailer { trailer: 0x3B },
    })
}

impl GIF {
    pub fn from_file(file_path: &str) -> GIF {
        file_to_gif(file_path).expect("Unable to read gif file")
    }

    pub fn save(&self, file_path: &str) {
        let bytes = encode_gif(self.clone());
        std::fs::write(file_path, bytes).expect("Unable to write file")
    }
    pub fn reverse(&self) -> GIF {
        let mut new_gif = self.clone();

        new_gif.images.reverse();

        new_gif
    }
    pub fn resize(&self, width: u16, height: u16) -> GIF {
        let mut new_gif = self.clone();

        new_gif.logical_screen_descriptor.width = width;
        new_gif.logical_screen_descriptor.height = height;

        new_gif
    }
}