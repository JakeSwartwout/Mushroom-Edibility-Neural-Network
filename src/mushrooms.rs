mod mushroom_types;
use mushroom_types::*;
mod standard_types;
use standard_types::*;

// holds all of the mushroom values as chars
pub struct MushroomChar {
    edible: char,
    cap_shape: char,
    cap_surface: char,
    cap_color: char,
    bruises: char,
    odor: char,
    gill_attachment: char,
    gill_spacing: char,
    gill_size: char,
    gill_color: char,
    stalk_shape: char,
    stalk_root: char,
    stalk_surface_above_ring: char,
    stalk_surface_below_ring: char,
    stalk_color_above_ring: char,
    stalk_color_below_ring: char,
    veil_type: char,
    veil_color: char,
    ring_number: char,
    ring_type: char,
    spore_print_color: char,
    population: char,
    habitat: char,
}

impl MushroomChar {
    pub fn new(attributes: Vec<char>) -> MushroomChar{
        MushroomChar {
            edible: attributes[0],
            cap_shape: attributes[1],
            cap_surface: attributes[2],
            cap_color: attributes[3],
            bruises: attributes[4],
            odor: attributes[5],
            gill_attachment: attributes[6],
            gill_spacing: attributes[7],
            gill_size: attributes[8],
            gill_color: attributes[9],
            stalk_shape: attributes[10],
            stalk_root: attributes[11],
            stalk_surface_above_ring: attributes[12],
            stalk_surface_below_ring: attributes[13],
            stalk_color_above_ring: attributes[14],
            stalk_color_below_ring: attributes[15],
            veil_type: attributes[16],
            veil_color: attributes[17],
            ring_number: attributes[18],
            ring_type: attributes[19],
            spore_print_color: attributes[20],
            population: attributes[21],
            habitat: attributes[22],
        }
    }
}





//holds all of the mushroom values as enums
pub struct MushroomEnum {
    edible: bool,
    cap_shape: CapShape,
    cap_surface: Surface,
    cap_color: Color,
    bruised: bool,
    odor: Odor,
    gill_attachment: GillAttachment,
    gill_spacing: GillSpacing,
    gill_size: GillSize,
    gill_color: Color,
    stalk_shape: StalkShape,
    stalk_root: StalkRoot,
    stalk_surface_above_ring: Surface,
    stalk_surface_below_ring: Surface,
    stalk_color_above_ring: Color,
    stalk_color_below_ring: Color,
    veil_type: VeilType,
    veil_color: Color,
    ring_number: i8,
    ring_type: RingType,
    spore_print_color: Color,
    population: Population,
    habitat: Habitat
}







