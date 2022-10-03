use druid::widget::{Align, Flex, Label, TextBox, Slider, ViewSwitcher, KnobStyle, SizedBox, Click, ControllerHost};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WidgetExt, WindowDesc, Color, theme, UnitPoint, KeyOrValue, WidgetId};
use rand::Rng;
use clipboard::{ClipboardProvider, ClipboardContext};


const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const HORIZONTAL_WIDGET_SPACING: f64 = 15.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Password generator!");

const DARK_COLOR: Color = Color::rgb8(38, 38, 38);
const WHITE_COLOR: Color = Color::rgb8(255, 255, 255);
const LIME_COLOR: Color = Color::rgb8(4, 255, 74);


#[derive(Debug, Clone, Data, Lens)]
struct HelloState {
    password: String,
    value: f64,
    strenght: String,
    entropy: f64,
    charsets: [CharSet; 4],
}


#[derive(Debug, Clone, Data)]
struct CharSet {
    name: String,
    set: String,
    status: bool,
    #[data(ignore)]
    widget_id: WidgetId
}


pub fn main() {
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((410.0, 223.0));
        
    let initial_state = HelloState {
        password: "Your password".into(),
        value: 6.0,
        strenght: "weak".to_string(),
        entropy: 0.0,
        charsets: [
            CharSet {
                name: "a-z".to_string(),
                set: "abcdefghijklmnopqrstuvwxyz".to_string(),
                status: false,
                widget_id: WidgetId::reserved(0)
            },
            CharSet {
                name: "A-Z".to_string(),
                set: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
                status: false,
                widget_id: WidgetId::reserved(1)
            },
            CharSet {
                name: "0-9".to_string(),
                set: "0123456789".to_string(),
                status: false,
                widget_id: WidgetId::reserved(2)
            },
            CharSet {
                name: "%!$".to_string(),
                set: ")(*&^%$#@!~".to_string(),
                status: false,
                widget_id: WidgetId::reserved(3)
            }
        ]
    };

    AppLauncher::with_window(main_window)
        .configure_env(|env, _| {
            env.set(theme::WINDOW_BACKGROUND_COLOR, DARK_COLOR);
            env.set(theme::WIDGET_PADDING_VERTICAL, 15.0);
            env.set(theme::WIDGET_PADDING_HORIZONTAL, 15.0);
        })
        .launch(initial_state)
        .expect("Failed to launch application");
}


fn build_root_widget() -> impl Widget<HelloState> {
    let row = Flex::row()
        .with_child(
            TextBox::new()
                .with_placeholder("Your password")
                .with_text_size(22.0)
                .fix_width(270.0)
                .fix_height(35.0)
                .border(Color::from_hex_str("#fff").unwrap(), 1.0)
                .lens(HelloState::password)
        )
        .with_spacer(HORIZONTAL_WIDGET_SPACING)
        .with_child(
            Label::new("R".to_string())
                .with_text_size(24.0)
                .align_horizontal(UnitPoint::CENTER)
                .fix_width(35.0)
                .fix_height(35.0)
                .border(Color::from_hex_str("#fff").unwrap(), 1.0)
                .on_click(|_, data: &mut HelloState, _| {
                    data.password = generate_password((data.value) as i64, data);
                    data.entropy = get_entropy(data);
                    data.strenght = get_strenght(data);
                })
        )
        .with_spacer(HORIZONTAL_WIDGET_SPACING)
        .with_child(
            Label::new("C".to_string())
                .with_text_size(24.0)
                .align_horizontal(UnitPoint::CENTER)
                .fix_width(35.0)
                .fix_height(35.0)
                .border(Color::from_hex_str("#fff").unwrap(), 1.0)
                .on_click(|_, data: &mut HelloState, _| {
                    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                    ctx.set_contents(data.password.clone()).unwrap();
                })
        );

    let desc = Flex::row()
        .with_child(
            Label::dynamic(|data: &HelloState, _| {
                format!("Strenght: {}", data.strenght)
            })
        )
        .with_spacer(120.0)
        .with_child(
            Label::dynamic(|data: &HelloState, _| {
                format!("Entropy: {:.2} bit", data.entropy)
            })
        );

    let slider = Flex::row()
        .with_child(
            Slider::new()
                .with_range(1.0, 64.0)
                .track_color(KeyOrValue::Concrete(LIME_COLOR))
                .knob_style(KnobStyle::Circle)
                .with_step(1.0)
                .fix_width(329.0)
                
                .lens(HelloState::value)
        )
        .with_spacer(HORIZONTAL_WIDGET_SPACING)
        .with_child(
            Label::dynamic(|data: &HelloState, _| {
                format!("{}", data.value)
            })  
                .with_text_size(16.0)
                .fix_width(32.0)
                .fix_height(21.0)
                .align_horizontal(UnitPoint::CENTER)
                .center()
            
        );

    let btns = Flex::row()
        .with_child(
            ViewSwitcher::new(
                |data: &HelloState, _| data.charsets.clone(), 
                |charsets, _, _| {
                return get_charset_btn(0, charsets).boxed();
            })
        )
        .with_spacer(HORIZONTAL_WIDGET_SPACING)
        .with_child(
            ViewSwitcher::new(
                |data: &HelloState, _| data.charsets.clone(), 
                |charsets, _, _| {
                return get_charset_btn(1, charsets).boxed();
            })
        )
        .with_spacer(HORIZONTAL_WIDGET_SPACING)
        .with_child(
            ViewSwitcher::new(
                |data: &HelloState, _| data.charsets.clone(), 
                |charsets, _, _| {
                return get_charset_btn(2, charsets).boxed();
            })
        )
        .with_spacer(HORIZONTAL_WIDGET_SPACING)
        .with_child(
            ViewSwitcher::new(
                |data: &HelloState, _| data.charsets.clone(), 
                |charsets, _, _| {
                return get_charset_btn(3, charsets).boxed();
            })
        );
    
    let layout = Flex::column()
        .with_child(row)
        .with_spacer(35.0)
        .with_child(desc)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(slider)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(btns);
        
    Align::centered(layout)
}


