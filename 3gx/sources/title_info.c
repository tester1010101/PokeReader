#include <3ds.h>
#include <string.h>
#include <stdio.h>
#include "title_info.h"


Result writeToSdmcDirectly(u32 buffer) {
    //
    // 
    // Initialize stuff to write to SDMC
    fsInit();
    Result res = 0;
    Handle fileHandle = 0;
    const char* filepath = "/t5.txt";
    
    // Convert file path to an FS path object
    FS_Path fsPath = fsMakePath(PATH_ASCII, (const u8*)filepath);

    // Open the file directly using the default FSUSER session handle
    res = FSUSER_OpenFileDirectly(&fileHandle, ARCHIVE_SDMC, fsMakePath(PATH_EMPTY, ""), 
                                  fsPath, FS_OPEN_WRITE | FS_OPEN_CREATE, 0);

    if (R_SUCCEEDED(res)) {
        // ##############################################
        // => Thanks to StackOverflow question # 3464194 
        // => Thanks to GeminiAI too, formatting

        u32 bytesWritten = 0;
        uint32_t v1 = buffer;
        char hex_string[9];

        snprintf(hex_string, sizeof(hex_string), "%08X", v1);

        // #####################################################
        // => Thanks to GeminiAI for functional code formatting

        // Write data directly to the file
        res = FSFILE_Write(fileHandle, &bytesWritten, 0, (const void*)hex_string, 9, FS_WRITE_FLUSH);

        // Close the file when done
        FSFILE_Close(fileHandle);
    }

    // Close stuff to write to SDMC accurately
    fsExit();
    return res;
    buffer = 0;
}

u64 g_program_id = 0;

u64 get_title_id()
{
  if (g_program_id == 0)
  {
    fsInit();
    u32 process_id = 0;
    svcGetProcessId(&process_id, CUR_PROCESS_HANDLE);
    FS_ProgramInfo info;
    FSUSER_GetProgramLaunchInfo(&info, process_id);
    g_program_id = info.programId;
    fsExit();
  }

  return g_program_id;
}

u16 g_remaster_version = 0;

u16 get_remaster_version() {
  if (g_remaster_version == 0) {
    fsInit();
    u32 processId = 0;
    svcGetProcessId(&processId, CUR_PROCESS_HANDLE);
    FS_ProductInfo info;
    FSUSER_GetProductInfo(&info, processId);
    fsExit();
    g_remaster_version = info.remasterVersion;
  }

  return g_remaster_version;
}
