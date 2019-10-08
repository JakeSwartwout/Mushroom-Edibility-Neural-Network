//all of the enums for the mushroom

pub enum CapShape {
    Bell,
    Conical,
    Convex,
    Flat,
    Knobbed,
    Sunken
}

pub enum Odor {
    Almond,
    Anise,
    Creosote,
    Fishy,
    Foul,
    Musty,
    Pungent,
    Spicy,
    None
}

pub enum GillAttachment {
    Attached,
    Descending,
    Free,
    Notched
}

pub enum GillSpacing {
    Close,
    Crowded,
    Distant
}

pub enum GillSize{
    Broad,
    Narrow
}

pub enum StalkShape {
    Enlarging,
    Tapering
}

pub enum StalkRoot {
    Bulbous,
    Club,
    Cup,
    Equal,
    Rhizomorphs,
    Rooted,
    Missing
}

pub enum VeilType {
    Partial,
    Universal
}

pub enum RingType {
    Cobwebby,
    Evanescent,
    Flaring,
    Large,
    Pendant,
    Sheathing,
    Zone,
    None
}

pub enum Population {
    Abundant,
    Clustered,
    Numerous,
    Scattered,
    Several,
    Solitary
}

pub enum Habitat {
    Grasses,
    Leaves,
    Meadows,
    Paths,
    Urban,
    Waste,
    Woods
}