fn generate_password(len: i64, data: &HelloState) -> String {
    let mut temp_charset = String::new();
            
    for i in data.charsets.clone() {
        if i.status {
            temp_charset.push_str(&i.set);
        }
    }
    if temp_charset.is_empty() {
        temp_charset.push_str(&data.charsets[0].set);
    }

    let charset = temp_charset.as_bytes();
    let mut rng = rand::thread_rng();
    let password: String = (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();

    password
}


fn get_number_possible_characters(charsets: [CharSet; 4]) -> f64 {
    let mut number_possible_characters: f64 = 0.0;

    for i in charsets {
        if i.status {
            number_possible_characters += i.set.len() as f64;
        }
    }

    if number_possible_characters == 0.0 {
        number_possible_characters = 26.0;
    }

    number_possible_characters    
}


fn get_entropy(data: &HelloState) -> f64 {
    let number_possible_characters = get_number_possible_characters(data.charsets.clone());
    data.password.len() as f64 * (number_possible_characters.log2() / 2.0_f64.log2())
}


fn get_strenght(data: &HelloState) -> String {
    let number_possible_characters = get_number_possible_characters(data.charsets.clone());
    if (number_possible_characters > 26.0) && (data.value > 10.0) {
        return String::from("strong");
    }

    return String::from("weak");
}


fn get_charset_btn(i: usize, charsets: &[CharSet; 4]) -> ControllerHost<SizedBox<HelloState>, Click<HelloState>> {      
    let charset = &charsets[i];
    let c = match charset.status {
        false => WHITE_COLOR,
        true => LIME_COLOR
    };
    
    Label::new(charset.name.clone())
        .with_text_color(c.clone())    
        .align_horizontal(UnitPoint::CENTER)
        .align_vertical(UnitPoint::CENTER)
        .border(c.clone(), 1.0)
        .with_id(charset.widget_id)
        .fix_width(70.0)
        .fix_height(35.0)
        .on_click(|e, data: &mut HelloState, _| {
            data.charsets[get_charset_index_by_widget_id(&data.charsets, e.widget_id())].status ^= true;
            data.password = generate_password((data.value) as i64, data);
            data.entropy = get_entropy(data);
            data.strenght = get_strenght(data);
        })
}


fn get_charset_index_by_widget_id(charsets: &[CharSet; 4], widget_id: WidgetId) -> usize {
    for (i, charset) in charsets.iter().enumerate() {
        if charset.widget_id == widget_id {
            return i;
        }
    }
    return 0;
}