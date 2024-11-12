#![allow(dead_code)]
pub mod background_color;
pub mod maze_colors;

use bevy::color::Color;

pub const NEUTRAL_0: Color =
    Color::srgb(0.20392156862745098, 0.5450980392156862, 0.7215686274509804);

pub const DARK_GREY: Color = Color::srgb(
    0.12549019607843137,
    0.12549019607843137,
    0.12549019607843137,
);

pub const PRIMARY_600: Color =
    Color::srgb(0.11372549019607843, 0.9764705882352941, 0.9098039215686274);

pub const PRIMARY_500: Color =
    Color::srgb(0.3411764705882353, 0.9803921568627451, 0.9176470588235294);

pub const PRIMARY_400: Color =
    Color::srgb(0.4666666666666667, 0.984313725490196, 0.9294117647058824);

pub const PRIMARY_300: Color =
    Color::srgb(0.5647058823529412, 0.9882352941176471, 0.9411764705882353);

pub const PRIMARY_200: Color =
    Color::srgb(0.6509803921568628, 0.9921568627450981, 0.9490196078431372);

pub const PRIMARY_100: Color =
    Color::srgb(0.6509803921568628, 0.9921568627450981, 0.9490196078431372);

pub const COMPLEMENTARY_100: Color = Color::srgb(249. / 255., 29. / 255., 46. / 255.);

pub const COMPLEMENTARY_200: Color = Color::srgb(250. / 255., 87. / 255., 104. / 255.);

pub const COMPLEMENTARY_300: Color = Color::srgb(251. / 255., 119. / 255., 133. / 255.);

pub const COMPLEMENTARY_400: Color = Color::srgb(252. / 255., 144. / 255., 157. / 255.);

pub const COMPLEMENTARY_500: Color = Color::srgb(253. / 255., 166. / 255., 177. / 255.);

pub const COMPLEMENTARY_600: Color = Color::srgb(254. / 255., 187. / 255., 195. / 255.);

#[allow(dead_code)]
pub const LIGHT_GREY: Color =
    Color::srgb(0.6078431372549019, 0.6823529411764706, 0.7372549019607844);
