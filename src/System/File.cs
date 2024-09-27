using System.Text.Json;
using Microsoft.JSInterop;

namespace System.IO;

public class File
{
    public static async Task<string> ReadAllTextAsync(IJSInProcessRuntime runtime, string filename)
    {
        return await runtime.InvokeAsync<string>("__TAURI__.tauri.invoke", "read_file_string", new { name = filename });
    }

    public static async Task<byte[]> ReadAllBytesAsync(IJSInProcessRuntime runtime, string filename)
    {
        JsonElement[] ret = await runtime.InvokeAsync<JsonElement[]>("__TAURI__.tauri.invoke", "read_file_bytes", new { name = filename });

        byte[] bts = new byte[ret.Length];
        for (int i = 0; i < ret.Length; i++)
        {
            bts[i] = ret[i].GetByte();
        }

        return bts;
    }

    public static async Task CreateAsync(IJSInProcessRuntime runtime, string filename)
    {
        await runtime.InvokeAsync<string>("__TAURI__.tauri.invoke", "create_file", new { path = filename });
    }
}