impl MushroomEnum {
    pub fn new(mushroom_char: MushroomChar) -> MushroomEnum {
        let edible = match mushroom_char.edible {
            'e' => true,
            _ => false,
        };
        let cap_shape = match mushroom_char.cap_shape {
            'b' => CapShape::Bell,
            'c' => CapShape::Conical,
            'x' => CapShape::Convex,
            'f' => CapShape::Flat,
            'k' => CapShape::Knobbed,
            's' => CapShape::Sunken,
            _ => CapShape::Bell,
        };
        let cap_surface = match_surface(mushroom_char.cap_surface);
        let cap_color = match_color(mushroom_char.cap_color);
        let bruised = match mushroom_char.bruises {
            't' => true,
            _ => false
        };
        let odor = match mushroom_char.odor {
            'a' => Odor::Almond,
            'l' => Odor::Anise,
            'c' => Odor::Creosote,
            'y' => Odor::Fishy,
            'f' => Odor::Foul,
            'm' => Odor::Musty,
            'p' => Odor::Pungent,
            's' => Odor::Spicy,
            'n' => Odor::None,
            _ => Odor::None
        };
        let gill_attachment = match mushroom_char.gill_attachment {
            'a' => GillAttachment::Attached,
            'd' => GillAttachment::Descending,
            'f' => GillAttachment::Free,
            'n' => GillAttachment::Notched,
            _ => GillAttachment::Attached,
        };
        let gill_spacing = match mushroom_char.gill_spacing {
            'c' => GillSpacing::Close,
            'w' => GillSpacing::Crowded,
            'd' => GillSpacing::Distant,
            _ => GillSpacing::Close,
        };
        let gill_size = match mushroom_char.gill_size {
            'b' => GillSize::Broad,
            'n' => GillSize::Narrow,
            _ => GillSize::Broad,
        };
        let gill_color = match_color(mushroom_char.gill_color);
        let stalk_shape = match mushroom_char.stalk_shape {
            'e' => StalkShape::Enlarging,
            't' => StalkShape::Tapering,
            _ => StalkShape::Enlarging,
        };
        let stalk_root = match mushroom_char.stalk_root {
            'b' => StalkRoot::Bulbous,
            'c' => StalkRoot::Club,
            'u' => StalkRoot::Cup,
            'e' => StalkRoot::Equal,
            'z' => StalkRoot::Rhizomorphs,
            'r' => StalkRoot::Rooted,
            '?' => StalkRoot::Missing,
            _ => StalkRoot::Missing,
        };
        let stalk_surface_above_ring = match_surface(mushroom_char.stalk_surface_above_ring);
        let stalk_surface_below_ring = match_surface(mushroom_char.stalk_surface_below_ring);
        let stalk_color_above_ring = match_color(mushroom_char.stalk_color_above_ring);
        let stalk_color_below_ring = match_color(mushroom_char.stalk_color_below_ring);
        let veil_type = match mushroom_char.veil_type {
            'p' => VeilType::Partial,
            'u' => VeilType::Universal,
            _ => VeilType::Partial,
        };
        let veil_color = match_color(mushroom_char.veil_color);
        let ring_number = match mushroom_char.ring_number {
            'n' => 0,
            'o' => 1,
            't' => 2,
            _ => 0,
        };
        let ring_type = match mushroom_char.ring_type {
            'c' => RingType::Cobwebby,
            'e' => RingType::Evanescent,
            'f' => RingType::Flaring,
            'l' => RingType::Large,
            'p' => RingType::Pendant,
            's' => RingType::Sheathing,
            'z' => RingType::Zone,
            'n' => RingType::None,
            _ => RingType::None,
        };
        let spore_print_color = match_color(mushroom_char.spore_print_color);
        let population = match mushroom_char.population {
            'a' => Population::Abundant,
            'c' => Population::Clustered,
            'n' => Population::Numerous,
            's' => Population::Scattered,
            'v' => Population::Several,
            'y' => Population::Solitary,
            _ => Population::Solitary,
        };
        let habitat = match mushroom_char.habitat {
            'g' => Habitat::Grasses,
            'l' => Habitat::Leaves,
            'm' => Habitat::Meadows,
            'p' => Habitat::Paths,
            'u' => Habitat::Urban,
            'w' => Habitat::Waste,
            'd' => Habitat::Woods,
            _ => Habitat::Grasses,
        };
        MushroomEnum{
            edible: edible,
            cap_shape: cap_shape,
            cap_surface: cap_surface,
            cap_color: cap_color,
            bruised: bruised,
            odor: odor,
            gill_attachment: gill_attachment,
            gill_spacing: gill_spacing,
            gill_size: gill_size,
            gill_color: gill_color,
            stalk_shape: stalk_shape,
            stalk_root: stalk_root,
            stalk_surface_above_ring: stalk_surface_above_ring,
            stalk_surface_below_ring: stalk_surface_below_ring,
            stalk_color_above_ring: stalk_color_above_ring,
            stalk_color_below_ring: stalk_color_below_ring,
            veil_type: veil_type,
            veil_color: veil_color,
            ring_number: ring_number,
            ring_type: ring_type,
            spore_print_color: spore_print_color,
            population: population,
            habitat: habitat 
        }
    }






