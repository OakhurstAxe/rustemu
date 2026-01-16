
pub mod vcs {

    use std::rc::Rc;
    use std::cell::{RefCell, RefMut};
    use std::fs;

    use emumemory::memory_mapper::emu_memory::MemoryMapper;
    use iced::widget::image::FilterMethod;
    use iced::widget::{center, Image};
    use iced::{Task, Element, Subscription, time};
    use iced::time::milliseconds;

    use emucpu::base_cpu::emu_cpu::BaseCpu;
    use emucpu::m6502::emu_cpu::M6502;
    use crate::vcs_memory::vcs::VcsMemory;
    use crate::vcs_parameters::vcs::VcsParameters;
    use crate::vcs_console_type::vcs::VcsConsoleType;
    use crate::vcs_riot::vcs::VcsRiot;
    use crate::vcs_tia::vcs::VcsTia;

    #[derive(Debug, Clone)]
    pub enum Message {
        Tick
    }

    pub struct VcsConsole {
        vcs_riot: Rc<RefCell<VcsRiot>>,
        vcs_console_type: Rc<RefCell<VcsConsoleType>>,
        vcs_tia: Rc<RefCell<VcsTia>>,
        vcs_audio: i32,
        cpu: M6502,
        total_ticks: u32,
        cpu_timer: i32,
        ticks_per_frame: u32,
        image: Vec<u8>
    }

    impl Default for VcsConsole {
        fn default() -> Self {
            Self::new()
        }
    }

    impl VcsConsole {

        pub fn new () -> VcsConsole {

            let rom = fs::read("/home/dmax/projects/rust/roms/Combat (NA).a26");
            let parameters: VcsParameters;
            parameters = VcsParameters::new(rom.unwrap());

            let console_type: Rc<RefCell<VcsConsoleType>> = Rc::new(RefCell::new(VcsConsoleType::new(parameters.console_type)));
            let riot: Rc<RefCell<VcsRiot>> = Rc::new(RefCell::new(VcsRiot::new()));
            let tia: Rc<RefCell<VcsTia>> = Rc::new(RefCell::new(VcsTia::new(Rc::clone(&console_type))));
            let memory: Rc<RefCell<dyn MemoryMapper>> = Rc::new(RefCell::new(VcsMemory::new (&parameters, Rc::clone(&tia), Rc::clone(&riot))));
            let cpu: M6502 = M6502::new(Rc::clone(&memory));

            let mut temp_instance = Self {
                vcs_riot: Rc::clone(&riot),
                vcs_console_type: Rc::clone(&console_type),
                vcs_tia: Rc::clone(&tia),
                vcs_audio: 0,
                cpu: cpu,
                total_ticks: 0,
                cpu_timer: 0,
                ticks_per_frame: (console_type.borrow().ticks_per_second() / console_type.borrow().get_frames_per_second() as i32) as u32,
                image: Vec::with_capacity(0)
            };

            let console: RefMut<'_, VcsConsoleType> = console_type.borrow_mut();
            temp_instance.image = Vec::with_capacity(console.get_x_resolution() as usize * console.get_y_resolution() as usize * 4);
            temp_instance.ticks_per_frame = console.ticks_per_second() as u32 / console.get_frames_per_second() as u32;
            temp_instance.start_up();

            temp_instance

        }

        fn start_up(&mut self) {
            self.cpu.reset();
            self.vcs_riot.borrow_mut().reset();
            self.vcs_tia.borrow_mut().reset();

            self.total_ticks = 0;
//            connect(&cpuTimer_, SIGNAL(timeout()), SLOT(StartNextFrame()));
//            cpuTimer_.setTimerType(Qt::PreciseTimer);
//            cpuTimer_.setInterval(16);
//            cpuTimer_.start();
        }

        pub fn update(&mut self, message: Message) -> Task<Message> {
            match message {
                Message::Tick => {
                    self.start_next_frame();
                    self.vcs_tia.borrow_mut().repaint();
                }
            }

            Task::none()
        }

        pub fn view(&self) -> Element<'_, Message> {
            let width: u32 = self.vcs_console_type.borrow().get_x_resolution() as u32;
            let height: u32 = self.vcs_console_type.borrow().get_y_resolution() as u32;

            let image: Vec<u8> = self.vcs_tia.borrow().get_screen();
            
            let content: Image = iced::widget::image(
                iced::widget::image::Handle::from_rgba(
                    width.try_into().unwrap(),
                    height.try_into().unwrap(), 
                    image))
                .width(width as u32)
                .height(height as u32)
                .filter_method(FilterMethod::Nearest);

            center(content).into()
        }

        pub fn subscription(&self) -> Subscription<Message> {
            time::every(milliseconds(1000 / 17 as u64)).map(|_| Message::Tick)
        }
        
        pub fn start_next_frame (&mut self) {
            let mut frame_ticks: u32 = 0;
            //vcsAudio_.ExecuteTick();
            while frame_ticks < self.ticks_per_frame {
                if self.total_ticks % 3 == 0 {

                    if !self.vcs_tia.borrow().is_cpu_blocked() {
                        self.cpu.execute_tick();
                    }
                    self.vcs_riot.borrow_mut().execute_tick();
                }

                self.vcs_tia.borrow_mut().execute_tick();
                
                if self.vcs_tia.borrow_mut().repaint() {
                    self.view();
                }
                self.total_ticks += 1;
                frame_ticks += 1;
            }
        }

    }
}