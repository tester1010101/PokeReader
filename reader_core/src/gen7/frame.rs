use super::{
    draw::{PkxType, draw_citra_info, draw_daycare, draw_pkx, draw_rng, draw_rn, draw_sos},
    reader::Gen7Reader,
};
use crate::{
    draw::{draw_pkx_mini, draw_pkx_rgb}, pnp, rng::{RngWrapper, Sfmt32, Sfmt64}, title::draw_rn2, utils::{
        ShowView, help_menu::HelpMenu, menu::{Menu, MenuOption}, sub_menu::SubMenu, sub_menu_capture::SubMenuCapture
    }
};
use once_cell::unsync::Lazy;

fn gen7_specific_help() {
    pnp::println!("SOS Controls:");
    pnp::println!("[X] + [Right]:");
    pnp::println!("   Set Caller slot to");
    pnp::println!("   the current ally.");
    pnp::println!("   Use this when you");
    pnp::println!("   faint the caller.");
    pnp::println!("");
    pnp::println!("[X] + [Up]/[Down]:");
    pnp::println!("   Manually change");
    pnp::println!("   the caller slot.");
    pnp::println!("   (Not recommended)");
    pnp::println!("");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Gen7View {
    MainMenu,
    Rng,
    Daycare,
    WildPokemon,
    Sos,
    Party,
    Box,
    Pelago,
    Citra,
    HelpMenu,
    TestMenu,
    TestMenu2,
}

struct PersistedState {
    sfmt: RngWrapper<Sfmt64>,
    sos_rng: RngWrapper<Sfmt32>,
    show_view: ShowView,
    view: Gen7View,
    main_menu: Menu<Gen7View>,
    help_menu: HelpMenu,
    wild_menu: SubMenu,
    party_menu: SubMenu,
    sos_menu: SubMenuCapture,
    pelago_menu: SubMenu,
}

const MENU: &[MenuOption<Gen7View>] = &[
    MenuOption::new(Gen7View::Rng, "RNG"),
    MenuOption::new(Gen7View::Daycare, "Daycare"),
    MenuOption::new(Gen7View::WildPokemon, "Wild"),
    MenuOption::new(Gen7View::Sos, "SOS"),
    MenuOption::new(Gen7View::Party, "Party"),
    MenuOption::new(Gen7View::Box, "Box"),
    MenuOption::new(Gen7View::Pelago, "Pelago"),
    MenuOption::new(Gen7View::Citra, "Citra"),
    MenuOption::new(Gen7View::HelpMenu, "Help"),
    MenuOption::new(Gen7View::TestMenu, "TestLabel"),
    MenuOption::new(Gen7View::TestMenu2, "TestLabel2"),
];

unsafe fn get_state() -> &'static mut PersistedState {
    static mut STATE: Lazy<PersistedState> = Lazy::new(|| PersistedState {
        sfmt: RngWrapper::default(),
        sos_rng: RngWrapper::default(),
        show_view: ShowView::default(),
        view: Gen7View::TestMenu2,//Gen7View::MainMenu,
        party_menu: SubMenu::new(1, 6),
        pelago_menu: SubMenu::new(1, 3),
        wild_menu: SubMenu::new(1, 4),
        sos_menu: SubMenuCapture::new(1, 4),
        help_menu: HelpMenu::new(gen7_specific_help),
        main_menu: Menu::new(MENU),
    });
    Lazy::force_mut(&mut STATE)
}

struct XP {
    xp1: bool,
}

struct ViewState {
    curview: Gen7View,
    xp2: bool,
}

struct FrameCounter0 {
    init_frame5: u32,
    init_frame4: u32,
    init_frame3: u32,
    init_frame2: u32,
    init_frame1: u32,
    init_frame: u32,
    hit_frame: u32,
    orif2: u32,
    orif: u32,
    rst: u32,
    pb0: bool,
}

unsafe fn get_state0() -> &'static mut ViewState {
    static mut STATE0: Lazy<ViewState> = Lazy::new(|| ViewState {
        curview: Gen7View::MainMenu,
        xp2: false,
    });
    Lazy::force_mut(&mut STATE0)
}

unsafe fn get_state2() -> &'static mut XP {
    static mut STATE2: Lazy<XP> = Lazy::new(|| XP {
        xp1: false,
    });
    Lazy::force_mut(&mut STATE2)
}

unsafe fn get_state3() -> &'static mut FrameCounter0 {
    static mut STATE3: Lazy<FrameCounter0> = Lazy::new(|| FrameCounter0 {
        init_frame5: 0,
        init_frame4: 0,
        init_frame3: 0,
        init_frame2: 0,
        init_frame1: 0,
        init_frame: 0,
        hit_frame: 0,
        orif2: 0,
        orif: 0,
        rst: 0,
        pb0: true,
    });
    Lazy::force_mut(&mut STATE3)
}

