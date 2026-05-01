// PNP Compatibility file
// PNP uses wasm, so all functions are wasm compatible - hence the weirdness.

#pragma once

#include <3ds.h>

void draw_to_screen(u32 screenId, u8 *framebuffer, u32 stride, u32 format);
void host_print(u32 ptr, u32 size, u32 color);
void host_read_mem(u32 game_addr, u32 size, u32 out_ptr);
void host_write_mem(u32 game_addr, u32 size, u32 in_ptr);
void scan_input();
u32 host_just_pressed();
u32 host_is_just_pressed(u32 io_bits);
void host_set_print_max_len(u32 max_len);
u64 host_get_game_title_id();
void set_game_start_ms(u64 time);
u64 host_game_start_ms();
void set_trampoline_addr(u32 trampoline);
u32 get_trampoline_addr();
void set_route_hook_addr(u32 route_hook);
u32 get_route_hook_addr();
u32 pa_from_va_ptr(u32 addr);

//use copy/write function from C/3ds and use in rust with var (rng) / (disk file open/write|update/close)

/*
    void    InitMenu(PluginMenu &menu)
    {
        menu += new MenuEntry("Jisho", nullptr, [](MenuEntry *entry)
        {
            std::string dictPath = "romfs:/JMdict_smol.txt";
            if (!File::Exists(dictPath)) {
                MessageBox("Missing dict", "Please install JMdict_smol.txt to your memory card root")();
            } else {
                const int bufferSize = 2000;
                const int maxResults = 20;
                const int indexMaxSize = 1000;

                Keyboard keyboard;
                std::string output = "";
                if (keyboard.Open(output) != 0) {
                    return;
                }
                std::string search = RomajiToHiragana(output);
                keyboard.Close();

                if (search == "") {
                    return;
                }

                File dict;
                File::Open(dict, dictPath, File::READ);
                char* buffer = new char[bufferSize];

*/