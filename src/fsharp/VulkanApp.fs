module VulkanApp

open System.Runtime.InteropServices

module private MacOSNative =
    [<DllImport("libnovel_game.dylib")>]
    extern int run()

module private LinuxNative =
    [<DllImport("libnovel_game.so")>]
    extern int run()

type VulkanApp =
    abstract member Run: unit -> int

type VulkanAppForMacOS() =
    interface VulkanApp with
        member this.Run() = MacOSNative.run()

type VulkanAppForLinux() =
    interface VulkanApp with
        member this.Run() = LinuxNative.run()