    pub fn to_bool_vector(&self) -> (Vec<bool>, bool) {
        let mut vector: Vec<bool> = Vec::new();

        //set the variables to a pair
        //(spaces_before, spaces_after)
        //so total num = spaces_before + 1 + spaces_after

        let cap_shape = match self.cap_shape {
            CapShape::Bell => (0, 5),
            CapShape::Conical => (1, 4),
            CapShape::Convex => (2,3),
            CapShape::Flat => (3,2),
            CapShape::Knobbed => (4,1),
            CapShape::Sunken => (5,0),
        };
        
        let cap_surface = match_surface_tuple(&self.cap_surface);
        let cap_color = match_color_tuple(&self.cap_color);
        let bruised = if self.bruised { (0,1) } else { (1,0) };
        let odor = match self.odor {
            Odor::Almond => (0,8),
            Odor::Anise => (1,7),
            Odor::Creosote => (2,6),
            Odor::Fishy => (3,5),
            Odor::Foul => (4,4),
            Odor::Musty => (5,3),
            Odor::Pungent => (6,2),
            Odor::Spicy => (7,1),
            Odor::None => (8,0),
        };
        let gill_attachment = match self.gill_attachment {
            GillAttachment::Attached => (0,3),
            GillAttachment::Descending => (1,2),
            GillAttachment::Free => (2,1),
            GillAttachment::Notched => (3,0),
        };
        let gill_spacing = match self.gill_spacing {
            GillSpacing::Close => (0,2),
            GillSpacing::Crowded => (1,1),
            GillSpacing::Distant => (2,0),
        };
        let gill_size = match self.gill_size {
            GillSize::Broad => (1,0),
            GillSize::Narrow => (0,1),
        };
        let gill_color = match_color_tuple(&self.gill_color);
        let stalk_shape = match self.stalk_shape {
            StalkShape::Enlarging => (0,1),
            StalkShape::Tapering => (1,0),
        };
        let stalk_root = match self.stalk_root {
            StalkRoot::Bulbous => (0,6),
            StalkRoot::Club => (1,5),
            StalkRoot::Cup => (2,4),
            StalkRoot::Equal => (3,3),
            StalkRoot::Rhizomorphs => (4,2),
            StalkRoot::Rooted => (5,1),
            StalkRoot::Missing => (6,0),
        };
        let stalk_surface_above_ring = match_surface_tuple(&self.stalk_surface_above_ring);
        let stalk_surface_below_ring = match_surface_tuple(&self.stalk_surface_below_ring);
        let stalk_color_above_ring = match_color_tuple(&self.stalk_color_above_ring);
        let stalk_color_below_ring = match_color_tuple(&self.stalk_color_below_ring);
        let veil_type = match self.veil_type {
            VeilType::Partial => (0,1),
            VeilType::Universal => (1,0),
        };
        let veil_color = match_color_tuple(&self.veil_color);
        let ring_number = (self.ring_number as u32, (2 - self.ring_number) as u32);
        let ring_type = match self.ring_type {
            RingType::Cobwebby => (0,7),
            RingType::Evanescent => (1,6),
            RingType::Flaring => (2,5),
            RingType::Large => (3,4),
            RingType::Pendant => (4,3),
            RingType::Sheathing => (5,2),
            RingType::Zone => (6,1),
            RingType::None => (7,0),
        };
        let spore_print_color = match_color_tuple(&self.spore_print_color);
        let population = match self.population {
            Population::Abundant => (0,5),
            Population::Clustered => (1,4),
            Population::Numerous => (2,3),
            Population::Scattered => (3,2),
            Population::Several => (4,1),
            Population::Solitary => (5,0),
        };
        let habitat = match self.habitat {
            Habitat::Grasses => (0,6),
            Habitat::Leaves => (1,5),
            Habitat::Meadows => (2,4),
            Habitat::Paths => (3,3),
            Habitat::Urban => (4,2),
            Habitat::Waste => (5,1),
            Habitat::Woods => (6,0),
        };

        vector.push_tuple(cap_shape);
        vector.push_tuple(cap_surface);
        vector.push_tuple(cap_color);
        vector.push_tuple(bruised);
        vector.push_tuple(odor);
        vector.push_tuple(gill_attachment);
        vector.push_tuple(gill_spacing);
        vector.push_tuple(gill_size);
        vector.push_tuple(gill_color);
        vector.push_tuple(stalk_shape);
        vector.push_tuple(stalk_root);
        vector.push_tuple(stalk_surface_above_ring);
        vector.push_tuple(stalk_surface_below_ring);
        vector.push_tuple(stalk_color_above_ring);
        vector.push_tuple(stalk_color_below_ring);
        vector.push_tuple(veil_type);
        vector.push_tuple(veil_color);
        vector.push_tuple(ring_number);
        vector.push_tuple(ring_type);
        vector.push_tuple(spore_print_color);
        vector.push_tuple(population);
        vector.push_tuple(habitat);

        (vector, self.edible)
    }

    pub fn to_float_vector(&self) -> (Vec<f32>, f32) {
        self.to_bool_vector().to_float_vector()
    }
}






//to push the boolean tuples into the vector

trait PushTuples {
    fn push_tuple(&mut self, (_before, _after):(u32, u32)){}
}

impl PushTuples for Vec<bool> {
    fn push_tuple(&mut self, (before, after): (u32, u32)) {
        for _i in 0..before {
            self.push(false);
        }
        self.push(true);
        for _i in 0..after {
            self.push(false);
        }
    }
}






//converts the vector of booleans to floats
trait BoolToFloat {
    fn to_float_vector(&self) -> (Vec<f32>, f32);
}

impl BoolToFloat for (Vec<bool>, bool) {
    fn to_float_vector(&self) -> (Vec<f32>, f32) {
        let mut floats:Vec<f32> = Vec::new();
        for boolean in &self.0 {
            if *boolean {
                floats.push(1.0);
            } else {
                floats.push(0.0);
            }
        }
        if self.1 {
            (floats, 1.0)
        } else {
            (floats, 0.0)
        }
    }
}





pub fn get_mushrooms(contents: &str) -> (Vec<Vec<f32>>, Vec<f32>){
    //split it into lines
    let lines: Vec<&str> = contents.split('\n').collect();

    //to hold each mushroom's list of values
    let mut mushrooms: Vec<Vec<f32>> = Vec::new();
    
    //to hold each mushroom's result (poisonous or not)
    let mut mushroom_results: Vec<f32> = Vec::new();
    
    for line in lines {
        //split the line up into strs
        let pieces_str: Vec<&str> = line.split(',').collect();
        
        //turn the strs into chars
        let mut pieces: Vec<char> = Vec::new();
        for piece in pieces_str {
            pieces.push(piece.chars().next().unwrap());
        }

        //create a mushroom object of the chars
        let mush_char: MushroomChar = MushroomChar::new(pieces);
        //create a mushroom object of enums
        let mush_enum: MushroomEnum = MushroomEnum::new(mush_char);
        //turn it into a vector
        let (mush_vector, mush_result) = mush_enum.to_float_vector();

        //add it to the list
        mushrooms.push(mush_vector);
        mushroom_results.push(mush_result);
    }//for line in lines

    (mushrooms, mushroom_results)
}