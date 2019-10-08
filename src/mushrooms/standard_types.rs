pub enum Surface {
    Fibrous,
    Grooves,
    Scaly,
    Silky,
    Smooth
}

pub fn match_surface(surface: char) -> Surface {
    match surface {
        'f' => Surface::Fibrous,
        'g' => Surface::Grooves,
        'y' => Surface::Scaly,
        'k' => Surface::Silky,
        's' => Surface::Smooth,
        _ => Surface::Smooth,
    }
}

pub fn match_surface_tuple(surface: &Surface) -> (u32, u32) {
    match surface {
        Surface::Fibrous => (0, 4),
        Surface::Grooves => (1, 3),
        Surface::Scaly => (2, 2),
        Surface::Silky => (3, 1),
        Surface::Smooth => (4, 0),
    }
}

pub enum Color {
    Black,
    Brown,
    Buff,
    Chocolate,
    Cinnamon,
    Gray,
    Green,
    Orange,
    Pink,
    Purple,
    Red,
    White,
    Yellow
}

pub fn match_color(color: char) -> Color{
    match color {
            'k' => Color::Black,
            'n' => Color::Brown,
            'b' => Color::Buff,
            'h' => Color::Chocolate,
            'c' => Color::Cinnamon,
            'g' => Color::Gray,
            'r' => Color::Green,
            'o' => Color::Orange,
            'p' => Color::Pink,
            'u' => Color::Purple,
            'e' => Color::Red,
            'w' => Color::White,
            'y' => Color::Yellow,
            _ => Color::Gray
        }
}

pub fn match_color_tuple(color: &Color) -> (u32, u32) {
    match color {
        Color::Black => (0,12),
        Color::Brown => (1,11),
        Color::Buff => (2,10),
        Color::Chocolate => (3,9),
        Color::Cinnamon => (4,8),
        Color::Gray => (5,7),
        Color::Green => (6,6),
        Color::Orange => (7,5),
        Color::Pink => (8,4),
        Color::Purple => (9,3),
        Color::Red => (10,2),
        Color::White => (11,1),
        Color::Yellow => (12,0),
    }
}