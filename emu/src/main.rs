
use iced::Theme;

use vcs::vcs_console::vcs::VcsConsole;

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application(VcsConsole::default, VcsConsole::update, VcsConsole::view)
        .subscription(VcsConsole::subscription)
        .theme(Theme::Dark)
        .centered()
        .run()
}

