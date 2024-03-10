Param([parameter(mandatory = $true)][String]$Name)

pwsh -Command "`$Env:RUSTFLAGS = `"-L$Env:VK_SDK_PATH/Lib`"; cargo run --example $Name --features Implements,Presentation,VK_KHR_win32_surface"
