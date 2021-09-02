mod island {
	#[derive(Debug)]
	pub struct Island {
		core_x: u8,
		core_y: u8,
		width: u8,
		height: u8,
		offset_x: u8,
		offset_y: u8,
		block_array: [[Option<Item>; 12]; 12],
		attachment_array: [[Option<Item>; 12]; 12],
		keybind_array: [[Option<String>; 12]; 12]
	}

	#[derive(Debug)]
	pub struct Item {
		id: String,
		xp: f64,
		xp_total: f64,
		xp_next: f64,
		tier: u32,
		damaged: bool,
		properties: Vec<Option<Property>>
	}

	#[derive(Debug)]
	pub struct Property {
		id: String,
		value: PropertyValue,
		layer: u32
	}

	#[derive(Debug)]
	pub enum PropertyValue {
		Int(i64),
		Float(f64),
		String(String),
		List(Vec<PropertyValue>)
	}

	pub fn parse_island(island_string: &str)/* -> Island*/ {
		let split = island_string.split_whitespace();
		let split = split.collect::<Vec<_>>();

		// Load name files for numeric id conversion
		let item_names = match std::fs::read_to_string("src/names/items") {
			Ok(file) => file,
			Err(error) => panic!("Couldn't open item names file: {:?}", error),
		};
		let item_names = item_names.split('\n').collect::<Vec<_>>();
		let item_property_names = match std::fs::read_to_string("src/names/item_properties") {
			Ok(file) => file,
			Err(error) => panic!("Couldn't open item properties names file: {:?}", error),
		};
		let item_property_names = item_property_names.split('\n').collect::<Vec<_>>();
		let projectile_property_names = match std::fs::read_to_string("src/names/projectile_properties") {
			Ok(file) => file,
			Err(error) => panic!("Couldn't open projectile properties names file: {:?}", error),
		};
		let projectile_property_names = projectile_property_names.split('\n').collect::<Vec<_>>();

		let mut block_array: [[Option<Item>; 12]; 12] = Default::default();
		let mut attachment_array: [[Option<Item>; 12]; 12] = Default::default();
		let mut keybind_array: [[Option<String>; 12]; 12] = Default::default();

		let mut slot = 0;
		let mut i = 0;

		while i < split.len() - 1 {
			let s = split[i].parse::<usize>();
			match s {
				Ok(skip) => {
					slot += skip;
					println!("Skipping {} spaces", skip);
					i += 1;
				}
				Err(_) => {
					block_array[slot % 12][slot / 12] = parse_item(split[i], &item_names, &item_property_names, &projectile_property_names);
					attachment_array[slot % 12][slot / 12] = parse_item(split[i+1], &item_names, &item_property_names, &projectile_property_names);
					//keybind_array[slot % 12][slot / 12] = parse_keybind(split[i+2]);
					println!("Block:      {}", split[i]);
					println!("Attachment: {}", split[i+1]);
					println!("Keybind:    {}", split[i+2]);
					slot += 1;
					i += 3;
				}
			}

			
		}
	}

	pub fn parse_item(item_string: &str, item_names: &Vec<&str>, item_property_names: &Vec<&str>, projectile_property_names: &Vec<&str>) -> Option<Item> {
		// '_' means no item
		if item_string == "_" {
			println!("Skipping item.");
			None
		}
		else {
			// Split string into a list of char lists
			let initial_split = item_string[1..item_string.len()-1].split('|');
			let mut split = Vec::new();
			for s in initial_split {
				split.push(s.chars().collect::<Vec<_>>());
			}

			// Parse/convert id
			let id = if split[0][0] == '§' {
				match split[0][1..].iter().cloned().collect::<String>().parse::<usize>() {
					Ok(val) => String::from(item_names[val]),
					Err(error) => panic!("Unable to parse item id '{}': {:?}", split[0].iter().cloned().collect::<String>(), error),
				}
			}
			else {
				split[0][1..].iter().cloned().collect::<String>()
			};

			let xp = match split[1].iter().cloned().collect::<String>().parse::<f64>() {
				Ok(val) => val,
				Err(error) => panic!("Unable to parse item xp '{}': {:?}", split[1].iter().cloned().collect::<String>(), error),
			};

			let xp_total = match split[2].iter().cloned().collect::<String>().parse::<f64>() {
				Ok(val) => val,
				Err(error) => panic!("Unable to parse item xp_total '{}': {:?}", split[2].iter().cloned().collect::<String>(), error),
			};

			let xp_next = match split[3].iter().cloned().collect::<String>().parse::<f64>() {
				Ok(val) => val,
				Err(error) => panic!("Unable to parse item xp_next '{}': {:?}", split[3].iter().cloned().collect::<String>(), error),
			};

			let tier = match split[4].iter().cloned().collect::<String>().parse::<u32>() {
				Ok(val) => val,
				Err(error) => panic!("Unable to parse item tier '{}': {:?}", split[4].iter().cloned().collect::<String>(), error),
			};

			let damaged = match split[5].iter().cloned().collect::<String>().parse::<usize>() {
				Ok(val) => val == 1,
				Err(error) => panic!("Unable to parse item damaged flag '{}': {:?}", split[4].iter().cloned().collect::<String>(), error),
			};

			let mut properties = Vec::new();
			let mut i = 6;
			let len = split.len();
			while i < len - 1 {
				properties.push(parse_property(&split[i].iter().cloned().collect::<String>()[..], &item_property_names, &projectile_property_names));
				i += 1;
				println!("Parsing property: {}", split[i].iter().cloned().collect::<String>())
			}
			println!("Properties: {:?}", properties);

			Option::Some(Item {
				id,
				xp,
				xp_total,
				xp_next,
				tier,
				damaged,
				properties
			})
		}
	}

	pub fn parse_property(property_string: &str, item_property_names: &Vec<&str>, projectile_property_names: &Vec<&str>) -> Option<Property> {
		let split = property_string.chars().collect::<Vec<_>>();

		// Find index of first instance of ':' which seperates id and data
		let mut i = 0;
		let len = split.len();
		let mut sep_index = 0;
		while i < len {
			if split[i] == ':' {
				sep_index = i;
				break;
			}
			i += 1;
		}
		if sep_index <= 1 || sep_index == len-1 {
			panic!("Unable to parse property '{}': Incorrect or missing ':' seperator.", split.iter().cloned().collect::<String>())
		}

		/*let (id, layer) = match split[0] {
			// Top layer int id
			'·' => {
				(match split.iter().cloned().collect::<String>()[1..sep_index-1].parse::<usize>() {
					Ok(val) => String::from(item_property_names[val]),
					Err(error) => panic!("Unable to parse property id '{}': {:?}", split.iter().cloned().collect::<String>(), error),
				}, 0 as u32)
			},
			// Top layer string id
			'°' => {
				(String::from(&split[1..sep_index].iter().cloned().collect::<String>()) , 0 as u32)
			},
		}*/
		None // DEBUG
	}

	pub fn parse_keybind(keybind_string: &str) -> Option<String> {
		None // DEBUG
	}
}


