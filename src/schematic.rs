use std::{fs::{File, OpenOptions}, io::{Read, Write, BufWriter, BufReader, BufRead, ErrorKind, Seek, SeekFrom}, cmp::max};
use crate::{component::*, extra_functions::StringFunctions};

#[allow(dead_code)]
pub struct Schematic{
    file: File,
    num_components: u8,
    num_dynamic: u8,
    num_connections: u8,
    dynamic_slots: [f32; 5],
}

impl Schematic{
    #[allow(dead_code)]
    pub fn new(file_name: &str) -> Schematic{
        let txt_name = file_name.to_owned()+".txt";
        let file = OpenOptions::new().write(true).read(true).truncate(true).open(txt_name.clone());
        let create_file = match file {
            Ok(file) => file,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => OpenOptions::new().write(true).read(true).create(true).open(txt_name).unwrap(),
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            }
        };
        Schematic { 
            file: create_file, 
            num_components: 0,
            num_dynamic: 0,
            num_connections: 0,
            dynamic_slots: [0.0,0.0,0.0,0.0,0.0]
        }
    }

    #[allow(dead_code)]
    fn add_comp( &mut self, comp_type:Components, comp_value: f32, prefix: CompPrefix, dynamic: bool, dynamic_index: u8){
        let mut writer = BufWriter::new(&self.file);
        let mut line:String = String::new();
        
        match comp_type {
            Components::Resistor=>line += "R",
            Components::Capacitor=>line += "C",
            Components::Battery=>line += "B",
            Components::VoltageMeter=>line += "V",
            Components::AmperageMeter=>line += "A"
        }

        if dynamic{
            line += "~";
            line += &dynamic_index.to_string();
        }else{
            line += format!("{:.2}", comp_value).as_str();
            match prefix {
                CompPrefix::PETA=>line += "P",
                CompPrefix::TERA=>line += "T",
                CompPrefix::GIGA=>line += "G",
                CompPrefix::MEGA=>line += "M",
                CompPrefix::KILO=>line += "K",
                CompPrefix::HECTO=>line += "H",
                CompPrefix::DECA=>line += "D",
                CompPrefix::NONE=>line += "",
                CompPrefix::DECI=>line += "d",
                CompPrefix::CENTI=>line += "c",
                CompPrefix::MILLI=>line += "m",
                CompPrefix::MICRO=>line += "u",
                CompPrefix::NANO=>line += "n",
                CompPrefix::PICO=>line += "p",
                CompPrefix::FEMTO=>line += "f",
            }
        }
        
        line += "!0/0;\n";
        let write_result = writer.write_all(&line.as_bytes());
        match write_result {
            Ok(()) => (),
            Err(e) => panic!("Failed to write component to file ({:?})", e),
        };
        self.num_components += 1;
    }

    #[allow(dead_code)]
    pub fn add(&mut self, component: &mut Component){
        component.index = self.num_components;
        Schematic::add_comp(self, component.comp_type, component.value, component.prefix, component.dynamic, component.dynamic_index);
    }

    #[allow(dead_code)]
    fn insert_conn_change(&self, replace_string: String, index: u64, previous_len: usize){
        let mut writer = BufWriter::new(&self.file);
        if replace_string.len() == previous_len {
            let reset_pos_res = writer.seek(SeekFrom::Start(index));
                match reset_pos_res {
                Ok(u) => u,
                Err(e) => panic!("Failed to sync file read location ({:?})", e),
            };
            let write_result = writer.write_all(&replace_string.as_bytes());
            match write_result {
                Ok(()) => (),
                Err(e) => panic!("Failed to write component to file ({:?})", e),
            };
        }else{
            let mut reader = BufReader::new(&self.file);
            let mut buffer:Vec<u8> = Vec::new();

            let buffer_pos_res = reader.seek(SeekFrom::Start(index + previous_len as u64));
            match buffer_pos_res {
                Ok(u) => u,
                Err(e) => panic!("Failed to sync file read location ({:?})", e),
            };

            let buffer_read_res = reader.read_to_end(&mut buffer);
            match buffer_read_res {
                Ok(u) => u,
                Err(e) => panic!("Failed to read untill end ({:?})", e),
            };

            let reset_pos_res = writer.seek(SeekFrom::Start(index));
            match reset_pos_res {
                Ok(u) => u,
                Err(e) => panic!("Failed to sync file write location ({:?})", e),
            };
            let write_result = writer.write_all(&replace_string.as_bytes());
            match write_result {
                Ok(()) => (),
                Err(e) => panic!("Failed to write component to file ({:?})", e),
            };

            let write_result = writer.write_all(&buffer);
            match write_result {
                Ok(()) => (),
                Err(e) => panic!("Failed to write component to file ({:?})", e),
            };
        }
    }

    
    #[allow(dead_code)]
    pub fn connect(&mut self, component1: Component, pin1: u8, component2: Component, pin2: u8){
        if pin1 > 1 {
            panic!("Idex for first assigned pin is to high");
        }
        if pin2 > 1 {
            panic!("Idex for second assigned pin is to high");
        }
        if component1.index == component2.index {
            panic!("Components cannot be the same");
        }
        let reset_res = self.file.seek(SeekFrom::Start(0));
        match reset_res {
            Ok(u) => u,
            Err(e) => panic!("Failed to sync file read location ({:?})", e),
        };

        let mut comp1_char_index: u64 = 0;
        let mut comp2_char_index: u64 = 0;

        let mut empty_buffer = String::new();
        let mut comp1_string = String::new();
        let mut comp2_string = String::new();
        let mut reader = BufReader::new(&self.file);
        for i in 0..max(component1.index, component2.index)+1{
            let read_result = reader.read_line(
                if i == component1.index {
                    &mut comp1_string
                }else if i ==component2.index{
                    &mut comp2_string
                }else{
                    &mut empty_buffer
                });
            let _size = match read_result {
                Ok(usize) => usize,
                Err(e) => panic!("Error ({:?})", e)
            };  
            if component1.index == 0 {
                comp1_char_index = 0;
            }else
            if i == component1.index - 1{
                comp1_char_index = reader.seek(SeekFrom::Current(0)).unwrap();
            }

            if component2.index == 0 {
                comp2_char_index = 0;
            }else
            if i == component2.index - 1{
                comp2_char_index = reader.seek(SeekFrom::Current(0)).unwrap();
            }
        }


        let start_len_1 = comp1_string.len();
        let start_len_2 = comp2_string.len();     
        
        let (start_index_1, stop_index_1) = 
        if pin1 == 0 {
            (
                {
                    let start_res = comp1_string.find('!');
                    match start_res {
                        Some(index) => index+1,
                        None => panic!("Character that progam is looking for does not exist")
                    }
                },
                {
                    let start_res = comp1_string.find('/');
                    match start_res {
                        Some(index) => index-1,
                        None => panic!("Character that program is looking for does not exist")
                    }
                },
            )
        }else{
            (
                {
                    let start_res = comp1_string.find('/');
                    match start_res {
                        Some(index) => index+1,
                        None => panic!("Character that progam is looking for does not exist")
                    }
                },
                {
                    let start_res = comp1_string.find(';');
                    match start_res {
                        Some(index) => index-1,
                        None => panic!("Character that program is looking for does not exist")
                    }
                },
            )
        };

        let (start_index_2, stop_index_2) = 
        if pin2 == 0{
            (
                {
                    let start_res = comp2_string.find('!');
                    match start_res {
                        Some(index) => index+1,
                        None => panic!("Character that progam is looking for does not exist")
                    }
                },
                {
                    let stop_res = comp2_string.find('/');
                    match stop_res {
                        Some(index) => index-1,
                        None => panic!("Character that program is looking for does not exist")
                    }
                },
            )
        }else{
            (
                {
                    let start_res = comp2_string.find('/');
                    match start_res {
                        Some(index) => index+1,
                        None => panic!("Character that progam is looking for does not exist")
                    }
                },
                {
                    let stop_res = comp2_string.find(';');
                    match stop_res {
                        Some(index) => index-1,
                        None => panic!("Character that program is looking for does not exist")
                    }
                },
            )
        };

        let con_1_loc:u32 = comp1_string.clone()[start_index_1..stop_index_1+1].parse().unwrap();
        let con_2_loc:u32 = comp2_string.clone()[start_index_2..stop_index_2+1].parse().unwrap();

        if con_1_loc == 0 && con_2_loc == 0{
            self.num_connections+=1;
            comp1_string = StringFunctions::insert_replace(comp1_string, start_index_1, stop_index_1, self.num_connections.to_string());
            comp2_string = StringFunctions::insert_replace(comp2_string, start_index_2, stop_index_2, self.num_connections.to_string());
        }else if con_1_loc != 0 && con_2_loc == 0{
            comp2_string = StringFunctions::insert_replace(comp2_string, start_index_2, stop_index_2, con_1_loc.to_string());
        }else if con_1_loc == 0 && con_2_loc != 0{
            comp1_string = StringFunctions::insert_replace(comp1_string, start_index_1, stop_index_1, con_2_loc.to_string());
        }else if con_1_loc != 0 && con_2_loc != 0 && con_1_loc != con_2_loc{


            //Still need to implement 2 connections connecting to eachother!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!


        }
        
        if comp1_char_index > comp2_char_index {
            self.insert_conn_change(comp1_string, comp1_char_index, start_len_1);
            self.insert_conn_change(comp2_string, comp2_char_index, start_len_2);
        }else{
            self.insert_conn_change(comp2_string, comp2_char_index, start_len_2);
            self.insert_conn_change(comp1_string, comp1_char_index, start_len_1);
        }
    }

    #[allow(dead_code)]
    pub fn compile(&self){
        let mut reader = BufReader::new(&self.file);
        let mut read_buffer;


        let reset_res = reader.seek(SeekFrom::Start(0));
        match reset_res {
            Ok(u) => u,
            Err(e) => panic!("Failed to sync file read location at({:?})", e),
        };
        let mut info_hold: Vec<ComponentHold> = Vec::new();
        let mut total_cons:u8 = 0;
        let mut _total_comps:u8 = 0;
        loop {
            read_buffer = String::new();
            let result = reader.read_line(&mut read_buffer);
            
            let read_size = match result {
                Ok(s)=>s,
                Err(e)=> panic!("Error reading: {:?}", e)
            };
            if read_size == 0 {
                break;
            }
            

            let comp_type = match read_buffer.chars().nth(0).unwrap() {
                'R'=>Components::Resistor,
                'C'=>Components::Capacitor,
                'B'=>Components::Battery,
                'V'=>Components::VoltageMeter,
                'A'=>Components::AmperageMeter,
                other_=>panic!("HOW?? {}", other_)
            };
            info_hold.push(ComponentHold::Type(comp_type));

            
            let exclam_index = read_buffer.find('!').unwrap();
            let slash_index = read_buffer.find('/').unwrap();
            let end_index = read_buffer.find(';').unwrap();

            let comp_value = &read_buffer[1..exclam_index-1];
            if comp_value == "~" {
                info_hold.push(ComponentHold::DynamicIndex(read_buffer[2..exclam_index].parse().unwrap()));
            }else{
                let prefix = match read_buffer.chars().nth(exclam_index-1).unwrap(){
                    'P'=>CompPrefix::PETA,
                    'T'=>CompPrefix::PETA,
                    'G'=>CompPrefix::PETA,
                    'M'=>CompPrefix::PETA,
                    'K'=>CompPrefix::PETA,
                    'H'=>CompPrefix::PETA,
                    'D'=>CompPrefix::PETA,
                    'd'=>CompPrefix::PETA,
                    'c'=>CompPrefix::PETA,
                    'm'=>CompPrefix::PETA,
                    'u'=>CompPrefix::PETA,
                    'n'=>CompPrefix::PETA,
                    'p'=>CompPrefix::PETA,
                    'f'=>CompPrefix::PETA,
                    _ => CompPrefix::NONE,
                };
                let value: f64 = comp_value.parse().unwrap();
                info_hold.push(ComponentHold::Value(CompPrefix::getFactor(prefix)*value));
            }

            let con_1:u8 = read_buffer[exclam_index+1..slash_index].parse().unwrap();
            let con_2:u8 = read_buffer[slash_index+1..end_index].parse().unwrap();
            if con_1 > total_cons {
                total_cons = con_1;
            }
            if con_2 > total_cons {
                total_cons = con_2;
            }
            info_hold.push(ComponentHold::Pins([con_1,con_2]));

            _total_comps+=1;
        }

        //idetify blocks
        let mut sorted_indexes: Vec<u8> = Vec::new();
        for i in 1..total_cons+1 {
            let mut sort_index:u8 = 0;
            let mut index:u8 = 0;
            for j in info_hold.clone() {
                match j {
                    ComponentHold::Pins([a,b]) => {
                        if i == a || i == b && (a != 0 && b != 0){
                            sorted_indexes.push(sort_index);
                        }
                    },
                    _ => ()
                };   
                if index % 3 == 0{
                    sort_index += 1;
                }
                index += 1;
            }
            sorted_indexes.push(255);
        }
        print!("{:?}",sorted_indexes);
        /*let mut sorted_indexes_hold: Vec<u8> = sorted_indexes.clone();
        for i in 1..total_cons+1{
            let mut num_comps_conn:u8 = 0;
        }*/
    }

    #[allow(dead_code)]
    pub fn run(&self){

    }
}