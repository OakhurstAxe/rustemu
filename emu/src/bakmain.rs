
use std::time::Duration;
use iced::widget::image::FilterMethod;
use iced::{Element, Task};
use iced::widget::center;

pub fn main() -> iced::Result {
    iced::application(boot, Example::update, Example::view)
    .subscription(|_| iced::time::every(Duration::from_millis(16)).map(|_| Message::Step))
    .run()
}

#[derive(Default)]
struct Example {
    frame: u64
}

fn boot() -> (Example, Task<Message>) {
    // Initialize the state with a default value.
    let initial_state = Example::default(); 
    
    // Optionally, return a starting task. Task::none() means no task.
    (initial_state, Task::none()) 
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Step
}

// Kevin Reid
impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::Step => self.frame += 1
        }
    }

    fn view(&self) -> Element<Message> {
        let mut image = vec![0u8; 256 * 240 * 4];
        for x in 0..256 {
            for y in 0..240 {
                let pixel_index = (x + y * 256) * 4;
                let val = ((x ^ y) * 1) as u8;
                image[pixel_index + 0] = val.wrapping_sub((self.frame * 100 / 80) as u8);
                image[pixel_index + 1] = val.wrapping_sub((self.frame * 100 / 60) as u8);
                image[pixel_index + 2] = val.wrapping_sub((self.frame * 100 / 40) as u8);
                image[pixel_index + 3] = 255;
            }
        }

        let content = iced::widget::image(iced::widget::image::Handle::from_rgba(256, 240, image))
            .width(512)
            .height(480)
            .filter_method(FilterMethod::Nearest);
        
        center(content).into()

    }
}