fn main() {
	island::parse_island("41 |§0|0|14.46|28.93|1|0|·18:16|·19:24|·20:9| _ _ 11 |§2|0|0|14.64|0|0| _ _ |§0|0|14.46|28.93|1|0|·18:16|·19:24|·20:9| _ _ 10 |§3|0|0|16.32|0|0| |§46|0|0|100|0|0| _ |§2|0|14.64|29.28|1|0|·18:16|·19:30|·20:13| |§110|0|15|30|1|0|·18:17|·21:7|·65:0.15|·66:¬24?| 1 |§0|0|14.46|28.93|1|0|·18:16|·19:24|·20:9| _ _ 9 |§3|0|0|16.32|0|0| |§47|0|15|30|1|0|·18:17|·26:0.07|·46:576| W |§2|0|14.64|29.28|1|0|·18:16|·19:30|·20:13| |§54|0|15|30|1|0|·18:17|·26:0.07|·47:216| AD |§0|0|14.46|28.93|1|0|·18:16|·19:24|·20:9| _ _ 9 |§2|0|0|14.64|0|0| |§110|0|15|30|1|0|·18:17|·21:7|·65:0.15|·66:¬24?| 1 |§0|0|14.46|28.93|1|0|·18:16|·19:24|·20:9| _ _ 10 |§0|0|14.46|28.93|1|0|·18:16|·19:24|·20:9| _ _ 42 ");
}