fn run_frame(reader: Gen7Reader) {
    pnp::set_print_max_len(22);

    // This is safe as long as this is guaranteed to run single threaded.
    // A lock hinders performance too much on a 3ds.
    let state = unsafe { get_state() };
    let bt_state = unsafe { get_state3() };
    let view_state0 = unsafe { get_state0() };
    
    state.sfmt.reinit_if_needed(reader.init_seed());
    state.sfmt.update_advances(reader.sfmt_state());

    if !state.show_view.check() {
        return;
    }
    
    /*
    let initialized = false;
    if !initialized {
        // init-stuff
        // close when done
        // initialized = true;        
    }
    */
    let is_locked = state.main_menu.update_lock();
    let is_cached = state.main_menu.update_cached();

    if (pnp::is_just_pressed(pnp::Button::A)) || (pnp::is_just_pressed(pnp::Button::Dright)) || (pnp::is_just_pressed(pnp::Button::Dleft)) {
        bt_state.pb0 = false;


           
        if state.sfmt.advances() == 477 {
            if !bt_state.pb0 {
                bt_state.init_frame = 1;
                bt_state.pb0 = true;
            }
        }

        if state.sfmt.advances() == 478 {
            if !bt_state.pb0 {
                bt_state.init_frame = 2;
                bt_state.pb0 = true;
            }
        }
        
        if state.sfmt.advances() > 478 {
            if !bt_state.pb0 {
                if bt_state.orif2 == 0 {
                    bt_state.init_frame = state.sfmt.advances();
                    bt_state.orif2 = 1;
                    bt_state.pb0 = true;
                }
            }
        }

        if state.sfmt.advances() > 478 {
            if !bt_state.pb0 {
                if bt_state.orif2 == 1 {
                    bt_state.init_frame1 = state.sfmt.advances();
                    bt_state.orif2 = 2;
                    bt_state.pb0 = true;
                }
            }
        }

        if state.sfmt.advances() > 478 {
            if !bt_state.pb0 {
                if bt_state.orif2 == 2 {
                    bt_state.init_frame2 = state.sfmt.advances();
                    bt_state.orif2 = 3;
                    bt_state.pb0 = true;
                }
            }
        }

        if state.sfmt.advances() > 478 {
            if !bt_state.pb0 {
                if bt_state.orif2 == 3 {
                    bt_state.init_frame3 = state.sfmt.advances();
                    bt_state.orif2 = 4;
                    bt_state.pb0 = true;
                }
            }
        }

        if state.sfmt.advances() > 478 {
            if !bt_state.pb0 {
                if bt_state.orif2 == 4 {
                    bt_state.init_frame4 = state.sfmt.advances();
                    bt_state.orif2 = 5;
                    bt_state.pb0 = true;
                }
            }
        }

        if state.sfmt.advances() > 478 {
            if !bt_state.pb0 {
                if bt_state.orif2 == 5 {
                    bt_state.init_frame5 = state.sfmt.advances();
                    bt_state.orif2 = 6;
                    bt_state.pb0 = true;
                }
            }
        }

        bt_state.orif = bt_state.hit_frame;
        bt_state.hit_frame = state.sfmt.advances();
        bt_state.rst = state.sfmt.advances() - bt_state.orif;
    }    

    state.view = state.main_menu.next_view(Gen7View::MainMenu, state.view);


    match state.view {
        Gen7View::Rng => draw_rng(&reader, &state.sfmt),
        Gen7View::Daycare => draw_daycare(&reader),
        Gen7View::WildPokemon => {
            let slot = state.wild_menu.update_and_draw(is_locked);
            draw_pkx(&reader.wild_pkm((slot - 1) as u32), PkxType::Wild);
        }
        Gen7View::Sos => {
            let prev_caller_slot = state.sos_menu.counter_value();
            let prev_correction_value = state.sos_menu.captured_value();
            let caller_slot = state.sos_menu.update_headless(
                is_locked,
                reader.sos_chain() as u32,
                reader.ally_slot(prev_caller_slot as u32, prev_correction_value) as usize + 1,
            );
            let correction_value = state.sos_menu.captured_value();
            draw_sos(&reader, &mut state.sos_rng, caller_slot as u32, correction_value);
        }
        Gen7View::Box => draw_pkx(&reader.box_pkm(), PkxType::Tame),
        Gen7View::Citra => draw_citra_info(&reader),
        Gen7View::Party => {
            let slot = state.party_menu.update_and_draw(is_locked);
            draw_pkx(&reader.party_pkm((slot - 1) as u32), PkxType::Tame);
        }
        Gen7View::Pelago => {
            let slot = state.pelago_menu.update_and_draw(is_locked);
            draw_pkx(&reader.pelago_pkm((slot - 1) as u32), PkxType::Wild)
        }
        Gen7View::HelpMenu => state.help_menu.update_and_draw(is_locked),
        //Gen7View::TestMenu => draw_test(&state.sfmt),

        Gen7View::TestMenu => {
            let slot = state.wild_menu.update_and_draw(is_locked);
            if slot == 0 {
                draw_pkx_mini(&reader.wild_pkm((slot - 1) as u32), PkxType::Wild, (slot - 1) as u32);
            }
            if slot == 1 {
                draw_pkx_mini(&reader.wild_pkm((slot - 1) as u32), PkxType::Wild, (slot - 1) as u32);
            }
            if slot == 2 {
                draw_pkx_mini(&reader.wild_pkm((slot - 1) as u32), PkxType::Wild, (slot - 1) as u32);
            }
            if slot == 3 {
                draw_pkx_mini(&reader.wild_pkm((slot - 1) as u32), PkxType::Wild, (slot - 1) as u32);
            }
            if slot == 4 {
                }
        }

        Gen7View::TestMenu2 => {
            let slot = state.wild_menu.update_and_draw(is_locked);
            if slot == 0 {
                pnp::println!("rst: {}", bt_state.rst);
                pnp::println!("hit: {}", bt_state.hit_frame);
                pnp::println!("{} :: >{}/{}/{}/", slot, bt_state.init_frame, bt_state.init_frame1, bt_state.init_frame2);
                pnp::println!("{} :: >{}/{}/{}/", slot, bt_state.init_frame3, bt_state.init_frame4, bt_state.init_frame5);
                draw_rn(&state.sfmt);
                draw_pkx_rgb(&reader.wild_pkm((slot - 1) as u32), PkxType::Wild, (slot - 1) as u32);
            }
            if slot == 1 {
                pnp::println!("rst: {}", bt_state.rst);
                pnp::println!("hit: {}", bt_state.hit_frame);
                pnp::println!("{} :: >{}/{}/{}/", slot, bt_state.init_frame, bt_state.init_frame1, bt_state.init_frame2);
                pnp::println!("{} :: >{}/{}/{}/", slot, bt_state.init_frame3, bt_state.init_frame4, bt_state.init_frame5);
                draw_rn(&state.sfmt);
                draw_pkx_rgb(&reader.wild_pkm((slot - 1) as u32), PkxType::Wild, (slot - 1) as u32);
            }
            if slot == 2 {
                pnp::println!("rst: {}", bt_state.rst);
                pnp::println!("hit: {}", bt_state.hit_frame);
                pnp::println!("{} :: >{}/{}/{}/", slot, bt_state.init_frame, bt_state.init_frame1, bt_state.init_frame2);
                pnp::println!("{} :: >{}/{}/{}/", slot, bt_state.init_frame3, bt_state.init_frame4, bt_state.init_frame5);
                draw_rn(&state.sfmt);
                draw_pkx_rgb(&reader.wild_pkm((slot - 1) as u32), PkxType::Wild, (slot - 1) as u32);
            }
            if slot == 3 {
                pnp::println!("rst: {}", bt_state.rst);
                pnp::println!("hit: {}", bt_state.hit_frame);            
                pnp::println!("{} :: >{}/{}/{}/", slot, bt_state.init_frame, bt_state.init_frame1, bt_state.init_frame2);
                pnp::println!("{} :: >{}/{}/{}/", slot, bt_state.init_frame3, bt_state.init_frame4, bt_state.init_frame5);
                draw_rn(&state.sfmt);
                draw_pkx_rgb(&reader.wild_pkm((slot - 1) as u32), PkxType::Wild, (slot - 1) as u32);
            }
            if slot == 4 {
                }
            }

        Gen7View::MainMenu => {
            state.main_menu.update_view();
            state.main_menu.draw();
            pnp::println!("{:#?}", draw_rn(&state.sfmt))
        }
    }

    let state2 = unsafe { get_state2() };
    
    if state2.xp1 == false {
        draw_rn2(&state.sfmt);
        state2.xp1 = true;
    }
}

pub fn run_sm_frame() {
    let reader = Gen7Reader::sm();
    run_frame(reader)
}

pub fn run_usum_frame() {
    let reader = Gen7Reader::usum();
    run_frame(reader)
}